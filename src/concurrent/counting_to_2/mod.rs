pub mod counting_to_2;

#[cfg(test)]
mod tests {
    use crate::concurrent::counting_to_2::counting_to_2::count_to_2;

    #[test]
    fn did_it_count() {
        let count = count_to_2();
        assert_eq!(2, count);
    }
}