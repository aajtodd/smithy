/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Shape types for the Smithy model.

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::shape_id::ShapeId;
use crate::traits::Trait;

// TODO - define API for shape builder(s)

/// A Smithy shape.
///
/// Shapes are the fundamental building blocks of a Smithy model. Each shape has a unique ID,
/// a set of traits, and a specific kind that determines its behavior and properties.
#[derive(Debug, Clone)]
pub struct Shape {
    /// The shape ID.
    id: ShapeId,
    /// All traits applied to this shape (keyed by trait ShapeId).
    traits: HashMap<ShapeId, Trait>,
    /// The specific kind of shape.
    kind: ShapeKind,
}

/// The specific kind of shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeKind {
    // Simple types
    /// A [boolean](https://smithy.io/2.0/spec/simple-types.html#boolean) shape
    Boolean,
    /// A [byte](https://smithy.io/2.0/spec/simple-types.html#byte) shape
    Byte,
    /// A [short](https://smithy.io/2.0/spec/simple-types.html#short) shape
    Short,
    /// An [integer](https://smithy.io/2.0/spec/simple-types.html#integer) shape
    Integer,
    /// A [long](https://smithy.io/2.0/spec/simple-types.html#long) shape
    Long,
    /// A [float](https://smithy.io/2.0/spec/simple-types.html#float) shape
    Float,
    /// A [double](https://smithy.io/2.0/spec/simple-types.html#double) shape
    Double,
    /// A [bigInteger](https://smithy.io/2.0/spec/simple-types.html#biginteger) shape
    BigInteger,
    /// A [bigDecimal](https://smithy.io/2.0/spec/simple-types.html#bigdecimal) shape
    BigDecimal,
    /// A [string](https://smithy.io/2.0/spec/simple-types.html#string) shape
    String,
    /// A [blob](https://smithy.io/2.0/spec/simple-types.html#blob) shape
    Blob,
    /// A [timestamp](https://smithy.io/2.0/spec/simple-types.html#timestamp) shape
    Timestamp,
    /// A [document](https://smithy.io/2.0/spec/simple-types.html#document) shape
    Document,
    /// An [enum](https://smithy.io/2.0/spec/simple-types.html#enum) shape
    Enum(EnumShape),
    /// An [intEnum](https://smithy.io/2.0/spec/simple-types.html#intenum) shape
    IntEnum(IntEnumShape),

    // Aggregate types
    /// A [list](https://smithy.io/2.0/spec/aggregate-types.html#list) shape
    List(ListShape),
    /// A [map](https://smithy.io/2.0/spec/aggregate-types.html#map) shape
    Map(MapShape),
    /// A [set](https://smithy.io/2.0/spec/aggregate-types.html#set) shape
    Set(SetShape),
    /// A [structure](https://smithy.io/2.0/spec/aggregate-types.html#structure) shape
    Structure(StructureShape),
    /// A [union](https://smithy.io/2.0/spec/aggregate-types.html#union) shape
    Union(UnionShape),

    // Service types
    /// A [service](https://smithy.io/2.0/spec/service-types.html#service) shape
    Service(ServiceShape),
    /// An [operation](https://smithy.io/2.0/spec/service-types.html#operation) shape
    Operation(OperationShape),
    /// A [resource](https://smithy.io/2.0/spec/service-types.html#resource) shape
    Resource(ResourceShape),

    // Special types
    /// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
    Member(MemberShape),
}

/// A member shape, which represents a field in a structure, union, or other aggregate shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberShape {
    /// The name of the member.
    pub member_name: String,
    /// The target shape that this member references.
    pub target: ShapeId,
}

/// A list shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListShape {
    /// The member shape that defines the type of elements in the list.
    pub member: MemberShape,
}

/// A map shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapShape {
    /// The member shape that defines the type of keys in the map.
    pub key: MemberShape,
    /// The member shape that defines the type of values in the map.
    pub value: MemberShape,
}

/// A set shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetShape {
    /// The member shape that defines the type of elements in the set.
    pub member: MemberShape,
}

/// A structure shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureShape {
    /// The members of the structure, keyed by member name.
    pub members: HashMap<String, MemberShape>,
}

/// A union shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionShape {
    /// The members of the union, keyed by member name.
    pub members: HashMap<String, MemberShape>,
}

/// A service shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceShape {
    /// The version of the service.
    pub version: String,
    /// The operations provided by the service.
    pub operations: Vec<ShapeId>,
    /// The resources provided by the service.
    pub resources: Vec<ShapeId>,
}

