/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Service shape types for the Smithy model.

use crate::shape::ShapeId;
use crate::shape::{
    error::BuildError, Shape, ShapeBuilder, ShapeMetadata, ShapeMetadataBuilder, ShapeProperties,
};

/// A [service](https://smithy.io/2.0/spec/service-types.html#service) shape
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceShape {
    pub(crate) metadata: ShapeMetadata,
    /// The operations that are directly defined on this service (introduced operations)
    pub introduced_operations: Vec<ShapeId>,
    /// All operations that are part of this service (including those from mixins)
    pub operations: Vec<ShapeId>,
    /// The resources that are directly defined on this service (introduced resources)
    pub introduced_resources: Vec<ShapeId>,
    /// All resources that are part of this service (including those from mixins)
    pub resources: Vec<ShapeId>,
    /// The errors that are directly defined on this service (introduced errors)
    pub introduced_errors: Vec<ShapeId>,
    /// All errors that can be thrown by this service (including those from mixins)
    pub errors: Vec<ShapeId>,
    /// The version of the service directly defined on this service (introduced version)
    pub introduced_version: Option<String>,
    /// The effective version of the service (including those from mixins)
    pub version: Option<String>,
}

impl ServiceShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ServiceShapeBuilder {
        ServiceShapeBuilder::new()
    }

    /// Create a builder from this shape.
    pub fn to_builder(self) -> ServiceShapeBuilder {
        ServiceShapeBuilder {
            metadata: self.metadata.to_builder(),
            introduced_operations: self.introduced_operations,
            introduced_resources: self.introduced_resources,
            introduced_errors: self.introduced_errors,
            introduced_version: self.introduced_version,
        }
    }
}

impl ShapeProperties for ServiceShape {
    fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

// Implement From for service shapes
impl From<ServiceShape> for Shape {
    fn from(shape: ServiceShape) -> Self {
        Shape::Service(shape)
    }
}

impl AsRef<ShapeId> for ServiceShape {
    fn as_ref(&self) -> &ShapeId {
        &self.metadata.id
    }
}

/// Builder for creating a service shape.
#[derive(Debug, Default)]
pub struct ServiceShapeBuilder {
    metadata: ShapeMetadataBuilder,
    introduced_operations: Vec<ShapeId>,
    introduced_resources: Vec<ShapeId>,
    introduced_errors: Vec<ShapeId>,
    introduced_version: Option<String>,
}

impl ServiceShapeBuilder {
    /// Create a new service shape builder.
    fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the service shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
        self
    }

    /// Set the version of the service.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.introduced_version = Some(version.into());
        self
    }

    /// Add an operation to the service.
    pub fn operation(mut self, operation: ShapeId) -> Self {
        self.introduced_operations.push(operation);
        self
    }

    /// Add multiple operations to the service.
    pub fn operations(mut self, operations: impl IntoIterator<Item = ShapeId>) -> Self {
        self.introduced_operations.extend(operations);
        self
    }

    /// Add a resource to the service.
    pub fn resource(mut self, resource: ShapeId) -> Self {
        self.introduced_resources.push(resource);
        self
    }

    /// Add multiple resources to the service.
    pub fn resources(mut self, resources: impl IntoIterator<Item = ShapeId>) -> Self {
        self.introduced_resources.extend(resources);
        self
    }

    /// Add an error to the service.
    pub fn error(mut self, error: ShapeId) -> Self {
        self.introduced_errors.push(error);
        self
    }

    /// Add multiple errors to the service.
    pub fn errors(mut self, errors: impl IntoIterator<Item = ShapeId>) -> Self {
        self.introduced_errors.extend(errors);
        self
    }

    /// Build the service shape.
    pub fn build(self) -> Result<ServiceShape, BuildError> {
        // Build the metadata
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Service(_)), "service")?;

        let metadata = self.metadata.build()?;

        // Start with introduced properties
        let mut operations = self.introduced_operations.clone();
        let mut resources = self.introduced_resources.clone();
        let mut errors = self.introduced_errors.clone();
        let mut version = self.introduced_version.clone();

        // Collect properties from mixins
        for mixin in &metadata.mixins {
            if let Shape::Service(service) = mixin {
                // Add operations from mixin
                for op in &service.operations {
                    if !operations.contains(op) {
                        operations.push(op.clone());
                    }
                }

                // Add resources from mixin
                for res in &service.resources {
                    if !resources.contains(res) {
                        resources.push(res.clone());
                    }
                }

                // Add errors from mixin
                for err in &service.errors {
                    if !errors.contains(err) {
                        errors.push(err.clone());
                    }
                }

                // Take version from mixin if not already set
                if version.is_none() {
                    version = service.version.clone();
                }
            }
        }

        Ok(ServiceShape {
            metadata,
            introduced_operations: self.introduced_operations,
            operations,
            introduced_resources: self.introduced_resources,
            resources,
            introduced_errors: self.introduced_errors,
            errors,
            introduced_version: self.introduced_version,
            version,
        })
    }
}

