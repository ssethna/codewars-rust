#[cfg(test)]
use l6kyu::algorithms::*;

mod tests {
    use super::*;

    #[test]
    fn crack_test() {
        assert_eq!(
            crack("827ccb0eea8a706c4c34a16891f84e7b".to_string()),
            Ok(12345)
        );
        assert_eq!(
            crack("86aa400b65433b608a9db30070ec60cd".to_string()),
            Ok(00078)
        );
    }
}