/// An operation shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationShape {
    /// The input shape for the operation.
    pub input: Option<ShapeId>,
    /// The output shape for the operation.
    pub output: Option<ShapeId>,
    /// The errors that can be thrown by the operation.
    pub errors: Vec<ShapeId>,
}

/// A resource shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceShape {
    /// The identifiers for the resource.
    pub identifiers: HashMap<String, ShapeId>,
    /// The create operation for the resource.
    pub create: Option<ShapeId>,
    /// The read operation for the resource.
    pub read: Option<ShapeId>,
    /// The update operation for the resource.
    pub update: Option<ShapeId>,
    /// The delete operation for the resource.
    pub delete: Option<ShapeId>,
    /// The list operation for the resource.
    pub list: Option<ShapeId>,
    /// The operations provided by the resource.
    pub operations: Vec<ShapeId>,
    /// The resources provided by the resource.
    pub resources: Vec<ShapeId>,
}

// FIXME - enum and intEnum shapes may need to retain the original order of members

/// An enum shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumShape {
    /// The members of the enum, keyed by member name.
    pub members: HashMap<String, EnumMember>,
}

/// An enum member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMember {
    /// The name of the enum member.
    pub name: String,
    /// The value of the enum member.
    pub value: String,
}

/// An integer enum shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntEnumShape {
    /// The members of the integer enum, keyed by member name.
    pub members: HashMap<String, IntEnumMember>,
}

/// An integer enum member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntEnumMember {
    /// The name of the integer enum member.
    pub name: String,
    /// The value of the integer enum member.
    pub value: i64,
}

/// A view of a list shape that provides access to both common shape fields and list-specific fields.
pub struct ListShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The list-specific fields.
    pub list: &'a ListShape,
}

/// A view of a map shape that provides access to both common shape fields and map-specific fields.
pub struct MapShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The map-specific fields.
    pub map: &'a MapShape,
}

/// A view of a set shape that provides access to both common shape fields and set-specific fields.
pub struct SetShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The set-specific fields.
    pub set: &'a SetShape,
}

/// A view of a structure shape that provides access to both common shape fields and structure-specific fields.
pub struct StructureShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The structure-specific fields.
    pub structure: &'a StructureShape,
}

/// A view of a union shape that provides access to both common shape fields and union-specific fields.
pub struct UnionShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The union-specific fields.
    pub union: &'a UnionShape,
}

/// A view of an enum shape that provides access to both common shape fields and enum-specific fields.
pub struct EnumShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The enum-specific fields.
    pub enum_shape: &'a EnumShape,
}

/// A view of an integer enum shape that provides access to both common shape fields and integer enum-specific fields.
pub struct IntEnumShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The integer enum-specific fields.
    pub int_enum: &'a IntEnumShape,
}

/// A view of a service shape that provides access to both common shape fields and service-specific fields.
pub struct ServiceShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The service-specific fields.
    pub service: &'a ServiceShape,
}

/// A view of an operation shape that provides access to both common shape fields and operation-specific fields.
pub struct OperationShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The operation-specific fields.
    pub operation: &'a OperationShape,
}

/// A view of a resource shape that provides access to both common shape fields and resource-specific fields.
pub struct ResourceShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The resource-specific fields.
    pub resource: &'a ResourceShape,
}

/// A view of a member shape that provides access to both common shape fields and member-specific fields.
pub struct MemberShapeView<'a> {
    /// The parent shape.
    pub shape: &'a Shape,
    /// The member-specific fields.
    pub member: &'a MemberShape,
}

impl Shape {
    /// Creates a new shape.
    ///
    /// # Arguments
    ///
    /// * `id` - The shape ID.
    /// * `kind` - The specific kind of shape.
    ///
    /// # Returns
    ///
    /// A new shape with the given ID and kind, and no traits.
    pub fn new(id: impl Into<ShapeId>, kind: ShapeKind) -> Self {
        Self {
            id: id.into(),
            traits: HashMap::new(),
            kind,
        }
    }

    /// Creates a new shape with the given traits
    pub fn new_with_traits(
        id: impl Into<ShapeId>,
        kind: ShapeKind,
        traits: HashMap<ShapeId, Trait>
    ) -> Self {
        Self { id: id.into(), traits, kind }
    }

