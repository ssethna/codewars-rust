//!
//! Modules categorized by Codewars labels - 7kyu *** Strings ***
//!

use shared::kata::*;

/// Complete the solution so that it returns true if the first
/// argument(string) passed in ends with the 2nd argument (also a string).
/// # Example
/// ``` solution("abc", "bc"); ``` <br>
/// returns "true"
pub fn solution(word: &str, ending: &str) -> bool {
    let kata = Kata {
        level: Level::L7kyu,
        tags: vec![Tag::Strings, Tag::Fundamentals],
        description: String::from("String ends with?"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    word.ends_with(ending)
}
