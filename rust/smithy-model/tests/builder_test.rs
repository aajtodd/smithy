/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use smithy_model::shape::{
    BlobShape, HasShapeId, HasTraits, ListShape, MapShape, MemberShape, ShapeBuilderExt,
    StringShape, StructureShape,
};
use smithy_model::traits::Trait;
use smithy_model::ShapeId;
use std::collections::HashMap;

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
        .documentation("A test string")
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
fn test_missing_id() {
    let result = StringShape::builder().build();
    assert!(result.is_err());
}

#[test]
fn test_invalid_id() {
    let result = StringShape::builder().id("invalid id").build();
    assert!(result.is_err());
}

#[test]
fn test_with_custom_trait() {
    let trait_id = ShapeId::new("example.foo", "customTrait").unwrap();
    let custom_trait = Trait::new(trait_id.clone());

    let shape = StringShape::builder()
        .id("example.foo#MyString")
        .with_trait(custom_trait)
        .build()
        .unwrap();

    assert_eq!(shape.traits().len(), 1);
    assert!(shape.has_trait(trait_id));
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
    let string_id = ShapeId::new("smithy.api", "String").unwrap();
    let member1 = MemberShape::new(
        "example.foo#MyStruct$name".parse().unwrap(),
        HashMap::new(),
        "name".to_string(),
        string_id.clone(),
    );

    let member2 = MemberShape::new(
        "example.foo#MyStruct$age".parse().unwrap(),
        HashMap::new(),
        "age".to_string(),
        ShapeId::new("smithy.api", "Integer").unwrap(),
    );

    let shape = StructureShape::builder()
        .id("example.foo#MyStruct")
        .member("name", member1)
        .member("age", member2)
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
        .documentation("A test structure")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyStruct");
    assert_eq!(shape.traits().len(), 1);
    assert!(shape.has_trait(ShapeId::new("smithy.api", "documentation").unwrap()));
}

#[test]
fn test_list_shape_builder() {
    let string_id = ShapeId::new("smithy.api", "String").unwrap();
    let member = MemberShape::new(
        "example.foo#MyList$member".parse().unwrap(),
        HashMap::new(),
        "member".to_string(),
        string_id,
    );

    let shape = ListShape::builder()
        .id("example.foo#MyList")
        .member(member)
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyList");
    assert_eq!(shape.traits().len(), 0);
    assert_eq!(shape.member.target.to_string(), "smithy.api#String");
}

#[test]
fn test_list_shape_builder_missing_member() {
    let result = ListShape::builder().id("example.foo#MyList").build();

    assert!(result.is_err());
}

#[test]
fn test_map_shape_builder() {
    let string_id = ShapeId::new("smithy.api", "String").unwrap();
    let integer_id = ShapeId::new("smithy.api", "Integer").unwrap();

    let key = MemberShape::new(
        "example.foo#MyMap$key".parse().unwrap(),
        HashMap::new(),
        "key".to_string(),
        string_id,
    );

    let value = MemberShape::new(
        "example.foo#MyMap$value".parse().unwrap(),
        HashMap::new(),
        "value".to_string(),
        integer_id,
    );

    let shape = MapShape::builder()
        .id("example.foo#MyMap")
        .key(key)
        .value(value)
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyMap");
    assert_eq!(shape.traits().len(), 0);
    assert_eq!(shape.key.target.to_string(), "smithy.api#String");
    assert_eq!(shape.value.target.to_string(), "smithy.api#Integer");
}

#[test]
fn test_map_shape_builder_missing_key() {
    let integer_id = ShapeId::new("smithy.api", "Integer").unwrap();
    let value = MemberShape::new(
        "example.foo#MyMap$value".parse().unwrap(),
        HashMap::new(),
        "value".to_string(),
        integer_id,
    );

    let result = MapShape::builder()
        .id("example.foo#MyMap")
        .value(value)
        .build();

    assert!(result.is_err());
}

#[test]
fn test_map_shape_builder_missing_value() {
    let string_id = ShapeId::new("smithy.api", "String").unwrap();
    let key = MemberShape::new(
        "example.foo#MyMap$key".parse().unwrap(),
        HashMap::new(),
        "key".to_string(),
        string_id,
    );

    let result = MapShape::builder().id("example.foo#MyMap").key(key).build();

    assert!(result.is_err());
}
