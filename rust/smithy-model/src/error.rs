/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error types for the Smithy model.

use thiserror::Error;

/// Result type for Smithy model operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Smithy model operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Error for invalid shape IDs.
    #[error("Invalid shape ID: {0}")]
    InvalidShapeId(String),

    /// Error for shape not found.
    #[error("Shape not found: {0}")]
    ShapeNotFound(String),

    /// Error for shape type mismatch.
    #[error("Shape type mismatch: expected {1} but got {2} for shape {0}")]
    ShapeTypeMismatch(String, String, String),

    /// Error for loading models.
    #[error("Error loading model: {0}")]
    LoadingError(String),

    /// Error for invalid selectors.
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    /// Error for validation failures.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error for I/O operations.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
