/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Validation functionality for the Smithy model.
//!
//! This module provides functionality for validating Smithy models.

use crate::error::Result;
use crate::model::Model;

/// A validator for Smithy models.
pub trait Validator {
    /// Validates a model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to validate.
    ///
    /// # Returns
    ///
    /// A list of validation events, or an error if validation fails.
    fn validate(&self, model: &Model) -> Result<Vec<ValidationEvent>>;
}

/// A validation event for a Smithy model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationEvent {
    /// The ID of the event.
    pub id: String,
    /// The severity of the event.
    pub severity: Severity,
    /// The message of the event.
    pub message: String,
    /// The shape ID associated with the event, if any.
    pub shape_id: Option<String>,
}

/// The severity of a validation event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// An error that prevents the model from being used.
    Error,
    /// A warning that does not prevent the model from being used.
    Warning,
    /// An informational message.
    Info,
}

/// Validates a model using the default validators.
///
/// # Arguments
///
/// * `model` - The model to validate.
///
/// # Returns
///
/// A list of validation events, or an error if validation fails.
pub fn validate(_model: &Model) -> Result<Vec<ValidationEvent>> {
    // This is a placeholder implementation.
    // The actual validation will be implemented in a future task.
    Ok(Vec::new())
}
