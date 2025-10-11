#[cfg(test)]
use l8kyu::bits::*;

mod tests {
    use super::*;

    #[test]
    fn convert_to_i32_test() {
        assert_eq!(convert_to_i32(10.0), 1092616192);
        assert_eq!(convert_to_i32(f32::INFINITY), 0x7f800000);
        assert_eq!(convert_to_i32(1.40129846432e-44), 10)
    }
}
