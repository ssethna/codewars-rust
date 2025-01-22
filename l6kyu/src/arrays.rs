//!
//! Modules categorized by Codewars labels - 6kyu *** Arrays ***
//!

use shared::kata::*;

/// Write a function that accepts an array of 10 integers (between 0 and 9), 
/// that returns a string of those numbers in the form of a phone number.
/// # Example
/// ``` create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]); ``` <br>
/// returns "(123) 456-7890"
pub fn create_phone_number(numbers: &[u8]) -> String {
    let kata = Kata {
        level: Level::L6kyu,
        tags: vec![Tag::Arrays, Tag::Strings, Tag::Regex, Tag::Algorithms],
        description: String::from("Create Phone Number"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    format!("({}{}{}) {}{}{}-{}{}{}{}", 
        numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], 
        numbers[6], numbers[7], numbers[8], numbers[9])

}
