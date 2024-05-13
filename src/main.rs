use std::io;
use std::io::Write;
use rand::thread_rng;
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
    let mut charsets: [&[u8]; 4] = [
        UPPERCASE_LETTERS,
        LOWERCASE_LETTERS,
        NUMBERS,
        SPECIAL_CHARS,
    ];

    let mut rng = thread_rng();
    charsets.shuffle(&mut rng);

    let mut merged = Vec::new();
    let mut remaining_length = length;

    for (i, charset) in charsets.iter().enumerate() {
        let chunk_size = if i == charsets.len() - 1 {
            remaining_length
        } else {
            remaining_length / (charsets.len() - i) as u32
        };
        merged.extend(charset.choose_multiple(&mut rng, chunk_size as usize).cloned());
        remaining_length -= chunk_size;
    }

    let mut rng_final_charset = thread_rng();
    merged.shuffle(&mut rng_final_charset);

    return merged.into_iter().map(|b| b as char).collect();
}