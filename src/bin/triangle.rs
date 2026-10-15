// Your task is to print a right-angled triangle pattern using asterisks (*).
// *****
// ****
// ***
// **
// *

// First line prints five stars
// Second prints four

fn main() {
    let mut counter: u8 = 7;

    for x in 0..6 {
        counter = counter - 1;
        if x == 0 {
            print!("");
        } else {
            print!("\n");
        }

        for y in 1..counter {
            // run for `counter-1` number of times

            print!("*");
        }
    }
}

//0 - 0->6
//1 - 0->5
//2 - 0->4
//3 - 0->3
//4 - 0->2
//5 - 0->1
