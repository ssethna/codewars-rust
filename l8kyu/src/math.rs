//!
//! Modules categorized by Codewars labels - 8kyu *** Mathematics ***
//!

use shared::kata::*;

/// Write a program that finds the summation of every number from 1 to num.
/// The number will always be a positive integer greater than 0.
/// Using Arithmetic Series Sum Formula - Carl Friedrich Gauss.
/// # Example
/// ``` summation(8); ``` <br>
/// returns 36
pub fn summation(n: i32) -> i32 {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals, Tag::Mathematics],
        description: String::from("Grasshopper - Summation"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );
    
    n*(n+1)/2
}
