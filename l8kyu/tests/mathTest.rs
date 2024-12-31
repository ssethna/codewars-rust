#[cfg(test)]
use l8kyu::math::summation;

mod tests {
    use super::*;

    #[test]
    fn summation_test() {
        let result = summation(8);
        assert_eq!(result, 36);
    }
}
