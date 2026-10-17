use std::io::{self, BufWriter, Read, Write};
use std::str;

// 4 5
// ^^^^^
// ^   ^
// ^   ^
// ^^^^^
fn print_lines_not_top_bottom(col: i32, handle: &mut impl Write) {
    write!(handle, "^").unwrap();

    if col > 2 {
        for _space in 1..=(col - 2) {
            write!(handle, " ").unwrap();
        }
        write!(handle, "^").unwrap();
    }
}

fn draw_rectangle(rows: u32, columns: u32) {
    // For the first and the nth line number of ^ is equal to number of columns
    let columns_i: i32 = columns.try_into().unwrap();
    let mut handle = BufWriter::new(io::stdout().lock());
    for i in 1..=rows {
        match i {
            i if (i == 1 || i == rows) => {
                // single line without any spaces
                for _j in 1..=columns {
                    write!(handle, "^").unwrap();
                }
            }

            _ => {
                // write!(handle, "Working on program").unwrap();
                print_lines_not_top_bottom(columns_i, &mut handle);
            }
        }
        writeln!(handle, "").unwrap();
    }
}

fn main() {
    // Get user input as bytes.
    let mut buffer = [0u8; 16];
    let num_of_bytes: usize = io::stdin()
        .read(&mut buffer)
        .expect("Failed to read bytes from stdin");

    let input_str =
        str::from_utf8(&buffer[..num_of_bytes]).expect("Failed to convert bytes to &str");

    // Let's separate those inputs
    // Let's build an interator which spilts whitespaces and parses u32
    let mut input_iter_u32 = input_str
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok());

    let rows = input_iter_u32.next().unwrap();
    let columns = input_iter_u32.next().unwrap();

    draw_rectangle(rows, columns);
}
