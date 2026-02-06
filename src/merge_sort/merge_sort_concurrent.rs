use crate::merge_sort::merge_sort_sequential::merge_sort_sequential;
use std::thread;

pub fn merge_sort_concurrent<T>(arr: &mut [T], num_threads: usize)
    where T: Eq + PartialOrd + Copy + Send
{
    let mut rounded_up_power_of_two_length = 1;
    while rounded_up_power_of_two_length < arr.len() {
        rounded_up_power_of_two_length *= 2;
    }

    let chunk_size = rounded_up_power_of_two_length / num_threads;
    let handles = Vec::with_capacity(num_threads);

    for i in 0..(num_threads-1) {
        let (left, right) = arr.split_at_mut(i*chunk_size);
        handles.push(
            thread::spawn(move || {
                merge_sort_sequential(left);
                return left;
            })
        );
        arr = right;
    }

    handles.push(
        thread::spawn(move || {
            merge_sort_sequential(arr);
            return arr;
        })
    );

}