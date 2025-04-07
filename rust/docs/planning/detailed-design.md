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
### Trait Implementation (WIP)

We're currently evaluating approaches for implementing Smithy traits in Rust. Traits in Smithy are metadata attached to shapes and are identified by their ShapeId.

#### Current Recommendation (Draft)

A generic map-based approach with typed accessor methods for common traits:

```rust
struct Traits {
    values: HashMap<ShapeId, serde_json::Value>,
}

impl Traits {
    pub fn has(&self, id: &ShapeId) -> bool {
        self.values.contains_key(id)
    }
    
    pub fn get(&self, id: &ShapeId) -> Option<&serde_json::Value> {
        self.values.get(id)
    }
    
    pub fn get_as<T: DeserializeOwned>(&self, id: &ShapeId) -> Result<T, TraitError> {
        match self.get(id) {
            Some(value) => serde_json::from_value(value.clone()).map_err(TraitError::DeserializationError),
            None => Err(TraitError::TraitNotFound(id.clone())),
        }
    }
    
    // Common trait accessors
    pub fn documentation(&self) -> Option<String> {
        let doc_id = ShapeId::new("smithy.api", "documentation");
        self.get_as::<String>(&doc_id).ok()
    }
    
    pub fn is_required(&self) -> bool {
        let required_id = ShapeId::new("smithy.api", "required");
        self.has(&required_id)
    }
}
```

#### Alternative Approaches Under Consideration

1. **Strongly-Typed Traits with Enum**
2. **Hybrid Approach with Known Traits**

#### TODO: Complete Trait Implementation Design

- Finalize the approach for trait implementation
- Document the rationale for the chosen approach
- Add examples of how common traits will be accessed
- Address performance considerations for trait access
- Consider serialization/deserialization requirements

### TODO: Model Container Design

- How should the Model container be structured?
- What query capabilities should it provide?
- How should it handle namespaces?
- What indexing or caching mechanisms should be implemented?

### TODO: Immutability vs. Mutability

- Should the model be immutable after construction?
- If mutable, how do we ensure consistency?

### TODO: Serialization

- How should we handle serialization/deserialization of the model?
- Should we implement serde traits for the model components?

### TODO: Metadata

- How should we store metadata about the model (version, source files, etc.)?
- Should metadata be part of the model or separate?

### TODO: Validation

- How should the model support validation?
- Should validation be built into the model or separate?
