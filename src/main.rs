use std::io;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

fn main() {
    println!("Generating a new password");

    println!("Enter the length of your password: ");

    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");

    let length: u32 = match length.trim().parse() {
        Ok(num) if num >= 12 && num <= 64 => num,
        Ok(num) => {
            println!("Password length should be between 12 and 64 characters: {num}");
            return;
        }
        Err(_) => {
            println!("Invalid input. Please enter a number between 12 and 64.");
            return;
        }
    };

    const UPPERCASE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &[u8] = b"0123456789";
    const SPECIAL_CHARS: &[u8] = b")(*&^%$#@!~";
    let mut charsets: [&[u8]; 4] = [
        UPPERCASE_LETTERS,
        LOWERCASE_LETTERS,
        NUMBERS,
        SPECIAL_CHARS,
    ];

    // Shuffle charsets to make password more random
    let mut rng_original_charset = thread_rng();
    charsets.shuffle(&mut rng_original_charset);

    let mut remaining_length = length;
    let mut merged = Vec::new();
    for (i, charset) in charsets.iter().enumerate() {
        let chunk_size = if i == charsets.len() - 1 {
            remaining_length
        } else {
            remaining_length / (charsets.len() - i) as u32
        };
        merged.extend_from_slice(&collect_chars(charset, chunk_size));
        remaining_length -= chunk_size;
    }

    let mut rng_final_charset = thread_rng();
    merged.shuffle(&mut rng_final_charset);

    let password: String = merged.into_iter().map(|b| b as char).collect();

    println!("Generated password: {}", password);
    println!("Length of password: {}", password.len());
}

fn collect_chars(chars: &[u8], length: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut result = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let idx = rng.gen_range(0..chars.len());
        result.push(chars[idx]);
    }

    result
}