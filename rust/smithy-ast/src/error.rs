/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error types for the Smithy AST.

use thiserror::Error;

/// Errors that can occur when parsing Smithy IDL.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error when parsing Smithy IDL syntax.
    #[error("Parse error: {message} at line {line}, column {column}")]
    Parse {
        /// Error message
        message: String,
        /// Line number where the error occurred
        line: usize,
        /// Column number where the error occurred
        column: usize,
    },

    /// Error when an unexpected token is encountered.
    #[error("Unexpected token: {found} at line {line}, column {column}, expected {expected}")]
    UnexpectedToken {
        /// The token that was found
        found: String,
        /// The token that was expected
        expected: String,
        /// Line number where the error occurred
        line: usize,
        /// Column number where the error occurred
        column: usize,
    },

    /// Error when an I/O operation fails.
    #[error("I/O error: {0}")]
    Io(String),
}

/// Result type for Smithy AST operations.
pub type Result<T> = std::result::Result<T, Error>;
