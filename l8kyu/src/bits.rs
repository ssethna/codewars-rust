//!
//! Modules categorized by Codewars labels - 8kyu *** Bits ***
//!

use shared::kata::*;

/// You are given a 32-bit float (single precision).
/// Return the number a signed 32-bit integer would hold 
/// with the same bit pattern.
/// # Example
/// ``` 10.0 (f32) == 01000001001000000000000000000000 (binary) ``` <br>
/// returns 1092616192 (i32)
pub fn convert_to_i32(f: f32) -> i32 {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Bits, Tag::Fundamentals],
        description: String::from("Casting binary float to integer"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    // unsafe { std::mem::transmute(f) } option 1
    f.to_bits() as i32  // option 2
}