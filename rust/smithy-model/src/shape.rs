/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Shape types for the Smithy model.

mod aggregate;
mod builder;
mod error;
pub mod iter;
mod member;
mod mixin;
mod operation;
mod resource;
mod service;
mod shape_id;
mod simple;
mod type_checks;

pub use self::aggregate::*;
pub use self::builder::*;
pub use self::error::*;
pub use self::member::*;
pub use self::operation::*;
pub use self::resource::*;
pub use self::service::*;
pub use self::simple::*;
use crate::shape::iter::Mixins;
use crate::traits::{BoxTrait, Trait, TraitMap};
use iter::Members;
pub use shape_id::ShapeId;
use std::hash::{Hash, Hasher};

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

/// Trait for accessing properties common to all shapes
pub trait ShapeProperties {
    /// Get the shape metadata
    #[doc(hidden)]
    fn metadata(&self) -> &ShapeMetadata;

    /// Get the shape's ID
    fn id(&self) -> &ShapeId {
        &self.metadata().id
    }

    /// Get the shape's mixins
    fn mixins(&self) -> Mixins<'_> {
        Mixins::new(&self.metadata().mixins)
    }

    /// Get all traits applied to this shape (including those from mixins)
    fn traits(&self) -> &TraitMap {
        &self.metadata().effective_traits
    }

    /// Get traits applied directly to this shape (excluding those from mixins)
    fn introduced_traits(&self) -> &TraitMap {
        &self.metadata().introduced_traits
    }

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

/// Common trait for all shape builders
pub trait ShapeBuilder {
    /// Get mutable access to the metadata builder
    #[doc(hidden)]
    fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder;

    /// Get mutable access to the trait map
    fn traits_mut(&mut self) -> &mut TraitMap {
        &mut self.metadata_mut().introduced_traits
    }

    /// Add a trait to the shape
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::shape::StringShape;
    /// use smithy_model::shape::{ShapeBuilder, ShapeProperties};
    /// use smithy_model::shape::ShapeId;
    /// use smithy_model::traits::{DynamicTrait, Trait};
    ///
    /// let trait_id = ShapeId::new("example.foo", "customTrait").unwrap();
    /// let custom_trait = DynamicTrait::new(trait_id.clone(), None);
    ///
    /// let shape = StringShape::builder()
    ///     .id("example.foo#MyString")
    ///     .with_trait(custom_trait)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(shape.has_trait(trait_id));
    /// ```
    fn with_trait<T: Trait + 'static>(mut self, trait_obj: T) -> Self
    where
        Self: Sized,
    {
        self.traits_mut().insert(Box::new(trait_obj));
        self
    }

    /// Add a mixin to the shape
    fn mixin(mut self, mixin: impl Into<Shape>) -> Self
    where
        Self: Sized,
    {
        let mixin = mixin.into();
        self.metadata_mut().add_mixin(mixin);
        self
    }

    /// Remove all mixins from the shape
    fn clear_mixins(mut self) -> Self
    where
        Self: Sized,
    {
        self.metadata_mut().clear_mixins();
        self
    }

    /// Remove a specific mixin from the shape by ID
    fn remove_mixin(mut self, id: impl AsRef<ShapeId>) -> Self
    where
        Self: Sized,
    {
        self.metadata_mut().remove_mixin(id.as_ref());
        self
    }
}

impl Shape {
    /// Returns a Members container for this shape.
    pub fn members(&self) -> Members<'_> {
        match self {
            Shape::Structure(shape) => shape.members(),
            Shape::Union(shape) => shape.members(),
            Shape::List(shape) => shape.members(),
            Shape::Set(shape) => shape.members(),
            Shape::Map(shape) => shape.members(),
            Shape::Enum(shape) => shape.members(),
            Shape::IntEnum(shape) => shape.members(),
            _ => Members::empty(),
        }
    }
}

