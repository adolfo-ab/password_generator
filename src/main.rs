use std::io;
use rand::{thread_rng, Rng};

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
            println!("Password length should be between 12 and 64 characters.");
            return;
        }
        Err(_) => {
            println!("Invalid input. Please enter a number between 12 and 64.");
            return;
        }
    };


    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~\
                            0123456789)(*&^%$#@!~"; 
                            // Repeat numbers and special characters to increase the probability of choosing them
                            
                            

    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("Generated password: {:?}", password)
}
