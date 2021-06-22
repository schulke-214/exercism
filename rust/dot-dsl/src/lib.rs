pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct Graph<'a> {
        pub attrs: HashMap<String, String>,
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self {
                attrs: HashMap::new(),
                nodes: vec![],
                edges: vec![],
            }
        }

        pub fn with_nodes(&self, nodes: &Vec<Node<'a>>) -> Self {
            Self {
                attrs: self.attrs.clone(),
                nodes: nodes.clone(),
                edges: self.edges.clone(),
            }
        }

        pub fn with_edges(&self, edges: &'a Vec<Edge<'a>>) -> Self {
            Self {
                attrs: self.attrs.clone(),
                nodes: self.nodes.clone(),
                edges: edges.clone(),
            }
        }

        pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
            Self {
                attrs: attrs
                    .iter()
                    .map(|(name, val)| (name.to_string(), val.to_string()))
                    .collect(),
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
                attrs: Option<HashMap<String, String>>,
            }
            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Self {
                        from,
                        to,
                        attrs: None,
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        from: self.from,
                        to: self.to,
                        attrs: Some(
                            attrs
                                .iter()
                                .map(|(name, value)| (name.to_string(), value.to_string()))
                                .collect(),
                        ),
                    }
                }

                pub fn get_attr(&self, attr_id: &str) -> Option<&str> {
                    self.attrs
                        .as_ref()?
                        .iter()
                        .find(|a| *a.0 == attr_id)
                        .map(|a| a.1.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node<'a> {
                pub name: &'a str,
                pub attrs: Option<HashMap<String, String>>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Self { name, attrs: None }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        name: self.name.clone(),
                        attrs: Some(
                            attrs
                                .iter()
                                .map(|(name, value)| (name.to_string(), value.to_string()))
                                .collect(),
                        ),
                    }
                }

                pub fn get_attr(&self, attr_id: &str) -> Option<&str> {
                    self.attrs
                        .as_ref()?
                        .iter()
                        .find(|a| *a.0 == attr_id)
                        .map(|a| a.1.as_str())
                }
            }
        }
    }
}
