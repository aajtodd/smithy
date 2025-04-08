/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Trait implementations for the Smithy model.
//!
//! This module provides the Trait type and its variants, which represent the
//! different kinds of traits in a Smithy model.

use crate::shape_id::ShapeId;
use std::collections::HashMap;

/// A trait in a Smithy model.
#[derive(Debug, Clone)]
pub struct Trait {
    /// The ID of the trait.
    pub id: ShapeId,
    /// The value of the trait.
    pub value: Option<TraitValue>,
}

/// A value for a trait in a Smithy model.
#[derive(Debug, Clone)]
pub enum TraitValue {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(f64),
    /// A boolean value.
    Boolean(bool),
    /// An array of values.
    Array(Vec<TraitValue>),
    /// An object with string keys and values.
    Object(HashMap<String, TraitValue>),
    /// A null value.
    Null,
}

impl Trait {
    /// Creates a new trait with no value.
    pub fn new(id: ShapeId) -> Self {
        Self { id, value: None }
    }

    /// Creates a new trait with a value.
    pub fn with_value(id: ShapeId, value: TraitValue) -> Self {
        Self {
            id,
            value: Some(value),
        }
    }

    /// Returns the ID of the trait.
    pub fn id(&self) -> &ShapeId {
        &self.id
    }

    /// Returns the value of the trait, if any.
    pub fn value(&self) -> Option<&TraitValue> {
        self.value.as_ref()
    }
}
