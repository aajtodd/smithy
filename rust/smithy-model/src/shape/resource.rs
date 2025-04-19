use crate::shape::{
    BuildError, HasShapeId, ProvideShapeMetadata, ProvideTraitsMut, ServiceShape, Shape,
    ShapeMetadata, ShapeMetadataBuilder,
};
use crate::traits::{Mixin, Trait, TraitMap};
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
    /// The operations directly defined on this resource (introduced operations)
    pub introduced_operations: Vec<ShapeId>,
    /// All operations that are part of this resource (including those from mixins)
    pub operations: Vec<ShapeId>,
    /// Collection operations for this resource
    pub collection_operations: Vec<ShapeId>,
    /// The resources directly defined on this resource (introduced resources)
    pub introduced_resources: Vec<ShapeId>,
    /// All resources that are part of this resource (including those from mixins)
    pub resources: Vec<ShapeId>,
}

impl ResourceShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ResourceShapeBuilder {
        ResourceShapeBuilder::new()
    }

    /// Create a builder for this shape.
    pub fn to_builder(&self) -> ResourceShapeBuilder {
        let mut builder = ResourceShapeBuilder::default();
        builder.metadata = self.metadata.to_builder();
        builder.identifiers = self.identifiers.clone();
        builder.create = self.create.clone();
        builder.read = self.read.clone();
        builder.update = self.update.clone();
        builder.delete = self.delete.clone();
        builder.list = self.list.clone();
        builder.introduced_operations = self.introduced_operations.clone();
        builder.collection_operations = self.collection_operations.clone();
        builder.introduced_resources = self.introduced_resources.clone();

        builder
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
    introduced_operations: Vec<ShapeId>,
    collection_operations: Vec<ShapeId>,
    introduced_resources: Vec<ShapeId>,
}

