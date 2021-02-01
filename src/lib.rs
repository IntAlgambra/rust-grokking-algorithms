mod search;
mod sort;

pub use search::simple_search;
pub use search::bin_search;
pub use sort::selection_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
