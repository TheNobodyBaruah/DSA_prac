use std::io::{self, Read, Write};
use std::str;

/* fn get_first_digit(mut n: u64) -> u64 {
    while n >= 10 {
        n /= 10;
    }
    n
} */

fn count_zeros(m: u64, zero_counter: u64) -> u64 {
    // TODO
    let mut zero_counter: u64 = zero_counter;
    let mut n = m;

    for _i in 1..=(n.ilog10() + 1) {
        if n % 10 == 0 {
            // increment zero_counter
            zero_counter = zero_counter + 1;
            n = n / 10;
        } else {
            n = n / 10;
        }
    }

    zero_counter
}

fn main() {
    // Input can be 18 digits long
    let mut buffer = [0u8; 23];
    let number_of_bytes = io::stdin().read(&mut buffer).expect("Failed to read bytes");

    let mut input_as_str: &str =
        str::from_utf8(&buffer[..number_of_bytes]).expect("Failed to convert to &str");

    let mut zero_counter = 0;

    // NOTE: Learnt new ways to strip prefix from &str and do something
    while let Some(stripped) = input_as_str.trim().strip_prefix('0') {
        input_as_str = stripped;
        zero_counter += 1;
    }

    if input_as_str.is_empty() {
        let mut handle = io::BufWriter::new(io::stdout().lock());
        writeln!(handle, "{zero_counter}").unwrap();
    } else {
        let input_as_u64: u64 = match input_as_str.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                panic!("Coudn't convert to u64");
            }
        };

        let number_of_zeros: u64 = count_zeros(input_as_u64, zero_counter);

        // print number of zeros
        let mut handle = io::BufWriter::new(io::stdout().lock());
        writeln!(handle, "{number_of_zeros}").unwrap();
    }
}
