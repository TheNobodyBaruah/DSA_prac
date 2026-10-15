use std::io::{self, Read, Write};
use std::str;

fn leap_year_or_not(year: u32) -> bool {
    // Check if divisible by 100, true then also divisible by 400
    // Not divisble by 100, then divisble by 4
    let div_by_100: u32 = year % 100;
    match div_by_100 {
        0 => {
            let div_by_400: u32 = year % 400;
            match div_by_400 {
                0 => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        _ => {
            let div_by_4_if_not_by_100: u32 = year % 4;
            match div_by_4_if_not_by_100 {
                0 => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
    }
}

fn main() {
    // Get user input
    let mut buffer = [0u8; 16];

    let number_of_bytes = std::io::stdin()
        .read(&mut buffer)
        .expect("Failed to read bytes");

    let input_text: &str =
        str::from_utf8(&buffer[..number_of_bytes]).expect("Failed to convert bytes to &str");

    let year: u32 = match input_text.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            panic!("Failed to convert &str to u32");
        }
    };

    let is_leap_year: bool = leap_year_or_not(year);
    match is_leap_year {
        true => {
            // print "YES"
            let mut handle = io::BufWriter::new(io::stdout().lock());

            write!(handle, "Yes\n").unwrap();
        }
        false => {
            // print "FALSE"
            let mut handle = io::BufWriter::new(io::stdout().lock());

            write!(handle, "No\n").unwrap();
        }
    }
}
