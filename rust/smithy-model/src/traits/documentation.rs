/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Documentation related traits
//!
//! See [documentation traits](https://smithy.io/2.0/spec/documentation-traits.html#)
//! in the Smithy specification.

use crate::define_simple_trait;
use crate::node::Node;
use crate::shape::ShapeId;
use crate::traits::{BoxTrait, Trait};
use std::any::Any;
use std::collections::HashMap;

define_simple_trait! {
    /// The documentation trait
    ///
    /// This trait provides documentation for a shape. It contains a string value
    /// that describes the shape's purpose and usage.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::traits::documentation::Documentation;
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

    trait_id: "smithy.api", "documentation";
    type: string;
}

/// The deprecated trait
///
/// This trait indicates that a shape is deprecated and may be removed in a future version.
/// It can optionally include a message and a date when the shape was deprecated.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::documentation::Deprecated;
/// use smithy_model::traits::Trait;
/// use smithy_model::node::Node;
///
/// // Create a simple deprecated trait
/// let deprecated = Deprecated::new();
/// assert!(deprecated.message.is_none());
/// assert!(deprecated.since.is_none());
///
/// // Create a deprecated trait with message and date
/// let deprecated = Deprecated::new()
///     .with_message("Use newShape instead")
///     .with_since("2020-01-01");
/// assert_eq!(deprecated.message.as_deref(), Some("Use newShape instead"));
/// assert_eq!(deprecated.since.as_deref(), Some("2020-01-01"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deprecated {
    /// Optional message explaining the deprecation
    pub message: Option<String>,
    /// Optional date when the shape was deprecated
    pub since: Option<String>,
}

impl Deprecated {
    /// Creates a new deprecated trait with no message or date
    pub fn new() -> Self {
        Self {
            message: None,
            since: None,
        }
    }

    /// Sets the message for this deprecated trait
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the since date for this deprecated trait
    pub fn with_since(mut self, since: impl Into<String>) -> Self {
        self.since = Some(since.into());
        self
    }
}

impl Default for Deprecated {
    fn default() -> Self {
        Self::new()
    }
}

impl Trait for Deprecated {
    fn static_id() -> &'static ShapeId {
        const TRAIT_ID: &ShapeId = &ShapeId::new_static("smithy.api", "deprecated");
        TRAIT_ID
    }

    fn id(&self) -> &ShapeId {
        Self::static_id()
    }

    fn to_node(&self) -> Node {
        let mut map = HashMap::new();

        if let Some(message) = &self.message {
            map.insert("message".to_string(), Node::String(message.clone()));
        }

        if let Some(since) = &self.since {
            map.insert("since".to_string(), Node::String(since.clone()));
        }

        Node::Object(map)
    }

    fn from_node(node: &Node) -> Option<Self> {
        match node {
            Node::Object(map) => {
                let message = map
                    .get("message")
                    .and_then(|n| n.as_str())
                    .map(String::from);
                let since = map.get("since").and_then(|n| n.as_str()).map(String::from);

                Some(Self { message, since })
            }
            Node::Null => Some(Self::new()),
            _ => None,
        }
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

    #[test]
    fn test_deprecated_trait() {
        // Test empty deprecated
        let deprecated = Deprecated::new();
        assert!(deprecated.message.is_none());
        assert!(deprecated.since.is_none());

        // Test with message and since
        let deprecated = Deprecated::new()
            .with_message("Use newShape instead")
            .with_since("2020-01-01");
        assert_eq!(deprecated.message.as_deref(), Some("Use newShape instead"));
        assert_eq!(deprecated.since.as_deref(), Some("2020-01-01"));

        // Test static_id
        assert_eq!(Deprecated::static_id().to_string(), "smithy.api#deprecated");

        // Test to_node and from_node
        let node = deprecated.to_node();
        if let Node::Object(map) = &node {
            assert_eq!(
                map.get("message").and_then(|n| n.as_str()),
                Some("Use newShape instead")
            );
            assert_eq!(
                map.get("since").and_then(|n| n.as_str()),
                Some("2020-01-01")
            );
        } else {
            panic!("Expected Node::Object");
        }

        let deprecated2 = Deprecated::from_node(&node).unwrap();
        assert_eq!(deprecated, deprecated2);

        // Test clone_trait
        let deprecated_box = deprecated.clone_trait();
        let deprecated3 = deprecated_box
            .as_any()
            .downcast_ref::<Deprecated>()
            .unwrap();
        assert_eq!(deprecated, *deprecated3);
    }

    #[test]
    fn test_deprecated_from_invalid_node() {
        let node = Node::String("invalid".to_string());
        assert!(Deprecated::from_node(&node).is_none());
    }
}
