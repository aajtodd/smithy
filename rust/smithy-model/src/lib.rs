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

// TODO - either hide all shape members and force going through accessors (e.g. exposing IndexMap is not desired)
// TODO - shape builders with members should allow querying for members as well, it's only possible to add/remove currently
// TODO - consider how to commonize named member builders/access
// TODO - resources are likely needing attention
// TODO - can probably commonize service/resource shapes (e.g. Java EntityShape(Builder))
// TODO - define model (container) APIs
// TODO - model parsing
// TODO - model loading/assembly
// TODO - model validation framework
// TODO - unit type
// TODO - additional traits
// TODO - cleanup our errors, they should probably all have shape ID references when appropriate and possibly source locations, etc.
// TODO - define our prelude
