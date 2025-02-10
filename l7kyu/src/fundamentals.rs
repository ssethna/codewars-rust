//!
//! Modules categorized by Codewars labels - 7kyu *** Fundamentals ***
//!

use shared::kata::*;

fn substitute(c: char) -> char {
    match c {
        'G' => 'A',
        'D' => 'E',
        'R' => 'Y',
        'P' => 'O',
        'L' => 'U',
        'K' => 'I',
        'g' => 'a',
        'd' => 'e',
        'r' => 'y',
        'p' => 'o',
        'l' => 'u',
        'k' => 'i',
        'A' => 'G',
        'E' => 'D',
        'Y' => 'R',
        'O' => 'P',
        'U' => 'L',
        'I' => 'K',
        'a' => 'g',
        'e' => 'd',
        'y' => 'r',
        'o' => 'p',
        'u' => 'l',
        'i' => 'k',
        _ => c,
    }
}

/// The GADERYPOLUKI is a simple substitution cipher used in scouting to
/// encipher messages. The encryption is based on short, easy to remember key.
/// The key is written as paired letters, which are in the cipher simple
/// replacement. The most frequently used key is "GA-DE-RY-PO-LU-KI".
/// Your task is to help scouts to encrypt and decrypt thier messages.
/// Write the Encipher and Decipher functions.
/// # Example
/// ``` encipher("ABCD") ``` <br>
/// returns "GBCE"
pub fn encipher(text: &str) -> String {
    let kata = Kata {
        level: Level::L7kyu,
        tags: vec![Tag::Fundamentals, Tag::Ciphers, Tag::Cryptography],
        description: String::from("GA-DE-RY-PO-LU-KI cipher"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    let enciphered: String = text.chars().map(substitute).collect();
    enciphered
}

/// The GADERYPOLUKI is a simple substitution cipher used in scouting to
/// encipher messages. Write the decrypt function.
/// # Example
/// ``` decipher("GBCE") ``` <br>
/// returns "ABCD"
pub fn decipher(text: &str) -> String {
    let kata = Kata {
        level: Level::L7kyu,
        tags: vec![Tag::Fundamentals, Tag::Ciphers, Tag::Cryptography],
        description: String::from("GA-DE-RY-PO-LU-KI cipher"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    let deciphered: String = text.chars().map(substitute).collect();
    deciphered
}

fn substitute_num(c: char) -> i32 {
    if c.is_ascii_alphabetic() {
        (c.to_ascii_uppercase() as i32) - ('A' as i32) + 1
    } else {
        0
    }
}

/// Digital Cypher assigns to each letter of the alphabet a unique number.
/// Instead of letters in encrypted word we write the corresponding number.
/// Then we add to each obtained digit consecutive digits from the key.
/// Write a function that accepts string and key number and returns an
/// array of integers.
/// Write the Encipher function.
/// # Example
/// ``` encipher("scout",1939) ``` <br>
/// returns [ 20, 12, 18, 30, 21]
pub fn digital_encipher(msg: String, n: i32) -> Vec<i32> {
    let kata = Kata {
        level: Level::L7kyu,
        tags: vec![Tag::Fundamentals, Tag::Ciphers, Tag::Cryptography],
        description: String::from("Digital cipher"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    let nums: Vec<i32> = msg.chars().map(substitute_num).collect();

    let n_str = n.to_string();
    let n_digits: Vec<i32> = n_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    nums.into_iter()
        .enumerate()
        .map(|(i, v)| v + n_digits[i % n_digits.len()])
        .collect()
}
