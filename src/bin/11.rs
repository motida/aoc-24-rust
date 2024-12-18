use anyhow::*;
use num::bigint::BigInt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut stones = read_input(reader);
        for i in 0..25 {
            stones = blink(stones);
        }
        Ok(stones.len())
    }

    // assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2

    println!("=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let stones = read_input(reader);
        let mut counter = vec_to_counter(stones);
        for i in 0..75 {
            counter = blink_2(counter);
        }
        Ok(counter.values().sum())
    }
    assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion
    Ok(())
}

fn blink(stones: Vec<num::bigint::BigInt>) -> Vec<num::bigint::BigInt> {
    let mut new_stones: Vec<num::bigint::BigInt> = Vec::new();
    for stone in stones {
        if stone == BigInt::from(0) {
            new_stones.push(BigInt::from(1));
            continue;
        }
        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let s1 = s[0..s.len() / 2].parse::<num::bigint::BigInt>().unwrap();
            let s2 = s[s.len() / 2..].parse::<num::bigint::BigInt>().unwrap();
            new_stones.push(s1);
            new_stones.push(s2);
            continue;
        }
        new_stones.push(stone * 2024);
    }
    new_stones
}

fn blink_2(stone_counter: HashMap<num::bigint::BigInt, usize>) -> HashMap<num::bigint::BigInt, usize>{
    let mut new_stone_counter = HashMap::new();
    for (stone, count) in stone_counter {
        if stone == BigInt::from(0) {
            let new_count = new_stone_counter.entry(BigInt::from(1)).or_insert(0);
            *new_count += count;
            continue;
        }
        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let s1 = s[0..s.len() / 2].parse::<num::bigint::BigInt>().unwrap();
            let s2 = s[s.len() / 2..].parse::<num::bigint::BigInt>().unwrap();
            let new_count = new_stone_counter.entry(BigInt::from(s1)).or_insert(0);
            *new_count += count;
            let new_count = new_stone_counter.entry(BigInt::from(s2)).or_insert(0);
            *new_count += count;
            continue;
        }
        let new_stone = stone * 2024;
        let new_count = new_stone_counter.entry(BigInt::from(new_stone)).or_insert(0);
        *new_count += count;
    }
    new_stone_counter
}

fn read_input<R: BufRead>(mut reader: R) -> Vec<num::bigint::BigInt> {
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    line = line.trim().to_string();
    let stones: Vec<num::bigint::BigInt> = line
        .split_whitespace()
        .map(|x| x.parse::<num::bigint::BigInt>().unwrap())
        .collect();
    stones
}

fn vec_to_counter(stones: Vec<num::bigint::BigInt>) -> HashMap<num::bigint::BigInt, usize> {
    let mut counter: HashMap<num::bigint::BigInt, usize> = HashMap::new();
    for stone in stones {
        let count = counter.entry(stone).or_insert(0);
        *count += 1;
    }
    counter
}
