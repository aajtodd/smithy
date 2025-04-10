/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Aggregate shape types for the Smithy model.

use std::collections::HashMap;

use crate::shape::{ProvideShapeMetadata, Shape, ShapeMetadata};
use crate::shape_id::ShapeId;
use crate::traits::Trait;

/// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
#[derive(Debug, Clone)]
pub struct ListShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [map](https://smithy.io/2.0/spec/aggregate-types.html#map) shape
#[derive(Debug, Clone)]
pub struct MapShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
#[derive(Debug, Clone)]
pub struct SetShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
#[derive(Debug, Clone)]
pub struct StructureShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
#[derive(Debug, Clone)]
pub struct UnionShape {
    pub(crate) metadata: ShapeMetadata,
}

// Implement ProvideShapeMetadata for all aggregate shapes
impl ProvideShapeMetadata for ListShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for MapShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for SetShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for StructureShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for UnionShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

// Implement constructors for aggregate shapes
impl ListShape {
    /// Create a new list shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl MapShape {
    /// Create a new map shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl SetShape {
    /// Create a new set shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl StructureShape {
    /// Create a new structure shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl UnionShape {
    /// Create a new union shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

// Implement From traits for converting shape structs to Shape enum
impl From<ListShape> for Shape {
    fn from(shape: ListShape) -> Self {
        Shape::List(shape)
    }
}

impl From<MapShape> for Shape {
    fn from(shape: MapShape) -> Self {
        Shape::Map(shape)
    }
}

impl From<SetShape> for Shape {
    fn from(shape: SetShape) -> Self {
        Shape::Set(shape)
    }
}

impl From<StructureShape> for Shape {
    fn from(shape: StructureShape) -> Self {
        Shape::Structure(shape)
    }
}

impl From<UnionShape> for Shape {
    fn from(shape: UnionShape) -> Self {
        Shape::Union(shape)
    }
}
