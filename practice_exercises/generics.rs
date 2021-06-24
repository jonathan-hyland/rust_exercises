// A reverse implementation of the exercises throughout Chapter 10 of The Rust Programming Language. Instead of returning the largest T
// it returns the smallest.

fn smallest<T: PartialOrd + Copy>(s: &[T]) -> T {
    let mut smallest = s[0];

    for &item in s {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

fn main() {
    let int_vec = vec![32, 45, 67, 89, 13, 7, 21];

    let smallest_int = smallest(&int_vec);
    println!("The smallest number is: {}", smallest_int);

    let char_vec = vec!['c', 'd', 'a', 'z', 'v', 'y'];

    let smallest_char = smallest(&char_vec);
    println!("The smallest character is: {}", smallest_char);
}