use std::{fs, cmp::Ordering};

fn main() {
    let input: Vec<isize> = fs::read_to_string("input")
        .expect("Could not read file")
        .split_whitespace()
        .map(|num| num.trim().parse::<isize>().unwrap() )
        .collect();

    let mut larger_than_prev = 0;
    for (last_depth, current_depth) in input.iter().zip(input.iter().skip(1)) {
        if current_depth.cmp(&last_depth) == Ordering::Greater {
            larger_than_prev += 1;
        }
    }

    println!("Amount of measurments larger than the previous: {larger_than_prev}");
}
