/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Trait implementations for the Smithy model.
//!
//! This module provides the Trait type and its variants, which represent the
//! different kinds of traits in a Smithy model.

use crate::shape_id::ShapeId;
use crate::Node;

/// A trait in a Smithy model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trait {
    /// The ID of the trait.
    pub id: ShapeId,
    /// The value of the trait.
    pub value: Option<Node>,
}

impl Trait {
    /// Creates a new trait with no value.
    pub fn new(id: impl Into<ShapeId>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }

    /// Creates a new trait with a value.
    pub fn new_with_value(id: impl Into<ShapeId>, value: Node) -> Self {
        Self {
            id: id.into(),
            value: Some(value),
        }
    }

    /// Returns the ID of the trait.
    pub fn id(&self) -> &ShapeId {
        &self.id
    }

    /// Returns the value of the trait, if any.
    pub fn value(&self) -> Option<&Node> {
        self.value.as_ref()
    }
}
