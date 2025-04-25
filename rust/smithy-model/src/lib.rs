/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Core model representation for the Smithy IDL.
//!
//! This crate provides the core data structures and functionality for working with Smithy models.

pub mod error;
pub mod shape;
#[macro_use]
pub mod traits;

pub mod node;

pub use node::Node;

pub use error::{Error, Result};

// TODO - define our prelude

// TODO - unit type
// TODO - consider how to commonize named member builders/access
//        - perhaps a simple macro and change from IndexMap to Members and change the members() function to return something else or &Members?
//        - expand capability of Members to add/remove members and combine with simple macro?
/*
e.g.

struct Members { ... }

fn members(&self) -> &Members;

impl Members {

    fn add_member(&mut self, member: MemberShape) { ... }
    fn remove_member(&mut self, member_id: impl AsRef<ShapeId>) { ... }
    fn clear_members(&mut self) { ... }
    fn get_member(&self, name: impl AsRef<str>) -> Option<&MemberShape>
    ...
}

struct StructureShapeBuilder {
    members: Members,
    ...
}

derive_named_members_builder(StructureShapeBuilder);

 */
// TODO - resources are likely needing attention
// TODO - can probably commonize service/resource shapes (e.g. Java EntityShape(Builder))
// TODO - define model (container) APIs
// TODO - model parsing
// TODO - model loading/assembly
// TODO - model validation framework
// TODO - trait macro(s) for defining simple traits
// TODO - additional traits
