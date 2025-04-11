/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Shape types for the Smithy model.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod aggregate;
mod builder;
mod error;
mod member;
mod service;
mod simple;
mod type_checks;

use crate::shape_id::ShapeId;
use crate::traits::Trait;

pub use self::aggregate::*;
pub use self::builder::*;
pub use self::error::*;
pub use self::member::*;
pub use self::service::*;
pub use self::simple::*;

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

    // FIXME - uncomment and revisit after the updated trait design is available
    // /// Get a specific trait with a concrete type
    // fn get_trait_as<T: Trait + 'static>(&self) -> Option<&T> {
    //     self.get_trait(T::static_id())
    //         .and_then(|t| t.as_any().downcast_ref::<T>())
    // }
    //
    // /// Get a specific trait with a concrete type, panicking if not found or wrong type
    // fn expect_trait<T: Trait + 'static>(&self) -> &T {
    //     self.get_trait_as::<T>().unwrap_or_else(|| {
    //         panic!(
    //             "Expected trait {} on shape {}, but it was not found or had the wrong type",
    //             T::static_id(),
    //             self.id()
    //         )
    //     })
    // }
}

/// Private trait for accessing shape metadata
pub(crate) trait ProvideShapeMetadata {
    /// Get the shape metadata
    fn meta(&self) -> &ShapeMetadata;
}

/// Common metadata for all shapes (implementation detail)
#[derive(Debug, Clone)]
pub(crate) struct ShapeMetadata {
    /// The shape ID
    id: ShapeId,
    /// Traits applied to this shape
    traits: HashMap<ShapeId, Trait>,
}

impl ShapeMetadata {
    /// Create new shape metadata
    pub(crate) fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
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

/// A Smithy shape.
///
/// Shapes are the fundamental building blocks of a Smithy model. Each shape has a unique ID,
/// a set of traits, and a specific kind that determines its behavior and properties.
#[derive(Debug, Clone)]
pub enum Shape {
    // Simple types
    /// A [boolean](https://smithy.io/2.0/spec/simple-types.html#boolean) shape
    Boolean(BooleanShape),
    /// A [byte](https://smithy.io/2.0/spec/simple-types.html#byte) shape
    Byte(ByteShape),
    /// A [short](https://smithy.io/2.0/spec/simple-types.html#short) shape
    Short(ShortShape),
    /// An [integer](https://smithy.io/2.0/spec/simple-types.html#integer) shape
    Integer(IntegerShape),
    /// A [long](https://smithy.io/2.0/spec/simple-types.html#long) shape
    Long(LongShape),
    /// A [float](https://smithy.io/2.0/spec/simple-types.html#float) shape
    Float(FloatShape),
    /// A [double](https://smithy.io/2.0/spec/simple-types.html#double) shape
    Double(DoubleShape),
    /// A [bigInteger](https://smithy.io/2.0/spec/simple-types.html#biginteger) shape
    BigInteger(BigIntegerShape),
    /// A [bigDecimal](https://smithy.io/2.0/spec/simple-types.html#bigdecimal) shape
    BigDecimal(BigDecimalShape),
    /// A [string](https://smithy.io/2.0/spec/simple-types.html#string) shape
    String(StringShape),
    /// A [blob](https://smithy.io/2.0/spec/simple-types.html#blob) shape
    Blob(BlobShape),
    /// A [timestamp](https://smithy.io/2.0/spec/simple-types.html#timestamp) shape
    Timestamp(TimestampShape),
    /// A [document](https://smithy.io/2.0/spec/simple-types.html#document) shape
    Document(DocumentShape),
    /// An [enum](https://smithy.io/2.0/spec/simple-types.html#enum) shape
    Enum(EnumShape),
    /// An [intEnum](https://smithy.io/2.0/spec/simple-types.html#intenum) shape
    IntEnum(IntEnumShape),

    // Aggregate types
    /// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
    List(ListShape),
    /// A [map](https://smithy.io/2.0/spec/aggregate-types.html#map) shape
    Map(MapShape),
    /// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
    Set(SetShape),
    /// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
    Structure(StructureShape),
    /// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
    Union(UnionShape),

    // Service types
    /// A [service](https://smithy.io/2.0/spec/service-types.html#service) shape
    Service(ServiceShape),
    /// An [operation](https://smithy.io/2.0/spec/service-types.html#operation) shape
    Operation(OperationShape),
    /// A [resource](https://smithy.io/2.0/spec/service-types.html#resource) shape
    Resource(ResourceShape),

    // Member shape
    /// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
    Member(MemberShape),
}

// Implement ProvideShapeMetadata for Shape by delegating to its variants
impl ProvideShapeMetadata for Shape {
    fn meta(&self) -> &ShapeMetadata {
        match self {
            Shape::Boolean(shape) => shape.meta(),
            Shape::Byte(shape) => shape.meta(),
            Shape::Short(shape) => shape.meta(),
            Shape::Integer(shape) => shape.meta(),
            Shape::Long(shape) => shape.meta(),
            Shape::Float(shape) => shape.meta(),
            Shape::Double(shape) => shape.meta(),
            Shape::BigInteger(shape) => shape.meta(),
            Shape::BigDecimal(shape) => shape.meta(),
            Shape::String(shape) => shape.meta(),
            Shape::Blob(shape) => shape.meta(),
            Shape::Timestamp(shape) => shape.meta(),
            Shape::Document(shape) => shape.meta(),
            Shape::Enum(shape) => shape.meta(),
            Shape::IntEnum(shape) => shape.meta(),
            Shape::List(shape) => shape.meta(),
            Shape::Map(shape) => shape.meta(),
            Shape::Set(shape) => shape.meta(),
            Shape::Structure(shape) => shape.meta(),
            Shape::Union(shape) => shape.meta(),
            Shape::Service(shape) => shape.meta(),
            Shape::Operation(shape) => shape.meta(),
            Shape::Resource(shape) => shape.meta(),
            Shape::Member(shape) => shape.meta(),
        }
    }
}

impl AsRef<ShapeId> for Shape {
    fn as_ref(&self) -> &ShapeId {
        self.id()
    }
}

// Implement basic equality and hashing for Shape
impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Shape {}

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::builder::ShapeBuilderExt;

    #[test]
    fn test_shape_id_access() {
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = shape.into();

        assert_eq!(shape.id().to_string(), "example.foo#MyString");
    }

    #[test]
    fn test_shape_traits_access() {
        let trait_id = ShapeId::new("smithy.api", "documentation").unwrap();

        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .documentation("A string shape")
            .build()
            .unwrap();
        let shape: Shape = shape.into();

        assert!(shape.has_trait(trait_id));
    }

    #[test]
    fn test_shape_enum_conversion() {
        let string_shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = string_shape.into();

        match shape {
            Shape::String(_) => {}
            _ => panic!("Expected Shape::String"),
        }
    }

    #[test]
    fn test_member_shape() {
        let target = ShapeId::new("smithy.api", "String").unwrap();
        let member = MemberShape::builder()
            .id("example.foo#MyStruct$name")
            .member_name("name")
            .target(target.clone())
            .build()
            .unwrap();

        assert_eq!(member.id().to_string(), "example.foo#MyStruct$name");
        assert_eq!(member.member_name, "name");
        assert_eq!(member.target, target);
    }
}
