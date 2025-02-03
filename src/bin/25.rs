use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "25"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####

"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let(locks, keys) = read_input(reader);
        let mut counter = 0;
        for lock in locks.iter() {
            for key in keys.iter() {
                let mut overlap: bool = false;
                for j in 0..5 {
                    if lock[j] + key[j] >5 {
                        overlap = true;
                    } 
                }
                if ! overlap {
                    counter += 1;
                }
            }
        }
        Ok(counter)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_lock_or_key(lines: &Vec<String>) -> (bool, [u8; 5]) {
    let mut array: [u8; 5] = [0; 5];
    let is_lock: bool;
    if lines[0] == "#####".to_string() {
        is_lock = true;
    } else {
        is_lock = false;
    }
    for j in 0..5 {
        let mut counter = 0;
        for i in 1..6 {
            if lines[i].chars().nth(j).unwrap() == '#' {
                counter += 1;
            }
        }
        array[j] = counter;
    }
    (is_lock, array)
}

fn read_input<R: BufRead>(reader: R) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut lock_lines = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    let mut line_counter = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            continue;
        }
        line_counter = line_counter + 1;

        lock_lines.push(line);
        if line_counter == 7 {
            let (is_lock, array) = read_lock_or_key(&lock_lines);
            if is_lock {
                locks.push(array);
            } else {
                keys.push(array);
            }
            lock_lines.clear();
            line_counter = 0;
        }
    }
    (locks, keys)
}
