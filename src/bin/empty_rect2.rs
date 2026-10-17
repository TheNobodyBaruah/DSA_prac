use std::io::{self, Read, Write};
use std::str;

fn draw_empty_rect(rows: u32, columns: u32) {
    // print rectangle
}

fn main() {
    let mut buffer = [0u8; 16];
    let num_of_bytes: usize = io::stdin().read(&mut buffer).expect("Failed to read bytes");
    let input_str: &str = str::from_utf8(&buffer[..num_of_bytes]).unwrap();
}
