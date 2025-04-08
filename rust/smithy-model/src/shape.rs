/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Shape implementations for the Smithy model.
//!
//! This module provides the Shape type and its variants, which represent the
//! different kinds of shapes in a Smithy model.

use crate::shape_id::ShapeId;
use crate::traits::Trait;
use std::collections::HashMap;

/// A shape in a Smithy model.
#[derive(Debug, Clone)]
pub enum Shape {
    /// A simple boolean type.
    Boolean(BooleanShape),
    /// A simple byte type.
    Byte(ByteShape),
    /// A simple short type.
    Short(ShortShape),
    /// A simple integer type.
    Integer(IntegerShape),
    /// A simple long type.
    Long(LongShape),
    /// A simple float type.
    Float(FloatShape),
    /// A simple double type.
    Double(DoubleShape),
    /// A simple string type.
    String(StringShape),
    /// A simple blob type.
    Blob(BlobShape),
    /// A simple timestamp type.
    Timestamp(TimestampShape),
    /// A list aggregate type.
    List(ListShape),
    /// A set aggregate type.
    Set(SetShape),
    /// A map aggregate type.
    Map(MapShape),
    /// A structure aggregate type.
    Structure(StructureShape),
    /// A union aggregate type.
    Union(UnionShape),
    /// A service type.
    Service(ServiceShape),
    /// An operation type.
    Operation(OperationShape),
    /// A resource type.
    Resource(Box<ResourceShape>),
    /// A member reference.
    Member(MemberShape),
}

/// Common properties for all shapes.
#[derive(Debug, Clone)]
pub struct ShapeCore {
    /// The ID of the shape.
    pub id: ShapeId,
    /// The traits applied to the shape.
    pub traits: HashMap<ShapeId, Trait>,
}

/// A simple boolean type.
#[derive(Debug, Clone)]
pub struct BooleanShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple byte type.
#[derive(Debug, Clone)]
pub struct ByteShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple short type.
#[derive(Debug, Clone)]
pub struct ShortShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple integer type.
#[derive(Debug, Clone)]
pub struct IntegerShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple long type.
#[derive(Debug, Clone)]
pub struct LongShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple float type.
#[derive(Debug, Clone)]
pub struct FloatShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple double type.
#[derive(Debug, Clone)]
pub struct DoubleShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple string type.
#[derive(Debug, Clone)]
pub struct StringShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple blob type.
#[derive(Debug, Clone)]
pub struct BlobShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A simple timestamp type.
#[derive(Debug, Clone)]
pub struct TimestampShape {
    /// The core shape properties.
    pub core: ShapeCore,
}

/// A list aggregate type.
#[derive(Debug, Clone)]
pub struct ListShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The member shape.
    pub member: Box<MemberShape>,
}

/// A set aggregate type.
#[derive(Debug, Clone)]
pub struct SetShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The member shape.
    pub member: Box<MemberShape>,
}

/// A map aggregate type.
#[derive(Debug, Clone)]
pub struct MapShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The key shape.
    pub key: Box<MemberShape>,
    /// The value shape.
    pub value: Box<MemberShape>,
}

/// A structure aggregate type.
#[derive(Debug, Clone)]
pub struct StructureShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The members of the structure.
    pub members: HashMap<String, MemberShape>,
}

/// A union aggregate type.
#[derive(Debug, Clone)]
pub struct UnionShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The members of the union.
    pub members: HashMap<String, MemberShape>,
}

/// A service type.
#[derive(Debug, Clone)]
pub struct ServiceShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The version of the service.
    pub version: String,
    /// The operations of the service.
    pub operations: Vec<ShapeId>,
    /// The resources of the service.
    pub resources: Vec<ShapeId>,
}

/// An operation type.
#[derive(Debug, Clone)]
pub struct OperationShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The input shape.
    pub input: Option<ShapeId>,
    /// The output shape.
    pub output: Option<ShapeId>,
    /// The errors that can be thrown by the operation.
    pub errors: Vec<ShapeId>,
}

/// A resource type.
#[derive(Debug, Clone)]
pub struct ResourceShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The identifiers of the resource.
    pub identifiers: HashMap<String, ShapeId>,
    /// The create operation.
    pub create: Option<ShapeId>,
    /// The read operation.
    pub read: Option<ShapeId>,
    /// The update operation.
    pub update: Option<ShapeId>,
    /// The delete operation.
    pub delete: Option<ShapeId>,
    /// The list operation.
    pub list: Option<ShapeId>,
    /// The operations of the resource.
    pub operations: Vec<ShapeId>,
    /// The resources of the resource.
    pub resources: Vec<ShapeId>,
}

/// A member reference.
#[derive(Debug, Clone)]
pub struct MemberShape {
    /// The core shape properties.
    pub core: ShapeCore,
    /// The target shape.
    pub target: ShapeId,
}

impl Shape {
    /// Returns the ID of the shape.
    pub fn id(&self) -> &ShapeId {
        match self {
            Shape::Boolean(s) => &s.core.id,
            Shape::Byte(s) => &s.core.id,
            Shape::Short(s) => &s.core.id,
            Shape::Integer(s) => &s.core.id,
            Shape::Long(s) => &s.core.id,
            Shape::Float(s) => &s.core.id,
            Shape::Double(s) => &s.core.id,
            Shape::String(s) => &s.core.id,
            Shape::Blob(s) => &s.core.id,
            Shape::Timestamp(s) => &s.core.id,
            Shape::List(s) => &s.core.id,
            Shape::Set(s) => &s.core.id,
            Shape::Map(s) => &s.core.id,
            Shape::Structure(s) => &s.core.id,
            Shape::Union(s) => &s.core.id,
            Shape::Service(s) => &s.core.id,
            Shape::Operation(s) => &s.core.id,
            Shape::Resource(s) => &s.core.id,
            Shape::Member(s) => &s.core.id,
        }
    }

    /// Returns the traits of the shape.
    pub fn traits(&self) -> &HashMap<ShapeId, Trait> {
        match self {
            Shape::Boolean(s) => &s.core.traits,
            Shape::Byte(s) => &s.core.traits,
            Shape::Short(s) => &s.core.traits,
            Shape::Integer(s) => &s.core.traits,
            Shape::Long(s) => &s.core.traits,
            Shape::Float(s) => &s.core.traits,
            Shape::Double(s) => &s.core.traits,
            Shape::String(s) => &s.core.traits,
            Shape::Blob(s) => &s.core.traits,
            Shape::Timestamp(s) => &s.core.traits,
            Shape::List(s) => &s.core.traits,
            Shape::Set(s) => &s.core.traits,
            Shape::Map(s) => &s.core.traits,
            Shape::Structure(s) => &s.core.traits,
            Shape::Union(s) => &s.core.traits,
            Shape::Service(s) => &s.core.traits,
            Shape::Operation(s) => &s.core.traits,
            Shape::Resource(s) => &s.core.traits,
            Shape::Member(s) => &s.core.traits,
        }
    }
}
