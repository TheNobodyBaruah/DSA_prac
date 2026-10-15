// Your task is to print the first five letters of the English alphabet.
// There is no input for this problem.
// Print the first five uppercase English letters, one per line.

fn main() {
    for byte in b'A'..b'F' {
        println!("{}", byte as char);
    }
}
