use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::clone::Clone;

pub fn selection_sort<T: PartialOrd + PartialEq + Clone>(mut list: Vec<T>) -> Vec<T> {
    let mut sorted: Vec<T> = Vec::new();
    for _ in 0..list.len() {
        let mut smallest: usize = 0;
        for (index,item) in list.iter().enumerate() {
            if item < &list[smallest] {
                smallest = index;
            }
        }
        sorted.push(list.swap_remove(smallest))
    }
    sorted
}

#[test]
fn test_selection_sort() {
    let test_list = vec![3, 5, 75, 23 ,77, 2];
    let sorted_vec= vec![2, 3, 5, 23, 75, 77];
    let sorted_by_func = selection_sort(test_list);
    assert!(
        sorted_vec.iter().eq(sorted_by_func.iter())
    )
}