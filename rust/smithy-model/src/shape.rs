/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Shape types for the Smithy model.

use std::hash::{Hash, Hasher};

mod aggregate;
mod builder;
mod error;
mod member;
mod service;
mod simple;
mod type_checks;

use crate::shape_id::ShapeId;
use crate::traits::{BoxTrait, Trait, TraitMap};

pub use self::aggregate::*;
pub use self::builder::*;
pub use self::error::*;
pub use self::member::*;
pub use self::service::*;
pub use self::simple::*;

/// A Smithy shape.
///
/// Shapes are the fundamental building blocks of a Smithy model. Each shape has a unique ID,
/// a set of traits, and a specific kind that determines its behavior and properties.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    // Simple types
    /// A [boolean](https://smithy.io/2.0/spec/simple-types.html#boolean) shape
    Boolean(BooleanShape),
    /// A [byte](https://smithy.io/2.0/spec/simple-types.html#byte) shape
    Byte(ByteShape),
    /// A [short](https://smithy.io/2.0/spec/simple-types.html#short) shape
    Short(ShortShape),
    /// An [integer](https://smithy.io/2.0/spec/simple-types.html#integer) shape
    Integer(IntegerShape),
    /// A [long](https://smithy.io/2.0/spec/simple-types.html#long) shape
    Long(LongShape),
    /// A [float](https://smithy.io/2.0/spec/simple-types.html#float) shape
    Float(FloatShape),
    /// A [double](https://smithy.io/2.0/spec/simple-types.html#double) shape
    Double(DoubleShape),
    /// A [bigInteger](https://smithy.io/2.0/spec/simple-types.html#biginteger) shape
    BigInteger(BigIntegerShape),
    /// A [bigDecimal](https://smithy.io/2.0/spec/simple-types.html#bigdecimal) shape
    BigDecimal(BigDecimalShape),
    /// A [string](https://smithy.io/2.0/spec/simple-types.html#string) shape
    String(StringShape),
    /// A [blob](https://smithy.io/2.0/spec/simple-types.html#blob) shape
    Blob(BlobShape),
    /// A [timestamp](https://smithy.io/2.0/spec/simple-types.html#timestamp) shape
    Timestamp(TimestampShape),
    /// A [document](https://smithy.io/2.0/spec/simple-types.html#document) shape
    Document(DocumentShape),
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

    // Member shape
    /// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
    Member(MemberShape),
}

// Implement ProvideShapeMetadata for Shape by delegating to its variants
impl ProvideShapeMetadata for Shape {
    fn meta(&self) -> &ShapeMetadata {
        match self {
            Shape::Boolean(shape) => shape.meta(),
            Shape::Byte(shape) => shape.meta(),
            Shape::Short(shape) => shape.meta(),
            Shape::Integer(shape) => shape.meta(),
            Shape::Long(shape) => shape.meta(),
            Shape::Float(shape) => shape.meta(),
            Shape::Double(shape) => shape.meta(),
            Shape::BigInteger(shape) => shape.meta(),
            Shape::BigDecimal(shape) => shape.meta(),
            Shape::String(shape) => shape.meta(),
            Shape::Blob(shape) => shape.meta(),
            Shape::Timestamp(shape) => shape.meta(),
            Shape::Document(shape) => shape.meta(),
            Shape::Enum(shape) => shape.meta(),
            Shape::IntEnum(shape) => shape.meta(),
            Shape::List(shape) => shape.meta(),
            Shape::Map(shape) => shape.meta(),
            Shape::Set(shape) => shape.meta(),
            Shape::Structure(shape) => shape.meta(),
            Shape::Union(shape) => shape.meta(),
            Shape::Service(shape) => shape.meta(),
            Shape::Operation(shape) => shape.meta(),
            Shape::Resource(shape) => shape.meta(),
            Shape::Member(shape) => shape.meta(),
        }
    }
}

impl AsRef<ShapeId> for Shape {
    fn as_ref(&self) -> &ShapeId {
        self.id()
    }
}

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the shape type (variant)
        std::mem::discriminant(self).hash(state);
        // Hash the ID
        self.id().hash(state);
        // We don't hash members and traits for performance reasons,
        // similar to the Java implementation which only hashes type and ID
    }
}

/// Common trait for all shape types providing access to shape ID
pub trait HasShapeId {
    /// Get the shape ID
    fn id(&self) -> &ShapeId;
}

/// Common trait for all shape types providing access to traits
pub trait HasTraits: HasShapeId {
    /// Get all traits applied to this shape (including those from mixins)
    fn traits(&self) -> &TraitMap;

    /// Get traits applied directly to this shape (excluding those from mixins)
    fn introduced_traits(&self) -> &TraitMap;

