/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! ShapeId implementation for the Smithy model.
//!
//! This module provides the ShapeId type, which uniquely identifies shapes in a Smithy model.
//! A shape ID consists of a namespace, a name, and an optional member name.
//! For example, `com.example.foo#Bar` or `com.example.foo#Bar$baz`.

use crate::error::{Error, Result};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

/// A unique identifier for a shape in a Smithy model.
///
/// A shape ID consists of a namespace, a name, and an optional member name.
/// For example, `com.example.foo#Bar` or `com.example.foo#Bar$baz`.
///
/// # Examples
///
/// ```
/// use smithy_model::ShapeId;
/// use std::str::FromStr;
///
/// // Create a shape ID from components
/// let shape_id = ShapeId::new("com.example", "MyShape").unwrap();
/// assert_eq!(shape_id.to_string(), "com.example#MyShape");
///
/// // Create a shape ID with a member
/// let member_id = ShapeId::new_with_member("com.example", "MyStruct", "field").unwrap();
/// assert_eq!(member_id.to_string(), "com.example#MyStruct$field");
///
/// // Parse a shape ID from a string
/// let parsed = ShapeId::from_str("com.example#MyShape").unwrap();
/// assert_eq!(parsed.namespace(), "com.example");
/// assert_eq!(parsed.name(), "MyShape");
/// assert_eq!(parsed.member(), None);
///
/// // Parse a shape ID with a member
/// let parsed_member = ShapeId::from_str("com.example#MyStruct$field").unwrap();
/// assert_eq!(parsed_member.namespace(), "com.example");
/// assert_eq!(parsed_member.name(), "MyStruct");
/// assert_eq!(parsed_member.member(), Some("field"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShapeId {
    namespace: Cow<'static, str>,
    name: Cow<'static, str>,
    member: Option<Cow<'static, str>>,
}

