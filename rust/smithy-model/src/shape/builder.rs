/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Builder traits for creating Smithy shapes.

use std::str::FromStr;

use crate::shape::error::BuildError;
use crate::shape::ShapeBuilder;
use crate::shape::ShapeId;
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
    use crate::shape::{ShapeBuilderExt, ShapeProperties, StringShape};
    use crate::traits::{type_refinement::Mixin, Trait};

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
}
