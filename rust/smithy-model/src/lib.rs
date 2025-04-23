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
pub mod traits;

pub mod node;
pub mod shape_id;

pub use node::Node;

pub use shape_id::ShapeId;

pub use error::{Error, Result};

// TODO - define our prelude

// TODO - unit type
// TODO - define macro(s) for common trait types like annotation, or single string/value
// TODO - organize traits into modules based on spec (e.g. constraint, refine, behavior, resource, etc)
// TODO - resources are likely needing attention
// TODO - can probably commonize service/resource shapes (e.g. Java EntityShape(Builder))
// TODO - define model (container) APIs
// TODO - model parsing
// TODO - model loading/assembly
// TODO - model validation framework
// TODO - trait macro(s) for defining simple traits
// TODO - additional traits
