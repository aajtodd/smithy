/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Type refinement traits
//!
//! See [type refinement traits](https://smithy.io/2.0/spec/type-refinement-traits.html#)
//! in the Smithy specification.

mod enum_value;
mod mixin;

pub use enum_value::EnumValue;
pub use mixin::Mixin;

use crate::define_simple_trait;

define_simple_trait! {
    /// The required trait
    ///
    /// This trait indicates that a member is required in its container.
    /// It is a marker trait with no value.
    ///
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Required;

    trait_id: "smithy.api", "required";
    type: annotation;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Trait;
    use crate::Node;

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
