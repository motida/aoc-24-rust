use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

//xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
        let readr = reader.lines();
        let mut total_mult: u64 = 0;
        for line in readr {
            let str = line.unwrap();
            for cap in re.captures_iter(&str) {
                let (n1, n2) = (
                    cap[1].parse::<u64>().unwrap(),
                    cap[2].parse::<u64>().unwrap(),
                );
                total_mult += n1 * n2;
            }
        }

        Ok(total_mult as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"(don\'t)|(do)|mul\((\d+)\,(\d+)\)").unwrap();
        let readr = reader.lines();
        let mut total_mult: u64 = 0;
        let mut enabled = true;
        for line in readr {
            let str = line.unwrap();
            for cap in re.captures_iter(&str) {
                if &cap[0] == "don't" {
                    enabled = false;
                } else if &cap[0] == "do" {
                    enabled = true;
                } else if enabled {
                    let (n1, n2) = (
                        cap[3].parse::<u64>().unwrap(),
                        cap[4].parse::<u64>().unwrap(),
                    );
                    total_mult += n1 * n2;
                }
                //let (n1, n2) = (cap[1].parse::<u64>().unwrap() , cap[2].parse::<u64>().unwrap());
                //total_mult += n1 * n2;
            }
        }

        Ok(total_mult as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
