use std::f64::INFINITY;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::graph::graph::Graph;

pub struct Path<'a> {
    path: Vec<&'a str>,
    weight: f64
}

impl<'a> Path<'a> {
    fn new(path: Vec<&'a str>, weight: f64) -> Path<'a> {
        Path {
            path,
            weight
        }
    }
    pub fn get_weight(&self) -> f64 {
        self.weight
    }
    pub fn get_path(&self) -> &Vec<&'a str>{
        &self.path
    }
 }

fn create_initial_weights<'a>(nodes: &HashSet<&'a str>, graph: &'a Graph, origin: &'a str) -> HashMap<&'a str, f64> {
    let mut weights = graph.get_weights(origin);
    weights.insert(origin, 0.0);
    for &node in nodes {
        match weights.get(node) {
            Some(_) => {
                continue;
            },
            None => {
                weights.insert(node, INFINITY);
            }
        }
    }
    weights
}

fn construct_path<'a>(
    parents: HashMap<&'a str, Option<&'a str>>, 
    origin: &'a str, 
    target: &'a str
) -> Vec<&'a str> {
    println!("parents: {:?}", parents);
    let mut path = vec![target];
    let mut current = target;
    while current != origin {
        current = parents.get(current).unwrap().unwrap();
        path.push(current);
    }
    path.into_iter().rev().collect()
}


pub fn dijkstra<'a>(graph: &'a Graph, origin: &'a str, target: &'a str ) -> Path<'a> {
    // Let's form vector of unprocessed nodes
    let mut unprocessed: HashSet<&'a str> = graph.get_nodes();
    unprocessed.remove(origin);
    // Let's form hash map of node's weights
    let mut weights: HashMap<&'a str, f64> = create_initial_weights(&unprocessed, &graph, &origin);
    // Let's form hash map with parent for each node
    let mut parents: HashMap<&'a str, Option<&'a str>> = weights.keys().fold(
        HashMap::new(),
        |mut acc, item| {
            acc.insert(item, Some(origin));
            acc
        }
    );
    while !unprocessed.is_empty() {
        // Finding the cheapest node
        let cheapest = weights.keys().fold(target, |acc, item| {
            if weights.get(item).unwrap() <= weights.get(acc).unwrap() && unprocessed.contains(item) {
                return item;
            }
            acc
        });
        // weight of cheapest nnode
        let &cheapest_weight = weights.get(cheapest).unwrap();
        // Updating weights of cheapest node neigbours
        for (node, current) in graph.get_weights(cheapest) {
            // if new node weight is samller, update weight and node parent
            if current + cheapest_weight < *weights.get(node).unwrap() {
                weights.insert(node, current + cheapest_weight);
                parents.insert(node, Some(cheapest));
            }
        }
        unprocessed.remove(cheapest);
    }
    let path = construct_path(parents, origin, target);
    let weight = *weights.get(target).unwrap();
    Path::new(path, weight)
}

#[cfg(test)]
mod test_dijkstra_2 {
    use super::dijkstra;
    use super::Graph;
    #[test]
    fn test_path_exists() {
        let mut graph = Graph::new();
        graph.add_node("book");
        graph.add_node("poster");
        graph.add_node("vinyl");
        graph.add_node("drums");
        graph.add_node("piano");
        graph.add_node("bass");
        graph.add_edge("book", "poster", 0.0);
        graph.add_edge("book", "vinyl", 5.0);
        graph.add_edge("poster", "drums", 35.0);
        graph.add_edge("poster", "bass", 30.0);
        graph.add_edge("vinyl", "bass", 15.0);
        graph.add_edge("vinyl", "drums", 20.0);
        graph.add_edge("bass", "piano", 20.0);
        graph.add_edge("drums", "piano", 10.0);
        let path = dijkstra(&graph, "book", "piano");
        println!("weight: {}", path.get_weight());
        println!("path: {:?}", path.get_path());
        assert_eq!(35.0, path.get_weight());
        assert_eq!(vec!["book", "vinyl", "drums", "piano"], *path.get_path());
    }
}