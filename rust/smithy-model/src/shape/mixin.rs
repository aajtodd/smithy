/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Utilities for working with mixins in shapes.

use crate::shape::error::BuildError;
use crate::shape::iter::Members;
use crate::shape::{
    HasMixins, HasShapeId, HasTraits, MemberShape, ProvideTraitsMut, Shape, ShapeId,
};
use std::collections::HashMap;

// FIXME - provide a unified way for builders to add/clear mixins. It's all adhoc right now.
// FIXME - unify "named member" builders as well for adding/removing members?

/// Computes effective members for a shape, including those from mixins.
///
/// This function merges members from mixins with locally defined members,
/// following the Smithy specification rules for member inheritance and trait precedence.
pub(crate) fn compute_effective_members(
    shape_id: &ShapeId,
    introduced_members: &Members<'_>,
    mixins: &[Shape],
) -> Result<HashMap<String, MemberShape>, BuildError> {
    // If there are no mixins, we can just convert the introduced members to a HashMap
    if mixins.is_empty() {
        return Ok(introduced_members
            .iter_named()
            .map(|(name, member)| (name.to_string(), member.clone()))
            .collect());
    }

    // Create a map to track computed members
    let mut computed_members = HashMap::new();

    // Process mixins in order (first to last)
    for mixin in mixins {
        for (name, mixin_member) in mixin.members().iter_named() {
            if let Some(local_member) = introduced_members.get(name) {
                // Handle case where the member exists locally
                process_local_member(
                    name,
                    local_member,
                    mixin_member,
                    mixins,
                    &mut computed_members,
                )?;
            } else {
                // Check if we need to process an existing mixin member
                let previous_opt = computed_members.get(name).cloned();
                if let Some(previous) = previous_opt {
                    process_existing_mixin_member(
                        name,
                        previous,
                        mixin_member,
                        &mut computed_members,
                    )?;
                } else {
                    // Create a new member based on the mixin member
                    let new_member = create_member_from_mixin(shape_id, name, mixin_member)?;
                    computed_members.insert(name.to_string(), new_member);
                }
            }
        }
    }

    // Add any remaining local members that weren't processed
    for (name, member) in introduced_members.iter_named() {
        if !computed_members.contains_key(name) {
            computed_members.insert(name.to_string(), member.clone());
        }
    }

    Ok(computed_members)
}

/// Processes a member that exists both locally and in a mixin.
///
/// This function handles the case where a member is defined locally in a shape
/// and also exists in one of the shape's mixins. It ensures that the local member
/// properly tracks its mixin origins.
fn process_local_member(
    name: &str,
    local_member: &MemberShape,
    mixin_member: &MemberShape,
    mixins: &[Shape],
    computed_members: &mut HashMap<String, MemberShape>,
) -> Result<(), BuildError> {
    // Validate that the local member doesn't conflict with the mixin member
    if local_member.target() != mixin_member.target() {
        return Err(BuildError::InvalidValue {
            field: name.to_string(),
            reason: format!(
                "Member conflict: member '{}' in shape '{}' targets '{}', but the same member in mixin shape '{}' targets '{}'. Members with the same name must target the same shape.",
                name,
                local_member.id().without_member(),
                local_member.target(),
                mixin_member.id().without_member(),
                mixin_member.target()
            ),
        });
    }

    // Check if the local member needs to be rebuilt with mixin information
    // This matches the Java logic: if the member has no mixins or doesn't contain this mixin.
    // This is necessary to handle cases where a member is defined locally but should be
    // tracking that it's also coming from a mixin. This can happen when a model is manually
    // constructed or when refactoring shapes.
    let needs_rebuild = local_member.mixins().is_empty()
        || !local_member
            .mixins()
            .iter()
            .any(|m| m.id() == mixin_member.id());

    if needs_rebuild {
        // Rebuild the member with proper mixin information
        let rebuilt_member = rebuild_member_mixins(local_member, mixins, name)?;
        computed_members.insert(name.to_string(), rebuilt_member);
    } else {
        // Use the local member as is
        computed_members.insert(name.to_string(), local_member.clone());
    }

    Ok(())
}

