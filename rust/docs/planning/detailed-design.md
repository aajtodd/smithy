# Smithy Rust Detailed Design

This document contains detailed design decisions for the Smithy Rust implementation.

## Core Model Representation

### Overview

The core model representation is the foundation of the Smithy Rust implementation, defining how we represent shapes, traits, and their relationships.

### Shape Representation

We will use an enum-based approach to represent Smithy shapes:

```rust
enum Shape {
    Boolean(BooleanShape),
    Byte(ByteShape),
    Short(ShortShape),
    Integer(IntegerShape),
    Long(LongShape),
    Float(FloatShape),
    Double(DoubleShape),
    String(StringShape),
    Blob(BlobShape),
    Timestamp(TimestampShape),
    List(ListShape),
    Set(SetShape),
    Map(MapShape),
    Structure(StructureShape),
    Union(UnionShape),
    Service(ServiceShape),
    Operation(OperationShape),
    Resource(ResourceShape),
    Member(MemberShape),
    // All other shapes defined in the specification
}
```

Each shape type will be a struct containing its specific properties:

```rust
struct BooleanShape {
    id: ShapeId,
    traits: Traits,
    // Any boolean-specific properties
}

struct StringShape {
    id: ShapeId,
    traits: Traits,
    // Any string-specific properties
}

// etc.
```

The `Shape` enum will implement methods for common operations:

```rust
impl Shape {
    pub fn id(&self) -> &ShapeId {
        match self {
            Shape::Boolean(s) => &s.id,
            Shape::String(s) => &s.id,
            // etc.
        }
    }
    
    pub fn traits(&self) -> &Traits {
        match self {
            Shape::Boolean(s) => &s.traits,
            Shape::String(s) => &s.traits,
            // etc.
        }
    }
    
    pub fn shape_type(&self) -> ShapeType {
        match self {
            Shape::Boolean(_) => ShapeType::Boolean,
            Shape::String(_) => ShapeType::String,
            // etc.
        }
    }
}
```

We will also use traits for grouping common functionality among related shapes:

```rust
trait CollectionShape {
    fn member(&self) -> &MemberShape;
}

impl CollectionShape for ListShape {
    fn member(&self) -> &MemberShape {
        &self.member
    }
}

impl CollectionShape for SetShape {
    fn member(&self) -> &MemberShape {
        &self.member
    }
}
```

With helper methods on the enum to work with these traits:

```rust
impl Shape {
    pub fn as_collection(&self) -> Option<&dyn CollectionShape> {
        match self {
            Shape::List(s) => Some(s as &dyn CollectionShape),
            Shape::Set(s) => Some(s as &dyn CollectionShape),
            _ => None,
        }
    }
}
```

#### Alternative Approaches Considered

1. **Trait-Based Polymorphism**

```rust
trait Shape {
    fn id(&self) -> &ShapeId;
    fn shape_type(&self) -> ShapeType;
    fn traits(&self) -> &Traits;
    // Common methods for all shapes
}

struct BooleanShape { /* ... */ }
struct StringShape { /* ... */ }
// etc.

impl Shape for BooleanShape { /* ... */ }
impl Shape for StringShape { /* ... */ }
// etc.
```

**Pros:**
- More extensible for custom shape types
- Cleaner interface for common operations
- More closely mirrors the Java implementation's class hierarchy

**Cons:**
- Requires dynamic dispatch (Box<dyn Shape>)
- More complex ownership model
- Potentially less efficient

2. **Hybrid Approach**

```rust
enum ShapeKind {
    Boolean(BooleanShape),
    String(StringShape),
    // etc.
}

struct ShapeContainer {
    id: ShapeId,
    traits: Traits,
    kind: ShapeKind,
}

trait ShapeExt {
    // Extension methods for specific shape operations
}

impl ShapeExt for ShapeContainer {
    // Implementations that delegate to the appropriate kind
}
```

**Pros:**
- Combines benefits of both approaches
- Maintains type safety while allowing for common interfaces
- Can optimize for common operations

**Cons:**
- More complex implementation
- Potential for duplication between enum and trait methods

3. **Type-Erased Approach with Downcasting**

