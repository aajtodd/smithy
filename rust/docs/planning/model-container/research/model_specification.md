# Smithy Model Specification Research

## Core Concepts from the Specification

The Smithy model is the core semantic model used by tools. It's independent of any particular serialized representation and contains:

1. **Metadata** - Schema-less extensibility mechanism for model-wide metadata
2. **Shapes** - Named data definitions that describe the structure of an API
3. **Shape IDs** - Unique identifiers for shapes (e.g., `smithy.example#MyShape`)
4. **Traits** - Specialized shapes that attach metadata to other shapes
5. **Applied Traits** - Instances of traits applied to shapes with configuration

### Model Structure

The model contains:
- A collection of shapes indexed by ShapeId
- Model-wide metadata
- The prelude (automatically included shapes and traits)

### Shape Types

Shapes are grouped into three categories:
1. **Simple types** - Types that don't contain nested types (e.g., `string`, `integer`)
2. **Aggregate types** - Types that contain members referencing other shapes (e.g., `structure`, `list`)
3. **Service types** - Types that define service organization (e.g., `service`, `operation`)

### Members

Members are defined in shapes to reference other shapes using a ShapeId. They're found in:
- `enum`
- `intEnum`
- `list`
- `map`
- `structure`
- `union`

### Shape IDs

A shape ID uniquely identifies shapes in the model with the syntax:
```
smithy.example.foo#ExampleShapeName$memberName
└─────────┬──────┘ └───────┬──────┘ └────┬───┘
     (Namespace)     (Shape name)  (Member name)
```

### Merging Models

Multiple model files can be merged to create a semantic model by:
1. Merging metadata objects
2. Adding shapes from each model file
3. Reconciling shapes with the same ID
4. Resolving conflicting traits

### Traits

Traits are model components attached to shapes to provide additional information. They:
- Can only be applied once to a shape (with conflict resolution rules)
- Have values compatible with their shape definition
- Can be defined by applying the `smithy.api#trait` trait to a shape

### Prelude

All Smithy models automatically include a prelude that defines:
- Various simple shapes
- Every trait defined in the core specification
