/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! ShapeId implementation for the Smithy model.
//!
//! This module provides the ShapeId type, which uniquely identifies shapes in a Smithy model.

use crate::error::{Error, Result};
use std::fmt;
use std::str::FromStr;

/// A unique identifier for a shape in a Smithy model.
///
/// A shape ID consists of a namespace, a name, and an optional member name.
/// For example, `com.example.foo#Bar` or `com.example.foo#Bar$baz`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShapeId {
    /// The namespace of the shape.
    namespace: String,
    /// The name of the shape.
    name: String,
    /// The optional member name.
    member: Option<String>,
}

impl ShapeId {
    /// Creates a new ShapeId.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace of the shape.
    /// * `name` - The name of the shape.
    /// * `member` - The optional member name.
    ///
    /// # Returns
    ///
    /// A new ShapeId.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        member: Option<impl Into<String>>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            member: member.map(Into::into),
        }
    }

    /// Returns the namespace of the shape.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the name of the shape.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the member name of the shape, if any.
    pub fn member(&self) -> Option<&str> {
        self.member.as_deref()
    }

    /// Returns true if this shape ID has a member name.
    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }

    /// Returns the absolute shape ID without the member name.
    pub fn shape_id(&self) -> ShapeId {
        ShapeId::new(&self.namespace, &self.name, None::<String>)
    }

    /// Returns a new shape ID with the given member name.
    pub fn with_member(&self, member: impl Into<String>) -> ShapeId {
        ShapeId::new(&self.namespace, &self.name, Some(member.into()))
    }
}

impl fmt::Display for ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.namespace, self.name)?;
        if let Some(member) = &self.member {
            write!(f, "${}", member)?;
        }
        Ok(())
    }
}

impl FromStr for ShapeId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // This is a placeholder implementation.
        // The actual parsing will be implemented in a future task.
        Err(Error::InvalidShapeId(format!(
            "ShapeId parsing not yet implemented: {}",
            s
        )))
    }
}
