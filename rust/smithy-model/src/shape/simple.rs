/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Simple shape types for the Smithy model.

use crate::shape::{
    builder::{self, parse_shape_id, ProvideTraitsMut},
    error::BuildError,
    MemberShape, ProvideShapeMetadata, Shape, ShapeMetadata,
};
use crate::traits::TraitMap;
use paste::paste;
use std::collections::HashMap;
use std::hash::Hash;

/// Macro to define a simple shape type with its builder
macro_rules! define_simple_shape {
    (
        $(#[$shape_meta:meta])*
        $shape_name:ident, $shape_variant:ident, $doc_link:expr
    ) => {
        paste! {
            $(#[$shape_meta])*
            /// A [$shape_name]($doc_link) shape
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $shape_name {
                pub(crate) metadata: ShapeMetadata,
            }

            /// Builder for creating a $shape_name.
            #[derive(Debug, Default)]
            pub struct [<$shape_name Builder>] {
                id: Option<String>,
                traits: TraitMap,
            }

            impl [<$shape_name Builder>] {
                /// Create a new $shape_name builder.
                pub fn new() -> Self {
                    Self::default()
                }

                /// Set the ID of the $shape_name.
                pub fn id(mut self, id: impl Into<String>) -> Self {
                    self.id = Some(id.into());
                    self
                }

                /// Build the $shape_name.
                pub fn build(self) -> Result<$shape_name, BuildError> {
                    use builder::{field_names, required_field_error};

                    let id_str = self.id.ok_or_else(|| required_field_error(field_names::ID))?;
                    let id = parse_shape_id(&id_str)?;

                    Ok($shape_name {
                        metadata: ShapeMetadata::new(id, self.traits),
                    })
                }
            }

            impl ProvideTraitsMut for [<$shape_name Builder>] {
                fn traits_mut(&mut self) -> &mut TraitMap {
                    &mut self.traits
                }
            }

            impl $shape_name {
                /// Create a new builder for this shape type.
                pub fn builder() -> [<$shape_name Builder>] {
                    [<$shape_name Builder>]::new()
                }

                /// Convert this shape back into a builder
                pub fn to_builder(self) -> [<$shape_name Builder>] {
                    [<$shape_name Builder>] {
                        id: Some(self.metadata.id.to_string()),
                        traits: self.metadata.traits,
                    }
                }
            }

            impl ProvideShapeMetadata for $shape_name {
                fn meta(&self) -> &ShapeMetadata {
                    &self.metadata
                }
            }

            impl From<$shape_name> for Shape {
                fn from(shape: $shape_name) -> Self {
                    Shape::$shape_variant(shape)
                }
            }

            impl Hash for $shape_name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.metadata.id.hash(state);
                }
            }
        }
    };
}

// Define all simple shape types
define_simple_shape!(
    BooleanShape,
    Boolean,
    "https://smithy.io/2.0/spec/simple-types.html#boolean"
);

define_simple_shape!(
    ByteShape,
    Byte,
    "https://smithy.io/2.0/spec/simple-types.html#byte"
);

define_simple_shape!(
    ShortShape,
    Short,
    "https://smithy.io/2.0/spec/simple-types.html#short"
);

define_simple_shape!(
    IntegerShape,
    Integer,
    "https://smithy.io/2.0/spec/simple-types.html#integer"
);

define_simple_shape!(
    LongShape,
    Long,
    "https://smithy.io/2.0/spec/simple-types.html#long"
);

define_simple_shape!(
    FloatShape,
    Float,
    "https://smithy.io/2.0/spec/simple-types.html#float"
);

define_simple_shape!(
    DoubleShape,
    Double,
    "https://smithy.io/2.0/spec/simple-types.html#double"
);

define_simple_shape!(
    BigIntegerShape,
    BigInteger,
    "https://smithy.io/2.0/spec/simple-types.html#biginteger"
);

define_simple_shape!(
    BigDecimalShape,
    BigDecimal,
    "https://smithy.io/2.0/spec/simple-types.html#bigdecimal"
);

define_simple_shape!(
    StringShape,
    String,
    "https://smithy.io/2.0/spec/simple-types.html#string"
);

define_simple_shape!(
    BlobShape,
    Blob,
    "https://smithy.io/2.0/spec/simple-types.html#blob"
);

