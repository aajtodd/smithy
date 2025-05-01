# Smithy Rust Model Container Design

## 1. Overview

### Purpose and Scope

The Smithy Model container is a core component of the Smithy Rust implementation that represents the semantic model of Smithy. It serves as the central data structure that holds all shapes, metadata, and relationships that make up a Smithy model.

### Key Design Principles

- **Immutability**: The model is immutable once created, making it easier to reason about and thread-safe
- **Ownership**: The model owns all shapes rather than using references
- **Extensibility**: The design supports extension through knowledge indexes and validation
- **Performance**: The model provides efficient access patterns and supports caching
- **Idiomatic Rust**: The API follows Rust conventions and leverages Rust's type system

### Relationship to Other Components

The Model container interacts with several other components:
- **Shape System**: Provides the shapes that are stored in the model
- **Parser**: Converts Smithy IDL and JSON AST into model objects
- **Validation Framework**: Validates the model against the Smithy specification
- **Code Generation**: Uses the model to generate code

## 2. Core Model Structure

### Model Struct Definition

```rust
pub struct Model {
    // Primary storage of shapes
    shapes: HashMap<ShapeId, Shape>,
    
    // Model metadata
    metadata: HashMap<String, Node>,
    
    // Optional caching fields for knowledge indexes
    // Using OnceCell for thread-safe lazy initialization
    knowledge_indexes: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}
```

### Ownership and Lifetime Considerations

The Model owns all shapes and metadata. This simplifies the API and avoids complex lifetime parameters. When shapes are added to the model, they are moved into the model's ownership.

### Thread Safety

The Model is designed to be thread-safe through:
- Immutability of the core data structures
- Thread-safe lazy initialization of knowledge indexes using `RwLock`
- Rust's ownership system to prevent data races

## 3. Model API

### Core Access Methods

```rust
impl Model {
    // Basic shape lookup
    pub fn get_shape(&self, id: impl AsRef<ShapeId>) -> Option<&Shape> {
        self.shapes.get(id.as_ref())
    }
    
    // Shape lookup that panics if not found
    pub fn expect_shape(&self, id: impl AsRef<ShapeId>) -> &Shape {
        self.get_shape(id.as_ref()).expect(&format!("Shape not found: {}", id.as_ref()))
    }
    
    // Type-safe shape lookup
    pub fn expect_shape_of_type<T: Shape>(&self, id: impl AsRef<ShapeId>) -> &T {
        let shape = self.expect_shape(id);
        shape.as_shape_type::<T>().expect(&format!(
            "Expected shape {} to be of type {}, but was {}",
            id.as_ref(),
            std::any::type_name::<T>(),
            shape.shape_type()
        ))
    }
    
    // Access metadata
    pub fn metadata(&self) -> &HashMap<String, Node> {
        &self.metadata
    }
    
    // Get specific metadata property
    pub fn metadata_property(&self, name: impl AsRef<str>) -> Option<&Node> {
        self.metadata.get(name.as_ref())
    }
    
    // Static methods to create builders and assemblers
    pub fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }
    
    pub fn assembler() -> ModelAssembler {
        ModelAssembler::new()
    }
    
    // Convert model back to builder
    pub fn to_builder(self) -> ModelBuilder {
        ModelBuilder {
            shapes: self.shapes,
            metadata: self.metadata,
        }
    }
}
```

### Shape Iteration with Shapes Iterator

```rust
impl Model {
    // Return an iterator over all shapes
    pub fn shapes(&self) -> Shapes<'_> {
        Shapes::new(self)
    }
}

pub struct Shapes<'a> {
    model: &'a Model,
    filter: Option<Box<dyn Fn(&Shape) -> bool + 'a>>,
}

impl<'a> Shapes<'a> {
    fn new(model: &'a Model) -> Self {
        Self {
            model,
            filter: None,
        }
    }
    
    // Filter shapes by type
    pub fn with_type<T: Shape>(&self) -> Self {
        let filter = self.filter.clone();
        Self {
            model: self.model,
            filter: Some(Box::new(move |shape| {
                shape.is::<T>() && filter.as_ref().map_or(true, |f| f(shape))
            })),
        }
    }
    
    // Filter shapes by trait
    pub fn with_trait(&self, trait_id: impl AsRef<ShapeId>) -> Self {
        let trait_id = trait_id.as_ref().clone();
        let filter = self.filter.clone();
        Self {
            model: self.model,
            filter: Some(Box::new(move |shape| {
                shape.has_trait(&trait_id) && filter.as_ref().map_or(true, |f| f(shape))
            })),
        }
    }
}

impl<'a> Iterator for Shapes<'a> {
    type Item = &'a Shape;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Implementation would iterate through shapes and apply filters
    }
}
```

