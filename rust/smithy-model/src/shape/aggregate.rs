/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Aggregate shape types for the Smithy model.

use std::collections::HashMap;

use crate::shape::{MemberShape, ProvideShapeMetadata, Shape, ShapeMetadata};
use crate::shape_id::ShapeId;
use crate::traits::Trait;

/// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
#[derive(Debug, Clone)]
pub struct ListShape {
    pub(crate) metadata: ShapeMetadata,
    /// The member shape that defines the type of elements in the list
    pub member: MemberShape,
}

/// A [map](https://smithy.io/2.0/spec/aggregate-types.html#map) shape
#[derive(Debug, Clone)]
pub struct MapShape {
    pub(crate) metadata: ShapeMetadata,
    /// The key member shape
    pub key: MemberShape,
    /// The value member shape
    pub value: MemberShape,
}

/// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
#[derive(Debug, Clone)]
pub struct SetShape {
    pub(crate) metadata: ShapeMetadata,
    /// The member shape that defines the type of elements in the set
    pub member: MemberShape,
}

/// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
#[derive(Debug, Clone)]
pub struct StructureShape {
    pub(crate) metadata: ShapeMetadata,
    /// The members of the structure, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

/// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
#[derive(Debug, Clone)]
pub struct UnionShape {
    pub(crate) metadata: ShapeMetadata,
    /// The members of the union, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

// Implement ProvideShapeMetadata for all aggregate shapes
impl ProvideShapeMetadata for ListShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for MapShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for SetShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for StructureShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for UnionShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

// Implement constructors for aggregate shapes
impl ListShape {
    /// Create a new list shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>, member: MemberShape) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            member,
        }
    }

    /// Get the member shape
    pub fn member(&self) -> &MemberShape {
        &self.member
    }
}

impl MapShape {
    /// Create a new map shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        key: MemberShape,
        value: MemberShape,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            key,
            value,
        }
    }

    /// Get the key member shape
    pub fn key(&self) -> &MemberShape {
        &self.key
    }

    /// Get the value member shape
    pub fn value(&self) -> &MemberShape {
        &self.value
    }
}

impl SetShape {
    /// Create a new set shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>, member: MemberShape) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            member,
        }
    }

    /// Get the member shape
    pub fn member(&self) -> &MemberShape {
        &self.member
    }
}

impl StructureShape {
    /// Create a new structure shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        members: HashMap<String, MemberShape>,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            members,
        }
    }

    /// Get all members of the structure
    pub fn members(&self) -> &HashMap<String, MemberShape> {
        &self.members
    }

    /// Get a specific member by name
    pub fn get_member(&self, name: &str) -> Option<&MemberShape> {
        self.members.get(name)
    }

    /// Check if the structure has a member with the given name
    pub fn has_member(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }
}

impl UnionShape {
    /// Create a new union shape
    pub fn new(
        id: ShapeId,
        traits: HashMap<ShapeId, Trait>,
        members: HashMap<String, MemberShape>,
    ) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
            members,
        }
    }

    /// Get all members of the union
    pub fn members(&self) -> &HashMap<String, MemberShape> {
        &self.members
    }

    /// Get a specific member by name
    pub fn get_member(&self, name: &str) -> Option<&MemberShape> {
        self.members.get(name)
    }

    /// Check if the union has a member with the given name
    pub fn has_member(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }
}

// Implement From traits for converting shape structs to Shape enum
impl From<ListShape> for Shape {
    fn from(shape: ListShape) -> Self {
        Shape::List(shape)
    }
}

impl From<MapShape> for Shape {
    fn from(shape: MapShape) -> Self {
        Shape::Map(shape)
    }
}

impl From<SetShape> for Shape {
    fn from(shape: SetShape) -> Self {
        Shape::Set(shape)
    }
}

impl From<StructureShape> for Shape {
    fn from(shape: StructureShape) -> Self {
        Shape::Structure(shape)
    }
}

impl From<UnionShape> for Shape {
    fn from(shape: UnionShape) -> Self {
        Shape::Union(shape)
    }
}
