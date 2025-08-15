#[cfg(test)]
use l8kyu::fundamentals::*;

mod tests {
    use super::*;

    #[test]
    fn simple_multiplication_test() {
        assert_eq!(simple_multiplication(1), 9);
        assert_eq!(simple_multiplication(2), 16);
        assert_eq!(simple_multiplication(4), 32);
        assert_eq!(simple_multiplication(5), 45);
    }

    #[test]
    fn no_space_test() {
        assert_eq!(
            "8j8mBliB8gimjB8B8jlB",
            no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string())
        );
        assert_eq!(
            "88Bifk8hB8BB8BBBB888chl8BhBfd",
            no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string())
        );
        assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
        assert_eq!(
            "jfBmgklf8hg88lbe8",
            no_space("jfBm  gk lf8hg  88lbe8 ".to_string())
        );
        assert_eq!("8jaam", no_space("8j aam".to_string()));
    }

    #[test]
    fn to_alternating_case_test() {
        assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
        assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
        assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
        assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
        assert_eq!(
            "Hello World",
            to_alternating_case(&to_alternating_case("Hello World")[..])
        );
        assert_eq!("12345", to_alternating_case("12345"));
        assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
        assert_eq!(
            "sTRING.tOaLTERNATINGcASE",
            to_alternating_case("String.ToAlternatingCase")
        );
    }

    #[test]
    fn derive_test() {
        assert_eq!(derive(7, 8), "56x^7");
        assert_eq!(derive(5, 9), "45x^8");
    }

    #[test]
    fn multi_table_test() {
        assert_eq!(multi_table(5), "1 * 5 = 5\n2 * 5 = 10\n3 * 5 = 15\n4 * 5 = 20\n5 * 5 = 25\n6 * 5 = 30\n7 * 5 = 35\n8 * 5 = 40\n9 * 5 = 45\n10 * 5 = 50");
        assert_eq!(multi_table(1), "1 * 1 = 1\n2 * 1 = 2\n3 * 1 = 3\n4 * 1 = 4\n5 * 1 = 5\n6 * 1 = 6\n7 * 1 = 7\n8 * 1 = 8\n9 * 1 = 9\n10 * 1 = 10");
    }

    #[test]
    fn is_love_test() {
        let result = is_love(5, 6);
        assert_eq!(result, true);
    }
}
