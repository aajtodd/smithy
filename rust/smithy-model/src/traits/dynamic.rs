/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Dynamic trait implementation.

use crate::node::Node;
use crate::shape_id::ShapeId;
use crate::traits::{BoxTrait, Trait};
use std::any::Any;

/// A dynamic trait for representing unknown or dynamically loaded traits
///
/// This trait implementation is used for traits that are not known at compile time
/// or for which no concrete implementation exists. It stores the trait ID and value
/// and provides access to them.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::{DynamicTrait, Trait};
/// use smithy_model::shape_id::ShapeId;
/// use smithy_model::node::Node;
///
/// let id = ShapeId::new_unchecked("example#customTrait");
/// let value = Node::String("custom value".to_string());
/// let trait_ = DynamicTrait::new(id.clone(), Some(value.clone()));
///
/// assert_eq!(trait_.id(), id);
/// assert_eq!(trait_.value(), Some(&value));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicTrait {
    id: ShapeId,
    value: Option<Node>,
}

impl DynamicTrait {
    /// Create a new dynamic trait
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the trait
    /// * `value` - The optional value of the trait
    pub fn new(id: impl Into<ShapeId>, value: impl Into<Option<Node>>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
        }
    }

    /// Get the value of this trait
    pub fn value(&self) -> Option<&Node> {
        self.value.as_ref()
    }
}

impl Trait for DynamicTrait {
    fn static_id() -> ShapeId {
        // This is a placeholder - the actual ID is stored in the instance
        ShapeId::new_unchecked("smithy.synthetic#dynamic")
    }

    // Override the default implementation to return the instance-specific ID
    fn id(&self) -> ShapeId {
        self.id.clone()
    }

    fn to_node(&self) -> Node {
        self.value.clone().unwrap_or(Node::Null)
    }

    fn from_node(_node: &Node) -> Option<Self> {
        // This can't be called directly since we need the ID
        // Use new() instead
        None
    }

    fn clone_trait(&self) -> BoxTrait {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_trait() {
        let id = ShapeId::new_unchecked("example#customTrait");
        let value = Node::String("custom value".to_string());
        let trait_ = DynamicTrait::new(id.clone(), Some(value.clone()));

        // Test id and value
        assert_eq!(trait_.id(), id);
        assert_eq!(trait_.value(), Some(&value));

        // Test to_node
        let node = trait_.to_node();
        assert_eq!(node, value);

        // Test clone_trait
        let trait_box = trait_.clone_trait();
        let trait2 = trait_box.as_any().downcast_ref::<DynamicTrait>().unwrap();
        assert_eq!(trait_, *trait2);
    }

    #[test]
    fn test_dynamic_trait_without_value() {
        let id = ShapeId::new_unchecked("example#emptyTrait");
        let trait_ = DynamicTrait::new(id.clone(), None);

        // Test id and value
        assert_eq!(trait_.id(), id);
        assert_eq!(trait_.value(), None);

        // Test to_node
        let node = trait_.to_node();
        assert_eq!(node, Node::Null);
    }
}
