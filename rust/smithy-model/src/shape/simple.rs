/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Simple shape types for the Smithy model.

use crate::shape::iter::Members;
use crate::shape::{
    error::BuildError, mixin, MemberShape, Shape, ShapeBuilder, ShapeMetadata,
    ShapeMetadataBuilder, ShapeProperties,
};
use crate::traits::type_refinement::EnumValue;
use indexmap::IndexMap;
use paste::paste;
use std::hash::Hash;

/// Macro to define a simple shape type with its builder
macro_rules! define_simple_shape {
    (
        $(#[$shape_meta:meta])*
        $shape_name:ident, $shape_variant:ident, $doc_link:expr
    ) => {
        paste! {
            $(#[$shape_meta])*
            #[doc = concat!("A [", stringify!($shape_name), "](", $doc_link, ") shape")]
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $shape_name {
                pub(crate) metadata: ShapeMetadata,
            }

            impl $shape_name {
                /// Create a new builder for this shape type.
                pub fn builder() -> [<$shape_name Builder>] {
                    [<$shape_name Builder>]::new()
                }

                /// Convert this shape back into a builder
                pub fn to_builder(self) -> [<$shape_name Builder>] {
                    [<$shape_name Builder>] {
                        metadata: self.metadata.to_builder(),
                    }
                }
            }

            impl ShapeProperties for $shape_name {
                fn metadata(&self) -> &ShapeMetadata {
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

            #[doc = concat!("Builder for creating a ", stringify!($shape_name))]
            #[derive(Debug, Default)]
            pub struct [<$shape_name Builder>] {
                metadata: ShapeMetadataBuilder,
            }

            impl [<$shape_name Builder>] {
                #[doc = concat!("Create a new ", stringify!($shape_name), " builder.")]
                pub(crate) fn new() -> Self {
                    Self::default()
                }

                #[doc = concat!("Set the ID of the ", stringify!($shape_name))]
                pub fn id(mut self, id: impl Into<String>) -> Self {
                    self.metadata = self.metadata.id(id);
                    self
                }

                #[doc = concat!("Build the ", stringify!($shape_name))]
                pub fn build(self) -> Result<$shape_name, BuildError> {
                    // Validate that mixins are of the same shape type
                    self.metadata
                        .validate_mixins(|shape| matches!(shape, Shape::$shape_variant(_)), stringify!($shape_variant))?;

                    let metadata = self.metadata.build()?;

                    Ok($shape_name {
                        metadata,
                    })
                }
            }

            impl ShapeBuilder for [<$shape_name Builder>] {
                fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder {
                    &mut self.metadata
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
    pub members: IndexMap<String, MemberShape>,
}

impl EnumShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> EnumShapeBuilder {
        EnumShapeBuilder::new()
    }

    /// Returns a Members container for this enum shape.
    pub fn members(&self) -> Members<'_> {
        Members::map(&self.members)
    }

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> EnumShapeBuilder {
        // FIXME - we aren't tracking introduced vs inherited members from mixins
        let mut builder = EnumShape::builder();
        builder.metadata = self.metadata.to_builder();
        builder.members = self.members;
        builder
    }
}

impl ShapeProperties for EnumShape {
    fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<EnumShape> for Shape {
    fn from(shape: EnumShape) -> Self {
        Shape::Enum(shape)
    }
}

/// Builder for creating an enum shape.
#[derive(Debug, Default)]
pub struct EnumShapeBuilder {
    metadata: ShapeMetadataBuilder,
    members: IndexMap<String, MemberShape>,
}

impl EnumShapeBuilder {
    /// Create a new enum shape builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the enum shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
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
        // Validate that mixins are of the same shape type
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Enum(_)), "enum")?;

        // Build the metadata
        let metadata = self.metadata.build()?;

        let members = mixin::compute_effective_members(self.members, &metadata)?;

        if members.is_empty() {
            return Err(BuildError::InvalidValue {
                field: "members".to_string(),
                reason: "Enum shape must have at least one member".to_string(),
            });
        }

        Ok(EnumShape { metadata, members })
    }
}

impl ShapeBuilder for EnumShapeBuilder {
    fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder {
        &mut self.metadata
    }
}

/// An [intEnum](https://smithy.io/2.0/spec/simple-types.html#intenum) shape
#[derive(Debug, Clone, PartialEq)]
pub struct IntEnumShape {
    pub(crate) metadata: ShapeMetadata,
    /// The integer enum members, keyed by member name
    pub members: IndexMap<String, MemberShape>,
    /// The integer values for each member, keyed by member name
    pub values: IndexMap<String, i64>,
}

impl IntEnumShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> IntEnumShapeBuilder {
        IntEnumShapeBuilder::new()
    }

    /// Returns a Members container for this intEnum shape.
    pub fn members(&self) -> Members<'_> {
        Members::map(&self.members)
    }

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> IntEnumShapeBuilder {
        // FIXME - we aren't keeping track of introduced members vs those from mixins here
        let mut builder = IntEnumShape::builder();
        builder.metadata = self.metadata.to_builder();
        builder.members = self.members;
        builder
    }
}

impl ShapeProperties for IntEnumShape {
    fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<IntEnumShape> for Shape {
    fn from(shape: IntEnumShape) -> Self {
        Shape::IntEnum(shape)
    }
}

/// Builder for creating an integer enum shape.
#[derive(Debug, Default)]
pub struct IntEnumShapeBuilder {
    metadata: ShapeMetadataBuilder,
    members: IndexMap<String, MemberShape>,
}

impl IntEnumShapeBuilder {
    /// Create a new integer enum shape builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the integer enum shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
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
        let member = member
            .to_builder()
            .with_trait(EnumValue::new_int(value))
            .build()
            .unwrap();
        self.members.insert(member.member_name.clone(), member);
        self
    }

    /// Build the integer enum shape.
    pub fn build(self) -> Result<IntEnumShape, BuildError> {
        // Validate that mixins are of the same shape type
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::IntEnum(_)), "intEnum")?;

        // Build the metadata
        let metadata = self.metadata.build()?;
        let members = mixin::compute_effective_members(self.members, &metadata)?;

        // compute the values
        let values: IndexMap<String, i64> = members
            .iter()
            .map(|(name, member)| {
                // Check if the member has the EnumValue trait
                let enum_value =
                    member
                        .get_trait_as::<EnumValue>()
                        .ok_or_else(|| BuildError::InvalidValue {
                            field: name.clone(),
                            reason: "intEnum shape Members must have the EnumValue trait"
                                .to_string(),
                        })?;

                // Parse the enum value as an i64
                let int_value =
                    enum_value
                        .0
                        .parse::<i64>()
                        .map_err(|_| BuildError::InvalidValue {
                            field: name.clone(),
                            reason: format!(
                                "EnumValue '{}' could not be parsed as an integer",
                                enum_value.0
                            ),
                        })?;

                // Return the name and parsed value
                Ok((name.clone(), int_value))
            })
            .collect::<Result<IndexMap<String, i64>, BuildError>>()?;

        // Validate that we have at least one member after merging
        if members.is_empty() {
            return Err(BuildError::InvalidValue {
                field: "members".to_string(),
                reason: "Integer enum shape must have at least one member".to_string(),
            });
        }

        // Check for duplicate values
        let mut seen_values = std::collections::HashSet::new();
        for (name, &value) in values.iter() {
            if !seen_values.insert(value) {
                return Err(BuildError::InvalidValue {
                    field: name.to_string(),
                    reason: format!(
                        "Duplicate integer value: {} found for intEnum member",
                        value
                    ),
                });
            }
        }

        Ok(IntEnumShape {
            metadata,
            members,
            values,
        })
    }
}

