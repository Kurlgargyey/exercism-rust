pub mod graph {
    use std::collections::HashMap;
    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use self::builders::*;
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: HashMap::<String, String>::new(),
            }
        }
        pub fn node(&self, node: &str) -> Option<Node> {
            for self_node in &self.nodes {
                if self_node.name == node.to_string() {
                    return Some(self_node.clone());
                }
            }
            None
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        }
        pub fn attr(&self, attr: &str) -> Option<&String> {
            self.attrs.get(attr)
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (attr, value) in attrs {
                self.attrs
                    .entry(attr.to_string())
                    .and_modify(|e| {
                        *e = value.to_string();
                    })
                    .or_insert(value.to_string());
            }
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges().push(edge.clone());
            }
            self
        }
    }

    impl Edges for Graph {
        fn edges(&mut self) -> &mut Vec<Edge> {
            &mut self.edges
        }
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use super::node::Node;
            use crate::graph::builders::*;
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                nodes: Vec<Node>,
                attrs: HashMap<String, String>,
                edges: Vec<Edge>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    let mut nodes = Vec::<Node>::new();
                    nodes.push(Node::new(node1));
                    nodes.push(Node::new(node2));
                    Edge {
                        nodes,
                        attrs: HashMap::<String, String>::new(),
                        edges: Vec::<Edge>::new(),
                    }
                }
                pub fn node(&self, node: &str) -> Option<Node> {
                    for self_node in &self.nodes {
                        if self_node.name == node.to_string() {
                            return Some(self_node.clone());
                        }
                    }
                    None
                }
                pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
                    for node in nodes {
                        self.nodes.push(node.clone());
                    }
                    self
                }
                pub fn attr(&self, attr: &str) -> Option<&str> {
                    Some(self.attrs.get(attr).unwrap().as_str())
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (attr, value) in attrs {
                        self.attrs
                            .entry(attr.to_string())
                            .and_modify(|e| {
                                *e = value.to_string();
                            })
                            .or_insert(value.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            use super::edge::Edge;
            use crate::graph::builders::*;
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name: String,
                edges: Vec<Edge>,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    let name = name.to_string();
                    Node {
                        name,
                        edges: Vec::<Edge>::new(),
                        attrs: HashMap::<String, String>::new(),
                    }
                }
                pub fn attr(&self, attr: &str) -> Option<&str> {
                    Some(self.attrs.get(attr).unwrap().as_str())
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (attr, value) in attrs {
                        self.attrs
                            .entry(attr.to_string())
                            .and_modify(|e| {
                                *e = value.to_string();
                            })
                            .or_insert(value.to_string());
                    }
                    self
                }
                pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
                    for edge in edges {
                        self.edges.push(edge.clone());
                    }
                    self
                }
            }
        }
    }
    pub mod builders {
        use std::collections::HashMap;
        use crate::graph::graph_items::node::Node;
        use crate::graph::graph_items::edge::Edge;
        pub trait Nodes {
            fn nodes(&mut self) -> &mut Vec<Node>;
            fn with_nodes(&mut self, nodes: &Vec<Node>) -> &Self {
                for node in nodes {
                    self.nodes().push(node.clone());
                }
                self
            }
        }
        pub trait Edges {
            fn edges(&mut self) -> &mut Vec<Edge>;
            fn with_edges(&mut self, edges: &Vec<Edge>) -> &Self {
                for edge in edges {
                    self.edges().push(edge.clone());
                }
                self
            }
        }
        pub trait Attributes {
            fn attrs(&mut self) -> &mut HashMap<String, String>;
            fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> &Self {
                for (attr, value) in attrs {
                    self.attrs()
                        .entry(attr.to_string())
                        .and_modify(|e| {
                            *e = value.to_string();
                        })
                        .or_insert(value.to_string());
                }
                self
            }
        }
    }
}
