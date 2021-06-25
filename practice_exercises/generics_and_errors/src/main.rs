// Generics and error checking exercise. Takes a generic input (which must implement Display and ToString), creates a file,
// and writes the input to the file. Random number is appended to the file name to ensure each input is written to its own file.

use std::io;
use std::fs::File;
use std::fmt::Display;
use std::io::Write;
use rand::Rng;

fn generic_input<T: Display + ToString>(input: T) -> Result<File, io::Error>  {
    let input = input.to_string(); // convert the input into a string
    let new_input = input.as_bytes(); // convert the string to bytes to be written
    let file_num = rand::thread_rng().gen_range(1..1001); // generate a random number to ensure previous files won't be over-written
    let file_name = "input".to_owned() + &file_num.to_string() + ".txt"; // variable to create file name

    let mut file = File::create(file_name)?;
    file.write(new_input)?;
    
    Ok(file)
}

fn main() {
    generic_input("It's a String!").expect("Not a valid input.");
    generic_input(32).expect("Not a valid input.");
    generic_input(55.01342132).expect("Not a valid input.");
}