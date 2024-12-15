use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::equal;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u128> {
        let mut answer: u128 = 0;

        let equations = read_input(reader);

        for equation in equations {
            let (expected, operands) = equation;
            let accumulator: u128 = 0;
            let total = check_equation(expected, &operands, 0, accumulator, 0, false);
            if total >= 1 {
                answer += expected;
            }
        }
        //let answer = reader.lines().flatten().count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    //assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u128> {
        let mut answer: u128 = 0;

        let equations = read_input(reader);

        for equation in equations {
            let (expected, operands) = equation;
            let accumulator: u128 = 0;
            let total = check_equation(expected, &operands, 0, accumulator, 0, true);
            if total >= 1 {
                answer += expected;
            }
        }
        //let answer = reader.lines().flatten().count();
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result: u128 = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn check_equation(
    expected: u128,
    operands: &Vec<u128>,
    index: usize,
    accumulator: u128,
    total_valid: u128,
    part_2: bool,
) -> u128 {

    let mut new_total_valid = total_valid;
    if index == operands.len() {
        if expected == accumulator {
            new_total_valid += 1;
        }
        return new_total_valid;
    }

    if accumulator > expected {
        return new_total_valid;
    }

    let operand: u128 = operands[index];
    if index == 0 {
        return check_equation(
            expected,
            operands,
            index + 1,
            operand,
            new_total_valid,
            part_2,
        );
    }

    if part_2 {
        new_total_valid = check_equation(
            expected,
            &operands,
            index + 1,
            concatenate(accumulator, operand),
            new_total_valid,
            part_2,
        );
    }
    new_total_valid = check_equation(
        expected,
        &operands,
        index + 1,
        accumulator + operand,
        new_total_valid,
        part_2,
    );
    check_equation(
        expected,
        &operands,
        index + 1,
        accumulator * operand,
        new_total_valid,
        part_2,
    )
}

fn concatenate(n1: u128, n2: u128) -> u128 {
    (n1.to_string() + &n2.to_string()).parse::<u128>().unwrap()
}

fn read_input<R: BufRead>(reader: R) -> Vec<(u128, Vec<u128>)> {
    let mut equations: Vec<(u128, Vec<u128>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(": ").collect();
        let result: u128 = line[0].parse::<u128>().unwrap();
        let operands: Vec<u128> = line[1]
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect();
        equations.push((result, operands));
    }

    equations
}
