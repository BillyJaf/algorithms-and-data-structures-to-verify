pub mod merge_sort;

#[cfg(test)]
mod tests {
    use crate::concurrent::merge_sort::merge_sort::merge_sort_concurrent;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn simple_sort() {
        let mut arr = vec![2,8,0,4,0,3];
        merge_sort_concurrent(&mut arr, 2);
        assert_eq!(arr, vec![0,0,2,3,4,8]);
    }

    #[test]
    fn medium_sort() {
        let mut arr_shuffled: Vec<i32> = (1..=1024).collect();
        let mut rng = thread_rng();
        arr_shuffled.shuffle(&mut rng);
        merge_sort_concurrent(&mut arr_shuffled, 16);

        let arr_sorted: Vec<i32> = (1..=1024).collect();

        assert_eq!(arr_shuffled, arr_sorted);
    }

    #[test]
    fn large_sort() {
        let mut arr_shuffled: Vec<i32> = (1..=65536).collect();
        let mut rng = thread_rng();
        arr_shuffled.shuffle(&mut rng);

        merge_sort_concurrent(&mut arr_shuffled, 16);

        let arr_sorted: Vec<i32> = (1..=65536).collect();

        assert_eq!(arr_shuffled, arr_sorted);
    }
}