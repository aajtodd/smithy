/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Deprecated trait implementation.

use std::any::Any;
use std::collections::HashMap;

use crate::node::Node;
use crate::shape_id::ShapeId;
use crate::traits::{BoxTrait, Trait};

/// The deprecated trait
///
/// This trait indicates that a shape is deprecated and may be removed in a future version.
/// It can optionally include a message and a date when the shape was deprecated.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::Deprecated;
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

const TRAIT_ID: &'static ShapeId = &ShapeId::new_static("smithy.api", "deprecated");

impl Trait for Deprecated {
    fn static_id() -> &'static ShapeId {
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
