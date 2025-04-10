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

/// Trait for accessing the traits container.
pub trait ProvideTraitsMut {
    /// Get mutable access to the traits container.
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait>;
}

/// Extension trait for shape builders with common trait methods.
pub trait ShapeBuilderExt: ProvideTraitsMut + Sized {
    /// Add documentation to the shape.
    fn documentation(mut self, doc: impl Into<String>) -> Self {
        // FIXME - when we introduce concrete prelude traits remove this manual buildup
        let _doc_string = doc.into();
        let trait_id = ShapeId::new("smithy.api", "documentation").unwrap();
        let trait_value = Trait::new(trait_id.clone());
        self.traits_mut().insert(trait_id, trait_value);
        self
    }

    /// Mark the shape as required.
    fn required(mut self) -> Self {
        // FIXME - when we introduce concrete prelude traits remove this manual buildup
        let trait_id = ShapeId::new("smithy.api", "required").unwrap();
        let trait_value = Trait::new(trait_id.clone());
        self.traits_mut().insert(trait_id, trait_value);
        self
    }

    /// Add a Smithy trait to the shape.
    fn with_trait(mut self, trait_value: Trait) -> Self {
        self.traits_mut()
            .insert(trait_value.id().clone(), trait_value);
        self
    }
}

// Implement ShapeBuilderExt for all types that implement ProvideTraitsMut
impl<T: ProvideTraitsMut> ShapeBuilderExt for T {}

/// Helper function to parse a string into a ShapeId.
pub(crate) fn parse_shape_id(id_str: &str) -> Result<ShapeId, BuildError> {
    ShapeId::from_str(id_str).map_err(|e| BuildError::InvalidValue {
        field: "id".to_string(),
        reason: format!("{}", e),
    })
}
