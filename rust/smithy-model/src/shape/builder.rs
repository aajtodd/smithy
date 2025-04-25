/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Builder traits for creating Smithy shapes.

use std::str::FromStr;

use crate::shape::error::BuildError;
use crate::shape::iter::Members;
use crate::shape::ShapeId;
use crate::shape::{ShapeBuilder, ShapeProperties};
use crate::traits::documentation::Documentation;
use crate::traits::type_refinement::{Mixin, Required};

/// Common field names used in builders
pub(crate) mod field_names {
    /// ID field name
    pub(crate) const ID: &str = "id";
    /// Target field name
    pub(crate) const TARGET: &str = "target";
    /// Key field name
    pub(crate) const KEY: &str = "key";
    /// Value field name
    pub(crate) const VALUE: &str = "value";
    /// Member field name
    pub(crate) const MEMBER: &str = "member";
    /// identifiers field name
    pub(crate) const IDENTIFIERS: &str = "identifiers";
    /// identifiers field name
    pub(crate) const MIXIN: &str = "mixin";
}

/// Helper functions for common builder operations
pub(crate) fn required_field_error(field: &str) -> BuildError {
    BuildError::MissingRequiredField {
        field: field.to_string(),
    }
}

/// Helper function to parse a string into a ShapeId or return an appropriate build error.
pub(crate) fn parse_shape_id(id_str: &str) -> Result<ShapeId, BuildError> {
    ShapeId::from_str(id_str).map_err(|e| BuildError::InvalidValue {
        field: field_names::ID.to_string(),
        reason: format!("{}", e),
    })
}

/// Validates that member shape IDs belong to the parent shape.
///
/// This function checks that each member's shape ID follows the format `parentShapeId$memberName`,
/// where `parentShapeId` is the ID of the parent shape and `memberName` is the name of the member.
pub(crate) fn validate_member_shape_ids(
    shape_id: &ShapeId,
    members: &Members<'_>,
) -> Result<(), BuildError> {
    for (name, member) in members.iter_named() {
        let member_id = member.id();
        if member_id.namespace() != shape_id.namespace()
            || member_id.name() != shape_id.name()
            || member_id.member() != Some(name)
        {
            return Err(BuildError::InvalidValue {
                field: name.to_string(),
                reason: format!(
                    "Expected the `{name}` member of `{shape_id}` to have an ID of `{shape_id}${name}` but found `{member_id}`",
                    name = name,
                    shape_id = shape_id,
                    member_id = member_id,
                ),
            });
        }
    }
    Ok(())
}

/// Extension trait for shape builders with common trait methods.
pub trait ShapeBuilderExt: ShapeBuilder + Sized {
    /// Add documentation to the shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::ShapeProperties;
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::shape::ShapeId;
    /// use smithy_model::traits::Trait;
    /// use smithy_model::traits::documentation::Documentation;
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .documentation("A string shape")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(Documentation::static_id()));
    /// ```
    fn documentation(mut self, doc: impl Into<String>) -> Self {
        self.traits_mut()
            .insert(Box::new(Documentation(doc.into())));
        self
    }

    /// Mark the shape as required.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::ShapeProperties;
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::shape::ShapeId;
    /// use smithy_model::traits::{type_refinement::Required, Trait};
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .required()
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(Required::static_id()));
    /// ```
    fn required(mut self) -> Self {
        self.traits_mut().insert(Box::new(Required));
        self
    }

    /// Mark the shape as a mixin.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::ShapeProperties;
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::traits::{type_refinement::Mixin, Trait};
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .as_mixin()
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(Mixin::static_id()));
    /// ```
    fn as_mixin(mut self) -> Self {
        self.traits_mut().insert(Box::new(Mixin::new()));
        self
    }
}

impl<T: ShapeBuilder + Sized> ShapeBuilderExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::{MemberShape, ShapeBuilderExt, ShapeId, ShapeProperties, StringShape};
    use crate::traits::{type_refinement::Mixin, Trait};
    use indexmap::IndexMap;

    #[test]
    fn test_mixin_builder_extension() {
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .as_mixin()
            .build()
            .unwrap();

        assert!(shape.has_trait(Mixin::static_id()));
        let mixin_trait = shape.get_trait_as::<Mixin>().unwrap();
        assert!(mixin_trait.local_traits.is_empty());
    }

    #[test]
    fn test_validate_member_shape_ids() {
        // Create a parent shape ID
        let parent_id = ShapeId::new("example", "MyStruct").unwrap();

        // Create a valid member
        let valid_member = MemberShape::builder()
            .id("example#MyStruct$validMember")
            .member_name("validMember")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap();

        // Create an invalid member with wrong namespace
        let wrong_namespace_member = MemberShape::builder()
            .id("wrong#MyStruct$wrongNamespace")
            .member_name("wrongNamespace")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap();

        // Create an invalid member with wrong shape name
        let wrong_shape_name_member = MemberShape::builder()
            .id("example#WrongShape$wrongShapeName")
            .member_name("wrongShapeName")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap();

        // Create an invalid member with wrong member name
        let wrong_member_name = MemberShape::builder()
            .id("example#MyStruct$wrongName")
            .member_name("correctName")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap();

        // Test with a valid member
        let mut members_map = IndexMap::new();
        members_map.insert("validMember".to_string(), valid_member);
        let members = Members::map(&members_map);
        assert!(validate_member_shape_ids(&parent_id, &members).is_ok());

        // Test with wrong namespace
        let mut members_map = IndexMap::new();
        members_map.insert("wrongNamespace".to_string(), wrong_namespace_member);
        let members = Members::map(&members_map);
        let result = validate_member_shape_ids(&parent_id, &members);
        assert!(result.is_err());
        if let Err(BuildError::InvalidValue { field, reason }) = result {
            assert_eq!(field, "wrongNamespace");
            assert!(reason.contains("Expected the `wrongNamespace` member of `example#MyStruct`"));
        } else {
            panic!("Expected InvalidValue error");
        }

        // Test with wrong shape name
        let mut members_map = IndexMap::new();
        members_map.insert("wrongShapeName".to_string(), wrong_shape_name_member);
        let members = Members::map(&members_map);
        let result = validate_member_shape_ids(&parent_id, &members);
        assert!(result.is_err());

        // Test with wrong member name
        let mut members_map = IndexMap::new();
        members_map.insert("correctName".to_string(), wrong_member_name);
        let members = Members::map(&members_map);
        let result = validate_member_shape_ids(&parent_id, &members);
        assert!(result.is_err());
    }
}
