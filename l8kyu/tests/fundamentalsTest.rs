#[cfg(test)]
use l8kyu::fundamentals::is_love;

mod tests {
    use super::*;

    #[test]
    fn is_love_test() {
        let result = is_love(5, 6);
        assert_eq!(result, true);
    }
}
