use crate::shape::{
    BuildError, ProvideShapeMetadata, ProvideTraitsMut, ServiceShape, Shape, ShapeMetadata,
    ShapeMetadataBuilder,
};
use crate::traits::TraitMap;
use crate::{shape, ShapeId};
use std::collections::HashMap;

/// A [resource](https://smithy.io/2.0/spec/service-types.html#resource) shape
#[derive(Debug, Clone, PartialEq)]
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

impl ResourceShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ResourceShapeBuilder {
        ResourceShapeBuilder::new()
    }
}

impl ProvideShapeMetadata for ResourceShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<ResourceShape> for Shape {
    fn from(shape: ResourceShape) -> Self {
        Shape::Resource(shape)
    }
}

/// Builder for creating a resource shape.
#[derive(Debug, Default)]
pub struct ResourceShapeBuilder {
    metadata: ShapeMetadataBuilder,
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
        self.metadata = self.metadata.id(id);
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

    /// Add a mixin to the resource shape.
    pub fn mixin(mut self, mixin: impl Into<Shape>) -> Self {
        self.metadata = self.metadata.with_mixin(mixin.into());
        self
    }

    /// Build the resource shape.
    pub fn build(self) -> Result<ResourceShape, BuildError> {
        if self.identifiers.is_empty() {
            return Err(BuildError::InvalidValue {
                field: shape::field_names::IDENTIFIERS.to_string(),
                reason: "Resource must have at least one identifier".to_string(),
            });
        }

        // Build the metadata
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Resource(_)), "resource")?;
        let metadata = self.metadata.build()?;

        Ok(ResourceShape {
            metadata,
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
    fn traits_mut(&mut self) -> &mut TraitMap {
        &mut self.metadata.introduced_traits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::HasShapeId;
    // Resource shape tests

    #[test]
    fn test_resource_shape_construction() {
        let identifier = ShapeId::new("example.foo", "ItemId").unwrap();
        let create = ShapeId::new("example.foo", "CreateItem").unwrap();
        let read = ShapeId::new("example.foo", "GetItem").unwrap();
        let update = ShapeId::new("example.foo", "UpdateItem").unwrap();
        let delete = ShapeId::new("example.foo", "DeleteItem").unwrap();
        let list = ShapeId::new("example.foo", "ListItems").unwrap();
        let child_resource = ShapeId::new("example.foo", "ItemPart").unwrap();
        let collection_operation = ShapeId::new("example.foo", "BatchGetItems").unwrap();

        let shape = ResourceShape::builder()
            .id("example.foo#Item")
            .identifier("itemId", identifier.clone())
            .create(create.clone())
            .read(read.clone())
            .update(update.clone())
            .delete(delete.clone())
            .list(list.clone())
            .resource(child_resource.clone())
            .operation(collection_operation.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#Item");
        assert_eq!(shape.identifiers.len(), 1);
        assert_eq!(shape.identifiers.get("itemId"), Some(&identifier));
        assert_eq!(shape.create, Some(create));
        assert_eq!(shape.read, Some(read));
        assert_eq!(shape.update, Some(update));
        assert_eq!(shape.delete, Some(delete));
        assert_eq!(shape.list, Some(list));
        assert_eq!(shape.resources.len(), 1);
        assert!(shape.resources.contains(&child_resource));
        assert_eq!(shape.operations.len(), 1);
        assert!(shape.operations.contains(&collection_operation));
    }

    #[test]
    fn test_resource_shape_minimal() {
        let identifier = ShapeId::new("example.foo", "ItemId").unwrap();

        let shape = ResourceShape::builder()
            .id("example.foo#Item")
            .identifier("itemId", identifier.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#Item");
        assert_eq!(shape.identifiers.len(), 1);
        assert_eq!(shape.create, None);
        assert_eq!(shape.read, None);
        assert_eq!(shape.update, None);
        assert_eq!(shape.delete, None);
        assert_eq!(shape.list, None);
        assert_eq!(shape.resources.len(), 0);
        assert_eq!(shape.operations.len(), 0);
    }

    #[test]
    fn test_resource_shape_missing_identifiers() {
        let result = ResourceShape::builder().id("example.foo#Item").build();

        assert!(result.is_err());
        match result {
            Err(BuildError::InvalidValue { field, reason }) => {
                assert_eq!(field, "identifiers");
                assert_eq!(reason, "Resource must have at least one identifier");
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }
}
