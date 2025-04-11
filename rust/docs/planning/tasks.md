# Implementation Plan

* Remember to use our workflow instructions to complete tasks.

## Current Focus: Trait Implementation

Reference the design in [trait design](trait-design.md)

### Task 1: Core Trait Interface

- [ ] Define the `Trait` trait in `traits.rs`
    - [ ] Add `static_id()` method for trait type identification
    - [ ] Add `id()` method with default implementation
    - [ ] Add `to_node()` method for serialization
    - [ ] Add `from_node()` method for deserialization
    - [ ] Add `from_node_boxed()` helper method
    - [ ] Add `clone_trait()` method for cloning trait objects
    - [ ] Add `as_any()` method for downcasting
- [ ] Implement the `BoxTrait` type alias
- [ ] Add comprehensive documentation for the trait methods
- [ ] Create unit tests for the trait interface

### Task 2: Basic Trait Implementations

NOTE: implementations of "built in" (traits defined in the `smithy.api` namespace) will live under the
`traits` module. e.g. the `Documentation` trait would be defined in `traits/documentation.rs`

- [ ] Implement `Documentation` trait
    - [ ] Define the struct and implement `Trait`
    - [ ] Implement serialization to `Node`
    - [ ] Implement deserialization from `Node`
    - [ ] Write unit tests for serialization/deserialization
    - [ ] Add documentation
- [ ] Implement `Required` trait
    - [ ] Define the struct and implement `Trait`
    - [ ] Implement serialization to `Node`
    - [ ] Implement deserialization from `Node`
    - [ ] Write unit tests for serialization/deserialization
    - [ ] Add documentation
- [ ] Implement `Deprecated` trait
    - [ ] Define the struct and implement `Trait`
    - [ ] Implement serialization to `Node`
    - [ ] Implement deserialization from `Node`
    - [ ] Write unit tests for serialization/deserialization
    - [ ] Add documentation
- [ ] Implement `DynamicTrait` for unknown traits
    - [ ] Define the struct with ID and value fields
    - [ ] Implement `Trait` with custom `id()` method
    - [ ] Add constructor and accessor methods
    - [ ] Write unit tests
    - [ ] Add documentation

### Task 3: Trait Registry

- [ ] Implement the `TraitRegistry` struct in `traits/registry.rs`
    - [ ] Define the struct with creators map
    - [ ] Implement constructor with built-in traits
- [ ] Add methods for registering trait types
    - [ ] Implement `register<T: Trait + 'static>()` method
    - [ ] Add registration of built-in traits
- [ ] Implement trait creation from ID and Node
    - [ ] Add `create_trait()` method
    - [ ] Handle fallback to `DynamicTrait`
    - [ ] Add `is_registered()` helper method
- [ ] Write unit tests for the registry
    - [ ] Test registration of traits
    - [ ] Test creation of known traits
    - [ ] Test fallback to dynamic traits
    - [ ] Test error handling


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
