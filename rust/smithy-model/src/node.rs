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

impl Node {
    /// Creates a new null node.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let null = Node::null();
    /// assert!(null.is_null());
    /// ```
    pub fn null() -> Self {
        Node::Null
    }

    /// Creates a new boolean node.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let boolean = Node::boolean(true);
    /// assert_eq!(boolean.as_bool(), Some(true));
    /// ```
    pub fn boolean(b: bool) -> Self {
        Node::Bool(b)
    }

    /// Creates a new string node.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let string = Node::string("value");
    /// assert_eq!(string.as_str(), Some("value"));
    /// ```
    pub fn string<S: Into<String>>(s: S) -> Self {
        Node::String(s.into())
    }

    /// Creates a new number node from an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let number = Node::number(42);
    /// assert_eq!(number.as_number().unwrap().as_i64(), Some(42));
    /// ```
    pub fn number<N: Into<Number>>(n: N) -> Self {
        Node::Number(n.into())
    }

    /// Creates a new array node.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let array = Node::array(vec![Node::boolean(true), Node::number(42)]);
    /// assert_eq!(array.as_array().unwrap().len(), 2);
    /// ```
    pub fn array(array: Vec<Node>) -> Self {
        Node::Array(array)
    }

    /// Creates a new object node.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use smithy_model::Node;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), Node::boolean(true));
    /// let object = Node::object(map);
    /// assert!(object.as_object().unwrap().contains_key("key"));
    /// ```
    pub fn object(map: HashMap<String, Node>) -> Self {
        Node::Object(map)
    }

    /// Returns true if the `Node` is a Null. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let null = Node::Null;
    /// assert!(null.is_null());
    ///
    /// let not_null = Node::Bool(true);
    /// assert!(!not_null.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Node::Null)
    }

    /// Returns true if the `Node` is a Boolean. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let boolean = Node::Bool(true);
    /// assert!(boolean.is_boolean());
    ///
    /// let not_boolean = Node::String("not a boolean".to_string());
    /// assert!(!not_boolean.is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        matches!(self, Node::Bool(_))
    }

    /// Returns true if the `Node` is a Number. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::{Node, Number};
    ///
    /// let number = Node::Number(Number::from(42));
    /// assert!(number.is_number());
    ///
    /// let not_number = Node::String("not a number".to_string());
    /// assert!(!not_number.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        matches!(self, Node::Number(_))
    }

    /// Returns true if the `Node` is a String. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let string = Node::String("value".to_string());
    /// assert!(string.is_string());
    ///
    /// let not_string = Node::Bool(true);
    /// assert!(!not_string.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Node::String(_))
    }

    /// Returns true if the `Node` is an Array. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let array = Node::Array(vec![Node::Bool(true)]);
    /// assert!(array.is_array());
    ///
    /// let not_array = Node::Bool(true);
    /// assert!(!not_array.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, Node::Array(_))
    }

    /// Returns true if the `Node` is an Object. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use smithy_model::Node;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), Node::Bool(true));
    /// let object = Node::Object(map);
    /// assert!(object.is_object());
    ///
    /// let not_object = Node::Bool(true);
    /// assert!(!not_object.is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        matches!(self, Node::Object(_))
    }

    /// If the `Node` is a Boolean, returns the associated bool.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let boolean = Node::Bool(true);
    /// assert_eq!(boolean.as_bool(), Some(true));
    ///
    /// let not_boolean = Node::String("not a boolean".to_string());
    /// assert_eq!(not_boolean.as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Node::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// If the `Node` is a Number, returns the associated Number.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::{Node, Number};
    ///
    /// let number = Node::Number(Number::from(42));
    /// assert!(number.as_number().is_some());
    ///
    /// let not_number = Node::String("not a number".to_string());
    /// assert!(not_number.as_number().is_none());
    /// ```
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Node::Number(n) => Some(n),
            _ => None,
        }
    }

    /// If the `Node` is an integer, represent it as i64 if possible. Returns None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Node::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `Node` is a number, represent it as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Node::Number(n) => Some(n.as_f64()),
            _ => None,
        }
    }

    /// If the `Node` is a String, returns the associated str.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let string = Node::String("value".to_string());
    /// assert_eq!(string.as_str(), Some("value"));
    ///
    /// let not_string = Node::Bool(true);
    /// assert_eq!(not_string.as_str(), None);
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Node::String(s) => Some(s),
            _ => None,
        }
    }

    /// If the `Node` is an Array, returns the associated vector.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let array = Node::Array(vec![Node::Bool(true)]);
    /// assert!(array.as_array().is_some());
    ///
    /// let not_array = Node::Bool(true);
    /// assert!(not_array.as_array().is_none());
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Array(a) => Some(a),
            _ => None,
        }
    }

    /// If the `Node` is an Array, returns a mutable reference to the associated vector.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::Node;
    ///
    /// let mut array = Node::Array(vec![Node::Bool(true)]);
    /// if let Some(vec) = array.as_array_mut() {
    ///     vec.push(Node::Bool(false));
    /// }
    /// assert_eq!(array.as_array().unwrap().len(), 2);
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Array(a) => Some(a),
            _ => None,
        }
    }

    /// If the `Node` is an Object, returns the associated map.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use smithy_model::Node;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), Node::Bool(true));
    /// let object = Node::Object(map);
    /// assert!(object.as_object().is_some());
    ///
    /// let not_object = Node::Bool(true);
    /// assert!(not_object.as_object().is_none());
    /// ```
    pub fn as_object(&self) -> Option<&HashMap<String, Node>> {
        match self {
            Node::Object(o) => Some(o),
            _ => None,
        }
    }

    /// If the `Node` is an Object, returns a mutable reference to the associated map.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use smithy_model::Node;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), Node::Bool(true));
    /// let mut object = Node::Object(map);
    /// if let Some(obj) = object.as_object_mut() {
    ///     obj.insert("another_key".to_string(), Node::Bool(false));
    /// }
    /// assert_eq!(object.as_object().unwrap().len(), 2);
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Node>> {
        match self {
            Node::Object(o) => Some(o),
            _ => None,
        }
    }
}

