/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Required trait implementation.

use crate::node::Node;
use crate::shape_id::ShapeId;
use crate::traits::{BoxTrait, Trait};
use std::any::Any;

/// The required trait
///
/// This trait indicates that a member is required in its container.
/// It is a marker trait with no value.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::Required;
/// use smithy_model::traits::Trait;
/// use smithy_model::node::Node;
///
/// let required = Required;
///
/// // Convert to and from a Node
/// let node = required.to_node();
/// let required2 = Required::from_node(&node).unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Required;

const TRAIT_ID: &ShapeId = &ShapeId::new_static("smithy.api", "required");

impl Trait for Required {
    fn static_id() -> &'static ShapeId {
        TRAIT_ID
    }

    fn id(&self) -> &ShapeId {
        Self::static_id()
    }

    fn to_node(&self) -> Node {
        Node::Object(Default::default())
    }

    fn from_node(node: &Node) -> Option<Self> {
        match node {
            Node::Object(_) | Node::Null => Some(Self),
            _ => None,
        }
    }

    fn clone_trait(&self) -> BoxTrait {
        Box::new(Self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_trait() {
        let required = Required;

        // Test static_id
        assert_eq!(Required::static_id().to_string(), "smithy.api#required");

        // Test to_node and from_node
        let node = required.to_node();
        assert!(matches!(node, Node::Object(_)));

        let required2 = Required::from_node(&node).unwrap();
        assert_eq!(required, required2);

        // Test clone_trait
        let required_box = required.clone_trait();
        let required3 = required_box.as_any().downcast_ref::<Required>().unwrap();
        assert_eq!(required, *required3);
    }

    #[test]
    fn test_required_from_invalid_node() {
        let node = Node::String("invalid".to_string());
        assert!(Required::from_node(&node).is_none());
    }
}
