use crate::merge_sort::merge_sort_sequential::merge_sort_sequential;
use crate::merge_sort::merge_sort_concurrent::merge_sort_concurrent;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

#[test]
fn small_sort_sequential() {
    let mut arr = vec![2,8,0,4,0,3];
    merge_sort_sequential(&mut arr);
    assert_eq!(arr, vec![0,0,2,3,4,8]);
}

#[test]
fn medium_sort_sequential() {
    let mut arr_shuffled: Vec<i32> = (1..=1024).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);
    merge_sort_sequential(&mut arr_shuffled);

    let arr_sorted: Vec<i32> = (1..=1024).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}

#[test]
fn large_sort_sequential() {
    let mut arr_shuffled: Vec<i32> = (1..=65536).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);
    merge_sort_sequential(&mut arr_shuffled);

    let arr_sorted: Vec<i32> = (1..=65536).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}

#[test]
fn simple_sort_concurrent() {
    let mut arr = vec![2,8,0,4,0,3];
    merge_sort_concurrent(&mut arr, 2);
    assert_eq!(arr, vec![0,0,2,3,4,8]);
}

#[test]
fn medium_sort_concurrent() {
    let mut arr_shuffled: Vec<i32> = (1..=1024).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);
    merge_sort_concurrent(&mut arr_shuffled, 16);

    let arr_sorted: Vec<i32> = (1..=1024).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}

#[test]
fn large_sort_concurrent() {
    let mut arr_shuffled: Vec<i32> = (1..=65536).collect();
    let mut rng = thread_rng();
    arr_shuffled.shuffle(&mut rng);

    merge_sort_concurrent(&mut arr_shuffled, 16);

    let arr_sorted: Vec<i32> = (1..=65536).collect();

    assert_eq!(arr_shuffled, arr_sorted);
}


#[test]
fn compare_large_sort() {
    let mut arr_shuffled_one: Vec<i32> = (1..=65536).collect();
    let mut rng = thread_rng();
    arr_shuffled_one.shuffle(&mut rng);
    let mut arr_shuffled_two = arr_shuffled_one.clone();

    let start = Instant::now();
    merge_sort_sequential(&mut arr_shuffled_one);
    let elapsed = start.elapsed();

    println!("Large sequential sort took: {:?}", elapsed);

    let start = Instant::now();
    merge_sort_concurrent(&mut arr_shuffled_two, 16);
    let elapsed = start.elapsed();

    println!("Large concurrent sort took: {:?}", elapsed);

    let arr_sorted: Vec<i32> = (1..=65536).collect();

    assert_eq!(arr_shuffled_one, arr_sorted);
    assert_eq!(arr_shuffled_two, arr_sorted);
}