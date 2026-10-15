use std::io::{Read, Write};
use std::str;

fn count_good_numbers(number_list: Vec<u32>) -> u32 {
    number_list
        .iter()
        .filter(|number| **number != 0 && (18 % *number == 0 || *number % 45 == 0))
        .count() as u32 // count counts the length of the vector
    // after filtering
}

fn main() {
    // Get user input
    // This would be the number of numbers that would be accepted
    // for being good numbers
    let mut buffer1 = [0u8; 16];

    let number_of_bytes1: usize = std::io::stdin()
        .read(&mut buffer1)
        .expect("Failed to read bytes from stdin");

    let user_input_1_as_str: &str = str::from_utf8(&buffer1[..number_of_bytes1]).unwrap();

    let user_input1: u32 = match user_input_1_as_str.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            panic!("Failed to convert &str to u32 in user_input1");
        }
    };

    let number_of_allowed_bytes_as_u32 = user_input1 + 4;
    let number_of_allowed_bytes: usize = number_of_allowed_bytes_as_u32 as usize;

    // We'll write to this vector
    let mut number_list_from_user_input_str: Vec<&str> = Vec::new();

    // Get user input
    let mut buffer2 = String::new();
    std::io::stdin()
        .read_line(&mut buffer2)
        .expect("Failed to read the user input");

    // Split at white space
    number_list_from_user_input_str.extend(buffer2.split_whitespace());
    let mut number_list_from_user_input: Vec<u32> = Vec::with_capacity(number_of_allowed_bytes);

    for number in number_list_from_user_input_str {
        number_list_from_user_input.push(number.parse::<u32>().unwrap());
    }

    let good_number: u32 = count_good_numbers(number_list_from_user_input);

    // printing the hard way
    let mut handle = std::io::BufWriter::new(std::io::stdout().lock());
    write!(handle, "{good_number}\n").unwrap()
}
