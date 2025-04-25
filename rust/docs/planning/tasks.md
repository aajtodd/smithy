# Implementation Tasks

This document outlines the tasks for implementing features and improvements in the Smithy Rust project.

## Current Focus: Member ID Validation and Mixin Member Separation

### Implementation Approach

#### Member ID Validation

We'll implement a utility function to validate that member shape IDs follow the format `parentShapeId$memberName`. This function will be used in the `build()` method of all shape builders that have members.

```rust
/// Validates that member shape IDs belong to the parent shape
pub(crate) fn validate_member_shape_ids(
  shape_id: &ShapeId,
  members: Members<'_>
) -> Result<(), BuildError> {
  for (name, member) in members.iter_named() {
    let member_id = member.id();
    if member_id.namespace() != shape_id.namespace() || member_id.name() != shape_id.name() {
      return Err(BuildError::InvalidValue {
        field: name.to_string(),
        reason: format!(
          "Expected the `{name}` member of `{shape_id}` to have an ID of `{shape_id}${name}` but found `{member_id}`",
          name = name,
          shape_id = shape_id,
          member_id = member.id(),
        ),
      });
    }
  }
  Ok(())
}
```

#### Mixin Member Separation

We'll implement a utility function to separate members that should be included in a builder from those inherited from mixins. This function will be used in the `to_builder()` method of all shapes with members.

```rust
/// Separates members into those that should be included in a builder and those that shouldn't.
/// 
/// Members are included in the builder if:
/// 1. They weren't inherited from mixins, OR
/// 2. They have locally introduced traits
///
/// Returns a new IndexMap containing only the members that should be included in the builder.
/// If no members were inherited from mixins, returns the original map to avoid unnecessary copying.
pub fn separate_mixin_members(members: IndexMap<String, MemberShape>) -> IndexMap<String, MemberShape> {
    // Quick check: if there are no members with mixins, we can return the original map
    if members.values().all(|member| member.mixins().is_empty()) {
        return members;
    }
    
    // Otherwise, we need to filter out members that were inherited from mixins and have no local traits
    let mut builder_members = IndexMap::new();
    
    for (name, member) in members {
        if member.mixins().is_empty() || !member.introduced_traits().is_empty() {
            builder_members.insert(name, member);
        }
    }
    
    builder_members
}
```

#### Example Usage in Shape Builders

For the `build()` method:

```rust
pub fn build(self) -> Result<StructureShape, BuildError> {
    // Validate mixins before building
    self.metadata
        .validate_mixins(|shape| matches!(shape, Shape::Structure(_)), "structure")?;

    // Build metadata first to get a valid ShapeId
    let metadata = self.metadata.build()?;

    // Validate member shape IDs
    builder::validate_member_shape_ids(&metadata.id, &self.members)?;

    // Compute effective members
    let members = mixin::compute_effective_members(self.members, &metadata)?;

    Ok(StructureShape { 
        metadata, 
        members,
    })
}
```

For the `to_builder()` method:

```rust
pub fn to_builder(self) -> StructureShapeBuilder {
    StructureShapeBuilder {
        metadata: self.metadata.to_builder(),
        members: mixin::separate_mixin_members(self.members),
    }
}
```

### Utility Functions

- [x] **UTIL-1**: Add `validate_member_shape_ids` function to `shape/builder.rs`
  - Validates that member shape IDs follow the format `parentShapeId$memberName`
  - Returns a `BuildError` with a descriptive message if validation fails
  - Consider implementing a version that works with the `Members` container

- [x] **UTIL-2**: Add `separate_mixin_members` function to `shape/mixin.rs`
  - Separates members that should be included in a builder from those inherited from mixins
  - Includes members that weren't inherited from mixins or have locally introduced traits
  - Optimizes by returning the original map if no filtering is needed

### Shape Builder Updates

- [x] **BUILD-1**: Update `StructureShapeBuilder.build()` to validate member IDs
  - Build metadata first to get a valid ShapeId
  - Call `validate_member_shape_ids` with `shape.members()`

- [x] **BUILD-2**: Update `UnionShapeBuilder.build()` to validate member IDs
  - Follow the same pattern as `StructureShapeBuilder.build()`

