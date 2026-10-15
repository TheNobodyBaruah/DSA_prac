use std::io;

struct Measurements {
    area: u32,
    perimeter: u32,
}

fn measure_rect(length: u32, breadth: u32) -> Measurements {
    Measurements {
        area: length * breadth,
        perimeter: 2 * (length + breadth),
    }
}

fn main() {
    // Read a line from standard input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Split the line by whitespace and parse the two integers
    let mut parts = input.split_whitespace();
    let length: u32 = parts.next().unwrap().parse::<u32>().unwrap();
    let breadth: u32 = parts.next().unwrap().parse::<u32>().unwrap();

    let rect = measure_rect(length, breadth);

    // Print in the exact format expected by the judge
    println!("Area = {}", rect.area);
    println!("Perimeter = {}", rect.perimeter);
}
