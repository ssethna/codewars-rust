//!
//! Modules categorized by Codewars labels - 8kyu *** Arrays ***
//!

use shared::kata::*;

/// Complete the square sum function so that it squares each number passed into it
/// and then sums the results together.
/// # Example
/// ``` square_sum(vec![5, 3, 4]); ``` <br>
/// returns 50
pub fn square_sum(vec: Vec<i32>) -> i32 {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Arrays, Tag::Lists, Tag::Fundamentals],
        description: String::from("Square(n) Sum"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    vec.iter().map(|&x| x * x).sum()
}
