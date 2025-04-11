use crate::shape::{
    field_names, parse_shape_id, required_field_error, ProvideShapeMetadata, ProvideTraitsMut,
    Shape, ShapeMetadata,
};
use crate::traits::Trait;
use crate::{shape, ShapeId};
use std::collections::HashMap;

/// A [member](https://smithy.io/2.0/spec/model.html#member-shapes) shape
#[derive(Debug, Clone)]
pub struct MemberShape {
    metadata: ShapeMetadata,
    /// The name of the member
    pub member_name: String,
    /// The target shape that this member references
    pub target: ShapeId,
}

/// Builder for creating a member shape.
#[derive(Debug, Default)]
pub struct MemberShapeBuilder {
    id: Option<String>,
    traits: HashMap<ShapeId, Trait>,
    member_name: Option<String>,
    target: Option<ShapeId>,
}

impl MemberShapeBuilder {
    /// Create a new member shape builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ID of the member shape.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
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
        let id_str = self
            .id
            .ok_or_else(|| required_field_error(field_names::ID))?;
        let id = parse_shape_id(&id_str)?;

        let member_name = self
            .member_name
            .ok_or_else(|| required_field_error(field_names::MEMBER))?;

        let target = self
            .target
            .ok_or_else(|| required_field_error(field_names::TARGET))?;

        Ok(MemberShape {
            metadata: ShapeMetadata::new(id, self.traits),
            member_name,
            target,
        })
    }
}

impl ProvideTraitsMut for MemberShapeBuilder {
    fn traits_mut(&mut self) -> &mut HashMap<ShapeId, Trait> {
        &mut self.traits
    }
}

impl MemberShape {
    /// Create a new builder for this shape type.
    pub fn builder() -> MemberShapeBuilder {
        MemberShapeBuilder::new()
    }
}

impl ProvideShapeMetadata for MemberShape {
    fn meta(&self) -> &ShapeMetadata {
        &self.metadata
    }
}

impl From<MemberShape> for Shape {
    fn from(shape: MemberShape) -> Self {
        Shape::Member(shape)
    }
}
