# Smithy Shape Design

This document outlines the design for representing Smithy shapes in the Rust implementation.

## Overview

Smithy shapes are the fundamental building blocks of a Smithy model. Each shape has a unique ID, a set of traits, and shape-specific data. Our design aims to provide a type-safe, ergonomic, and idiomatic Rust representation of Smithy shapes while maintaining compatibility with the Smithy specification.

## Core Design

### Key Traits

We use a trait-based approach with blanket implementations to provide common functionality:

```rust
/// Common trait for all shape types providing access to shape ID
pub trait HasShapeId {
    /// Get the shape ID
    fn id(&self) -> &ShapeId;
}

/// Common trait for all shape types providing access to traits
pub trait HasTraits {
    /// Get all traits applied to this shape
    fn traits(&self) -> &HashMap<ShapeId, Trait>;
    
    /// Check if this shape has a specific trait
    fn has_trait(&self, trait_id: impl AsRef<ShapeId>) -> bool {
        self.traits().contains_key(trait_id.as_ref())
    }
    
    /// Get a specific trait by ID
    fn get_trait(&self, trait_id: impl AsRef<ShapeId>) -> Option<&Trait> {
        self.traits().get(trait_id.as_ref())
    }
    
    /// Get a specific trait with a concrete type
    fn get_trait_as<T: Trait + 'static>(&self) -> Option<&T> {
        self.get_trait(T::static_id())
            .and_then(|t| t.as_any().downcast_ref::<T>())
    }
    
    /// Get a specific trait with a concrete type, panicking if not found or wrong type
    fn expect_trait<T: Trait + 'static>(&self) -> &T {
        self.get_trait_as::<T>().unwrap_or_else(|| {
            panic!(
                "Expected trait {} on shape {}, but it was not found or had the wrong type",
                T::static_id(),
                self.id()
            )
        })
    }
}

/// Private trait for accessing shape metadata
trait ProvideShapeMetadata {
    /// Get the shape metadata
    fn meta(&self) -> &ShapeMetadata;
}
```

### Shape Metadata

We use a private `ShapeMetadata` struct to store common shape data:

```rust
/// Common metadata for all shapes (implementation detail)
#[derive(Debug, Clone)]
struct ShapeMetadata {
    /// The shape ID
    id: ShapeId,
    /// Traits applied to this shape
    traits: HashMap<ShapeId, Trait>,
}

impl ShapeMetadata {
    /// Create new shape metadata
    fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self { id, traits }
    }
}

// Blanket implementations for any type that provides shape metadata
impl<T: ProvideShapeMetadata> HasShapeId for T {
    fn id(&self) -> &ShapeId {
        &self.meta().id
    }
}

impl<T: ProvideShapeMetadata> HasTraits for T {
    fn traits(&self) -> &HashMap<ShapeId, Trait> {
        &self.meta().traits
    }
}
```

### Shape Enum

We use an enum to represent all possible shape types:

```rust
/// A Smithy shape.
#[derive(Debug, Clone)]
pub enum Shape {
    /// A [boolean](https://smithy.io/2.0/spec/simple-types.html#boolean) shape
    Boolean(BooleanShape),
    /// A [string](https://smithy.io/2.0/spec/simple-types.html#string) shape
    String(StringShape),
    // ... other simple types
    
    /// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
    Structure(StructureShape),
    /// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
    Union(UnionShape),
    /// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
    List(ListShape),
    /// A [map](https://smithy.io/2.0/spec/aggregate-types.html#map) shape
    Map(MapShape),
    /// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
    Set(SetShape),
    
    /// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
    Member(MemberShape),
    
    /// An [enum](https://smithy.io/2.0/spec/simple-types.html#enum) shape
    Enum(EnumShape),
    /// An [intEnum](https://smithy.io/2.0/spec/simple-types.html#intenum) shape
    IntEnum(IntEnumShape),
    // ... other shape types
}
```

The `Shape` enum implements `ProvideShapeMetadata` by delegating to its variants:

```rust
impl ProvideShapeMetadata for Shape {
    fn meta(&self) -> &ShapeMetadata {
        match self {
            Shape::Boolean(shape) => shape.meta(),
            Shape::String(shape) => shape.meta(),
            Shape::Structure(shape) => shape.meta(),
            // ... other variants
        }
    }
}
```

We also provide `From` implementations for each shape variant:

```rust
impl From<StructureShape> for Shape {
    fn from(shape: StructureShape) -> Self {
        Shape::Structure(shape)
    }
}

// ... other From implementations
```

### Shape Type Implementations

Each shape type is implemented as a struct with its specific data and a reference to its metadata:

```rust
/// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
#[derive(Debug, Clone)]
pub struct StructureShape {
    /// Shape metadata (private)
    metadata: ShapeMetadata,
    /// The members of the structure, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

impl StructureShape {
    /// Create a new structure shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        members: HashMap<String, MemberShape>,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            members,
        }
    }
}

impl ProvideShapeMetadata for StructureShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}
```

### Member Shapes

Member shapes are used in aggregate types to reference other shapes:

```rust
/// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
#[derive(Debug, Clone)]
pub struct MemberShape {
    /// Shape metadata (private)
    metadata: ShapeMetadata,
    /// The name of the member
    pub member_name: String,
    /// The target shape that this member references
    pub target: ShapeId,
}

impl MemberShape {
    /// Create a new member shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        member_name: String,
        target: ShapeId,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            member_name,
            target,
        }
    }
}

impl ProvideShapeMetadata for MemberShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}
```

