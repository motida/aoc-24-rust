use anyhow::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";


fn main() -> Result<()> {
    start_day(DAY);


    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total: u32 = 0;
        let (mut matrix, mut start_pos_x, mut start_pos_y) = generate_matrix(reader);
        let matrix_height = matrix.len();
        let matrix_width = matrix[0].len();
        
        total = traverse(matrix.clone(), start_pos_x as isize , start_pos_y as isize);
    
        Ok(total as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     let mut total: u32 = 0;
    //     let matrix = generate_matrix(reader);
  
    //     Ok(total as usize)
    // }
    // //
    // assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    // //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    fn traverse(mut matrix: Vec<Vec<char>>, mut start_pos_x: isize, mut start_pos_y: isize) -> u32 {
    
        let delta: HashMap<char, (isize, isize)>  = HashMap::from([ ('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
        let next_direction: HashMap<char, char> = HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);

        let mut total: u32 = 0;
        let mut pos_x: isize;
        let mut pos_y: isize;
        (pos_x, pos_y) = (start_pos_x, start_pos_y);
        let mut current_direction = '^';

        let mut visited: HashSet<(char, isize, isize)> = HashSet::new();
        let mut cycles: usize = 0;

        loop {
            visited.insert((current_direction, pos_x, pos_y));
            if test_cycle(&matrix, pos_x, pos_y, current_direction, &delta,  &next_direction, &visited) {
                cycles += 1;
            }
            if matrix[(pos_x+delta[&current_direction].0) as usize][(pos_y+delta[&current_direction].1) as usize] == '#' {
                current_direction = next_direction[&current_direction];
            }
            matrix[pos_x as usize][pos_y as usize] = 'X';
            
            pos_x = pos_x + delta[&current_direction].0 ;
            pos_y = pos_y + delta[&current_direction].1 ;
            if pos_x <= 0 || pos_x >= (matrix[0].len() as isize) - 1 || pos_y <= 0 || pos_y >= (matrix.len() as isize) - 1 {
                break;
            }
            matrix[pos_x as usize][pos_y as usize] = current_direction;
            
            //println!("{}", total);
        } 
        matrix[pos_x as usize][pos_y as usize] = 'X';
        
        print_matrix(&matrix);
        for v in matrix {
            for c in v {
                if c == 'X' {
                    total += 1;
                }
            }
        }
        println!("{:?}", visited);
        println!("{}", visited.len());
        println!("cycles:   {}", cycles);
        total
    }

    fn test_cycle(matrix: &Vec<Vec<char>>, pos_x: isize, pos_y: isize, direction: char, 
        delta: &HashMap<char, (isize, isize)>,  next_direction: &HashMap<char, char>,
        visited:&HashSet<(char, isize, isize)>) -> bool {
        let obstacle_pos_x: isize = pos_x + delta[&direction].0 ;
        let obstacle_pos_y: isize = pos_y + delta[&direction].1 ;
        let direction = next_direction[&direction];
        let next_pos_x = pos_x + delta[&direction].0 ;
        let next_pos_y = pos_y + delta[&direction].1 ;
        //println!("pos: {}  {},{}", direction, next_pos_x, next_pos_y);
        //println!("visited: {:?}", visited);
        if visited.contains(&(direction, next_pos_x, next_pos_y)) {
            println!("found: {}  {},{}", direction, obstacle_pos_x, obstacle_pos_y);
            return true;
        }
        false
    }

    fn generate_matrix<R: BufRead>(reader: R) -> (Vec<Vec<char>>, i32, i32) {
        let mut matrix = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut start_pos_x = 0;
        let mut start_pos_y = 0;
        for line in reader.lines() {
            let mut chars_vec = Vec::<char>::new();
            y = 0;
            for c in line.unwrap().chars() {
                chars_vec.push(c);
                if c == '^' {
                    (start_pos_x, start_pos_y) = (x, y);     
                }
                y += 1;
            }
            matrix.push(chars_vec);
            x += 1;
        }
        println!("{:?}", matrix);
        println!("{},{}", start_pos_x, start_pos_y);
        (matrix, start_pos_x, start_pos_y)
    }

    fn print_matrix(matrix: &Vec<Vec<char>>) {
        for line in matrix {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }


    Ok(())
}






