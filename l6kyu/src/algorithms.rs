//!
//! Modules categorized by Codewars labels - 6kyu *** Algorithms ***
//!

use md5::{Digest, Md5};
use shared::kata::*;

/// Given is a md5 hash of a five digits long PIN. It is given as string.
/// Your task is to return the cracked PIN as string.
/// # Example
/// ``` crack("827ccb0eea8a706c4c34a16891f84e7b"); ``` <br>
/// returns Ok(12345)
pub fn crack(string: String) -> Result<i32, ()> {
    let kata = Kata {
        level: Level::L6kyu,
        tags: vec![Tag::Algorithms, Tag::Cryptography],
        description: String::from("Crack the PIN"),
    };

    println!(
        "Level: {:?}, Tags: {:?}, Description: {}",
        kata.level, kata.tags, kata.description
    );

    let mut pin = String::new();

    for i in 0..100000 {
        pin = format!("{:05}", i);

        let mut hasher = Md5::new();
        hasher.update(pin.as_bytes());
        let result = hasher.finalize();
        let pin_hash = hex::encode(result);

        if pin_hash == string {
            break;
        }
    }

    Ok(pin.parse().expect("not a number"))
}
