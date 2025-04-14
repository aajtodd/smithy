# Smithy Mixins Design

This document outlines the design for implementing Smithy mixins in the Rust implementation.

## Overview

Mixins in Smithy are a mechanism for reusing shape definitions across multiple shapes. A mixin is a shape marked with the `@mixin` trait. Adding a mixin to a shape causes the members and traits of the mixin to be copied into the target shape.

## Core Components

### 1. Mixin Trait Definition

```rust
/// The mixin trait
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Mixin {
    /// Traits that should not be copied to shapes that use this mixin
    pub local_traits: HashSet<ShapeId>,
}

const TRAIT_ID: &'static ShapeId = &ShapeId::new_static("smithy.api", "mixin");

impl Trait for Mixin {
    fn static_id() -> &'static ShapeId {
        &TRAIT_ID
    }

    fn id(&self) -> &ShapeId {
        Self::static_id()
    }

    fn to_node(&self) -> Node {
        let mut obj = HashMap::new();
        if !self.local_traits.is_empty() {
            let local_traits = self.local_traits.iter()
                .map(|id| Node::String(id.to_string()))
                .collect::<Vec<_>>();
            obj.insert("localTraits".to_string(), Node::Array(local_traits));
        }
        Node::Object(obj)
    }

    fn from_node(node: &Node) -> Option<Self> {
        match node {
            Node::Object(obj) => {
                let local_traits = if let Some(Node::Array(arr)) = obj.get("localTraits") {
                    arr.iter()
                        .filter_map(|n| n.as_str())
                        .filter_map(|s| ShapeId::from_str(s).ok())
                        .collect()
                } else {
                    Vec::new()
                };
                Some(Self { local_traits })
            }
            Node::Bool(true) => Some(Self { local_traits: Vec::new() }),
            _ => None,
        }
    }

    fn clone_trait(&self) -> BoxTrait {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

### 2. ShapeMetadata with Mixins

```rust
/// Common metadata for all shapes (implementation detail)
#[derive(Debug, Clone)]
pub(crate) struct ShapeMetadata {
    /// The shape ID
    id: ShapeId,
    /// Traits applied directly to this shape (introduced traits)
    introduced_traits: TraitMap,
    /// All traits applied to this shape (including those from mixins)
    effective_traits: TraitMap,
    /// Mixins applied to this shape (full shape references)
    mixins: Vec<Shape>,
}

impl ShapeMetadata {
    /// Create new shape metadata
    pub(crate) fn new(id: ShapeId, traits: TraitMap) -> Self {
        Self { 
            id, 
            introduced_traits: traits.clone(),
            effective_traits: traits,
            mixins: Vec::new(),
        }
    }
    
    /// Add a mixin to this shape
    pub(crate) fn add_mixin(&mut self, mixin: Shape) {
        self.mixins.push(mixin);
    }
}
```

### 3. HasMixins Trait

```rust
/// Common trait for all shape types providing access to mixins
pub trait HasMixins: HasShapeId {
    /// Get the mixins applied to this shape
    fn mixins(&self) -> &[Shape];
}

// Blanket implementation for any type that provides shape metadata
impl<T: ProvideShapeMetadata> HasMixins for T {
    fn mixins(&self) -> &[Shape] {
        &self.meta().mixins
    }
}
```

### 4. Updated HasTraits Trait

```rust
/// Common trait for all shape types providing access to traits
pub trait HasTraits: HasShapeId {
    /// Get all traits applied to this shape (including those from mixins)
    fn traits(&self) -> &TraitMap;
    
    /// Get traits applied directly to this shape (excluding those from mixins)
    fn introduced_traits(&self) -> &TraitMap;

    /// Check if this shape has a specific trait
    fn has_trait(&self, trait_id: impl AsRef<ShapeId>) -> bool {
        self.traits().contains_key(trait_id.as_ref())
    }

    // Other methods remain the same...
}

// Blanket implementation
impl<T: ProvideShapeMetadata> HasTraits for T {
    fn traits(&self) -> &TraitMap {
        &self.meta().effective_traits
    }
    
    fn introduced_traits(&self) -> &TraitMap {
        &self.meta().introduced_traits
    }
}
```

### 5. ShapeBuilderExt with Mixin Support

```rust
/// Extension trait for shape builders with common trait methods.
pub trait ShapeBuilderExt: ProvideTraitsMut + Sized {
    // Existing methods...
    
    /// Mark the shape as a mixin.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StructureShape;
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::traits::{Mixin, Trait};
    ///
    /// let mixin = StructureShape::builder()
    ///     .id("example.foo#MyMixin")
    ///     .mixin()
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(mixin.has_trait(Mixin::static_id()));
    /// ```
    fn mixin(self) -> Self {
        self.with_trait(Mixin::default())
    }
}
```

## Shape Builder Implementation

### StructureShapeBuilder with Mixin Support

```rust
/// Builder for creating a structure shape.
#[derive(Debug, Default)]
pub struct StructureShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
    members: HashMap<String, MemberShape>,
    mixins: Vec<Shape>,
}

