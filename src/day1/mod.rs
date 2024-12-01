use aoc_runner_derive::aoc;
use regex::Regex;
use std::fmt;
use std::fmt::Display;

pub struct DayResult {
    value: i32,
}

impl Display for DayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


pub fn get_input_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let re = Regex::new(r"(\d+)\s*(\d+)").unwrap();

    // Collect and parse input lists
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.split('\n') {
        if let Some(captures) = re.captures((&line).as_ref()) {
            let left: i32 = captures[1].parse().unwrap();
            let right: i32 = captures[2].parse().unwrap();

            left_list.push(left);
            right_list.push(right);
        }
    }

    // Sort lists
    left_list.sort_unstable();
    right_list.sort_unstable();

    (left_list, right_list)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (left_list, right_list) = get_input_lists(input);
    let part1_output = left_list.iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();
    part1_output
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (left_list, right_list) = get_input_lists(input);
    let part2_output = left_list.iter()
        .map(|&i| {
            i * right_list.iter()
                .filter(|&&j| j == i)
                .count() as i32
        })
        .sum();
    part2_output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part1() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let result = part1(input);
        assert_eq!(format!("{}", result), "11");
    }

    #[test]
    fn test_day01_part2() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let result = part2(input);
        assert_eq!(format!("{}", result), "31");
    }
}