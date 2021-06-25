// Convert temperatures between Fahrenheit and Celsius.
use std::io;
use std::str::FromStr;

fn main() {
    // Initialize input variables. One for temp type (Celsius or Fahrenheit) one for the actual temperature value.
    let mut temp_type = String::new();
    let mut temp = String::new();

    println!("Enter the temperature.");

    io::stdin()
        .read_line(&mut temp)
        .expect("Entry failed, try again.");

    // Convert the String into a Float.
    let temp_f = f64::from_str(&temp.trim()).unwrap();
    
    let mut cmd_loop = true;

    // Loop to ensure user enters the specified input.

    while cmd_loop == true {
        println!("Type 'c' for Celsius, 'f' for Fahrenheit.");

        io::stdin()
            .read_line(&mut temp_type)
            .expect("Entry failed, try again.");

        if temp_type.trim() == "c" {
            cmd_loop = false;
            println!("{}", c_to_f(temp_f))
        } else if temp_type.trim() == "f" {
            cmd_loop = false;
            println!("{}", f_to_c(temp_f))
        } else {
            cmd_loop = true;
            println!("Please enter a valid temperature.");
            temp_type = String::new();
        }
    }
}

fn c_to_f(x: f64) -> f64 {
    x * 1.80 + 32.0
}

fn f_to_c(x: f64) -> f64 {
    x - 32.0 / 1.80
}