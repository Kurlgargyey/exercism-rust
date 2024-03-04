pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::*;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    pub mod graph_items {
        use std::collections::HashMap;
        pub struct Edge {
            nodes: Vec<Node>,
            attrs: HashMap<String, String>,
        }
        pub struct Node;
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: HashMap::<String, String>::new(),
            }
        }
        pub fn with_nodes(&self, nodes: &[&str]) -> Self {
            todo!("with nodes")
        }
        pub fn with_attrs(&self, attrs: &Vec<&str>) -> Self {
            todo!("with attrs")
        }
        pub fn with_edges(&self, edges: &Vec<(&str, &str)>) -> Self {
            todo!("with edges")
        }
    }
}
