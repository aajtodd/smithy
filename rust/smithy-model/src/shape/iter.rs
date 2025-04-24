/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Iterator types for working with shapes.

use crate::shape::{MemberShape, Shape, ShapeProperties};
use crate::ShapeId;
use indexmap::IndexMap;
use std::ops::Index;

/// A container for accessing the members of a shape.
///
/// This provides a unified interface for working with members of different shape types,
/// regardless of how they store their members internally.
pub struct Members<'a> {
    inner: MembersImpl<'a>,
}

/// Implementation details for the Members container.
enum MembersImpl<'a> {
    /// For shapes that store members in a HashMap (Structure, Union)
    HashMap(&'a IndexMap<String, MemberShape>),
    /// For shapes with a single member (List, Set)
    Single {
        name: &'static str,
        member: &'a MemberShape,
    },
    /// For Map shapes with key and value members
    KeyValue {
        key: &'a MemberShape,
        value: &'a MemberShape,
    },
    /// For shapes with no members
    Empty,
}

impl<'a> Members<'a> {
    /// Create a Members container from a HashMap of members.
    pub(crate) fn map(map: &'a IndexMap<String, MemberShape>) -> Self {
        Self {
            inner: MembersImpl::HashMap(map),
        }
    }

    /// Create a Members container with a single member.
    pub(crate) fn single(name: &'static str, member: &'a MemberShape) -> Self {
        Self {
            inner: MembersImpl::Single { name, member },
        }
    }

    /// Create a Members container with key and value members.
    pub(crate) fn key_value(key: &'a MemberShape, value: &'a MemberShape) -> Self {
        Self {
            inner: MembersImpl::KeyValue { key, value },
        }
    }

    /// Create an empty Members container.
    pub(crate) fn empty() -> Self {
        Self {
            inner: MembersImpl::Empty,
        }
    }

    /// Returns the number of members.
    pub fn len(&self) -> usize {
        match &self.inner {
            MembersImpl::HashMap(map) => map.len(),
            MembersImpl::Single { .. } => 1,
            MembersImpl::KeyValue { .. } => 2,
            MembersImpl::Empty => 0,
        }
    }

    /// Returns true if there are no members.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets a member by name.
    pub fn get(&self, name: &str) -> Option<&MemberShape> {
        match &self.inner {
            MembersImpl::HashMap(map) => map.get(name),
            MembersImpl::Single {
                name: member_name,
                member,
            } => {
                if *member_name == name {
                    Some(member)
                } else {
                    None
                }
            }
            MembersImpl::KeyValue { key, value } => match name {
                "key" => Some(key),
                "value" => Some(value),
                _ => None,
            },
            MembersImpl::Empty => None,
        }
    }

    /// Returns true if a member with the given name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Returns an iterator over member shapes.
    pub fn iter(&self) -> MemberIter<'a> {
        match &self.inner {
            MembersImpl::HashMap(map) => MemberIter {
                inner: MemberIterImpl::HashMap(map.values()),
            },
            MembersImpl::Single { member, .. } => MemberIter {
                inner: MemberIterImpl::Single(member, false),
            },
            MembersImpl::KeyValue { key, value } => MemberIter {
                inner: MemberIterImpl::KeyValue {
                    key,
                    value,
                    state: 0,
                },
            },
            MembersImpl::Empty => MemberIter {
                inner: MemberIterImpl::Empty,
            },
        }
    }

    /// Returns an iterator over member names.
    pub fn names(&self) -> NameIter<'a> {
        match &self.inner {
            MembersImpl::HashMap(map) => NameIter {
                inner: NameIterImpl::HashMap(map.keys()),
            },
            MembersImpl::Single { name, .. } => NameIter {
                inner: NameIterImpl::Single(name, false),
            },
            MembersImpl::KeyValue { .. } => NameIter {
                inner: NameIterImpl::KeyValue {
                    yielded_key: false,
                    yielded_value: false,
                },
            },
            MembersImpl::Empty => NameIter {
                inner: NameIterImpl::Empty,
            },
        }
    }

    /// Returns an iterator over (name, member) pairs.
    pub fn iter_named(&self) -> NamedIter<'a> {
        match &self.inner {
            MembersImpl::HashMap(map) => NamedIter {
                inner: NamedIterImpl::HashMap(map.iter()),
            },
            MembersImpl::Single { name, member } => NamedIter {
                inner: NamedIterImpl::Single {
                    name,
                    member,
                    yielded: false,
                },
            },
            MembersImpl::KeyValue { key, value } => NamedIter {
                inner: NamedIterImpl::KeyValue {
                    key,
                    value,
                    state: 0,
                },
            },
            MembersImpl::Empty => NamedIter {
                inner: NamedIterImpl::Empty,
            },
        }
    }
}