/// Rebuilds a member's mixins to ensure it properly tracks all mixin origins.
///
/// This function is used to handle cases where a member is defined locally in a shape
/// but should be tracking that it's also coming from one or more mixins. This can happen
/// when a model is manually constructed or when refactoring shapes.
fn rebuild_member_mixins(
    member: &MemberShape,
    mixins: &[Shape],
    member_name: &str,
) -> Result<MemberShape, BuildError> {
    let mut builder = member.clone().to_builder();

    // Clear existing mixins and rebuild them from all mixins
    builder = builder.clear_mixins();

    // Add mixins from all mixin shapes
    for mixin in mixins {
        if let Some(mixin_member) = mixin.members().get(member_name) {
            builder = builder.mixin(mixin_member.clone());
        }
    }

    builder.build()
}

/// Processes a member that exists in multiple mixins.
///
/// This function handles the case where a member is defined in multiple mixins
/// applied to the same shape. It ensures that traits are properly merged and
/// that the member tracks all of its mixin origins.
fn process_existing_mixin_member(
    name: &str,
    previous: MemberShape,
    mixin_member: &MemberShape,
    computed_members: &mut HashMap<String, MemberShape>,
) -> Result<(), BuildError> {
    // Validate that the members don't conflict
    if previous.target() != mixin_member.target() {
        return Err(BuildError::InvalidValue {
            field: name.to_string(),
            reason: format!(
                "Mixin conflict: member '{}' from mixin '{}' targets '{}', but the same member from mixin '{}' targets '{}'. Members with the same name across mixins must target the same shape.",
                name,
                previous.id().without_member(),
                previous.target(),
                mixin_member.id().without_member(),
                mixin_member.target()
            ),
        });
    }

    // Add this mixin member to the existing member's mixins and merge traits
    let mut builder = previous.to_builder();
    builder = builder.mixin(mixin_member.clone());

    // Add traits from the mixin member
    for (_, trait_obj) in mixin_member.traits().iter() {
        builder.traits_mut().insert(trait_obj.clone_trait());
    }

    computed_members.insert(name.to_string(), builder.build()?);
    Ok(())
}

