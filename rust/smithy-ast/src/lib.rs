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

//! AST representation and parser for the Smithy IDL.
//!
//! This crate provides an Abstract Syntax Tree (AST) representation of Smithy models
//! and a parser for the Smithy IDL format.

pub mod ast;
pub mod error;
pub mod parser;

// Re-export of common types for convenience
pub use ast::Ast;
pub use error::Error;
