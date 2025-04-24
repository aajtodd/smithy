/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Trait implementations for the Smithy model.
//!
//! This module provides the [Trait] type and its implementations, which represent the
//! different kinds of traits in a Smithy model.

use std::any::Any;
use std::collections::HashMap;
use std::fmt;

use crate::node::Node;
use crate::shape_id::ShapeId;

mod deprecated;
mod documentation;
mod dynamic;
mod mixin;
mod registry;
mod required;
pub mod type_refinement;

pub use deprecated::Deprecated;
pub use documentation::Documentation;
pub use dynamic::DynamicTrait;
pub use mixin::Mixin;
pub use registry::TraitRegistry;
pub use required::Required;

/// Type alias for boxed trait objects
pub type BoxTrait = Box<dyn Trait>;

// TODO - could we make this simpler by having a custom derive `Trait(id)` proc macro or something?

/// Trait for implementing Smithy traits
///
/// This trait defines the interface for all Smithy traits. Each trait implementation
/// must provide methods for serialization, deserialization, and type identification.
///
/// # Examples
///
/// ```
/// use std::any::Any;
/// use smithy_model::traits::{Trait, BoxTrait};
/// use smithy_model::shape_id::ShapeId;
/// use smithy_model::node::Node;
///
/// #[derive(Clone, Debug)]
/// struct MyTrait(String);
///
/// const TRAIT_ID: &'static ShapeId = &ShapeId::new_static("example", "myTrait");
///
/// impl Trait for MyTrait {
///     fn static_id() -> &'static ShapeId {
///         TRAIT_ID
///     }
///
///     fn id(&self) -> &ShapeId {
///         Self::static_id()
///     }
///
///     fn to_node(&self) -> Node {
///         Node::String(self.0.clone())
///     }
///
///     fn from_node(node: &Node) -> Option<Self> {
///         node.as_str().map(|s| Self(s.to_string()))
///     }
///
///     fn clone_trait(&self) -> BoxTrait {
///         Box::new(self.clone())
///     }
///
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
/// ```
pub trait Trait: Any + fmt::Debug {
    /// Returns the static ID of this trait type
    ///
    /// This method returns the ShapeId that uniquely identifies this trait type.
    /// It is used for trait registration and lookup.
    fn static_id() -> &'static ShapeId
    where
        Self: Sized;

    /// Returns the ID of this trait instance
    ///
    /// Implementations MUST return the same ID as `static_id()`.
    /// The only exception is unknown traits which are mapped to [DynamicTrait]
    /// which will override this method to return their instance-specific ID.
    fn id(&self) -> &ShapeId;

    /// Convert this trait to a Node for serialization
    ///
    /// This method is used when serializing the trait to a Smithy model.
    fn to_node(&self) -> Node;

    /// Create an instance of this trait from a Node
    ///
    /// This method is used when deserializing a trait from a Smithy model.
    /// It should return None if the Node cannot be converted to this trait type.
    fn from_node(node: &Node) -> Option<Self>
    where
        Self: Sized;

    /// Create a boxed trait from a Node
    ///
    /// This is a helper method that calls from_node and boxes the result.
    /// It is primarily used by the trait registry.
    fn from_node_boxed(node: &Node) -> Option<BoxTrait>
    where
        Self: Sized,
    {
        Self::from_node(node).map(|t| Box::new(t) as BoxTrait)
    }

    /// Clone this trait
    ///
    /// This method is required because `Box<dyn Trait>` cannot implement Clone directly.
    fn clone_trait(&self) -> BoxTrait;

    /// Convert to Any for downcasting
    ///
    /// This method is used for downcasting trait objects to concrete types.
    /// The default implementation should be sufficient for most traits.
    fn as_any(&self) -> &dyn Any;
}

/// A collection of traits applied to a shape
#[derive(Debug, Default)]
pub struct TraitMap(HashMap<ShapeId, BoxTrait>);

impl Clone for TraitMap {
    fn clone(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|(k, v)| (k.clone(), v.clone_trait()))
                .collect(),
        )
    }
}