define_simple_shape!(
    TimestampShape,
    Timestamp,
    "https://smithy.io/2.0/spec/simple-types.html#timestamp"
);

define_simple_shape!(
    DocumentShape,
    Document,
    "https://smithy.io/2.0/spec/simple-types.html#document"
);

/// An [enum](https://smithy.io/2.0/spec/simple-types.html#enum) shape
#[derive(Debug, Clone, PartialEq)]
pub struct EnumShape {
    pub(crate) metadata: ShapeMetadata,
    /// The enum members, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

/// Builder for creating an enum shape.
#[derive(Debug, Default)]
pub struct EnumShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
    members: HashMap<String, MemberShape>,
}

impl EnumShapeBuilder {
    /// Create a new enum shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the enum shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a member to the enum shape.
    pub fn member(mut self, member: MemberShape) -> Self {
        // FIXME - need the equivalent of this java
        // if (!member.getTarget().equals(UnitTypeTrait.UNIT)) {
        //     throw new SourceException(String.format(
        //         "Enum members may only target `smithy.api#Unit`, but found `%s`",
        //         member.getTarget()), getSourceLocation());
        // }
        // if (!member.hasTrait(EnumValueTrait.ID)) {
        //     member = member.toBuilder()
        //         .addTrait(EnumValueTrait.builder().stringValue(member.getMemberName()).build())
        //         .build();
        // }

        self.members.insert(member.member_name.clone(), member);
        self
    }

    /// Build the enum shape.
    pub fn build(self) -> Result<EnumShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        if self.members.is_empty() {
            return Err(BuildError::InvalidValue {
                field: "members".to_string(),
                reason: "Enum shape must have at least one member".to_string(),
            });
        }

        Ok(EnumShape {
            metadata: ShapeMetadata::new(id, self.traits),
            members: self.members,
        })
    }
}

impl ProvideTraitsMut for EnumShapeBuilder {
    fn traits_mut(&mut self) -> &mut TraitMap {
        &mut self.traits
    }
}

impl EnumShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> EnumShapeBuilder {
        EnumShapeBuilder::new()
    }

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> EnumShapeBuilder {
        EnumShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.traits,
            members: self.members,
        }
    }
}

impl ProvideShapeMetadata for EnumShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<EnumShape> for Shape {
    fn from(shape: EnumShape) -> Self {
        Shape::Enum(shape)
    }
}

/// An [intEnum](https://smithy.io/2.0/spec/simple-types.html#intenum) shape
#[derive(Debug, Clone, PartialEq)]
pub struct IntEnumShape {
    pub(crate) metadata: ShapeMetadata,
    /// The integer enum members, keyed by member name
    pub members: HashMap<String, MemberShape>,
    /// The integer values for each member, keyed by member name
    pub values: HashMap<String, i64>,
}

/// Builder for creating an integer enum shape.
#[derive(Debug, Default)]
pub struct IntEnumShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
    members: HashMap<String, MemberShape>,
    values: HashMap<String, i64>,
}

impl IntEnumShapeBuilder {
    /// Create a new integer enum shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the integer enum shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a member to the integer enum shape.
    pub fn member(mut self, member: MemberShape, value: i64) -> Self {
        // FIXME - need equivalent of this java
        // if (!member.getTarget().equals(UnitTypeTrait.UNIT)) {
        //     throw new SourceException(String.format(
        //         "intEnum members may only target `smithy.api#Unit`, but found `%s`",
        //         member.getTarget()), getSourceLocation());
        // }
        let name_str = member.member_name.clone();
        self.members.insert(name_str.clone(), member);
        self.values.insert(name_str, value);
        self
    }

    /// Build the integer enum shape.
    pub fn build(self) -> Result<IntEnumShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        if self.members.is_empty() {
            return Err(BuildError::InvalidValue {
                field: "members".to_string(),
                reason: "Integer enum shape must have at least one member".to_string(),
            });
        }

        // Check for duplicate values
        let mut seen_values = std::collections::HashSet::new();
        for &value in self.values.values() {
            if !seen_values.insert(value) {
                return Err(BuildError::InvalidValue {
                    field: "values".to_string(),
                    reason: format!("Duplicate integer value: {}", value),
                });
            }
        }

