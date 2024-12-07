use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
fn main() -> Result<()> {
    start_day(DAY);

    const SIZE: usize = 100;
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix: Vec<Vec<u16>> = vec![vec![0; SIZE]; SIZE];
        let mut updates: Vec<Vec<u16>> = Vec::new();
        let mut answer: usize = 0;
        (matrix, updates) = read_input(reader);
        for update in &updates {
            if is_correct_ordering(&update, &matrix) {
                answer += update[update.len() / 2] as usize;
            }
        }

        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // //endregion

    // //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix: Vec<Vec<u16>> = vec![vec![0; SIZE]; SIZE];
        let mut updates: Vec<Vec<u16>> = Vec::new();
        let mut answer: usize = 0;
        (matrix, updates) = read_input(reader);
        for update in &updates {
            if is_correct_ordering(&update, &matrix) {
                continue;
            }
            answer += order_update(&update, &matrix);
        }
        Ok(answer as usize)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    fn is_correct_ordering(update: &Vec<u16>, matrix: &Vec<Vec<u16>>) -> bool {
        for pair in update.windows(2) {
            if matrix[pair[0] as usize][pair[1] as usize] == 0 {
                return false;
            }
        }
        return true;
    }

    fn order_update(update: &Vec<u16>, matrix: &Vec<Vec<u16>>) -> usize {
        let mut counter = HashMap::new();
        for n in 0..update.len() {
            let v1 = [&update[..n], &update[n + 1..]].concat();
            let mut count: u16 = 0;
            for v in v1 {
                count += matrix[update[n] as usize][v as usize];
            }
            counter.insert(update[n], count);
        }

        let mut hash_vec: Vec<(&u16, &u16)> = counter.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        return *hash_vec[hash_vec.len() / 2].0 as usize;
    }

    fn read_input<R: BufRead>(reader: R) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
        let mut matrix: Vec<Vec<u16>> = vec![vec![0; SIZE]; SIZE];
        let mut updates: Vec<Vec<u16>> = Vec::new();
        let mut first_section: bool = true;
        for line in reader.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                first_section = false;
                continue;
            }
            if first_section {
                let pair: Vec<u16> = line.split('|').map(|x| x.parse::<u16>().unwrap()).collect();
                matrix[pair[0] as usize][pair[1] as usize] = 1;
            } else {
                let update: Vec<u16> = line.split(',').map(|x| x.parse::<u16>().unwrap()).collect();
                updates.push(update);
            }
        }
        (matrix, updates)
    }

    Ok(())
}