    /// Returns the shape ID.
    pub fn id(&self) -> &ShapeId {
        &self.id
    }

    /// Returns all the shape's traits.
    pub fn traits(&self) -> &HashMap<ShapeId, Trait> {
        &self.traits
    }

    /// Check if this shape has a particular trait applied
    pub fn has_trait(&self, trait_id: &ShapeId) -> bool {
        self.traits.contains_key(trait_id)
    }

    /// Get a specific trait applied to this shape
    pub fn get_trait(&self, trait_id: &ShapeId) -> Option<&Trait> {
        self.traits.get(trait_id)
    }

    /// Returns the shape kind.
    pub fn kind(&self) -> &ShapeKind {
        &self.kind
    }
}

impl Shape {
    // Type checking methods

    /// Returns true if this is a simple shape.
    pub fn is_simple(&self) -> bool {
        matches!(
            self.kind,
            ShapeKind::Boolean
                | ShapeKind::Byte
                | ShapeKind::Short
                | ShapeKind::Integer
                | ShapeKind::Long
                | ShapeKind::Float
                | ShapeKind::Double
                | ShapeKind::BigInteger
                | ShapeKind::BigDecimal
                | ShapeKind::String
                | ShapeKind::Blob
                | ShapeKind::Timestamp
                | ShapeKind::Document
                | ShapeKind::Enum(_)
                | ShapeKind::IntEnum(_)
        )
    }

    /// Returns true if this is an aggregate shape.
    pub fn is_aggregate(&self) -> bool {
        matches!(
            self.kind,
            ShapeKind::List(_)
                | ShapeKind::Map(_)
                | ShapeKind::Set(_)
                | ShapeKind::Structure(_)
                | ShapeKind::Union(_)
        )
    }

    /// Returns true if this is a service shape.
    pub fn is_service(&self) -> bool {
        matches!(self.kind, ShapeKind::Service(_))
    }

    /// Returns true if this is an operation shape.
    pub fn is_operation(&self) -> bool {
        matches!(self.kind, ShapeKind::Operation(_))
    }

    /// Returns true if this is a resource shape.
    pub fn is_resource(&self) -> bool {
        matches!(self.kind, ShapeKind::Resource(_))
    }

    /// Returns true if this is a member shape.
    pub fn is_member(&self) -> bool {
        matches!(self.kind, ShapeKind::Member(_))
    }

    /// Returns true if this is a boolean shape.
    pub fn is_boolean(&self) -> bool {
        matches!(self.kind, ShapeKind::Boolean)
    }

    /// Returns true if this is a byte shape.
    pub fn is_byte(&self) -> bool {
        matches!(self.kind, ShapeKind::Byte)
    }

    /// Returns true if this is a short shape.
    pub fn is_short(&self) -> bool {
        matches!(self.kind, ShapeKind::Short)
    }

    /// Returns true if this is an integer shape.
    pub fn is_integer(&self) -> bool {
        matches!(self.kind, ShapeKind::Integer)
    }

    /// Returns true if this is a long shape.
    pub fn is_long(&self) -> bool {
        matches!(self.kind, ShapeKind::Long)
    }

    /// Returns true if this is a float shape.
    pub fn is_float(&self) -> bool {
        matches!(self.kind, ShapeKind::Float)
    }

    /// Returns true if this is a double shape.
    pub fn is_double(&self) -> bool {
        matches!(self.kind, ShapeKind::Double)
    }

    /// Returns true if this is a big integer shape.
    pub fn is_big_integer(&self) -> bool {
        matches!(self.kind, ShapeKind::BigInteger)
    }

    /// Returns true if this is a big decimal shape.
    pub fn is_big_decimal(&self) -> bool {
        matches!(self.kind, ShapeKind::BigDecimal)
    }

    /// Returns true if this is a string shape.
    pub fn is_string(&self) -> bool {
        matches!(self.kind, ShapeKind::String)
    }

    /// Returns true if this is a blob shape.
    pub fn is_blob(&self) -> bool {
        matches!(self.kind, ShapeKind::Blob)
    }

    /// Returns true if this is a timestamp shape.
    pub fn is_timestamp(&self) -> bool {
        matches!(self.kind, ShapeKind::Timestamp)
    }

    /// Returns true if this is a document shape.
    pub fn is_document(&self) -> bool {
        matches!(self.kind, ShapeKind::Document)
    }

    /// Returns true if this is an enum shape.
    pub fn is_enum(&self) -> bool {
        matches!(self.kind, ShapeKind::Enum(_))
    }

