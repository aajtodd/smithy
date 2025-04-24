/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Trait registry for managing trait creation.

use std::collections::HashMap;

use crate::node::Node;
use crate::shape::ShapeId;
use crate::traits::documentation::{Deprecated, Documentation};
use crate::traits::type_refinement::Required;
use crate::traits::{BoxTrait, DynamicTrait, Trait};

/// Registry for trait creation during deserialization
///
/// The trait registry manages the creation of traits from IDs and Node values.
/// It maintains a mapping of trait IDs to creator functions that can instantiate
/// the appropriate trait type.
///
/// # Examples
///
/// ```
/// use smithy_model::traits::{TraitRegistry, documentation::Documentation, Trait};
/// use smithy_model::shape::ShapeId;
/// use smithy_model::node::Node;
///
/// let registry = TraitRegistry::standard();
/// let id = Documentation::static_id();
/// let node = Node::String("This is documentation".to_string());
///
/// let trait_ = registry.create_trait(&id, &node);
/// let doc = trait_.as_any().downcast_ref::<Documentation>().unwrap();
/// assert_eq!(doc.0, "This is documentation");
/// ```
pub struct TraitRegistry {
    creators: HashMap<String, fn(&Node) -> Option<BoxTrait>>,
}

impl TraitRegistry {
    /// Create a new trait registry with built-in traits registered
    pub fn standard() -> Self {
        let mut registry = Self {
            creators: HashMap::new(),
        };

        // FIXME - ensure we register all prelude traits
        // Register built-in traits
        registry.register::<Documentation>();
        registry.register::<Required>();
        registry.register::<Deprecated>();

        registry
    }

    /// Create a new empty trait registry
    pub fn empty() -> Self {
        Self {
            creators: HashMap::new(),
        }
    }

    /// Register a trait type with this registry
    ///
    /// This method registers a trait type with the registry, allowing it to be
    /// created from an ID and Node during deserialization.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The trait type to register, which must implement the Trait trait
    pub fn register<T: Trait + 'static>(&mut self) {
        let id = T::static_id().to_string();
        self.creators.insert(id, T::from_node_boxed);
    }

    /// Check if a trait type is registered with this registry
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the trait to check
    ///
    /// # Returns
    ///
    /// `true` if the trait is registered, `false` otherwise
    pub fn is_registered(&self, id: impl AsRef<ShapeId>) -> bool {
        self.creators.contains_key(&id.as_ref().to_string())
    }

    /// Create a trait from an ID and Node
    ///
    /// This method attempts to create a trait from the given ID and Node.
    /// If the trait type is registered, it will use the registered creator function.
    /// Otherwise, it will fall back to creating a DynamicTrait.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the trait to create
    /// * `node` - The Node containing the trait value
    ///
    /// # Returns
    ///
    /// A boxed trait object representing the trait
    pub fn create_trait(&self, id: impl AsRef<ShapeId>, node: &Node) -> BoxTrait {
        let id_ref = id.as_ref();

        // Try to create a registered trait
        if let Some(creator) = self.creators.get(&id_ref.to_string()) {
            if let Some(trait_) = creator(node) {
                return trait_;
            }
        }

        // Fall back to DynamicTrait
        Box::new(DynamicTrait::new(id_ref.clone(), Some(node.clone())))
    }
}

impl Default for TraitRegistry {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn test_registry_with_known_trait() {
        let registry = TraitRegistry::standard();
        let id = Documentation::static_id();
        let node = Node::String("This is documentation".to_string());

        let trait_ = registry.create_trait(id, &node);
        let doc = trait_.as_any().downcast_ref::<Documentation>().unwrap();
        assert_eq!(doc.0, "This is documentation");
    }

    #[test]
    fn test_registry_with_unknown_trait() {
        let registry = TraitRegistry::standard();
        let id = ShapeId::new_unchecked("example#unknownTrait");
        let node = Node::String("unknown value".to_string());

        let trait_ = registry.create_trait(&id, &node);
        let dynamic = trait_.as_any().downcast_ref::<DynamicTrait>().unwrap();
        assert_eq!(*dynamic.id(), id);
        assert_eq!(dynamic.value(), Some(&node));
    }

    #[test]
    fn test_registry_is_registered() {
        let registry = TraitRegistry::standard();

        assert!(registry.is_registered(Documentation::static_id()));
        assert!(registry.is_registered(Required::static_id()));
        assert!(registry.is_registered(Deprecated::static_id()));

        assert!(!registry.is_registered(ShapeId::new_unchecked("example#unknownTrait")));
    }

    #[test]
    fn test_custom_trait_registration() {
        #[derive(Clone, Debug, PartialEq)]
        struct CustomTrait(String);

        const TRAIT_ID: &ShapeId = &ShapeId::new_static("example", "customTrait");

        impl Trait for CustomTrait {
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

        let mut registry = TraitRegistry::standard();
        registry.register::<CustomTrait>();

        assert!(registry.is_registered(CustomTrait::static_id()));

        let id = CustomTrait::static_id();
        let node = Node::String("custom value".to_string());

        let trait_ = registry.create_trait(id, &node);
        let custom = trait_.as_any().downcast_ref::<CustomTrait>().unwrap();
        assert_eq!(custom.0, "custom value");
    }
}
