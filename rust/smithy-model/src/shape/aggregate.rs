/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Aggregate shape types for the Smithy model.

use crate::shape::{
    builder::{self, parse_shape_id, ProvideTraitsMut},
    error::BuildError,
    MemberShape, ProvideShapeMetadata, Shape, ShapeMetadata,
};
use crate::shape_id::ShapeId;
use crate::traits::Trait;
use std::collections::HashMap;

/// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
#[derive(Debug, Clone)]
pub struct ListShape {
    pub(crate) metadata: ShapeMetadata,
    /// The member shape that defines the type of elements in the list
    pub member: MemberShape,
}

/// Builder for creating a list shape.
#[derive(Debug, Default)]
pub struct ListShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    member: Option<MemberShape>,
}

impl ListShapeBuilder {
    /// Create a new list shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the list shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the member shape for the list.
    pub fn member(mut self, member: MemberShape) -> Self {
        self.member = Some(member);
        self
    }

    /// Build the list shape.
    pub fn build(self) -> Result<ListShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        let member = self
            .member
            .ok_or_else(|| required_field_error(field_names::MEMBER))?;

        Ok(ListShape {
            metadata: ShapeMetadata::new(id, self.traits),
            member,
        })
    }
}

impl ProvideTraitsMut for ListShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl ListShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> ListShapeBuilder {
        ListShapeBuilder::new()
    }

    /// Returns the member shape for this list shape.
    pub fn member(&self) -> &MemberShape {
        &self.member
    }
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

/// Builder for creating a map shape.
#[derive(Debug, Default)]
pub struct MapShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    key: Option<MemberShape>,
    value: Option<MemberShape>,
}

impl MapShapeBuilder {
    /// Create a new map shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the map shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the key member shape for the map.
    pub fn key(mut self, key: MemberShape) -> Self {
        self.key = Some(key);
        self
    }

    /// Set the value member shape for the map.
    pub fn value(mut self, value: MemberShape) -> Self {
        self.value = Some(value);
        self
    }

    /// Build the map shape.
    pub fn build(self) -> Result<MapShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        let key = self
            .key
            .ok_or_else(|| required_field_error(field_names::KEY))?;
        let value = self
            .value
            .ok_or_else(|| required_field_error(field_names::VALUE))?;

        Ok(MapShape {
            metadata: ShapeMetadata::new(id, self.traits),
            key,
            value,
        })
    }
}

impl ProvideTraitsMut for MapShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl MapShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> MapShapeBuilder {
        MapShapeBuilder::new()
    }

    /// Returns the value member shape for this map
    pub fn value(&self) -> &MemberShape {
        &self.value
    }

    /// Returns the key member shape for this map
    pub fn key(&self) -> &MemberShape {
        &self.key
    }
}

/// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
#[derive(Debug, Clone)]
pub struct SetShape {
    pub(crate) metadata: ShapeMetadata,
    /// The member shape that defines the type of elements in the set
    pub member: MemberShape,
}

/// Builder for creating a set shape.
#[derive(Debug, Default)]
pub struct SetShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    member: Option<MemberShape>,
}

impl SetShapeBuilder {
    /// Create a new set shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the set shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the member shape for the set.
    pub fn member(mut self, member: MemberShape) -> Self {
        self.member = Some(member);
        self
    }

    /// Build the set shape.
    pub fn build(self) -> Result<SetShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        let member = self
            .member
            .ok_or_else(|| required_field_error(field_names::MEMBER))?;

        Ok(SetShape {
            metadata: ShapeMetadata::new(id, self.traits),
            member,
        })
    }
}

impl ProvideTraitsMut for SetShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl SetShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> SetShapeBuilder {
        SetShapeBuilder::new()
    }

    /// Returns the member shape for this set shape.
    pub fn member(&self) -> &MemberShape {
        &self.member
    }
}

/// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
#[derive(Debug, Clone)]
pub struct StructureShape {
    pub(crate) metadata: ShapeMetadata,
    /// The members of the structure, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

/// Builder for creating a structure shape.
#[derive(Debug, Default)]
pub struct StructureShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    members: HashMap<String, MemberShape>,
}

impl StructureShapeBuilder {
    /// Create a new structure shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the structure shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a member to the structure shape.
    pub fn member(mut self, name: impl Into<String>, member: MemberShape) -> Self {
        self.members.insert(name.into(), member);
        self
    }

    /// Build the structure shape.
    pub fn build(self) -> Result<StructureShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        Ok(StructureShape {
            metadata: ShapeMetadata::new(id, self.traits),
            members: self.members,
        })
    }
}

impl ProvideTraitsMut for StructureShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl StructureShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> StructureShapeBuilder {
        StructureShapeBuilder::new()
    }

    /// Returns all member shapes for this structure
    pub fn members(&self) -> &HashMap<String, MemberShape> {
        &self.members
    }
}

/// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
#[derive(Debug, Clone)]
pub struct UnionShape {
    pub(crate) metadata: ShapeMetadata,
    /// The members of the union, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

/// Builder for creating a union shape.
#[derive(Debug, Default)]
pub struct UnionShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    members: HashMap<String, MemberShape>,
}

impl UnionShapeBuilder {
    /// Create a new union shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the union shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a member to the union shape.
    pub fn member(mut self, name: impl Into<String>, member: MemberShape) -> Self {
        self.members.insert(name.into(), member);
        self
    }

    /// Build the union shape.
    pub fn build(self) -> Result<UnionShape, BuildError> {
        use builder::{field_names, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        Ok(UnionShape {
            metadata: ShapeMetadata::new(id, self.traits),
            members: self.members,
        })
    }
}

impl ProvideTraitsMut for UnionShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl UnionShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> UnionShapeBuilder {
        UnionShapeBuilder::new()
    }

    /// Returns all member shapes for this union
    pub fn members(&self) -> &HashMap<String, MemberShape> {
        &self.members
    }
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

// Implement From for all aggregate shapes
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
