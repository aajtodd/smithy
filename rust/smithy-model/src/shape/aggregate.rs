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
use crate::traits::TraitMap;
use std::collections::HashMap;

/// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
#[derive(Debug, Clone, PartialEq)]
pub struct ListShape {
    pub(crate) metadata: ShapeMetadata,
    /// The member shape that defines the type of elements in the list
    pub member: MemberShape,
}

/// Builder for creating a list shape.
#[derive(Debug, Default)]
pub struct ListShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
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
    fn traits_mut(&mut self) -> &mut TraitMap {
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

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> ListShapeBuilder {
        ListShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.effective_traits,
            member: Some(self.member),
        }
    }
}

/// A [map](https://smithy.io/2.0/spec/aggregate-types.html#map) shape
#[derive(Debug, Clone, PartialEq)]
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
    traits: TraitMap,
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
    fn traits_mut(&mut self) -> &mut TraitMap {
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

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> MapShapeBuilder {
        MapShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.effective_traits,
            key: Some(self.key),
            value: Some(self.value),
        }
    }
}

/// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
#[derive(Debug, Clone, PartialEq)]
pub struct SetShape {
    pub(crate) metadata: ShapeMetadata,
    /// The member shape that defines the type of elements in the set
    pub member: MemberShape,
}

/// Builder for creating a set shape.
#[derive(Debug, Default)]
pub struct SetShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
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
    fn traits_mut(&mut self) -> &mut TraitMap {
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

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> SetShapeBuilder {
        SetShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.effective_traits,
            member: Some(self.member),
        }
    }
}

/// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
#[derive(Debug, Clone, PartialEq)]
pub struct StructureShape {
    pub(crate) metadata: ShapeMetadata,
    /// The members of the structure, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

/// Builder for creating a structure shape.
#[derive(Debug, Default)]
pub struct StructureShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
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
    pub fn member(mut self, member: MemberShape) -> Self {
        self.members.insert(member.member_name.clone(), member);
        self
    }

    /// Remove a member by name from the structure shape (if it exists)
    pub fn remove_member(mut self, member_name: impl AsRef<str>) -> Self {
        self.members.remove(member_name.as_ref());
        self
    }

