#[cfg(test)]
use l8kyu::strings::*;

mod tests {
    use super::*;

    #[test]
    fn remove_char_test() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }

    #[test]
    fn boolean_to_string_test() {
        assert_eq!(
            boolean_to_string(true),
            "true",
            "When we pass in true, we want the string \"true\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
    }
}
