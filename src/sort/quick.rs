use std::cmp::PartialEq;
use std::cmp::PartialOrd;

fn split_list<T: PartialEq + PartialOrd>(mut list: Vec<T>) -> (Vec<T>,  T, Vec<T>) {
    let middle_index = list.len() / 2;
    let mut  smaller = Vec::new();
    let mut  larger = Vec::new();
    let middle =  list.swap_remove(middle_index);
    while list.len() > 0 {
        let elem = list.pop().unwrap();
        if elem > middle {
            larger.push(elem);
        } else {
            smaller.push(elem);
        }
    }
    (smaller, middle, larger)
}

fn join<T: PartialEq + PartialOrd>(mut smaller: Vec<T>, middle: T, mut larger: Vec<T>) -> Vec<T> {
    let mut joined: Vec<T> = Vec::new();
    joined.append(&mut smaller);
    joined.push(middle);
    joined.append(&mut larger);
    joined
}

fn sort_base<T: PartialOrd + PartialEq>(mut list: Vec<T>) -> Vec<T> {
    if list.len() > 2 {
        panic!("list too long to be considered base case");
    } else if list.len() == 1 || list.len() == 0 {
        return list;
    } 
    if list[1] <= list[0] {
        list.swap(0, 1);
    }
    list
}

pub fn quick_sort<T: PartialOrd + PartialEq>(list: Vec<T>) -> Vec<T> {
    if list.len() < 3 {
        return sort_base(list);
    }
    let (smaller, middle, larger) = split_list(list);
    join(quick_sort(smaller), middle, quick_sort(larger))
}

#[test]
fn test_quick_sort() {
    let test_list = vec![3, 5, 75, 23 ,77, 2];
    let sorted_vec= vec![2, 3, 5, 23, 75, 77];
    let sorted_by_func = quick_sort(test_list);
    assert!(
        sorted_vec.iter().eq(sorted_by_func.iter())
    )
}