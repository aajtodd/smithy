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
    /// The operations that are part of this service
    pub operations: Vec<ShapeId>,
    /// The resources that are part of this service
    pub resources: Vec<ShapeId>,
    /// The version of the service
    pub version: Option<String>,
}

/// An [operation](https://smithy.io/2.0/spec/service-types.html#operation) shape
#[derive(Debug, Clone)]
pub struct OperationShape {
    pub(crate) metadata: ShapeMetadata,
    /// The input shape ID, if any
    pub input: Option<ShapeId>,
    /// The output shape ID, if any
    pub output: Option<ShapeId>,
    /// The errors that can be thrown by this operation
    pub errors: Vec<ShapeId>,
}

/// A [resource](https://smithy.io/2.0/spec/service-types.html#resource) shape
#[derive(Debug, Clone)]
pub struct ResourceShape {
    pub(crate) metadata: ShapeMetadata,
    /// The identifiers for this resource
    pub identifiers: HashMap<String, ShapeId>,
    /// The create operation for this resource, if any
    pub create: Option<ShapeId>,
    /// The read operation for this resource, if any
    pub read: Option<ShapeId>,
    /// The update operation for this resource, if any
    pub update: Option<ShapeId>,
    /// The delete operation for this resource, if any
    pub delete: Option<ShapeId>,
    /// The list operation for this resource, if any
    pub list: Option<ShapeId>,
    /// The operations that are part of this resource
    pub operations: Vec<ShapeId>,
    /// The resources that are part of this resource
    pub resources: Vec<ShapeId>,
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
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        operations: Vec<ShapeId>,
        resources: Vec<ShapeId>,
        version: Option<String>,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            operations,
            resources,
            version,
        }
    }

    // FIXME - consider not exposing this as we usually want TopDownIndex instead
    /// Get the operations that are part of this service
    pub fn operations(&self) -> &[ShapeId] {
        &self.operations
    }

    /// Get the resources that are part of this service
    pub fn resources(&self) -> &[ShapeId] {
        &self.resources
    }

    /// Get the version of the service
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

impl OperationShape {
    /// Create a new operation shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        input: Option<ShapeId>,
        output: Option<ShapeId>,
        errors: Vec<ShapeId>,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            input,
            output,
            errors,
        }
    }

    /// Get the input shape ID, if any
    pub fn input(&self) -> Option<&ShapeId> {
        self.input.as_ref()
    }

    /// Get the output shape ID, if any
    pub fn output(&self) -> Option<&ShapeId> {
        self.output.as_ref()
    }

    /// Get the errors that can be thrown by this operation
    pub fn errors(&self) -> &[ShapeId] {
        &self.errors
    }
}

impl ResourceShape {
    /// Create a new resource shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        identifiers: HashMap<String, ShapeId>,
        create: Option<ShapeId>,
        read: Option<ShapeId>,
        update: Option<ShapeId>,
        delete: Option<ShapeId>,
        list: Option<ShapeId>,
        operations: Vec<ShapeId>,
        resources: Vec<ShapeId>,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            identifiers,
            create,
            read,
            update,
            delete,
            list,
            operations,
            resources,
        }
    }

    /// Get the identifiers for this resource
    pub fn identifiers(&self) -> &HashMap<String, ShapeId> {
        &self.identifiers
    }

    /// Get the create operation for this resource, if any
    pub fn create(&self) -> Option<&ShapeId> {
        self.create.as_ref()
    }

    /// Get the read operation for this resource, if any
    pub fn read(&self) -> Option<&ShapeId> {
        self.read.as_ref()
    }

    /// Get the update operation for this resource, if any
    pub fn update(&self) -> Option<&ShapeId> {
        self.update.as_ref()
    }

    /// Get the delete operation for this resource, if any
    pub fn delete(&self) -> Option<&ShapeId> {
        self.delete.as_ref()
    }

    /// Get the list operation for this resource, if any
    pub fn list(&self) -> Option<&ShapeId> {
        self.list.as_ref()
    }

    /// Get the operations that are part of this resource
    pub fn operations(&self) -> &[ShapeId] {
        &self.operations
    }

    /// Get the resources that are part of this resource
    pub fn resources(&self) -> &[ShapeId] {
        &self.resources
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
