/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use smithy_model::shape::{BlobShape, HasShapeId, HasTraits, ShapeBuilderExt, StringShape};
use smithy_model::shape_id::ShapeId;
use smithy_model::traits::Trait;

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