### Knowledge Index Integration

```rust
pub trait KnowledgeIndex: 'static {
    fn from_model(model: &Model) -> Self;
}

impl Model {
    pub fn knowledge<T: KnowledgeIndex>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        
        // Try to get from cache first
        let read_guard = self.knowledge_indexes.read().unwrap();
        if let Some(index) = read_guard.get(&type_id) {
            if let Some(index) = index.downcast_ref::<T>() {
                return index;
            }
        }
        drop(read_guard);
        
        // Not in cache, create new
        let index = T::from_model(self);
        let mut write_guard = self.knowledge_indexes.write().unwrap();
        let index_ref = write_guard
            .entry(type_id)
            .or_insert_with(|| Box::new(index) as Box<dyn Any + Send + Sync>);
        
        // Downcast and return
        index_ref.downcast_ref::<T>().unwrap()
    }
}
```

## 4. ModelBuilder

### Builder Pattern Implementation

```rust
pub struct ModelBuilder {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, Node>,
}

impl ModelBuilder {
    pub(crate) fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    // Fluent builder methods
    
    pub fn add_shape(mut self, shape: impl Into<Shape>) -> Self {
        let shape = shape.into();
        self.put_shape(shape);
        self
    }
    
    pub fn add_shapes<I>(mut self, shapes: I) -> Self 
    where
        I: IntoIterator,
        I::Item: Into<Shape>,
    {
        for shape in shapes {
            self.put_shape(shape.into());
        }
        self
    }
    
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<Node>) -> Self {
        self.put_metadata(key, value);
        self
    }
    
    pub fn build(self) -> Result<Model, ModelError> {
        // Validate shapes before building
        self.validate_shapes()?;
        
        Ok(Model {
            shapes: self.shapes,
            metadata: self.metadata,
            knowledge_indexes: RwLock::new(HashMap::new()),
        })
    }
    
    // Non-fluent builder methods
    
    pub fn put_shape(&mut self, shape: Shape) {
        let id = shape.id().clone();
        
        // If shape is not a member shape, add it directly
        if !shape.is_member_shape() {
            self.shapes.insert(id, shape);
            
            // Add all member shapes automatically
            if let Some(members) = shape.members() {
                for member in members {
                    let member_id = member.id().clone();
                    self.shapes.insert(member_id, member);
                }
            }
        }
    }
    
    pub fn put_metadata(&mut self, key: impl Into<String>, value: impl Into<Node>) {
        self.metadata.insert(key.into(), value.into());
    }
    
    pub fn remove_shape(&mut self, id: impl AsRef<ShapeId>) {
        let id = id.as_ref();
        
        // If shape exists and is not a member shape
        if let Some(shape) = self.shapes.remove(id) {
            // Remove all member shapes as well
            if let Some(members) = shape.members() {
                for member in members {
                    self.shapes.remove(member.id());
                }
            }
        }
    }
    
    pub fn remove_metadata(&mut self, key: impl AsRef<str>) {
        self.metadata.remove(key.as_ref());
    }
    
    pub fn shapes(&self) -> Shapes<'_> {
        Shapes::new_from_map(&self.shapes)
    }
    
    // Validation helper
    fn validate_shapes(&self) -> Result<(), ModelError> {
        // Validate that all shape references point to shapes in the model
        // Validate that member shapes belong to their parent shapes
        // Other validations as needed
        Ok(())
    }
}
```

## 5. ModelAssembler

### Assembler Pattern Implementation

