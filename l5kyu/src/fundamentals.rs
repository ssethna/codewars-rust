//!
//! Modules categorized by Codewars labels - 5kyu *** Fundamentals ***
//!

use shared::kata::*;

/// A squared string has n lines, each substring being n characters long.
/// For example: s = "abcd\nefgh\nijkl\nmnop" is a squared string of size 4.
/// We will use squared strings to code and decode texts. To make things
/// easier we suppose that our original text doesn't include the character '\n'.
/// # Example
/// ``` code("Alan Turing") ``` <br>
/// returns "i A\nnTl\ngua\nrn"
pub fn code(s: &str) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![
            Tag::Fundamentals,
            Tag::Strings,
            Tag::Ciphers,
            Tag::Cryptography,
        ],
        description: String::from("Coding with Squared Strings - code"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    let n = (s.len() as f32).sqrt().ceil() as usize;
    println!("l is {} and n is {n}", s.len());

    // Step 2: Pad the text with ASCII char 11 (vertical tab, \x0b)
    // until the length is n * n
    let mut padded_s = s.to_string();
    while padded_s.len() < n * n {
        padded_s.push(char::from(11)); // ASCII code 11
    }

    // Step 3: Convert the padded string into a square of size n
    let mut square = Vec::with_capacity(n);
    for i in 0..n {
        square.push(padded_s[i * n..(i + 1) * n].to_string());
    }

    // Step 4: Rotate the square 90 degrees clockwise
    let mut rotated_square = vec![String::new(); n];
    for i in 0..n {
        for j in 0..n {
            rotated_square[j].push(square[n - 1 - i].chars().nth(j).unwrap());
        }
    }

    // Print rotated square
    // println!("Rotated square:");
    // for row in &rotated_square {
    //     let row_with_escape = row.replace("\x0b", "\\x0b"); // Show vertical tab as text
    //     println!("{}", row_with_escape);
    // }

    // Step 5: Return the rotated square as a single string
    rotated_square.join("\n")
}

/// A squared string has n lines, each substring being n characters long.
/// For example: s = "abcd\nefgh\nijkl\nmnop" is a squared string of size 4.
/// We will use squared strings to code and decode texts. To make things
/// easier we suppose that our original text doesn't include the character '\n'.
/// # Example
/// ``` code("\x0bi A\n\x0bnTl\n\x0bgua\n\x0b\x0brn") ``` <br>
/// returns "Alan Turing"
pub fn decode(s: &str) -> String {
    let kata = Kata {
        level: Level::L8kyu,
        tags: vec![
            Tag::Fundamentals,
            Tag::Strings,
            Tag::Ciphers,
            Tag::Cryptography,
        ],
        description: String::from("Coding with Squared Strings - decode"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    // Step 1: Calculate the size of the square (n)
    let len = s.len();
    let n = (len as f64).sqrt() as usize; // Derive n from the length of the string

    // Step 2: Convert the squared string `s` into rows (split by newlines)
    let rows: Vec<String> = s.lines().map(|line| line.to_string()).collect();

    // Ensure the correct number of rows and columns
    if rows.len() != n {
        panic!("The number of rows does not match the expected size");
    }

    // Step 3: Apply the counter-clockwise rotation
    let mut decoded_square = vec![String::new(); n];
    for i in 0..n {
        for j in 0..n {
            // The counter-clockwise rotation (this is the reverse of a clockwise rotation)
            decoded_square[j].push(rows[i].chars().nth(n - 1 - j).unwrap());
        }
    }

    // Step 4: Join the decoded square into a single string and clean up the padding
    let decoded_string: String = decoded_square.concat(); // Concatenate the rows into a single string

    // Step 5: Remove any padding (ASCII char 11 - vertical tab)
    decoded_string.trim_matches(char::from(11)).to_string()
}