impl ResourceShapeBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the shape ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
        self
    }

    /// Add an identifier to the resource.
    pub fn identifier(mut self, name: impl Into<String>, target: ShapeId) -> Self {
        self.identifiers.insert(name.into(), target);
        self
    }

    /// Set the create operation for the resource.
    pub fn create(mut self, create: ShapeId) -> Self {
        self.create = Some(create);
        self
    }

    /// Set the read operation for the resource.
    pub fn read(mut self, read: ShapeId) -> Self {
        self.read = Some(read);
        self
    }

    /// Set the update operation for the resource.
    pub fn update(mut self, update: ShapeId) -> Self {
        self.update = Some(update);
        self
    }

    /// Set the delete operation for the resource.
    pub fn delete(mut self, delete: ShapeId) -> Self {
        self.delete = Some(delete);
        self
    }

    /// Set the list operation for the resource.
    pub fn list(mut self, list: ShapeId) -> Self {
        self.list = Some(list);
        self
    }

    /// Add an operation to the resource.
    pub fn operation(mut self, operation: ShapeId) -> Self {
        self.introduced_operations.push(operation);
        self
    }

    /// Add a collection operation to the resource.
    pub fn collection_operation(mut self, operation: ShapeId) -> Self {
        self.collection_operations.push(operation);
        self
    }

    /// Add multiple collection operations to the resource.
    pub fn collection_operations(mut self, operations: Vec<ShapeId>) -> Self {
        self.collection_operations.extend(operations);
        self
    }

    /// Add a resource to the resource.
    pub fn resource(mut self, resource: ShapeId) -> Self {
        self.introduced_resources.push(resource);
        self
    }

    /// Add multiple resources to the resource.
    pub fn resources(mut self, resources: Vec<ShapeId>) -> Self {
        self.introduced_resources.extend(resources);
        self
    }

    /// Add a mixin to the resource shape.
    pub fn mixin(mut self, mixin: impl Into<Shape>) -> Self {
        self.metadata = self.metadata.with_mixin(mixin.into());
        self
    }

    /// Build the resource shape.
    pub fn build(self) -> Result<ResourceShape, BuildError> {
        // Build the metadata
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Resource(_)), "resource")?;
        let metadata = self.metadata.build()?;

        // Check if this is a mixin resource
        let is_mixin = metadata.effective_traits.contains_key(Mixin::static_id());

        // Resource mixins cannot have identifiers, CRUD operations, etc.
        if is_mixin {
            if !self.identifiers.is_empty()
                || self.create.is_some()
                || self.read.is_some()
                || self.update.is_some()
                || self.delete.is_some()
                || self.list.is_some()
            {
                return Err(BuildError::InvalidValue {
                    field: shape::field_names::MIXIN.to_string(),
                    reason: "Resource shapes with the mixin trait may not define identifiers or lifecycle operations".to_string(),
                });
            }
        } else if self.identifiers.is_empty() {
            // Non-mixin resources must have at least one identifier
            return Err(BuildError::InvalidValue {
                field: shape::field_names::IDENTIFIERS.to_string(),
                reason: "Resource must have at least one identifier".to_string(),
            });
        }

        // For non-mixin resources, collect operations from mixins
        let mut operations = self.introduced_operations.clone();
        let mut resources = self.introduced_resources.clone();

        // Collect operations and resources from mixins
        for mixin in &metadata.mixins {
            if let Shape::Resource(resource) = mixin {
                // Add operations from mixin
                for op in &resource.operations {
                    if !operations.contains(op) {
                        operations.push(op.clone());
                    }
                }

                // Add resources from mixin
                for res in &resource.resources {
                    if !resources.contains(res) {
                        resources.push(res.clone());
                    }
                }
            }
        }

        Ok(ResourceShape {
            metadata,
            identifiers: self.identifiers,
            create: self.create,
            read: self.read,
            update: self.update,
            delete: self.delete,
            list: self.list,
            introduced_operations: self.introduced_operations,
            operations,
            collection_operations: self.collection_operations,
            introduced_resources: self.introduced_resources,
            resources,
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
    use crate::shape::{HasShapeId, ShapeBuilderExt};
    // Resource shape tests

    #[test]
    fn test_resource_shape_construction() {
        let identifier = ShapeId::new_unchecked("example.foo#ItemId");
        let create = ShapeId::new_unchecked("example.foo#CreateItem");
        let read = ShapeId::new_unchecked("example.foo#GetItem");
        let update = ShapeId::new_unchecked("example.foo#UpdateItem");
        let delete = ShapeId::new_unchecked("example.foo#DeleteItem");
        let list = ShapeId::new_unchecked("example.foo#ListItems");
        let child_resource = ShapeId::new_unchecked("example.foo#ItemPart");
        let operation = ShapeId::new_unchecked("example.foo#GetItems");
        let collection_operation = ShapeId::new_unchecked("example.foo#BatchGetItems");

        let shape = ResourceShape::builder()
            .id("example.foo#Item")
            .identifier("itemId", identifier.clone())
            .create(create.clone())
            .read(read.clone())
            .update(update.clone())
            .delete(delete.clone())
            .list(list.clone())
            .resource(child_resource.clone())
            .operation(operation.clone())
            .collection_operation(collection_operation.clone())
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
        assert_eq!(shape.introduced_resources.len(), 1);
        assert!(shape.introduced_resources.contains(&child_resource));
        assert_eq!(shape.resources.len(), 1);
        assert!(shape.resources.contains(&child_resource));
        assert_eq!(shape.introduced_operations.len(), 1);
        assert!(shape.introduced_operations.contains(&operation));
        assert_eq!(shape.operations.len(), 1);
        assert!(shape.operations.contains(&operation));
        assert_eq!(shape.collection_operations.len(), 1);
        assert!(shape.collection_operations.contains(&collection_operation));
    }

    #[test]
    fn test_resource_shape_minimal() {
        let identifier = ShapeId::new_unchecked("example.foo#ItemId");

        let shape = ResourceShape::builder()
            .id("example.foo#Item")
            .identifier("itemId", identifier.clone())
            .build()
            .unwrap();

        assert_eq!(shape.identifiers.len(), 1);
        assert_eq!(shape.create, None);
        assert_eq!(shape.read, None);
        assert_eq!(shape.update, None);
        assert_eq!(shape.delete, None);
        assert_eq!(shape.list, None);
        assert_eq!(shape.introduced_resources.len(), 0);
        assert_eq!(shape.resources.len(), 0);
        assert_eq!(shape.introduced_operations.len(), 0);
        assert_eq!(shape.operations.len(), 0);
        assert_eq!(shape.collection_operations.len(), 0);
    }

    #[test]
    fn test_resource_shape_missing_identifiers() {
        let result = ResourceShape::builder().id("example.foo#Item").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_resource_mixin_validation() {
        let result = ResourceShape::builder()
            .id("example.foo#MixinResource")
            .identifier("id", ShapeId::new_unchecked("example.foo#Id"))
            .with_trait(Mixin::new())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_resource_shape_with_mixins() {
        // Create a mixin resource
        let mixin_op = ShapeId::new_unchecked("example.foo#MixinOp");
        let mixin_res = ShapeId::new_unchecked("example.foo#MixinRes");

        // Add the mixin trait to make it valid
        let mixin = ResourceShape::builder()
            .id("example.foo#MixinResource")
            .with_trait(Mixin::new())
            .operation(mixin_op.clone())
            .resource(mixin_res.clone())
            .build()
            .unwrap();

        // Create a resource that uses the mixin
        let direct_op = ShapeId::new_unchecked("example.foo#DirectOp");
        let direct_res = ShapeId::new_unchecked("example.foo#DirectRes");
        let direct_collection_op = ShapeId::new_unchecked("example.foo#DirectCollectionOp");
        let identifier = ShapeId::new_unchecked("example.foo#Id");

        let resource = ResourceShape::builder()
            .id("example.foo#Resource")
            .identifier("id", identifier)
            .operation(direct_op.clone())
            .collection_operation(direct_collection_op.clone())
            .resource(direct_res.clone())
            .mixin(mixin)
            .build()
            .unwrap();

        // Check operations and resources include those from the mixin
        assert_eq!(resource.introduced_operations.len(), 1);
        assert!(resource.introduced_operations.contains(&direct_op));
        assert_eq!(resource.operations.len(), 2);
        assert!(resource.operations.contains(&direct_op));
        assert!(resource.operations.contains(&mixin_op));
        assert_eq!(resource.collection_operations.len(), 1);
        assert!(resource
            .collection_operations
            .contains(&direct_collection_op));
        assert_eq!(resource.introduced_resources.len(), 1);
        assert!(resource.introduced_resources.contains(&direct_res));
        assert_eq!(resource.resources.len(), 2);
        assert!(resource.resources.contains(&direct_res));
        assert!(resource.resources.contains(&mixin_res));
    }
}
