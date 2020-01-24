pub mod graph {
    use std::collections::HashMap;
    use graph_items::edge::Edge;
    use graph_items::node::Node;

    #[derive(Default, Debug, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = Vec::from(nodes);
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = Vec::from(edges);
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for &(k, v) in attrs {
                self.attrs.insert(k.to_owned(), v.to_owned());
            }
            self
        }
        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&n| n.name == node_name)
        }
        pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
            self.attrs.get(attr_name).map(|s| s.as_str())
        }
    }
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_owned(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(k, v) in attrs {
                        self.attrs.insert(k.to_owned(), v.to_owned());
                    }
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_owned(),
                        to: to.to_owned(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(k, v) in attrs {
                        self.attrs.insert(k.to_owned(), v.to_owned());
                    }
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
    }
}
