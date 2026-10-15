use std::io::{Read, Write};
fn is_vowel(letter: &str) -> bool {
    let vowel: [&str; 5] = ["a", "e", "i", "o", "u"];
    if vowel.contains(&letter) { true } else { false }
}

fn main() {
    // get user input
    let mut buffer = [0u8; 16];
    // mutate buffer
    let bytes = std::io::stdin()
        .read(&mut buffer)
        .expect("Failed to read user input");
    let input_text: &str = str::from_utf8(&buffer[..bytes]).expect("Failed to read &str");

    let boolean: bool = is_vowel(input_text.trim());
    match boolean {
        true => {
            let mut handle = std::io::BufWriter::new(std::io::stdout().lock());
            write!(handle, "YES\n").unwrap();
            return;
        }
        false => {
            let mut handle = std::io::BufWriter::new(std::io::stdout().lock());
            write!(handle, "NO\n").unwrap();
            return;
        }
    }
}
