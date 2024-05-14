use std::io;
use std::io::Write;
use rand::{thread_rng};
use rand::seq::SliceRandom;

const UPPERCASE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL_CHARS: &[u8] = b")(*&^%$#@!~";

fn main() {

    let length = get_password_length();
    let password = generate_password(length);
    println!("Generated password: {}", password);
    println!("Length of password: {}", password.len());
}

fn get_password_length() -> u32 {
    let mut input = String::new();

    println!("Enter the length of your password: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Invalid input. Please enter a number between 12 and 64.");
        std::process::exit(1);
    })
}

fn generate_password(length: u32) -> String {
    let charsets: [&[u8]; 4] = [
        UPPERCASE_LETTERS,
        LOWERCASE_LETTERS,
        NUMBERS,
        SPECIAL_CHARS,
    ];

    let mut rng1 = thread_rng();

    let mut merged = Vec::new();

    // Ensure we have at least 1 character of each type
    for charset in charsets.iter() {
        merged.push(*charset.choose(&mut rng1).unwrap());
    }

    // Fill the rest of the password randomly
    let all_chars: Vec<u8> = charsets.concat();merged.extend((0..length-4).map(|_| all_chars.choose(&mut rng1)).flatten());

    let mut rng2 = thread_rng();
    merged.shuffle(&mut rng2);

    return merged.into_iter().map(|b| b as char).collect();
}

