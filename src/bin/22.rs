use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "22"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
1
10
100
2024
";

const TEST2: &str = "\
1
2
3
2024
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let secret_numbers = read_input(reader);
        for secret_number in secret_numbers {
            let next_secret_number = calc_secret_number(secret_number, 2000);
            answer += next_secret_number;
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(37327623, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut changes = HashSet::<[isize; 4]>::new();
        let mut changes_wins_vec = Vec::<HashMap<[isize; 4], usize>>::new();
        let secret_numbers = read_input(reader);
        for secret_number in secret_numbers {
            let changes_wins = calc_diffs(secret_number, 2000);
            changes_wins_vec.push(changes_wins.clone());
            changes.extend(changes_wins.keys());
        }
        let answer = find_best_sequence(changes, changes_wins_vec);

        Ok(answer)
    }

    assert_eq!(23, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn find_best_sequence(
    changes: HashSet<[isize; 4]>,
    changes_wins_vec: Vec<HashMap<[isize; 4], usize>>,
) -> usize {
    let mut max_win = 0;
    for seq in changes {
        let mut win = 0;
        for changes_wins in changes_wins_vec.iter() {
            win += *changes_wins.get(&seq).unwrap_or(&0);
        }
        if win >= max_win {
            max_win = win;
        }
    }
    max_win
}

fn calc_diffs(secret_number: usize, next: usize) -> HashMap<[isize; 4], usize> {
    let mut changes_wins = HashMap::<[isize; 4], usize>::new();
    let mut ones_digit = secret_number % 10;
    let mut ones_digits = vec![ones_digit];
    let mut diffs = Vec::<isize>::new();
    diffs.push(0);
    let mut prev_ones_digit = ones_digit;
    let mut next_secret_number = secret_number;
    for i in 1..next {
        next_secret_number = calc_next_secret_number(next_secret_number);
        ones_digit = next_secret_number % 10;
        ones_digits.push(ones_digit);
        diffs.push(ones_digit as isize - prev_ones_digit as isize);
        prev_ones_digit = ones_digit;
    }
    for i in 4..diffs.len() {
        let k = &diffs[i - 3..=i];
        let k = [k[0], k[1], k[2], k[3]];
        changes_wins.entry(k).or_insert(ones_digits[i]);
    }

    changes_wins
}

fn calc_secret_number(secret_number: usize, next: usize) -> usize {
    let mut next_secret_number = secret_number;
    for _ in 0..next {
        next_secret_number = calc_next_secret_number(next_secret_number);
    }
    next_secret_number
}

fn calc_next_secret_number(secret_number: usize) -> usize {
    let mut next_secret_number = secret_number;
    let temp = next_secret_number << 6;
    next_secret_number = temp ^ next_secret_number;
    next_secret_number = next_secret_number & 16777215;
    let temp = next_secret_number >> 5;
    next_secret_number = temp ^ next_secret_number;
    next_secret_number = next_secret_number & 16777215;
    let temp = next_secret_number << 11;
    next_secret_number = temp ^ next_secret_number;
    next_secret_number = next_secret_number & 16777215;
    next_secret_number
}

fn read_input<R: BufRead>(reader: R) -> Vec<usize> {
    let secret_numbers = reader
        .lines()
        .flatten()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    secret_numbers
}
