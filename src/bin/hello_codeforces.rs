use std::io::{Read, Write};
fn hello_codeforces(n: u32) {
    for i in 1..=n {
        let mut handle = std::io::BufWriter::new(std::io::stdout().lock());
        write!(handle, "Hello Codeforces {i}\n").unwrap();
    }
}

fn main() {
    // Get user input the efficient way
    let mut buffer = [0u8; 16];

    let bytes = std::io::stdin()
        .read(&mut buffer)
        .expect("Failed to get input");

    let input_text = str::from_utf8(&buffer[..bytes]).expect("Input was not valid text");

    let n: u32 = match input_text.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("That was not a valid u32");
            return;
        }
    };

    hello_codeforces(n);
}
