// your task is to print the letter z using asterisks (*) as shown below

// *****   first line n stars
//    *    second line 1 star -> star at n-1 th postn
//   *     thrid line 1 star  -> star at n-2 th postn
//  *      fourth line 1 star
// *****   nth line n star

/* fn print_z(n: u8) {
    for i in 0..n + 1 {
        if i != 0 && i != n {
            print!("\n");
        } else {
            (0..(n + 1)).for_each(|_| {
                print!("*");
            });
        }

        for _ in 0..(n - i - 1) {
            if i != 0 || i != n {
                print!(" ");
            } else {
                print!("*");
            }
        }

        if i != 0 || i != n {
            print!("*");
        }
    }
} */

fn print_z(n: u8) {
    for row in 0..n {
        if row == 0 || row == n - 1 {
            for _ in 0..n {
                print!("*");
            }
        } else {
            for _ in 0..(n - row - 1) {
                print!(" ");
            }
            print!("*");
        }

        print!("\n");
    }
}

fn main() {
    print_z(5);
}
