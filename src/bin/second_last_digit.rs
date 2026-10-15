use std::io::{Read, Write};

fn finding_total_digits(number: u32) -> u32 {
    if number == 0 {
        return 1;
    }
    number.ilog10() + 1
}

fn finding_index_of_last_digit(number: u32) -> u32 {
    if number == 0 {
        return 1;
    }
    number.ilog10()
}

fn find_second_last_digit(n: u32, index: u32) -> u32 {
    let index_of_last_digit: u32 = finding_index_of_last_digit(n);
    // find the second last digit in a number
    let last_digit: u32 = (n / 10_u32.pow(index_of_last_digit - index)) % 10;
    last_digit
}

fn main() {
    // Get user input
    let mut buffer = [0u8; 16];
    let number_of_bytes = std::io::stdin().read(&mut buffer).unwrap();
    let input_text: &str = str::from_utf8(&buffer[..number_of_bytes]).unwrap();
    let input_number: u32 = match input_text.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            panic!("Input cannot be converted to u32");
        }
    };

    let index: u32 = finding_index_of_last_digit(input_number) - 1; // Since, we want second last
    // index

    let digit: u32 = find_second_last_digit(input_number, index);

    let mut handle = std::io::BufWriter::new(std::io::stdout().lock());
    write!(handle, "{digit}\n").unwrap();
}
