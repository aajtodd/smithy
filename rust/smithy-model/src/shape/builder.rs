/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Builder traits for creating Smithy shapes.

use std::collections::HashMap;
use std::str::FromStr;

use crate::shape::error::BuildError;
use crate::shape_id::ShapeId;
use crate::traits::Trait;

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
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait>;
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
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .documentation("A string shape")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(ShapeId::new("smithy.api", "documentation").unwrap()));
    /// ```
    fn documentation(mut self, doc: impl Into<String>) -> Self {
        let _doc_string = doc.into();
        let trait_id = ShapeId::new("smithy.api", "documentation").unwrap();
        let trait_value = Trait::new(trait_id.clone());
        self.traits_mut().insert(trait_id, trait_value);
        self
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
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .required()
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(ShapeId::new("smithy.api", "required").unwrap()));
    /// ```
    fn required(mut self) -> Self {
        let trait_id = ShapeId::new("smithy.api", "required").unwrap();
        let trait_value = Trait::new(trait_id.clone());
        self.traits_mut().insert(trait_id, trait_value);
        self
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
    /// use smithy_model::traits::Trait;
    ///
    /// let trait_id = ShapeId::new("example.foo", "customTrait").unwrap();
    /// let custom_trait = Trait::new(trait_id.clone());
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .with_trait(custom_trait)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(trait_id));
    /// ```
    fn with_trait(mut self, trait_value: Trait) -> Self {
        self.traits_mut()
            .insert(trait_value.id().clone(), trait_value);
        self
    }
}

// Implement ShapeBuilderExt for all types that implement ProvideTraitsMut
impl<T: ProvideTraitsMut> ShapeBuilderExt for T {}
