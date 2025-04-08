/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! AST data structures for the Smithy IDL.
//!
//! This module defines the Abstract Syntax Tree (AST) representation of Smithy models.

use std::collections::HashMap;

/// The root AST node representing a complete Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    /// The version of the Smithy IDL.
    pub version: String,
    /// The namespace declarations in the model.
    pub namespaces: Vec<Namespace>,
    /// Any control statements in the model.
    pub control_statements: Vec<ControlStatement>,
    /// Any metadata in the model.
    pub metadata: HashMap<String, MetadataValue>,
}

/// A namespace declaration in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct Namespace {
    /// The name of the namespace.
    pub name: String,
    /// The shapes defined in this namespace.
    pub shapes: Vec<ShapeStatement>,
    /// Any use statements in this namespace.
    pub use_statements: Vec<UseStatement>,
}

/// A shape statement in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeStatement {
    /// The type of the shape.
    pub shape_type: ShapeType,
    /// The name of the shape.
    pub name: String,
    /// Any traits applied to the shape.
    pub traits: Vec<TraitStatement>,
    /// Any members of the shape (for aggregate shapes).
    pub members: Vec<MemberStatement>,
    /// Any mixins applied to the shape.
    pub mixins: Vec<String>,
    /// Any resource identifiers (for resource shapes).
    pub identifiers: HashMap<String, String>,
    /// Any operations associated with the shape (for service or resource shapes).
    pub operations: Vec<String>,
    /// Any resources associated with the shape (for service or resource shapes).
    pub resources: Vec<String>,
    /// Any errors associated with the shape (for operation shapes).
    pub errors: Vec<String>,
    /// The input shape for an operation.
    pub input: Option<String>,
    /// The output shape for an operation.
    pub output: Option<String>,
}

/// A member statement in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct MemberStatement {
    /// The name of the member.
    pub name: String,
    /// The target shape of the member.
    pub target: String,
    /// Any traits applied to the member.
    pub traits: Vec<TraitStatement>,
}

/// A trait statement in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct TraitStatement {
    /// The name of the trait.
    pub name: String,
    /// The value of the trait, if any.
    pub value: Option<TraitValue>,
}

/// A use statement in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct UseStatement {
    /// The shape ID being imported.
    pub shape_id: String,
}

/// A control statement in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlStatement {
    /// The name of the control statement.
    pub name: String,
    /// The value of the control statement.
    pub value: MetadataValue,
}

/// The type of a shape in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeType {
    /// A simple boolean type.
    Boolean,
    /// A simple byte type.
    Byte,
    /// A simple short type.
    Short,
    /// A simple integer type.
    Integer,
    /// A simple long type.
    Long,
    /// A simple float type.
    Float,
    /// A simple double type.
    Double,
    /// A simple string type.
    String,
    /// A simple blob type.
    Blob,
    /// A simple timestamp type.
    Timestamp,
    /// A list aggregate type.
    List,
    /// A set aggregate type.
    Set,
    /// A map aggregate type.
    Map,
    /// A structure aggregate type.
    Structure,
    /// A union aggregate type.
    Union,
    /// A service type.
    Service,
    /// An operation type.
    Operation,
    /// A resource type.
    Resource,
    /// A member reference.
    Member,
}

/// A value for a trait in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub enum TraitValue {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(f64),
    /// A boolean value.
    Boolean(bool),
    /// An array of values.
    Array(Vec<TraitValue>),
    /// An object with string keys and values.
    Object(HashMap<String, TraitValue>),
    /// A null value.
    Null,
}

/// A value for metadata in a Smithy model.
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataValue {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(f64),
    /// A boolean value.
    Boolean(bool),
    /// An array of values.
    Array(Vec<MetadataValue>),
    /// An object with string keys and values.
    Object(HashMap<String, MetadataValue>),
    /// A null value.
    Null,
}

impl Ast {
    /// Creates a new empty AST.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            namespaces: Vec::new(),
            control_statements: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}