impl ShapeBuilder for ServiceShapeBuilder {
    fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder {
        &mut self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::operation::OperationShape;
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
        assert_eq!(shape.introduced_version, Some("2023-01-01".to_string()));
        assert_eq!(shape.version, Some("2023-01-01".to_string()));
        assert_eq!(shape.introduced_operations.len(), 2);
        assert_eq!(shape.operations.len(), 2);
        assert_eq!(shape.introduced_resources.len(), 1);
        assert_eq!(shape.resources.len(), 1);
        assert!(shape.introduced_operations.contains(&operation1));
        assert!(shape.introduced_operations.contains(&operation2));
        assert!(shape.operations.contains(&operation1));
        assert!(shape.operations.contains(&operation2));
        assert!(shape.introduced_resources.contains(&resource));
        assert!(shape.resources.contains(&resource));
    }

    #[test]
    fn test_service_shape_without_version() {
        let shape = ServiceShape::builder()
            .id("example.foo#MyService")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyService");
        assert_eq!(shape.introduced_version, None);
        assert_eq!(shape.version, None);
        assert_eq!(shape.introduced_operations.len(), 0);
        assert_eq!(shape.operations.len(), 0);
        assert_eq!(shape.introduced_resources.len(), 0);
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

#[test]
fn test_service_shape_with_mixins() {
    // Create a mixin service
    let mixin_op = ShapeId::new("example.foo", "MixinOp").unwrap();
    let mixin_res = ShapeId::new("example.foo", "MixinRes").unwrap();
    let mixin_err = ShapeId::new("example.foo", "MixinErr").unwrap();

    let mixin = ServiceShape::builder()
        .id("example.foo#MixinService")
        .version("2023-01-01")
        .operation(mixin_op.clone())
        .resource(mixin_res.clone())
        .error(mixin_err.clone())
        .build()
        .unwrap();

    // Add @mixin trait to the mixin
    use crate::traits::type_refinement::Mixin;
    let mut mixin_with_trait = mixin.clone();
    mixin_with_trait
        .metadata
        .effective_traits
        .insert(Box::new(Mixin::new()));

    // Create a service that uses the mixin
    let direct_op = ShapeId::new("example.foo", "DirectOp").unwrap();
    let direct_res = ShapeId::new("example.foo", "DirectRes").unwrap();
    let direct_err = ShapeId::new("example.foo", "DirectErr").unwrap();

    let service = ServiceShape::builder()
        .id("example.foo#MainService")
        .operation(direct_op.clone())
        .resource(direct_res.clone())
        .error(direct_err.clone())
        .mixin(mixin_with_trait)
        .build()
        .unwrap();

    // Check introduced properties
    assert_eq!(service.introduced_operations.len(), 1);
    assert!(service.introduced_operations.contains(&direct_op));
    assert_eq!(service.introduced_resources.len(), 1);
    assert!(service.introduced_resources.contains(&direct_res));
    assert_eq!(service.introduced_errors.len(), 1);
    assert!(service.introduced_errors.contains(&direct_err));
    assert_eq!(service.introduced_version, None);

    // Check effective properties
    assert_eq!(service.operations.len(), 2);
    assert!(service.operations.contains(&direct_op));
    assert!(service.operations.contains(&mixin_op));
    assert_eq!(service.resources.len(), 2);
    assert!(service.resources.contains(&direct_res));
    assert!(service.resources.contains(&mixin_res));
    assert_eq!(service.errors.len(), 2);
    assert!(service.errors.contains(&direct_err));
    assert!(service.errors.contains(&mixin_err));
    assert_eq!(service.version, Some("2023-01-01".to_string()));
}