```rust
struct Shape {
    id: ShapeId,
    shape_type: ShapeType,
    traits: Traits,
    data: Box<dyn Any>,
}

impl Shape {
    fn as_boolean(&self) -> Option<&BooleanShape> {
        self.data.downcast_ref::<BooleanShape>()
    }
    // Similar methods for other shape types
}
```

**Pros:**
- Single concrete type for all shapes
- Flexible internal representation
- Can be extended with new shape types

**Cons:**
- Runtime type checking
- Less type safety
- More complex error handling

#### Rationale for Chosen Approach

We chose the enum-based approach for the following reasons:

1. **Rust Idiomatic Design**: This approach leverages Rust's strengths in algebraic data types and pattern matching.

2. **Performance**: Enums provide better performance characteristics than trait objects, avoiding dynamic dispatch overhead.

3. **Type Safety**: The enum approach provides compile-time type safety and exhaustive pattern matching.

4. **Memory Efficiency**: This approach can be more memory-efficient, which is important for handling large models.

5. **Closed Set of Types**: Since the shape types are fixed by the Smithy specification, we don't need the extensibility that trait objects would provide.

6. **Serialization Support**: Enums work well with serde for serialization/deserialization.

7. **Simplicity**: A pure enum approach is simpler and more straightforward than hybrid approaches.

### Shape References

For references between shapes, we will use an identifier-based approach:

```rust
// Shape identifiers
#[derive(Clone, PartialEq, Eq, Hash)]
struct ShapeId {
    namespace: String,
    name: String,
    member: Option<String>,
}

// Shape definitions with ID-based references
struct ListShape {
    id: ShapeId,
    traits: Traits,
    member: ShapeId,  // Reference to member shape by ID
}

struct StructureShape {
    id: ShapeId,
    traits: Traits,
    members: HashMap<String, ShapeId>,  // References to member shapes by ID
}

// Model container with lookup methods
struct Model {
    shapes: HashMap<ShapeId, Shape>,
}

impl Model {
    pub fn get_shape(&self, id: &ShapeId) -> Option<&Shape> {
        self.shapes.get(id)
    }
    
    // Helper methods for common traversals
    pub fn get_list_member(&self, list: &ListShape) -> Option<&Shape> {
        self.get_shape(&list.member)
    }
    
    pub fn get_structure_member(&self, structure: &StructureShape, name: &str) -> Option<&Shape> {
        structure.members.get(name).and_then(|id| self.get_shape(id))
    }
}
```

#### Alternative Approaches Considered

1. **Rust References with Arena Allocation**

```rust
struct ListShape<'a> {
    id: ShapeId,
    traits: Traits,
    member: &'a Shape<'a>,  // Direct reference to member shape
}

// Using a crate like typed-arena
struct Model<'a> {
    arena: Arena<Shape<'a>>,
    shapes: HashMap<ShapeId, &'a Shape<'a>>,
}
```

**Pros:**
- Direct references are more ergonomic to use
- Better performance for traversing the model
- Still maintains memory safety

**Cons:**
- Complex lifetime management
- Difficult to modify the model after creation
- Serialization/deserialization is more complex
- Arena allocation adds complexity

2. **Reference Counting**

```rust
struct ListShape {
    id: ShapeId,
    traits: Traits,
    member: Rc<Shape>,  // Reference-counted pointer to member shape
}

struct Model {
    shapes: HashMap<ShapeId, Rc<Shape>>,
}
```

**Pros:**
- Direct references without complex lifetime management
- Can modify the model after creation (with some care)
- More ergonomic for traversing the model

**Cons:**
- Potential for reference cycles (would need Weak references)
- Performance overhead of reference counting
- More complex ownership model
- Serialization/deserialization is more complex

3. **Hybrid Approach**

```rust
struct ListShape {
    id: ShapeId,
    traits: Traits,
    member_id: ShapeId,  // Always store the ID
    member: Option<ShapeRef>,  // Cached reference, populated on demand
}

enum ShapeRef {
    Direct(&'static Shape),  // For static shapes
    Rc(Rc<Shape>),          // For dynamic shapes
    Weak(Weak<Shape>),      // For potential cycles
}
```

**Pros:**
- Combines benefits of identifier-based and reference-based approaches
- Can optimize for common traversal patterns
- Flexible for different use cases

**Cons:**
- Most complex implementation
- Multiple ways to access the same data
- Potential for inconsistency if not managed carefully

