use anyhow::*;
use itertools::any;
use std::arch::is_aarch64_feature_detected;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::discriminant;
use std::path::Ancestors;
use std::{isize, usize};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix = read_matrix(reader);
        print_matrix(&matrix);

        let antennas = extract_antennas(&matrix);

        let antinodes: HashSet<(usize, usize)> =
            find_all_antinodes(&antennas, matrix.len(), matrix[0].len());

        matrix_with_antinodes(&mut matrix, &antinodes);
        print_matrix(&matrix);
        Ok(antinodes.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix = read_matrix(reader);
        print_matrix(&matrix);

        let antennas = extract_antennas(&matrix);

        let antinodes: HashSet<(usize, usize)> =
            find_all_antinodes_extended(&antennas, matrix.len(), matrix[0].len());

        matrix_with_antinodes(&mut matrix, &antinodes);
        print_matrix(&matrix);
        Ok(antinodes.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    //endregion

    fn read_matrix<R: BufRead>(reader: R) -> Vec<Vec<char>> {
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

    fn print_matrix(matrix: &Vec<Vec<char>>) {
        println!("");
        for line in matrix {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }

    fn matrix_with_antinodes(matrix: &mut Vec<Vec<char>>, antinodes: &HashSet<(usize, usize)>) {
        for (x, y) in antinodes {
            if matrix[*x][*y] == '.' {
                matrix[*x][*y] = '#';
            } else {
                matrix[*x][*y] = 'Z';
            }
        }
    }

    fn extract_antennas(matrix: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                let c = &matrix[x][y];
                if matrix[x][y] == '.' {
                    continue;
                }
                if antennas.contains_key(c) {
                    antennas.get_mut(c).unwrap().push((x, y));
                } else {
                    antennas.insert(matrix[x][y], vec![(x, y)]);
                }
            }
        }
        antennas
    }

    fn find_all_antinodes(
        anthennas: &HashMap<char, Vec<(usize, usize)>>,
        matrix_height: usize,
        matrix_width: usize,
    ) -> HashSet<(usize, usize)> {
        //let mut antinodes: Vec<(usize, usize)> = Vec::new();
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
        for (anthenna_freq, anthennas) in anthennas {
            //println!("{}, {:?}", anthenna_freq, anthennas);

            for i in 0..anthennas.len() {
                for j in (i + 1)..anthennas.len() {
                    let (x1, y1) = anthennas[i];
                    let (x2, y2) = anthennas[j];
                    let x_distance = (x1 as isize - x2 as isize).abs();
                    let y_distance = (y1 as isize - y2 as isize).abs();
                    if (x1 <= x2 && y1 < y2) || (x2 <= x1 && y2 < y1) {
                        let ax_1 = x1 as isize - x_distance;
                        let ax_2 = x2 as isize + x_distance;
                        let ay_1 = y1 as isize - y_distance;
                        let ay_2 = y2 as isize + y_distance;
                        if is_inside_matrix(
                            ax_1,
                            ay_1,
                            matrix_height as isize,
                            matrix_width as isize,
                        ) {
                            antinodes.insert((ax_1 as usize, ay_1 as usize));
                        }
                        if is_inside_matrix(
                            ax_2,
                            ay_2,
                            matrix_height as isize,
                            matrix_width as isize,
                        ) {
                            antinodes.insert((ax_2 as usize, ay_2 as usize));
                        }
                    }

                    if (x1 < x2 && y1 >= y2) || (x2 < x1 && y2 >= y1) {
                        let ax_1 = x1 as isize - x_distance;
                        let ax_2 = x2 as isize + x_distance;
                        let ay_1 = y1 as isize + y_distance;
                        let ay_2 = y2 as isize - y_distance;
                        if is_inside_matrix(
                            ax_1,
                            ay_1,
                            matrix_height as isize,
                            matrix_width as isize,
                        ) {
                            antinodes.insert((ax_1 as usize, ay_1 as usize));
                        }
                        if is_inside_matrix(
                            ax_2,
                            ay_2,
                            matrix_height as isize,
                            matrix_width as isize,
                        ) {
                            antinodes.insert((ax_2 as usize, ay_2 as usize));
                        }
                    }
                }
            }
        }
        antinodes
    }

    fn find_all_antinodes_extended(
        anthennas: &HashMap<char, Vec<(usize, usize)>>,
        matrix_height: usize,
        matrix_width: usize,
    ) -> HashSet<(usize, usize)> {
        //let mut antinodes: Vec<(usize, usize)> = Vec::new();
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
        for (anthenna_freq, anthennas) in anthennas {
            //println!("{}, {:?}", anthenna_freq, anthennas);

            for i in 0..anthennas.len() {
                for j in (i + 1)..anthennas.len() {
                    let (x1, y1) = anthennas[i];
                    let (x2, y2) = anthennas[j];
                    let x_distance = (x1 as isize - x2 as isize).abs();
                    let y_distance = (y1 as isize - y2 as isize).abs();
                    let mut x_distance_multiple = 0;
                    let mut y_distance_multiple = 0;
                    while x_distance_multiple <= matrix_height as isize
                        && y_distance_multiple <= matrix_width as isize
                    {
                        if (x1 <= x2 && y1 < y2) || (x2 <= x1 && y2 < y1) {
                            let ax_1 = x1 as isize - x_distance_multiple;
                            let ax_2 = x2 as isize + x_distance_multiple;
                            let ay_1 = y1 as isize - y_distance_multiple;
                            let ay_2 = y2 as isize + y_distance_multiple;
                            if is_inside_matrix(
                                ax_1,
                                ay_1,
                                matrix_height as isize,
                                matrix_width as isize,
                            ) {
                                antinodes.insert((ax_1 as usize, ay_1 as usize));
                            }
                            if is_inside_matrix(
                                ax_2,
                                ay_2,
                                matrix_height as isize,
                                matrix_width as isize,
                            ) {
                                antinodes.insert((ax_2 as usize, ay_2 as usize));
                            }
                        }

                        if (x1 < x2 && y1 >= y2) || (x2 < x1 && y2 >= y1) {
                            let ax_1 = x1 as isize - x_distance_multiple;
                            let ax_2 = x2 as isize + x_distance_multiple;
                            let ay_1 = y1 as isize + y_distance_multiple;
                            let ay_2 = y2 as isize - y_distance_multiple;
                            if is_inside_matrix(
                                ax_1,
                                ay_1,
                                matrix_height as isize,
                                matrix_width as isize,
                            ) {
                                antinodes.insert((ax_1 as usize, ay_1 as usize));
                            }
                            if is_inside_matrix(
                                ax_2,
                                ay_2,
                                matrix_height as isize,
                                matrix_width as isize,
                            ) {
                                antinodes.insert((ax_2 as usize, ay_2 as usize));
                            }
                        }
                        x_distance_multiple += x_distance;
                        y_distance_multiple += y_distance;

                    }
                }
            }
        }
        antinodes
    }

    fn is_inside_matrix(x: isize, y: isize, matrix_height: isize, matrix_width: isize) -> bool {
        x >= 0 && x < matrix_height && y >= 0 && y < matrix_width
    }

    Ok(())
}
