# Smithy Rust Project Planning

## Overview

Smithy is an Interface Definition Language (IDL) for defining services and generating both clients and servers.
The Smithy specification and reference implementation is written in the Java programming language. We would like to 
write an equivalent in the Rust programming language. 

## Important Documents and Information

* This document (which we will refer to as `rust plan` or `planning` for short)
* The detailed design `rust/docs/planning/detailed-design.md` which contains a more detailed design for sub components
* Instructions Amazon Q should use when collaborating on this project or executing any tasks: `rust/docs/planning/workflow.md` 
* Core specification documents in `/docs/source-2.0/spec/` that define the Smithy language and model:
  * `model.rst` - Core model concepts and definitions
  * `idl.rst` - Smithy IDL syntax and semantics
  * `json-ast.rst` - JSON AST format for serializing models
  * `simple-types.rst`, `aggregate-types.rst` - Shape type definitions
  * `service-types.rst` - Service, operation, and resource definitions
  * `selectors.rst` - Selector language for querying models
  * `model-validation.rst` - Rules for validating models
* Design documents in `/designs/` that explain key architectural decisions and features:
  * `mixins.md` - Design for shape mixins
  * `enum-shapes.md` - Design for enumeration shapes
  * `defaults-and-model-evolution.md` - How defaults work and model compatibility
  * `operation-input-output-and-unit-types.md` - Operation I/O design
* Java reference implementation in `/smithy-model/` and other modules:
  * `/smithy-model/src/main/java/software/amazon/smithy/model/Model.java` - Core model class
  * `/smithy-model/src/main/java/software/amazon/smithy/model/shapes/` - Shape implementations
  * `/smithy-model/src/main/java/software/amazon/smithy/model/traits/` - Trait implementations
  * `/smithy-model/src/main/java/software/amazon/smithy/model/loader/` - Model loading
  * `/smithy-model/src/main/java/software/amazon/smithy/model/validation/` - Validation framework
  * `/smithy-model/src/main/java/software/amazon/smithy/model/selector/` - Selector implementation
  * `/smithy-cli/src/main/java/software/amazon/smithy/cli/` - CLI implementation

## Project Structure

The Smithy Rust implementation is organized into the following crates:

```
/rust/
├── Cargo.toml                 # Workspace configuration
├── smithy-ast/                # AST representation and IDL parser
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs             # Main library entry point
│       ├── error.rs           # Error types for parsing operations
│       ├── ast.rs             # AST data structures
│       └── parser.rs          # Parser implementation
└── smithy-model/              # Semantic model
    ├── Cargo.toml
    └── src/
        ├── lib.rs             # Main library entry point
        ├── error.rs           # Error types for model operations
        ├── shape_id.rs        # ShapeId implementation
        ├── shape.rs           # Shape implementations
        ├── traits.rs          # Trait implementations
        ├── model.rs           # Model implementation
        ├── loader.rs          # Model loading from AST and JSON
        ├── validation.rs      # Validation framework
        └── selector.rs        # Selector implementation
```

This separation allows the parser to focus solely on syntax without worrying about semantic validation, while the model can focus on the semantic representation and operations.

## Architecture Overview

The Smithy Rust implementation will need to provide equivalent functionality to the Java implementation, including:

1. **Core Model Representation** - Data structures to represent Smithy models
2. **Model Loading** - Parsing and loading Smithy IDL and JSON AST files
3. **Model Validation** - Validating models against the Smithy specification
4. **Selectors** - Implementation of the selector language for querying models
5. **Traits** - Support for built-in and custom traits
6. **Code Generation** - Framework for generating code from models
7. **CLI** - Command-line interface for working with models

## Key Components

### 1. Core Model

The core model will represent the fundamental data structures of Smithy:

- **Shapes**: The basic building blocks (structures, lists, maps, services, operations, etc.)
- **Traits**: Metadata attached to shapes
- **Model**: The container for all shapes and their relationships

### 2. Parser and Loader

- **IDL Parser**: Parse Smithy IDL syntax into model objects
- **JSON AST Parser**: Parse Smithy JSON AST into model objects
- **Model Assembler**: Combine multiple model files into a single cohesive model

### 3. Validation

- **Validator Framework**: Extensible system for model validation
- **Built-in Validators**: Implementation of all standard Smithy validators

### 4. Selectors

- **Selector Parser**: Parse selector expressions
- **Selector Evaluator**: Evaluate selectors against models

### 5. Code Generation

- **Code Generator Framework**: Extensible system for generating code
- **Template System**: Support for code templates

### 6. CLI

- **Command Framework**: Extensible command system
- **Built-in Commands**: Standard commands like build, validate, etc.

## Implementation Strategy

We will implement the Rust version of Smithy in phases:

1. **Phase 1**: Core model representation and basic loading
2. **Phase 2**: Validation and selectors
3. **Phase 3**: Code generation framework
4. **Phase 4**: CLI and tooling

## Design Considerations

### Rust-specific Design Choices

- **Ownership and Borrowing**: How to handle references between shapes
- **Error Handling**: Using Result for error propagation
- **Traits vs Traits**: Disambiguating Rust traits from Smithy traits
- **Generics and Type Parameters**: Leveraging Rust's type system

### Performance Considerations

- **Memory Efficiency**: Minimizing allocations and copies
- **Parsing Performance**: Efficient parsing of large models
- **Validation Speed**: Fast validation for large models

### Extensibility

- **Plugin System**: Supporting extensions and custom validators
- **Custom Traits**: Easy definition of custom traits
- **Code Generation**: Flexible code generation system
