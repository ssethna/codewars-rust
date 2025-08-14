//!
//! Modules categorized by Codewars labels - 8kyu *** Fundamentals ***
//!

use shared::kata::*;

/// Write a function that removes the spaces from the string,
/// then return the resultant string.
/// # Example
/// ``` "8 j 8   mBliB8g  imjB8B8  jl  B" ``` <br>
/// returns "8j8mBliB8gimjB8B8jlB"
pub fn no_space(x: String) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals, Tag::Strings],
        description: String::from("altERnaTIng cAsE <=> ALTerNAtiNG CaSe"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    // let no_spaces: String = x.chars().filter(|c| *c != ' ').collect();
    // no_spaces
    // Found better solution below.
    x.replace(" ", "")
}

/// Define String.prototype.toAlternatingCase (or a similar function/method
/// such as to_alternating_case/toAlternatingCase/ToAlternatingCase in your
/// selected language; see the initial solution for details) such that each
/// lowercase letter becomes uppercase and each uppercase letter becomes lowercase.
/// # Example
/// ``` "hello world".toAlternatingCase() ``` <br>
/// returns "HELLO WORLD"
pub fn to_alternating_case(s: &str) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals],
        description: String::from("altERnaTIng cAsE <=> ALTerNAtiNG CaSe"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// This function takes two numbers as parameters, the first number
/// being the coefficient, and the second number being the exponent.
/// The exponent will never be 1, and neither number will ever be 0.
/// # Example
/// ``` derive(7, 8) ``` <br>
/// returns "56x^7"
pub fn derive(coefficient: u32, exponent: u32) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals],
        description: String::from("Take the Derivative"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    format!("{}x^{}", coefficient * exponent, exponent - 1)
}

/// Your goal is to return multiplication table for number that is always
/// an integer from 1 to 10.
/// # Example
/// ``` multi_table(5) ``` <br>
/// returns "1 * 5 = 5\n2 * 5 = 10\n3 * 5 = 15\n4 * 5 = 20\n5 * 5 = 25\n
/// 6 * 5 = 30\n7 * 5 = 35\n8 * 5 = 40\n9 * 5 = 45\n10 * 5 = 50"
pub fn multi_table(n: u64) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals, Tag::Strings],
        description: String::from("Multiplication table for number"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    let mut mult_t = String::from("");
    for i in 1..10 {
        let line = format!("{} * {} = {}\n", i, n, i * n);
        mult_t.push_str(&line);
    }
    let last_line = format!("{} * {} = {}", 10, n, 10 * n);
    mult_t.push_str(&last_line);
    mult_t
}

/// Timmy & Sarah think they are in love, but around where they live, they will only
/// know once they pick a flower each. If one of the flowers has an even number of
/// petals and the other has an odd number of petals it means they are in love.
/// Write a function that will take the number of petals of each flower and
/// return true if they are in love and false if they aren't.
/// # Example
/// ``` is_love(5, 7); ``` <br>
/// returns false
pub fn is_love(flower1: u16, flower2: u16) -> bool {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![Tag::Fundamentals],
        description: String::from("Opposites Attract"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    if (flower1 + flower2) % 2 != 0 {
        return true;
    } else {
        return false;
    }
}