/// Creates a new member based on a mixin member.
///
/// This function creates a new member for a shape based on a member from one of
/// the shape's mixins. The new member has the same target and traits as the mixin
/// member, but with an ID based on the target shape.
fn create_member_from_mixin(
    shape_id: &ShapeId,
    name: &str,
    mixin_member: &MemberShape,
) -> Result<MemberShape, BuildError> {
    let mut builder = MemberShape::builder()
        .id(format!("{}${}", shape_id, name))
        .member_name(name.to_string())
        .target(mixin_member.target().clone())
        .mixin(mixin_member.clone());

    // Copy traits from the mixin member
    for (_, trait_obj) in mixin_member.traits().iter() {
        builder.traits_mut().insert(trait_obj.clone_trait());
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::{MemberShape, StructureShape};
    use crate::traits::Documentation;
    use crate::traits::Required;
    use crate::traits::Trait;
    use std::collections::HashMap;

    // Helper function to create a test structure shape with the given ID and members
    fn create_structure(id: &str, members: Vec<(&'static str, &'static str)>) -> StructureShape {
        let mut structure_members = Vec::new();

        for (name, target) in members {
            let member = MemberShape::builder()
                .id(format!("{}${}", id, name))
                .member_name(name)
                .target(ShapeId::new_unchecked(target))
                .build()
                .unwrap();

            structure_members.push(member);
        }

        StructureShape::builder()
            .id(id)
            .members(structure_members)
            .build()
            .unwrap()
    }

    // Helper function to create a member with traits
    fn create_member_with_traits(
        id: &str,
        name: &'static str,
        target: &'static str,
        traits: Vec<Box<dyn Trait>>,
    ) -> MemberShape {
        let mut builder = MemberShape::builder()
            .id(id)
            .member_name(name)
            .target(ShapeId::new_unchecked(target));

        for trait_obj in traits {
            builder.traits_mut().insert(trait_obj);
        }

        builder.build().unwrap()
    }

    #[test]
    fn test_compute_effective_members_no_mixins() {
        // Create a structure with no mixins
        let structure = create_structure(
            "example#Test",
            vec![("foo", "smithy.api#String"), ("bar", "smithy.api#Integer")],
        );

        // Get the members
        let members = Members::map(&structure.members);

        // Compute effective members
        let effective_members = compute_effective_members(structure.id(), &members, &[]).unwrap();

        // Verify that the effective members match the original members
        assert_eq!(effective_members.len(), 2);
        assert!(effective_members.contains_key("foo"));
        assert!(effective_members.contains_key("bar"));
        assert_eq!(
            effective_members["foo"].target().to_string(),
            "smithy.api#String"
        );
        assert_eq!(
            effective_members["bar"].target().to_string(),
            "smithy.api#Integer"
        );
    }

    #[test]
    fn test_compute_effective_members_with_mixin() {
        // Create a mixin structure
        let mixin = create_structure("example#Mixin", vec![("mixin_member", "smithy.api#String")]);

        // Create a structure with local members
        let structure =
            create_structure("example#Test", vec![("local_member", "smithy.api#Integer")]);

        // Get the local members
        let members = Members::map(&structure.members);

        // Compute effective members
        let effective_members =
            compute_effective_members(structure.id(), &members, &[mixin.into()]).unwrap();

        // Verify that the effective members include both mixin and local members
        assert_eq!(effective_members.len(), 2);
        assert!(effective_members.contains_key("mixin_member"));
        assert!(effective_members.contains_key("local_member"));
        assert_eq!(
            effective_members["mixin_member"].target().to_string(),
            "smithy.api#String"
        );
        assert_eq!(
            effective_members["local_member"].target().to_string(),
            "smithy.api#Integer"
        );

        // Verify that the mixin member has the correct ID and tracks its mixin origin
        assert_eq!(
            effective_members["mixin_member"].id().to_string(),
            "example#Test$mixin_member"
        );
        // FIXME - mixins() returns &[Shape]
        // let x = effective_members["mixin_member"].mixins().iter().map(|s| s.id()).any(|s| *s == ShapeId::new_unchecked("example#Mixin$mixin_member"));
        assert!(effective_members["mixin_member"]
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin$mixin_member")));
    }

    #[test]
    fn test_compute_effective_members_with_multiple_mixins() {
        // Create first mixin structure
        let mixin1 = create_structure("example#Mixin1", vec![("foo", "smithy.api#String")]);

        // Create second mixin structure
        let mixin2 = create_structure("example#Mixin2", vec![("bar", "smithy.api#Integer")]);

        // Create a structure with local members
        let structure = create_structure("example#Test", vec![("baz", "smithy.api#Boolean")]);

        // Get the local members
        let members = Members::map(&structure.members);

        // Compute effective members
        let effective_members =
            compute_effective_members(structure.id(), &members, &[mixin1.into(), mixin2.into()])
                .unwrap();

        // Verify that the effective members include all members
        assert_eq!(effective_members.len(), 3);
        assert!(effective_members.contains_key("foo"));
        assert!(effective_members.contains_key("bar"));
        assert!(effective_members.contains_key("baz"));
        assert_eq!(
            effective_members["foo"].target().to_string(),
            "smithy.api#String"
        );
        assert_eq!(
            effective_members["bar"].target().to_string(),
            "smithy.api#Integer"
        );
        assert_eq!(
            effective_members["baz"].target().to_string(),
            "smithy.api#Boolean"
        );

        // Verify that the mixin members have the correct IDs and track their mixin origins
        assert_eq!(
            effective_members["foo"].id().to_string(),
            "example#Test$foo"
        );
        assert_eq!(
            effective_members["bar"].id().to_string(),
            "example#Test$bar"
        );
        assert!(effective_members["foo"]
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin1$foo")));
        assert!(effective_members["bar"]
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin2$bar")));
    }

    #[test]
    fn test_compute_effective_members_with_conflicting_mixins() {
        // Create first mixin structure
        let mixin1 = create_structure("example#Mixin1", vec![("conflict", "smithy.api#String")]);

        // Create second mixin structure with conflicting member
        let mixin2 = create_structure("example#Mixin2", vec![("conflict", "smithy.api#Integer")]);

        // Get the local members
        let structure = create_structure("example#Test", vec![]);
        let members = Members::map(&structure.members);

        // Compute effective members - should fail due to conflict
        let result =
            compute_effective_members(structure.id(), &members, &[mixin1.into(), mixin2.into()]);

        assert!(result.is_err());
        if let Err(BuildError::InvalidValue { field, reason }) = result {
            assert_eq!(field, "conflict");
            assert!(reason.contains("Mixin conflict: member 'conflict' from mixin 'example#Test' targets 'smithy.api#String', but the same member from mixin 'example#Mixin2' targets 'smithy.api#Integer'"), "reason={}", reason)
        } else {
            panic!("Expected InvalidValue error");
        }
    }

    #[test]
    fn test_compute_effective_members_with_local_conflict() {
        // Create mixin structure
        let mixin = create_structure("example#Mixin", vec![("conflict", "smithy.api#String")]);

        // Create structure with conflicting local member
        let mut structure_members = HashMap::new();
        let member = MemberShape::builder()
            .id("example#Test$conflict")
            .member_name("conflict")
            .target(ShapeId::new_unchecked("smithy.api#Integer"))
            .build()
            .unwrap();
        structure_members.insert("conflict".to_string(), member);

        // Get the local members
        let members = Members::map(&structure_members);

        // Compute effective members - should fail due to conflict
        let result = compute_effective_members(
            &ShapeId::new_unchecked("example#Test"),
            &members,
            &[mixin.into()],
        );

        assert!(result.is_err());
        if let Err(BuildError::InvalidValue { field, reason }) = result {
            assert_eq!(field, "conflict");
            assert!(reason.contains("Member conflict: member 'conflict' in shape 'example#Test' targets 'smithy.api#Integer', but the same member in mixin shape 'example#Mixin' targets 'smithy.api#String'"), "reason={}", reason);
        } else {
            panic!("Expected InvalidValue error");
        }
    }

    #[test]
    fn test_compute_effective_members_with_trait_inheritance() {
        // Create mixin structure with a member that has traits
        let mut mixin_members = Vec::new();
        let mixin_member = create_member_with_traits(
            "example#Mixin$member",
            "member",
            "smithy.api#String",
            vec![Box::new(Documentation("Mixin documentation".to_string()))],
        );
        mixin_members.push(mixin_member);

        let mixin = StructureShape::builder()
            .id("example#Mixin")
            .members(mixin_members)
            .build()
            .unwrap();

        // Create structure with local members
        let structure = create_structure("example#Test", vec![]);
        let members = Members::map(&structure.members);

        // Compute effective members
        let effective_members =
            compute_effective_members(structure.id(), &members, &[mixin.into()]).unwrap();

        // Verify that the effective member inherited the trait
        assert!(effective_members.contains_key("member"));
        let member = &effective_members["member"];
        assert!(member
            .traits()
            .contains_key(&ShapeId::new_unchecked("smithy.api#documentation")));

        if let Some(doc) = member.get_trait_as::<Documentation>() {
            assert_eq!(doc.0, "Mixin documentation");
        } else {
            panic!("Expected documentation trait");
        }
    }

    #[test]
    fn test_compute_effective_members_with_trait_precedence() {
        // Create mixin structure with a member that has traits
        let mut mixin_members = Vec::new();
        let mixin_member = create_member_with_traits(
            "example#Mixin$member",
            "member",
            "smithy.api#String",
            vec![
                Box::new(Documentation("Mixin documentation".to_string())),
                Box::new(Required),
            ],
        );
        mixin_members.push(mixin_member);

        let mixin = StructureShape::builder()
            .id("example#Mixin")
            .members(mixin_members)
            .build()
            .unwrap();

        // Create structure with local member that has a different documentation trait
        let mut structure_members = HashMap::new();
        let local_member = create_member_with_traits(
            "example#Test$member",
            "member",
            "smithy.api#String",
            vec![Box::new(Documentation("Local documentation".to_string()))],
        );
        structure_members.insert("member".to_string(), local_member);

        // Get the local members
        let members = Members::map(&structure_members);

        // Compute effective members
        let effective_members = compute_effective_members(
            &ShapeId::new_unchecked("example#Test"),
            &members,
            &[mixin.into()],
        )
        .unwrap();

        // Verify that the local trait takes precedence
        assert!(effective_members.contains_key("member"));
        let member = &effective_members["member"];

        if let Some(doc) = member.get_trait_as::<Documentation>() {
            assert_eq!(doc.0, "Local documentation");
        } else {
            panic!("Expected documentation trait");
        }

        // But the required trait from the mixin should still be present
        assert!(member.get_trait_as::<Required>().is_some());
    }

    #[test]
    fn test_compute_effective_members_with_multiple_mixin_traits() {
        // Create first mixin with a trait
        let mut mixin1_members = Vec::new();
        let mixin1_member = create_member_with_traits(
            "example#Mixin1$member",
            "member",
            "smithy.api#String",
            vec![Box::new(Documentation("Mixin1 documentation".to_string()))],
        );
        mixin1_members.push(mixin1_member);

        let mixin1 = StructureShape::builder()
            .id("example#Mixin1")
            .members(mixin1_members)
            .build()
            .unwrap();

        // Create second mixin with a different trait
        let mut mixin2_members = Vec::new();
        let mixin2_member = create_member_with_traits(
            "example#Mixin2$member",
            "member",
            "smithy.api#String",
            vec![Box::new(Required)],
        );
        mixin2_members.push(mixin2_member);

        let mixin2 = StructureShape::builder()
            .id("example#Mixin2")
            .members(mixin2_members)
            .build()
            .unwrap();

        // Create structure with no local members
        let structure = create_structure("example#Test", vec![]);
        let members = Members::map(&structure.members);

        // Compute effective members
        let effective_members =
            compute_effective_members(structure.id(), &members, &[mixin1.into(), mixin2.into()])
                .unwrap();

        // Verify that the member has traits from both mixins
        assert!(effective_members.contains_key("member"));
        let member = &effective_members["member"];

        assert!(member.has_trait(Documentation::static_id()));
        assert!(member.has_trait(Required::static_id()));

        // Verify that the member tracks both mixin origins
        assert!(member
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin1$member")));
        assert!(member
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin2$member")));
    }

    #[test]
    fn test_rebuild_member_mixins() {
        // Create mixins
        let mixin1 = create_structure("example#Mixin1", vec![("member", "smithy.api#String")]);

        let mixin2 = create_structure("example#Mixin2", vec![("member", "smithy.api#String")]);

        // Create a local member with no mixins
        let local_member = MemberShape::builder()
            .id("example#Test$member")
            .member_name("member")
            .target(ShapeId::new_unchecked("smithy.api#String"))
            .build()
            .unwrap();

        // Rebuild the member's mixins
        let rebuilt =
            rebuild_member_mixins(&local_member, &[mixin1.into(), mixin2.into()], "member")
                .unwrap();

        // Verify that the rebuilt member has both mixins
        assert_eq!(rebuilt.mixins().len(), 2);
        assert!(rebuilt
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin1$member")));
        assert!(rebuilt
            .mixins()
            .contains(&ShapeId::new_unchecked("example#Mixin2$member")));
    }
}