/// An iterator over member shapes.
pub struct MemberIter<'a> {
    inner: MemberIterImpl<'a>,
}

/// Implementation details for the MemberIter.
enum MemberIterImpl<'a> {
    /// Iterator for HashMap-based members
    HashMap(indexmap::map::Values<'a, String, MemberShape>),
    /// Iterator for a single member
    Single(&'a MemberShape, bool),
    /// Iterator for key and value members
    KeyValue {
        key: &'a MemberShape,
        value: &'a MemberShape,
        state: u8,
    },
    /// Empty iterator
    Empty,
}

impl<'a> Iterator for MemberIter<'a> {
    type Item = &'a MemberShape;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            MemberIterImpl::HashMap(iter) => iter.next(),
            MemberIterImpl::Single(member, yielded) => {
                if !*yielded {
                    *yielded = true;
                    Some(*member)
                } else {
                    None
                }
            }
            MemberIterImpl::KeyValue { key, value, state } => match *state {
                0 => {
                    *state = 1;
                    Some(*key)
                }
                1 => {
                    *state = 2;
                    Some(*value)
                }
                _ => None,
            },
            MemberIterImpl::Empty => None,
        }
    }
}

/// An iterator over member names.
pub struct NameIter<'a> {
    inner: NameIterImpl<'a>,
}

/// Implementation details for the NameIter.
enum NameIterImpl<'a> {
    /// Iterator for HashMap-based members
    HashMap(indexmap::map::Keys<'a, String, MemberShape>),
    /// Iterator for a single member
    Single(&'static str, bool),
    /// Iterator for key and value members
    KeyValue {
        yielded_key: bool,
        yielded_value: bool,
    },
    /// Empty iterator
    Empty,
}

impl<'a> Iterator for NameIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            NameIterImpl::HashMap(iter) => iter.next().map(|s| s.as_str()),
            NameIterImpl::Single(name, yielded) => {
                if !*yielded {
                    *yielded = true;
                    Some(*name)
                } else {
                    None
                }
            }
            NameIterImpl::KeyValue {
                yielded_key,
                yielded_value,
            } => {
                if !*yielded_key {
                    *yielded_key = true;
                    Some("key")
                } else if !*yielded_value {
                    *yielded_value = true;
                    Some("value")
                } else {
                    None
                }
            }
            NameIterImpl::Empty => None,
        }
    }
}

/// An iterator over (name, member) pairs.
pub struct NamedIter<'a> {
    inner: NamedIterImpl<'a>,
}

/// Implementation details for the NamedIter.
enum NamedIterImpl<'a> {
    /// Iterator for HashMap-based members
    HashMap(indexmap::map::Iter<'a, String, MemberShape>),
    /// Iterator for a single member
    Single {
        name: &'static str,
        member: &'a MemberShape,
        yielded: bool,
    },
    /// Iterator for key and value members
    KeyValue {
        key: &'a MemberShape,
        value: &'a MemberShape,
        state: u8,
    },
    /// Empty iterator
    Empty,
}

