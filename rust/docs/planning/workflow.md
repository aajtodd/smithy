# Development Workflow

This document outlines the workflow for implementing the Smithy Rust project with Amazon Q assistance.

## General Workflow Instructions

* Update the detailed design `detailed-design.md` if we make any changes to the design or architecture as we go
* Update the implementation notes `implementation-notes.md` with a summary of our discussions and questions as we flesh out the design in a way it can be
  referenced later for writing about how this project was implemented along the way.
* Ensure all tests pass before considering a task complete
* Commit changes to git with a meaningful commit message
* Stop and wait for feedback before moving to the next task
* **Always confirm planned changes before executing them** - outline the changes and get approval before proceeding
* **Make small, focused changes** rather than large scaffolding operations
* **Preview code changes** before applying them, especially for file creation or modification

## 1. Task Definition

Each development task should be clearly defined with:

- **Task ID**: A unique identifier for the task (e.g., `MODEL-001`)
- **Description**: A clear description of what needs to be implemented
- **Acceptance Criteria**: What constitutes successful completion
- **Dependencies**: Any tasks that must be completed first
- **Reference Materials**: Links to relevant documentation or Java implementation

## 2. Implementation Workflow

### 2.1 Task Planning

1. **Review the task** and understand its requirements
2. **Examine reference materials** from the Java implementation or Smithy specification
3. **Identify key design decisions** that need to be made
4. **Outline the implementation approach** before writing code

### 2.2 Implementation

1. **Start with a skeleton** of the required types and functions
2. **Implement core functionality** first, then edge cases
3. **Add documentation comments** as you go
4. **Write unit tests** for the implementation

### 2.3 Review and Refinement

1. **Review the implementation** against the acceptance criteria
2. **Run tests** to ensure correctness
3. **Refine the implementation** based on feedback
4. **Document any design decisions** or trade-offs made

## 3. Amazon Q Assistance

When working with Amazon Q to implement tasks:

### 3.1 Task Specification

Provide Amazon Q with:

- The task description and acceptance criteria
- Relevant context from the Smithy specification or Java implementation
- Any constraints or requirements for the Rust implementation

Example:
```
I need help implementing the ShapeId type for our Smithy Rust project. 
The ShapeId should represent a unique identifier for a shape in a Smithy model,
with a namespace, name, and optional member component.

Reference: In the Java implementation, this is defined in ShapeId.java with methods
for parsing, formatting, and comparing shape IDs.
```

### 3.2 Iterative Development

1. **Start with a high-level approach** and get feedback
2. **Refine the implementation** through conversation
3. **Ask for specific improvements** or alternatives when needed
4. **Validate the implementation** against requirements

### 3.3 Code Integration

After Amazon Q provides code:

1. **Review the code** for correctness and style
2. **Test the implementation** with unit tests
3. **Integrate the code** into the project
4. **Document any issues** encountered during integration

## 4. Testing Strategy

### 4.1 Unit Testing

- Each component should have comprehensive unit tests
- Tests should cover normal cases, edge cases, and error conditions
- Use test fixtures from the Java implementation where applicable

### 4.2 Integration Testing

- Test components together to ensure they work as a system
- Parse example models from the Smithy specification
- Verify compatibility with models from the Java implementation

## 5. Documentation

### 5.1 Code Documentation

- All public types and functions should have documentation comments
- Include examples for complex functionality
- Document design decisions and trade-offs

### 5.2 Project Documentation

- Update the project documentation as components are implemented
- Document the architecture and design patterns used
- Provide usage examples for common tasks

## 6. Task Tracking

Track tasks using:

- **Status**: Not Started, In Progress, Review, Complete
- **Priority**: High, Medium, Low
- **Assignee**: Who is responsible for the task
- **Due Date**: When the task should be completed
- **Notes**: Any additional information or context

## 7. Example Task Workflow

### Task: Implement ShapeId

1. **Define the requirements** for ShapeId based on the specification
2. **Review the Java implementation** for reference
3. **Design the Rust API** for ShapeId
4. **Implement the core functionality** with Amazon Q assistance
5. **Write unit tests** for the implementation
6. **Document the API** with examples
7. **Integrate with other components** as needed
8. **Review and refine** based on feedback

## 8. Code Generation and Implementation Guidelines

### 8.1 Incremental Implementation

- **Confirm Before Executing**: Always confirm planned changes before executing them
- **Small, Focused Changes**: Make small, focused changes rather than large scaffolding operations
- **Review Before Implementation**: Outline the planned implementation and get approval before proceeding
- **Step-by-Step Approach**: Break down implementation into smaller steps with clear checkpoints

### 8.2 Code Review Process

- **Preview Changes**: Generate a preview of changes before applying them
- **Explain Rationale**: Explain the rationale behind implementation choices
- **Highlight Alternatives**: Discuss alternative approaches that were considered
- **Document Trade-offs**: Document any trade-offs made in the implementation

## 9. Code Quality Standards

### 9.1 Code Formatting and Style

- **Use Cargo Fmt**: All code must be formatted with `cargo fmt` before committing
- **Run Clippy**: Use `cargo clippy` to catch common mistakes and improve code quality
- **Pass Tests**: All tests must pass with `cargo test --all-features` before committing

### 9.2 Documentation and Warnings

- **Include License Header**: All source files must include the following license header:
  ```rust
  /*
   * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
   * SPDX-License-Identifier: Apache-2.0
   */
  ```
- **Enable Warnings and Doc Features**: All crate root files (lib.rs) must include:
  ```rust
  #![warn(
      missing_docs,
      rustdoc::missing_crate_level_docs,
      unreachable_pub,
      rust_2018_idioms
  )]
  #![cfg_attr(docsrs, feature(doc_auto_cfg))]
  ```
- **Document Public API**: All public items must have documentation comments
- **Include Examples**: Include examples in documentation for complex functionality