```rust
pub struct ModelAssembler {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, Node>,
    properties: HashMap<String, Node>,
    enable_prelude: bool,
    enable_validation: bool,
    validator_registry: Option<ValidatorRegistry>,
    trait_registry: Option<TraitRegistry>,
    imports: Vec<PathBuf>,
}

impl ModelAssembler {
    // Constants for common properties
    pub const ALLOW_UNKNOWN_TRAITS: &'static str = "allowUnknownTraits";
    
    pub(crate) fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            metadata: HashMap::new(),
            properties: HashMap::new(),
            enable_prelude: true,
            enable_validation: true,
            validator_registry: None,
            trait_registry: None,
            imports: Vec::new(),
        }
    }
    
    // Fluent builder methods
    
    pub fn add_shape(mut self, shape: impl Into<Shape>) -> Self {
        let shape = shape.into();
        self.put_shape(shape);
        self
    }
    
    pub fn add_shapes<I>(mut self, shapes: I) -> Self 
    where
        I: IntoIterator,
        I::Item: Into<Shape>,
    {
        for shape in shapes {
            self.put_shape(shape.into());
        }
        self
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<Node>) -> Self {
        self.put_metadata(key, value);
        self
    }
    
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<Node>) -> Self {
        self.put_property(key, value);
        self
    }
    
    pub fn enable_prelude(mut self, enabled: bool) -> Self {
        self.enable_prelude = enabled;
        self
    }
    
    pub fn enable_validation(mut self, enabled: bool) -> Self {
        self.enable_validation = enabled;
        self
    }
    
    pub fn add_model(mut self, model: Model) -> Self {
        // Add all shapes from the model
        for shape in model.shapes() {
            self.put_shape(shape.clone());
        }
        
        // Merge metadata
        for (key, value) in model.metadata() {
            self.metadata.insert(key.clone(), value.clone());
        }
        
        self
    }
    
    pub fn with_import(mut self, path: impl Into<PathBuf>) -> Self {
        self.imports.push(path.into());
        self
    }
    
    pub fn with_imports<I>(mut self, paths: I) -> Self 
    where
        I: IntoIterator,
        I::Item: Into<PathBuf>,
    {
        for path in paths {
            self.imports.push(path.into());
        }
        self
    }
    
    pub fn add_validator(mut self, validator: impl Validator) -> Self {
        let registry = self.validator_registry.get_or_insert_with(ValidatorRegistry::new);
        registry.add_validator(validator);
        self
    }
    
    pub fn with_validator_registry(mut self, registry: ValidatorRegistry) -> Self {
        self.validator_registry = Some(registry);
        self
    }
    
    pub fn with_trait_registry(mut self, registry: TraitRegistry) -> Self {
        self.trait_registry = Some(registry);
        self
    }
    
    pub fn assemble(&self) -> Result<Model, ValidationError> {
        // 1. Load models from imports
        let mut builder = ModelBuilder::new();
        
        // 2. Add shapes from imports
        for path in &self.imports {
            let imported_model = self.load_model_from_path(path)?;
            for shape in imported_model.shapes() {
                builder.put_shape(shape.clone());
            }
            
            // Merge metadata
            for (key, value) in imported_model.metadata() {
                builder.put_metadata(key.clone(), value.clone());
            }
        }
        
        // 3. Add prelude if enabled
        if self.enable_prelude {
            let prelude = self.load_prelude()?;
            for shape in prelude.shapes() {
                builder.put_shape(shape.clone());
            }
        }
        
        // 4. Add shapes from this assembler
        for (_, shape) in &self.shapes {
            builder.put_shape(shape.clone());
        }
        
        // 5. Add metadata from this assembler
        for (key, value) in &self.metadata {
            builder.put_metadata(key.clone(), value.clone());
        }
        
        // 6. Build the model
        let model = builder.build()?;
        
        // 7. Validate if enabled
        if self.enable_validation {
            self.validate_model(&model)?;
        }
        
        Ok(model)
    }
    
    // Non-fluent methods
    
    pub fn put_shape(&mut self, shape: Shape) {
        let id = shape.id().clone();
        
        // If shape is not a member shape, add it directly
        if !shape.is_member_shape() {
            self.shapes.insert(id, shape);
            
            // Add all member shapes automatically
            if let Some(members) = shape.members() {
                for member in members {
                    let member_id = member.id().clone();
                    self.shapes.insert(member_id, member);
                }
            }
        }
    }
    
    pub fn put_metadata(&mut self, key: impl Into<String>, value: impl Into<Node>) {
        self.metadata.insert(key.into(), value.into());
    }
    
    pub fn put_property(&mut self, key: impl Into<String>, value: impl Into<Node>) {
        self.properties.insert(key.into(), value.into());
    }
    
    pub fn remove_property(&mut self, key: impl AsRef<str>) {
        self.properties.remove(key.as_ref());
    }
    
    pub fn remove_metadata(&mut self, key: impl AsRef<str>) {
        self.metadata.remove(key.as_ref());
    }
    
    pub fn set_validator_registry(&mut self, registry: ValidatorRegistry) {
        self.validator_registry = Some(registry);
    }
    
    pub fn set_trait_registry(&mut self, registry: TraitRegistry) {
        self.trait_registry = Some(registry);
    }
    
    pub fn add_import(&mut self, path: impl Into<PathBuf>) {
        self.imports.push(path.into());
    }
    
    pub fn add_imports<I>(&mut self, paths: I) 
    where
        I: IntoIterator,
        I::Item: Into<PathBuf>,
    {
        for path in paths {
            self.imports.push(path.into());
        }
    }
    
    // Helper methods
    
    fn load_model_from_path(&self, path: &Path) -> Result<Model, ValidationError> {
        // Implementation would load a model from a file
        // This would use the parser to parse the file and create a model
        unimplemented!()
    }
    
    fn load_prelude(&self) -> Result<Model, ValidationError> {
        // Implementation would load the prelude model
        unimplemented!()
    }
    
    fn validate_model(&self, model: &Model) -> Result<(), ValidationError> {
        // Implementation would validate the model using the validator registry
        unimplemented!()
    }
}
```

