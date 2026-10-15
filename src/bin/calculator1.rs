use std::io::{self, Read};
use std::str;

struct Calc {
    add: i32,
    subs: i32,
    mult: i32,
    div: i32,
    rem: i32,
}

fn calculator(m: i32, n: i32) -> Calc {
    let sum: i32 = m + n;
    let diff: i32 = m - n;
    let prod: i32 = m * n;
    let division: i32 = m / n;
    let modulo: i32 = m % n;

    Calc {
        add: sum,
        subs: diff,
        mult: prod,
        div: division,
        rem: modulo,
    }
}

fn main() {
    let mut user_input = [0u8, 8];

    let bytes_read = io::stdin()
        .read(&mut user_input)
        .expect("Failed to read from stdin");

    let input_text = str::from_utf8(&user_input[..bytes_read]).expect("Input was not valid");

    let number: u32 = match input_text.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            panic!("1");
        }
    };

    let result: Calc = calculator()
}
