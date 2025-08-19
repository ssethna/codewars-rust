//!
//! Modules categorized by Codewars labels - 8kyu *** Arrays ***
//!

use shared::kata::*;

/// You are given two sorted arrays that contain only integers.
/// These arrays may be sorted in either ascending or descending order.
/// Your task is to merge them into a single array, ensuring that:
/// The resulting array is sorted in ascending order. Any duplicate values
/// are removed, so each integer appears only once. If both input arrays are
/// empty, return an empty array. No input validation is needed, as both
/// arrays are guaranteed to contain zero or more integers.
pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Arrays, Tag::Fundamentals],
        description: String::from("Merge two sorted arrays into one"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    // let mut vec1 = arr1.to_vec();
    // let mut vec2 = arr2.to_vec();

    // if !vec1.is_sorted() {
    //     vec1.reverse();
    // }
    // if !vec2.is_sorted() {
    //     vec2.reverse();
    // }

    // let mut merged = vec1;
    // merged.extend(vec2);

    // merged.sort();
    // merged.dedup();

    // Found better solution on cw

    let mut merged = [arr1, arr2].concat();
    merged.sort();
    merged.dedup();

    merged
}

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