    /// Check if this shape has a specific trait
    fn has_trait(&self, trait_id: impl AsRef<ShapeId>) -> bool {
        self.traits().contains_key(trait_id.as_ref())
    }

    /// Get a specific trait by ID
    fn get_trait(&self, trait_id: impl AsRef<ShapeId>) -> Option<&BoxTrait> {
        self.traits().get(trait_id.as_ref())
    }

    /// Get a specific trait with a concrete type
    fn get_trait_as<T: Trait + 'static>(&self) -> Option<&T> {
        self.get_trait(T::static_id())
            .and_then(|t| t.as_any().downcast_ref::<T>())
    }

    /// Get a specific trait with a concrete type, panicking if not found or wrong type
    fn expect_trait<T: Trait + 'static>(&self) -> &T {
        self.get_trait_as::<T>().unwrap_or_else(|| {
            panic!(
                "Expected trait {} on shape {}, but it was not found or had the wrong type",
                T::static_id(),
                self.id()
            )
        })
    }
}

/// Private trait for accessing shape metadata
pub(crate) trait ProvideShapeMetadata {
    /// Get the shape metadata
    fn meta(&self) -> &ShapeMetadata;
}

/// Common trait for all shape types providing access to mixins
pub trait HasMixins: HasShapeId {
    /// Get the mixins applied to this shape
    fn mixins(&self) -> &[Shape];
}

// Blanket implementation for any type that provides shape metadata
impl<T: ProvideShapeMetadata> HasMixins for T {
    fn mixins(&self) -> &[Shape] {
        &self.meta().mixins
    }
}

/// Builder for creating shape metadata
#[derive(Debug, Default, Clone)]
pub(crate) struct ShapeMetadataBuilder {
    /// The shape ID string
    id: Option<String>,
    /// Traits applied directly to this shape (introduced traits)
    pub(crate) introduced_traits: TraitMap,
    /// Mixins applied to this shape
    mixins: Vec<Shape>,
}

impl ShapeMetadataBuilder {
    /// Create a new shape metadata builder
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the shape
    pub(crate) fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a trait to the shape
    pub(crate) fn with_trait(mut self, trait_obj: impl Trait) -> Self {
        self.introduced_traits.insert(Box::new(trait_obj));
        self
    }

    /// Add a mixin to the shape
    pub(crate) fn with_mixin(mut self, mixin: Shape) -> Self {
        self.mixins.push(mixin);
        self
    }

    /// Validate that all mixins have the @mixin trait and are of the expected shape type
    pub(crate) fn validate_mixins<F>(
        &self,
        shape_type_validator: F,
        shape_type_name: &str,
    ) -> Result<(), BuildError>
    where
        F: Fn(&Shape) -> bool,
    {
        use crate::traits::{Mixin, Trait};

        for mixin in &self.mixins {
            // Validate that the mixin has the @mixin trait
            if !mixin.has_trait(Mixin::static_id()) {
                return Err(BuildError::InvalidValue {
                    field: field_names::MIXIN.to_string(),
                    reason: format!(
                        "Shape {} is used as a mixin but does not have the @mixin trait",
                        mixin.id()
                    ),
                });
            }

            // Validate that the mixin is of the expected shape type
            if !shape_type_validator(mixin) {
                return Err(BuildError::InvalidValue {
                    field: field_names::MIXIN.to_string(),
                    reason: format!("Mixin {} is not a {} shape", mixin.id(), shape_type_name),
                });
            }
        }

        Ok(())
    }

    /// Build the shape metadata
    pub(crate) fn build(self) -> Result<ShapeMetadata, BuildError> {
        use builder::{field_names, parse_shape_id, required_field_error};

        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        let mut metadata = ShapeMetadata::new(id, self.introduced_traits);

        // Add mixins
        for mixin in self.mixins {
            metadata.add_mixin(mixin);
        }

        // TODO: Compute effective traits from mixins
        // This will be implemented as part of the mixin resolution functions

        Ok(metadata)
    }
}

/// Common metadata for all shapes (implementation detail)
#[derive(Debug, Clone)]
pub(crate) struct ShapeMetadata {
    /// The shape ID
    id: ShapeId,
    /// Traits applied directly to this shape (introduced traits)
    introduced_traits: TraitMap,
    /// All traits applied to this shape (including those from mixins)
    effective_traits: TraitMap,
    /// Mixins applied to this shape (full shape references)
    mixins: Vec<Shape>,
}

impl ShapeMetadata {
    /// Create new shape metadata
    pub(crate) fn new(id: ShapeId, traits: TraitMap) -> Self {
        Self {
            id,
            introduced_traits: traits.clone(),
            effective_traits: traits,
            mixins: Vec::new(),
        }
    }

