/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Service shape types for the Smithy model.

use std::collections::HashMap;

use crate::shape::{
    builder::{parse_shape_id, ProvideTraitsMut},
    error::BuildError,
    ProvideShapeMetadata, Shape, ShapeMetadata,
};
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

/// Builder for creating a service shape.
#[derive(Debug, Default)]
pub struct ServiceShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    operations: Vec<ShapeId>,
    resources: Vec<ShapeId>,
    version: Option<String>,
}

impl ServiceShapeBuilder {
    /// Create a new service shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the service shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the version of the service.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add an operation to the service.
    pub fn operation(mut self, operation: ShapeId) -> Self {
        self.operations.push(operation);
        self
    }

    /// Add multiple operations to the service.
    pub fn operations(mut self, operations: Vec<ShapeId>) -> Self {
        self.operations.extend(operations);
        self
    }

    /// Add a resource to the service.
    pub fn resource(mut self, resource: ShapeId) -> Self {
        self.resources.push(resource);
        self
    }

    /// Add multiple resources to the service.
    pub fn resources(mut self, resources: Vec<ShapeId>) -> Self {
        self.resources.extend(resources);
        self
    }

    /// Build the service shape.
    pub fn build(self) -> Result<ServiceShape, BuildError> {
        let id_str = self.id.ok_or_else(|| BuildError::MissingRequiredField {
            field: "id".to_string(),
        })?;

        let id = parse_shape_id(&id_str)?;

        Ok(ServiceShape {
            metadata: ShapeMetadata::new(id, self.traits),
            operations: self.operations,
            resources: self.resources,
            version: self.version,
        })
    }
}

impl ProvideTraitsMut for ServiceShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl ServiceShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ServiceShapeBuilder {
        ServiceShapeBuilder::new()
    }
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

/// Builder for creating an operation shape.
#[derive(Debug, Default)]
pub struct OperationShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    input: Option<ShapeId>,
    output: Option<ShapeId>,
    errors: Vec<ShapeId>,
}

impl OperationShapeBuilder {
    /// Create a new operation shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the operation shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the input shape ID.
    pub fn input(mut self, input: ShapeId) -> Self {
        self.input = Some(input);
        self
    }

    /// Set the output shape ID.
    pub fn output(mut self, output: ShapeId) -> Self {
        self.output = Some(output);
        self
    }

    /// Add an error shape ID.
    pub fn error(mut self, error: ShapeId) -> Self {
        self.errors.push(error);
        self
    }

    /// Add multiple error shape IDs.
    pub fn errors(mut self, errors: Vec<ShapeId>) -> Self {
        self.errors.extend(errors);
        self
    }

    /// Build the operation shape.
    pub fn build(self) -> Result<OperationShape, BuildError> {
        let id_str = self.id.ok_or_else(|| BuildError::MissingRequiredField {
            field: "id".to_string(),
        })?;

        let id = parse_shape_id(&id_str)?;

        Ok(OperationShape {
            metadata: ShapeMetadata::new(id, self.traits),
            input: self.input,
            output: self.output,
            errors: self.errors,
        })
    }
}

impl ProvideTraitsMut for OperationShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl OperationShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> OperationShapeBuilder {
        OperationShapeBuilder::new()
    }
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

/// Builder for creating a resource shape.
#[derive(Debug, Default)]
pub struct ResourceShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    identifiers: HashMap<String, ShapeId>,
    create: Option<ShapeId>,
    read: Option<ShapeId>,
    update: Option<ShapeId>,
    delete: Option<ShapeId>,
    list: Option<ShapeId>,
    operations: Vec<ShapeId>,
    resources: Vec<ShapeId>,
}

impl ResourceShapeBuilder {
    /// Create a new resource shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the resource shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add an identifier to the resource.
    pub fn identifier(mut self, name: impl Into<String>, shape_id: ShapeId) -> Self {
        self.identifiers.insert(name.into(), shape_id);
        self
    }

    /// Set the create operation for this resource.
    pub fn create(mut self, create: ShapeId) -> Self {
        self.create = Some(create);
        self
    }

    /// Set the read operation for this resource.
    pub fn read(mut self, read: ShapeId) -> Self {
        self.read = Some(read);
        self
    }

    /// Set the update operation for this resource.
    pub fn update(mut self, update: ShapeId) -> Self {
        self.update = Some(update);
        self
    }

    /// Set the delete operation for this resource.
    pub fn delete(mut self, delete: ShapeId) -> Self {
        self.delete = Some(delete);
        self
    }

    /// Set the list operation for this resource.
    pub fn list(mut self, list: ShapeId) -> Self {
        self.list = Some(list);
        self
    }

    /// Add an operation to the resource.
    pub fn operation(mut self, operation: ShapeId) -> Self {
        self.operations.push(operation);
        self
    }

    /// Add multiple operations to the resource.
    pub fn operations(mut self, operations: Vec<ShapeId>) -> Self {
        self.operations.extend(operations);
        self
    }

    /// Add a resource to the resource.
    pub fn resource(mut self, resource: ShapeId) -> Self {
        self.resources.push(resource);
        self
    }

    /// Add multiple resources to the resource.
    pub fn resources(mut self, resources: Vec<ShapeId>) -> Self {
        self.resources.extend(resources);
        self
    }

    /// Build the resource shape.
    pub fn build(self) -> Result<ResourceShape, BuildError> {
        let id_str = self.id.ok_or_else(|| BuildError::MissingRequiredField {
            field: "id".to_string(),
        })?;

        let id = parse_shape_id(&id_str)?;

        Ok(ResourceShape {
            metadata: ShapeMetadata::new(id, self.traits),
            identifiers: self.identifiers,
            create: self.create,
            read: self.read,
            update: self.update,
            delete: self.delete,
            list: self.list,
            operations: self.operations,
            resources: self.resources,
        })
    }
}

impl ProvideTraitsMut for ResourceShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl ResourceShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ResourceShapeBuilder {
        ResourceShapeBuilder::new()
    }
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

// Implement From for service shapes
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
