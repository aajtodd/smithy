/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Type checking methods for Smithy shapes.

use crate::shape::member::MemberShape;
use crate::shape::operation::OperationShape;
use crate::shape::resource::ResourceShape;
use crate::shape::{
    aggregate::{ListShape, MapShape, SetShape, StructureShape, UnionShape},
    service::ServiceShape,
    simple::{
        BigDecimalShape, BigIntegerShape, BlobShape, BooleanShape, ByteShape, DoubleShape,
        FloatShape, IntegerShape, LongShape, ShortShape, StringShape, TimestampShape,
    },
    DocumentShape, EnumShape, IntEnumShape, Shape,
};

impl Shape {
    /// Check if this shape is a boolean shape.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Shape::Boolean(_))
    }

    /// Check if this shape is a byte shape.
    pub fn is_byte(&self) -> bool {
        matches!(self, Shape::Byte(_))
    }

    /// Check if this shape is a short shape.
    pub fn is_short(&self) -> bool {
        matches!(self, Shape::Short(_))
    }

    /// Check if this shape is an integer shape.
    pub fn is_integer(&self) -> bool {
        matches!(self, Shape::Integer(_))
    }

    /// Check if this shape is a long shape.
    pub fn is_long(&self) -> bool {
        matches!(self, Shape::Long(_))
    }

    /// Check if this shape is a float shape.
    pub fn is_float(&self) -> bool {
        matches!(self, Shape::Float(_))
    }

    /// Check if this shape is a double shape.
    pub fn is_double(&self) -> bool {
        matches!(self, Shape::Double(_))
    }

    /// Check if this shape is a big integer shape.
    pub fn is_big_integer(&self) -> bool {
        matches!(self, Shape::BigInteger(_))
    }

    /// Check if this shape is a big decimal shape.
    pub fn is_big_decimal(&self) -> bool {
        matches!(self, Shape::BigDecimal(_))
    }

    /// Check if this shape is a string shape.
    pub fn is_string(&self) -> bool {
        matches!(self, Shape::String(_))
    }

    /// Check if this shape is a blob shape.
    pub fn is_blob(&self) -> bool {
        matches!(self, Shape::Blob(_))
    }

    /// Check if this shape is a timestamp shape.
    pub fn is_timestamp(&self) -> bool {
        matches!(self, Shape::Timestamp(_))
    }

    /// Check if this shape is a list shape.
    pub fn is_list(&self) -> bool {
        matches!(self, Shape::List(_))
    }

    /// Check if this shape is a map shape.
    pub fn is_map(&self) -> bool {
        matches!(self, Shape::Map(_))
    }

    /// Check if this shape is a set shape.
    pub fn is_set(&self) -> bool {
        matches!(self, Shape::Set(_))
    }

    /// Check if this shape is a structure shape.
    pub fn is_structure(&self) -> bool {
        matches!(self, Shape::Structure(_))
    }

    /// Check if this shape is a union shape.
    pub fn is_union(&self) -> bool {
        matches!(self, Shape::Union(_))
    }

    /// Check if this shape is a service shape.
    pub fn is_service(&self) -> bool {
        matches!(self, Shape::Service(_))
    }

    /// Check if this shape is an operation shape.
    pub fn is_operation(&self) -> bool {
        matches!(self, Shape::Operation(_))
    }

    /// Check if this shape is a resource shape.
    pub fn is_resource(&self) -> bool {
        matches!(self, Shape::Resource(_))
    }

    /// Check if this shape is a member shape.
    pub fn is_member(&self) -> bool {
        matches!(self, Shape::Member(_))
    }

    /// Get this shape as a boolean shape, if it is one.
    pub fn as_boolean(&self) -> Option<&BooleanShape> {
        match self {
            Shape::Boolean(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a byte shape, if it is one.
    pub fn as_byte(&self) -> Option<&ByteShape> {
        match self {
            Shape::Byte(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a short shape, if it is one.
    pub fn as_short(&self) -> Option<&ShortShape> {
        match self {
            Shape::Short(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as an integer shape, if it is one.
    pub fn as_integer(&self) -> Option<&IntegerShape> {
        match self {
            Shape::Integer(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a long shape, if it is one.
    pub fn as_long(&self) -> Option<&LongShape> {
        match self {
            Shape::Long(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a float shape, if it is one.
    pub fn as_float(&self) -> Option<&FloatShape> {
        match self {
            Shape::Float(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a double shape, if it is one.
    pub fn as_double(&self) -> Option<&DoubleShape> {
        match self {
            Shape::Double(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a big integer shape, if it is one.
    pub fn as_big_integer(&self) -> Option<&BigIntegerShape> {
        match self {
            Shape::BigInteger(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a big decimal shape, if it is one.
    pub fn as_big_decimal(&self) -> Option<&BigDecimalShape> {
        match self {
            Shape::BigDecimal(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a string shape, if it is one.
    pub fn as_string(&self) -> Option<&StringShape> {
        match self {
            Shape::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a blob shape, if it is one.
    pub fn as_blob(&self) -> Option<&BlobShape> {
        match self {
            Shape::Blob(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a timestamp shape, if it is one.
    pub fn as_timestamp(&self) -> Option<&TimestampShape> {
        match self {
            Shape::Timestamp(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as an enum shape, if it is one
    pub fn as_enum(&self) -> Option<&EnumShape> {
        match self {
            Shape::Enum(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as an intEnum shape, if it is one
    pub fn as_int_enum(&self) -> Option<&IntEnumShape> {
        match self {
            Shape::IntEnum(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a document shape, if it is one
    pub fn as_document(&self) -> Option<&DocumentShape> {
        match self {
            Shape::Document(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a list shape, if it is one.
    pub fn as_list(&self) -> Option<&ListShape> {
        match self {
            Shape::List(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a map shape, if it is one.
    pub fn as_map(&self) -> Option<&MapShape> {
        match self {
            Shape::Map(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a set shape, if it is one.
    pub fn as_set(&self) -> Option<&SetShape> {
        match self {
            Shape::Set(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a structure shape, if it is one.
    pub fn as_structure(&self) -> Option<&StructureShape> {
        match self {
            Shape::Structure(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a union shape, if it is one.
    pub fn as_union(&self) -> Option<&UnionShape> {
        match self {
            Shape::Union(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a service shape, if it is one.
    pub fn as_service(&self) -> Option<&ServiceShape> {
        match self {
            Shape::Service(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as an operation shape, if it is one.
    pub fn as_operation(&self) -> Option<&OperationShape> {
        match self {
            Shape::Operation(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a resource shape, if it is one.
    pub fn as_resource(&self) -> Option<&ResourceShape> {
        match self {
            Shape::Resource(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a member shape, if it is one.
    pub fn as_member(&self) -> Option<&MemberShape> {
        match self {
            Shape::Member(s) => Some(s),
            _ => None,
        }
    }

    /// Get this shape as a boolean shape, panicking if it is not one.
    pub fn expect_boolean(&self) -> &BooleanShape {
        self.as_boolean()
            .expect("Expected a boolean shape, but got a different shape type")
    }

    /// Get this shape as a byte shape, panicking if it is not one.
    pub fn expect_byte(&self) -> &ByteShape {
        self.as_byte()
            .expect("Expected a byte shape, but got a different shape type")
    }

    /// Get this shape as a short shape, panicking if it is not one.
    pub fn expect_short(&self) -> &ShortShape {
        self.as_short()
            .expect("Expected a short shape, but got a different shape type")
    }

    /// Get this shape as an integer shape, panicking if it is not one.
    pub fn expect_integer(&self) -> &IntegerShape {
        self.as_integer()
            .expect("Expected an integer shape, but got a different shape type")
    }

    /// Get this shape as a long shape, panicking if it is not one.
    pub fn expect_long(&self) -> &LongShape {
        self.as_long()
            .expect("Expected a long shape, but got a different shape type")
    }

    /// Get this shape as a float shape, panicking if it is not one.
    pub fn expect_float(&self) -> &FloatShape {
        self.as_float()
            .expect("Expected a float shape, but got a different shape type")
    }

    /// Get this shape as a double shape, panicking if it is not one.
    pub fn expect_double(&self) -> &DoubleShape {
        self.as_double()
            .expect("Expected a double shape, but got a different shape type")
    }

    /// Get this shape as a big integer shape, panicking if it is not one.
    pub fn expect_big_integer(&self) -> &BigIntegerShape {
        self.as_big_integer()
            .expect("Expected a big integer shape, but got a different shape type")
    }

    /// Get this shape as a big decimal shape, panicking if it is not one.
    pub fn expect_big_decimal(&self) -> &BigDecimalShape {
        self.as_big_decimal()
            .expect("Expected a big decimal shape, but got a different shape type")
    }

    /// Get this shape as a string shape, panicking if it is not one.
    pub fn expect_string(&self) -> &StringShape {
        self.as_string()
            .expect("Expected a string shape, but got a different shape type")
    }

    /// Get this shape as a blob shape, panicking if it is not one.
    pub fn expect_blob(&self) -> &BlobShape {
        self.as_blob()
            .expect("Expected a blob shape, but got a different shape type")
    }

    /// Get this shape as a timestamp shape, panicking if it is not one.
    pub fn expect_timestamp(&self) -> &TimestampShape {
        self.as_timestamp()
            .expect("Expected a timestamp shape, but got a different shape type")
    }

    /// Get this shape as an enum shape, panicking if it is not one.
    pub fn expect_enum(&self) -> &EnumShape {
        self.as_enum()
            .expect("Expected an enum shape, but got a different shape type")
    }

    /// Get this shape as an intEnum shape, panicking if it is not one.
    pub fn expect_int_enum(&self) -> &IntEnumShape {
        self.as_int_enum()
            .expect("Expected an intEnum shape, but got a different shape type")
    }

    /// Get this shape as a document shape, panicking if it is not one.
    pub fn expect_document(&self) -> &DocumentShape {
        self.as_document()
            .expect("Expected a document shape, but got a different shape type")
    }

    /// Get this shape as a list shape, panicking if it is not one.
    pub fn expect_list(&self) -> &ListShape {
        self.as_list()
            .expect("Expected a list shape, but got a different shape type")
    }

    /// Get this shape as a map shape, panicking if it is not one.
    pub fn expect_map(&self) -> &MapShape {
        self.as_map()
            .expect("Expected a map shape, but got a different shape type")
    }

    /// Get this shape as a set shape, panicking if it is not one.
    pub fn expect_set(&self) -> &SetShape {
        self.as_set()
            .expect("Expected a set shape, but got a different shape type")
    }

    /// Get this shape as a structure shape, panicking if it is not one.
    pub fn expect_structure(&self) -> &StructureShape {
        self.as_structure()
            .expect("Expected a structure shape, but got a different shape type")
    }

    /// Get this shape as a union shape, panicking if it is not one.
    pub fn expect_union(&self) -> &UnionShape {
        self.as_union()
            .expect("Expected a union shape, but got a different shape type")
    }

    /// Get this shape as a service shape, panicking if it is not one.
    pub fn expect_service(&self) -> &ServiceShape {
        self.as_service()
            .expect("Expected a service shape, but got a different shape type")
    }

    /// Get this shape as an operation shape, panicking if it is not one.
    pub fn expect_operation(&self) -> &OperationShape {
        self.as_operation()
            .expect("Expected an operation shape, but got a different shape type")
    }

    /// Get this shape as a resource shape, panicking if it is not one.
    pub fn expect_resource(&self) -> &ResourceShape {
        self.as_resource()
            .expect("Expected a resource shape, but got a different shape type")
    }

    /// Get this shape as a member shape, panicking if it is not one.
    pub fn expect_member(&self) -> &MemberShape {
        self.as_member()
            .expect("Expected a member shape, but got a different shape type")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_methods() {
        let string_shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = string_shape.into();

        assert!(!shape.is_boolean());
        assert!(!shape.is_byte());
        assert!(!shape.is_short());
        assert!(!shape.is_integer());
        assert!(!shape.is_long());
        assert!(!shape.is_float());
        assert!(!shape.is_double());
        assert!(!shape.is_big_integer());
        assert!(!shape.is_big_decimal());
        assert!(shape.is_string());
        assert!(!shape.is_blob());
        assert!(!shape.is_timestamp());
        assert!(!shape.is_list());
        assert!(!shape.is_map());
        assert!(!shape.is_set());
        assert!(!shape.is_structure());
        assert!(!shape.is_union());
        assert!(!shape.is_service());
        assert!(!shape.is_operation());
        assert!(!shape.is_resource());
        assert!(!shape.is_member());
    }

    #[test]
    fn test_as_methods() {
        let string_shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = string_shape.into();

        assert!(shape.as_boolean().is_none());
        assert!(shape.as_byte().is_none());
        assert!(shape.as_short().is_none());
        assert!(shape.as_integer().is_none());
        assert!(shape.as_long().is_none());
        assert!(shape.as_float().is_none());
        assert!(shape.as_double().is_none());
        assert!(shape.as_big_integer().is_none());
        assert!(shape.as_big_decimal().is_none());
        assert!(shape.as_string().is_some());
        assert!(shape.as_blob().is_none());
        assert!(shape.as_timestamp().is_none());
        assert!(shape.as_list().is_none());
        assert!(shape.as_map().is_none());
        assert!(shape.as_set().is_none());
        assert!(shape.as_structure().is_none());
        assert!(shape.as_union().is_none());
        assert!(shape.as_service().is_none());
        assert!(shape.as_operation().is_none());
        assert!(shape.as_resource().is_none());
        assert!(shape.as_member().is_none());
    }

    #[test]
    fn test_expect_methods() {
        let string_shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = string_shape.into();

        let _ = shape.expect_string();
    }

    #[test]
    #[should_panic]
    fn test_expect_methods_panic() {
        let string_shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = string_shape.into();

        let _ = shape.expect_boolean();
    }

    #[test]
    fn test_structure_shape_methods() {
        let structure_shape = StructureShape::builder()
            .id("example.foo#MyStruct")
            .build()
            .unwrap();
        let shape: Shape = structure_shape.into();

        assert!(shape.is_structure());
        assert!(shape.as_structure().is_some());
        let _ = shape.expect_structure();
    }
}