impl ShapeBuilder for IntEnumShapeBuilder {
    fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder {
        &mut self.metadata
    }
}

#[cfg(test)]
mod tests {
    // Simple shape tests
    use super::*;
    use crate::shape::builder::ShapeBuilderExt;
    use crate::traits::{Mixin, Required, Trait};
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

    #[test]
    fn test_simple_shape_with_mixins() {
        // Create a mixin string shape with a pattern trait
        let mixin = StringShape::builder()
            .id("example.foo#PatternString")
            .with_trait(Required)
            .with_trait(Mixin::new())
            .build()
            .unwrap();

        // Create a string shape that uses the mixin
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .mixin(mixin)
            .build()
            .unwrap();

        // The shape should inherit the pattern trait from the mixin
        assert_eq!(shape.id().to_string(), "example.foo#MyString");
        assert_eq!(shape.traits().len(), 1);
        assert!(shape.has_trait(Required::static_id()));
    }

    #[test]
    fn test_simple_shape_mixin_validation() {
        // Create a boolean mixin
        let boolean_mixin = BooleanShape::builder()
            .id("example.foo#BooleanMixin")
            .with_trait(Mixin::new())
            .build()
            .unwrap();

        // Try to use a boolean mixin with a string shape - should fail
        let result = StringShape::builder()
            .id("example.foo#MyString")
            .mixin(boolean_mixin)
            .build();

        assert!(result.is_err());
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
                assert_eq!(field, "SECOND");
                assert!(reason.contains("Duplicate integer value: 1"));
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_enum_shape_with_mixins() {
        let unit_id = ShapeId::new("smithy.api", "Unit").unwrap();

        // Create a mixin enum shape
        let mixin = EnumShape::builder()
            .id("example.foo#MixinEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MixinEnum$MIXIN_MEMBER")
                    .member_name("MIXIN_MEMBER")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
            )
            .with_trait(Mixin::new())
            .build()
            .unwrap();

        // Create an enum shape that uses the mixin
        let shape = EnumShape::builder()
            .id("example.foo#MyEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MyEnum$DIRECT_MEMBER")
                    .member_name("DIRECT_MEMBER")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
            )
            .mixin(mixin)
            .build()
            .unwrap();

