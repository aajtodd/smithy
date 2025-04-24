use crate::shape::{
    field_names, required_field_error, Shape, ShapeBuilder, ShapeMetadata, ShapeMetadataBuilder,
    ShapeProperties,
};
use crate::{shape, ShapeId};

/// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
#[derive(Debug, Clone, PartialEq)]
pub struct MemberShape {
    metadata: ShapeMetadata,
    /// The name of the member
    pub member_name: String,
    /// The target shape that this member references
    pub target: ShapeId,
}

impl MemberShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> MemberShapeBuilder {
        MemberShapeBuilder::new()
    }

    /// Get the target shape ID of this member shape
    pub fn target(&self) -> &ShapeId {
        &self.target
    }

    /// Convert this shape back into a builder
    pub fn to_builder(self) -> MemberShapeBuilder {
        MemberShapeBuilder {
            metadata: self.metadata.to_builder(),
            member_name: Some(self.member_name),
            target: Some(self.target),
        }
    }
}

impl ShapeProperties for MemberShape {
    fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<MemberShape> for Shape {
    fn from(shape: MemberShape) -> Self {
        Shape::Member(shape)
    }
}

/// Builder for creating a member shape.
#[derive(Debug, Default)]
pub struct MemberShapeBuilder {
    metadata: ShapeMetadataBuilder,
    member_name: Option<String>,
    target: Option<ShapeId>,
}

impl MemberShapeBuilder {
    /// Create a new member shape builder.
    fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the member shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.metadata = self.metadata.id(id);
        self
    }

    /// Set the name of the member.
    pub fn member_name(mut self, name: impl Into<String>) -> Self {
        self.member_name = Some(name.into());
        self
    }

    /// Set the target shape ID.
    pub fn target(mut self, target: ShapeId) -> Self {
        self.target = Some(target);
        self
    }

    /// Build the member shape.
    pub fn build(self) -> Result<MemberShape, shape::BuildError> {
        let metadata = self.metadata.build()?;

        let member_name = self
            .member_name
            .ok_or_else(|| required_field_error(field_names::MEMBER))?;

        let target = self
            .target
            .ok_or_else(|| required_field_error(field_names::TARGET))?;

        Ok(MemberShape {
            metadata,
            member_name,
            target,
        })
    }
}

impl ShapeBuilder for MemberShapeBuilder {
    fn metadata_mut(&mut self) -> &mut ShapeMetadataBuilder {
        &mut self.metadata
    }
}
