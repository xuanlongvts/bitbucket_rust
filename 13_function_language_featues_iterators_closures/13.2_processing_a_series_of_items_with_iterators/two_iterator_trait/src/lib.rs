#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        // Methods that Consume the Iterator
        assert_eq!(total, 6);
    }

    #[test]
    fn method_produce_other_iterators() {
        let v2: Vec<i32> = vec![1, 2, 3];
        let v2_iter: Vec<i32> = v2.iter().map(|x| x + 1).collect();
        assert_eq!(v2_iter, vec![2, 3, 4]);
    }
}
