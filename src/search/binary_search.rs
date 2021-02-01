use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;

pub fn bin_search<T: PartialOrd + PartialEq + Debug>(list: Vec<T>, elem: T) -> Option<usize> {
    let mut low_index: usize = 0;
    let mut high_index: usize = list.len() - 1;
    while low_index <= high_index {
        let attempt: usize = (high_index + low_index) / 2;
        if list[attempt] == elem {
            return Some(attempt)
        } else if list[attempt] > elem {
            high_index = attempt - 1;
        } else {
            low_index = attempt + 1;
        }
    }
    None
}

#[test]
fn test_bin_search() {
    let list = vec![123, 432, 654, 999, 10432, 9403223];
    let index = bin_search(list, 999).unwrap();
    assert_eq!(index, 3);
}

#[test]
#[should_panic]
fn test_no_such_element() {
    let list = vec![123, 432, 654, 999, 10432, 9403223];
    bin_search(list, 888).unwrap();
}