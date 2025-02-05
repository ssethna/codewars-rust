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
}
