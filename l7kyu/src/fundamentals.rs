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
