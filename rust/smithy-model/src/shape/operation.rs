use crate::shape::{
    BuildError, ProvideShapeMetadata, ProvideTraitsMut, ServiceShape, Shape, ShapeMetadata,
    ShapeMetadataBuilder,
};
use crate::traits::TraitMap;
use crate::ShapeId;

/// An [operation](https://smithy.io/2.0/spec/service-types.html#operation) shape
#[derive(Debug, Clone, PartialEq)]
pub struct OperationShape {
    pub(crate) metadata: ShapeMetadata,
    /// The input shape ID, if any
    pub input: Option<ShapeId>,
    /// The output shape ID, if any
    pub output: Option<ShapeId>,
    /// The errors that can be thrown by this operation
    pub errors: Vec<ShapeId>,
}

/// Builder for creating an operation shape.
#[derive(Debug, Default)]
pub struct OperationShapeBuilder {
    metadata: ShapeMetadataBuilder,
    input: Option<ShapeId>,
    output: Option<ShapeId>,
    errors: Vec<ShapeId>,
}

impl OperationShapeBuilder {
    /// Create a new operation shape builder.
    pub fn new() -> Self {
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
        self.errors.push(error);
        self
    }

    /// Add multiple error shape IDs.
    pub fn errors(mut self, errors: Vec<ShapeId>) -> Self {
        self.errors.extend(errors);
        self
    }

    /// Add a mixin to the operation shape.
    pub fn mixin(mut self, mixin: impl Into<Shape>) -> Self {
        self.metadata = self.metadata.with_mixin(mixin.into());
        self
    }

    /// Build the operation shape.
    pub fn build(self) -> Result<OperationShape, BuildError> {
        // Build the metadata
        self.metadata
            .validate_mixins(|shape| matches!(shape, Shape::Operation(_)), "operation")?;

        let metadata = self.metadata.build()?;

        Ok(OperationShape {
            metadata,
            input: self.input,
            output: self.output,
            errors: self.errors,
        })
    }
}

impl ProvideTraitsMut for OperationShapeBuilder {
    fn traits_mut(&mut self) -> &mut TraitMap {
        &mut self.metadata.introduced_traits
    }
}

impl OperationShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> OperationShapeBuilder {
        OperationShapeBuilder::new()
    }
}

impl ProvideShapeMetadata for OperationShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<OperationShape> for Shape {
    fn from(shape: OperationShape) -> Self {
        Shape::Operation(shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::HasShapeId;
    // Operation shape tests

    #[test]
    fn test_operation_shape_construction() {
        let input = ShapeId::new("example.foo", "MyInput").unwrap();
        let output = ShapeId::new("example.foo", "MyOutput").unwrap();
        let error1 = ShapeId::new("example.foo", "Error1").unwrap();
        let error2 = ShapeId::new("example.foo", "Error2").unwrap();

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
        assert_eq!(shape.errors.len(), 0);
    }
}
