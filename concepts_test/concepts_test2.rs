// Generate the nth Fibonacci number.

use std::io;
use std::str::FromStr;

fn main() {
    println!("Enter which Fibionacci number you want.");

    let mut fib_num = String::new();

    io::stdin()
        .read_line(&mut fib_num)
        .expect("Entry failed, try again.");
    
    let n = i32::from_str(&fib_num.trim()).unwrap();
    println!("{}", fib_sequence(n));
}

fn fib_sequence(x: i32) -> i32 {

    let mut x_count = x;
    let mut x1 = 0;
    let mut x2 = 1;


    let sequence = loop {
        let sum = x1 + x2;
            if x_count == 1 {
                break x1;
            }
        x1 = x2; 
        x2 = sum;
        x_count -= 1;
            if x_count == 0 {
                break sum;
            }
        };
    sequence
    }