pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    #[derive(Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use crate::graph::Node;
            #[derive(Debug, PartialEq, Eq)]
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
            }
        }
        pub mod node {
            use std::collections::HashMap;
            use crate::graph::Edge;
            #[derive(Debug, PartialEq, Eq)]
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
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: HashMap::<String, String>::new(),
            }
        }
        pub fn with_nodes(&self, nodes: &Vec<Node>) -> Self {
            todo!("with nodes")
        }
        pub fn with_attrs(&self, attrs: &Vec<(&str, &str)>) -> Self {
            todo!("with attrs")
        }
        pub fn with_edges(&self, edges: &Vec<Edge>) -> Self {
            todo!("with edges")
        }
    }
}