## 6. Knowledge Index Pattern

### KnowledgeIndex Trait Definition

```rust
pub trait KnowledgeIndex: 'static {
    fn from_model(model: &Model) -> Self;
}
```

### Example Implementations

```rust
// Service Index - provides access to all services in the model
pub struct ServiceIndex {
    services: HashMap<ShapeId, ServiceShape>,
}

impl KnowledgeIndex for ServiceIndex {
    fn from_model(model: &Model) -> Self {
        let mut services = HashMap::new();
        for service in model.shapes().with_type::<ServiceShape>() {
            services.insert(service.id().clone(), service.clone());
        }
        Self { services }
    }
}

impl ServiceIndex {
    pub fn get_service(&self, id: impl AsRef<ShapeId>) -> Option<&ServiceShape> {
        self.services.get(id.as_ref())
    }
    
    pub fn services(&self) -> impl Iterator<Item = &ServiceShape> {
        self.services.values()
    }
}

// Trait Index - provides access to shapes with specific traits
pub struct TraitIndex {
    shapes_with_traits: HashMap<ShapeId, Vec<ShapeId>>,
}

impl KnowledgeIndex for TraitIndex {
    fn from_model(model: &Model) -> Self {
        let mut shapes_with_traits = HashMap::new();
        
        for shape in model.shapes() {
            for trait_id in shape.traits().keys() {
                shapes_with_traits
                    .entry(trait_id.clone())
                    .or_insert_with(Vec::new)
                    .push(shape.id().clone());
            }
        }
        
        Self { shapes_with_traits }
    }
}

impl TraitIndex {
    pub fn shapes_with_trait(&self, trait_id: impl AsRef<ShapeId>) -> Vec<ShapeId> {
        self.shapes_with_traits
            .get(trait_id.as_ref())
            .cloned()
            .unwrap_or_default()
    }
}
```

### Usage Examples

```rust
// Get all services in the model
let service_index = model.knowledge::<ServiceIndex>();
for service in service_index.services() {
    println!("Service: {}", service.id());
}

// Get all shapes with a specific trait
let trait_index = model.knowledge::<TraitIndex>();
let shapes_with_trait = trait_index.shapes_with_trait("smithy.api#documentation");
for shape_id in shapes_with_trait {
    println!("Shape with documentation: {}", shape_id);
}
```

## 7. Validation Framework

### Validator Trait

```rust
pub trait Validator {
    fn validate(&self, model: &Model) -> Vec<ValidationEvent>;
    
    fn id(&self) -> &str;
    
    fn description(&self) -> &str {
        "No description provided"
    }
}
```

### Validation Events

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
```

### Validator Registry

```rust
pub struct ValidatorRegistry {
    validators: HashMap<String, Box<dyn Validator>>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }
    
    pub fn add_validator<V: Validator + 'static>(&mut self, validator: V) {
        self.validators.insert(validator.id().to_string(), Box::new(validator));
    }
    
    pub fn validate(&self, model: &Model) -> Vec<ValidationEvent> {
        let mut events = Vec::new();
        
        for validator in self.validators.values() {
            events.extend(validator.validate(model));
        }
        
        events
    }
}
```

### Example Validator

```rust
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
            if let Some(structure) = shape.as_shape_type::<StructureShape>() {
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

## 8. Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("Shape with ID {0} already exists in the model")]
    DuplicateShape(ShapeId),
    
    #[error("Invalid shape reference: {0}")]
    InvalidShapeReference(ShapeId),
    
    #[error("Invalid member shape: {0}")]
    InvalidMemberShape(ShapeId),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Multiple validation errors: {0}")]
    MultipleErrors(Vec<ValidationEvent>),
}
```

### Result Pattern Usage

All operations that can fail return a `Result` type:

```rust
// Building a model
pub fn build(self) -> Result<Model, ModelError> { ... }

