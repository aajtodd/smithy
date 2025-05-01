# Model Container Design Requirements

## Question 1: What is your preference regarding the immutability of the Model container? Should we follow the Java implementation with a fully immutable model and builder pattern, or would you prefer a different approach?

**Answer:** The model should be immutable, this makes it much easier to reason about. We can offer ways to convert back to a builder or transform a model but the container itself should be read only just like shapes are.

## Question 2: How should we handle ownership of shapes within the model? Should the model own all shapes (like in Java), or should we use references with appropriate lifetime parameters?

**Answer:** Own all shapes, we could consider a "ModelView" like API if we really find the need for it but I think we want the model to own all shapes

## Question 3: What kind of indexing capabilities should the model provide? Beyond basic ShapeId lookup, what other ways should users be able to query shapes in the model?

**Answer:** We probably want ways to iterate over the shapes, we may want the ability to get all shapes of a particular type or shapes with a particular trait (or both of type with trait). Java makes heavy use of "KnowledgeIndexes" for particular shape lookup/iteration/etc.

## Question 4: How should we handle member shapes in the model? Should they be automatically added/removed with their containing shapes (like in Java), or should we take a different approach?

**Answer:** Follow java approach, member shapes are added automatically (and not allowed to be added on their own)

## Question 5: Should we implement a Knowledge Index pattern similar to Java for caching computed information about the model? If so, how should we adapt it to be more idiomatic in Rust?

**Answer:** I think knowledge indexes in java have worked out really well overall for encapsulating a specific view of the model and getting information in particular contexts. This still seems useful and we should probably emulate it but make it idiomatic for Rust. What that looks like is up for debate, design, and iteration most likely.

## Question 6: How should we handle caching in the model? Should we use eager caching (compute indexes at model creation), lazy caching (compute on first access), or a combination?

**Answer:** Probably lazy to start with but we should be open to performance optimizations if they make sense and show real world benefits/differences.

## Question 7: How should we handle model validation? Should validation be part of the model construction process, or should it be a separate step?

**Answer:** Java separates the two and allows building a model without validation but has a separate "ModelAssembler" that has the ability to validate models (in addition to other things it does). I think we should follow a similar approach

## Question 8: How should we handle the prelude in the model? Should it be automatically included, or should it be explicitly added?

**Answer:** What does Java do? I think they make use of JVM's SPI ability to autoload the prelude but I'm guessing thats only for the assembler. I think we probably want the prelude to only be part of the assembler API but I'm not 100% sure right now.

## Question 9: After checking the Java implementation, I see that the ModelAssembler includes the prelude by default but allows disabling it with a `disablePrelude()` method. Should we follow this approach in our Rust implementation, or would you prefer a different default behavior?

**Answer:** I see no reason to differ at this time.

## Question 10: How should we handle thread safety in the model? Since the model is immutable, should we implement thread-safe caching mechanisms (like Java's ConcurrentHashMap), or should we use Rust's ownership system to handle this differently?

**Answer:** Rust's ownership system should handle this.

## Question 11: How should we handle error reporting in the model? Should we use Result types for operations that might fail, or should we use a more specialized error reporting mechanism?

**Answer:** Result is idiomatic Rust

## Question 12: Do you feel we have enough information to proceed with designing the model container, or are there other aspects we should consider?

**Answer:** I think this is enough for now.

## Question 13: For the core Model struct, what fields should it contain? Based on our research, I'm thinking of:
1. A HashMap<ShapeId, Shape> for storing shapes
2. A HashMap<String, Node> for metadata
3. Optional caching fields for performance

**Answer:** Looks good at first glance yes

## Question 14: For the caching mechanism in the Model, we discussed using lazy caching. Should we use Rust's `OnceCell` or `Mutex` for thread-safe lazy initialization, or would you prefer a different approach like `once_cell` or `lazy_static` crates?

**Answer:** Models are loaded dynamically so we won't be able to cache until runtime. The Java implementation looks like it has a "blackboard" concept where it keeps knowledge indexes cached. I think we will need caching but we may want to do it without first, add benchmarks, and then figure out our caching strategy

## Question 15: For the Model API, what core methods should we provide? I'm thinking of:
1. `get_shape(&self, id: &ShapeId) -> Option<&Shape>` - Basic shape lookup
2. `get_metadata(&self) -> &HashMap<String, Node>` - Access metadata
3. `get_metadata_property(&self, name: &str) -> Option<&Node>` - Get specific metadata
4. `shapes(&self) -> impl Iterator<Item = &Shape>` - Iterate over all shapes
5. `shapes_of_type<T: Shape>(&self) -> impl Iterator<Item = &T>` - Iterate over shapes of a specific type