impl StructureShapeBuilder {
    // Other methods...
    
    /// Add a mixin to the structure shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::{StructureShape, Shape};
    /// use smithy_model::shape::ShapeBuilderExt;
    ///
    /// let mixin = StructureShape::builder()
    ///     .id("example.foo#MyMixin")
    ///     .mixin()
    ///     .build()
    ///     .unwrap();
    ///
    /// let structure = StructureShape::builder()
    ///     .id("example.foo#MyStructure")
    ///     .mixin(mixin)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn mixin(mut self, mixin: impl Into<Shape>) -> Self {
        self.mixins.push(mixin.into());
        self
    }
    
    /// Build the structure shape.
    pub fn build(self) -> Result<StructureShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;
        
        // Validate mixins
        for mixin in &self.mixins {
            // Check that the mixin has the @mixin trait
            if !mixin.has_trait(Mixin::static_id()) {
                return Err(BuildError::InvalidValue {
                    field: "mixin".to_string(),
                    reason: format!("Shape {} is used as a mixin but does not have the @mixin trait", mixin.id()),
                });
            }
            
            // Check that the mixin is of the same type
            if !matches!(mixin, Shape::Structure(_)) {
                return Err(BuildError::InvalidValue {
                    field: "mixin".to_string(),
                    reason: format!("Mixin {} is not a structure shape", mixin.id()),
                });
            }
        }
        
        // Check for cycles in mixins
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        for mixin in &self.mixins {
            detect_cycles(mixin, &mut visited, &mut path)?;
        }
        
        // Create metadata with introduced traits
        let mut metadata = ShapeMetadata::new(id, self.traits);
        
        // Add mixins to metadata
        for mixin in self.mixins {
            metadata.add_mixin(mixin);
        }
        
        // Compute effective traits
        compute_effective_traits(&mut metadata)?;
        
        // Compute effective members
        let effective_members = compute_effective_members(&metadata, &self.members)?;
        
        Ok(StructureShape {
            metadata,
            members: effective_members,
        })
    }
}
```

## Mixin Resolution Functions

```rust
/// Compute effective traits for a shape, including those from mixins
fn compute_effective_traits(metadata: &mut ShapeMetadata) -> Result<(), BuildError> {
    let mut traits = TraitMap::new();
    
    // Process mixins in order (first to last)
    for mixin in &metadata.mixins {
        // Get traits from the mixin, excluding local traits
        let mixin_traits = get_mixin_traits(mixin)?;
        
        // Apply mixin traits (later mixins override earlier ones)
        for (id, trait_obj) in mixin_traits.iter() {
            traits.insert(trait_obj.clone_trait());
        }
    }
    
    // Apply introduced traits (highest precedence)
    for (id, trait_obj) in metadata.introduced_traits.iter() {
        traits.insert(trait_obj.clone_trait());
    }
    
    // Update effective traits
    metadata.effective_traits = traits;
    
    Ok(())
}

/// Get traits from a mixin, excluding local traits
fn get_mixin_traits(mixin: &Shape) -> Result<TraitMap, BuildError> {
    let mut traits = mixin.introduced_traits().clone();
    
    // Remove the @mixin trait
    traits.remove(Mixin::static_id());
    
    // Remove local traits
    if let Some(mixin_trait) = mixin.get_trait_as::<Mixin>() {
        for local_trait_id in &mixin_trait.local_traits {
            traits.remove(local_trait_id);
        }
    }
    
    Ok(traits)
}

/// Compute effective members for a structure, including those from mixins
fn compute_effective_members(
    metadata: &ShapeMetadata,
    local_members: &HashMap<String, MemberShape>,
) -> Result<HashMap<String, MemberShape>, BuildError> {
    let mut members = HashMap::new();
    
    // Process mixins in order (first to last)
    for mixin in &metadata.mixins {
        match mixin {
            Shape::Structure(structure) => {
                for (name, member) in structure.members() {
                    // Check for conflicts
                    if let Some(existing) = members.get(&name) {
                        if existing.target() != member.target() {
                            return Err(BuildError::InvalidValue {
                                field: "mixin".to_string(),
                                reason: format!(
                                    "Conflicting member {} in mixins: {} vs {}",
                                    name, existing.target(), member.target()
                                ),
                            });
                        }
                    } else {
                        // Create a new member with the target shape's ID
                        let new_member = MemberShape::builder()
                            .id(format!("{}${}", metadata.id, name))
                            .member_name(name.clone())
                            .target(member.target().clone())
                            .build()?;
                        
                        // Copy traits from the original member
                        let mut new_member_with_traits = new_member;
                        for (trait_id, trait_obj) in member.traits().iter() {
                            new_member_with_traits = new_member_with_traits.with_trait(trait_obj.clone_trait());
                        }
                        
                        members.insert(name.clone(), new_member_with_traits);
                    }
                }
            }
            _ => {
                return Err(BuildError::InvalidValue {
                    field: "mixin".to_string(),
                    reason: format!("Mixin {} is not a structure shape", mixin.id()),
                });
            }
        }
    }
    
    // Apply local members (may override mixin members)
    for (name, member) in local_members {
        // Check for conflicts
        if let Some(existing) = members.get(name) {
            if existing.target() != member.target() {
                return Err(BuildError::InvalidValue {
                    field: "member".to_string(),
                    reason: format!(
                        "Member {} conflicts with mixin member: {} vs {}",
                        name, existing.target(), member.target()
                    ),
                });
            }
        }
        members.insert(name.clone(), member.clone());
    }
    
    Ok(members)
}

