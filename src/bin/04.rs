use anyhow::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const WORD: &str = "XMAS";

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total: u32 = 0;
        let mut matrix = generate_matrix(reader);
        let matrix_size = matrix.len();
        // horizontal
        total += horizontal(&matrix);
        // // vertical
        total += vertical(&matrix);
        // diagonals
        total += count_diagonal_references(&matrix, matrix_size);

        mirror(&mut matrix);

        total += count_diagonal_references(&matrix, matrix_size);
    
        Ok(total as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total: u32 = 0;
        let matrix = generate_matrix(reader);
        let matrix_size = matrix.len();
        for i in 1..(matrix.len()-1) {
            for j in 1..(matrix[i].len()-1){               
                if matrix[i][j] != 'A' {
                    continue;   
                }
                if matrix[i-1][j-1] == 'M' && matrix[i-1][j+1] == 'M' && matrix[i+1][j-1] == 'S' && matrix[i+1][j+1] == 'S' {
                    total += 1;
                }
                if matrix[i-1][j-1] == 'M' && matrix[i-1][j+1] == 'S' && matrix[i+1][j-1] == 'M' && matrix[i+1][j+1] == 'S' {
                    total += 1;
                }  
                if matrix[i-1][j-1] == 'S' && matrix[i-1][j+1] == 'M' && matrix[i+1][j-1] == 'S' && matrix[i+1][j+1] == 'M' {
                    total += 1;
                } 
                if matrix[i-1][j-1] == 'S' && matrix[i-1][j+1] == 'S' && matrix[i+1][j-1] == 'M' && matrix[i+1][j+1] == 'M' {
                    total += 1;
                }  
            }
        }
        Ok(total as usize)
    }
    //
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

        
    
    fn mirror(matrix: &mut Vec<Vec<char>>) {
        for n in 0..matrix.len() {
            matrix[n].reverse();  
        }
    }

    fn count_diagonal_references(matrix: &Vec<Vec<char>>, matrix_size: usize) -> u32 {
        let mut count: u32 = 0;
    
        for n in 0..matrix.len() {
      
            let mut line = String::from("");
            for m in 0..=n {
                //print!("{}", matrix[n][m]);
                line.push(matrix[m][n-m]);
            }
            if line.len() >= WORD.len(){
                count += count_occurences(&line, &String::from(WORD));
            }
        }

        for n in 0..matrix.len()-1 {
            let mut line = String::from("");
            for m in 0..=n {
                line.push(matrix[matrix_size-1-m][matrix_size-1-(n-m)]);
            }
            if line.len() >= WORD.len(){
                count += count_occurences(&line, &String::from(WORD));
            }
        } 
        return count;    
    }

    fn horizontal(matrix: &Vec<Vec<char>>) -> u32 {
        let mut total: u32 = 0;
        for n in 0..matrix.len() {
            let mut line = String::from("");
            for m in 0..matrix[n].len() {
                //print!("{}", matrix[n][m]);
                line.push(matrix[n][m]);
            }
            total += count_occurences(&line, &String::from(WORD));
        }
        return total;
    }

    fn vertical(matrix: &Vec<Vec<char>>) -> u32 {
        let mut total: u32 = 0;
        for m in 0..matrix[0].len() {
            let mut line = String::from("");
            for n in 0..matrix.len() {
                //print!("{}", matrix[n][m]);
                line.push(matrix[n][m]);
            }
            total += count_occurences(&line, &String::from(WORD));
        }
        return total;
    }

    fn count_occurences(str: &String, substr: &String) -> u32 {
        let reverse_str = &(str.chars().rev().collect::<String>());
        let pattern = Regex::new(substr).unwrap();
        let count = pattern.find_iter(str).count() as u32;
        let count_reverse = pattern.find_iter(reverse_str).count() as u32;
        return count + count_reverse;
    }

    fn generate_matrix<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        let mut matrix = Vec::new();
        for line in reader.lines() {
            let mut chars_vec = Vec::<char>::new();
            for c in line.unwrap().chars() {
                chars_vec.push(c);
            }
            matrix.push(chars_vec);
        }
        matrix
    }

    Ok(())
}