/// Validates a namespace according to the Smithy specification.
///
/// A valid namespace consists of one or more identifiers separated by dots.
fn validate_namespace(namespace: &str) -> Result<()> {
    if namespace.is_empty() {
        return Err(Error::InvalidShapeId(
            "Namespace cannot be empty".to_string(),
        ));
    }

    for (i, segment) in namespace.split('.').enumerate() {
        if segment.is_empty() {
            return Err(Error::InvalidShapeId(
                "Namespace contains empty segment".to_string(),
            ));
        }

        match validate_identifier(segment) {
            Ok(_) => {}
            Err(Error::InvalidShapeId(msg)) => {
                return Err(Error::InvalidShapeId(format!(
                    "Invalid namespace segment at position {}: {}",
                    i, msg
                )));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

/// Validates an identifier according to the Smithy specification.
///
/// A valid identifier must start with a letter or underscore(s) followed by a letter or digit,
/// and can contain only letters, numbers, and underscores.
fn validate_identifier(identifier: &str) -> Result<()> {
    if identifier.is_empty() {
        return Err(Error::InvalidShapeId(
            "Identifier cannot be empty".to_string(),
        ));
    }

    let mut chars = identifier.chars();

    // Check first character (IdentifierStart)
    match chars.next() {
        Some(c) if c.is_alphabetic() => {} // ALPHA is valid
        Some('_') => {
            // One or more underscores followed by ALPHA or DIGIT
            let mut rest = chars.clone();

            while let Some('_') = rest.next() {
                chars.next(); // Advance the original iterator
            }

            match chars.next() {
                Some(c) if c.is_alphanumeric() => {}, // Valid after underscore(s)
                _ => return Err(Error::InvalidShapeId(format!(
                    "Identifier '{}' starts with underscore(s) but must be followed by a letter or digit",
                    identifier
                )))
            }
        }
        Some(c) => {
            return Err(Error::InvalidShapeId(format!(
                "Identifier '{}' starts with '{}' but must start with a letter or underscore",
                identifier, c
            )))
        }
        None => {
            return Err(Error::InvalidShapeId(
                "Identifier cannot be empty".to_string(),
            ))
        }
    }

    // Check remaining characters (IdentifierChars)
    for (i, c) in chars.enumerate() {
        if !c.is_alphanumeric() && c != '_' {
            return Err(Error::InvalidShapeId(format!(
                "Identifier '{}' contains invalid character '{}' at position {}; only letters, numbers, and underscores are allowed",
                identifier, c, i + 1
            )));
        }
    }

    Ok(())
}

/// Helper function to create a ShapeId from parts with validation.
fn try_from_parts(
    namespace: impl Into<Cow<'static, str>>,
    name: impl Into<Cow<'static, str>>,
    member: Option<impl Into<Cow<'static, str>>>,
) -> Result<ShapeId> {
    let namespace = namespace.into();
    let name = name.into();

    validate_namespace(&namespace)?;
    validate_identifier(&name)?;

    let member = if let Some(m) = member {
        let m = m.into();
        validate_identifier(&m)?;
        Some(m)
    } else {
        None
    };

    Ok(ShapeId {
        namespace,
        name,
        member,
    })
}

impl ShapeId {
    /// Creates a new ShapeId without a member.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace of the shape.
    /// * `name` - The name of the shape.
    ///
    /// # Returns
    ///
    /// A new ShapeId, or an error if the namespace or name is invalid.
    pub fn new(
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<Self> {
        try_from_parts(namespace, name, None::<String>)
    }

    /// Creates a new ShapeId from a static string without checking the validity of the id
    pub const fn new_static(namespace: &'static str, name: &'static str) -> ShapeId {
        ShapeId {
            namespace: Cow::Borrowed(namespace),
            name: Cow::Borrowed(name),
            member: None,
        }
    }

    /// Creates a new ShapeId from a static string without checking the validity of the id
    #[doc(hidden)]
    pub fn new_unchecked(id: &'static str) -> ShapeId {
        let (namespace, name) = id.split_once('#').unwrap();
        let (name, member) = if let Some((name, member)) = name.split_once('$') {
            (name, Some(member))
        } else {
            (name, None)
        };

        ShapeId {
            namespace: namespace.into(),
            name: name.into(),
            member: member.map(|m| m.into()),
        }
    }

    /// Creates a new ShapeId with a member.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace of the shape.
    /// * `name` - The name of the shape.
    /// * `member` - The member name.
    ///
    /// # Returns
    ///
    /// A new ShapeId, or an error if any component is invalid.
    pub fn new_with_member(
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        member: impl Into<Cow<'static, str>>,
    ) -> Result<Self> {
        try_from_parts(namespace, name, Some(member))
    }

    /// Returns the namespace of the shape.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the name of the shape.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the member name of the shape, if any.
    pub fn member(&self) -> Option<&str> {
        self.member.as_deref()
    }

    /// Returns true if this shape ID has a member name.
    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }

    /// Returns a new shape ID with the given member name.
    ///
    /// # Arguments
    ///
    /// * `member` - The member name to add.
    ///
    /// # Returns
    ///
    /// A new ShapeId with the given member, or an error if the member name is invalid.
    pub fn with_member(&self, member: impl Into<Cow<'static, str>>) -> Result<ShapeId> {
        let member = member.into();
        validate_identifier(&member)?;

        Ok(ShapeId {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            member: Some(member),
        })
    }

    /// Create a new ShapeId with no member
    pub fn without_member(&self) -> ShapeId {
        ShapeId {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            member: None,
        }
    }
}

impl fmt::Display for ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.namespace, self.name)?;
        if let Some(member) = &self.member {
            write!(f, "${}", member)?;
        }
        Ok(())
    }
}

impl FromStr for ShapeId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Check for empty string
        if s.is_empty() {
            return Err(Error::InvalidShapeId(
                "Shape ID cannot be empty".to_string(),
            ));
        }

        // Split the string into namespace#name and optional $member
        let pound_index = s.find('#').ok_or_else(|| {
            Error::InvalidShapeId(format!(
                "Missing namespace delimiter '#' in shape ID: '{}'",
                s
            ))
        })?;

        if pound_index == 0 {
            return Err(Error::InvalidShapeId(format!(
                "Missing namespace before '#' in shape ID: '{}'",
                s
            )));
        }

        if pound_index == s.len() - 1 {
            return Err(Error::InvalidShapeId(format!(
                "Missing shape name after '#' in shape ID: '{}'",
                s
            )));
        }

        if s[pound_index + 1..].contains('#') {
            return Err(Error::InvalidShapeId(format!(
                "Shape ID contains multiple '#' characters: '{}'",
                s
            )));
        }

        let namespace = &s[..pound_index];
        let rest = &s[pound_index + 1..];

        // Split name and optional member
        let (name, member) = match rest.find('$') {
            Some(dollar_index) => {
                if dollar_index == 0 {
                    return Err(Error::InvalidShapeId(format!(
                        "Missing shape name before '$' in shape ID: '{}'",
                        s
                    )));
                }

                if dollar_index == rest.len() - 1 {
                    return Err(Error::InvalidShapeId(format!(
                        "Missing member name after '$' in shape ID: '{}'",
                        s
                    )));
                }

                if rest[dollar_index + 1..].contains('$') {
                    return Err(Error::InvalidShapeId(format!(
                        "Shape ID contains multiple '$' characters: '{}'",
                        s
                    )));
                }

                (&rest[..dollar_index], Some(&rest[dollar_index + 1..]))
            }
            None => (rest, None),
        };

        // Use try_from_parts to validate and create the ShapeId
        try_from_parts(
            namespace.to_string(),
            name.to_string(),
            member.map(|m| m.to_string()),
        )
    }
}

