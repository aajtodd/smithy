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
