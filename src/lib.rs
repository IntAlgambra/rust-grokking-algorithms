mod search;
mod sort;
mod graph;

pub use search::simple_search;
pub use search::bin_search;
pub use sort::selection_sort;
pub use sort::quick_sort;
pub use graph::wide_search;
pub use graph::dijkstra;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
