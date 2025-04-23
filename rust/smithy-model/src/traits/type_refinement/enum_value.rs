/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::any::Any;
use std::fmt;

use crate::node::Node;
use crate::shape_id::ShapeId;
use crate::traits::{BoxTrait, Trait};

/// The [enumValue](https://smithy.io/2.0/spec/simple-types.html#enum-enumvalue-trait) trait
///
/// This trait is used to define the string value of an enum member.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumValue(pub String);

impl EnumValue {
    /// Create a new enum value trait
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Create a new enum value trait with an integer value
    pub fn new_int(value: i64) -> Self {
        Self(value.to_string())
    }

    /// Try to get this enumValue as an integer
    pub fn expect_int_value(&self) -> i64 {
        self.0.parse().expect("expected integer enumValue")
    }
}

impl Trait for EnumValue {
    fn static_id() -> &'static ShapeId {
        const TRAIT_ID: &ShapeId = &ShapeId::new_static("smithy.api", "enumValue");
        TRAIT_ID
    }

    fn id(&self) -> &ShapeId {
        Self::static_id()
    }

    fn to_node(&self) -> Node {
        Node::from(self.0.clone())
    }

    fn from_node(node: &Node) -> Option<Self>
    where
        Self: Sized,
    {
        match node {
            Node::String(s) => Some(Self(s.clone())),
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

impl fmt::Display for EnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enumValue: {}", self.0)
    }
}
