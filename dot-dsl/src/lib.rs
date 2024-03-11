pub mod graph {
    macro_rules! implement_attrs {
        () => {
                pub fn attr(&self, attr: &str) -> Option<&str> {
                if let Some(attr) = self.attrs.get(attr) { Some(attr) } else { None }
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
        };
    }

    macro_rules! implement_edges {
        () => {
            pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        }
        };
    }

    macro_rules! implement_nodes {
        () => {
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
        };
    }

    use std::collections::HashMap;
    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;

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
        implement_nodes!();
        implement_edges!();
        implement_attrs!();
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use super::node::Node;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                nodes: Vec<Node>,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    let mut nodes = Vec::<Node>::new();
                    nodes.push(Node::new(node1));
                    nodes.push(Node::new(node2));
                    Edge {
                        nodes,
                        attrs: HashMap::<String, String>::new(),
                    }
                }
                implement_nodes!();
                implement_attrs!();
            }
        }
        pub mod node {
            use std::collections::HashMap;
            use super::edge::Edge;

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

                implement_edges!();
                implement_attrs!();
            }
        }
    }
}
