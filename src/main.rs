mod day1;

use std::fs::File;
use std::io::prelude::*;
use day1::resulting_frequency;
use day1::first_frequency;

fn main() {
    let filename = "src/inputs/1.input.txt";
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let numbers: Vec<&str> = contents.split('\n').collect();

    println!("Answer: {}", resulting_frequency(numbers));
    // println!("Answer: {}", first_frequency(numbers));

}