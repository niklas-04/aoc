use itertools::Itertools;
use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input_to_parse: &str) -> Input {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let amount_of_numbers = input_to_parse.split_whitespace().count();
    if amount_of_numbers % 2 != 0 {
        panic!("The given input are different lengths!");
    }

    // Each element from one list is matched with one from the other, therefore tuples is used
    let input_iter = input_to_parse
        .split_whitespace()
        .tuples::<(&str, &str)>()
        .map(|(a, b)| {
            let left_parsed = a.parse::<u32>().expect("Invalid input");
            let right_parsed = b.parse::<u32>().expect("Invalid input");
            (left_parsed, right_parsed)
        });

    for (left_chunk, right_chunk) in input_iter {
        left.push(left_chunk);
        right.push(right_chunk);
    }
    (left, right)
}

pub fn part_01(input: &Input) -> u32 {
    let mut left = input.0.clone();
    left.sort_unstable();

    let mut right = input.1.clone();
    right.sort_unstable();

    if left.len() != right.len() {
        panic!("mismatched list lengths");
    }

    let mut sum: u32 = 0;
    for (l, r) in left.iter().zip(right) {
        sum += l.abs_diff(r);
    }
    sum
}

pub fn part_02(input: &Input) -> u32 {
    let mut occurences: HashMap<u32, u32> = HashMap::new();

    let right = input.1.clone();
    for number in right {
        let mut amount_of_occurences = 1;

        if occurences.contains_key(&number) {
            amount_of_occurences = occurences.get(&number).unwrap().to_owned() + 1;
        }

        occurences.insert(number, amount_of_occurences);
    }
    println!("{:?}", occurences);

    let left = input.0.clone();
    left.iter()
        .filter_map(|num| occurences.get(num).map(|freq| num * freq))
        .sum()
}