// Assembling a model
pub fn assemble(&self) -> Result<Model, ValidationError> { ... }

// Loading a model from a file
fn load_model_from_path(&self, path: &Path) -> Result<Model, ValidationError> { ... }
```

## 9. Examples

### Creating a Model

```rust
// Create a model with the builder
let model = Model::builder()
    .add_shape(StringShape::builder().id("example#MyString").build()?)
    .add_shape(StructureShape::builder()
        .id("example#MyStructure")
        .member(MemberShape::builder()
            .id("example#MyStructure$myString")
            .target("example#MyString")
            .build()?)
        .build()?)
    .metadata("version", Node::from("1.0"))
    .build()?;

// Access shapes in the model
let my_string = model.expect_shape_of_type::<StringShape>("example#MyString");
let my_structure = model.expect_shape_of_type::<StructureShape>("example#MyStructure");

// Iterate over shapes
for shape in model.shapes() {
    println!("Shape: {}", shape.id());
}

// Iterate over shapes of a specific type
for string_shape in model.shapes().with_type::<StringShape>() {
    println!("String shape: {}", string_shape.id());
}

// Iterate over shapes with a specific trait
for shape in model.shapes().with_trait("smithy.api#documentation") {
    println!("Shape with documentation: {}", shape.id());
}
```

### Assembling a Model

```rust
// Discover models in a directory
let discovered = ModelDiscovery::discover_models("/path/to/models")?;

// Assemble a model from multiple sources
let model = Model::assembler()
    .add_shape(StringShape::builder().id("example#MyString").build()?)
    .with_imports(discovered)
    .with_import("/path/to/my-model.smithy")
    .with_property(ModelAssembler::ALLOW_UNKNOWN_TRAITS, true)
    .enable_validation(true)
    .assemble()?;
```

### Using Knowledge Indexes

```rust
// Define a custom knowledge index
struct OperationIndex {
    operations_by_service: HashMap<ShapeId, Vec<OperationShape>>,
}

impl KnowledgeIndex for OperationIndex {
    fn from_model(model: &Model) -> Self {
        let mut operations_by_service = HashMap::new();
        
        // Find all services
        for service in model.shapes().with_type::<ServiceShape>() {
            let service_id = service.id().clone();
            let mut service_operations = Vec::new();
            
            // Find all operations for this service
            for operation_id in service.operations() {
                if let Some(operation) = model.get_shape(operation_id) {
                    if let Some(operation) = operation.as_shape_type::<OperationShape>() {
                        service_operations.push(operation.clone());
                    }
                }
            }
            
            operations_by_service.insert(service_id, service_operations);
        }
        
        Self { operations_by_service }
    }
}

impl OperationIndex {
    pub fn operations_for_service(&self, service_id: impl AsRef<ShapeId>) -> &[OperationShape] {
        self.operations_by_service
            .get(service_id.as_ref())
            .map(|ops| ops.as_slice())
            .unwrap_or(&[])
    }
}

// Use the knowledge index
let operation_index = model.knowledge::<OperationIndex>();
let operations = operation_index.operations_for_service("example#MyService");
for operation in operations {
    println!("Operation: {}", operation.id());
}
```

## 10. Implementation Plan

### Phase 1: Core Model Structure

1. Implement the basic Model struct with shapes and metadata
2. Implement the ModelBuilder with basic functionality
3. Implement the Shapes iterator for basic shape access
4. Add tests for core functionality

### Phase 2: Knowledge Index Pattern

1. Implement the KnowledgeIndex trait
2. Add caching mechanism to the Model
3. Implement basic knowledge indexes (ServiceIndex, TraitIndex)
4. Add tests for knowledge indexes

### Phase 3: ModelAssembler and Validation

1. Implement the ModelAssembler
2. Implement the Validator trait and ValidationEvent
3. Implement the ValidatorRegistry
4. Add basic validators
5. Add tests for validation

### Phase 4: Model Loading and Discovery

1. Implement model loading from files
2. Implement model discovery
3. Add support for the prelude
4. Add tests for loading and discovery

### Phase 5: Optimization and Refinement

1. Add benchmarks for performance testing
2. Optimize caching mechanisms
3. Refine APIs based on usage patterns
4. Add comprehensive documentation
5. Add more tests for edge cases
