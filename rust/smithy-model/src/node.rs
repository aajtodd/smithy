/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Representation of a Smithy [node value](https://smithy.io/2.0/spec/model.html#node-values)

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::Hash;

/// Representation of a node value in a Smithy model.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(Number),
    /// A boolean value.
    Bool(bool),
    /// An array of values.
    Array(Vec<Node>),
    // FIXME - define our own Map type like serde_json that preserves insertion order
    /// An object with string keys and values.
    Object(HashMap<String, Node>),
    /// A null value.
    Null,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    inner: Inner
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number({})", self)
    }
}

#[derive(Clone, Copy)]
enum Inner {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Inner::PosInt(a), Inner::PosInt(b)) => a == b,
            (Inner::NegInt(a), Inner::NegInt(b)) => a == b,
            (Inner::Float(a), Inner::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Inner { }

impl Hash for Inner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Inner::PosInt(n) => n.hash(state),
            Inner::NegInt(n) => n.hash(state),
            Inner::Float(n) => {
                let bits = n.to_bits();
                bits.hash(state);
            }
        }
    }
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO - consider using itoa and ryu crates for performance here
            Inner::PosInt(n) => write!(f, "{}", n),
            Inner::NegInt(n) => write!(f, "{}", n),
            Inner::Float(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}


impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Node::String(s) => s.hash(state),
            Node::Number(n) => n.hash(state),
            Node::Bool(b) => b.hash(state),
            Node::Array(a) => a.hash(state),
            Node::Object(o) => {
                // FIXME - replace if we replace with our own ordered map type
                let ordered: BTreeMap<_, _> = o.iter().collect();
                o.len().hash(state);
                for (k, v) in ordered {
                    k.hash(state);
                    v.hash(state);
                }
            }
            Node::Null => 0.hash(state),
        }
    }
}