    /// Returns true if this is an integer enum shape.
    pub fn is_int_enum(&self) -> bool {
        matches!(self.kind, ShapeKind::IntEnum(_))
    }

    /// Returns true if this is a list shape.
    pub fn is_list(&self) -> bool {
        matches!(self.kind, ShapeKind::List(_))
    }

    /// Returns true if this is a map shape.
    pub fn is_map(&self) -> bool {
        matches!(self.kind, ShapeKind::Map(_))
    }

    /// Returns true if this is a set shape.
    pub fn is_set(&self) -> bool {
        matches!(self.kind, ShapeKind::Set(_))
    }

    /// Returns true if this is a structure shape.
    pub fn is_structure(&self) -> bool {
        matches!(self.kind, ShapeKind::Structure(_))
    }

    /// Returns true if this is a union shape.
    pub fn is_union(&self) -> bool {
        matches!(self.kind, ShapeKind::Union(_))
    }
}

impl Shape {
    // Type conversion methods

    /// Returns this shape as a list shape view, if it is one.
    pub fn as_list(&self) -> Option<ListShapeView<'_>> {
        match &self.kind {
            ShapeKind::List(list) => Some(ListShapeView { shape: self, list }),
            _ => None,
        }
    }

    /// Returns this shape as a map shape view, if it is one.
    pub fn as_map(&self) -> Option<MapShapeView<'_>> {
        match &self.kind {
            ShapeKind::Map(map) => Some(MapShapeView { shape: self, map }),
            _ => None,
        }
    }

    /// Returns this shape as a set shape view, if it is one.
    pub fn as_set(&self) -> Option<SetShapeView<'_>> {
        match &self.kind {
            ShapeKind::Set(set) => Some(SetShapeView { shape: self, set }),
            _ => None,
        }
    }

    /// Returns this shape as a structure shape view, if it is one.
    pub fn as_structure(&self) -> Option<StructureShapeView<'_>> {
        match &self.kind {
            ShapeKind::Structure(structure) => Some(StructureShapeView {
                shape: self,
                structure,
            }),
            _ => None,
        }
    }

    /// Returns this shape as a union shape view, if it is one.
    pub fn as_union(&self) -> Option<UnionShapeView<'_>> {
        match &self.kind {
            ShapeKind::Union(union) => Some(UnionShapeView { shape: self, union }),
            _ => None,
        }
    }

    /// Returns this shape as an enum shape view, if it is one.
    pub fn as_enum(&self) -> Option<EnumShapeView<'_>> {
        match &self.kind {
            ShapeKind::Enum(enum_shape) => Some(EnumShapeView { 
                shape: self, 
                enum_shape 
            }),
            _ => None,
        }
    }

    /// Returns this shape as an integer enum shape view, if it is one.
    pub fn as_int_enum(&self) -> Option<IntEnumShapeView<'_>> {
        match &self.kind {
            ShapeKind::IntEnum(int_enum) => Some(IntEnumShapeView { 
                shape: self, 
                int_enum 
            }),
            _ => None,
        }
    }

    /// Returns this shape as a service shape view, if it is one.
    pub fn as_service(&self) -> Option<ServiceShapeView<'_>> {
        match &self.kind {
            ShapeKind::Service(service) => Some(ServiceShapeView {
                shape: self,
                service,
            }),
            _ => None,
        }
    }

    /// Returns this shape as an operation shape view, if it is one.
    pub fn as_operation(&self) -> Option<OperationShapeView<'_>> {
        match &self.kind {
            ShapeKind::Operation(operation) => Some(OperationShapeView {
                shape: self,
                operation,
            }),
            _ => None,
        }
    }

    /// Returns this shape as a resource shape view, if it is one.
    pub fn as_resource(&self) -> Option<ResourceShapeView<'_>> {
        match &self.kind {
            ShapeKind::Resource(resource) => Some(ResourceShapeView {
                shape: self,
                resource,
            }),
            _ => None,
        }
    }

    /// Returns this shape as a member shape view, if it is one.
    pub fn as_member(&self) -> Option<MemberShapeView<'_>> {
        match &self.kind {
            ShapeKind::Member(member) => Some(MemberShapeView {
                shape: self,
                member,
            }),
            _ => None,
        }
    }
}
impl Shape {
    // Expect methods that panic on type mismatch