    /// Add a mixin to this shape
    pub(crate) fn add_mixin(&mut self, mixin: Shape) {
        self.mixins.push(mixin);
    }

    /// Convert this metadata to a builder
    pub(crate) fn to_builder(&self) -> ShapeMetadataBuilder {
        let mut builder = ShapeMetadataBuilder::new().id(self.id.to_string());

        // Copy introduced traits
        for trait_obj in self.introduced_traits.values() {
            let cloned = trait_obj.clone_trait();
            builder.introduced_traits.insert(cloned);
        }

        // Copy mixins
        for mixin in &self.mixins {
            builder.mixins.push(mixin.clone());
        }

        builder
    }
}

// Blanket implementations for any type that provides shape metadata
impl<T: ProvideShapeMetadata> HasShapeId for T {
    fn id(&self) -> &ShapeId {
        &self.meta().id
    }
}

impl<T: ProvideShapeMetadata> HasTraits for T {
    fn traits(&self) -> &TraitMap {
        &self.meta().effective_traits
    }

    fn introduced_traits(&self) -> &TraitMap {
        &self.meta().introduced_traits
    }
}

impl PartialEq for ShapeMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.introduced_traits == other.introduced_traits
            && self.effective_traits == other.effective_traits
            && self.mixins == other.mixins
    }
}

impl Eq for ShapeMetadata {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::builder::ShapeBuilderExt;
    use crate::traits::Documentation;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_shape_id_access() {
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = shape.into();

        assert_eq!(shape.id().to_string(), "example.foo#MyString");
    }

    #[test]
    fn test_shape_traits_access() {
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .documentation("A string shape")
            .build()
            .unwrap();
        let shape: Shape = shape.into();

        assert!(shape.has_trait(Documentation::static_id()));
    }

    #[test]
    fn test_shape_enum_conversion() {
        let string_shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();
        let shape: Shape = string_shape.into();

        match shape {
            Shape::String(_) => {}
            _ => panic!("Expected Shape::String"),
        }
    }

    #[test]
    fn test_member_shape() {
        let target = ShapeId::new("smithy.api", "String").unwrap();
        let member = MemberShape::builder()
            .id("example.foo#MyStruct$name")
            .member_name("name")
            .target(target.clone())
            .build()
            .unwrap();

        assert_eq!(member.id().to_string(), "example.foo#MyStruct$name");
        assert_eq!(member.member_name, "name");
        assert_eq!(member.target, target);
    }

    // Core trait tests

    /// Tests for the core traits (HasShapeId, HasTraits, ProvideShapeMetadata)
    mod trait_tests {
        use super::*;

        use std::str::FromStr;

        // Test fixture - a simple shape that implements ProvideShapeMetadata
        fn create_test_shape() -> StringShape {
            StringShape::builder()
                .id("example.foo#TestShape")
                .documentation("Test documentation")
                .build()
                .unwrap()
        }

        #[test]
        fn test_has_shape_id() {
            let shape = create_test_shape();

            // Test HasShapeId methods
            assert_eq!(shape.id().to_string(), "example.foo#TestShape");
            assert_eq!(shape.id().namespace(), "example.foo");
            assert_eq!(shape.id().name(), "TestShape");
        }

        #[test]
        fn test_has_traits() {
            let shape = create_test_shape();
            let trait_id = Documentation::static_id();
            let nonexistent = ShapeId::from_str("smithy.api#nonexistent").unwrap();

            // Test HasTraits methods
            assert!(shape.has_trait(&trait_id));
            assert!(shape.has_trait(&trait_id));
            assert!(shape.get_trait(&trait_id).is_some());
            assert!(shape.get_trait(&nonexistent).is_none());

            assert!(shape.get_trait_as::<Documentation>().is_some());
            assert_eq!(
                shape.expect_trait::<Documentation>().0,
                "Test documentation"
            );

            // Test introduced_traits
            assert_eq!(shape.introduced_traits().len(), 1);
            assert!(shape.introduced_traits().contains_key(&trait_id));
        }

