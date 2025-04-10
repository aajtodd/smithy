/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Simple shape types for the Smithy model.

use std::collections::HashMap;

use crate::shape::{ProvideShapeMetadata, Shape, ShapeMetadata};
use crate::shape_id::ShapeId;
use crate::traits::Trait;

/// A [boolean](https://smithy.io/2.0/spec/simple-types.html#boolean) shape
#[derive(Debug, Clone)]
pub struct BooleanShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [byte](https://smithy.io/2.0/spec/simple-types.html#byte) shape
#[derive(Debug, Clone)]
pub struct ByteShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [short](https://smithy.io/2.0/spec/simple-types.html#short) shape
#[derive(Debug, Clone)]
pub struct ShortShape {
    pub(crate) metadata: ShapeMetadata,
}

/// An [integer](https://smithy.io/2.0/spec/simple-types.html#integer) shape
#[derive(Debug, Clone)]
pub struct IntegerShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [long](https://smithy.io/2.0/spec/simple-types.html#long) shape
#[derive(Debug, Clone)]
pub struct LongShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [float](https://smithy.io/2.0/spec/simple-types.html#float) shape
#[derive(Debug, Clone)]
pub struct FloatShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [double](https://smithy.io/2.0/spec/simple-types.html#double) shape
#[derive(Debug, Clone)]
pub struct DoubleShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [bigInteger](https://smithy.io/2.0/spec/simple-types.html#biginteger) shape
#[derive(Debug, Clone)]
pub struct BigIntegerShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [bigDecimal](https://smithy.io/2.0/spec/simple-types.html#bigdecimal) shape
#[derive(Debug, Clone)]
pub struct BigDecimalShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [string](https://smithy.io/2.0/spec/simple-types.html#string) shape
#[derive(Debug, Clone)]
pub struct StringShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [blob](https://smithy.io/2.0/spec/simple-types.html#blob) shape
#[derive(Debug, Clone)]
pub struct BlobShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [timestamp](https://smithy.io/2.0/spec/simple-types.html#timestamp) shape
#[derive(Debug, Clone)]
pub struct TimestampShape {
    pub(crate) metadata: ShapeMetadata,
}

/// A [document](https://smithy.io/2.0/spec/simple-types.html#document) shape
#[derive(Debug, Clone)]
pub struct DocumentShape {
    pub(crate) metadata: ShapeMetadata,
}

/// An [enum](https://smithy.io/2.0/spec/simple-types.html#enum) shape
#[derive(Debug, Clone)]
pub struct EnumShape {
    pub(crate) metadata: ShapeMetadata,
}

/// An [intEnum](https://smithy.io/2.0/spec/simple-types.html#intenum) shape
#[derive(Debug, Clone)]
pub struct IntEnumShape {
    pub(crate) metadata: ShapeMetadata,
}

// Implement ProvideShapeMetadata for all simple shapes
impl ProvideShapeMetadata for BooleanShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for ByteShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for ShortShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for IntegerShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for LongShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for FloatShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for DoubleShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for BigIntegerShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for BigDecimalShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for StringShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for BlobShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for TimestampShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for DocumentShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for EnumShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl ProvideShapeMetadata for IntEnumShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

// Implement constructors for simple shapes
impl BooleanShape {
    /// Create a new boolean shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl ByteShape {
    /// Create a new byte shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl ShortShape {
    /// Create a new short shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl IntegerShape {
    /// Create a new integer shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl LongShape {
    /// Create a new long shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl FloatShape {
    /// Create a new float shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl DoubleShape {
    /// Create a new double shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl BigIntegerShape {
    /// Create a new big integer shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl BigDecimalShape {
    /// Create a new big decimal shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl StringShape {
    /// Create a new string shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl BlobShape {
    /// Create a new blob shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl TimestampShape {
    /// Create a new timestamp shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl DocumentShape {
    /// Create a new document shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl EnumShape {
    /// Create a new enum shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

impl IntEnumShape {
    /// Create a new integer enum shape
    pub fn new(id: ShapeId, traits: HashMap<ShapeId, Trait>) -> Self {
        Self {
            metadata: ShapeMetadata::new(id, traits),
        }
    }
}

// Implement From traits for converting shape structs to Shape enum
impl From<BooleanShape> for Shape {
    fn from(shape: BooleanShape) -> Self {
        Shape::Boolean(shape)
    }
}

impl From<ByteShape> for Shape {
    fn from(shape: ByteShape) -> Self {
        Shape::Byte(shape)
    }
}

impl From<ShortShape> for Shape {
    fn from(shape: ShortShape) -> Self {
        Shape::Short(shape)
    }
}

impl From<IntegerShape> for Shape {
    fn from(shape: IntegerShape) -> Self {
        Shape::Integer(shape)
    }
}

impl From<LongShape> for Shape {
    fn from(shape: LongShape) -> Self {
        Shape::Long(shape)
    }
}

impl From<FloatShape> for Shape {
    fn from(shape: FloatShape) -> Self {
        Shape::Float(shape)
    }
}

impl From<DoubleShape> for Shape {
    fn from(shape: DoubleShape) -> Self {
        Shape::Double(shape)
    }
}

impl From<BigIntegerShape> for Shape {
    fn from(shape: BigIntegerShape) -> Self {
        Shape::BigInteger(shape)
    }
}

impl From<BigDecimalShape> for Shape {
    fn from(shape: BigDecimalShape) -> Self {
        Shape::BigDecimal(shape)
    }
}

impl From<StringShape> for Shape {
    fn from(shape: StringShape) -> Self {
        Shape::String(shape)
    }
}

impl From<BlobShape> for Shape {
    fn from(shape: BlobShape) -> Self {
        Shape::Blob(shape)
    }
}

impl From<TimestampShape> for Shape {
    fn from(shape: TimestampShape) -> Self {
        Shape::Timestamp(shape)
    }
}

impl From<DocumentShape> for Shape {
    fn from(shape: DocumentShape) -> Self {
        Shape::Document(shape)
    }
}

impl From<EnumShape> for Shape {
    fn from(shape: EnumShape) -> Self {
        Shape::Enum(shape)
    }
}

impl From<IntEnumShape> for Shape {
    fn from(shape: IntEnumShape) -> Self {
        Shape::IntEnum(shape)
    }
}
