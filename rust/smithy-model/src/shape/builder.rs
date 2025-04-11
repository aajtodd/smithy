/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Builder traits for creating Smithy shapes.

use std::str::FromStr;

use crate::shape::error::BuildError;
use crate::shape_id::ShapeId;
use crate::traits::{Documentation, Required, Trait, TraitMap};

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

/// Trait for accessing the traits container.
pub trait ProvideTraitsMut {
    /// Get mutable access to the traits container.
    fn traits_mut(&mut self) -> &mut TraitMap;
}

/// Extension trait for shape builders with common trait methods.
pub trait ShapeBuilderExt: ProvideTraitsMut + Sized {
    /// Add documentation to the shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::{HasShapeId, HasTraits};
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::shape_id::ShapeId;
    /// use smithy_model::traits::{Documentation, Trait};
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .documentation("A string shape")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(Documentation::static_id()));
    /// ```
    fn documentation(self, doc: impl Into<String>) -> Self {
        self.with_trait(Documentation(doc.into()))
    }

    /// Mark the shape as required.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::{HasShapeId, HasTraits};
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::shape_id::ShapeId;
    /// use smithy_model::traits::{Required, Trait};
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .required()
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(Required::static_id()));
    /// ```
    fn required(self) -> Self {
        self.with_trait(Required)
    }

    /// Add a Smithy trait to the shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::{HasShapeId, HasTraits};
    /// use smithy_model::shape::ShapeBuilderExt;
    /// use smithy_model::shape_id::ShapeId;
    /// use smithy_model::traits::{DynamicTrait, Trait};
    ///
    /// let trait_id = ShapeId::new("example.foo", "customTrait").unwrap();
    /// let custom_trait = DynamicTrait::new(trait_id.clone(), None);
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .with_trait(custom_trait)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(trait_id));
    /// ```
    fn with_trait(mut self, strait: impl Trait) -> Self {
        self.traits_mut().insert(Box::new(strait));
        self
    }
}

// Implement ShapeBuilderExt for all types that implement ProvideTraitsMut
impl<T: ProvideTraitsMut> ShapeBuilderExt for T {}
