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
//! This crate provides the fundamental data structures for representing Smithy models,
//! including shapes, traits, and their relationships.

pub mod error;
pub mod loader;
pub mod model;
pub mod selector;
pub mod shape;
pub mod shape_id;
pub mod traits;
pub mod validation;

// Re-export of common types for convenience
pub use model::Model;
pub use shape::Shape;
pub use shape_id::ShapeId;
pub use traits::Trait;
