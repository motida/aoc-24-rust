use anyhow::*;
use itertools::any;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::discriminant;
use std::path::Ancestors;
use std::{isize, usize};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (map, trailheads) = read_input(reader);
        //print_map(&map);
        let answer = calculate_scores(&map, &trailheads);

        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (map, trailheads) = read_input(reader);
        //print_map(&map);
        let answer = calculate_ratings(&map, trailheads);

        Ok(answer)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    fn read_input<R: BufRead>(reader: R) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
        let mut map = Vec::new();
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let mut int_vec = Vec::new();
            for (j, c) in line.unwrap().chars().enumerate() {
                if c == '0' {
                    trailheads.push((i, j));
                }
                int_vec.push(c.to_digit(10).unwrap() as u8);
            }
            map.push(int_vec);
        }
        (map, trailheads)
    }

    fn print_map(map: &Vec<Vec<u8>>) {
        println!("");
        for line in map {
            for i in line {
                print!("{}", i);
            }
            println!();
        }
    }

    fn calculate_scores(map: &Vec<Vec<u8>>, trailheads: &Vec<(usize, usize)>) -> usize {
        if trailheads.len() == 0 {
            return 0;
        }
        let mut total_score = 0;
        for trailhead in trailheads {
            let trailhead = *trailhead;
            let score = calculate_score(map, trailhead);
            total_score += score;
        }
        total_score
    }


    fn calculate_score(map: &Vec<Vec<u8>>, trailhead: (usize, usize)) -> usize {
        let mut current_height = 0;
        let mut current_trailheads = HashSet::new();
        current_trailheads.insert(trailhead);
        let mut next_trailheads = HashSet::new();
        while current_trailheads.len() > 0 && current_height < 9 {
            for (i, j) in current_trailheads.iter() {
                let i = *i;
                let j = *j;
                if i > 0 && map[i - 1][j] == current_height + 1 {
                    next_trailheads.insert((i - 1, j));
                }
                if i < (map.len() - 1) && map[i + 1][j] == current_height + 1 {
                    next_trailheads.insert((i + 1, j));
                }
                if j > 0 && map[i][j - 1] == current_height + 1 {
                    next_trailheads.insert((i, j - 1));
                }
                if j < (map[0].len() - 1) && map[i][j + 1] == current_height + 1 {
                    next_trailheads.insert((i, j + 1));
                }
            }
            current_trailheads = next_trailheads.clone();
            next_trailheads = HashSet::new();
            current_height += 1;
        }
        if current_height == 9 {
            current_trailheads.len()
        } else {
            0
        }
    }
    
    fn calculate_ratings(map: &Vec<Vec<u8>>, trailheads: Vec<(usize, usize)>) -> usize {
        let mut current_height = 0;
        let mut current_trailheads = trailheads.clone();
        let mut next_trailheads = Vec::new();
        while current_trailheads.len() > 0 && current_height < 9 {
            for (i, j) in current_trailheads.iter() {
                let i = *i;
                let j = *j;
                if i > 0 && map[i - 1][j] == current_height + 1 {
                    next_trailheads.push((i - 1, j));
                }
                if i < (map.len() - 1) && map[i + 1][j] == current_height + 1 {
                    next_trailheads.push((i + 1, j));
                }
                if j > 0 && map[i][j - 1] == current_height + 1 {
                    next_trailheads.push((i, j - 1));
                }
                if j < (map[0].len() - 1) && map[i][j + 1] == current_height + 1 {
                    next_trailheads.push((i, j + 1));
                }
            }
            current_trailheads = next_trailheads.clone();
            next_trailheads = Vec::new();
            current_height += 1;
        }
        if current_height == 9 {
            current_trailheads.len()
        } else {
            0
        }
    }
    Ok(())
}