#### Rationale for Chosen Approach

We chose the identifier-based approach for the following reasons:

1. **Simplicity**: This approach has the simplest ownership model and avoids complex lifetime management.

2. **Avoids Cycles**: It completely avoids the problem of reference cycles.

3. **Serialization**: It makes serialization and deserialization straightforward.

4. **Conceptual Match**: It aligns with the Smithy conceptual model where shapes reference each other by ID.

5. **Immutability**: It works well with an immutable model design, which is generally safer and easier to reason about.

6. **Performance Optimization**: We can add caching or indexing to optimize common lookup patterns if needed.

### Trait Implementation

Traits in Smithy are metadata attached to shapes and are identified by their ShapeId. Our implementation needs to balance flexibility, type safety, and performance.

#### Design Decisions

We will implement a hybrid approach that supports both dynamic model loading and type-safe programmatic model building:

```rust
// Core trait representation
pub struct Trait {
    id: TraitId,
    value: TraitValue,
}

// Trait value representation
pub enum TraitValue {
    Bool(bool),
    String(String),
    Number(f64),
    Array(Vec<TraitValue>),
    Object(HashMap<String, TraitValue>),
}

// Container for traits attached to a shape
pub struct TraitContainer {
    traits: HashMap<TraitId, Trait>,
}

impl TraitContainer {
    // Dynamic API for model loading
    pub fn apply(&mut self, trait_: Trait) -> Result<(), TraitApplicationError> {
        // Implementation
        self.traits.insert(trait_.id.clone(), trait_);
        Ok(())
    }
    
    // Generic getter
    pub fn get(&self, id: &TraitId) -> Option<&Trait> {
        self.traits.get(id)
    }
    
    // Type-safe API for programmatic use
    pub fn apply_typed<T: SmithyTrait>(&mut self, trait_value: T) -> Result<(), TraitApplicationError> {
        self.apply(Trait::new(T::trait_id(), trait_value.as_trait_value()))
    }
    
    // Type-safe getter for programmatic use
    pub fn get_typed<T: SmithyTrait>(&self) -> Option<T> {
        self.get(&T::trait_id()).and_then(|t| T::from_trait(t))
    }
    
    // Common trait accessors
    pub fn documentation(&self) -> Option<String> {
        let doc_id = TraitId::new_builtin("documentation");
        self.get(&doc_id).and_then(|t| {
            if let TraitValue::String(s) = &t.value {
                Some(s.clone())
            } else {
                None
            }
        })
    }
    
    pub fn is_required(&self) -> bool {
        let required_id = TraitId::new_builtin("required");
        self.traits.contains_key(&required_id)
    }
}

// Trait for type-safe trait implementations
pub trait SmithyTrait: Sized {
    fn trait_id() -> TraitId;
    fn as_trait_value(&self) -> TraitValue;
    fn from_trait(trait_: &Trait) -> Option<Self>;
}

// Example implementation for a specific trait
pub struct Documentation(String);

impl SmithyTrait for Documentation {
    fn trait_id() -> TraitId {
        TraitId::new_builtin("documentation")
    }
    
    fn as_trait_value(&self) -> TraitValue {
        TraitValue::String(self.0.clone())
    }
    
    fn from_trait(trait_: &Trait) -> Option<Self> {
        if let TraitValue::String(s) = &trait_.value {
            Some(Documentation(s.clone()))
        } else {
            None
        }
    }
}
```

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

#### Serialization and Deserialization

For serialization and deserialization, we'll use serde:

