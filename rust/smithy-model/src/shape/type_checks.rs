/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Type checking and conversion methods for the Shape enum.

use crate::shape::{aggregate::*, service::*, simple::*, HasShapeId, MemberShape, Shape};

impl Shape {
    // Type checking methods for simple shapes

    /// Returns true if this shape is a boolean shape.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Shape::Boolean(_))
    }

    /// Returns true if this shape is a byte shape.
    pub fn is_byte(&self) -> bool {
        matches!(self, Shape::Byte(_))
    }

    /// Returns true if this shape is a short shape.
    pub fn is_short(&self) -> bool {
        matches!(self, Shape::Short(_))
    }

    /// Returns true if this shape is an integer shape.
    pub fn is_integer(&self) -> bool {
        matches!(self, Shape::Integer(_))
    }

    /// Returns true if this shape is a long shape.
    pub fn is_long(&self) -> bool {
        matches!(self, Shape::Long(_))
    }

    /// Returns true if this shape is a float shape.
    pub fn is_float(&self) -> bool {
        matches!(self, Shape::Float(_))
    }

    /// Returns true if this shape is a double shape.
    pub fn is_double(&self) -> bool {
        matches!(self, Shape::Double(_))
    }

    /// Returns true if this shape is a big integer shape.
    pub fn is_big_integer(&self) -> bool {
        matches!(self, Shape::BigInteger(_))
    }

    /// Returns true if this shape is a big decimal shape.
    pub fn is_big_decimal(&self) -> bool {
        matches!(self, Shape::BigDecimal(_))
    }

    /// Returns true if this shape is a string shape.
    pub fn is_string(&self) -> bool {
        matches!(self, Shape::String(_))
    }

    /// Returns true if this shape is a blob shape.
    pub fn is_blob(&self) -> bool {
        matches!(self, Shape::Blob(_))
    }

    /// Returns true if this shape is a timestamp shape.
    pub fn is_timestamp(&self) -> bool {
        matches!(self, Shape::Timestamp(_))
    }

    /// Returns true if this shape is a document shape.
    pub fn is_document(&self) -> bool {
        matches!(self, Shape::Document(_))
    }

    /// Returns true if this shape is an enum shape.
    pub fn is_enum(&self) -> bool {
        matches!(self, Shape::Enum(_))
    }

    /// Returns true if this shape is an integer enum shape.
    pub fn is_int_enum(&self) -> bool {
        matches!(self, Shape::IntEnum(_))
    }

    // Type checking methods for aggregate shapes

    /// Returns true if this shape is a list shape.
    pub fn is_list(&self) -> bool {
        matches!(self, Shape::List(_))
    }

    /// Returns true if this shape is a map shape.
    pub fn is_map(&self) -> bool {
        matches!(self, Shape::Map(_))
    }

    /// Returns true if this shape is a set shape.
    pub fn is_set(&self) -> bool {
        matches!(self, Shape::Set(_))
    }

    /// Returns true if this shape is a structure shape.
    pub fn is_structure(&self) -> bool {
        matches!(self, Shape::Structure(_))
    }

    /// Returns true if this shape is a union shape.
    pub fn is_union(&self) -> bool {
        matches!(self, Shape::Union(_))
    }

    // Type checking methods for service shapes

    /// Returns true if this shape is a service shape.
    pub fn is_service(&self) -> bool {
        matches!(self, Shape::Service(_))
    }

    /// Returns true if this shape is an operation shape.
    pub fn is_operation(&self) -> bool {
        matches!(self, Shape::Operation(_))
    }

    /// Returns true if this shape is a resource shape.
    pub fn is_resource(&self) -> bool {
        matches!(self, Shape::Resource(_))
    }

    /// Returns true if this shape is a member shape.
    pub fn is_member(&self) -> bool {
        matches!(self, Shape::Member(_))
    }

    // Type conversion methods for simple shapes

    /// Returns this shape as a boolean shape, if it is one.
    pub fn as_boolean(&self) -> Option<&BooleanShape> {
        match self {
            Shape::Boolean(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a byte shape, if it is one.
    pub fn as_byte(&self) -> Option<&ByteShape> {
        match self {
            Shape::Byte(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a short shape, if it is one.
    pub fn as_short(&self) -> Option<&ShortShape> {
        match self {
            Shape::Short(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as an integer shape, if it is one.
    pub fn as_integer(&self) -> Option<&IntegerShape> {
        match self {
            Shape::Integer(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a long shape, if it is one.
    pub fn as_long(&self) -> Option<&LongShape> {
        match self {
            Shape::Long(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a float shape, if it is one.
    pub fn as_float(&self) -> Option<&FloatShape> {
        match self {
            Shape::Float(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a double shape, if it is one.
    pub fn as_double(&self) -> Option<&DoubleShape> {
        match self {
            Shape::Double(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a big integer shape, if it is one.
    pub fn as_big_integer(&self) -> Option<&BigIntegerShape> {
        match self {
            Shape::BigInteger(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a big decimal shape, if it is one.
    pub fn as_big_decimal(&self) -> Option<&BigDecimalShape> {
        match self {
            Shape::BigDecimal(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a string shape, if it is one.
    pub fn as_string(&self) -> Option<&StringShape> {
        match self {
            Shape::String(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a blob shape, if it is one.
    pub fn as_blob(&self) -> Option<&BlobShape> {
        match self {
            Shape::Blob(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a timestamp shape, if it is one.
    pub fn as_timestamp(&self) -> Option<&TimestampShape> {
        match self {
            Shape::Timestamp(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a document shape, if it is one.
    pub fn as_document(&self) -> Option<&DocumentShape> {
        match self {
            Shape::Document(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as an enum shape, if it is one.
    pub fn as_enum(&self) -> Option<&EnumShape> {
        match self {
            Shape::Enum(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as an integer enum shape, if it is one.
    pub fn as_int_enum(&self) -> Option<&IntEnumShape> {
        match self {
            Shape::IntEnum(shape) => Some(shape),
            _ => None,
        }
    }

    // Type conversion methods for aggregate shapes

    /// Returns this shape as a list shape, if it is one.
    pub fn as_list(&self) -> Option<&ListShape> {
        match self {
            Shape::List(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a map shape, if it is one.
    pub fn as_map(&self) -> Option<&MapShape> {
        match self {
            Shape::Map(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a set shape, if it is one.
    pub fn as_set(&self) -> Option<&SetShape> {
        match self {
            Shape::Set(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a structure shape, if it is one.
    pub fn as_structure(&self) -> Option<&StructureShape> {
        match self {
            Shape::Structure(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a union shape, if it is one.
    pub fn as_union(&self) -> Option<&UnionShape> {
        match self {
            Shape::Union(shape) => Some(shape),
            _ => None,
        }
    }

    // Type conversion methods for service shapes

    /// Returns this shape as a service shape, if it is one.
    pub fn as_service(&self) -> Option<&ServiceShape> {
        match self {
            Shape::Service(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as an operation shape, if it is one.
    pub fn as_operation(&self) -> Option<&OperationShape> {
        match self {
            Shape::Operation(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a resource shape, if it is one.
    pub fn as_resource(&self) -> Option<&ResourceShape> {
        match self {
            Shape::Resource(shape) => Some(shape),
            _ => None,
        }
    }

    /// Returns this shape as a member shape, if it is one.
    pub fn as_member(&self) -> Option<&MemberShape> {
        match self {
            Shape::Member(shape) => Some(shape),
            _ => None,
        }
    }

    // Expect methods for simple shapes

    /// Returns this shape as a boolean shape, panicking if it is not one.
    pub fn expect_boolean(&self) -> &BooleanShape {
        self.as_boolean()
            .unwrap_or_else(|| panic!("Expected boolean shape, but got {}", self.id()))
    }

    /// Returns this shape as a byte shape, panicking if it is not one.
    pub fn expect_byte(&self) -> &ByteShape {
        self.as_byte()
            .unwrap_or_else(|| panic!("Expected byte shape, but got {}", self.id()))
    }

    /// Returns this shape as a short shape, panicking if it is not one.
    pub fn expect_short(&self) -> &ShortShape {
        self.as_short()
            .unwrap_or_else(|| panic!("Expected short shape, but got {}", self.id()))
    }

    /// Returns this shape as an integer shape, panicking if it is not one.
    pub fn expect_integer(&self) -> &IntegerShape {
        self.as_integer()
            .unwrap_or_else(|| panic!("Expected integer shape, but got {}", self.id()))
    }

    /// Returns this shape as a long shape, panicking if it is not one.
    pub fn expect_long(&self) -> &LongShape {
        self.as_long()
            .unwrap_or_else(|| panic!("Expected long shape, but got {}", self.id()))
    }

    /// Returns this shape as a float shape, panicking if it is not one.
    pub fn expect_float(&self) -> &FloatShape {
        self.as_float()
            .unwrap_or_else(|| panic!("Expected float shape, but got {}", self.id()))
    }

    /// Returns this shape as a double shape, panicking if it is not one.
    pub fn expect_double(&self) -> &DoubleShape {
        self.as_double()
            .unwrap_or_else(|| panic!("Expected double shape, but got {}", self.id()))
    }

    /// Returns this shape as a big integer shape, panicking if it is not one.
    pub fn expect_big_integer(&self) -> &BigIntegerShape {
        self.as_big_integer()
            .unwrap_or_else(|| panic!("Expected big integer shape, but got {}", self.id()))
    }

    /// Returns this shape as a big decimal shape, panicking if it is not one.
    pub fn expect_big_decimal(&self) -> &BigDecimalShape {
        self.as_big_decimal()
            .unwrap_or_else(|| panic!("Expected big decimal shape, but got {}", self.id()))
    }

    /// Returns this shape as a string shape, panicking if it is not one.
    pub fn expect_string(&self) -> &StringShape {
        self.as_string()
            .unwrap_or_else(|| panic!("Expected string shape, but got {}", self.id()))
    }

    /// Returns this shape as a blob shape, panicking if it is not one.
    pub fn expect_blob(&self) -> &BlobShape {
        self.as_blob()
            .unwrap_or_else(|| panic!("Expected blob shape, but got {}", self.id()))
    }

    /// Returns this shape as a timestamp shape, panicking if it is not one.
    pub fn expect_timestamp(&self) -> &TimestampShape {
        self.as_timestamp()
            .unwrap_or_else(|| panic!("Expected timestamp shape, but got {}", self.id()))
    }

    /// Returns this shape as a document shape, panicking if it is not one.
    pub fn expect_document(&self) -> &DocumentShape {
        self.as_document()
            .unwrap_or_else(|| panic!("Expected document shape, but got {}", self.id()))
    }

    /// Returns this shape as an enum shape, panicking if it is not one.
    pub fn expect_enum(&self) -> &EnumShape {
        self.as_enum()
            .unwrap_or_else(|| panic!("Expected enum shape, but got {}", self.id()))
    }

    /// Returns this shape as an integer enum shape, panicking if it is not one.
    pub fn expect_int_enum(&self) -> &IntEnumShape {
        self.as_int_enum()
            .unwrap_or_else(|| panic!("Expected integer enum shape, but got {}", self.id()))
    }

    // Expect methods for aggregate shapes

    /// Returns this shape as a list shape, panicking if it is not one.
    pub fn expect_list(&self) -> &ListShape {
        self.as_list()
            .unwrap_or_else(|| panic!("Expected list shape, but got {}", self.id()))
    }

    /// Returns this shape as a map shape, panicking if it is not one.
    pub fn expect_map(&self) -> &MapShape {
        self.as_map()
            .unwrap_or_else(|| panic!("Expected map shape, but got {}", self.id()))
    }

    /// Returns this shape as a set shape, panicking if it is not one.
    pub fn expect_set(&self) -> &SetShape {
        self.as_set()
            .unwrap_or_else(|| panic!("Expected set shape, but got {}", self.id()))
    }

    /// Returns this shape as a structure shape, panicking if it is not one.
    pub fn expect_structure(&self) -> &StructureShape {
        self.as_structure()
            .unwrap_or_else(|| panic!("Expected structure shape, but got {}", self.id()))
    }

    /// Returns this shape as a union shape, panicking if it is not one.
    pub fn expect_union(&self) -> &UnionShape {
        self.as_union()
            .unwrap_or_else(|| panic!("Expected union shape, but got {}", self.id()))
    }

    // Expect methods for service shapes

    /// Returns this shape as a service shape, panicking if it is not one.
    pub fn expect_service(&self) -> &ServiceShape {
        self.as_service()
            .unwrap_or_else(|| panic!("Expected service shape, but got {}", self.id()))
    }

    /// Returns this shape as an operation shape, panicking if it is not one.
    pub fn expect_operation(&self) -> &OperationShape {
        self.as_operation()
            .unwrap_or_else(|| panic!("Expected operation shape, but got {}", self.id()))
    }

    /// Returns this shape as a resource shape, panicking if it is not one.
    pub fn expect_resource(&self) -> &ResourceShape {
        self.as_resource()
            .unwrap_or_else(|| panic!("Expected resource shape, but got {}", self.id()))
    }

    /// Returns this shape as a member shape, panicking if it is not one.
    pub fn expect_member(&self) -> &MemberShape {
        self.as_member()
            .unwrap_or_else(|| panic!("Expected member shape, but got {}", self.id()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape_id::ShapeId;
    use std::collections::HashMap;

    #[test]
    fn test_is_methods() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let traits = HashMap::new();
        let string_shape = StringShape::new(id.clone(), traits);
        let shape: Shape = string_shape.into();

        assert!(shape.is_string());
        assert!(!shape.is_integer());
        assert!(!shape.is_structure());
    }

    #[test]
    fn test_as_methods() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let traits = HashMap::new();
        let string_shape = StringShape::new(id.clone(), traits);
        let shape: Shape = string_shape.into();

        assert!(shape.as_string().is_some());
        assert!(shape.as_integer().is_none());
        assert!(shape.as_structure().is_none());
    }

    #[test]
    fn test_expect_methods() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let traits = HashMap::new();
        let string_shape = StringShape::new(id.clone(), traits);
        let shape: Shape = string_shape.into();

        let _ = shape.expect_string(); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Expected integer shape")]
    fn test_expect_methods_panic() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let traits = HashMap::new();
        let string_shape = StringShape::new(id.clone(), traits);
        let shape: Shape = string_shape.into();

        let _ = shape.expect_integer(); // Should panic
    }

    #[test]
    fn test_structure_shape_methods() {
        let id = ShapeId::new("com.example", "Person").unwrap();
        let traits = HashMap::new();
        let members = HashMap::new();
        let structure_shape = StructureShape::new(id.clone(), traits, members);
        let shape: Shape = structure_shape.into();

        assert!(shape.is_structure());
        assert!(shape.as_structure().is_some());
        let _ = shape.expect_structure(); // Should not panic
    }
}