        #[test]
        fn test_shape_metadata() {
            let shape = create_test_shape();

            // Test that metadata is correctly initialized
            assert_eq!(shape.meta().id.to_string(), "example.foo#TestShape");
            assert_eq!(shape.meta().effective_traits.len(), 1);
            assert!(shape
                .meta()
                .effective_traits
                .contains_key(&ShapeId::new("smithy.api", "documentation").unwrap()));

            // Test that introduced_traits matches effective_traits initially
            assert_eq!(shape.meta().introduced_traits.len(), 1);
            assert_eq!(shape.meta().mixins.len(), 0);
            assert_eq!(
                shape.meta().introduced_traits.keys().collect::<Vec<_>>(),
                shape.meta().effective_traits.keys().collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_blanket_implementations() {
            // Test that blanket implementations work for different shape types
            let string_shape = StringShape::builder()
                .id("example.foo#String")
                .build()
                .unwrap();

            let boolean_shape = BooleanShape::builder()
                .id("example.foo#Boolean")
                .build()
                .unwrap();

            let structure_shape = StructureShape::builder()
                .id("example.foo#Structure")
                .build()
                .unwrap();

            // All should implement HasShapeId
            assert_eq!(string_shape.id().name(), "String");
            assert_eq!(boolean_shape.id().name(), "Boolean");
            assert_eq!(structure_shape.id().name(), "Structure");

            // All should implement HasTraits
            assert_eq!(string_shape.traits().len(), 0);
            assert_eq!(boolean_shape.traits().len(), 0);
            assert_eq!(structure_shape.traits().len(), 0);
        }
    }

    // Shape equality and hash tests

    #[test]
    fn test_shape_equality() {
        let shape1 = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();

        let shape2 = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap();

        let shape3 = StringShape::builder()
            .id("example.foo#DifferentString")
            .build()
            .unwrap();

        // Same type, ID, and no traits should be equal
        assert_eq!(shape1, shape2);

        // Different ID should not be equal
        assert_ne!(shape1, shape3);

        // Test with traits
        let shape_with_trait = StringShape::builder()
            .id("example.foo#MyString")
            .documentation("A test string shape")
            .build()
            .unwrap();

        // Same ID but different traits should NOT be equal
        // This matches the Java implementation's behavior
        assert_ne!(shape1, shape_with_trait);

        // Verify hash implementation is consistent with equality
        let mut map = HashMap::new();
        map.insert(shape1.clone(), "value1");
        assert!(map.contains_key(&shape2));
        assert!(!map.contains_key(&shape3));
        assert!(!map.contains_key(&shape_with_trait));

        // Test with identical traits
        let shape_with_same_trait = StringShape::builder()
            .id("example.foo#MyString")
            .documentation("A test string shape")
            .build()
            .unwrap();

        // Same ID and same traits should be equal
        assert_eq!(shape_with_trait, shape_with_same_trait);

        // Different shape types with same ID should not be equal
        let bool_shape = BooleanShape::builder()
            .id("example.foo#MyString") // Same ID as string_shape
            .build()
            .unwrap();

        assert_ne!(Shape::from(shape1), Shape::from(bool_shape));
    }

    // Type checking tests

    #[test]
    fn test_shape_type_checking() {
        let string_shape: Shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap()
            .into();

        let boolean_shape: Shape = BooleanShape::builder()
            .id("example.foo#MyBoolean")
            .build()
            .unwrap()
            .into();

        // Test is_* methods
        assert!(string_shape.is_string());
        assert!(!string_shape.is_boolean());

        assert!(boolean_shape.is_boolean());
        assert!(!boolean_shape.is_string());

        // Test as_* methods
        assert!(string_shape.as_string().is_some());
        assert!(string_shape.as_boolean().is_none());

        assert!(boolean_shape.as_boolean().is_some());
        assert!(boolean_shape.as_string().is_none());

        // Test expect_* methods
        let _ = string_shape.expect_string(); // Should not panic
        let _ = boolean_shape.expect_boolean(); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Expected a boolean shape")]
    fn test_expect_wrong_type() {
        let string_shape: Shape = StringShape::builder()
            .id("example.foo#MyString")
            .build()
            .unwrap()
            .into();

        // This should panic
        let _ = string_shape.expect_boolean();
    }

    // Builder pattern tests

    #[test]
    fn test_builder_pattern() {
        // Test builder chaining
        let shape = StringShape::builder()
            .id("example.foo#MyString")
            .documentation("A test string shape")
            .required()
            .build()
            .unwrap();

        // Verify all traits were applied
        assert!(shape.has_trait(ShapeId::from_str("smithy.api#documentation").unwrap()));
        assert!(shape.has_trait(ShapeId::from_str("smithy.api#required").unwrap()));
    }

    #[test]
    fn test_builder_errors() {
        // Test missing ID
        let result = StringShape::builder().build();
        assert!(result.is_err());
        match result {
            Err(BuildError::MissingRequiredField { field }) => {
                assert_eq!(field, "id");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }

        // Test invalid ID
        let result = StringShape::builder().id("invalid-id").build();
        assert!(result.is_err());
        match result {
            Err(BuildError::InvalidValue { field, .. }) => {
                assert_eq!(field, "id");
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }
}
