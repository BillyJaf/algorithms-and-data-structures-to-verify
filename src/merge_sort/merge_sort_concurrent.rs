use std::thread;

use super::merge_sort_sequential::merge_sort_sequential;

pub fn merge_sort_concurrent(arr: &mut [i32], num_threads: usize) {
    let mut power_of_two = 1;
    let mut threshold = 0;
    while power_of_two < num_threads {
        power_of_two *= 2;
        threshold += 1;
    }
    merge_sort(arr, threshold);
}

fn merge_sort(arr: &mut [i32], threshold: usize) {
    let mid_point = arr.len() / 2;

    if mid_point == 0 {
        return;
    }

    if threshold == 0 {
        merge_sort_sequential(arr);
        return;
    }

    let (left, right) = arr.split_at_mut(mid_point);

    let mut left_clone = Vec::from(left);
    let mut right_clone = Vec::from(right);

    let left_handle = thread::spawn(move || {
        merge_sort(&mut left_clone, threshold - 1);
        return left_clone;
    });

    let right_handle = thread::spawn(move || {
        merge_sort(&mut right_clone, threshold - 1);
        return right_clone;
    });

    let mut left_sorted = left_handle.join().unwrap();
    let mut right_sorted = right_handle.join().unwrap();
    let mut temp_vec = Vec::with_capacity(left_sorted.len() + right_sorted.len());

    merge(&mut left_sorted, &mut right_sorted, &mut temp_vec);
    arr.copy_from_slice(&temp_vec);
}

fn merge(left: &mut [i32], right: &mut [i32], temp_vec: &mut Vec<i32>) {   
    let mut left_pointer = 0;
    let mut right_pointer = 0;

    while left_pointer < left.len() && right_pointer < right.len() {
        if left[left_pointer] <= right[right_pointer] {
            temp_vec.push(left[left_pointer]);
            left_pointer += 1;
        } else {
            temp_vec.push(right[right_pointer]);
            right_pointer += 1;
        }
    }

    while left_pointer < left.len() {
        temp_vec.push(left[left_pointer]);
        left_pointer += 1;
    }

    while right_pointer < right.len() {
        temp_vec.push(right[right_pointer]);
        right_pointer += 1;
    }
}