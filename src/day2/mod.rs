use std::num::ParseIntError;
use aoc_runner_derive::aoc;

pub fn get_input_lists(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()
}

fn levels_checker(numbers: &[i32]) -> bool {

    for window in numbers.windows(3) {
        let [a, b, c] = [window[0], window[1], window[2]];

        let diff1 = (b - a).abs();
        let diff2 = (c - b).abs();

        if diff1 < 1 || diff1 > 3 || diff2 < 1 || diff2 > 3 {
            return false;
        }

        if (b - a).signum() != (c - b).signum() {
            return false;
        }
    }

    true
}

fn part2_levels_checker(numbers: &[i32]) -> bool {
    if numbers.len() < 3 {
        return false;
    }
    if levels_checker(&numbers) {
        return true;
    }
    for skip_index in 0..numbers.len() {
        let test_sequence: Vec<i32> = numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_index)
            .map(|(_, &x)| x)
            .collect();

        if levels_checker(&test_sequence) {
            return true;
        }
    }

    false
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<i32, Box<ParseIntError>> {
    let mut result_counter = 0;
    let all_numbers = get_input_lists(input)?;
    for numbers in all_numbers {
        if levels_checker(&numbers) {
            result_counter += 1;
        }
    }
    Ok(result_counter)
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> Result<i32, Box<ParseIntError>> {
    let mut result_counter = 0;
    let all_numbers = get_input_lists(input)?;
    for numbers in all_numbers {
        if part2_levels_checker(&numbers) {
            result_counter += 1;
        }
    }
    Ok(result_counter)
}