/// Implement HashMap like accessors for traits, this type acts similar to a HashMap
impl TraitMap {
    /// Creates an empty TraitMap
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns true if the map contains a trait with the given ID
    pub fn contains_key(&self, id: &ShapeId) -> bool {
        self.0.contains_key(id)
    }

    /// Returns a reference to the trait with the given ID
    pub fn get(&self, id: &ShapeId) -> Option<&BoxTrait> {
        self.0.get(id)
    }

    /// Returns a mutable reference to the trait with the given ID
    pub fn get_mut(&mut self, id: &ShapeId) -> Option<&mut BoxTrait> {
        self.0.get_mut(id)
    }

    /// Inserts a trait into the map
    ///
    /// Returns the previous trait if one existed with the same ID
    pub fn insert(&mut self, trait_obj: BoxTrait) -> Option<BoxTrait> {
        self.0.insert(trait_obj.id().clone(), trait_obj)
    }

    /// Removes a trait from the map, returning it if it existed
    pub fn remove(&mut self, id: &ShapeId) -> Option<BoxTrait> {
        self.0.remove(id)
    }

    /// Returns true if the map contains no traits
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of traits in the map
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Removes all traits from the map
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns an iterator over the traits in the map
    pub fn iter(&self) -> impl Iterator<Item = (&ShapeId, &BoxTrait)> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the traits in the map
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ShapeId, &mut BoxTrait)> {
        self.0.iter_mut()
    }

    /// Returns an iterator over the trait IDs in the map
    pub fn keys(&self) -> impl Iterator<Item = &ShapeId> {
        self.0.keys()
    }

    /// Returns an iterator over the traits in the map
    pub fn values(&self) -> impl Iterator<Item = &BoxTrait> {
        self.0.values()
    }

    /// Returns a mutable iterator over the traits in the map
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut BoxTrait> {
        self.0.values_mut()
    }
}

impl PartialEq for TraitMap {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|(key, value)| {
            other
                .get(key)
                .is_some_and(|v| value.to_node() == v.to_node())
        })
    }
}

impl Eq for TraitMap {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestTrait(String);

    const TRAIT_ID: &ShapeId = &ShapeId::new_static("test", "testTrait");

    impl Trait for TestTrait {
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

    #[test]
    fn test_trait_map_basic_operations() {
        let mut map = TraitMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        let trait_id = TestTrait::static_id();
        let test_trait = Box::new(TestTrait("test".to_string())) as BoxTrait;

        // Test insert
        assert!(map.insert(test_trait).is_none());
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);

        // Test contains_key
        assert!(map.contains_key(trait_id));

        // Test get
        let retrieved = map.get(trait_id).unwrap();
        assert_eq!(retrieved.to_node(), Node::String("test".to_string()));

        // Test remove
        let removed = map.remove(trait_id).unwrap();
        assert_eq!(removed.to_node(), Node::String("test".to_string()));
        assert!(map.is_empty());

        // Test clear
        map.insert(Box::new(TestTrait("test2".to_string())));
        assert!(!map.is_empty());
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_trait_map_iterators() {
        let mut map = TraitMap::new();
        let trait1 = Box::new(TestTrait("test1".to_string())) as BoxTrait;
        let trait2 = Box::new(TestTrait("test2".to_string())) as BoxTrait;

        map.insert(trait1);
        map.insert(trait2);

        // Test iter
        let mut iter_count = 0;
        for (id, trait_obj) in map.iter() {
            assert_eq!(id, TestTrait::static_id());
            assert!(matches!(trait_obj.to_node(), Node::String(_)));
            iter_count += 1;
        }
        assert_eq!(iter_count, 1); // Only one trait since they have the same ID

        // Test keys
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], TestTrait::static_id());

        // Test values
        let values: Vec<_> = map.values().collect();
        assert_eq!(values.len(), 1);
        assert!(matches!(values[0].to_node(), Node::String(_)));
    }

    #[test]
    fn test_trait_map_clone() {
        let mut map = TraitMap::new();
        map.insert(Box::new(TestTrait("test".to_string())));

        let cloned = map.clone();
        assert_eq!(cloned.len(), map.len());
        assert_eq!(
            cloned.get(TestTrait::static_id()).unwrap().to_node(),
            map.get(TestTrait::static_id()).unwrap().to_node()
        );
    }
}