```rust
// For JSON AST parsing
impl ModelLoader {
    fn parse_shape_from_json(&mut self, json: &serde_json::Value) -> Result<(), LoaderError> {
        let shape_id = json.get("id")
            .and_then(|v| v.as_str())
            .ok_or(LoaderError::MissingShapeId)?;
        
        let shape_type = json.get("type")
            .and_then(|v| v.as_str())
            .ok_or(LoaderError::MissingShapeType)?;
        
        let mut shape = self.create_shape(shape_id, shape_type)?;
        
        // Parse traits (prefixed with $ in JSON AST)
        for (key, value) in json.as_object().unwrap() {
            if key.starts_with('$') {
                let trait_id = TraitId::from_str(&key[1..])?; // Remove $ prefix
                let trait_value = self.parse_trait_value(value)?;
                shape.apply_trait(Trait::new(trait_id, trait_value));
            }
        }
        
        self.shapes.insert(shape.id().clone(), shape);
        Ok(())
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

#### Rationale for Chosen Approach

We chose the hybrid approach for the following reasons:

1. **Flexibility**: It supports both dynamic loading from files and type-safe programmatic model building.

2. **Simplicity**: The core representation is straightforward and easy to understand.

3. **Separation of Concerns**: Parsing is separated from validation, allowing for better error reporting.

4. **Performance**: The approach minimizes allocations and copies while still providing type safety where possible.

5. **Extensibility**: It's easy to add new traits and trait validators.

6. **Compatibility**: The approach aligns well with the Smithy specification and can handle all trait types.

7. **Ergonomics**: It provides convenient accessors for common traits while still supporting generic access.

### Model Container Design

The Model container is a key component of the core model representation, responsible for organizing, indexing, and providing access to shapes.

#### Design Decisions

We will implement an immutable Model container with efficient lookup capabilities:

```rust
pub struct Model {
    // Primary storage of shapes
    shapes: HashMap<ShapeId, Shape>,
    
    // Indexes for efficient lookups
    namespace_index: HashMap<String, Vec<ShapeId>>,
    shape_type_index: HashMap<ShapeType, Vec<ShapeId>>,
    
    // Metadata
    smithy_version: String,
    metadata: HashMap<String, MetadataValue>,
}

impl Model {
    // Basic shape access
    pub fn get_shape(&self, id: &ShapeId) -> Option<&Shape> {
        self.shapes.get(id)
    }
    
    // Namespace-based queries
    pub fn get_shapes_in_namespace(&self, namespace: &str) -> Vec<&Shape> {
        self.namespace_index.get(namespace)
            .map_or(Vec::new(), |ids| {
                ids.iter()
                   .filter_map(|id| self.get_shape(id))
                   .collect()
            })
    }
    
    // Type-based queries
    pub fn get_shapes_of_type(&self, shape_type: ShapeType) -> Vec<&Shape> {
        self.shape_type_index.get(&shape_type)
            .map_or(Vec::new(), |ids| {
                ids.iter()
                   .filter_map(|id| self.get_shape(id))
                   .collect()
            })
    }
    
    // Helper methods for common operations
    pub fn get_services(&self) -> Vec<&ServiceShape> {
        self.get_shapes_of_type(ShapeType::Service)
            .into_iter()
            .filter_map(|shape| {
                if let Shape::Service(service) = shape {
                    Some(service)
                } else {
                    None
                }
            })
            .collect()
    }
}
```

For model construction and transformation, we'll use a builder pattern:

```rust
pub struct ModelBuilder {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, MetadataValue>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn from(model: &Model) -> Self {
        Self {
            shapes: model.shapes.clone(),
            metadata: model.metadata.clone(),
        }
    }
    
    pub fn add_shape(&mut self, shape: Shape) -> Result<&mut Self, ModelError> {
        let id = shape.id().clone();
        if self.shapes.contains_key(&id) {
            return Err(ModelError::DuplicateShape(id));
        }
        self.shapes.insert(id, shape);
        Ok(self)
    }
    
    pub fn build(self) -> Result<Model, ModelError> {
        // Build indexes and validate before returning
        // ...
    }
}

// Example of model transformation
impl Model {
    pub fn with_added_shape(&self, shape: Shape) -> Result<Model, ModelError> {
        let mut builder = ModelBuilder::from(self);
        builder.add_shape(shape)?;
        builder.build()
    }
}
```

#### Alternative Approaches Considered

1. **Mutable Model**

```rust
pub struct Model {
    shapes: HashMap<ShapeId, Shape>,
    // Other fields
}

impl Model {
    pub fn add_shape(&mut self, shape: Shape) -> Result<(), ModelError> {
        // Implementation
    }
    
    pub fn remove_shape(&mut self, id: &ShapeId) {
        // Implementation
    }
}
```

**Pros:**
- Simpler API for modifications
- More efficient for multiple changes
- Familiar to users of other mutable APIs

**Cons:**
- Thread safety concerns
- More complex reasoning about state
- Potential for inconsistent state
- Need for validation after changes

2. **Hybrid Approach with Internal Mutability**

```rust
pub struct Model {
    inner: RefCell<ModelInner>,
}

