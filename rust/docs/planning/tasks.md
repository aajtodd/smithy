# Implementation Plan

* Remember to use our workflow instructions to complete tasks.

## Current Focus: Trait Implementation

Reference the design in [trait design](trait-design.md)

### Task 1: Core Trait Interface

- [x] Define the `Trait` trait in `traits.rs`
    - [x] Add `static_id()` method for trait type identification
    - [x] Add `id()` method with default implementation
    - [x] Add `to_node()` method for serialization
    - [x] Add `from_node()` method for deserialization
    - [x] Add `from_node_boxed()` helper method
    - [x] Add `clone_trait()` method for cloning trait objects
    - [x] Add `as_any()` method for downcasting
- [x] Implement the `BoxTrait` type alias
- [x] Add comprehensive documentation for the trait methods
- [x] Create unit tests for the trait interface

### Task 2: Basic Trait Implementations

NOTE: implementations of "built in" (traits defined in the `smithy.api` namespace) will live under the
`traits` module. e.g. the `Documentation` trait would be defined in `traits/documentation.rs`

- [x] Implement `Documentation` trait
    - [x] Define the struct and implement `Trait`
    - [x] Implement serialization to `Node`
    - [x] Implement deserialization from `Node`
    - [x] Write unit tests for serialization/deserialization
    - [x] Add documentation
- [x] Implement `Required` trait
    - [x] Define the struct and implement `Trait`
    - [x] Implement serialization to `Node`
    - [x] Implement deserialization from `Node`
    - [x] Write unit tests for serialization/deserialization
    - [x] Add documentation
- [x] Implement `Deprecated` trait
    - [x] Define the struct and implement `Trait`
    - [x] Implement serialization to `Node`
    - [x] Implement deserialization from `Node`
    - [x] Write unit tests for serialization/deserialization
    - [x] Add documentation
- [x] Implement `DynamicTrait` for unknown traits
    - [x] Define the struct with ID and value fields
    - [x] Implement `Trait` with custom `id()` method
    - [x] Add constructor and accessor methods
    - [x] Write unit tests
    - [x] Add documentation

### Task 3: Trait Registry

- [x] Implement the `TraitRegistry` struct in `traits/registry.rs`
    - [x] Define the struct with creators map
    - [x] Implement constructor with built-in traits
- [x] Add methods for registering trait types
    - [x] Implement `register<T: Trait + 'static>()` method
    - [x] Add registration of built-in traits
- [x] Implement trait creation from ID and Node
    - [x] Add `create_trait()` method
    - [x] Handle fallback to `DynamicTrait`
    - [x] Add `is_registered()` helper method
- [x] Write unit tests for the registry
    - [x] Test registration of traits
    - [x] Test creation of known traits
    - [x] Test fallback to dynamic traits
    - [x] Test error handling


### Task 4: Additional Trait Implementations

- [ ] Implement common Smithy traits from the `smithy.api` namespace from the Smithy specification
    - [ ] `Sensitive` trait
    - [ ] `Pattern` trait
    - [ ] `Length` trait
    - [ ] `Range` trait
    - [ ] `Title` trait
    - [ ] `Trait` trait (meta-trait)
- [ ] Write unit tests for each trait
    - [ ] Test serialization/deserialization
    - [ ] Test validation logic if applicable
- [ ] Update trait registry to include all implemented traits

### Task 5: Documentation and Examples

- [ ] Document the trait system
    - [ ] Add detailed comments to all types and methods
    - [ ] Update the design document with final implementation details
- [ ] Add examples for common use cases
    - [ ] Example for creating and using traits
    - [ ] Example for working with dynamic traits
    - [ ] Example for implementing custom traits
- [ ] Create a guide for implementing custom traits
    - [ ] Step-by-step instructions
    - [ ] Best practices
    - [ ] Common pitfalls to avoid

## Implementation Strategy

We will implement the trait system in the following order:

1. Start with the core interfaces (`Trait`, `BoxTrait`)
2. Implement a few basic traits (`Documentation`, `Required`)
3. Add the `DynamicTrait` implementation
4. Implement the `TraitRegistry`
5. Update the `Shape` struct to work with traits
6. Implement serialization/deserialization
7. Add more trait implementations
8. Add tests and documentation
9. Replace existing occurrences of "manually constructed" `documentation` and `required` traits with the new actual definitions 

This approach allows us to build and test incrementally, ensuring each component works before moving on to the next.

## Testing Strategy

- **Unit Tests**: Each component will have comprehensive unit tests
- **Integration Tests**: Test the interaction between components
- **Property Tests**: Use property-based testing for serialization/deserialization
- **Example Tests**: Create example-based tests for common use cases
- **Edge Cases**: Test error handling and edge cases

## Next Steps

After completing the trait implementation, we will move on to:

1. Designing the model (container) APIs
2. Model validation
3. Selector implementation
4. Code generation framework