- [x] **BUILD-3**: Update `EnumShapeBuilder.build()` to validate member IDs
  - Follow the same pattern as `StructureShapeBuilder.build()`

- [x] **BUILD-4**: Update `IntEnumShapeBuilder.build()` to validate member IDs
  - Follow the same pattern as `StructureShapeBuilder.build()`

- [x] **BUILD-5**: Update `ListShapeBuilder.build()` to validate its single member ID
  - Follow the same pattern as `StructureShapeBuilder.build()`

- [x] **BUILD-6**: Update `SetShapeBuilder.build()` to validate its single member ID
  - Follow the same pattern as `StructureShapeBuilder.build()`

- [x] **BUILD-7**: Update `MapShapeBuilder.build()` to validate its key and value member IDs
  - Follow the same pattern as `StructureShapeBuilder.build()`

### to_builder() Method Updates

- [x] **TOBUILDER-1**: Update `StructureShape.to_builder()` to use `separate_mixin_members`
  - Replace the current implementation with one that uses the utility function

- [x] **TOBUILDER-2**: Update `UnionShape.to_builder()` to use `separate_mixin_members`
  - Follow the same pattern as `StructureShape.to_builder()`

- [x] **TOBUILDER-3**: Update `EnumShape.to_builder()` to use `separate_mixin_members`
  - Follow the same pattern as `StructureShape.to_builder()`

- [x] **TOBUILDER-4**: Update `IntEnumShape.to_builder()` to use `separate_mixin_members`
  - Follow the same pattern as `StructureShape.to_builder()`

### Tests

- [ ] **TEST-3**: Add test for `StructureShape.to_builder()` with mixins in `mixin_test.rs`
  - Test that local members are preserved
  - Test that mixin members are excluded
  - Example test:

    ```rust
    #[test]
    fn test_structure_shape_to_builder_with_mixins() {
        // Create a mixin structure with members and traits
        let mixin_member = MemberShape::builder()
            .id("example#MixinStruct$mixinMember")
            .member_name("mixinMember")
            .target(ShapeId::new_unchecked("smithy.api#String"))
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
            .target(ShapeId::new_unchecked("smithy.api#Integer"))
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

        // Convert back to a builder
        let structure_builder = structure.to_builder();

        // The builder should only have the local member
        assert_eq!(structure_builder.members.len(), 1);
        assert!(!structure_builder.members.contains_key("mixinMember"));
        assert!(structure_builder.members.contains_key("localMember"));

        // Build again after removing the mixin
        let structure2 = structure_builder.remove_mixin(mixin).build().unwrap();
        
        // Should only have the local member
        assert_eq!(structure2.members().len(), 1);
        assert!(structure2.members().contains("localMember"));
        assert!(!structure2.members().contains("mixinMember"));
    }
    ```

- [ ] **TEST-4**: Add test for `UnionShape.to_builder()` with mixins in `mixin_test.rs`
  - Follow the same pattern as the `StructureShape` test

- [ ] **TEST-5**: Add test for `EnumShape.to_builder()` with mixins in `mixin_test.rs`
  - Follow the same pattern as the `StructureShape` test

- [ ] **TEST-6**: Add test for `IntEnumShape.to_builder()` with mixins in `mixin_test.rs`
  - Follow the same pattern as the `StructureShape` test

## Implementation Notes

- Each task is designed to be small and focused, allowing for incremental implementation
- Tasks should be completed in order, as later tasks may depend on earlier ones
- After completing each task, run tests to ensure everything works correctly
- Update this document as tasks are completed or if new tasks are identified

## Completed Tasks

- [x] **MIXIN-1**: Implement basic mixin support in `ShapeMetadata`
- [x] **MIXIN-2**: Implement `compute_effective_traits` function
- [x] **MIXIN-3**: Implement `compute_effective_members` function
- [x] **MIXIN-4**: Add validation for mixin shape types
- [x] **MIXIN-5**: Consolidate shape traits into `ShapeProperties`
- [x] **MIXIN-6**: Enhance builder pattern for mixins
- [x] **MIXIN-7**: Create `define_trait` macro for simple traits