    /// Returns this shape as a list shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a list shape.
    pub fn expect_list(&self) -> ListShapeView<'_> {
        self.as_list().expect(&format!(
            "Expected shape {} to be a List, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a map shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a map shape.
    pub fn expect_map(&self) -> MapShapeView<'_> {
        self.as_map().expect(&format!(
            "Expected shape {} to be a Map, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a set shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a set shape.
    pub fn expect_set(&self) -> SetShapeView<'_> {
        self.as_set().expect(&format!(
            "Expected shape {} to be a Set, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a structure shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a structure shape.
    pub fn expect_structure(&self) -> StructureShapeView<'_> {
        self.as_structure().expect(&format!(
            "Expected shape {} to be a Structure, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a union shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a union shape.
    pub fn expect_union(&self) -> UnionShapeView<'_> {
        self.as_union().expect(&format!(
            "Expected shape {} to be a Union, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a service shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a service shape.
    pub fn expect_service(&self) -> ServiceShapeView<'_> {
        self.as_service().expect(&format!(
            "Expected shape {} to be a Service, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as an operation shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not an operation shape.
    pub fn expect_operation(&self) -> OperationShapeView<'_> {
        self.as_operation().expect(&format!(
            "Expected shape {} to be an Operation, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a resource shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a resource shape.
    pub fn expect_resource(&self) -> ResourceShapeView<'_> {
        self.as_resource().expect(&format!(
            "Expected shape {} to be a Resource, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as a member shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not a member shape.
    pub fn expect_member(&self) -> MemberShapeView<'_> {
        self.as_member().expect(&format!(
            "Expected shape {} to be a Member, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as an enum shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not an enum shape.
    pub fn expect_enum(&self) -> EnumShapeView<'_> {
        self.as_enum().expect(&format!(
            "Expected shape {} to be an Enum, but got {:?}",
            self.id, self.kind
        ))
    }

    /// Returns this shape as an integer enum shape view, or panics if it's not one.
    ///
    /// # Panics
    ///
    /// Panics if the shape is not an integer enum shape.
    pub fn expect_int_enum(&self) -> IntEnumShapeView<'_> {
        self.as_int_enum().expect(&format!(
            "Expected shape {} to be an IntEnum, but got {:?}",
            self.id, self.kind
        ))
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.traits == other.traits && std::mem::discriminant(&self.kind) == std::mem::discriminant(&other.kind)
    }
}

impl Eq for Shape {}

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash ID and shape type, matching Java implementation
        self.id.hash(state);
        std::mem::discriminant(&self.kind).hash(state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::node::Node;

    #[test]
    fn test_shape_creation() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let shape = Shape::new(id.clone(), ShapeKind::String);

        assert_eq!(shape.id(), &id);
        assert!(shape.traits().is_empty());
        assert!(matches!(shape.kind(), ShapeKind::String));
    }

    #[test]
    fn test_shape_with_traits() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let trait_id = ShapeId::new("smithy.api", "required").unwrap();
        let trait_value = Trait::new_with_value(trait_id.clone(), Node::Bool(true));

        let mut traits = HashMap::new();
        traits.insert(trait_id, trait_value);

        let shape = Shape::new_with_traits(id.clone(), ShapeKind::String, traits.clone());

        assert_eq!(shape.id(), &id);
        assert_eq!(shape.traits(), &traits);
        assert!(matches!(shape.kind(), ShapeKind::String));
    }

    #[test]
    fn test_shape_equality() {
        let id1 = ShapeId::new("com.example", "MyString").unwrap();
        let id2 = ShapeId::new("com.example", "OtherString").unwrap();

        let shape1 = Shape::new(id1.clone(), ShapeKind::String);
        let shape2 = Shape::new(id1.clone(), ShapeKind::String);
        let shape3 = Shape::new(id2.clone(), ShapeKind::String);
        let shape4 = Shape::new(id1.clone(), ShapeKind::Integer);

        assert_eq!(shape1, shape2);
        assert_ne!(shape1, shape3);
        assert_ne!(shape1, shape4);
    }

    #[test]
    fn test_shape_hash() {
        use std::collections::HashSet;

        let id1 = ShapeId::new("com.example", "MyString").unwrap();
        let id2 = ShapeId::new("com.example", "OtherString").unwrap();

        let shape1 = Shape::new(id1.clone(), ShapeKind::String);
        let shape2 = Shape::new(id1.clone(), ShapeKind::String);
        let shape3 = Shape::new(id2.clone(), ShapeKind::String);

        let mut set = HashSet::new();
        set.insert(shape1);

        assert!(set.contains(&shape2));
        assert!(!set.contains(&shape3));
    }

    #[test]
    fn test_list_shape() {
        let list_id = ShapeId::new("com.example", "MyList").unwrap();
        let target_id = ShapeId::new("com.example", "MyString").unwrap();

        let member = MemberShape {
            member_name: "member".to_string(),
            target: target_id,
        };

        let list_shape = ListShape { member };
        let shape = Shape::new(list_id, ShapeKind::List(list_shape));

        assert!(shape.is_list());
        assert!(!shape.is_map());
        assert!(!shape.is_structure());

        let list_view = shape.as_list().unwrap();
        assert_eq!(list_view.list.member.member_name, "member");
        assert_eq!(
            list_view.list.member.target.to_string(),
            "com.example#MyString"
        );

        // Test expect_list
        let list_view = shape.expect_list();
        assert_eq!(list_view.list.member.member_name, "member");
    }

    #[test]
    #[should_panic(expected = "Expected shape com.example#MyString to be a List")]
    fn test_expect_list_panics() {
        let id = ShapeId::new("com.example", "MyString").unwrap();
        let shape = Shape::new(id, ShapeKind::String);

        // This should panic
        let _ = shape.expect_list();
    }

    #[test]
    fn test_structure_shape() {
        let struct_id = ShapeId::new("com.example", "MyStruct").unwrap();
        let string_id = ShapeId::new("com.example", "MyString").unwrap();
        let int_id = ShapeId::new("com.example", "MyInt").unwrap();

        let name_member = MemberShape {
            member_name: "name".to_string(),
            target: string_id,
        };

        let age_member = MemberShape {
            member_name: "age".to_string(),
            target: int_id,
        };

        let mut members = HashMap::new();
        members.insert("name".to_string(), name_member);
        members.insert("age".to_string(), age_member);

        let struct_shape = StructureShape { members };
        let shape = Shape::new(struct_id, ShapeKind::Structure(struct_shape));

        assert!(shape.is_structure());
        assert!(!shape.is_list());

        let struct_view = shape.as_structure().unwrap();
        assert_eq!(struct_view.structure.members.len(), 2);
        assert!(struct_view.structure.members.contains_key("name"));
        assert!(struct_view.structure.members.contains_key("age"));

        // Test expect_structure
        let struct_view = shape.expect_structure();
        assert_eq!(struct_view.structure.members.len(), 2);
    }

    #[test]
    fn test_enum_shape() {
        let enum_id = ShapeId::new("com.example", "MyEnum").unwrap();
        
        let mut members = HashMap::new();
        members.insert("RED".to_string(), EnumMember {
            name: "RED".to_string(), 
            value: "red".to_string() 
        });
        members.insert("GREEN".to_string(), EnumMember { 
            name: "GREEN".to_string(), 
            value: "green".to_string() 
        });
        
        let enum_shape = EnumShape { members };
        let shape = Shape::new(enum_id, ShapeKind::Enum(enum_shape));
        
        assert!(shape.is_enum());
        assert!(shape.is_simple());
        assert!(!shape.is_aggregate());
        
        let enum_view = shape.as_enum().unwrap();
        assert_eq!(enum_view.enum_shape.members.len(), 2);
        assert!(enum_view.enum_shape.members.contains_key("RED"));
        assert_eq!(enum_view.enum_shape.members["RED"].value, "red");
    }
    
    #[test]
    fn test_int_enum_shape() {
        let enum_id = ShapeId::new("com.example", "MyIntEnum").unwrap();
        
        let mut members = HashMap::new();
        members.insert("ONE".to_string(), IntEnumMember { 
            name: "ONE".to_string(), 
            value: 1 
        });
        members.insert("TWO".to_string(), IntEnumMember { 
            name: "TWO".to_string(), 
            value: 2 
        });
        
        let int_enum_shape = IntEnumShape { members };
        let shape = Shape::new(enum_id, ShapeKind::IntEnum(int_enum_shape));
        
        assert!(shape.is_int_enum());
        assert!(shape.is_simple());
        assert!(!shape.is_aggregate());
        
        let int_enum_view = shape.as_int_enum().unwrap();
        assert_eq!(int_enum_view.int_enum.members.len(), 2);
        assert!(int_enum_view.int_enum.members.contains_key("ONE"));
        assert_eq!(int_enum_view.int_enum.members["ONE"].value, 1);
    }
}