/// Representation of a numeric node value
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    inner: Inner
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number({})", self)
    }
}

impl Number {
    /// Returns true if the `Number` can be represented as an `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::Number;
    ///
    /// let integer = Number::from(42);
    /// assert!(integer.is_i64());
    ///
    /// let negative = Number::from(-42);
    /// assert!(negative.is_i64());
    ///
    /// let float = Number::from(42.5);
    /// assert!(!float.is_i64());
    /// ```
    pub fn is_i64(&self) -> bool {
        match self.inner {
            Inner::NegInt(_) => true,
            Inner::PosInt(n) => n <= i64::MAX as u64,
            Inner::Float(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented as a `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::Number;
    ///
    /// let integer = Number::from(42u64);
    /// assert!(integer.is_u64());
    ///
    /// let negative = Number::from(-42);
    /// assert!(!negative.is_u64());
    ///
    /// let float = Number::from(42.5);
    /// assert!(!float.is_u64());
    /// ```
    pub fn is_u64(&self) -> bool {
        matches!(self.inner, Inner::PosInt(_))
    }

    /// Returns true if the `Number` can be represented as an `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::Number;
    ///
    /// let integer = Number::from(42);
    /// assert!(integer.is_f64());
    ///
    /// let float = Number::from(42.5);
    /// assert!(float.is_f64());
    /// ```
    pub fn is_f64(&self) -> bool {
        true // All numbers can be represented as f64
    }

    /// If the `Number` is an integer, returns the associated `i64`.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::Number;
    ///
    /// let integer = Number::from(42);
    /// assert_eq!(integer.as_i64(), Some(42));
    ///
    /// let negative = Number::from(-42);
    /// assert_eq!(negative.as_i64(), Some(-42));
    ///
    /// let float = Number::from(42.5);
    /// assert_eq!(float.as_i64(), None);
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match self.inner {
            Inner::NegInt(n) => Some(n),
            Inner::PosInt(n) if n <= i64::MAX as u64 => Some(n as i64),
            _ => None,
        }
    }

    /// If the `Number` is a non-negative integer, returns the associated `u64`.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::Number;
    ///
    /// let integer = Number::from(42u64);
    /// assert_eq!(integer.as_u64(), Some(42));
    ///
    /// let negative = Number::from(-42);
    /// assert_eq!(negative.as_u64(), None);
    ///
    /// let float = Number::from(42.5);
    /// assert_eq!(float.as_u64(), None);
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match self.inner {
            Inner::PosInt(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the number as an `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smithy_model::node::Number;
    ///
    /// let integer = Number::from(42);
    /// assert_eq!(integer.as_f64(), 42.0);
    ///
    /// let float = Number::from(42.5);
    /// assert_eq!(float.as_f64(), 42.5);
    /// ```
    pub fn as_f64(&self) -> f64 {
        match self.inner {
            Inner::PosInt(n) => n as f64,
            Inner::NegInt(n) => n as f64,
            Inner::Float(n) => n,
        }
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
impl From<i8> for Number {
    fn from(n: i8) -> Self {
        Number { inner: if n < 0 { Inner::NegInt(n as i64) } else { Inner::PosInt(n as u64) } }
    }
}

impl From<i16> for Number {
    fn from(n: i16) -> Self {
        Number { inner: if n < 0 { Inner::NegInt(n as i64) } else { Inner::PosInt(n as u64) } }
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number { inner: if n < 0 { Inner::NegInt(n as i64) } else { Inner::PosInt(n as u64) } }
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Number { inner: if n < 0 { Inner::NegInt(n) } else { Inner::PosInt(n as u64) } }
    }
}

impl From<u8> for Number {
    fn from(n: u8) -> Self {
        Number { inner: Inner::PosInt(n as u64) }
    }
}

impl From<u16> for Number {
    fn from(n: u16) -> Self {
        Number { inner: Inner::PosInt(n as u64) }
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Self {
        Number { inner: Inner::PosInt(n as u64) }
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Self {
        Number { inner: Inner::PosInt(n) }
    }
}

impl From<f32> for Number {
    fn from(n: f32) -> Self {
        Number { inner: Inner::Float(n as f64) }
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Number { inner: Inner::Float(n) }
    }
}

// From implementations for Node
impl From<bool> for Node {
    fn from(b: bool) -> Self {
        Node::Bool(b)
    }
}

impl From<String> for Node {
    fn from(s: String) -> Self {
        Node::String(s)
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        Node::String(s.to_string())
    }
}

impl From<Number> for Node {
    fn from(n: Number) -> Self {
        Node::Number(n)
    }
}

impl From<Vec<Node>> for Node {
    fn from(v: Vec<Node>) -> Self {
        Node::Array(v)
    }
}

impl From<HashMap<String, Node>> for Node {
    fn from(m: HashMap<String, Node>) -> Self {
        Node::Object(m)
    }
}

// From implementations for primitive types to Node via Number
impl From<i8> for Node {
    fn from(n: i8) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<i16> for Node {
    fn from(n: i16) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<i32> for Node {
    fn from(n: i32) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<i64> for Node {
    fn from(n: i64) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<u8> for Node {
    fn from(n: u8) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<u16> for Node {
    fn from(n: u16) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<u32> for Node {
    fn from(n: u32) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<u64> for Node {
    fn from(n: u64) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<f32> for Node {
    fn from(n: f32) -> Self {
        Node::Number(Number::from(n))
    }
}

impl From<f64> for Node {
    fn from(n: f64) -> Self {
        Node::Number(Number::from(n))
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
