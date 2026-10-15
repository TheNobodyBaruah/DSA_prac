use std::io::{self, Read};
use std::str;

fn multiply(n: u32, i: u32) -> u32 {
    n * i
}

fn main() {
    // NOTE: Reading user input without allocating heap memory
    // Reding bytes directly and then converting it to `&str`
    let mut user_input = [0u8; 3];

    let bytes_read = io::stdin()
        .read(&mut user_input)
        .expect("Failed to read buffer!");

    let input_text = str::from_utf8(&user_input[..bytes_read]).expect("Input was not valid text!");

    let number_from_user_input: u32 = match input_text.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("That was not a valid number!");
            return;
        }
    };
    // println!("{number_from_user_input}");

    for i in 1..=10 {
        let product: u32 = multiply(number_from_user_input, i);
        println!("{number_from_user_input} * {i} = {product}");
    }
}
