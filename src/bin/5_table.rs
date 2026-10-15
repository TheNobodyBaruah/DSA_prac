fn fives(n: u32) -> u32 {
    n * 5
}

fn main() {
    for i in 1..=10 {
        println!("5 * {} = {}", i, fives(i));
    }
}
