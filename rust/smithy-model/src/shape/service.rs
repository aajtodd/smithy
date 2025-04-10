/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Service shape types for the Smithy model.

use std::collections::HashMap;

use crate::shape::{ProvideShapeMetadata, Shape, ShapeMetadata};
use crate::shape_id::ShapeId;
use crate::traits::Trait;

/// A [service](https://smithy.io/2.0/spec/service-types.html#service) shape
#[derive(Debug, Clone)]
pub struct ServiceShape {
    pub(crate) metadata: ShapeMetadata,
}

/// An [operation](https://smithy.io/2.0/spec/service-types.html#operation) shape
#[derive(Debug, Clone)]
pub struct OperationShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [resource](https://smithy.io/2.0/spec/service-types.html#resource) shape
#[derive(Debug, Clone)]
pub struct ResourceShape {
    pub(crate) metadata: ShapeMetadata,
}

// Implement ProvideShapeMetadata for all service shapes
impl ProvideShapeMetadata for ServiceShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for OperationShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for ResourceShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

// Implement constructors for service shapes
impl ServiceShape {
    /// Create a new service shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl OperationShape {
    /// Create a new operation shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl ResourceShape {
    /// Create a new resource shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

// Implement From traits for converting shape structs to Shape enum
impl From<ServiceShape> for Shape {
    fn from(shape: ServiceShape) -> Self {
        Shape::Service(shape)
    }
}

impl From<OperationShape> for Shape {
    fn from(shape: OperationShape) -> Self {
        Shape::Operation(shape)
    }
}

impl From<ResourceShape> for Shape {
    fn from(shape: ResourceShape) -> Self {
        Shape::Resource(shape)
    }
}
