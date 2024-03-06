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
    }

    impl Nodes for Graph {
        fn nodes(&mut self) -> &mut Vec<Node> {
            &mut self.nodes
        }
    }
    impl Edges for Graph {
        fn edges(&mut self) -> &mut Vec<Edge> {
            &mut self.edges
        }
    }
    impl Attributes for Graph {
        fn attrs(&mut self) -> &mut HashMap<String, String> {
            &mut self.attrs
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
            }

            impl Nodes for Edge {
                fn nodes(&mut self) -> &mut Vec<Node> {
                    &mut self.nodes
                }
            }
            impl Attributes for Edge {
                fn attrs(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            use super::edge::Edge;
            use crate::graph::builders::*;
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                name: String,
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
            }
            impl Edges for Node {
                fn edges(&mut self) -> &mut Vec<Edge> {
                    &mut self.edges
                }
            }
            impl Attributes for Node {
                fn attrs(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
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
