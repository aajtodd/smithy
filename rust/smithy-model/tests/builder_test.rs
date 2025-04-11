/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use smithy_model::shape::MemberShape;
use smithy_model::shape::ShapeBuilderExt;
use smithy_model::shape::{BlobShape, StringShape};
use smithy_model::shape::{HasShapeId, HasTraits};
use smithy_model::shape::{ListShape, MapShape, StructureShape};
use smithy_model::shape_id::ShapeId;
use smithy_model::traits::DynamicTrait;

#[test]
fn test_string_shape_builder() {
    let shape = StringShape::builder()
        .id("example.foo#MyString")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyString");
    assert_eq!(shape.traits().len(), 0);
}

#[test]
fn test_string_shape_builder_with_traits() {
    let shape = StringShape::builder()
        .id("example.foo#MyString")
        .documentation("A string shape")
        .required()
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyString");
    assert_eq!(shape.traits().len(), 2);
    assert!(shape.has_trait(ShapeId::new("smithy.api", "documentation").unwrap()));
    assert!(shape.has_trait(ShapeId::new("smithy.api", "required").unwrap()));
}

#[test]
fn test_blob_shape_builder() {
    let shape = BlobShape::builder()
        .id("example.foo#MyBlob")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyBlob");
    assert_eq!(shape.traits().len(), 0);
}

#[test]
fn test_structure_shape_builder() {
    let shape = StructureShape::builder()
        .id("example.foo#MyStruct")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyStruct");
    assert_eq!(shape.traits().len(), 0);
    assert_eq!(shape.members.len(), 0);
}

#[test]
fn test_structure_shape_builder_with_members() {
    let member1 = MemberShape::builder()
        .id("example.foo#MyStruct$name")
        .member_name("name")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build()
        .unwrap();

    let member2 = MemberShape::builder()
        .id("example.foo#MyStruct$age")
        .member_name("age")
        .target(ShapeId::new("smithy.api", "Integer").unwrap())
        .build()
        .unwrap();

    let shape = StructureShape::builder()
        .id("example.foo#MyStruct")
        .member(member1)
        .member(member2)
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyStruct");
    assert_eq!(shape.members.len(), 2);
    assert!(shape.members.contains_key("name"));
    assert!(shape.members.contains_key("age"));
}

#[test]
fn test_structure_shape_builder_with_traits() {
    let shape = StructureShape::builder()
        .id("example.foo#MyStruct")
        .documentation("A structure shape")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyStruct");
    assert_eq!(shape.traits().len(), 1);
    assert!(shape.has_trait(ShapeId::new("smithy.api", "documentation").unwrap()));
}

#[test]
fn test_list_shape_builder() {
    let member = MemberShape::builder()
        .id("example.foo#MyList$member")
        .member_name("member")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build()
        .unwrap();

    let shape = ListShape::builder()
        .id("example.foo#MyList")
        .member(member)
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyList");
    assert_eq!(shape.member.target.to_string(), "smithy.api#String");
}

#[test]
fn test_list_shape_builder_missing_member() {
    let result = ListShape::builder().id("example.foo#MyList").build();

    assert!(result.is_err());
}

#[test]
fn test_map_shape_builder() {
    let key = MemberShape::builder()
        .id("example.foo#MyMap$key")
        .member_name("key")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build()
        .unwrap();

    let value = MemberShape::builder()
        .id("example.foo#MyMap$value")
        .member_name("value")
        .target(ShapeId::new("smithy.api", "Integer").unwrap())
        .build()
        .unwrap();

    let shape = MapShape::builder()
        .id("example.foo#MyMap")
        .key(key)
        .value(value)
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyMap");
    assert_eq!(shape.key.target.to_string(), "smithy.api#String");
    assert_eq!(shape.value.target.to_string(), "smithy.api#Integer");
}

#[test]
fn test_map_shape_builder_missing_value() {
    let value = MemberShape::builder()
        .id("example.foo#MyMap$value")
        .member_name("value")
        .target(ShapeId::new("smithy.api", "Integer").unwrap())
        .build()
        .unwrap();

    let result = MapShape::builder()
        .id("example.foo#MyMap")
        .value(value)
        .build();

    assert!(result.is_err());
}

#[test]
fn test_map_shape_builder_missing_key() {
    let key = MemberShape::builder()
        .id("example.foo#MyMap$key")
        .member_name("key")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build()
        .unwrap();

    let result = MapShape::builder().id("example.foo#MyMap").key(key).build();

    assert!(result.is_err());
}

#[test]
fn test_missing_id() {
    let result = StringShape::builder().build();
    assert!(result.is_err());
}

#[test]
fn test_invalid_id() {
    let result = StringShape::builder().id("invalid").build();
    assert!(result.is_err());
}

#[test]
fn test_with_custom_trait() {
    let trait_id = ShapeId::new("example.foo", "customTrait").unwrap();
    let custom_trait = DynamicTrait::new(trait_id.clone(), None);

    let shape = StringShape::builder()
        .id("example.foo#MyString")
        .with_trait(custom_trait)
        .build()
        .unwrap();

    assert!(shape.has_trait(trait_id));
}

#[test]
fn test_member_shape_builder() {
    let shape = MemberShape::builder()
        .id("example.foo#MyStruct$name")
        .member_name("name")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyStruct$name");
    assert_eq!(shape.traits().len(), 0);
    assert_eq!(shape.member_name, "name");
    assert_eq!(shape.target.to_string(), "smithy.api#String");
}

#[test]
fn test_member_shape_builder_with_traits() {
    let shape = MemberShape::builder()
        .id("example.foo#MyStruct$name")
        .member_name("name")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .documentation("A member shape")
        .required()
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyStruct$name");
    assert_eq!(shape.traits().len(), 2);
    assert!(shape.has_trait(ShapeId::new("smithy.api", "documentation").unwrap()));
    assert!(shape.has_trait(ShapeId::new("smithy.api", "required").unwrap()));
}

#[test]
fn test_member_shape_builder_missing_id() {
    let result = MemberShape::builder()
        .member_name("name")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build();

    assert!(result.is_err());
}

#[test]
fn test_member_shape_builder_missing_member_name() {
    let result = MemberShape::builder()
        .id("example.foo#MyStruct$name")
        .target(ShapeId::new("smithy.api", "String").unwrap())
        .build();

    assert!(result.is_err());
}

#[test]
fn test_member_shape_builder_missing_target() {
    let result = MemberShape::builder()
        .id("example.foo#MyStruct$name")
        .member_name("name")
        .build();

    assert!(result.is_err());
}
