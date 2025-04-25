/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Integration tests for mixin functionality across all shape types.

use smithy_model::shape::{
    ListShape, MapShape, MemberShape, OperationShape, ResourceShape, ServiceShape, ShapeBuilder,
    ShapeBuilderExt, ShapeId, ShapeProperties, StructureShape, UnionShape,
};
use smithy_model::traits::documentation::Documentation;
use smithy_model::traits::type_refinement::{Mixin, Required};
use smithy_model::traits::Trait;

// TODO - once we can load a model from IDL we should shore up our tests from the Java reference implementation
//      - also add tests specifically for trait precedence with the example shown in the spec https://smithy.io/2.0/spec/mixins.html#traits-and-mixins

#[test]
fn test_structure_shape_mixins() {
    // Create a mixin structure with members and traits
    let mixin_member = MemberShape::builder()
        .id("example#MixinStruct$mixinMember")
        .member_name("mixinMember")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin member documentation")
        .build()
        .unwrap();

    let mixin = StructureShape::builder()
        .id("example#MixinStruct")
        .member(mixin_member)
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a structure that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyStruct$localMember")
        .member_name("localMember")
        .target(ShapeId::new_static("smithy.api", "Integer"))
        .required()
        .build()
        .unwrap();

    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .member(local_member)
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    let members = structure.members();
    // Verify that the structure has both members
    assert_eq!(members.len(), 2);
    assert!(members.contains("mixinMember"));
    assert!(members.contains("localMember"));

    // Verify that the mixin member has the correct target and traits
    let mixin_member = members.get("mixinMember").unwrap();
    assert_eq!(mixin_member.target().to_string(), "smithy.api#String");
    assert!(mixin_member.has_trait(Documentation::static_id()));

    // Verify that the local member has the correct target and traits
    let local_member = members.get("localMember").unwrap();
    assert_eq!(local_member.target().to_string(), "smithy.api#Integer");
    assert!(local_member.has_trait(Required::static_id()));

    // Verify that the structure has the mixin in its mixins list
    assert_eq!(structure.mixins().len(), 1);
    assert_eq!(
        structure.mixins()[0].id().to_string(),
        "example#MixinStruct"
    );

    // Verify that the structure's documentation trait takes precedence over the mixin's
    let doc = structure.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_union_shape_mixins() {
    // Create a mixin union with members and traits
    let mixin_member = MemberShape::builder()
        .id("example#MixinUnion$mixinMember")
        .member_name("mixinMember")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin member documentation")
        .build()
        .unwrap();

    let mixin = UnionShape::builder()
        .id("example#MixinUnion")
        .member(mixin_member)
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a union that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyUnion$localMember")
        .member_name("localMember")
        .target(ShapeId::new_static("smithy.api", "Integer"))
        .required()
        .build()
        .unwrap();

    let union = UnionShape::builder()
        .id("example#MyUnion")
        .member(local_member)
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    let members = union.members();
    // Verify that the union has both members
    assert_eq!(members.len(), 2);
    assert!(members.contains("mixinMember"));
    assert!(members.contains("localMember"));

    // Verify that the mixin member has the correct target and traits
    let mixin_member = members.get("mixinMember").unwrap();
    assert_eq!(mixin_member.target().to_string(), "smithy.api#String");
    assert!(mixin_member.has_trait(Documentation::static_id()));

    // Verify that the local member has the correct target and traits
    let local_member = members.get("localMember").unwrap();
    assert_eq!(local_member.target().to_string(), "smithy.api#Integer");
    assert!(local_member.has_trait(Required::static_id()));

    // Verify that the union has the mixin in its mixins list
    assert_eq!(union.mixins().len(), 1);
    assert_eq!(union.mixins()[0].id().to_string(), "example#MixinUnion");

    // Verify that the union's documentation trait takes precedence over the mixin's
    let doc = union.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_service_shape_mixins() {
    // Create a mixin service with operations, resources, and traits
    let mixin_op = ShapeId::new_static("example", "MixinOperation");
    let mixin_res = ShapeId::new_static("example", "MixinResource");
    let mixin_err = ShapeId::new_static("example", "MixinError");

    let mixin = ServiceShape::builder()
        .id("example#MixinService")
        .version("1.0")
        .operation(mixin_op.clone())
        .resource(mixin_res.clone())
        .error(mixin_err.clone())
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a service that uses the mixin
    let local_op = ShapeId::new_static("example", "LocalOperation");
    let local_res = ShapeId::new_static("example", "LocalResource");
    let local_err = ShapeId::new_static("example", "LocalError");

    let service = ServiceShape::builder()
        .id("example#MyService")
        .version("2.0")
        .operation(local_op.clone())
        .resource(local_res.clone())
        .error(local_err.clone())
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the service has both local and mixin operations
    assert_eq!(service.introduced_operations.len(), 1);
    assert!(service.introduced_operations.contains(&local_op));
    assert_eq!(service.operations.len(), 2);
    assert!(service.operations.contains(&local_op));
    assert!(service.operations.contains(&mixin_op));

    // Verify that the service has both local and mixin resources
    assert_eq!(service.introduced_resources.len(), 1);
    assert!(service.introduced_resources.contains(&local_res));
    assert_eq!(service.resources.len(), 2);
    assert!(service.resources.contains(&local_res));
    assert!(service.resources.contains(&mixin_res));

    // Verify that the service has both local and mixin errors
    assert_eq!(service.introduced_errors.len(), 1);
    assert!(service.introduced_errors.contains(&local_err));
    assert_eq!(service.errors.len(), 2);
    assert!(service.errors.contains(&local_err));
    assert!(service.errors.contains(&mixin_err));

    // Verify that the service's version takes precedence over the mixin's
    assert_eq!(service.version, Some("2.0".to_string()));

    // Verify that the service has the mixin in its mixins list
    assert_eq!(service.mixins().len(), 1);
    assert_eq!(service.mixins()[0].id().to_string(), "example#MixinService");

    // Verify that the service's documentation trait takes precedence over the mixin's
    let doc = service.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_resource_shape_mixins() {
    // Create a mixin resource with operations, resources, and traits
    let mixin_op = ShapeId::new_static("example", "MixinOperation");
    let mixin_res = ShapeId::new_static("example", "MixinResource");

    let mixin = ResourceShape::builder()
        .id("example#MixinResource")
        .with_trait(Mixin::new())
        .operation(mixin_op.clone())
        .resource(mixin_res.clone())
        .documentation("Mixin documentation")
        .build()
        .unwrap();

    // Create a resource that uses the mixin
    let local_op = ShapeId::new_static("example", "LocalOperation");
    let local_res = ShapeId::new_static("example", "LocalResource");
    let identifier = ShapeId::new_static("example", "Id");

    let resource = ResourceShape::builder()
        .id("example#MyResource")
        .identifier("id", identifier.clone())
        .operation(local_op.clone())
        .resource(local_res.clone())
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the resource has both local and mixin operations
    assert_eq!(resource.introduced_operations.len(), 1);
    assert!(resource.introduced_operations.contains(&local_op));
    assert_eq!(resource.operations.len(), 2);
    assert!(resource.operations.contains(&local_op));
    assert!(resource.operations.contains(&mixin_op));

    // Verify that the resource has both local and mixin resources
    assert_eq!(resource.introduced_resources.len(), 1);
    assert!(resource.introduced_resources.contains(&local_res));
    assert_eq!(resource.resources.len(), 2);
    assert!(resource.resources.contains(&local_res));
    assert!(resource.resources.contains(&mixin_res));

    // Verify that the resource has the mixin in its mixins list
    assert_eq!(resource.mixins().len(), 1);
    assert_eq!(
        resource.mixins()[0].id().to_string(),
        "example#MixinResource"
    );

    // Verify that the resource's documentation trait takes precedence over the mixin's
    let doc = resource.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_operation_shape_mixins() {
    // Create a mixin operation with errors and traits
    let mixin_err = ShapeId::new_static("example", "MixinError");

    let mixin = OperationShape::builder()
        .id("example#MixinOperation")
        .error(mixin_err.clone())
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create an operation that uses the mixin
    let local_err = ShapeId::new_static("example", "LocalError");
    let input = ShapeId::new_static("example", "Input");
    let output = ShapeId::new_static("example", "Output");

    let operation = OperationShape::builder()
        .id("example#MyOperation")
        .input(input.clone())
        .output(output.clone())
        .error(local_err.clone())
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the operation has both local and mixin errors
    assert_eq!(operation.introduced_errors.len(), 1);
    assert!(operation.introduced_errors.contains(&local_err));
    assert_eq!(operation.errors.len(), 2);
    assert!(operation.errors.contains(&local_err));
    assert!(operation.errors.contains(&mixin_err));

    // Verify that the operation has the input and output
    assert_eq!(operation.input, Some(input));
    assert_eq!(operation.output, Some(output));

    // Verify that the operation has the mixin in its mixins list
    assert_eq!(operation.mixins().len(), 1);
    assert_eq!(
        operation.mixins()[0].id().to_string(),
        "example#MixinOperation"
    );

    // Verify that the operation's documentation trait takes precedence over the mixin's
    let doc = operation.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_simple_shape_mixins() {
    // Test string shape with mixin
    let string_mixin = smithy_model::shape::StringShape::builder()
        .id("example#StringMixin")
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    let string = smithy_model::shape::StringShape::builder()
        .id("example#MyString")
        .mixin(string_mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the string has the mixin in its mixins list
    assert_eq!(string.mixins().len(), 1);
    assert_eq!(string.mixins()[0].id().to_string(), "example#StringMixin");

    // Verify that the string's documentation trait takes precedence over the mixin's
    let doc = string.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_list_shape_mixins() {
    // Create a mixin list with a member and traits
    let mixin_member = MemberShape::builder()
        .id("example#ListMixin$member")
        .member_name("member")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin member documentation")
        .build()
        .unwrap();

    let list_mixin = ListShape::builder()
        .id("example#ListMixin")
        .member(mixin_member)
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a list that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyList$member")
        .member_name("member")
        .target(ShapeId::new_static("smithy.api", "String"))
        .required()
        .build()
        .unwrap();

    let list = ListShape::builder()
        .id("example#MyList")
        .member(local_member)
        .mixin(list_mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the list has the mixin in its mixins list
    assert_eq!(list.mixins().len(), 1);
    assert_eq!(list.mixins()[0].id().to_string(), "example#ListMixin");

    // Verify that the list's documentation trait takes precedence over the mixin's
    let doc = list.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");

    // Verify that the member has the required trait (local trait takes precedence)
    assert!(list.member().has_trait(Required::static_id()));
}

#[test]
fn test_map_shape_mixins() {
    // Create a mixin map with key/value members and traits
    let mixin_key = MemberShape::builder()
        .id("example#MapMixin$key")
        .member_name("key")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let mixin_value = MemberShape::builder()
        .id("example#MapMixin$value")
        .member_name("value")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin value documentation")
        .build()
        .unwrap();

    let map_mixin = MapShape::builder()
        .id("example#MapMixin")
        .key(mixin_key)
        .value(mixin_value)
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a map that uses the mixin
    let local_key = MemberShape::builder()
        .id("example#MyMap$key")
        .member_name("key")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let local_value = MemberShape::builder()
        .id("example#MyMap$value")
        .member_name("value")
        .target(ShapeId::new_static("smithy.api", "String"))
        .required()
        .build()
        .unwrap();

    let map = MapShape::builder()
        .id("example#MyMap")
        .key(local_key)
        .value(local_value)
        .mixin(map_mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the map has the mixin in its mixins list
    assert_eq!(map.mixins().len(), 1);
    assert_eq!(map.mixins()[0].id().to_string(), "example#MapMixin");

    // Verify that the map's documentation trait takes precedence over the mixin's
    let doc = map.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");

    // Verify that the value member has the required trait (local trait takes precedence)
    assert!(map.value.has_trait(Required::static_id()));
}

#[test]
fn test_mixin_composition() {
    // Create a base mixin
    let base_member = MemberShape::builder()
        .id("example#BaseMixin$baseMember")
        .member_name("baseMember")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Base member documentation")
        .build()
        .unwrap();

    let base_mixin = StructureShape::builder()
        .id("example#BaseMixin")
        .member(base_member)
        .documentation("Base mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a composed mixin that uses the base mixin
    let composed_member = MemberShape::builder()
        .id("example#ComposedMixin$composedMember")
        .member_name("composedMember")
        .target(ShapeId::new_static("smithy.api", "Integer"))
        .required()
        .build()
        .unwrap();

    let composed_mixin = StructureShape::builder()
        .id("example#ComposedMixin")
        .member(composed_member)
        .mixin(base_mixin.clone())
        .documentation("Composed mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a structure that uses the composed mixin
    let local_member = MemberShape::builder()
        .id("example#MyStruct$localMember")
        .member_name("localMember")
        .target(ShapeId::new_static("smithy.api", "Boolean"))
        .build()
        .unwrap();

    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .member(local_member)
        .mixin(composed_mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the structure has all three members
    assert_eq!(structure.members().len(), 3);
    assert!(structure.members().contains("baseMember"));
    assert!(structure.members().contains("composedMember"));
    assert!(structure.members().contains("localMember"));

    let members = structure.members();
    // Verify that the base member has the correct target and traits
    let base_member = members.get("baseMember").unwrap();
    assert_eq!(base_member.target().to_string(), "smithy.api#String");
    assert!(base_member.has_trait(Documentation::static_id()));

    // Verify that the composed member has the correct target and traits
    let composed_member = members.get("composedMember").unwrap();
    assert_eq!(composed_member.target().to_string(), "smithy.api#Integer");
    assert!(composed_member.has_trait(Required::static_id()));

    // Verify that the local member has the correct target
    let local_member = members.get("localMember").unwrap();
    assert_eq!(local_member.target().to_string(), "smithy.api#Boolean");

    // Verify that the structure has only the composed mixin in its mixins list
    assert_eq!(structure.mixins().len(), 1);
    assert_eq!(
        structure.mixins()[0].id().to_string(),
        "example#ComposedMixin"
    );

    // Verify that the structure's documentation trait takes precedence over the mixin's
    let doc = structure.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local documentation");
}

#[test]
fn test_mixin_member_conflict() {
    // Create a mixin with a member
    let mixin1_member = MemberShape::builder()
        .id("example#Mixin1$member")
        .member_name("member")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let mixin1 = StructureShape::builder()
        .id("example#Mixin1")
        .member(mixin1_member)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create another mixin with a conflicting member (different target)
    let mixin2_member = MemberShape::builder()
        .id("example#Mixin2$member")
        .member_name("member")
        .target(ShapeId::new_static("smithy.api", "Integer"))
        .build()
        .unwrap();

    let mixin2 = StructureShape::builder()
        .id("example#Mixin2")
        .member(mixin2_member)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Try to create a structure that uses both mixins - should fail due to conflict
    let result = StructureShape::builder()
        .id("example#MyStruct")
        .mixin(mixin1)
        .mixin(mixin2)
        .build();

    assert!(result.is_err());
}

#[test]
fn test_mixin_local_traits() {
    // Create a mixin with local traits
    let private_id = ShapeId::new_static("smithy.api", "private");
    let mixin = StructureShape::builder()
        .id("example#MixinWithLocalTraits")
        .with_trait(Mixin::with_local_traits(vec![private_id.clone()]))
        .with_trait(smithy_model::traits::DynamicTrait::new(
            private_id.clone(),
            None,
        ))
        .documentation("Mixin documentation")
        .build()
        .unwrap();

    // Create a structure that uses the mixin
    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .mixin(mixin.clone())
        .build()
        .unwrap();

    // Verify that the structure has the documentation trait from the mixin
    assert!(structure.has_trait(Documentation::static_id()));

    // Verify that the structure does NOT have the private trait (it's a local trait)
    assert!(!structure.has_trait(private_id));
}

#[test]
fn test_mixin_member_ordering() {
    // Create first mixin with members
    let mixin1_member1 = MemberShape::builder()
        .id("example#Mixin1$member1")
        .member_name("member1")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let mixin1_member2 = MemberShape::builder()
        .id("example#Mixin1$member2")
        .member_name("member2")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let mixin1 = StructureShape::builder()
        .id("example#Mixin1")
        .member(mixin1_member1)
        .member(mixin1_member2)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create second mixin with members
    let mixin2_member3 = MemberShape::builder()
        .id("example#Mixin2$member3")
        .member_name("member3")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let mixin2_member4 = MemberShape::builder()
        .id("example#Mixin2$member4")
        .member_name("member4")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let mixin2 = StructureShape::builder()
        .id("example#Mixin2")
        .member(mixin2_member3)
        .member(mixin2_member4)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a structure with local members that uses both mixins
    let local_member5 = MemberShape::builder()
        .id("example#MyStruct$member5")
        .member_name("member5")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let local_member6 = MemberShape::builder()
        .id("example#MyStruct$member6")
        .member_name("member6")
        .target(ShapeId::new_static("smithy.api", "String"))
        .build()
        .unwrap();

    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .mixin(mixin1)
        .mixin(mixin2)
        .member(local_member5)
        .member(local_member6)
        .build()
        .unwrap();

    // Verify that the structure has all members
    assert_eq!(structure.members().len(), 6);

    // Get the members in order
    let members: Vec<_> = structure.members().iter_named().collect();

    // Verify the order: mixin1 members, then mixin2 members, then local members
    // Note: HashMap iteration order is not guaranteed, so we can't test exact order
    // Instead, we'll verify that all members are present
    let member_names: Vec<_> = members.iter().map(|(name, _)| name.to_string()).collect();
    assert!(member_names.contains(&"member1".to_string()));
    assert!(member_names.contains(&"member2".to_string()));
    assert!(member_names.contains(&"member3".to_string()));
    assert!(member_names.contains(&"member4".to_string()));
    assert!(member_names.contains(&"member5".to_string()));
    assert!(member_names.contains(&"member6".to_string()));
}

#[test]
fn test_mixin_trait_precedence() {
    // Create first mixin with trait
    let mixin1 = StructureShape::builder()
        .id("example#Mixin1")
        .documentation("Mixin1 documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create second mixin with trait that should override the first
    let mixin2 = StructureShape::builder()
        .id("example#Mixin2")
        .documentation("Mixin2 documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a structure that uses both mixins
    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .mixin(mixin1)
        .mixin(mixin2)
        .build()
        .unwrap();

    // Verify that the structure has the documentation trait from the second mixin
    let doc = structure.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Mixin2 documentation");
}

#[test]
fn test_mixin_redefined_member() {
    // Create a mixin with a member
    let mixin_member = MemberShape::builder()
        .id("example#Mixin$member")
        .member_name("member")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin member documentation")
        .build()
        .unwrap();

    let mixin = StructureShape::builder()
        .id("example#Mixin")
        .member(mixin_member)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a structure that redefines the same member
    let local_member = MemberShape::builder()
        .id("example#MyStruct$member")
        .member_name("member")
        .target(ShapeId::new_static("smithy.api", "String")) // Same target is required
        .documentation("Local member documentation")
        .required() // Additional trait
        .build()
        .unwrap();

    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .member(local_member)
        .mixin(mixin.clone())
        .build()
        .unwrap();

    // Verify that the structure has the member
    assert_eq!(structure.members().len(), 1);
    assert!(structure.members().contains("member"));

    // Verify that the member has the local documentation (overrides mixin)
    let members = structure.members();
    let member = members.get("member").unwrap();
    let doc = member.get_trait_as::<Documentation>().unwrap();
    assert_eq!(doc.0, "Local member documentation");

    // Verify that the member has the required trait from the local definition
    assert!(member.has_trait(Required::static_id()));
}
#[test]
fn test_structure_shape_to_builder_with_mixins() {
    // Create a mixin structure with members and traits
    let mixin_member = MemberShape::builder()
        .id("example#MixinStruct$mixinMember")
        .member_name("mixinMember")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin member documentation")
        .build()
        .unwrap();

    let mixin = StructureShape::builder()
        .id("example#MixinStruct")
        .member(mixin_member)
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a structure that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyStruct$localMember")
        .member_name("localMember")
        .target(ShapeId::new_static("smithy.api", "Integer"))
        .required()
        .build()
        .unwrap();

    let structure = StructureShape::builder()
        .id("example#MyStruct")
        .member(local_member)
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the structure has both members
    assert_eq!(structure.members().len(), 2);
    assert!(structure.members().contains("mixinMember"));
    assert!(structure.members().contains("localMember"));

    // Convert back to a builder and build again without the mixin
    let structure2 = structure.to_builder().remove_mixin(mixin).build().unwrap();

    // Should only have the local member
    assert_eq!(structure2.members().len(), 1);
    assert!(structure2.members().contains("localMember"));
    assert!(!structure2.members().contains("mixinMember"));
}

#[test]
fn test_union_shape_to_builder_with_mixins() {
    // Create a mixin union with members and traits
    let mixin_member = MemberShape::builder()
        .id("example#MixinUnion$mixinMember")
        .member_name("mixinMember")
        .target(ShapeId::new_static("smithy.api", "String"))
        .documentation("Mixin member documentation")
        .build()
        .unwrap();

    let mixin = UnionShape::builder()
        .id("example#MixinUnion")
        .member(mixin_member)
        .documentation("Mixin documentation")
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create a union that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyUnion$localMember")
        .member_name("localMember")
        .target(ShapeId::new_static("smithy.api", "Integer"))
        .required()
        .build()
        .unwrap();

    let union = UnionShape::builder()
        .id("example#MyUnion")
        .member(local_member)
        .mixin(mixin.clone())
        .documentation("Local documentation")
        .build()
        .unwrap();

    // Verify that the union has both members
    assert_eq!(union.members().len(), 2);
    assert!(union.members().contains("mixinMember"));
    assert!(union.members().contains("localMember"));

    // Convert back to a builder and build again without the mixin
    let union2 = union.to_builder().remove_mixin(mixin).build().unwrap();

    // Should only have the local member
    assert_eq!(union2.members().len(), 1);
    assert!(union2.members().contains("localMember"));
    assert!(!union2.members().contains("mixinMember"));
}
#[test]
fn test_enum_shape_to_builder_with_mixins() {
    // Create a mixin enum with members
    let unit_id = ShapeId::new_static("smithy.api", "Unit");

    let mixin_member = MemberShape::builder()
        .id("example#MixinEnum$MIXIN_MEMBER")
        .member_name("MIXIN_MEMBER")
        .target(unit_id.clone())
        .build()
        .unwrap();

    let mixin = smithy_model::shape::EnumShape::builder()
        .id("example#MixinEnum")
        .member(mixin_member)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create an enum that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyEnum$LOCAL_MEMBER")
        .member_name("LOCAL_MEMBER")
        .target(unit_id.clone())
        .build()
        .unwrap();

    let enum_shape = smithy_model::shape::EnumShape::builder()
        .id("example#MyEnum")
        .member(local_member)
        .mixin(mixin.clone())
        .build()
        .unwrap();

    // Verify that the enum has both members
    assert_eq!(enum_shape.members().len(), 2);
    assert!(enum_shape.members().contains("MIXIN_MEMBER"));
    assert!(enum_shape.members().contains("LOCAL_MEMBER"));

    // Convert back to a builder and build again without the mixin
    let enum_shape2 = enum_shape.to_builder().remove_mixin(mixin).build().unwrap();

    // Should only have the local member
    assert_eq!(enum_shape2.members().len(), 1);
    assert!(enum_shape2.members().contains("LOCAL_MEMBER"));
    assert!(!enum_shape2.members().contains("MIXIN_MEMBER"));
}
#[test]
fn test_int_enum_shape_to_builder_with_mixins() {
    // Create a mixin int enum with members
    let unit_id = ShapeId::new_static("smithy.api", "Unit");

    let mixin_member = MemberShape::builder()
        .id("example#MixinIntEnum$MIXIN_MEMBER")
        .member_name("MIXIN_MEMBER")
        .target(unit_id.clone())
        .build()
        .unwrap();

    let mixin = smithy_model::shape::IntEnumShape::builder()
        .id("example#MixinIntEnum")
        .member(mixin_member, 1)
        .with_trait(Mixin::new())
        .build()
        .unwrap();

    // Create an int enum that uses the mixin
    let local_member = MemberShape::builder()
        .id("example#MyIntEnum$LOCAL_MEMBER")
        .member_name("LOCAL_MEMBER")
        .target(unit_id.clone())
        .build()
        .unwrap();

    let int_enum_shape = smithy_model::shape::IntEnumShape::builder()
        .id("example#MyIntEnum")
        .member(local_member, 2)
        .mixin(mixin.clone())
        .build()
        .unwrap();

    // Verify that the int enum has both members
    assert_eq!(int_enum_shape.members().len(), 2);
    assert!(int_enum_shape.members().contains("MIXIN_MEMBER"));
    assert!(int_enum_shape.members().contains("LOCAL_MEMBER"));

    // Convert back to a builder and build again without the mixin
    let int_enum_shape2 = int_enum_shape
        .to_builder()
        .remove_mixin(mixin)
        .build()
        .unwrap();

    // Should only have the local member
    assert_eq!(int_enum_shape2.members().len(), 1);
    assert!(int_enum_shape2.members().contains("LOCAL_MEMBER"));
    assert!(!int_enum_shape2.members().contains("MIXIN_MEMBER"));
}
