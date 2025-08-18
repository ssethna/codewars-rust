//!
//! Modules categorized by Codewars labels - 8kyu *** Strings ***
//!

use shared::kata::*;

/// Your goal is to write a function that removes the first and last
/// characters of a string. You're given one parameter, the original string.
/// Important: Your function should handle strings of any length ≥ 2 characters.
/// For strings with exactly 2 characters, return an empty string.
/// # Example
/// ``` remove_char("eloquent"); ``` <br>
/// returns "loquen"
pub fn remove_char(s: &str) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Strings, Tag::Fundamentals],
        description: String::from("Remove First and Last Character"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    s[1..s.len() - 1].to_string()
}

/// Implement a function which convert the given boolean value into its
/// string representation.
/// # Example
/// ``` boolean_to_string(true); ``` <br>
/// returns "true"
pub fn boolean_to_string(b: bool) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Strings, Tag::Fundamentals],
        description: String::from("Convert a Boolean to a String"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    b.to_string()
}
