#[cfg(test)]
use l8kyu::arrays::*;

mod tests {
    use super::*;

    #[test]
    fn merge_arrays_test() {
        assert_eq!(
            merge_arrays(&[1, 2, 3, 4], &[5, 6, 7, 8]),
            &[1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(
            merge_arrays(&[1, 3, 5, 7, 9], &[10, 8, 6, 4, 2]),
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        assert_eq!(
            merge_arrays(&[1, 3, 5, 7, 9, 11, 12], &[1, 2, 3, 4, 5, 10, 12]),
            &[1, 2, 3, 4, 5, 7, 9, 10, 11, 12]
        );
    }

    #[test]
    fn first_non_consecutive_test() {
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 6, 7, 8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 5, 6, 7, 8]), None);
        assert_eq!(first_non_consecutive(&vec![4, 6, 7, 8, 9, 11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4, 5, 6, 7, 8, 9, 11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5, -4, -3, -1]), Some(-1));
    }

    #[test]
    fn square_sum_test() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
        assert_eq!(square_sum(vec![]), 0);
    }
}
