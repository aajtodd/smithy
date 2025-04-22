/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Documentation trait implementation.

use crate::node::Node;
use crate::shape_id::ShapeId;
use crate::traits::{BoxTrait, Trait};
use std::any::Any;

/// The documentation trait
///
/// This trait provides documentation for a shape. It contains a string value
/// that describes the shape's purpose and usage.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::Documentation;
/// use smithy_model::traits::Trait;
/// use smithy_model::node::Node;
///
/// let doc = Documentation("This is a documentation string".to_string());
/// assert_eq!(doc.0, "This is a documentation string");
///
/// // Convert to and from a Node
/// let node = doc.to_node();
/// let doc2 = Documentation::from_node(&node).unwrap();
/// assert_eq!(doc.0, doc2.0);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Documentation(pub String);

const TRAIT_ID: &ShapeId = &ShapeId::new_static("smithy.api", "documentation");

impl Trait for Documentation {
    fn static_id() -> &'static ShapeId {
        TRAIT_ID
    }

    fn id(&self) -> &ShapeId {
        Self::static_id()
    }

    fn to_node(&self) -> Node {
        Node::String(self.0.clone())
    }

    fn from_node(node: &Node) -> Option<Self> {
        node.as_str().map(|s| Self(s.to_string()))
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
    fn test_documentation_trait() {
        let doc = Documentation("This is a documentation string".to_string());
        assert_eq!(doc.0, "This is a documentation string");

        // Test static_id
        assert_eq!(
            Documentation::static_id().to_string(),
            "smithy.api#documentation"
        );

        // Test to_node and from_node
        let node = doc.to_node();
        let doc2 = Documentation::from_node(&node).unwrap();
        assert!(matches!(node, Node::String(s) if s == "This is a documentation string"));

        assert_eq!(doc.0, doc2.0);

        // Test clone_trait
        let doc_box = doc.clone_trait();
        let doc3 = doc_box.as_any().downcast_ref::<Documentation>().unwrap();
        assert_eq!(doc.0, doc3.0);
    }

    #[test]
    fn test_documentation_from_invalid_node() {
        let node = Node::Number(42.0.into());
        assert!(Documentation::from_node(&node).is_none());
    }
}
