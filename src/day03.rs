use core::{num, panic};
use std::borrow::Borrow;

use regex::{Captures, Regex};

type Input = Vec<[u32; 2]>; // Collection of two numbers to be multiplied. (Maybe redundant?)

pub fn parse(str: &str) -> Input {
    let mut res: Input = Vec::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Cannot create regex");
    let caps: Vec<Captures> = re.captures_iter(str).collect();
    for cap in caps {
        let num1 = cap
            .get(1)
            .expect("missing first number")
            .as_str()
            .parse::<u32>()
            .expect("failed to parse first number");

        let num2 = cap
            .get(2)
            .expect("missing first number")
            .as_str()
            .parse::<u32>()
            .expect("failed to parse second number");

        res.push([num1, num2]);
    }
    res
}

pub fn part_01(input: &Input) -> u32 {
    let mut sum = 0;
    input.iter().for_each(|nums| sum += nums[0] * nums[1]);
    sum
}