impl Model {
    pub fn add_shape(&self, shape: Shape) -> Result<(), ModelError> {
        self.inner.borrow_mut().add_shape(shape)
    }
}
```

**Pros:**
- Appears immutable from outside
- Can optimize internal operations
- Familiar API

**Cons:**
- Runtime borrow checking
- Potential panics
- Still has thread safety issues
- More complex implementation

#### Rationale for Chosen Approach

We chose the immutable model with builder pattern for the following reasons:

1. **Thread Safety**: The immutable model can be safely shared across threads without locks.

2. **Reasoning**: It's easier to reason about code when models can't change unexpectedly.

3. **Consistency**: The model is always in a valid state after construction.

4. **Caching**: Results of computations can be safely cached without invalidation concerns.

5. **Functional Style**: Aligns well with functional programming patterns in Rust.

6. **Performance**: While transformations require copying, most use cases involve reading rather than modifying models.

7. **Simplicity**: Avoids complex state management and validation logic after modifications.

## Parser Design

### Overview

The Smithy Rust parser will convert Smithy IDL and JSON AST files into model objects. The parser will be implemented using Logos for lexing, LALRPOP for parsing, and Ariadne for error reporting.

### Key Components

1. **Lexer Implementation with Logos:**
   - Define token types based on Smithy specification
   - Preserve comments and source location information
   - Handle whitespace appropriately

2. **Parser Implementation with LALRPOP:**
   - Define grammar rules based on Smithy specification
   - Optimize for Rust while maintaining clear mapping to specification
   - Generate a clean AST structure

3. **AST Design:**
   - Create idiomatic Rust enums and structs
   - Include source location information for error reporting
   - Preserve comments for potential formatters

4. **Model Building:**
   - Implement conversion from AST to final model objects
   - Resolve references between shapes
   - Perform initial validation

5. **Error Handling:**
   - Implement detailed error reporting with Ariadne
   - Categorize errors for better user experience
   - Include context in error messages

### Design Decisions

#### AST Structure
- Implement a separate AST representation from the final model
- This separation supports maintainability and potential future formatting utilities
- Resolve shape references during the model building phase, not during parsing

#### Trait Handling
- Represent trait values generically in the AST
- Defer validation beyond syntax checking to the model building/validation phase

#### Error Reporting
- Include surrounding context in error messages
- Implement error categorization (syntax vs. semantic errors)
- Leverage Ariadne for rich error diagnostics

#### Component Structure
- Separate lexing (using Logos), parsing (using LALRPOP), and AST construction
- Design as distinct components even if they operate in a single pass

#### Performance Measurement
- Benchmark AST construction and model loading separately
- Track memory usage
- Use Criterion crate for benchmarking
- No immediate need to benchmark against Java implementation

#### Documentation
- Document primarily for Rust developers based on the specification
- Add cross-references to Java implementation where helpful for context

#### Large File Handling
- Design with large model files in mind from the beginning

### Testing Strategy
- Reuse test models from Java implementation
- Add Rust-specific unit and integration tests
- Implement fuzzing tests for the parser

### Implementation Phases

1. **Phase 1: Basic Lexer and Parser**
   - Implement token definitions with Logos
   - Create basic LALRPOP grammar for core Smithy syntax
   - Generate simple AST structures

2. **Phase 2: Complete Parser**
   - Extend grammar to cover all Smithy syntax
   - Implement comprehensive AST
   - Add source tracking and error reporting

3. **Phase 3: Model Building**
   - Implement conversion from AST to model objects
   - Add reference resolution
   - Implement basic validation

4. **Phase 4: Optimization and Testing**
   - Optimize parser performance
   - Implement comprehensive test suite
   - Add benchmarks


### TODO: Serialization

- How should we handle serialization/deserialization of the model?
- Should we implement serde traits for the model components?

### TODO: Metadata

- How should we store metadata about the model (version, source files, etc.)?
- Should metadata be part of the model or separate?

### TODO: Errors

- How should errors be represented?

## Validation Framework

### Overview

The validation framework ensures that Smithy models conform to the specification and any additional validation rules. It needs to be extensible, provide clear error messages, and support different validation severity levels.

### Design Decisions

We will implement a trait-based validation framework with support for custom validators:

```rust
#[derive(Clone, Debug)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
    Danger,  // For potentially breaking changes
}

