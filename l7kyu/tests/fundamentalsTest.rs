#[cfg(test)]
use l7kyu::fundamentals::*;

mod tests {
    use super::*;

    #[test]
    fn encipher_test() {
        assert_eq!(encipher("ABCD"), "GBCE");
        assert_eq!(encipher("Ala has a cat"), "Gug hgs g cgt");
        assert_eq!(encipher("gaderypoluki"), "agedyropulik");
    }

    #[test]
    fn decipher_test() {
        assert_eq!(decipher("GBCE"), "ABCD");
        assert_eq!(decipher("Gug hgs g cgt"), "Ala has a cat");
        assert_eq!(decipher("agedyropulik"), "gaderypoluki");
    }

    #[test]
    fn encipher_digital_test() {
        assert_eq!(
            digital_encipher("scout".to_string(), 1939),
            vec![20, 12, 18, 30, 21]
        );
        assert_eq!(
            digital_encipher("masterpiece".to_string(), 1939),
            vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]
        );
    }
}
