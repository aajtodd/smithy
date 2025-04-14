/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Service shape types for the Smithy model.

use crate::shape::resource::ResourceShape;
use crate::shape::{
    builder::ProvideTraitsMut, error::BuildError, ProvideShapeMetadata, Shape, ShapeMetadata,
    ShapeMetadataBuilder,
};
use crate::shape_id::ShapeId;
use crate::traits::TraitMap;

/// A [service](https://smithy.io/2.0/spec/service-types.html#service) shape
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceShape {
    pub(crate) metadata: ShapeMetadata,
    /// The operations that are part of this service
    pub operations: Vec<ShapeId>,
    /// The resources that are part of this service
    pub resources: Vec<ShapeId>,
    /// The errors that can be thrown by this service
    pub errors: Vec<ShapeId>,
    /// The version of the service
    pub version: Option<String>,
}

/// Builder for creating a service shape.
#[derive(Debug, Default)]
pub struct ServiceShapeBuilder {
    metadata: ShapeMetadataBuilder,
    operations: Vec<ShapeId>,
    resources: Vec<ShapeId>,
    errors: Vec<ShapeId>,
    version: Option<String>,
}

impl ServiceShapeBuilder {
    /// Create a new service shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the service shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
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
    pub fn operations(mut self, operations: impl IntoIterator<Item = ShapeId>) -> Self {
        self.operations.extend(operations);
        self
    }

    /// Add a resource to the service.
    pub fn resource(mut self, resource: ShapeId) -> Self {
        self.resources.push(resource);
        self
    }

    /// Add multiple resources to the service.
    pub fn resources(mut self, resources: impl IntoIterator<Item = ShapeId>) -> Self {
        self.resources.extend(resources);
        self
    }

    /// Add an error to the service.
    pub fn error(mut self, error: ShapeId) -> Self {
        self.errors.push(error);
        self
    }

    /// Add multiple errors to the service.
    pub fn errors(mut self, errors: impl IntoIterator<Item = ShapeId>) -> Self {
        self.errors.extend(errors);
        self
    }

    /// Add a mixin to the service shape.
    pub fn mixin(mut self, mixin: impl Into<Shape>) -> Self {
        self.metadata = self.metadata.with_mixin(mixin.into());
        self
    }

    /// Build the service shape.
    pub fn build(self) -> Result<ServiceShape, BuildError> {
        // Build the metadata
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Service(_)), "service")?;

        let metadata = self.metadata.build()?;

        Ok(ServiceShape {
            metadata,
            operations: self.operations,
            resources: self.resources,
            errors: self.errors,
            version: self.version,
        })
    }
}

impl ProvideTraitsMut for ServiceShapeBuilder {
    fn traits_mut(&mut self) -> &mut TraitMap {
        &mut self.metadata.introduced_traits
    }
}

impl ServiceShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ServiceShapeBuilder {
        ServiceShapeBuilder::new()
    }
}

// Implement ProvideShapeMetadata for all service shapes
impl ProvideShapeMetadata for ServiceShape {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::operation::OperationShape;
    use crate::shape::HasShapeId;
    // Service shape tests

    #[test]
    fn test_service_shape_construction() {
        let operation1 = ShapeId::new("example.foo", "GetItem").unwrap();
        let operation2 = ShapeId::new("example.foo", "PutItem").unwrap();
        let resource = ShapeId::new("example.foo", "Item").unwrap();

        let shape = ServiceShape::builder()
            .id("example.foo#MyService")
            .version("2023-01-01")
            .operation(operation1.clone())
            .operation(operation2.clone())
            .resource(resource.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyService");
        assert_eq!(shape.version, Some("2023-01-01".to_string()));
        assert_eq!(shape.operations.len(), 2);
        assert_eq!(shape.resources.len(), 1);
        assert!(shape.operations.contains(&operation1));
        assert!(shape.operations.contains(&operation2));
        assert!(shape.resources.contains(&resource));
    }

    #[test]
    fn test_service_shape_without_version() {
        let shape = ServiceShape::builder()
            .id("example.foo#MyService")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyService");
        assert_eq!(shape.version, None);
        assert_eq!(shape.operations.len(), 0);
        assert_eq!(shape.resources.len(), 0);
    }

    // Operation shape tests

    #[test]
    fn test_operation_shape_construction() {
        let input = ShapeId::new("example.foo", "MyInput").unwrap();
        let output = ShapeId::new("example.foo", "MyOutput").unwrap();
        let error1 = ShapeId::new("example.foo", "Error1").unwrap();
        let error2 = ShapeId::new("example.foo", "Error2").unwrap();

        let shape = OperationShape::builder()
            .id("example.foo#MyOperation")
            .input(input.clone())
            .output(output.clone())
            .error(error1.clone())
            .error(error2.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyOperation");
        assert_eq!(shape.input, Some(input));
        assert_eq!(shape.output, Some(output));
        assert_eq!(shape.errors.len(), 2);
        assert!(shape.errors.contains(&error1));
        assert!(shape.errors.contains(&error2));
    }

    #[test]
    fn test_operation_shape_without_io() {
        let shape = OperationShape::builder()
            .id("example.foo#MyOperation")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyOperation");
        assert_eq!(shape.input, None);
        assert_eq!(shape.output, None);
        assert_eq!(shape.errors.len(), 0);
    }
}
