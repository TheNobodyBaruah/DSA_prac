// Rectangle

/* You are given the length and breadth of a rectangle. Your task is to calculate its area and perimeter.

The formulas are:

Area =
 length ×
 breadth
Perimeter =
 2×(length+breadth) */

struct Measurements {
    area: u32,
    perimeter: u32,
}

impl Measurements {
    fn measure_rect(length: u32, breadth: u32) -> Measurements {
        Measurements {
            area: length * breadth,
            perimeter: 2 * (length + breadth),
        }
    }
}

fn main() {
    // Get user input
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");

    let user_input: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // println!("{:?}", user_input);

    let rect: Measurements = Measurements::measure_rect(user_input[0], user_input[1]);

    println!("Area = {}\nPerimeter = {}", rect.area, rect.perimeter);
}
