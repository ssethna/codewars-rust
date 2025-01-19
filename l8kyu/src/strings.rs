//!
//! Modules categorized by Codewars labels - 8kyu *** Strings ***
//!

use shared::kata::*;


/// Implement a function which convert the given boolean value into its 
/// string representation.
/// # Example
/// ``` boolean_to_string(true); ``` <br>
/// returns "true"
pub fn boolean_to_string(b: bool) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Strings, Tag::Fundamentals],
        description: String::from("Square(n) Sum"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    b.to_string()
}