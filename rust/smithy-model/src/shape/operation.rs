use crate::shape::{
    BuildError, Shape, ShapeBuilder, ShapeMetadata, ShapeMetadataBuilder, ShapeProperties,
};
use crate::traits::{Mixin, Trait};
use crate::ShapeId;

/// An [operation](https://smithy.io/2.0/spec/service-types.html#operation) shape
#[derive(Debug, Clone, PartialEq)]
pub struct OperationShape {
    pub(crate) metadata: ShapeMetadata,
    /// The input shape ID, if any
    pub input: Option<ShapeId>,
    /// The output shape ID, if any
    pub output: Option<ShapeId>,
    /// The errors directly defined on this operation (introduced errors)
    pub introduced_errors: Vec<ShapeId>,
    /// All errors that can be thrown by this operation (including those from mixins)
    pub errors: Vec<ShapeId>,
}

impl OperationShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> OperationShapeBuilder {
        OperationShapeBuilder::new()
    }

    /// Create a builder from this shape.
    pub fn to_builder(self) -> OperationShapeBuilder {
        OperationShapeBuilder {
            metadata: self.metadata.to_builder(),
            input: self.input,
            output: self.output,
            introduced_errors: self.introduced_errors,
        }
    }
}

impl ShapeProperties for OperationShape {
    fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<OperationShape> for Shape {
    fn from(shape: OperationShape) -> Self {
        Shape::Operation(shape)
    }
}

/// Builder for creating an operation shape.
#[derive(Debug, Default)]
pub struct OperationShapeBuilder {
    metadata: ShapeMetadataBuilder,
    input: Option<ShapeId>,
    output: Option<ShapeId>,
    introduced_errors: Vec<ShapeId>,
}

impl OperationShapeBuilder {
    /// Create a new operation shape builder.
    fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the operation shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
        self
    }

    /// Set the input shape ID.
    pub fn input(mut self, input: ShapeId) -> Self {
        self.input = Some(input);
        self
    }

    /// Set the output shape ID.
    pub fn output(mut self, output: ShapeId) -> Self {
        self.output = Some(output);
        self
    }

    /// Add an error shape ID.
    pub fn error(mut self, error: ShapeId) -> Self {
        self.introduced_errors.push(error);
        self
    }

    /// Add multiple error shape IDs.
    pub fn errors(mut self, errors: Vec<ShapeId>) -> Self {
        self.introduced_errors.extend(errors);
        self
    }

    /// Build the operation shape.
    pub fn build(self) -> Result<OperationShape, BuildError> {
        // Build the metadata
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Operation(_)), "operation")?;

        let metadata = self.metadata.build()?;

        // Check if this is a mixin operation
        let is_mixin = metadata.effective_traits.contains_key(Mixin::static_id());

        // Mixin operations must use unit type for input and output
        if is_mixin && (self.input.is_some() || self.output.is_some()) {
            return Err(BuildError::InvalidValue {
                field: "input/output".to_string(),
                reason: "Operation shapes with the mixin trait may not define input or output"
                    .to_string(),
            });
        }

        // Initialize errors with the directly specified errors
        let introduced_errors = self.introduced_errors.clone();
        let mut errors = introduced_errors.clone();

        // Add errors from mixins
        for mixin in &metadata.mixins {
            if let Shape::Operation(operation) = mixin {
                // Add errors from mixin if not already present
                for error in &operation.errors {
                    if !errors.contains(error) {
                        errors.push(error.clone());
                    }
                }
            }
        }

        Ok(OperationShape {
            metadata,
            input: self.input,
            output: self.output,
            introduced_errors,
            errors,
        })
    }
}

impl ShapeBuilder for OperationShapeBuilder {
    fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder {
        &mut self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Mixin;
    // Operation shape tests

    #[test]
    fn test_operation_shape_construction() {
        let input = ShapeId::new_unchecked("example.foo#MyInput");
        let output = ShapeId::new_unchecked("example.foo#MyOutput");
        let error1 = ShapeId::new_unchecked("example.foo#Error1");
        let error2 = ShapeId::new_unchecked("example.foo#Error2");

        let shape = OperationShape::builder()
            .id("example.foo#MyOperation")
            .input(input.clone())
            .output(output.clone())
            .error(error1.clone())
            .error(error2.clone())
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyOperation");
        assert_eq!(shape.input, Some(input));
        assert_eq!(shape.output, Some(output));
        assert_eq!(shape.introduced_errors.len(), 2);
        assert!(shape.introduced_errors.contains(&error1));
        assert!(shape.introduced_errors.contains(&error2));
        assert_eq!(shape.errors.len(), 2);
        assert!(shape.errors.contains(&error1));
        assert!(shape.errors.contains(&error2));
    }

    #[test]
    fn test_operation_shape_without_io() {
        let shape = OperationShape::builder()
            .id("example.foo#MyOperation")
            .build()
            .unwrap();

        assert_eq!(shape.id().to_string(), "example.foo#MyOperation");
        assert_eq!(shape.input, None);
        assert_eq!(shape.output, None);
        assert_eq!(shape.introduced_errors.len(), 0);
        assert_eq!(shape.errors.len(), 0);
    }

    #[test]
    fn test_operation_shape_with_mixins() {
        // Create a mixin operation (must have no input/output)
        let mixin_error = ShapeId::new_unchecked("example.foo#MixinError");

        let mixin = OperationShape::builder()
            .id("example.foo#MixinOperation")
            .error(mixin_error.clone())
            .with_trait(Mixin::new())
            .build()
            .unwrap();

        // Test case: Operation inherits errors from mixin
        let direct_input = ShapeId::new_unchecked("example.foo#DirectInput");
        let direct_output = ShapeId::new_unchecked("example.foo#DirectOutput");
        let direct_error = ShapeId::new_unchecked("example.foo#DirectError");

        let operation = OperationShape::builder()
            .id("example.foo#Operation")
            .input(direct_input.clone())
            .output(direct_output.clone())
            .error(direct_error.clone())
            .mixin(mixin)
            .build()
            .unwrap();

        assert_eq!(operation.input, Some(direct_input));
        assert_eq!(operation.output, Some(direct_output));
        assert_eq!(operation.introduced_errors.len(), 1);
        assert!(operation.introduced_errors.contains(&direct_error));
        assert_eq!(operation.errors.len(), 2);
        assert!(operation.errors.contains(&direct_error));
        assert!(operation.errors.contains(&mixin_error));
    }

    #[test]
    fn test_operation_mixin_validation() {
        // Mixin operations cannot have input or output
        let input = ShapeId::new_unchecked("example.foo#Input");

        let result = OperationShape::builder()
            .id("example.foo#MixinOperation")
            .input(input)
            .with_trait(Mixin::new())
            .build();

        assert!(result.is_err());
    }
}
