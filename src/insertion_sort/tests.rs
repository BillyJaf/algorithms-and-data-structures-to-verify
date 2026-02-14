use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::insertion_sort::insertion_sort::insertion_sort;

#[test]
fn small_sort_sequential() {
    let mut arr_shuffled: Vec<i32> = vec![2,8,0,4,0,3];
    insertion_sort(&mut arr_shuffled);
    assert_eq!(arr_shuffled, vec![0,0,2,3,4,8]);
}

#[test]
fn medium_sort_sequential() {
    let mut arr_shuffled: Vec<i32> = (1..=1024).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);
    insertion_sort(&mut arr_shuffled);

    let arr_sorted: Vec<i32> = (1..=1024).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}

#[test]
fn large_sort_sequential() {
    let mut arr_shuffled: Vec<i32> = (1..=16384).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);
    insertion_sort(&mut arr_shuffled);

    let arr_sorted: Vec<i32> = (1..=16384).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}