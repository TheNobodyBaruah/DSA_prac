use std::io::{self, Read, Write};
use std::str;

fn finding_number_of_digits(n: u32) -> u32 {
    // TODO
    n.ilog10() + 1
}

fn count_zeros(n: u32) -> u32 {
    // TODO
}

fn main() {
    // Input can be 18 digits long
    let mut buffer = [0u8; 23];
    let number_of_bytes = io::stdin().read(&mut buffer).expect("Failed to read bytes");

    let input_as_str: &str =
        str::from_utf8(&buffer[..number_of_bytes]).expect("Failed to convert to &str");

    let input_as_u32: u32 = match input_as_str.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            panic!("Coudn't convert to u32");
        }
    };

    let number_of_zeros: u32 = count_zeros(input_as_u32);

    // print number of zeros
    let mut handle = io::BufWriter::new(io::stdout().lock());
    writeln!(handle, "{number_of_zeros}").unwrap();
}
