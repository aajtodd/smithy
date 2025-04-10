# Smithy Shape Implementation Tasks

This document outlines the tasks required to implement our updated shape design as described in `shape-design.md`.

## Overview

We need to refactor the current shape implementation to move from a design where shape ID and traits are stored in the parent `Shape` struct to a design where each shape variant stores its own metadata. We'll also implement a builder pattern for creating shapes.

## Current Implementation

The current implementation in `shape.rs` uses:
- A `Shape` struct with `id`, `traits`, and `kind` fields
- A `ShapeKind` enum for different shape types
- Shape-specific data stored in the enum variants
- Helper methods on `Shape` for accessing shape-specific data

## Target Implementation

Our target implementation will:
- Use a `Shape` enum with variants for each shape type
- Store shape ID and traits in each shape variant via a `ShapeMetadata` struct
- Provide common functionality through traits (`HasShapeId`, `HasTraits`)
- Use a private `ProvideShapeMetadata` trait with blanket implementations
- Implement a builder pattern for creating shapes

## Tasks

### Task SHAPE-001: Define Core Traits and Metadata

**Description**: Define the core traits and metadata structure for the new shape design.

**Steps**:
1. Define the `HasShapeId` trait
2. Define the `HasTraits` trait with default implementations
3. Define the private `ProvideShapeMetadata` trait
4. Implement the `ShapeMetadata` struct
5. Create blanket implementations of `HasShapeId` and `HasTraits` for types that implement `ProvideShapeMetadata`

**Acceptance Criteria**:
- All traits and structs are properly defined
- Blanket implementations work correctly
- Documentation is complete

### Task SHAPE-002: Implement Shape Enum and Basic Variants

**Description**: Refactor the `Shape` from a struct to an enum and implement basic shape variants.

**Steps**:
1. Define the `Shape` enum with variants for all shape types
2. Implement `ProvideShapeMetadata` for `Shape` by delegating to variants
3. Implement basic shape structs (Boolean, String, etc.) with `ShapeMetadata`
4. Implement `ProvideShapeMetadata` for each shape struct
5. Implement `From` traits for converting shape structs to `Shape` enum

**Acceptance Criteria**:
- `Shape` is properly defined as an enum
- Basic shape variants are implemented
- All shapes implement `ProvideShapeMetadata`
- `From` traits work correctly

### Task SHAPE-003: Implement Aggregate Shape Types

**Description**: Implement aggregate shape types (Structure, Union, List, Map, Set).

**Steps**:
1. Define the `StructureShape`, `UnionShape`, `ListShape`, `MapShape`, and `SetShape` structs
2. Update these structs to store `MemberShape` directly instead of `ShapeId`
3. Implement `ProvideShapeMetadata` for each shape
4. Implement constructors and accessors for each shape
5. Implement `From` traits for converting to `Shape` enum

**Acceptance Criteria**:
- All aggregate shapes are properly implemented
- Member shapes are stored directly in aggregate shapes
- All shapes implement `ProvideShapeMetadata`
- `From` traits work correctly

### Task SHAPE-004: Implement Service Shape Types

**Description**: Implement service shape types (Service, Operation, Resource).

**Steps**:
1. Define the `ServiceShape`, `OperationShape`, and `ResourceShape` structs
2. Implement `ProvideShapeMetadata` for each shape
3. Implement constructors and accessors for each shape
4. Implement `From` traits for converting to `Shape` enum

**Acceptance Criteria**:
- All service shapes are properly implemented
- All shapes implement `ProvideShapeMetadata`
- `From` traits work correctly

### Task SHAPE-005: Implement Enum and IntEnum Shapes

**Description**: Implement enum and integer enum shape types.

**Steps**:
1. Define the `EnumShape` and `IntEnumShape` structs
2. Update these structs to store `MemberShape` directly
3. Implement `ProvideShapeMetadata` for each shape
4. Implement constructors and accessors for each shape
5. Implement `From` traits for converting to `Shape` enum

**Acceptance Criteria**:
- Enum shapes are properly implemented
- Member shapes are stored directly in enum shapes
- All shapes implement `ProvideShapeMetadata`
- `From` traits work correctly

### Task SHAPE-006: Implement Shape Type Checking and Conversion Methods

**Description**: Implement methods for checking shape types and converting between them.

**Steps**:
1. Implement type checking methods (`is_structure`, `is_list`, etc.)
2. Implement type conversion methods (`as_structure`, `as_list`, etc.)
3. Implement expect methods (`expect_structure`, `expect_list`, etc.)

**Acceptance Criteria**:
- All type checking methods work correctly
- All type conversion methods work correctly
- All expect methods work correctly and provide good error messages

### Task SHAPE-007: Define Builder Error Type

**Description**: Define the error type for shape builders.

**Steps**:
1. Define the `BuildError` enum with variants for different error types
2. Implement `std::error::Error` and `std::fmt::Display` for `BuildError`
3. Implement conversion from other error types to `BuildError`