/// Detect cycles in mixin references
fn detect_cycles(shape: &Shape, visited: &mut HashSet<ShapeId>, path: &mut Vec<ShapeId>) -> Result<(), BuildError> {
    let shape_id = shape.id().clone();
    
    if path.contains(&shape_id) {
        // Found a cycle
        let cycle_path = path.iter()
            .skip_while(|id| **id != shape_id)
            .chain(std::iter::once(&shape_id))
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
        
        return Err(BuildError::InvalidValue {
            field: "mixin".to_string(),
            reason: format!("Cyclic mixin reference detected: {}", cycle_path),
        });
    }
    
    if visited.contains(&shape_id) {
        // Already visited this shape, no cycle through this path
        return Ok(());
    }
    
    visited.insert(shape_id.clone());
    path.push(shape_id);
    
    for mixin in shape.mixins() {
        detect_cycles(mixin, visited, path)?;
    }
    
    path.pop();
    
    Ok(())
}
```

## Example Usage

```rust
// Define a mixin structure
let mixin = StructureShape::builder()
    .id("example.foo#UserInfoMixin")
    .member(
        MemberShape::builder()
            .id("example.foo#UserInfoMixin$userId")
            .member_name("userId")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .required()
            .build()
            .unwrap()
    )
    .mixin()  // Simple utility for the common case
    .build()
    .unwrap();

// For a mixin with local traits, use with_trait directly
let mixin_with_local_traits = StructureShape::builder()
    .id("example.foo#PrivateMixin")
    .with_trait(Mixin { local_traits: vec![Private::static_id().clone()].into() })
    .private()
    .build()
    .unwrap();

// Use the mixin in another structure
let user_details = StructureShape::builder()
    .id("example.foo#UserDetails")
    .mixin(mixin)  // Takes advantage of existing From<StructureShape> for Shape
    .member(
        MemberShape::builder()
            .id("example.foo#UserDetails$alias")
            .member_name("alias")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap()
    )
    .build()
    .unwrap();

// The UserDetails structure now has both members: userId and alias
assert!(user_details.members().contains_key("userId"));
assert!(user_details.members().contains_key("alias"));

// The userId member has the required trait
let user_id_member = user_details.members().get("userId").unwrap();
assert!(user_id_member.has_trait(Required::static_id()));

// The shape has the mixin in its mixins list
assert_eq!(user_details.mixins().len(), 1);
assert_eq!(user_details.mixins()[0].id().to_string(), "example.foo#UserInfoMixin");

// But the shape doesn't have the mixin trait itself
assert!(!user_details.has_trait(Mixin::static_id()));
```

## Key Benefits of This Design

1. **Early Validation**: Mixins are validated during the build process, catching errors early.

2. **Complete Resolution**: Effective traits and members are computed during build, providing a complete view of the shape.

3. **Clear Separation**: Introduced traits vs. effective traits are clearly separated.

4. **Full Shape References**: Storing full shapes for mixins allows for complete validation and resolution.

5. **Consistent API**: The API is consistent with the existing shape design.

## Implementation Considerations

1. **Memory Usage**: Storing full shapes rather than just IDs will use more memory, but provides more complete validation and resolution.

2. **Cycle Detection**: Careful cycle detection is implemented to prevent infinite recursion.

3. **Member Ordering**: Members from mixins come before members defined directly in the shape, following the Smithy specification.

4. **Trait Precedence**: Traits follow the precedence rules defined in the Smithy specification:
   - Traits applied directly to a shape take precedence over traits from mixins
   - Traits from later mixins take precedence over traits from earlier mixins
   - The `@mixin` trait itself is not inherited
   - Traits listed in the `localTraits` property of the `@mixin` trait are not inherited

5. **Special Cases**: Special handling is required for different shape types:
   - Service mixins can define any property
   - Resource mixins cannot define any properties
   - Operation mixins cannot define input or output shapes other than Unit
