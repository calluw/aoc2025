use nom::{
    IResult,
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map_res, value},
    multi::separated_list0,
};
use std::str::FromStr;

advent_of_code::solution!(1);

fn parse_sign(input: &str) -> IResult<&str, i32> {
    alt((value(-1, char('L')), value(1, char('R'))))(input)
}

fn parse_instr(input: &str) -> IResult<&str, i32> {
    let (input, sign) = parse_sign(input)?;
    let (input, amount) = map_res(digit1, i32::from_str)(input)?;
    Ok((input, sign * amount))
}

fn parse_instrs(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(char('\n'), parse_instr)(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, instrs) = parse_instrs(input).expect("Input to parse");
    println!("{:?}", instrs);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
