//!
//! Modules categorized by Codewars labels *** Mathematics ***
//!

use crate::kata::Kata;
use crate::kata::Level;
use crate::kata::Tag;

/// Write a program that finds the summation of every number from 1 to num.
/// The number will always be a positive integer greater than 0.
/// Using Arithmetic Series Sum Formula - Carl Friedrich Gauss.
pub fn summation(n: i32) -> i32 {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals, Tag::Mathematics],
        description: String::from("Summation"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );
    
    n*(n+1)/2
}
