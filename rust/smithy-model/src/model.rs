/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Model container for Smithy shapes.
//!
//! The Model container is responsible for organizing, indexing, and providing access to shapes.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::node::Node;
use crate::shape::{Shape, ShapeId};

/// A container for Smithy shapes.
///
/// The Model is the primary container for all shapes in a Smithy model. It provides
/// efficient access to shapes by ID, namespace, and shape type.
#[derive(Debug)]
pub struct Model {
    // Primary storage of shapes
    shapes: HashMap<ShapeId, Shape>,

    // Model metadata
    metadata: HashMap<String, Node>,

    // Optional caching fields for knowledge indexes
    // Using RwLock for thread-safe lazy initialization
    knowledge_indexes: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl Model {
    /// Creates a new empty model.
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            metadata: HashMap::new(),
            knowledge_indexes: RwLock::new(HashMap::new()),
        }
    }

    /// Returns the number of shapes in the model.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Gets a shape by ID.
    pub fn get_shape(&self, id: impl AsRef<ShapeId>) -> Option<&Shape> {
        self.shapes.get(id.as_ref())
    }

    /// Gets a shape by ID, panicking if not found.
    pub fn expect_shape(&self, id: impl AsRef<ShapeId>) -> &Shape {
        let id_ref = id.as_ref();
        self.get_shape(id_ref)
            .unwrap_or_else(|| panic!("Shape not found: {}", id_ref))
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        Self {
            shapes: self.shapes.clone(),
            metadata: self.metadata.clone(),
            knowledge_indexes: RwLock::new(HashMap::new()), // Start with fresh knowledge indexes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::{ShapeProperties, StringShape};

    #[test]
    fn test_empty_model() {
        let model = Model::new();
        assert_eq!(model.shape_count(), 0);
    }

    #[test]
    fn test_get_shape() {
        let mut model = Model::new();
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();

        // TODO - use yet to exist builder API
        model
            .shapes
            .insert(shape.id().clone(), shape.clone().into());

        let retrieved = model.get_shape(shape.id());
        assert!(retrieved.is_some());

        let non_existent_id = ShapeId::new_unchecked("example.foo#NonExistent");
        let non_existent = model.get_shape(&non_existent_id);
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_expect_shape() {
        let mut model = Model::new();
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();

        model
            .shapes
            .insert(shape.id().clone(), shape.clone().into());

        let retrieved = model.expect_shape(shape.id());
        assert_eq!(retrieved.id(), shape.id());
    }

    #[test]
    #[should_panic(expected = "Shape not found")]
    fn test_expect_shape_panics() {
        let model = Model::new();
        let non_existent_id = ShapeId::new_unchecked("example.foo#NonExistent");
        model.expect_shape(&non_existent_id);
    }
}
