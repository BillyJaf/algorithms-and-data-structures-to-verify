pub mod counting_to_n;

#[cfg(test)]
mod tests {
    use crate::concurrent::counting_to_n::counting_to_n::count_to_n;

    #[test]
    fn did_it_count_small() {
        let count = count_to_n(2);
        assert_eq!(2, count);
    }

    #[test]
    fn did_it_count_medium() {
        let count = count_to_n(64);
        assert_eq!(64, count);
    }

    #[test]
    fn did_it_count_large() {
        let count = count_to_n(1024);
        assert_eq!(1024, count);
    }
}