impl ShapeProperties for Shape {
    fn metadata(&self) -> &ShapeMetadata {
        match self {
            Shape::Boolean(shape) => shape.metadata(),
            Shape::Byte(shape) => shape.metadata(),
            Shape::Short(shape) => shape.metadata(),
            Shape::Integer(shape) => shape.metadata(),
            Shape::Long(shape) => shape.metadata(),
            Shape::Float(shape) => shape.metadata(),
            Shape::Double(shape) => shape.metadata(),
            Shape::BigInteger(shape) => shape.metadata(),
            Shape::BigDecimal(shape) => shape.metadata(),
            Shape::String(shape) => shape.metadata(),
            Shape::Blob(shape) => shape.metadata(),
            Shape::Timestamp(shape) => shape.metadata(),
            Shape::Document(shape) => shape.metadata(),
            Shape::Enum(shape) => shape.metadata(),
            Shape::IntEnum(shape) => shape.metadata(),
            Shape::List(shape) => shape.metadata(),
            Shape::Map(shape) => shape.metadata(),
            Shape::Set(shape) => shape.metadata(),
            Shape::Structure(shape) => shape.metadata(),
            Shape::Union(shape) => shape.metadata(),
            Shape::Service(shape) => shape.metadata(),
            Shape::Operation(shape) => shape.metadata(),
            Shape::Resource(shape) => shape.metadata(),
            Shape::Member(shape) => shape.metadata(),
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

/// Builder for creating shape metadata
#[derive(Debug, Default, Clone)]
pub struct ShapeMetadataBuilder {
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

    /// Add a mixin to the shape
    pub(crate) fn add_mixin(&mut self, mixin: Shape) {
        self.mixins.push(mixin);
    }

    /// Remove a mixin by id
    pub(crate) fn remove_mixin(&mut self, mixin_id: &ShapeId) {
        let idx = self.mixins.iter().position(|m| m.id() == mixin_id);
        if let Some(idx) = idx {
            self.mixins.remove(idx);
        }
    }

    /// Clear all mixins
    pub(crate) fn clear_mixins(&mut self) {
        self.mixins.clear();
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
        let introduced_traits = self.introduced_traits;
        let mixins = self.mixins;

        // TODO - maybe use Arc<TraitMap> such that effective traits is cloned cheaply when
        // mixins are not used since effective and introduced will be the same.
        // Compute effective traits from mixins
        let effective_traits = mixin::compute_effective_traits(&introduced_traits, &mixins);

        Ok(ShapeMetadata {
            id,
            introduced_traits,
            effective_traits,
            mixins,
        })
    }
}

/// Common metadata for all shapes (implementation detail)
#[derive(Debug, Clone)]
pub struct ShapeMetadata {
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
            assert!(shape.has_trait(trait_id));
            assert!(shape.has_trait(trait_id));
            assert!(shape.get_trait(trait_id).is_some());
            assert!(shape.get_trait(&nonexistent).is_none());

            assert!(shape.get_trait_as::<Documentation>().is_some());
            assert_eq!(
                shape.expect_trait::<Documentation>().0,
                "Test documentation"
            );

            // Test introduced_traits
            assert_eq!(shape.introduced_traits().len(), 1);
            assert!(shape.introduced_traits().contains_key(trait_id));
        }

        #[test]
        fn test_shape_metadata() {
            let shape = create_test_shape();

            // Test that metadata is correctly initialized
            assert_eq!(shape.metadata().id.to_string(), "example.foo#TestShape");
            assert_eq!(shape.metadata().effective_traits.len(), 1);
            assert!(shape
                .metadata()
                .effective_traits
                .contains_key(&ShapeId::new("smithy.api", "documentation").unwrap()));

            // Test that introduced_traits matches effective_traits initially
            assert_eq!(shape.metadata().introduced_traits.len(), 1);
            assert_eq!(shape.metadata().mixins.len(), 0);
            assert_eq!(
                shape
                    .metadata()
                    .introduced_traits
                    .keys()
                    .collect::<Vec<_>>(),
                shape.metadata().effective_traits.keys().collect::<Vec<_>>()
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
