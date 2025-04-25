/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Macro for defining simple Smithy traits.
///
/// This macro reduces boilerplate when implementing simple traits with a single value.
///
/// # Examples
///
/// ```rust
/// use smithy_model::traits::define_simple_trait;
/// use smithy_model::node::Node;
///
/// // Define a string trait
/// define_simple_trait! {
///     /// Documentation for the trait.
///     #[derive(Debug, Clone, PartialEq)]
///     pub struct Documentation(pub String);
///
///     trait_id: "smithy.api", "documentation";
///     type: string;
/// }
///
/// // Define an annotation trait
/// define_simple_trait! {
///     /// Marks a shape as required.
///     #[derive(Debug, Clone, PartialEq)]
///     pub struct Required;
///
///     trait_id: "smithy.api", "required";
///     type: annotation;
/// }
/// ```
///
#[macro_export]
macro_rules! define_simple_trait {
    // String trait
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident(pub String);

        trait_id: $namespace:expr, $name_part:expr;
        type: string;
    ) => {
        $(#[$attr])*
        $vis struct $name(pub String);

        impl $crate::traits::Trait for $name {
            fn static_id() -> &'static $crate::shape::ShapeId {
                const TRAIT_ID: &$crate::shape::ShapeId = &$crate::shape::ShapeId::new_static($namespace, $name_part);
                TRAIT_ID
            }

            fn id(&self) -> &$crate::shape::ShapeId {
                Self::static_id()
            }

            fn to_node(&self) -> $crate::node::Node {
                $crate::node::Node::String(self.0.clone())
            }

            fn from_node(node: &$crate::node::Node) -> Option<Self> {
                match node {
                    $crate::node::Node::String(s) => Some(Self(s.clone())),
                    _ => None,
                }
            }

            fn clone_trait(&self) -> $crate::traits::BoxTrait {
                Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    // Boolean trait
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident(pub bool);

        trait_id: $namespace:expr, $name_part:expr;
        type: boolean;
    ) => {
        $(#[$attr])*
        $vis struct $name(pub bool);

        impl $crate::traits::Trait for $name {
            fn static_id() -> &'static $crate::shape::ShapeId {
                const TRAIT_ID: &$crate::shape::ShapeId = &$crate::shape::ShapeId::new_static($namespace, $name_part);
                TRAIT_ID
            }

            fn id(&self) -> &$crate::shape::ShapeId {
                Self::static_id()
            }

            fn to_node(&self) -> $crate::node::Node {
                $crate::node::Node::Bool(self.0)
            }

            fn from_node(node: &$crate::node::Node) -> Option<Self> {
                match node {
                    $crate::node::Node::Bool(b) => Some(Self(*b)),
                    _ => None,
                }
            }

            fn clone_trait(&self) -> $crate::traits::BoxTrait {
                Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    // Annotation trait (flag trait)
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident;

        trait_id: $namespace:expr, $name_part:expr;
        type: annotation;
    ) => {
        $(#[$attr])*
        $vis struct $name;

        impl $crate::traits::Trait for $name {
            fn static_id() -> &'static $crate::shape::ShapeId {
                const TRAIT_ID: &$crate::shape::ShapeId = &$crate::shape::ShapeId::new_static($namespace, $name_part);
                TRAIT_ID
            }

            fn id(&self) -> &$crate::shape::ShapeId {
                Self::static_id()
            }

            fn to_node(&self) -> $crate::node::Node {
                $crate::node::Node::Object(Default::default())
            }

            fn from_node(node: &$crate::node::Node) -> Option<Self> {
                match node {
                    $crate::node::Node::Object(_) | $crate::node::Node::Null => Some(Self),
                    _ => None,
                }
            }

            fn clone_trait(&self) -> $crate::traits::BoxTrait {
                Box::new(Self)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }
    };

    // Number trait
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident(pub Number);

        trait_id: $namespace:expr, $name_part:expr;
        type: number;
    ) => {
        $(#[$attr])*
        $vis struct $name(pub $crate::node::Number);

        impl $crate::traits::Trait for $name {
            fn static_id() -> &'static $crate::shape::ShapeId {
                const TRAIT_ID: &$crate::shape::ShapeId = &$crate::shape::ShapeId::new_static($namespace, $name_part);
                TRAIT_ID
            }

            fn id(&self) -> &$crate::shape::ShapeId {
                Self::static_id()
            }

            fn to_node(&self) -> $crate::node::Node {
                $crate::node::Node::Number(self.0.clone())
            }

            fn from_node(node: &$crate::node::Node) -> Option<Self> {
                match node {
                    $crate::node::Node::Number(n) => Some(Self(n.clone())),
                    _ => None,
                }
            }

            fn clone_trait(&self) -> $crate::traits::BoxTrait {
                Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    // Array trait
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident(pub Vec<$element_type:ty>);

        trait_id: $namespace:expr, $name_part:expr;
        type: array;
        element_to_node: $element_to_node:expr;
        node_to_element: $node_to_element:expr;
    ) => {
        $(#[$attr])*
        $vis struct $name(pub Vec<$element_type>);

        impl $crate::traits::Trait for $name {
            fn static_id() -> &'static $crate::shape::ShapeId {
                const TRAIT_ID: &$crate::shape::ShapeId = &$crate::shape::ShapeId::new_static($namespace, $name_part);
                TRAIT_ID
            }

            fn id(&self) -> &$crate::shape::ShapeId {
                Self::static_id()
            }

            fn to_node(&self) -> $crate::node::Node {
                let elements = self.0.iter().map($element_to_node).collect();
                $crate::node::Node::Array(elements)
            }

            fn from_node(node: &$crate::node::Node) -> Option<Self> {
                match node {
                    $crate::node::Node::Array(arr) => {
                        let elements = arr.iter().filter_map($node_to_element).collect();
                        Some(Self(elements))
                    },
                    _ => None,
                }
            }

            fn clone_trait(&self) -> $crate::traits::BoxTrait {
                Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::node::Number;
    use crate::traits::Trait;
    use crate::Node;

    // Define test traits using the macro
    define_simple_trait! {
        /// Documentation trait for testing
        #[derive(Debug, Clone, PartialEq)]
        struct TestDocumentation(pub String);

        trait_id: "test", "documentation";
        type: string;
    }

    define_simple_trait! {
        /// Boolean trait for testing
        #[derive(Debug, Clone, PartialEq)]
        struct TestDeprecated(pub bool);

        trait_id: "test", "deprecated";
        type: boolean;
    }

    define_simple_trait! {
        /// Annotation trait for testing
        #[derive(Debug, Clone, PartialEq)]
        struct TestRequired;

        trait_id: "test", "required";
        type: annotation;
    }

    define_simple_trait! {
        /// Number trait for testing
        #[derive(Debug, Clone, PartialEq)]
        struct TestMinLength(pub Number);

        trait_id: "test", "minLength";
        type: number;
    }

    define_simple_trait! {
        /// Array trait for testing
        #[derive(Debug, Clone, PartialEq)]
        struct TestTags(pub Vec<String>);

        trait_id: "test", "tags";
        type: array;
        element_to_node: |s| Node::String(s.clone());
        node_to_element: |n| n.as_str().map(|s| s.to_string());
    }

    #[test]
    fn test_string_trait() {
        // Test trait ID
        assert_eq!(
            TestDocumentation::static_id().to_string(),
            "test#documentation"
        );

        // Test creating trait instance
        let doc = TestDocumentation("Test documentation".to_string());
        assert_eq!(doc.0, "Test documentation");

        // Test to_node
        let node = doc.to_node();
        assert!(matches!(node, Node::String(s) if s == "Test documentation"));

        // Test from_node with valid node
        let string_node = Node::String("New documentation".to_string());
        let parsed_doc = TestDocumentation::from_node(&string_node).unwrap();
        assert_eq!(parsed_doc.0, "New documentation");

        // Test from_node with invalid node
        let bool_node = Node::Bool(true);
        assert!(TestDocumentation::from_node(&bool_node).is_none());

        // Test clone_trait
        let cloned = doc.clone_trait();
        let downcast = cloned.as_any().downcast_ref::<TestDocumentation>().unwrap();
        assert_eq!(downcast.0, "Test documentation");
    }

    #[test]
    fn test_boolean_trait() {
        // Test trait ID
        assert_eq!(TestDeprecated::static_id().to_string(), "test#deprecated");

        // Test creating trait instance
        let deprecated = TestDeprecated(true);
        assert!(deprecated.0);

        // Test to_node
        let node = deprecated.to_node();
        assert!(matches!(node, Node::Bool(b) if b));

        // Test from_node with valid node
        let bool_node = Node::Bool(false);
        let parsed = TestDeprecated::from_node(&bool_node).unwrap();
        assert!(!parsed.0);

        // Test from_node with invalid node
        let string_node = Node::String("invalid".to_string());
        assert!(TestDeprecated::from_node(&string_node).is_none());

        // Test clone_trait
        let cloned = deprecated.clone_trait();
        let downcast = cloned.as_any().downcast_ref::<TestDeprecated>().unwrap();
        assert!(downcast.0);
    }

    #[test]
    fn test_annotation_trait() {
        // Test trait ID
        assert_eq!(TestRequired::static_id().to_string(), "test#required");

        // Test creating trait instance
        let required = TestRequired;

        // Test to_node
        let node = required.to_node();
        assert!(matches!(node, Node::Object(obj) if obj.is_empty()));

        // Test from_node with valid nodes
        let obj_node = Node::Object(std::collections::HashMap::new());
        assert!(TestRequired::from_node(&obj_node).is_some());

        let null_node = Node::Null;
        assert!(TestRequired::from_node(&null_node).is_some());

        // Test from_node with invalid node
        let string_node = Node::String("invalid".to_string());
        assert!(TestRequired::from_node(&string_node).is_none());

        // Test clone_trait
        let cloned = required.clone_trait();
        assert!(cloned.as_any().downcast_ref::<TestRequired>().is_some());
    }

    #[test]
    fn test_number_trait() {
        // Test trait ID
        assert_eq!(TestMinLength::static_id().to_string(), "test#minLength");

        // Test creating trait instance
        let min_length = TestMinLength(Number::from(10));
        assert_eq!(min_length.0.to_string(), "10");

        // Test to_node
        let node = min_length.to_node();
        assert!(matches!(node, Node::Number(n) if n.to_string() == "10"));

        // Test from_node with valid node
        let number_node = Node::Number(Number::from(20));
        let parsed = TestMinLength::from_node(&number_node).unwrap();
        assert_eq!(parsed.0.to_string(), "20");

        // Test from_node with invalid node
        let string_node = Node::String("invalid".to_string());
        assert!(TestMinLength::from_node(&string_node).is_none());

        // Test clone_trait
        let cloned = min_length.clone_trait();
        let downcast = cloned.as_any().downcast_ref::<TestMinLength>().unwrap();
        assert_eq!(downcast.0.to_string(), "10");
    }

    #[test]
    fn test_array_trait() {
        // Test trait ID
        assert_eq!(TestTags::static_id().to_string(), "test#tags");

        // Test creating trait instance
        let tags = TestTags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(tags.0, vec!["tag1", "tag2"]);

        // Test to_node
        let node = tags.to_node();
        if let Node::Array(arr) = node {
            assert_eq!(arr.len(), 2);
            assert!(matches!(&arr[0], Node::String(s) if s == "tag1"));
            assert!(matches!(&arr[1], Node::String(s) if s == "tag2"));
        } else {
            panic!("Expected Node::Array");
        }

        // Test from_node with valid node
        let array_node = Node::Array(vec![
            Node::String("tag3".to_string()),
            Node::String("tag4".to_string()),
        ]);
        let parsed = TestTags::from_node(&array_node).unwrap();
        assert_eq!(parsed.0, vec!["tag3", "tag4"]);

        // Test from_node with invalid elements
        let mixed_array = Node::Array(vec![
            Node::String("tag5".to_string()),
            Node::Bool(true), // This should be filtered out
            Node::String("tag6".to_string()),
        ]);
        let parsed = TestTags::from_node(&mixed_array).unwrap();
        assert_eq!(parsed.0, vec!["tag5", "tag6"]);

        // Test from_node with invalid node
        let string_node = Node::String("invalid".to_string());
        assert!(TestTags::from_node(&string_node).is_none());

        // Test clone_trait
        let cloned = tags.clone_trait();
        let downcast = cloned.as_any().downcast_ref::<TestTags>().unwrap();
        assert_eq!(downcast.0, vec!["tag1", "tag2"]);
    }
}
