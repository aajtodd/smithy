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