#[derive(Clone, Debug)]
pub struct ValidationEvent {
    id: String,           // Unique identifier for the validation rule
    severity: ValidationSeverity,
    message: String,      // Human-readable message
    shape_id: Option<ShapeId>,  // Associated shape if applicable
    location: Option<SourceLocation>,  // Source location if available
    related_events: Vec<ValidationEvent>,  // For related issues
}

impl ValidationEvent {
    pub fn error(message: impl Into<String>, shape_id: ShapeId) -> Self {
        Self {
            id: "".to_string(),  // Can be set later
            severity: ValidationSeverity::Error,
            message: message.into(),
            shape_id: Some(shape_id),
            location: None,
            related_events: Vec::new(),
        }
    }
    
    // Similar constructors for warning, info, etc.
}

pub trait Validator {
    fn validate(&self, model: &Model) -> Vec<ValidationEvent>;
    
    fn id(&self) -> &str;
    
    fn description(&self) -> &str {
        "No description provided"
    }
    
    // Method to configure the validator from metadata
    fn configure(&mut self, _config: &MetadataValue) {
        // Default implementation does nothing
        // Specific validators can override this to handle configuration
    }
}

pub struct ValidatorRegistry {
    validators: HashMap<String, Box<dyn Validator>>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }
    
    pub fn register<V: Validator + 'static>(&mut self, validator: V) {
        self.validators.insert(validator.id().to_string(), Box::new(validator));
    }
    
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Box<dyn Validator>> {
        self.validators.get_mut(id)
    }
    
    pub fn validate(&self, model: &Model) -> Vec<ValidationEvent> {
        let mut events = Vec::new();
        
        for validator in self.validators.values() {
            events.extend(validator.validate(model));
        }
        
        events
    }
}

impl Model {
    pub fn validate(&self) -> Vec<ValidationEvent> {
        let mut registry = self.create_default_validator_registry();
        
        // Add validators from metadata
        self.add_metadata_validators(&mut registry);
        
        registry.validate(self)
    }
    
    pub fn validate_with(&self, registry: &ValidatorRegistry) -> Vec<ValidationEvent> {
        registry.validate(self)
    }
    
    fn create_default_validator_registry(&self) -> ValidatorRegistry {
        let mut registry = ValidatorRegistry::new();
        
        // Register built-in validators
        registry.register(ShapeValidators::new());
        registry.register(TraitValidators::new());
        registry.register(ServiceValidators::new());
        // etc.
        
        registry
    }
    
    fn add_metadata_validators(&self, registry: &mut ValidatorRegistry) {
        // Check for validators in metadata
        if let Some(MetadataValue::Object(validators)) = self.metadata.get("validators") {
            for (validator_id, config) in validators {
                // Handle built-in validators with custom configuration
                if let Some(existing) = registry.get_mut(validator_id) {
                    existing.configure(config);
                }
                
                // Handle custom validators specified in metadata
                if let Some(validator) = self.create_validator_from_metadata(validator_id, config) {
                    registry.register(validator);
                }
            }
        }
    }
    
    fn create_validator_from_metadata(&self, id: &str, config: &MetadataValue) -> Option<Box<dyn Validator>> {
        // Implementation would depend on how custom validators are registered
        None
    }
}

pub struct ValidationResult {
    events: Vec<ValidationEvent>,
}

impl ValidationResult {
    pub fn new(events: Vec<ValidationEvent>) -> Self {
        Self { events }
    }
    
    pub fn has_errors(&self) -> bool {
        self.events.iter().any(|e| matches!(e.severity, ValidationSeverity::Error))
    }
    
    pub fn errors(&self) -> impl Iterator<Item = &ValidationEvent> {
        self.events.iter().filter(|e| matches!(e.severity, ValidationSeverity::Error))
    }
    
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationEvent> {
        self.events.iter().filter(|e| matches!(e.severity, ValidationSeverity::Warning))
    }
    