        Ok(IntEnumShape {
            metadata: ShapeMetadata::new(id, self.traits),
            members: self.members,
            values: self.values,
        })
    }
}

impl ProvideTraitsMut for IntEnumShapeBuilder {
    fn traits_mut(&mut self) -> &mut TraitMap {
        &mut self.traits
    }
}

impl IntEnumShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> IntEnumShapeBuilder {
        IntEnumShapeBuilder::new()
    }

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> IntEnumShapeBuilder {
        IntEnumShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.traits,
            members: self.members,
            values: self.values,
        }
    }
}

impl ProvideShapeMetadata for IntEnumShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<IntEnumShape> for Shape {
    fn from(shape: IntEnumShape) -> Self {
        Shape::IntEnum(shape)
    }
}

#[cfg(test)]
mod tests {
    // Simple shape tests
    use super::*;
    use crate::shape::builder::ShapeBuilderExt;
    use crate::shape::{HasShapeId, HasTraits};
    use crate::ShapeId;
    use std::str::FromStr;

    #[test]
    fn test_string_shape_construction() {
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyString");
        assert_eq!(shape.traits().len(), 0);
    }

    #[test]
    fn test_boolean_shape_construction() {
        let shape = BooleanShape::builder()
            .id("example.foo#MyBoolean")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyBoolean");
        assert_eq!(shape.traits().len(), 0);
    }

    #[test]
    fn test_document_shape_construction() {
        let shape = DocumentShape::builder()
            .id("example.foo#MyDocument")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyDocument");
        assert_eq!(shape.traits().len(), 0);
    }

    #[test]
    fn test_simple_shape_with_traits() {
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .documentation("A test string")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyString");
        assert_eq!(shape.traits().len(), 1);
        assert!(shape.has_trait(ShapeId::from_str("smithy.api#documentation").unwrap()));
    }

    // Enum shape tests

    #[test]
    fn test_enum_shape_construction() {
        let unit_id = ShapeId::new("smithy.api", "Unit").unwrap();

        let shape = EnumShape::builder()
            .id("example.foo#MyEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MyEnum$FIRST")
                    .member_name("FIRST")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
            )
            .member(
                MemberShape::builder()
                    .id("example.foo#MyEnum$SECOND")
                    .member_name("SECOND")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyEnum");
        assert_eq!(shape.members.len(), 2);
        assert!(shape.members.contains_key("FIRST"));
        assert!(shape.members.contains_key("SECOND"));
    }

    #[test]
    fn test_enum_shape_empty_members() {
        let result = EnumShape::builder().id("example.foo#MyEnum").build();

        assert!(result.is_err());
        match result {
            Err(BuildError::InvalidValue { field, reason }) => {
                assert_eq!(field, "members");
                assert_eq!(reason, "Enum shape must have at least one member");
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_int_enum_shape_construction() {
        let unit_id = ShapeId::new("smithy.api", "Unit").unwrap();

        let shape = IntEnumShape::builder()
            .id("example.foo#MyIntEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MyIntEnum$FIRST")
                    .member_name("FIRST")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
                1,
            )
            .member(
                MemberShape::builder()
                    .id("example.foo#MyIntEnum$SECOND")
                    .member_name("SECOND")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
                2,
            )
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyIntEnum");
        assert_eq!(shape.members.len(), 2);
        assert_eq!(shape.values.len(), 2);
        assert_eq!(shape.values.get("FIRST"), Some(&1));
        assert_eq!(shape.values.get("SECOND"), Some(&2));
    }

    #[test]
    fn test_int_enum_shape_duplicate_values() {
        let unit_id = ShapeId::new("smithy.api", "Unit").unwrap();

        let result = IntEnumShape::builder()
            .id("example.foo#MyIntEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MyIntEnum$FIRST")
                    .member_name("FIRST")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
                1,
            )
            .member(
                MemberShape::builder() // Same value as FIRST
                    .id("example.foo#MyIntEnum$SECOND")
                    .member_name("SECOND")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
                1,
            )
            .build();

        assert!(result.is_err());
        match result {
            Err(BuildError::InvalidValue { field, reason }) => {
                assert_eq!(field, "values");
                assert!(reason.contains("Duplicate integer value: 1"));
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }
}
