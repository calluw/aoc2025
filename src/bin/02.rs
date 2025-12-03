use itertools::Itertools;
use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

advent_of_code::solution!(2);

fn parse_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(
        char(','),
        separated_pair(
            map_res(digit1, u64::from_str),
            char('-'),
            map_res(digit1, u64::from_str),
        ),
    )(input)
}

fn id_is_invalid(id: u64) -> bool {
    let id_str = id.to_string();
    let first_num = &id_str[..(id_str.len() / 2)];
    let second_num = &id_str[(id_str.len() / 2)..];
    first_num == second_num
}

fn invalid_ids_in_range(range: (u64, u64)) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for id in range.0..=range.1 {
        if id_is_invalid(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

fn id_is_invalid_p2(id: u64) -> bool {
    let id_str = id.to_string();
    let max_divisor = id_str.len();
    for divisor in 2..=max_divisor {
        let sub_strs: Vec<String> = id_str
            .chars()
            .chunks(id_str.len() / divisor)
            .into_iter()
            .map(|s| s.collect())
            .collect();
        if sub_strs.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    }
    false
}

fn invalid_ids_in_range_p2(range: (u64, u64)) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for id in range.0..=range.1 {
        if id_is_invalid_p2(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, ranges) = parse_ranges(input).expect("Input to parse");
    let mut invalid_ids = Vec::new();
    for range in ranges {
        invalid_ids.append(&mut invalid_ids_in_range(range));
    }
    Some(invalid_ids.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, ranges) = parse_ranges(input).expect("Input to parse");
    let mut invalid_ids = Vec::new();
    for range in ranges {
        invalid_ids.append(&mut invalid_ids_in_range_p2(range));
    }
    Some(invalid_ids.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