    pub fn format_report(&self) -> String {
        // Format a human-readable report of validation issues
        let mut report = String::new();
        
        let errors: Vec<_> = self.errors().collect();
        let warnings: Vec<_> = self.warnings().collect();
        
        if !errors.is_empty() {
            report.push_str(&format!("Found {} validation errors:\n", errors.len()));
            for (i, error) in errors.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, error.message));
                if let Some(shape_id) = &error.shape_id {
                    report.push_str(&format!("   Shape: {}\n", shape_id));
                }
            }
        }
        
        if !warnings.is_empty() {
            if !report.is_empty() {
                report.push('\n');
            }
            report.push_str(&format!("Found {} validation warnings:\n", warnings.len()));
            for (i, warning) in warnings.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, warning.message));
                if let Some(shape_id) = &warning.shape_id {
                    report.push_str(&format!("   Shape: {}\n", shape_id));
                }
            }
        }
        
        if report.is_empty() {
            report.push_str("No validation issues found.");
        }
        
        report
    }
}
```

#### Example Built-in Validator

```rust
// Example of a built-in validator
pub struct ShapeValidators;

impl ShapeValidators {
    pub fn new() -> Self {
        Self
    }
}

impl Validator for ShapeValidators {
    fn validate(&self, model: &Model) -> Vec<ValidationEvent> {
        let mut events = Vec::new();
        
        // Validate each shape
        for shape in model.shapes() {
            // Example validation: Check for empty structure members
            if let Shape::Structure(structure) = shape {
                if structure.members().is_empty() {
                    events.push(ValidationEvent::error(
                        "Structure must have at least one member",
                        structure.id().clone(),
                    ));
                }
            }
            
            // More validations...
        }
        
        events
    }
    
    fn id(&self) -> &str {
        "smithy.validators.shape"
    }
    
    fn description(&self) -> &str {
        "Validates shape constraints defined in the Smithy specification"
    }
}
```

### Alternative Approaches Considered

#### 1. Validator Functions Instead of Trait Objects

```rust
type ValidatorFn = fn(&Model) -> Vec<ValidationEvent>;

pub struct ValidatorRegistry {
    validators: Vec<(String, ValidatorFn)>,
}
```

**Pros:**
- No dynamic dispatch overhead
- Simpler implementation

**Cons:**
- Less flexibility for validators that need state
- Harder to implement complex validators
- Less object-oriented, which might make it harder to organize related validation logic

#### 2. Validation During Model Construction

```rust
impl ModelBuilder {
    pub fn build(self) -> Result<Model, ValidationError> {
        let model = self.create_model();
        let events = model.validate();
        
        if events.iter().any(|e| matches!(e.severity, ValidationSeverity::Error)) {
            Err(ValidationError::new(events))
        } else {
            Ok(model)
        }
    }
}
```

**Pros:**
- Ensures models are always valid after construction
- Prevents invalid models from being created

**Cons:**
- Less flexibility for use cases that need to work with invalid models
- May be inefficient for incremental validation during editing
- Complicates the model construction process

#### 3. Visitor Pattern for Validation

```rust
pub trait ValidationVisitor {
    fn visit_structure(&mut self, structure: &StructureShape) -> Vec<ValidationEvent>;
    fn visit_service(&mut self, service: &ServiceShape) -> Vec<ValidationEvent>;
    // Methods for other shape types
}

pub struct Validator {
    visitors: Vec<Box<dyn ValidationVisitor>>,
}
```

**Pros:**
- Clear separation of validation logic by shape type
- More structured approach to validation
- Can optimize validation by shape type

**Cons:**
- More complex implementation
- Requires more boilerplate code
- Less flexible for validations that span multiple shape types

### Rationale for Chosen Approach

We chose the trait-based validator approach for the following reasons:

1. **Extensibility**: The trait-based approach makes it easy to add new validators, both built-in and custom.

2. **Organization**: Validators can be organized logically by validation domain (shapes, traits, services, etc.).

3. **State Management**: Validators can maintain state if needed for complex validations.

4. **Simplicity**: The API is straightforward and easy to understand.

5. **Flexibility**: The approach supports different validation strategies (validate all at once, validate incrementally, etc.).

6. **Separation of Concerns**: Validation is separate from model construction, allowing for more flexible use cases.

7. **Compatibility**: The approach aligns well with how validation works in the Java implementation.

8. **Metadata Support**: The design accommodates validators specified in model metadata, allowing for customization of validation behavior.

### TODO: Validation

- How should we handle validation of specific shape types?
- Should we implement a more specialized validation framework for traits?
