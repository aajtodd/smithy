### Trait Implementation

Smithy traits are metadata attached to shapes that provide additional information or behavior. In our Rust implementation, we represent traits using a combination of the `Trait` Rust trait and concrete trait implementations.

#### Core Trait Design

We use a trait-based approach with downcasting for type-safe access:

```rust
/// Type alias for boxed trait objects
pub type BoxTrait = Box<dyn Trait>;

/// Trait for implementing Smithy traits
pub trait Trait: Any + Send + Sync {
    /// Returns the static ID of this trait type
    fn static_id() -> ShapeId where Self: Sized;
    
    /// Returns the ID of this trait instance (defaults to static_id)
    fn id(&self) -> ShapeId {
        Self::static_id()
    }
    
    /// Convert this trait to a Node for serialization
    fn to_node(&self) -> Node;
    
    /// Create an instance of this trait from a Node
    fn from_node(node: &Node) -> Option<Self> where Self: Sized;
    
    /// Create a boxed trait from a Node
    fn from_node_boxed(node: &Node) -> Option<BoxTrait> where Self: Sized {
        Self::from_node(node).map(|t| Box::new(t) as BoxTrait)
    }
    
    /// Clone this trait
    fn clone_trait(&self) -> BoxTrait;
    
    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

Each concrete trait type implements this interface:

```rust
/// The documentation trait
#[derive(Clone, Debug)]
pub struct Documentation(pub String);

impl Trait for Documentation {
    fn static_id() -> ShapeId {
        ShapeId::new_unchecked("smithy.api", "documentation")
    }
    
    fn to_node(&self) -> Node {
        Node::String(self.0.clone())
    }
    
    fn from_node(node: &Node) -> Option<Self> {
        node.as_str().map(|s| Self(s.to_string()))
    }
    
    fn clone_trait(&self) -> BoxTrait {
        Box::new(self.clone())
    }
}
```

#### Dynamic Trait Support

To handle unknown or dynamically loaded traits, we provide a `DynamicTrait` implementation:

```rust
/// A dynamic trait for representing unknown or dynamically loaded traits
#[derive(Clone, Debug)]
pub struct DynamicTrait {
    id: ShapeId,
    value: Option<Node>,
}

impl DynamicTrait {
    /// Create a new dynamic trait
    pub fn new(id: impl Into<ShapeId>, value: impl Into<Option<Node>>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
        }
    }
    
    /// Get the value of this trait
    pub fn value(&self) -> Option<&Node> {
        self.value.as_ref()
    }
}

impl Trait for DynamicTrait {
    fn static_id() -> ShapeId {
        // This is a placeholder - the actual ID is stored in the instance
        ShapeId::new_unchecked("", "dynamic")
    }
    
    // Override the default implementation to return the instance-specific ID
    fn id(&self) -> ShapeId {
        self.id.clone()
    }
    
    fn to_node(&self) -> Node {
        self.value.clone().unwrap_or(Node::Null)
    }
    
    fn from_node(_node: &Node) -> Option<Self> {
        // This can't be called directly since we need the ID
        // Use new() instead
        None
    }
    
    fn clone_trait(&self) -> BoxTrait {
        Box::new(self.clone())
    }
}
```

#### Trait Registry

The trait registry manages the creation of traits from IDs and Node values:

```rust
/// Registry for trait creation during deserialization
pub struct TraitRegistry {
    creators: HashMap<String, fn(&Node) -> Option<BoxTrait>>,
}

impl TraitRegistry {
    pub fn new() -> Self {
        let mut registry = Self { creators: HashMap::new() };
        
        // Register built-in traits
        registry.register::<Documentation>();
        registry.register::<Required>();
        // Register other built-in traits
        
        registry
    }
    
    pub fn register<T: Trait + 'static>(&mut self) {
        let id = T::static_id().to_string();
        self.creators.insert(id, T::from_node_boxed);
    }
    
    /// Create a trait from an ID and Node, falling back to DynamicTrait if not registered
    pub fn create_trait(&self, id: impl AsRef<ShapeId>, node: &Node) -> BoxTrait {
        let id_ref = id.as_ref();
        
        // Try to create a registered trait
        if let Some(creator) = self.creators.get(&id_ref.to_string()) {
            if let Some(trait_) = creator(node) {
                return trait_;
            }
        }
        
        // Fall back to DynamicTrait
        Box::new(DynamicTrait::new(id_ref.clone(), Some(node.clone())))
    }
}
```

#### Shape Integration

Shapes store traits in a HashMap and provide methods for accessing them:

```rust
pub struct Shape {
    id: ShapeId,
    traits: HashMap<ShapeId, BoxTrait>,
    kind: ShapeKind,
}

