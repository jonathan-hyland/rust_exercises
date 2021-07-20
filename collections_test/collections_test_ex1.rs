// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let list_of_integers = vec![10, 20, 30, 30, 40, 50, 60, 70 ,80, 90, 100];
    integer_avg(&list_of_integers);
    integer_median(&list_of_integers);
    integer_mode(&list_of_integers);
}

fn integer_avg(v: &Vec<u32>) -> () {
    let mut sum = 0;
    let count = v.len();
    let u_count = u32::try_from(count);
    
    for i in v {
        sum += i;
        // println!("{}", sum); // Check to ensure sum is working.
    }

    let avg = sum / u_count.unwrap();

    println!("{}", avg)
}

fn integer_median(v: &Vec<u32>) -> () {
    let count = v.len();
    let median = count / 2;
    println!("{}", v[median])
}

fn integer_mode(v: &Vec<u32>) -> () {
    let mut int_hash = HashMap::new();

    for i in v {
        let count = int_hash.entry(i).or_insert(0);
        *count += 1;
    }

    for (key, val) in int_hash.iter() {
        if val > &1 {
            println!("Median: {}; repeats {} times", key, val);
        } else if val <= &1 {
            
        } else {
            println!("NO MEDIAN!");
        }

    }
}