        // The shape should have both its own member and the mixin member
        assert_eq!(shape.id().to_string(), "example.foo#MyEnum");
        assert_eq!(shape.members.len(), 2);
        assert!(shape.members.contains_key("DIRECT_MEMBER"));
        assert!(shape.members.contains_key("MIXIN_MEMBER"));
    }

    #[test]
    fn test_int_enum_shape_with_mixins() {
        let unit_id = ShapeId::new("smithy.api", "Unit").unwrap();

        // Create a mixin int enum shape
        let mixin = IntEnumShape::builder()
            .id("example.foo#MixinIntEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MixinIntEnum$MIXIN_MEMBER")
                    .member_name("MIXIN_MEMBER")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
                1,
            )
            .with_trait(Mixin::new())
            .build()
            .unwrap();

        // Create an int enum shape that uses the mixin
        let shape = IntEnumShape::builder()
            .id("example.foo#MyIntEnum")
            .member(
                MemberShape::builder()
                    .id("example.foo#MyIntEnum$DIRECT_MEMBER")
                    .member_name("DIRECT_MEMBER")
                    .target(unit_id.clone())
                    .build()
                    .unwrap(),
                2,
            )
            .mixin(mixin)
            .build()
            .unwrap();

        // The shape should have both its own member and the mixin member
        assert_eq!(shape.id().to_string(), "example.foo#MyIntEnum");
        assert_eq!(shape.members.len(), 2);
        assert!(shape.members.contains_key("DIRECT_MEMBER"));
        assert!(shape.members.contains_key("MIXIN_MEMBER"));
        assert_eq!(shape.values.get("DIRECT_MEMBER"), Some(&2));
        assert_eq!(shape.values.get("MIXIN_MEMBER"), Some(&1));
    }
}
