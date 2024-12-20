use anyhow::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "12"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut garden = read_matrix(reader);
        let (garden_regioned, _) = calc_regions(&mut garden);
        let total_price = calc_price(&garden_regioned);
        Ok(total_price)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut garden = read_matrix(reader);
        let (garden_regioned, region_count) = calc_regions(&mut garden);
        let total_price = calc_price_2(&garden_regioned, region_count);
        Ok(total_price)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

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

    fn print_map(map: &Vec<Vec<usize>>) {
        println!("");
        for line in map {
            print!("|");
            for n in line {
                print!("{:03}|", n);
            }
            println!();
        }
    }

    fn calc_regions(garden: &mut Vec<Vec<char>>) -> (Vec<Vec<usize>>, usize) {
        let n = garden.len();
        let m = garden[0].len();
        let mut map = Vec::new();
        for _ in 0..n {
            let mut row = Vec::new();
            for _ in 0..m {
                row.push(0);
            }
            map.push(row);
        }
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut region: usize = 0;
        for i in 0..garden.len() {
            for j in 0..garden[i].len() {
                if !visited.contains(&(i, j)) {
                    stack.push((i, j));
                    region += 1;
                    map[i][j] = region;
                    while stack.len() > 0 {
                        let (x, y) = stack.pop().unwrap();
                        visited.insert((x, y));
                        if x > 0
                            && map[x - 1][y] == 0
                            && garden[x - 1][y] == garden[x][y]
                            && !visited.contains(&(x - 1, y))
                        {
                            map[x - 1][y] = region;
                            stack.push((x - 1, y));
                        }
                        if x < n - 1
                            && map[x + 1][y] == 0
                            && garden[x + 1][y] == garden[x][y]
                            && !visited.contains(&(x + 1, y))
                        {
                            map[x + 1][y] = region;
                            stack.push((x + 1, y));
                        }
                        if y > 0
                            && map[x][y - 1] == 0
                            && garden[x][y - 1] == garden[x][y]
                            && !visited.contains(&(x, y - 1))
                        {
                            map[x][y - 1] = region;
                            stack.push((x, y - 1));
                        }
                        if y < m - 1
                            && map[x][y + 1] == 0
                            && garden[x][y + 1] == garden[x][y]
                            && !visited.contains(&(x, y + 1))
                        {
                            map[x][y + 1] = region;
                            stack.push((x, y + 1));
                        }
                    }
                }
            }
        }
        (map, region)
    }

    fn calc_price(map: &Vec<Vec<usize>>) -> usize {
        let mut total_price = 0;
        let mut region_perimeters: HashMap<usize, usize> = HashMap::new();
        let mut region_areas: HashMap<usize, usize> = HashMap::new();
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let new_count = region_areas.entry(map[x][y]).or_insert(0);
                *new_count += 1;
                let new_count = region_perimeters.entry(map[x][y]).or_insert(0);
                *new_count += calc_plot_perimeter(&map, x, y);
            }
        }

        for region in region_perimeters.keys() {
            total_price += region_perimeters[region] * region_areas[region];
        }
        total_price
    }

    fn calc_price_2(map: &Vec<Vec<usize>>, region_count: usize) -> usize {
        let mut total_price = 0;
        let mut region_sides: HashMap<usize, usize> = HashMap::new();
        let mut region_areas: HashMap<usize, usize> = HashMap::new();

        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let new_count = region_areas.entry(map[x][y]).or_insert(0);
                *new_count += 1;
            }
        }
        for region in 1..=region_count {
            region_sides.insert(region, calc_region_sides(map, region));
        }
        for region in region_sides.keys() {
            total_price += region_sides[region] * region_areas[region];
        }
        total_price
    }

    fn calc_region_sides(map: &Vec<Vec<usize>>, region: usize) -> usize {
        let mut sides = 0;
        for x in 0..map.len() {
            let mut in_side: bool = false;
            for y in 0..map[x].len() {
                if map[x][y] == region {
                    if in_side {
                        if x == 0 || map[x - 1][y] != region {
                            continue;
                        } else {
                            in_side = false;
                        }
                    } else {
                        if x == 0 || map[x - 1][y] != region {
                            sides += 1;
                            in_side = true;
                        } else {
                            in_side = false;
                        }
                    }
                } else {
                    in_side = false;
                }
            }
            in_side = false;
            for y in 0..map[x].len() {
                if map[x][y] == region {
                    if in_side {
                        if x == map.len() - 1 || map[x + 1][y] != region {
                            continue;
                        } else {
                            in_side = false;
                        }
                    } else {
                        if x == map.len() - 1 || map[x + 1][y] != region {
                            sides += 1;
                            in_side = true;
                        } else {
                            in_side = false;
                        }
                    }
                } else {
                    in_side = false;
                }
            }
        }

        for y in 0..map[0].len() {
            let mut in_side: bool = false;
            for x in 0..map.len() {
                if map[x][y] == region {
                    if in_side {
                        if y == 0 || map[x][y - 1] != region {
                            continue;
                        } else {
                            in_side = false;
                        }
                    } else {
                        if y == 0 || map[x][y - 1] != region {
                            sides += 1;
                            in_side = true;
                        } else {
                            in_side = false;
                        }
                    }
                } else {
                    in_side = false;
                }
            }
            in_side = false;
            for x in 0..map.len() {
                if map[x][y] == region {
                    if in_side {
                        if y == map[x].len() - 1 || map[x][y + 1] != region {
                            continue;
                        } else {
                            in_side = false;
                        }
                    } else {
                        if y == map[x].len() - 1 || map[x][y + 1] != region {
                            sides += 1;
                            in_side = true;
                        } else {
                            in_side = false;
                        }
                    }
                } else {
                    in_side = false;
                }
            }
        }
        sides
    }

    fn calc_plot_perimeter(garden: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
        let region = garden[x][y];
        let mut plot_perimeter = 0;
        if (x > 0 && garden[x - 1][y] != region) || x == 0 {
            plot_perimeter += 1;
        }
        if (x < garden.len() - 1 && garden[x + 1][y] != region) || x == garden.len() - 1 {
            plot_perimeter += 1;
        }
        if (y > 0 && garden[x][y - 1] != region) || y == 0 {
            plot_perimeter += 1;
        }
        if (y < garden[0].len() - 1 && garden[x][y + 1] != region) || y == garden[0].len() - 1 {
            plot_perimeter += 1;
        }
        plot_perimeter
    }

    Ok(())
}
