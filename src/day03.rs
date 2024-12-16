use regex::{Captures, Regex};

type Input = Vec<[u32; 3]>; // Collection of two numbers to be multiplied. (Maybe redundant?)

pub fn parse(str: &str) -> Input {
    let mut res: Input = Vec::new();

    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Cannot create regex");
    let caps: Vec<Captures> = re_mul.captures_iter(str).collect();

    let mut do_mul = 1; // 1 = on, 0 = off
    for cap in caps {
        let conditional_match = cap.get(0).expect("Cannot get regex");
        match conditional_match.as_str() {
            "do()" => {
                do_mul = 1;
            }
            "don't()" => {
                do_mul = 0;
            }
            _ => {
                // Default case
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

                res.push([do_mul, num1, num2]);
            }
        }
    }
    res
}

pub fn part_01(input: &Input) -> u32 {
    input.iter().fold(0, |sum, nums| sum + nums[1] * nums[2])
}

pub fn part_02(input: &Input) -> u32 {
    input.iter().fold(0, |sum, nums| {
        if nums[0] == 1 {
            sum + nums[1] * nums[2]
        } else {
            sum
        }
    })
}
