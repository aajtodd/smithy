/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Shape types for the Smithy model.

mod aggregate;
mod service;
mod simple;
mod type_checks;

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::shape_id::ShapeId;
use crate::traits::Trait;

pub use self::aggregate::*;
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

    // Special types
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

/// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
#[derive(Debug, Clone)]
pub struct MemberShape {
    metadata: ShapeMetadata,
    /// The name of the member
    pub member_name: String,
    /// The target shape that this member references
    pub target: ShapeId,
}

impl ProvideShapeMetadata for MemberShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
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

impl From<MemberShape> for Shape {
    fn from(shape: MemberShape) -> Self {
        Shape::Member(shape)
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

// Implement Display for Shape
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_id_access() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let traits = HashMap::new();
        let shape = StringShape::new(id.clone(), traits);

        assert_eq!(shape.id(), &id);
    }

    #[test]
    fn test_shape_traits_access() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let trait_id = ShapeId::new("smithy.api", "documentation").unwrap();

        let mut traits = HashMap::new();
        traits.insert(trait_id.clone(), Trait::new(trait_id.clone()));

        let shape = StringShape::new(id.clone(), traits);

        assert!(shape.has_trait(&trait_id));
        assert!(shape.get_trait(&trait_id).is_some());
    }

    #[test]
    fn test_shape_enum_conversion() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let traits = HashMap::new();
        let string_shape = StringShape::new(id.clone(), traits);

        let shape: Shape = string_shape.into();

        assert_eq!(shape.id(), &id);
        assert!(matches!(shape, Shape::String(_)));
    }

    #[test]
    fn test_member_shape() {
        let id = ShapeId::new_with_member("com.example", "MyStruct", "myMember").unwrap();
        let traits = HashMap::new();
        let target = ShapeId::new("com.example", "String").unwrap();

        let member = MemberShape::new(id.clone(), traits, "myMember".to_string(), target.clone());

        assert_eq!(member.id(), &id);
        assert_eq!(member.member_name, "myMember");
        assert_eq!(member.target, target);
    }
}