**Answer:** Going to throw out some ideas and possible access patterns and see what we can come up with:

1. get_shape(&self, id: impl AsRef<ShapeId>) -> Option<&Shape> - Basic shape lookup
2. expect_shape(&self, id: impl AsRef<ShapeId> -> &Shape - basic shape lookup, panics if shape ID does not exist
3. metadata(&self) -> &HashMap<String, Node> - Access metadata
4. metadata_property(&self, name: impl AsRef<str>) -> Option<&Node> - Get specific metadata
5. shapes(&self) -> Shapes<'_> - Iterate over all shapes, Shapes is like our Members type that consolidates logic around querying shapes
6. assembler() -> ModelAssembler - create a new model assembler
7. builder() -> ModelBuilder - create a new model builder
8. to_builder(self) -> ModelBuidler - convert a model back into a builder

```rust
pub struct Shapes<'a> {
    model: &'a Model
}

impl<'a> Shapes<'a> {
    ...
}
```

Example access patterns:

```rust
for shape in model.shapes() {
    // iterate over all shapes
}

for string_shapes in model.shapes().with_type::<StringShape>() {
    // iterate over string shapes
}

let string_shapes_with_traits = model.shapes()
    .with_type::<StringShape>()
    .with_trait(MyTrait)
    .with_trait(MySecondTrait)
    .collect::<Vec<_>>();
```

## Question 16: For the ModelBuilder API, what methods should we provide? Based on your previous answers and the Java implementation, I'm thinking of:

```rust
pub struct ModelBuilder {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, Node>,
}

impl ModelBuilder {
    pub fn new() -> Self { ... }
    
    pub fn add_shape(&mut self, shape: Shape) -> Result<&mut Self, ModelError> { ... }
    
    pub fn add_shapes<I: IntoIterator<Item = Shape>>(&mut self, shapes: I) -> Result<&mut Self, ModelError> { ... }
    
    pub fn remove_shape(&mut self, id: impl AsRef<ShapeId>) -> &mut Self { ... }
    
    pub fn put_metadata(&mut self, key: impl Into<String>, value: Node) -> &mut Self { ... }
    
    pub fn remove_metadata(&mut self, key: impl AsRef<str>) -> &mut Self { ... }
    
    pub fn build(self) -> Result<Model, ModelError> { ... }
}
```

**Answer:** What about something like this (I've modified some of the signatures and adjusted some patterns):

```rust
pub struct ModelBuilder {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, Node>,
}

impl ModelBuilder {
    pub(crate) fn new() -> Self { ... }

    // fluent builder methods

    pub fn add_shape(mut self, shape: impl Into<Shape>) -> Self { ... }

    pub fn add_shapes<I: IntoIterator<Item = Shape>>(mut self, shapes: I) -> Self { ... }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<Node>) -> Self { ... }

    pub fn build(self) -> Result<Model, ModelError> { ... }



    // non-fluent builder methods


    pub fn put_metadata(&mut self, key: impl Into<String>, value: impl Into<Node>) { ... }

    pub fn remove_shape(&mut self, id: impl AsRef<ShapeId>) { ... }

    pub fn remove_metadata(&mut self, key: impl AsRef<str>) -> { ... }

    // get current shapes
    pub fn shapes(&self) -> Shapes<'_> // re-use our iterator possibly or return &HashMap<ShapeId, Shape>

}
```

## Question 17: For the ModelAssembler API, what methods should we provide? Based on our previous discussions and the Java implementation, I'm thinking of:

```rust
pub struct ModelAssembler {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, Node>,
    disable_prelude: bool,
    disable_validation: bool,
    // Other fields for validation, etc.
}

impl ModelAssembler {
    pub fn new() -> Self { ... }
    
    pub fn add_shape(&mut self, shape: impl Into<Shape>) -> &mut Self { ... }
    
    pub fn add_shapes<I: IntoIterator<Item = Shape>>(&mut self, shapes: I) -> &mut Self { ... }
    
    pub fn put_metadata(&mut self, key: impl Into<String>, value: impl Into<Node>) -> &mut Self { ... }
    
    pub fn disable_prelude(&mut self) -> &mut Self { ... }
    
    pub fn disable_validation(&mut self) -> &mut Self { ... }
    
    pub fn assemble(&self) -> Result<Model, ValidationError> { ... }
}
```

**Answer:** 

```rust
pub struct ModelAssembler {
    shapes: HashMap<ShapeId, Shape>,
    metadata: HashMap<String, Node>,
    properties: HashMap<String, Node>,
    enable_prelude: bool,
    enable_validation: bool,
    // Other fields for validation, etc.
}

impl ModelAssembler {
    pub(crate) fn new() -> Self { ... }

    // fluent builder methods

    // add a shape to the model
    pub fn add_shape(mut self, shape: impl Into<Shape>) -> Self { ... }

    // add all shapes to the model
    pub fn add_shapes<I: IntoIterator<Item = Shape>>(mut self, shapes: I) -> Self { ... }

    // add metadata to the model
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<Node>) -> Self { ... }

    // set a model assembler property
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<Node>) -> Self { ... }

    // enable or disable the prelude
    pub fn enable_prelude(mut self, enabled: bool) -> Self { ... }

    // enable or disable validation
    pub fn enable_validation(mut self, enabled: bool) -> Self { ... }

    // Merge a loaded model into the model assembler
    pub fn add_model(mut self, model: Model) -> Self { ... }


    // assemble the model
    pub fn assemble(&self) -> Result<Model, ValidationError> { ... }


    // non-fluent methods


    pub fn put_property(&mut self, key: impl Into<String>, value: impl Into<Node>) -> { ... }
    pub fn remove_property(&mut self, key: impl AsRef<str>) -> { ... }
    pub fn put_metadata(&mut self, key: impl Into<String>, value: impl Into<Node>) -> { ... }
    pub fn remove_metadata(&mut self, key: impl AsRef<str>) -> { ... }
}
```


We haven't designed these yet but we'll also need to consider model validators and trait registry

```rust

impl ModelAssembler {

    // add a custom validator
    pub fn add_validator(mut self, validator: impl Validator) -> Self { ... }
    // override the validator registry (responsible for dynamically resolving validator definitions)
    pub fn with_validator_registry(mut self, registry: ValidatorRegistry) -> Self  { ... }
    pub fn set_validator_registry(&mut self, registry: ValidatorRegistry)  { ... }

    // override the trait registry completely
    pub fn with_trait_registry(mut self, registry: TraitRegistry) -> Self  { ... }
    pub fn set_trait_registry(&mut self, registry: TraitRegistry) -> { ... }
}

```

Also need a custom/builtin property for allowing unknown traits:

```rust

let result = Model::assembler()
     .with_property(ModelAssembler::ALLOW_UNKNOWN_TRAITS, true)
     ...
     .assemble();

```


Finally we'll also need to consider how to load models from a path or discover them:

```rust
impl ModelAssembler {
    pub fn with_import(mut self, path: impl Into<Path>) -> Self { ... }
    pub fn with_imports<I: IntoIterator<Item = Path>>(mut self, paths: I) -> Self { ... }
    pub fn add_import(&mut self, path: impl Into<Path>) -> { ... }
    pub fn add_imports<I: IntoIterator<Item = Path>>(&mut self, paths: I) -> { ... }


}

struct ModelDiscovery { ... }

impl ModelDiscovery {
    fn discover_models(&mut self, path: impl AsRef<Path>) -> Result<Vec<Path>, Error> { ... }
}

```


```rust

// discover all smithy models in a directory
let discovered = ModelDiscovery::discover_models("/path/to/mymodels-dir")?;

let model = Model::assembler()
    .with_shape(MyCustomShape)
    .with_imports(discovered)
    .with_import("/path/to/my-model.smithy")
    .with_import("/second/path/to/my-model.json")
    .assemble()?;

```

## Question 18: For the Knowledge Index pattern, how should we adapt it to be more idiomatic in Rust? Based on our previous discussions, I'm thinking of something like:

```rust
pub trait KnowledgeIndex {
    // Method to create a new instance from a model
    fn from_model(model: &Model) -> Self;
}

impl Model {
    // Get or compute a knowledge index
    pub fn knowledge<T: KnowledgeIndex + 'static>(&self) -> &T {
        // Implementation would use interior mutability to cache the index
    }
}

// Example usage
struct ServiceIndex {
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

// Usage
let service_index = model.knowledge::<ServiceIndex>();
```

**Answer:** This seems reasonable