    /// Remove all members from the structure shape
    pub fn clear_members(mut self) -> Self {
        self.members.clear();
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
    fn traits_mut(&mut self) -> &mut TraitMap {
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

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> StructureShapeBuilder {
        StructureShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.effective_traits,
            members: self.members,
        }
    }
}

/// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
#[derive(Debug, Clone, PartialEq)]
pub struct UnionShape {
    pub(crate) metadata: ShapeMetadata,
    /// The members of the union, keyed by member name
    pub members: HashMap<String, MemberShape>,
}

/// Builder for creating a union shape.
#[derive(Debug, Default)]
pub struct UnionShapeBuilder {
    id: Option<String>,
    traits: TraitMap,
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
    pub fn member(mut self, member: MemberShape) -> Self {
        self.members.insert(member.member_name.clone(), member);
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
    fn traits_mut(&mut self) -> &mut TraitMap {
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

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> UnionShapeBuilder {
        UnionShapeBuilder {
            id: Some(self.metadata.id.to_string()),
            traits: self.metadata.effective_traits,
            members: self.members,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::HasShapeId;
    use crate::ShapeId;
    // List shape tests

    #[test]
    fn test_list_shape_construction() {
        let target = ShapeId::new("smithy.api", "String").unwrap();
        let member = MemberShape::builder()
            .id("example.foo#MyList$member")
            .member_name("member")
            .target(target)
            .build()
            .unwrap();

        let shape = ListShape::builder()
            .id("example.foo#MyList")
            .member(member.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyList");
        assert_eq!(shape.member.id().to_string(), "example.foo#MyList$member");
        assert_eq!(shape.member.target.to_string(), "smithy.api#String");
    }

    #[test]
    fn test_list_shape_missing_member() {
        let result = ListShape::builder().id("example.foo#MyList").build();

        assert!(result.is_err());
        match result {
            Err(BuildError::MissingRequiredField { field }) => {
                assert_eq!(field, "member");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }

    // Map shape tests

    #[test]
    fn test_map_shape_construction() {
        let key_target = ShapeId::new("smithy.api", "String").unwrap();
        let value_target = ShapeId::new("smithy.api", "Integer").unwrap();

        let key = MemberShape::builder()
            .id("example.foo#MyMap$key")
            .member_name("key")
            .target(key_target)
            .build()
            .unwrap();

        let value = MemberShape::builder()
            .id("example.foo#MyMap$value")
            .member_name("value")
            .target(value_target)
            .build()
            .unwrap();

        let shape = MapShape::builder()
            .id("example.foo#MyMap")
            .key(key.clone())
            .value(value.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyMap");
        assert_eq!(shape.key.id().to_string(), "example.foo#MyMap$key");
        assert_eq!(shape.value.id().to_string(), "example.foo#MyMap$value");
        assert_eq!(shape.key.target.to_string(), "smithy.api#String");
        assert_eq!(shape.value.target.to_string(), "smithy.api#Integer");
    }

    // Set shape tests

    #[test]
    fn test_set_shape_construction() {
        let target = ShapeId::new("smithy.api", "String").unwrap();
        let member = MemberShape::builder()
            .id("example.foo#MySet$member")
            .member_name("member")
            .target(target)
            .build()
            .unwrap();

        let shape = SetShape::builder()
            .id("example.foo#MySet")
            .member(member.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MySet");
        assert_eq!(shape.member.id().to_string(), "example.foo#MySet$member");
        assert_eq!(shape.member.target.to_string(), "smithy.api#String");
    }

    // Structure shape tests

    #[test]
    fn test_structure_shape_construction() {
        let string_target = ShapeId::new("smithy.api", "String").unwrap();
        let int_target = ShapeId::new("smithy.api", "Integer").unwrap();

        let name_member = MemberShape::builder()
            .id("example.foo#MyStruct$name")
            .member_name("name")
            .target(string_target)
            .build()
            .unwrap();

        let age_member = MemberShape::builder()
            .id("example.foo#MyStruct$age")
            .member_name("age")
            .target(int_target)
            .build()
            .unwrap();

        let shape = StructureShape::builder()
            .id("example.foo#MyStruct")
            .member(name_member.clone())
            .member(age_member.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyStruct");
        assert_eq!(shape.members().len(), 2);
        let members = &shape.members;
        assert!(members.get("name").is_some());
        assert!(members.get("age").is_some());
        assert_eq!(
            members.get("name").unwrap().target.to_string(),
            "smithy.api#String"
        );
        assert_eq!(
            members.get("age").unwrap().target.to_string(),
            "smithy.api#Integer"
        );
    }

    #[test]
    fn test_structure_shape_member_management() {
        let string_target = ShapeId::new("smithy.api", "String").unwrap();
        let int_target = ShapeId::new("smithy.api", "Integer").unwrap();

        let name_member = MemberShape::builder()
            .id("example.foo#MyStruct$name")
            .member_name("name")
            .target(string_target)
            .build()
            .unwrap();

        let age_member = MemberShape::builder()
            .id("example.foo#MyStruct$age")
            .member_name("age")
            .target(int_target)
            .build()
            .unwrap();

        // Add members
        let shape = StructureShape::builder()
            .id("example.foo#MyStruct")
            .member(name_member.clone())
            .member(age_member.clone())
            .build()
            .unwrap();

        assert_eq!(shape.members().len(), 2);

        // Remove a member
        let shape = shape.to_builder().remove_member("age").build().unwrap();

        assert_eq!(shape.members().len(), 1);
        assert!(shape.members.contains_key("name"));
        assert!(!shape.members.contains_key("age"));

        // Clear members
        let shape = shape.to_builder().clear_members().build().unwrap();

        assert_eq!(shape.members().len(), 0);
    }

    // Union shape tests

    #[test]
    fn test_union_shape_construction() {
        let string_target = ShapeId::new("smithy.api", "String").unwrap();
        let int_target = ShapeId::new("smithy.api", "Integer").unwrap();

        let string_member = MemberShape::builder()
            .id("example.foo#MyUnion$stringValue")
            .member_name("stringValue")
            .target(string_target)
            .build()
            .unwrap();

        let int_member = MemberShape::builder()
            .id("example.foo#MyUnion$intValue")
            .member_name("intValue")
            .target(int_target)
            .build()
            .unwrap();

        let shape = UnionShape::builder()
            .id("example.foo#MyUnion")
            .member(string_member.clone())
            .member(int_member.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyUnion");
        assert_eq!(shape.members().len(), 2);
        assert!(shape.members.contains_key("stringValue"));
        assert!(shape.members.contains_key("intValue"));
    }
}
