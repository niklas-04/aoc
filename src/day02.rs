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

pub fn is_safe(report: &[u32]) -> bool {
    let mut inc = true;
    let mut dec = true;

    for win in report.windows(2) {
        if win[0].abs_diff(win[1]) > 3 {
            return false;
        }

        if win[0] > win[1] {
            inc = false;
        } else if win[0] < win[1] {
            dec = false
        } else {
            return false;
        }
    }

    if !inc && !dec {
        return false;
    }
    true
}

pub fn part_01(input: &Input) -> u32 {
    input.iter().filter(|report| is_safe(report)).count() as u32
}

pub fn part_02(input: &Input) -> u32 {
    input
        .iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|i| {
                    let (before, after) = report.split_at(i);
                    is_safe(&[before, &after[1..]].concat())
                })
        })
        .count() as u32
}
