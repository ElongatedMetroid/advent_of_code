use std::{process, fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let f = File::open("input").unwrap_or_else(|error| {
        println!("Could not open file: {error}");
        process::exit(1);
    });
    let reader = BufReader::new(f);
    let mut binary = String::new();

    // Hashmap, entrys will be column number, add one to that entrys data each time a new
    // Key = Column Num, Value = Amount of zeros
    let mut zeros: HashMap<usize, usize> = HashMap::new();
    // Key = Column Num, Value = Amount of ones
    let mut ones: HashMap<usize, usize> = HashMap::new();

    // Foreach line...
    for line in reader.lines() {
        let line = line.unwrap();

        for (key, char) in line.chars().enumerate() {
            match char {
                '0' => { zeros.entry(key).and_modify(|amount| *amount += 1).or_insert(0); },
                '1' => { ones.entry(key).and_modify(|amount| *amount += 1).or_insert(0); },
                _ => (),
            }

        }
    }

    // Create binary number
    for key in 0..12 {
        if zeros[&key] > ones[&key] {
            binary.push('0');
        } else {
            binary.push('1');
        }
    }

    let gamma_rate = u16::from_str_radix(format!("0000{binary}").as_str(), 2).unwrap();
    let epsilon_rate = !u16::from_str_radix(format!("1111{binary}").as_str(), 2).unwrap();
    println!(
        "binary: {binary}\ngamma rate: {gamma_rate}\nepsilon rate: {epsilon_rate}\nTotal power consumption: {}",
        gamma_rate as u32 * epsilon_rate as u32,
    );

}
