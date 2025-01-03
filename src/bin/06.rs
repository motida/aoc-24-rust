use anyhow::*;
use std::arch::is_aarch64_feature_detected;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{isize, usize};

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
        let (matrix, (start_x, start_y)) = read_matrix(reader);

        let (cycle_free, path_length, path) = simulate(matrix, (start_x, start_y));

        Ok(path_length)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (original_matrix, (start_x, start_y)) = read_matrix(reader);
        let (cycle_free, _, mut path) = simulate(original_matrix.clone(), (start_x, start_y));

        let mut obstructions = 0;
        path.remove(&(start_x, start_y)); 
        for (x, y) in path {
            let mut matrix = original_matrix.clone();
            matrix[x][y] = 'O';
            let (is_cycle, _, _) = simulate(matrix.clone(), (start_x, start_y));
            if is_cycle {
                obstructions += 1;
            }
        }
        Ok(obstructions)
    }
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    // //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    fn simulate(
        mut matrix: Vec<Vec<char>>,
        (start_x, start_y): (usize, usize),
    ) -> (bool, usize, HashSet<(usize, usize)>) {
        //print_matrix(&matrix);
        let delta_movement: HashMap<char, (isize, isize)> =
            HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
        let turn_direction: HashMap<char, char> =
            HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);

        let mut visited: HashSet<(char, usize, usize)> = HashSet::new();

        let (mut x, mut y) = (start_x, start_y);
        let mut direction = matrix[x][y];
        let mut is_cycle = false;

        loop {
            if visited.contains(&(direction, x, y)) {
                is_cycle = true;
                break;
            }
            // Whenever on boundry must be on direction of falling off
            if is_on_boundry(x, y, matrix.len(), matrix[0].len()) {
                visited.insert((direction, x, y));
                break;
            }
            let next_x = (x as isize + delta_movement[&direction].0) as usize;
            let next_y = (y as isize + delta_movement[&direction].1) as usize;
            //next_x
            if matrix[next_x][next_y] == '#' || matrix[next_x][next_y] == 'O' {
                direction = turn_direction[&direction];
                continue;
            }
            visited.insert((direction, x, y));
            matrix[x][y] = 'X';
            x = ((x as isize) + delta_movement[&direction].0) as usize;
            y = ((y as isize) + delta_movement[&direction].1) as usize;
        
            matrix[x][y] = direction;
        }
        matrix[x][y] = 'X';
        //print_matrix(&matrix);
        let unique_visited: HashSet<(usize, usize)> =
            visited.into_iter().map(|x: (char, usize, usize)| (x.1, x.2)).collect();
        (is_cycle, unique_visited.len(), unique_visited)
    }

    fn read_matrix<R: BufRead>(reader: R) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut matrix = Vec::new();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut start_x = 0;
        let mut start_y = 0;
        for line in reader.lines() {
            let mut chars_vec = Vec::<char>::new();
            y = 0;
            for c in line.unwrap().chars() {
                chars_vec.push(c);
                if c == '^' {
                    (start_x, start_y) = (x, y);
                }
                y += 1;
            }
            matrix.push(chars_vec);
            x += 1;
        }
        (matrix, (start_x, start_y))
    }

    fn print_matrix(matrix: &Vec<Vec<char>>) {
        println!("");
        for line in matrix {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }

    fn is_on_boundry(x: usize, y: usize, matrix_height: usize, matrix_width: usize) -> bool {
        if x <= 0 || x >= (matrix_height - 1) || y <= 0 || y >= (matrix_width -1) {
            true
        } else {
            false
        }
    }

    Ok(())
}
