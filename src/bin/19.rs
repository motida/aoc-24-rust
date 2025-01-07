use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"; // TODO: Add the test input

/* bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb */
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let (towels, designs) = read_input(reader);
        let mut counter = 0;
        for design in designs {
            if is_possible(design, &towels) {
                counter += 1;
            }
        }
        Ok(counter)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (towels, designs) = read_input(reader);
        let mut counter = 0;
        for design in designs {
            counter += count_possible(&design, &towels, &mut HashMap::new());
        }
        //let answer = reader.lines().flatten().count();
        Ok(counter)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);
    // //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn is_possible(design: String, towels: &Vec<String>) -> bool {
    let mut possible: bool = false;
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        possible =
            design.starts_with(towel) && is_possible(design[towel.len()..].to_string(), towels);
        if possible {
            break;
        }
    }
    possible
}

fn count_possible(design: &String, towels: &Vec<String>, memo: &mut HashMap<String, usize>) -> usize {
    let mut possible: usize = 0;
    if design.is_empty() {
        return 1;
    }
    for towel in towels {
        if design.starts_with(towel) {
            let next_design = design[towel.len()..].to_string();
            if memo.contains_key(&next_design) {
                possible += memo[&next_design];
            } else {
                let c = count_possible(&next_design, towels, memo);
                possible += c;
                memo.insert(next_design, c);
            }
        }
    }
    possible
}

fn read_input<R: BufRead>(reader: R) -> (Vec<String>, Vec<String>) {
    let lines = reader.lines().flatten().collect::<Vec<String>>();
    let towels: Vec<String> = lines[0].split(',').map(|x| x.trim().to_string()).collect();
    let designs = lines[2..].to_vec();
    (towels, designs)
}
