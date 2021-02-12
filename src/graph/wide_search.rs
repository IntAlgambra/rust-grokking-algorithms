use std::{collections::HashMap, hash::Hash};
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn wide_search<T: Eq + Hash>(graph: &HashMap<T, Vec<T>>, center: T, target: T) -> bool {
    let neigbours = graph.get(&center).unwrap();
    let mut queue = VecDeque::new();
    let mut checked: HashSet<&T> = HashSet::new();
    for neighbour in neigbours {
        queue.push_back(neighbour)
    }
    while queue.len() > 0 {
        println!("len: {}", queue.len());
        let friend = queue.pop_front().unwrap();
        if checked.contains(friend) {
            continue;
        }
        checked.insert(friend);
        if friend == &target {
            return true;
        }
        for neighbour in graph.get(friend).unwrap() {
            queue.push_back(neighbour);
        }
    }
    false
}

#[test]
fn test_wide_search() {
    let mut test_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    test_graph.insert("You", vec!["Bob", "Claire", "Alice"]);
    test_graph.insert("Alice", vec!["Peggy", "You"]);
    test_graph.insert("Peggy", vec!["Alice", "Bob"]);
    test_graph.insert("Anudj", vec!["Bob"]);
    test_graph.insert("Bob", vec!["Anudj", "Peggy", "You"]);
    test_graph.insert("Claire", vec!["You", "Tom", "Jonny"]);
    test_graph.insert("Tom", vec!["You"]);
    test_graph.insert("Jonny", vec!["You"]);
    test_graph.insert("Vanda", vec!["Vision", "Steven"]);
    test_graph.insert("Steven", vec!["Tony", "Bruce", "Natasha"]);
    let check_1 = wide_search(&test_graph, "You", "Tom");
    let check_2 = wide_search(&test_graph, "You", "Natasha");
    assert!(check_1);
    assert!(!check_2);
}