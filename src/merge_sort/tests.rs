use crate::merge_sort::merge_sort_sequential::merge_sort_sequential;
use crate::merge_sort::merge_sort_concurrent::merge_sort_concurrent;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[test]
fn simple_sort_sequential() {
    let mut arr = vec![2,8,0,4,0,3];
    merge_sort_sequential(&mut arr);
    assert_eq!(arr, vec![0,0,2,3,4,8]);
}

#[test]
fn longer_sort_sequential() {
    let mut arr = vec![15,13,8,9,10,12,14,0,1,2,3,6,5,7,4,11];
    merge_sort_sequential(&mut arr);
    assert_eq!(arr, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
}

#[test]
fn large_sort_sequential() {
    let mut arr_shuffled: Vec<i32> = (1..=1024).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);
    merge_sort_sequential(&mut arr_shuffled);

    let arr_sorted: Vec<i32> = (1..=1024).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}

#[test]
fn simple_sort_concurrent() {
    let mut arr = vec![2,8,0,4,0,3];
    merge_sort_concurrent(&mut arr, 8);
    assert_eq!(arr, vec![0,0,2,3,4,8]);
}