/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error types for the Smithy model.

use thiserror::Error;

/// Errors that can occur when working with Smithy models.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error when a shape ID is invalid.
    #[error("Invalid shape ID: {0}")]
    InvalidShapeId(String),

    /// Error when a shape is not found in the model.
    #[error("Shape not found: {0}")]
    ShapeNotFound(String),

    /// Error when a trait is invalid.
    #[error("Invalid trait: {0}")]
    InvalidTrait(String),

    /// Error when a model validation fails.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error when a selector is invalid.
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    /// Error when loading a model.
    #[error("Model loading error: {0}")]
    LoadingError(String),

    /// Error when an I/O operation fails.
    #[error("I/O error: {0}")]
    Io(String),

    /// Error when parsing JSON.
    #[error("JSON error: {0}")]
    Json(String),

    /// Error when converting from AST to model.
    #[error("AST conversion error: {0}")]
    AstConversion(String),
}

/// Result type for Smithy model operations.
pub type Result<T> = std::result::Result<T, Error>;
