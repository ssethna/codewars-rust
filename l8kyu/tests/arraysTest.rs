#[cfg(test)]
use l8kyu::arrays::*;

mod tests {
    use super::*;

    #[test]
    fn square_sum_test() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
        assert_eq!(square_sum(vec![]), 0);
    }
}