impl TryFrom<&str> for ShapeId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        s.parse()
    }
}

impl TryFrom<String> for ShapeId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        s.parse()
    }
}

impl AsRef<ShapeId> for ShapeId {
    fn as_ref(&self) -> &ShapeId {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shape_id() {
        let shape_id = ShapeId::new("com.example", "MyShape").unwrap();
        assert_eq!(shape_id.namespace(), "com.example");
        assert_eq!(shape_id.name(), "MyShape");
        assert_eq!(shape_id.member(), None);
        assert!(!shape_id.has_member());
    }

    #[test]
    fn test_new_shape_id_with_member() {
        let shape_id = ShapeId::new_with_member("com.example", "MyStruct", "field").unwrap();
        assert_eq!(shape_id.namespace(), "com.example");
        assert_eq!(shape_id.name(), "MyStruct");
        assert_eq!(shape_id.member(), Some("field"));
        assert!(shape_id.has_member());
    }

    #[test]
    fn test_with_member() {
        let shape_id = ShapeId::new("com.example", "MyStruct").unwrap();
        let member_id = shape_id.with_member("field").unwrap();
        assert_eq!(member_id.namespace(), "com.example");
        assert_eq!(member_id.name(), "MyStruct");
        assert_eq!(member_id.member(), Some("field"));
    }

    #[test]
    fn test_display() {
        let shape_id = ShapeId::new("com.example", "MyShape").unwrap();
        assert_eq!(shape_id.to_string(), "com.example#MyShape");

        let member_id = ShapeId::new_with_member("com.example", "MyStruct", "field").unwrap();
        assert_eq!(member_id.to_string(), "com.example#MyStruct$field");
    }

    #[test]
    fn test_from_str() {
        let shape_id = ShapeId::from_str("com.example#MyShape").unwrap();
        assert_eq!(shape_id.namespace(), "com.example");
        assert_eq!(shape_id.name(), "MyShape");
        assert_eq!(shape_id.member(), None);

        let member_id = ShapeId::from_str("com.example#MyStruct$field").unwrap();
        assert_eq!(member_id.namespace(), "com.example");
        assert_eq!(member_id.name(), "MyStruct");
        assert_eq!(member_id.member(), Some("field"));
    }

    #[test]
    fn test_try_from() {
        let shape_id = ShapeId::try_from("com.example#MyShape").unwrap();
        assert_eq!(shape_id.namespace(), "com.example");
        assert_eq!(shape_id.name(), "MyShape");
        assert_eq!(shape_id.member(), None);

        let member_id = ShapeId::try_from("com.example#MyStruct$field".to_string()).unwrap();
        assert_eq!(member_id.namespace(), "com.example");
        assert_eq!(member_id.name(), "MyStruct");
        assert_eq!(member_id.member(), Some("field"));
    }

    #[test]
    fn test_from_str_errors() {
        // Empty string
        assert!(ShapeId::from_str("").is_err());

        // Missing #
        assert!(ShapeId::from_str("com.example.MyShape").is_err());

        // Multiple #
        assert!(ShapeId::from_str("com.example#My#Shape").is_err());

        // Multiple $
        assert!(ShapeId::from_str("com.example#MyShape$field$extra").is_err());

        // Empty namespace
        assert!(ShapeId::from_str("#MyShape").is_err());

        // Empty name
        assert!(ShapeId::from_str("com.example#").is_err());

        // Empty member
        assert!(ShapeId::from_str("com.example#MyShape$").is_err());
    }

    #[test]
    fn test_namespace_validation() {
        // Valid namespaces
        assert!(ShapeId::new("com.example", "MyShape").is_ok());
        assert!(ShapeId::new("com", "MyShape").is_ok());
        assert!(ShapeId::new("com.example.sub", "MyShape").is_ok());
        assert!(ShapeId::new("_com.example", "MyShape").is_ok());
        assert!(ShapeId::new("com._example", "MyShape").is_ok());

        // Invalid namespaces
        assert!(ShapeId::new("", "MyShape").is_err());
        assert!(ShapeId::new("com..example", "MyShape").is_err());
        assert!(ShapeId::new(".com.example", "MyShape").is_err());
        assert!(ShapeId::new("com.example.", "MyShape").is_err());
        assert!(ShapeId::new("com.exam-ple", "MyShape").is_err());
    }

    #[test]
    fn test_identifier_validation() {
        // Valid identifiers
        assert!(ShapeId::new("com.example", "MyShape").is_ok());
        assert!(ShapeId::new("com.example", "My_Shape").is_ok());
        assert!(ShapeId::new("com.example", "_MyShape").is_ok());
        assert!(ShapeId::new("com.example", "__MyShape").is_ok());
        assert!(ShapeId::new("com.example", "_1Shape").is_ok());

        // Invalid identifiers
        assert!(ShapeId::new("com.example", "").is_err());
        assert!(ShapeId::new("com.example", "1Shape").is_err());
        assert!(ShapeId::new("com.example", "My-Shape").is_err());
        assert!(ShapeId::new("com.example", "_").is_err());
        assert!(ShapeId::new("com.example", "__").is_err());
    }

    #[test]
    fn test_member_validation() {
        // Valid members
        assert!(ShapeId::new_with_member("com.example", "MyShape", "field").is_ok());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "field_name").is_ok());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "_field").is_ok());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "__field").is_ok());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "_1field").is_ok());

        // Invalid members
        assert!(ShapeId::new_with_member("com.example", "MyShape", "").is_err());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "1field").is_err());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "field-name").is_err());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "_").is_err());
        assert!(ShapeId::new_with_member("com.example", "MyShape", "__").is_err());
    }

    #[test]
    fn test_equality() {
        let id1 = ShapeId::new("com.example", "MyShape").unwrap();
        let id2 = ShapeId::new("com.example", "MyShape").unwrap();
        let id3 = ShapeId::new("com.example", "OtherShape").unwrap();
        let id4 = ShapeId::new_with_member("com.example", "MyShape", "field").unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id1, id4);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let id1 = ShapeId::new("com.example", "MyShape").unwrap();
        let id2 = ShapeId::new("com.example", "MyShape").unwrap();
        let id3 = ShapeId::new("com.example", "OtherShape").unwrap();

        let mut set = HashSet::new();
        set.insert(id1);

        assert!(set.contains(&id2));
        assert!(!set.contains(&id3));
    }
}
