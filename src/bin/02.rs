use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total: i32 = 0;
        let readr = reader.lines();
        for line in readr {
            let l = line.unwrap();
            let v: Vec<u32> = l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            if is_safe(&v) {
                total += 1;
            } 
        }
        //let answer = reader.lines().flatten().count();
        Ok(total as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

   let input_file = BufReader::new(File::open(INPUT_FILE)?);
   let result = time_snippet!(part1(input_file)?);
   println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total: i32 = 0;
        let readr = reader.lines();
        for line in readr {
            let l = line.unwrap();
            let v: Vec<u32> = l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            if is_safe(&v) {
                total += 1;
            } else {
                for n in 0..v.len() {
                    let v1 = [&v[..n], &v[n+1..]].concat();
                    if is_safe(&v1) {
                        total += 1;
                        break;
                    }
                }

            }
        
        }
        //let answer = reader.lines().flatten().count();
        Ok(total as usize)
    }
    
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())


}

fn is_safe(v: &Vec<u32>)-> bool {
    let d: Vec<i32> = v.windows(2).map(|x| x[1] as i32 - x[0] as i32).collect();

    if d.iter().all(|w| w >=&1 && w <= &3) {
        return true;
    }
    if d.iter().all(|w|  w <= &-1 && w >= &-3 ) {
        return true;
    }
    return false;
}
