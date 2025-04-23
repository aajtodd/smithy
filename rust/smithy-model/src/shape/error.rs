/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error types for shape building operations.

use std::error::Error;
use std::fmt;

// FIXME - our errors should really include shape ID most of the time probably

/// Error type for shape building operations.
#[derive(Debug)]
pub enum BuildError {
    /// A required field was not set.
    MissingRequiredField {
        /// The name of the field that was missing.
        field: String,
    },
    /// A field had an invalid value.
    InvalidValue {
        /// The name of the field with the invalid value.
        field: String,
        /// A description of why the value is invalid.
        reason: String,
    },
    /// Other errors that don't fit into the categories above.
    Other(String),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::MissingRequiredField { field } => {
                write!(f, "Missing required field: {}", field)
            }
            BuildError::InvalidValue { field, reason } => {
                write!(f, "Invalid value for field {}: {}", field, reason)
            }
            BuildError::Other(message) => write!(f, "{}", message),
        }
    }
}

impl Error for BuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_required_field_display() {
        let err = BuildError::MissingRequiredField {
            field: "name".to_string(),
        };
        assert_eq!(err.to_string(), "Missing required field: name");
    }

    #[test]
    fn test_invalid_value_display() {
        let err = BuildError::InvalidValue {
            field: "age".to_string(),
            reason: "must be positive".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Invalid value for field age: must be positive"
        );
    }

    #[test]
    fn test_other_display() {
        let err = BuildError::Other("Something went wrong".to_string());
        assert_eq!(err.to_string(), "Something went wrong");
    }
}
