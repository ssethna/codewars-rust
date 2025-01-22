#[cfg(test)]
use l7kyu::strings::*;

mod tests {
    use super::*;

    #[test]
    fn solution_test() {
        assert_eq!(
            solution("abc", "bc"),
            true,
            "When 2nd string matches end of 1st string  \"true\" as output"
        );
        assert_eq!(
            solution("strawberry", "banana"),
            false,
            "When 2nd string does not match end of 1st string  \"false\" as output"
        );
    }
}
