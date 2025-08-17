//!
//! Modules categorized by Codewars labels - 8kyu *** Arrays ***
//!

use shared::kata::*;

/// Your task is to find the first element of an array that is not consecutive.
/// If the whole array is consecutive then return null.
/// The array will always have at least 2 elements and all elements will be numbers.
/// The numbers will also all be unique and in ascending order. The numbers could
/// be positive or negative and the first non-consecutive could be either too!
/// # Example
/// ``` [1,2,3,4,6,7,8] ``` <br>
/// returns 6
pub fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Arrays, Tag::Fundamentals],
        description: String::from("Find the first non-consecutive number"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] != 1 {
            return Some(arr[i]);
        }
    }
    None
}

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
