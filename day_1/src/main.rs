#![feature(array_windows)]
use std::fs;

fn main() {
    // Read input into a string
    let input: Vec<isize> = fs::read_to_string("input")
        .expect("Could not read file")
        // Split the whitespace
        .split_whitespace()
        // Parse each thing seperated by white space into an isize
        .map(|num| num.trim().parse::<isize>().unwrap())
        // Collect each isize into a Vec<isize>
        .collect();

    // ----- Part One -----
    let mut larger_than_prev = 0;
    // Zip up the regular iterator of input to the iterator + 1 (the next number)
    for (last_depth, current_depth) in input.iter().zip(input.iter().skip(1)) {
        // If the current depth is greater than the last_depth ...
        if last_depth < current_depth {
            // Add one to our counter
            larger_than_prev += 1;
        }
    }

    println!("Amount of measurments larger than the previous: {larger_than_prev}");

    // ----- Part Two ------
    let mut larger_than_prev = 0;
    for [a1, a2_b1, a3_b2, b3] in input.array_windows() {
        let prev = a1 + a2_b1 + a3_b2;
        let curr = a2_b1 + a3_b2 + b3;

        if prev < curr {
            larger_than_prev += 1;
        }
    }
    println!("Amount of sums larger the previous: {larger_than_prev}");
}