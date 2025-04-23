# Implementation Plan

* Remember to use our workflow instructions to complete tasks.

## Current Focus: Mixin Implementation

* Reference the design in [mixin-design.md](mixin-design.md)
* Mixin spec (from the root of our rust project folder): `../docs/source-2.0/spec/mixins.rst`

### Mixin Implementation Tasks

#### Phase 1: Core Components

1. **MIXIN-001: Define the Mixin trait**
   - Create the `Mixin` struct in `traits/mod.rs`
   - Implement the `Trait` trait for `Mixin`
   - Add unit tests for the `Mixin` trait

2. **MIXIN-002: Update ShapeMetadata to support mixins**
   - Add `introduced_traits`, `effective_traits`, and `mixins` fields to `ShapeMetadata`
   - Update `ShapeMetadata::new` to initialize these fields
   - Add `add_mixin` method to `ShapeMetadata`

3. **MIXIN-003: Update HasTraits trait**
   - Add `introduced_traits` method to `HasTraits` trait
   - Update the blanket implementation to return the appropriate fields
   - Update existing code that uses `traits()` if necessary

4. **MIXIN-004: Add HasMixins trait**
   - Create the `HasMixins` trait in `shape.rs`
   - Add blanket implementation for types that implement `ProvideShapeMetadata`
   - Add unit tests for the `HasMixins` trait

5. **MIXIN-005: Add mixin utility to ShapeBuilderExt**
   - Add `mixin` method to `ShapeBuilderExt` trait
   - Add unit tests for the `mixin` method

#### Phase 2: Shape Builder Implementation

6. **MIXIN-006: Update StructureShapeBuilder for mixins**
   - Add `mixins` field to `StructureShapeBuilder`
   - Add `mixin` method to add a mixin to the builder
   - Update the `build` method to validate mixins
   - Add unit tests for the updated builder

7. **MIXIN-007: Update UnionShapeBuilder for mixins**
   - Add `mixins` field to `UnionShapeBuilder`
   - Add `mixin` method to add a mixin to the builder
   - Update the `build` method to validate mixins
   - Add unit tests for the updated builder

8. **MIXIN-008: Update ServiceShape and ServiceShapeBuilder for mixins**
   - Add `mixin` method to add a mixin to the builder
   - Add `introduced_version` field to the `ServiceShape`
   - Add `introduced_rename` field to the `ServiceShape`
   - Add `introduced_resources` field 
   - Add `introduced_operations` field
   - Add missing `error` method to add a common error to `ServiceShapeBuilder`
   - Add missing `errors` method to add multiple common errors to `ServiceShapeBuilder`
   - Add `introduced_errors` field to the `ServiceShape`
   - Update the `build` method to validate mixins
   - Add unit tests for the updated builder

9. **MIXIN-009: Update ResourceShapeBuilder for mixins**
   - Add `mixin` method to add a mixin to the builder
   - Update the `build` method to validate mixins
   - Add `introduced_resources` field to track resources applied directly vs effective (including mixins)
   - Add `introduced_operations` field to track operations applied directly vs effective (including mixins)
   - Add missing `collection_operations` field 
   - Add unit tests for the updated builder

10. **MIXIN-010: Update OperationShapeBuilder for mixins**
    - Add `mixin` method to add a mixin to the builder
    - Add `introduced_errors` field to `OperationShape` to track resources applied directly to the operation shape vs effective (including mixins)
    - Update the `build` method to validate mixins
      - Operation mixins cannot target anything other than the unit shape for input and output fields. 
    - Add unit tests for the updated builder

11. **MIXIN-011: Update simple shape builders for mixins**
    - Add mixin support to simple shape builders (String, Boolean, etc.)
    - Add unit tests for the updated builders

#### Phase 3: Mixin Resolution Functions

At this point we've mostly added support for being able to add mixins to a shape but we now have to fill in 
actually consuming the mixin in each shape (applying traits, coping members, etc). 

12. **MIXIN-012: Implement cycle detection**
    - Create a function to detect cycles in mixin references
    - Add unit tests for cycle detection

13. **MIXIN-013: Implement trait resolution**
    - Create a function to compute effective traits from mixins
    - Add unit tests for trait resolution

14. **MIXIN-014: Implement member resolution for structures**
    - Create a function to compute effective members for structures
    - Add unit tests for member resolution

15. **MIXIN-015: Implement member resolution for unions**
    - Create a function to compute effective members for unions
    - Add unit tests for member resolution

16. **MIXIN-016: Implement property resolution for services**
    - Create a function to merge service properties from mixins
    - Add unit tests for service property resolution

17. **MIXIN-017: Implement property resolution for operations**
    - Create a function to merge operation properties from mixins
    - Add unit tests for operation property resolution

#### Phase 4: Integration and Testing

18. **MIXIN-018: Add comprehensive integration tests**
    - Create tests for complex mixin scenarios
    - Test trait inheritance and precedence
    - Test member inheritance and conflicts
    - Test cycle detection

19. **MIXIN-019: Add documentation**
    - Update module documentation to explain mixin support
    - Add examples to the documentation
    - Update README if necessary

20. **MIXIN-020: Performance optimization**
    - Profile mixin resolution performance
    - Optimize if necessary
    - Add benchmarks for mixin resolution

#### Phase 5: Cleanup

21. **MIXIN-021: Look for opportunities to commonize**
    - Java uses an EntityShape and EntityShapeBuilder for shapes like service and resource that have common fields (operations, resources)
    - We are taking `Vec<T>` in our builders in several places (e.g. ResourceShapeBuilder::collection_operations) we should be consistent and use `impl Into<Iter<Item=ShapeId>>` or simiilar
    - Missing `to_builder()` for some of our shapes e.g. StructureShape and possibly others
