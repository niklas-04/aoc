use itertools::Itertools;

type Input = Vec<Vec<u32>>;

pub fn parse(str: &str) -> Input {
    str.lines() // Splits by \n
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect() // knows the vector type because of Input being returned
        })
        .collect()
}

pub fn part_01_helper(num: i32) -> i32 {
    if num > 0 {
        1
    } else {
        -1
    }
}

pub fn part01_solver(input: &Input, dampener: u32) -> u32 {
    let mut safe_reports = 0;

    for vec in input {
        let mut diff_amount = 0;
        let mut bad_levels = 0;
        let mut len_to_check = vec.len() as i32 - 1; // -1 because first one has no effect

        for (&current, &next) in vec.iter().tuple_windows() {
            let diff = (current as i32) - (next as i32);

            if diff == 0 || diff.abs() > 3 {
                // We "remove" one unsafe element
                break;
            } else {
                diff_amount += part_01_helper(diff);
            }
        }

        if diff_amount.abs() == len_to_check {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn part_01(input: &Input) -> u32 {
    part01_solver(input, 0)
}

pub fn part_02(input: &Input) -> u32 {
    part01_solver(input, 100)
}
