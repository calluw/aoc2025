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

fn apply_rotation(start: i32, delta: i32) -> i32 {
    (start + delta).rem_euclid(100)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, instrs) = parse_instrs(input).expect("Input to parse");
    let mut pos = 50;
    let mut zeroes: u64 = 0;
    for instr in instrs {
        pos = apply_rotation(pos, instr);
        if pos == 0 {
            zeroes += 1;
        }
    }
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, instrs) = parse_instrs(input).expect("Input to parse");
    let mut pos = 50;
    let mut zeroes_hit: u64 = 0;
    for instr in instrs {
        // 24 - 24 = 1
        // 24 - 125 = 2
        // 24 + 76 = 1
        // -24 + 25 = 1

        // Take remainder of delta as the effective non-full spin delta, if
        // delta is negative then check that final pos is greater, if delta is
        // positive then check that final pos is less, if so it went past the
        // modulo
        // To account for full spins take divisor of abs delta with 100
        // To account for edge case, add zero if final pos is zero
        let prev_pos = pos;
        pos = apply_rotation(pos, instr);
        println!("Rotate {} from {} to {}", instr, prev_pos, pos);
        if ((prev_pos - pos) * instr.signum() > 0) {
            println!("Hit zero during that");
            zeroes_hit += 1;
        }
        if pos == 0 {
            println!("Also landed on zero");
            zeroes_hit += 1;
        }
        println!("And also got {} full spins", instr.abs().div_euclid(100));
        zeroes_hit += instr.abs().div_euclid(100) as u64;
        println!("Zeroes: {}", zeroes_hit);
    }
    Some(zeroes_hit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