**Acceptance Criteria**:
- `BuildError` is properly defined
- Error messages are clear and helpful
- Conversion from other error types works correctly

### Task SHAPE-008: Implement Builder Traits

**Description**: Implement the traits needed for the builder pattern.

**Steps**:
1. Define the `ProvideTraitsMut` trait
2. Define the `ShapeBuilderExt` trait with common trait methods
3. Implement blanket implementation of `ShapeBuilderExt` for types that implement `ProvideTraitsMut`

**Acceptance Criteria**:
- All traits are properly defined
- Common trait methods work correctly
- Blanket implementation works correctly

### Task SHAPE-009: Implement Basic Shape Builders

**Description**: Implement builders for basic shape types.

**Steps**:
1. Implement builders for simple shapes (Boolean, String, etc.)
2. Implement `ProvideTraitsMut` for each builder
3. Implement builder methods for each shape
4. Implement `build` methods that return `Result<Shape, BuildError>`

**Acceptance Criteria**:
- All basic shape builders are properly implemented
- All builders implement `ProvideTraitsMut`
- Builder methods work correctly
- `build` methods validate inputs and return appropriate errors

### Task SHAPE-010: Implement Aggregate Shape Builders

**Description**: Implement builders for aggregate shape types.

**Steps**:
1. Implement builders for aggregate shapes (Structure, Union, List, Map, Set)
2. Implement `ProvideTraitsMut` for each builder
3. Implement builder methods for each shape
4. Implement `build` methods that return `Result<Shape, BuildError>`

**Acceptance Criteria**:
- All aggregate shape builders are properly implemented
- All builders implement `ProvideTraitsMut`
- Builder methods work correctly
- `build` methods validate inputs and return appropriate errors

### Task SHAPE-011: Implement Service Shape Builders

**Description**: Implement builders for service shape types.

**Steps**:
1. Implement builders for service shapes (Service, Operation, Resource)
2. Implement `ProvideTraitsMut` for each builder
3. Implement builder methods for each shape
4. Implement `build` methods that return `Result<Shape, BuildError>`

**Acceptance Criteria**:
- All service shape builders are properly implemented
- All builders implement `ProvideTraitsMut`
- Builder methods work correctly
- `build` methods validate inputs and return appropriate errors

### Task SHAPE-012: Implement Enum Shape Builders

**Description**: Implement builders for enum and integer enum shape types.

**Steps**:
1. Implement builders for enum shapes (Enum, IntEnum)
2. Implement `ProvideTraitsMut` for each builder
3. Implement builder methods for each shape
4. Implement specialized methods like `value` for enum builders
5. Implement `build` methods that return `Result<Shape, BuildError>`

**Acceptance Criteria**:
- All enum shape builders are properly implemented
- All builders implement `ProvideTraitsMut`
- Builder methods work correctly
- Specialized methods work correctly
- `build` methods validate inputs and return appropriate errors

### Task SHAPE-013: Update Tests

**Description**: Update existing tests and add new tests for the new shape design.

**Steps**:
1. Update existing tests to work with the new shape design
2. Add tests for new functionality (traits, builders, etc.)
3. Add tests for error cases
4. Ensure all tests pass

**Acceptance Criteria**:
- All tests pass
- Test coverage is comprehensive
- Edge cases are tested

### Task SHAPE-014: Update Documentation

**Description**: Update documentation to reflect the new shape design.

**Steps**:
1. Update documentation comments in code
2. Update examples in documentation
3. Update design documentation

**Acceptance Criteria**:
- Documentation is complete and accurate
- Examples are up-to-date
- Design documentation reflects the implemented design

## Implementation Plan

1. Start with core traits and metadata (SHAPE-001)
2. Implement the Shape enum and basic variants (SHAPE-002)
3. Implement aggregate shapes (SHAPE-003)
4. Implement service shapes (SHAPE-004)
5. Implement enum shapes (SHAPE-005)
6. Implement type checking and conversion methods (SHAPE-006)
7. Implement the builder pattern (SHAPE-007 through SHAPE-012)
8. Update tests and documentation (SHAPE-013, SHAPE-014)

## Dependencies

- The `ShapeId` type must be fully implemented
- The `Trait` type must be fully implemented

## Risks and Mitigations

**Risk**: Breaking changes to the API
**Mitigation**: Implement the new design in a separate branch and thoroughly test before merging

**Risk**: Performance regression
**Mitigation**: Benchmark the new implementation against the old one

**Risk**: Increased complexity
**Mitigation**: Ensure good documentation and examples

## Timeline

Estimated time for completion: 2-3 weeks

- Week 1: Tasks SHAPE-001 through SHAPE-006
- Week 2: Tasks SHAPE-007 through SHAPE-012
- Week 3: Tasks SHAPE-013 and SHAPE-014, testing and refinement