## Design Rationale

### Why an Enum-Based Approach?

We chose an enum-based approach for the `Shape` type for several reasons:

1. **Closed Set of Types**: Smithy defines a fixed set of shape types, making an enum a natural fit.
2. **Pattern Matching**: Rust's pattern matching works well with enums, making it easy to handle different shape types.
3. **Type Safety**: The enum approach provides compile-time type safety and exhaustive pattern matching.
4. **Memory Efficiency**: Enums can be more memory-efficient than trait objects.
5. **Idiomatic Rust**: This approach is more idiomatic in Rust compared to deep inheritance hierarchies.

### Why Store Shape Metadata in Each Variant?

In our previous design, we stored shape ID and traits in the parent `Shape` struct. We've now moved these to each shape variant for several reasons:

1. **Direct Access**: Each shape has direct access to its own ID and traits.
2. **Member Shape Access**: Member shapes in aggregate types can be accessed directly with their full shape information.
3. **Consistency with Java**: This approach is more consistent with the Java implementation where each shape has its own ID and traits.
4. **Usability**: It's more intuitive and ergonomic for users of the API.

### Why Use Traits and Blanket Implementations?

We use traits (`HasShapeId`, `HasTraits`) with blanket implementations for types that implement `ProvideShapeMetadata` for several reasons:

1. **Code Reuse**: Avoid duplicating common functionality across shape types.
2. **Consistency**: Ensure consistent behavior across all shape types.
3. **Extensibility**: Make it easy to add new shape types that automatically get the common functionality.
4. **Encapsulation**: Hide implementation details while providing a clean public API.

### Why Store MemberShape Instead of ShapeId?

In our previous design, aggregate shapes stored references to member shapes using `ShapeId`. We've changed this to store `MemberShape` directly for several reasons:

1. **Direct Access to Member Traits**: Member traits like `enumValue` can be accessed directly.
2. **Consistency with Java**: This approach is more consistent with the Java implementation.
3. **Usability**: It's more intuitive and ergonomic for users of the API.
4. **Completeness**: The model contains all necessary information without requiring additional lookups.

## Alternative Approaches Considered

### 1. Trait Object-Based Approach

```rust
pub trait Shape {
    fn id(&self) -> &ShapeId;
    fn traits(&self) -> &HashMap<ShapeId, Trait>;
    fn shape_type(&self) -> ShapeType;
    // Other common methods
}

pub struct StructureShape {
    id: ShapeId,
    traits: HashMap<ShapeId, Trait>,
    members: HashMap<String, Box<dyn Shape>>,
}

impl Shape for StructureShape {
    // Implementation
}
```

**Pros:**
- More extensible for custom shape types
- Cleaner interface for common operations
- More closely mirrors the Java implementation's class hierarchy

**Cons:**
- Requires dynamic dispatch (Box<dyn Shape>)
- More complex ownership model
- Potentially less efficient
- Less idiomatic in Rust

### 2. Hybrid Approach with ShapeKind

```rust
pub struct Shape {
    id: ShapeId,
    traits: HashMap<ShapeId, Trait>,
    kind: ShapeKind,
}

pub enum ShapeKind {
    Boolean,
    String,
    Structure(StructureData),
    // Other shape kinds
}

pub struct StructureData {
    members: HashMap<String, MemberData>,
}

pub struct MemberData {
    member_name: String,
    target: ShapeId,
}
```

**Pros:**
- Simpler ownership model
- Potentially more memory-efficient
- Centralized storage of common data

**Cons:**
- Member shapes don't have their own ID and traits
- Less intuitive API for accessing member traits
- Inconsistent with the Java implementation
- More complex to implement type-safe trait access

### 3. Type-Erased Approach with Downcasting

```rust
pub struct Shape {
    id: ShapeId,
    traits: HashMap<ShapeId, Trait>,
    data: Box<dyn Any>,
    shape_type: ShapeType,
}

impl Shape {
    fn as_structure(&self) -> Option<&StructureData> {
        self.data.downcast_ref::<StructureData>()
    }
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
- Less idiomatic in Rust

## Trade-offs and Design Decisions

### 1. Type Safety vs. Runtime Flexibility

We prioritized type safety while still providing runtime flexibility through:
- Strong typing with the enum-based approach
- Type-safe trait access through downcasting
- Support for unknown traits through `DynamicTrait`

### 2. Memory Efficiency vs. API Ergonomics

We balanced memory efficiency and API ergonomics by:
- Using an enum to represent all shape types
- Storing shape metadata in each variant
- Providing convenient methods for common operations

### 3. Consistency with Java vs. Idiomatic Rust

We aimed to be consistent with the Java implementation while still being idiomatic in Rust by:
- Using Rust's enum pattern instead of inheritance
- Providing similar functionality with different implementation details
- Maintaining the same conceptual model

### 4. Encapsulation vs. Direct Access

We balanced encapsulation and direct access by:
- Making `ShapeMetadata` private
- Providing public constructors that hide implementation details
- Exposing necessary data through public fields and methods

## Conclusion

Our design for representing Smithy shapes in Rust provides a good balance of type safety, ergonomics, and performance. It maintains compatibility with the Smithy specification while leveraging Rust's strengths. The enum-based approach with shape-specific variants and common traits provides a clean, idiomatic API that should be intuitive for users familiar with both Rust and Smithy.
