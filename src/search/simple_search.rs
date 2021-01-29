use std::cmp::PartialEq;
use std::cmp::PartialOrd;

pub fn simple_search<T: PartialOrd + PartialEq>(list: Vec<T>, elem: T) -> Option<usize> {
    for (index, item) in list.iter().enumerate() {
        if *item == elem {
            return Some(index);
        }
    }
    None
}

#[test]
fn test_simple_search() {
    let test_vec = vec![1, 5, 2, 4, 3];
    let index = simple_search(test_vec, 5).unwrap();
    assert_eq!(1, index);
}