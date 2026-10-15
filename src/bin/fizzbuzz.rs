use std::io::{self, Read, Write};
use std::str;

fn fizzbuzz(n: u32) {
    // Aquiring the lock once before the loop starts
    let mut handle = io::BufWriter::new(io::stdout().lock());

    (1..=n).for_each(|num| match (num % 3, num % 5) {
        (0, 0) => {
            writeln!(handle, "FizzBuzz").unwrap();
        }
        (0, _) => {
            writeln!(handle, "Fizz").unwrap();
        }
        (_, 0) => {
            writeln!(handle, "Buzz").unwrap();
        }
        (_, _) => {
            writeln!(handle, "{num}").unwrap();
        }
    })
}

fn main() {
    let mut buffer = [0u8; 16];
    let number_of_bytes = io::stdin().read(&mut buffer).expect("Failed to read bytes");
    let input_str: &str = str::from_utf8(&buffer[..number_of_bytes]).unwrap();

    let input_num: u32 = match input_str.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            panic!("Failed to convert str into u32");
        }
    };
    fizzbuzz(input_num);
}
