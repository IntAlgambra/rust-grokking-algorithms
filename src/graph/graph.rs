use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Edge<'a> {
    origin: &'a str,
    target: &'a str,
    weight: f64
}

impl<'a> Edge<'a> {
    fn new(origin: &'a str, target: &'a str, weight: f64) -> Edge<'a> {
        Edge {
            origin,
            target,
            weight
        }
    }
}

pub struct Graph<'a> {
    nodes: Vec<&'a str>,
    edges: Vec<Edge<'a>>
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }
    pub fn add_node(&mut self,node: &'a str) {
        if self.nodes.contains(&node) {
            panic!("Graph already contains this node")
        }
        self.nodes.push(node)
    }
    pub fn add_edge(&mut self, origin: &'a str, target: &'a str, weight: f64) {
        if !self.nodes.contains(&origin) || ! self.nodes.contains(&target) {
            panic!("Graph doesn't contain these nodes")
        }
        let edge_exists = self.edges.iter().try_fold(false, |acc, item| {
            if item.target == target && item.origin == origin {
                return Some(true);
            }
            Some(acc)
        }).unwrap();
        if edge_exists {
            panic!("Graph already has edge between these nodes")
        }
        let edge = Edge::new(origin, target, weight);
        self.edges.push(edge);
    }
    pub fn get_weights(&self, origin: &'a str) -> HashMap<&'a str, f64> {
        let mut weights: HashMap<&'a str, f64> = HashMap::new();
        for edge in &self.edges {
            if edge.origin == origin {
                weights.insert(&edge.target, edge.weight);
            }
        }
        weights
    }
    pub fn get_nodes(&self) -> HashSet<&'a str> {
        self.nodes.clone().into_iter().collect()
    }
}

#[cfg(test)]
mod test_graph_structure {

    use super::{Edge, Graph};

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new("lol", "kek", 42.0);
        assert_eq!(42.0, edge.weight);
        assert_eq!("lol", edge.origin);
        assert_eq!("kek", edge.target);
    }

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new();
        assert_eq!(0, graph.nodes.len());
        assert_eq!(0, graph.edges.len());
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node("lol");
        assert_eq!(1, graph.nodes.len())

    }
    
    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_node("lol");
        graph.add_node("kek");
        graph.add_edge("lol", "kek", 42.0);
        assert_eq!(1, graph.edges.len());
    }
}