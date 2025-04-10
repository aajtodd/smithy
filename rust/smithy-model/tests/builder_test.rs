/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use smithy_model::shape::{
    BlobShape, HasShapeId, HasTraits, ListShape, MapShape, MemberShape, OperationShape,
    ResourceShape, ServiceShape, ShapeBuilderExt, StringShape, StructureShape,
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

#[test]
fn test_service_shape_builder() {
    let shape = ServiceShape::builder()
        .id("example.foo#MyService")
        .version("2023-01-01")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyService");
    assert_eq!(shape.traits().len(), 0);
    assert_eq!(shape.operations.len(), 0);
    assert_eq!(shape.resources.len(), 0);
    assert_eq!(shape.version, Some("2023-01-01".to_string()));
}

#[test]
fn test_service_shape_builder_with_operations_and_resources() {
    let op1 = ShapeId::new("example.foo", "GetItem").unwrap();
    let op2 = ShapeId::new("example.foo", "PutItem").unwrap();
    let res1 = ShapeId::new("example.foo", "Item").unwrap();

    let shape = ServiceShape::builder()
        .id("example.foo#MyService")
        .operation(op1.clone())
        .operation(op2.clone())
        .resource(res1.clone())
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#MyService");
    assert_eq!(shape.operations.len(), 2);
    assert_eq!(shape.resources.len(), 1);
    assert!(shape.operations.contains(&op1));
    assert!(shape.operations.contains(&op2));
    assert!(shape.resources.contains(&res1));
}

#[test]
fn test_operation_shape_builder() {
    let shape = OperationShape::builder()
        .id("example.foo#GetItem")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#GetItem");
    assert_eq!(shape.traits().len(), 0);
    assert!(shape.input.is_none());
    assert!(shape.output.is_none());
    assert_eq!(shape.errors.len(), 0);
}

#[test]
fn test_operation_shape_builder_with_input_output_errors() {
    let input = ShapeId::new("example.foo", "GetItemInput").unwrap();
    let output = ShapeId::new("example.foo", "GetItemOutput").unwrap();
    let error1 = ShapeId::new("example.foo", "NotFoundError").unwrap();
    let error2 = ShapeId::new("example.foo", "InternalError").unwrap();

    let shape = OperationShape::builder()
        .id("example.foo#GetItem")
        .input(input.clone())
        .output(output.clone())
        .error(error1.clone())
        .error(error2.clone())
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#GetItem");
    assert_eq!(shape.input, Some(input));
    assert_eq!(shape.output, Some(output));
    assert_eq!(shape.errors.len(), 2);
    assert!(shape.errors.contains(&error1));
    assert!(shape.errors.contains(&error2));
}

#[test]
fn test_resource_shape_builder() {
    let shape = ResourceShape::builder()
        .id("example.foo#Item")
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#Item");
    assert_eq!(shape.traits().len(), 0);
    assert_eq!(shape.identifiers.len(), 0);
    assert!(shape.create.is_none());
    assert!(shape.read.is_none());
    assert!(shape.update.is_none());
    assert!(shape.delete.is_none());
    assert!(shape.list.is_none());
    assert_eq!(shape.operations.len(), 0);
    assert_eq!(shape.resources.len(), 0);
}

#[test]
fn test_resource_shape_builder_with_crud_operations() {
    let id_shape = ShapeId::new("example.foo", "ItemId").unwrap();
    let create_op = ShapeId::new("example.foo", "CreateItem").unwrap();
    let read_op = ShapeId::new("example.foo", "GetItem").unwrap();
    let update_op = ShapeId::new("example.foo", "UpdateItem").unwrap();
    let delete_op = ShapeId::new("example.foo", "DeleteItem").unwrap();
    let list_op = ShapeId::new("example.foo", "ListItems").unwrap();

    let shape = ResourceShape::builder()
        .id("example.foo#Item")
        .identifier("itemId", id_shape.clone())
        .create(create_op.clone())
        .read(read_op.clone())
        .update(update_op.clone())
        .delete(delete_op.clone())
        .list(list_op.clone())
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#Item");
    assert_eq!(shape.identifiers.len(), 1);
    assert_eq!(shape.identifiers.get("itemId"), Some(&id_shape));
    assert_eq!(shape.create, Some(create_op));
    assert_eq!(shape.read, Some(read_op));
    assert_eq!(shape.update, Some(update_op));
    assert_eq!(shape.delete, Some(delete_op));
    assert_eq!(shape.list, Some(list_op));
}

#[test]
fn test_resource_shape_builder_with_nested_resources_and_operations() {
    let op1 = ShapeId::new("example.foo", "GetItemDetails").unwrap();
    let op2 = ShapeId::new("example.foo", "GetItemHistory").unwrap();
    let res1 = ShapeId::new("example.foo", "ItemComment").unwrap();
    let res2 = ShapeId::new("example.foo", "ItemTag").unwrap();

    let shape = ResourceShape::builder()
        .id("example.foo#Item")
        .operation(op1.clone())
        .operation(op2.clone())
        .resource(res1.clone())
        .resource(res2.clone())
        .build()
        .unwrap();

    assert_eq!(shape.id().to_string(), "example.foo#Item");
    assert_eq!(shape.operations.len(), 2);
    assert_eq!(shape.resources.len(), 2);
    assert!(shape.operations.contains(&op1));
    assert!(shape.operations.contains(&op2));
    assert!(shape.resources.contains(&res1));
    assert!(shape.resources.contains(&res2));
}
