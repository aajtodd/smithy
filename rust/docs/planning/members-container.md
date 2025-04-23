# Members Container Design

## Overview

This document outlines the design for a `Members` container that provides a unified way to access, query, and iterate over the members of different shape types in the Smithy model.

## Problem Statement

Different shape types in Smithy have different ways of storing and accessing their members:

- `StructureShape` and `UnionShape` store members in a `HashMap<String, MemberShape>`
- `ListShape` and `SetShape` have a single member
- `MapShape` has two members: key and value
- Other shapes have no members

We need a consistent interface for working with members regardless of the underlying storage mechanism.

## Design Goals

1. Provide a unified interface for accessing members of any shape type
2. Support iteration, lookup, and querying of members
3. Avoid unnecessary allocations and copies
4. Make the code more maintainable by encapsulating member access logic
5. Support the implementation of mixin member resolution

## Solution: Members Container

We'll create a `Members` container that can be constructed from different shape types and provides a consistent way to work with their members.

### Members Struct

```rust
/// A container for accessing the members of a shape
pub struct Members<'a> {
    inner: MembersImpl<'a>,
}

enum MembersImpl<'a> {
    // For shapes that store members in a HashMap
    HashMap(&'a HashMap<String, MemberShape>),
    // For shapes with a single member (List, Set)
    Single {
        name: &'static str,
        member: &'a MemberShape,
    },
    // For Map shapes with key and value members
    KeyValue {
        key: &'a MemberShape,
        value: &'a MemberShape,
    },
    // For shapes with no members
    Empty,
}
```

### Constructors

```rust
impl<'a> Members<'a> {
    /// Create a Members container from a HashMap of members
    pub(crate) fn from_hash_map(map: &'a HashMap<String, MemberShape>) -> Self {
        Self {
            inner: MembersImpl::HashMap(map),
        }
    }

    /// Create a Members container with a single member
    pub(crate) fn single(name: &'static str, member: &'a MemberShape) -> Self {
        Self {
            inner: MembersImpl::Single { name, member },
        }
    }

    /// Create a Members container with key and value members
    pub(crate) fn key_value(key: &'a MemberShape, value: &'a MemberShape) -> Self {
        Self {
            inner: MembersImpl::KeyValue { key, value },
        }
    }

    /// Create an empty Members container
    pub(crate) fn empty() -> Self {
        Self {
            inner: MembersImpl::Empty,
        }
    }
}
```

### Member Access Methods

```rust
impl<'a> Members<'a> {
    /// Returns the number of members
    pub fn len(&self) -> usize {
        match &self.inner {
            MembersImpl::HashMap(map) => map.len(),
            MembersImpl::Single { .. } => 1,
            MembersImpl::KeyValue { .. } => 2,
            MembersImpl::Empty => 0,
        }
    }

    /// Returns true if there are no members
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over member names
    pub fn names(&self) -> impl Iterator<Item = &str> + '_ {
        match &self.inner {
            MembersImpl::HashMap(map) => MemberNamesHashMap(map.keys()),
            MembersImpl::Single { name, .. } => MemberNamesSingle { name, yielded: false },
            MembersImpl::KeyValue { .. } => MemberNamesKeyValue { yielded_key: false, yielded_value: false },
            MembersImpl::Empty => MemberNamesEmpty,
        }
    }

    /// Returns an iterator over member shapes
    pub fn iter(&self) -> impl Iterator<Item = &MemberShape> + '_ {
        match &self.inner {
            MembersImpl::HashMap(map) => MemberShapesHashMap(map.values()),
            MembersImpl::Single { member, .. } => MemberShapesSingle { member, yielded: false },
            MembersImpl::KeyValue { key, value } => MemberShapesKeyValue { key, value, state: 0 },
            MembersImpl::Empty => MemberShapesEmpty,
        }
    }

    /// Returns an iterator over (name, member) pairs
    pub fn iter_named(&self) -> impl Iterator<Item = (&str, &MemberShape)> + '_ {
        match &self.inner {
            MembersImpl::HashMap(map) => MemberPairsHashMap(map.iter()),
            MembersImpl::Single { name, member } => MemberPairsSingle { name, member, yielded: false },
            MembersImpl::KeyValue { key, value } => MemberPairsKeyValue { key, value, state: 0 },
            MembersImpl::Empty => MemberPairsEmpty,
        }
    }

    /// Gets a member by name
    pub fn get(&self, name: &str) -> Option<&MemberShape> {
        match &self.inner {
            MembersImpl::HashMap(map) => map.get(name),
            MembersImpl::Single { name: member_name, member } => {
                if *member_name == name {
                    Some(member)
                } else {
                    None
                }
            },
            MembersImpl::KeyValue { key, value } => {
                match name {
                    "key" => Some(key),
                    "value" => Some(value),
                    _ => None,
                }
            },
            MembersImpl::Empty => None,
        }
    }

    /// Returns true if a member with the given name exists
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}
```


### Shape Enum Method

The `Shape` enum will have a `members()` method that returns a `Members` container:

```rust
impl Shape {
    /// Returns a Members container for this shape
    pub fn members(&self) -> Members {
        match self {
            Shape::Structure(shape) => Members::from_hash_map(shape.members()),
            Shape::Union(shape) => Members::from_hash_map(shape.members()),
            Shape::List(shape) => Members::single("member", &shape.member),
            Shape::Set(shape) => Members::single("member", &shape.member),
            Shape::Map(shape) => Members::key_value(&shape.key, &shape.value),
            _ => Members::empty(),
        }
    }
}
```

## Usage Examples

### Iterating over members of a structure

```rust
let structure = StructureShape::builder()
    .id("example#Test")
    .member(
        MemberShape::builder()
            .id("example#Test$foo")
            .member_name("foo")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap(),
    )
    .build()
    .unwrap();

for member in structure.members().iter() {
    println!("Member: {}", member.member_name);
}
```

### Looking up a member by name

```rust
let shape: Shape = structure.into();
if let Some(member) = shape.members().get("foo") {
    println!("Found member: {}", member.member_name);
}
```

### Iterating over member names and shapes

```rust
for (name, member) in shape.members().iter_named() {
    println!("Member {} targets {}", name, member.target());
}
```

### Converting to a HashMap

Users can make use of Rust's standard iterator APIs and `collect()` to convert to a `HashMap` or other data structures.

```rust
let members_map: HashMap<String, MemberShape> = shape.members()
    .iter_named()
    .map(|(name, member)| (name.to_string(), member.clone()))
    .collect();
```

## Benefits

1. **Unified Interface**: Provides a consistent way to work with members of any shape type
2. **Comprehensive**: Supports iteration, lookup, and querying of members
3. **Encapsulation**: Hides the implementation details of how members are stored in different shape types
4. **Efficiency**: Avoids unnecessary allocations and copies when possible
5. **Maintainability**: Makes the code more maintainable by centralizing member access logic
6. **Flexibility**: Can be extended to support additional operations in the future

## Implementation Plan

1. Add the `Members` struct and its implementation to `shape/util.rs`
2. Add (or replace/refactor) shape variants to have a `members()` function that returns the new `Members` container
3. Add the `members()` method to the `Shape` enum that delegates to each variant that has a `members()` method
4. Add unit tests (for each shape variant) to verify the behavior
5. Update documentation to reflect the new API
6. Implement `compute_effective_members` using the new `Members` container