impl Shape {
    /// Check if this shape has a specific trait by ID
    pub fn has_trait(&self, trait_id: impl AsRef<ShapeId>) -> bool {
        self.traits.contains_key(trait_id.as_ref())
    }
    
    /// Check if this shape has a specific trait type
    pub fn has_trait_type<T: Trait + 'static>(&self) -> bool {
        self.has_trait(T::static_id())
    }
    
    /// Get a specific trait by ID
    pub fn get_trait_by_id(&self, trait_id: impl AsRef<ShapeId>) -> Option<&dyn Trait> {
        self.traits.get(trait_id.as_ref()).map(|t| t.as_ref())
    }
    
    /// Get a specific trait with a concrete type
    pub fn get_trait<T: Trait + 'static>(&self) -> Option<&T> {
        self.get_trait_by_id(T::static_id())
            .and_then(|t| t.as_any().downcast_ref::<T>())
    }
    
    /// Get a specific trait with a concrete type, panicking if not found or wrong type
    pub fn expect_trait<T: Trait + 'static>(&self) -> &T {
        self.get_trait::<T>().unwrap_or_else(|| {
            panic!(
                "Expected trait {} on shape {}, but it was not found or had the wrong type",
                T::static_id(),
                self.id
            )
        })
    }
    
    /// Apply a trait to this shape
    pub fn with_trait<T: Trait + 'static>(mut self, trait_: T) -> Self {
        let id = trait_.id();
        self.traits.insert(id, Box::new(trait_));
        self
    }
}
```

#### Design Considerations

1. **Type Safety**: We use Rust's type system to provide type-safe access to traits through downcasting.

2. **Flexibility**: The design supports both known and unknown traits through the `DynamicTrait` implementation.

3. **Performance**: We optimize for common operations like trait lookup by using a HashMap for storage.

4. **Extensibility**: The system is extensible for custom traits through the `Trait` interface.

5. **Compatibility**: We maintain compatibility with the Smithy specification by supporting all trait types.

6. **Ergonomics**: The API provides convenient methods for common operations while still supporting advanced use cases.


#### Trait Conflicts and Validation

Trait conflicts and validation will be handled during the model validation phase rather than during parsing:

```rust
// During model validation
pub struct TraitConflictValidator;

impl Validator for TraitConflictValidator {
    fn validate(&self, model: &Model) -> Vec<ValidationEvent> {
        let mut events = Vec::new();
        
        // Check each shape for trait conflicts
        for shape in model.shapes() {
            let traits = shape.traits();
            
            // Check each trait against others for conflicts
            for (id1, trait1) in traits.iter() {
                if let Some(conflicts) = self.get_trait_conflicts(id1) {
                    for conflict_id in conflicts {
                        if traits.contains_key(conflict_id) {
                            events.push(ValidationEvent::error(
                                format!("Trait {} conflicts with {}", id1, conflict_id),
                                shape.id().clone(),
                            ));
                        }
                    }
                }
            }
        }
        
        events
    }
    
    fn get_trait_conflicts(&self, trait_id: &TraitId) -> Option<&[TraitId]> {
        // Implementation would look up conflicts for the given trait
        // This could be from built-in knowledge or loaded from trait definitions
        // ...
    }
}
```

#### Alternative Approaches Considered

1. **Dynamic Approach**

A purely dynamic approach with runtime validation:

```rust
pub struct TraitContainer {
    traits: HashMap<TraitId, Trait>,
}

impl TraitContainer {
    pub fn apply(&mut self, trait_: Trait, shape: &Shape) -> Result<(), TraitApplicationError> {
        // Runtime validation of trait application
        validate_trait_application(&trait_, shape)?;
        self.traits.insert(trait_.id.clone(), trait_);
        Ok(())
    }
}
```

**Pros:**
- Simpler implementation
- More flexible for dynamic loading
- Easier to extend with new traits

**Cons:**
- Less type safety
- No compile-time checks
- Less ergonomic for programmatic use

2. **Type-Safe Approach**

A strongly typed approach using Rust's type system:

```rust
pub trait StructureShape {}
pub trait ServiceShape {}

pub trait StructureOnlyTrait: SmithyTrait {}

impl<T: StructureShape> Shape<T> {
    pub fn apply_structure_trait<S: StructureOnlyTrait>(&mut self, trait_: S) -> Result<(), TraitApplicationError> {
        // No need to check if this is a structure - the type system ensures it
        self.apply_trait(trait_)
    }
}
```

**Pros:**
- Strong compile-time guarantees
- Better IDE support
- More ergonomic for programmatic use

**Cons:**
- Complex implementation
- Difficult to use for dynamic loading
- Requires extensive type definitions