impl<'a> Iterator for NamedIter<'a> {
    type Item = (&'a str, &'a MemberShape);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            NamedIterImpl::HashMap(iter) => iter.next().map(|(k, v)| (k.as_str(), v)),
            NamedIterImpl::Single {
                name,
                member,
                yielded,
            } => {
                if !*yielded {
                    *yielded = true;
                    Some((*name, *member))
                } else {
                    None
                }
            }
            NamedIterImpl::KeyValue { key, value, state } => match *state {
                0 => {
                    *state = 1;
                    Some(("key", *key))
                }
                1 => {
                    *state = 2;
                    Some(("value", *value))
                }
                _ => None,
            },
            NamedIterImpl::Empty => None,
        }
    }
}

/// A container for accessing mixins in a shape.
pub struct Mixins<'a> {
    mixins: &'a Vec<Shape>,
}

impl<'a> Mixins<'a> {
    /// Create a new Mixins container from a slice of shapes.
    pub(crate) fn new(mixins: &'a Vec<Shape>) -> Self {
        Self { mixins }
    }

    /// Returns the number of mixins.
    #[inline]
    pub fn len(&self) -> usize {
        self.mixins.len()
    }

    /// Returns true if there are no mixins.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.mixins.is_empty()
    }

    /// Returns an iterator over the mixins.
    pub fn iter(&self) -> impl Iterator<Item = &'a Shape> {
        self.mixins.iter()
    }

    /// Returns a mixin by index.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&'a Shape> {
        self.mixins.get(index)
    }

    /// Returns true if the mixins contain a shape with the given ID.
    #[inline]
    pub fn contains(&self, id: &ShapeId) -> bool {
        self.mixins.iter().any(|mixin| mixin.id() == id)
    }

    /// Returns a slice containing the entire vector.
    #[inline]
    pub fn as_slice(&self) -> &'a [Shape] {
        self.mixins.as_slice()
    }
}

impl<'a> From<&'a Vec<Shape>> for Mixins<'a> {
    fn from(mixins: &'a Vec<Shape>) -> Self {
        Self::new(mixins)
    }
}

impl<'a> IntoIterator for Mixins<'a> {
    type Item = &'a Shape;
    type IntoIter = std::slice::Iter<'a, Shape>;

    fn into_iter(self) -> Self::IntoIter {
        self.mixins.iter()
    }
}

