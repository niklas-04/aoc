use std::fs::{exists, read_to_string};
use std::{env, error::Error};

//use day01::{parse, part_01, part_02};
//mod day01;

// use day02::{parse, part_01, part_02};
// mod day02;

use day03::{parse, part_01};
mod day03;

// TODO: Structure functions from different days cleaner
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    const DAY: &str = "day03";

    let path = format!("../inputs/{}.txt", DAY); // relative path from where 'cargo run'
    let input_string = &get_input_file(&path).unwrap();
    let input = parse(&input_string);

    let solution_01 = part_01(&input);
    //let solution_02 = part_02(&input);
    //
    println!("{}", solution_01);
    //println!("{}", solution_02);
}

// Uses file path relative to where main.rs is located
pub fn get_input_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = read_to_string(file_path)?;
    Ok(input)
}
