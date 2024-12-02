use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; 
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        let mut vec = Vec::new();

        let readr = reader.lines();
        for line in readr {
            let l = line.unwrap();
            let pair: Vec<u32> = l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            vec1.push(pair[0]);
            vec2.push(pair[1]);
         }
        vec1.sort();
        vec2.sort();
        vec = vec1.into_iter().zip(vec2).map(|(a, b)| if a > b {a-b} else {b-a}).collect();
        let sum: u32 = vec.iter().sum();

        Ok(sum as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut map1: HashMap<u32, u32> = HashMap::new();
        let mut map2: HashMap<u32, u32> = HashMap::new();

        let readr = reader.lines();
        for line in readr {
            let l = line.unwrap();
            let pair: Vec<u32> = l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

            *map1.entry(pair[0]).or_default() += 1;
            *map2.entry(pair[1]).or_default() += 1;

         }
         let mut sum: u32 = 0;
         for (key, value) in map1.into_iter() {
            if map2.contains_key(&key) {
               sum += key * value * map2[&key];
            }
        }
        Ok(sum as usize)
    }
    
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