impl Index<usize> for Mixins<'_> {
    type Output = Shape;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mixins[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape_id::ShapeId;

    #[test]
    fn test_members_empty() {
        let members = Members::empty();
        assert_eq!(members.len(), 0);
        assert!(members.is_empty());
        assert!(members.iter().next().is_none());
        assert!(members.names().next().is_none());
        assert!(members.iter_named().next().is_none());
        assert!(members.get("any").is_none());
        assert!(!members.contains("any"));
    }

    #[test]
    fn test_members_single() {
        let member = MemberShape::builder()
            .id("example#Test$member")
            .member_name("member")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap();

        let members = Members::single("member", &member);
        assert_eq!(members.len(), 1);
        assert!(!members.is_empty());

        // Test iter()
        let iter_result: Vec<_> = members.iter().collect();
        assert_eq!(iter_result.len(), 1);
        assert_eq!(iter_result[0].id().to_string(), "example#Test$member");

        // Test names()
        let names_result: Vec<_> = members.names().collect();
        assert_eq!(names_result.len(), 1);
        assert_eq!(names_result[0], "member");

        // Test iter_named()
        let named_result: Vec<_> = members.iter_named().collect();
        assert_eq!(named_result.len(), 1);
        assert_eq!(named_result[0].0, "member");
        assert_eq!(named_result[0].1.id().to_string(), "example#Test$member");

        // Test get() and contains()
        assert!(members.get("member").is_some());
        assert_eq!(
            members.get("member").unwrap().id().to_string(),
            "example#Test$member"
        );
        assert!(members.contains("member"));
        assert!(members.get("nonexistent").is_none());
        assert!(!members.contains("nonexistent"));
    }

    #[test]
    fn test_members_key_value() {
        let key = MemberShape::builder()
            .id("example#Test$key")
            .member_name("key")
            .target(ShapeId::new("smithy.api", "String").unwrap())
            .build()
            .unwrap();

        let value = MemberShape::builder()
            .id("example#Test$value")
            .member_name("value")
            .target(ShapeId::new("smithy.api", "Integer").unwrap())
            .build()
            .unwrap();

        let members = Members::key_value(&key, &value);
        assert_eq!(members.len(), 2);
        assert!(!members.is_empty());

        // Test iter()
        let iter_result: Vec<_> = members.iter().collect();
        assert_eq!(iter_result.len(), 2);
        assert_eq!(iter_result[0].id().to_string(), "example#Test$key");
        assert_eq!(iter_result[1].id().to_string(), "example#Test$value");

        // Test names()
        let names_result: Vec<_> = members.names().collect();
        assert_eq!(names_result.len(), 2);
        assert_eq!(names_result[0], "key");
        assert_eq!(names_result[1], "value");

        // Test iter_named()
        let named_result: Vec<_> = members.iter_named().collect();
        assert_eq!(named_result.len(), 2);
        assert_eq!(named_result[0].0, "key");
        assert_eq!(named_result[0].1.id().to_string(), "example#Test$key");
        assert_eq!(named_result[1].0, "value");
        assert_eq!(named_result[1].1.id().to_string(), "example#Test$value");

        // Test get() and contains()
        assert!(members.get("key").is_some());
        assert_eq!(
            members.get("key").unwrap().id().to_string(),
            "example#Test$key"
        );
        assert!(members.contains("key"));
        assert!(members.get("value").is_some());
        assert_eq!(
            members.get("value").unwrap().id().to_string(),
            "example#Test$value"
        );
        assert!(members.contains("value"));
        assert!(members.get("nonexistent").is_none());
        assert!(!members.contains("nonexistent"));
    }

    #[test]
    fn test_members_hash_map() {
        let mut map = IndexMap::new();
        map.insert(
            "foo".to_string(),
            MemberShape::builder()
                .id("example#Test$foo")
                .member_name("foo")
                .target(ShapeId::new("smithy.api", "String").unwrap())
                .build()
                .unwrap(),
        );
        map.insert(
            "bar".to_string(),
            MemberShape::builder()
                .id("example#Test$bar")
                .member_name("bar")
                .target(ShapeId::new("smithy.api", "Integer").unwrap())
                .build()
                .unwrap(),
        );

        let members = Members::map(&map);
        assert_eq!(members.len(), 2);
        assert!(!members.is_empty());

        // Test iter()
        let iter_result: Vec<_> = members.iter().collect();
        assert_eq!(iter_result.len(), 2);
        assert!(iter_result
            .iter()
            .any(|m| m.id().to_string() == "example#Test$foo"));
        assert!(iter_result
            .iter()
            .any(|m| m.id().to_string() == "example#Test$bar"));

        // Test names()
        let names_result: Vec<_> = members.names().collect();
        assert_eq!(names_result.len(), 2);
        assert!(names_result.contains(&"foo"));
        assert!(names_result.contains(&"bar"));

        // Test iter_named()
        let named_result: Vec<_> = members.iter_named().collect();
        assert_eq!(named_result.len(), 2);
        assert!(named_result.iter().any(|(name, _)| *name == "foo"));
        assert!(named_result.iter().any(|(name, _)| *name == "bar"));
        assert!(named_result
            .iter()
            .any(|(name, member)| *name == "foo" && member.id().to_string() == "example#Test$foo"));
        assert!(named_result
            .iter()
            .any(|(name, member)| *name == "bar" && member.id().to_string() == "example#Test$bar"));

        // Test get() and contains()
        assert!(members.get("foo").is_some());
        assert_eq!(
            members.get("foo").unwrap().id().to_string(),
            "example#Test$foo"
        );
        assert!(members.contains("foo"));
        assert!(members.get("bar").is_some());
        assert_eq!(
            members.get("bar").unwrap().id().to_string(),
            "example#Test$bar"
        );
        assert!(members.contains("bar"));
        assert!(members.get("nonexistent").is_none());
        assert!(!members.contains("nonexistent"));
    }
}
