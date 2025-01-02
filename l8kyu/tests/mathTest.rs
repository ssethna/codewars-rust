#[cfg(test)]
use l8kyu::math::*;

mod tests {
    use super::*;

    #[test]
    fn summation_test() {
        let result = summation(8);
        assert_eq!(result, 36);
    }
}
