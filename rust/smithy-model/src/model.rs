/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Model implementation for the Smithy model.
//!
//! This module provides the Model type, which represents a complete Smithy model.

use crate::error::Result;
use crate::shape::Shape;
use crate::shape_id::ShapeId;
use std::collections::HashMap;

/// A complete Smithy model.
#[derive(Debug, Clone)]
pub struct Model {
    /// The shapes in the model.
    shapes: HashMap<ShapeId, Shape>,
    /// The metadata associated with the model.
    metadata: HashMap<String, MetadataValue>,
}

/// A value for metadata in a Smithy model.
#[derive(Debug, Clone)]
pub enum MetadataValue {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(f64),
    /// A boolean value.
    Boolean(bool),
    /// An array of values.
    Array(Vec<MetadataValue>),
    /// An object with string keys and values.
    Object(HashMap<String, MetadataValue>),
    /// A null value.
    Null,
}

impl Model {
    /// Creates a new empty model.
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Returns a builder for creating a model.
    pub fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }

    /// Returns the shape with the given ID, if it exists.
    pub fn get_shape(&self, id: &ShapeId) -> Option<&Shape> {
        self.shapes.get(id)
    }

    /// Returns all shapes in the model.
    pub fn shapes(&self) -> impl Iterator<Item = &Shape> {
        self.shapes.values()
    }

    /// Returns the metadata value with the given key, if it exists.
    pub fn get_metadata(&self, key: &str) -> Option<&MetadataValue> {
        self.metadata.get(key)
    }

    /// Returns all metadata in the model.
    pub fn metadata(&self) -> impl Iterator<Item = (&String, &MetadataValue)> {
        self.metadata.iter()
    }
}

/// A builder for creating a model.
#[derive(Debug, Default)]
pub struct ModelBuilder {
    /// The shapes in the model.
    shapes: HashMap<ShapeId, Shape>,
    /// The metadata associated with the model.
    metadata: HashMap<String, MetadataValue>,
}

impl ModelBuilder {
    /// Creates a new model builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a shape to the model.
    pub fn shape(mut self, shape: Shape) -> Self {
        let id = shape.id().clone();
        self.shapes.insert(id, shape);
        self
    }

    /// Adds multiple shapes to the model.
    pub fn shapes(mut self, shapes: impl IntoIterator<Item = Shape>) -> Self {
        for shape in shapes {
            let id = shape.id().clone();
            self.shapes.insert(id, shape);
        }
        self
    }

    /// Adds a metadata value to the model.
    pub fn metadata(mut self, key: impl Into<String>, value: MetadataValue) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Builds the model.
    pub fn build(self) -> Result<Model> {
        // This is a placeholder implementation.
        // The actual validation will be implemented in a future task.
        Ok(Model {
            shapes: self.shapes,
            metadata: self.metadata,
        })
    }
}
impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
