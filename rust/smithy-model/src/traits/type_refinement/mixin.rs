/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Mixin trait implementation.

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::node::Node;
use crate::shape::ShapeId;
use crate::traits::{BoxTrait, Trait};

/// The mixin trait
///
/// This trait marks a shape as a mixin, which allows it to be used as a template
/// for other shapes. When a shape uses a mixin, it inherits the members and traits
/// of the mixin shape.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::{type_refinement::Mixin, Trait};
/// use smithy_model::shape::ShapeId;
///
/// let mixin = Mixin::new();
/// assert!(mixin.local_traits.is_empty());
///
/// let mixin_with_local_traits = Mixin::with_local_traits(vec![
///     ShapeId::new_unchecked("smithy.api#private")
/// ]);
/// assert_eq!(mixin_with_local_traits.local_traits.len(), 1);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Mixin {
    /// Traits that should not be copied to shapes that use this mixin
    pub local_traits: HashSet<ShapeId>,
}

impl Mixin {
    /// Create a new mixin trait with no local traits
    pub fn new() -> Self {
        Self {
            local_traits: HashSet::new(),
        }
    }

    /// Create a new mixin trait with the specified local traits
    pub fn with_local_traits(local_traits: impl IntoIterator<Item = ShapeId>) -> Self {
        let local_traits: HashSet<_> = local_traits.into_iter().collect();
        Self { local_traits }
    }
}

const TRAIT_ID: &ShapeId = &ShapeId::new_static("smithy.api", "mixin");

impl Trait for Mixin {
    fn static_id() -> &'static ShapeId {
        TRAIT_ID
    }

    fn id(&self) -> &ShapeId {
        Self::static_id()
    }

    fn to_node(&self) -> Node {
        let mut obj = HashMap::new();
        if !self.local_traits.is_empty() {
            let local_traits = self
                .local_traits
                .iter()
                .map(|id| Node::String(id.to_string()))
                .collect::<Vec<_>>();
            obj.insert("localTraits".to_string(), Node::Array(local_traits));
        }
        Node::Object(obj)
    }

    fn from_node(node: &Node) -> Option<Self> {
        match node {
            Node::Object(obj) => {
                let local_traits = if let Some(Node::Array(arr)) = obj.get("localTraits") {
                    arr.iter()
                        .filter_map(|n| n.as_str())
                        .filter_map(|s| ShapeId::from_str(s).ok())
                        .collect()
                } else {
                    Vec::new()
                };
                Some(Mixin::with_local_traits(local_traits))
            }
            Node::Bool(true) => Some(Mixin::new()),
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

impl Default for Mixin {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixin_new() {
        let mixin = Mixin::new();
        assert!(mixin.local_traits.is_empty());
    }

    #[test]
    fn test_mixin_with_local_traits() {
        let local_traits = vec![
            ShapeId::new_unchecked("smithy.api#private"),
            ShapeId::new_unchecked("smithy.api#deprecated"),
        ];
        let mixin = Mixin::with_local_traits(local_traits.clone());
        let local_traits: HashSet<_> = local_traits.into_iter().collect();
        assert_eq!(mixin.local_traits, local_traits);
    }

    #[test]
    fn test_mixin_to_node_empty() {
        let mixin = Mixin::new();
        let node = mixin.to_node();
        assert_eq!(node, Node::Object(HashMap::new()));
    }

    #[test]
    fn test_mixin_to_node_with_local_traits() {
        let local_traits = vec![
            ShapeId::new_unchecked("smithy.api#private"),
            ShapeId::new_unchecked("smithy.api#deprecated"),
        ];
        let mixin = Mixin::with_local_traits(local_traits);

        let node = mixin.to_node();

        match node {
            Node::Object(obj) => {
                assert!(obj.contains_key("localTraits"));
                match obj.get("localTraits") {
                    Some(Node::Array(arr)) => {
                        assert_eq!(arr.len(), 2);
                        assert!(arr.contains(&Node::String("smithy.api#private".to_string())));
                        assert!(arr.contains(&Node::String("smithy.api#deprecated".to_string())));
                    }
                    _ => panic!("Expected Array for localTraits"),
                }
            }
            _ => panic!("Expected Object node"),
        }
    }

    #[test]
    fn test_mixin_from_node_empty_object() {
        let node = Node::Object(HashMap::new());
        let mixin = Mixin::from_node(&node).unwrap();
        assert!(mixin.local_traits.is_empty());
    }

    #[test]
    fn test_mixin_from_node_bool_true() {
        let node = Node::Bool(true);
        let mixin = Mixin::from_node(&node).unwrap();
        assert!(mixin.local_traits.is_empty());
    }

    #[test]
    fn test_mixin_from_node_with_local_traits() {
        let mut obj = HashMap::new();
        let local_traits = vec![
            Node::String("smithy.api#private".to_string()),
            Node::String("smithy.api#deprecated".to_string()),
        ];
        obj.insert("localTraits".to_string(), Node::Array(local_traits));
        let node = Node::Object(obj);

        let mixin = Mixin::from_node(&node).unwrap();

        assert_eq!(mixin.local_traits.len(), 2);
        assert!(mixin
            .local_traits
            .contains(&ShapeId::new_unchecked("smithy.api#private")));
        assert!(mixin
            .local_traits
            .contains(&ShapeId::new_unchecked("smithy.api#deprecated")));
    }

    #[test]
    fn test_mixin_from_node_invalid() {
        let node = Node::String("not a mixin".to_string());
        assert!(Mixin::from_node(&node).is_none());
    }

    #[test]
    fn test_mixin_trait_id() {
        assert_eq!(Mixin::static_id().to_string(), "smithy.api#mixin");
    }
}
