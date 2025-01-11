use anyhow::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "20"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    #[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
    struct Position {
        x: usize,
        y: usize,
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (map, start_pos, end_pos) = read_input(reader);
        let (shortest_path, lookup) = trace_path(&map, start_pos, end_pos);

        let saves = count_saves(&map, &shortest_path, &lookup, 2);
        let total_saves_over_100 = saves
            .into_iter()
            .filter(|(k, _v)| k >= &100)
            .map(|(_, v)| v)
            .sum::<usize>();

        Ok(total_saves_over_100)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (map, start_pos, end_pos) = read_input(reader);
        let (shortest_path, lookup) = trace_path(&map, start_pos, end_pos);

        let saves = count_saves(&map, &shortest_path, &lookup, 20);
        let total_saves_over_100 = saves
            .into_iter()
            .filter(|(k, _v)| k >= &100)
            .map(|(_, v)| v)
            .sum::<usize>();

        Ok(total_saves_over_100)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    fn count_saves(
        map: &Vec<Vec<char>>,
        path: &Vec<Position>,
        lookup: &HashMap<Position, usize>,
        cheat_picoseconds: usize,
    ) -> HashMap<usize, usize> {
        let mut saves = HashMap::new();
        for position in path.iter() {
            let valid_cheats = get_cheats(map, *position, cheat_picoseconds);
            for (from, to) in valid_cheats {
                if lookup[&to] < lookup[&from] {
                    continue;
                }
                let original_distance = lookup[&to] - lookup[&from];
                let distance = min_distance(from, to);
                let save = if distance < original_distance {
                    original_distance - distance
                } else {
                    0
                };
                if save > 0 {
                    *saves.entry(save).or_insert(0) += 1;
                }
            }
        }

        saves
    }

    fn get_cheats(
        map: &Vec<Vec<char>>,
        position: Position,
        cheat_picoseconds: usize,
    ) -> Vec<(Position, Position)> {
        let mut cheats: Vec<(Position, Position)> = Vec::new();
        let mut extended_neighborhood: HashSet<(Position, Position)> = HashSet::new();
        let mut neighborhood: HashSet<(Position, Position)> = HashSet::from([(position, position)]);
        for picosecond in 1..=cheat_picoseconds {
            let mut next_neighborhood = HashSet::new();
            for (_, curr_position) in neighborhood.iter() {
                if curr_position.x > 0 {
                    next_neighborhood.insert((
                        position,
                        Position {
                            x: curr_position.x - 1,
                            y: curr_position.y,
                        },
                    ));
                }
                if curr_position.x < map.len() - 1 {
                    next_neighborhood.insert((
                        position,
                        Position {
                            x: curr_position.x + 1,
                            y: curr_position.y,
                        },
                    ));
                }
                if curr_position.y > 0 {
                    next_neighborhood.insert((
                        position,
                        Position {
                            x: curr_position.x,
                            y: curr_position.y - 1,
                        },
                    ));
                }
                if curr_position.y < map[0].len() - 1 {
                    next_neighborhood.insert((
                        position,
                        Position {
                            x: curr_position.x,
                            y: curr_position.y + 1,
                        },
                    ));
                }
            }
            //println!("next neighborhood: {:?}", next_neighborhood);
            if picosecond > 1 {
                extended_neighborhood.extend(&next_neighborhood);
            }
            neighborhood = next_neighborhood;
        }

        for (distance, next_position) in extended_neighborhood {
            if map[next_position.x][next_position.y] == '.' && position != next_position {
                cheats.push((distance, next_position));
            }
        }
        cheats
    }

    fn min_distance(p1: Position, p2: Position) -> usize {
        (num::abs(p2.x as isize - p1.x as isize) + num::abs(p2.y as isize - p1.y as isize)) as usize
    }

    fn trace_path(
        // Only one path exists
        map: &Vec<Vec<char>>,
        start_position: Position,
        end_position: Position,
    ) -> (Vec<Position>, HashMap<Position, usize>) {
        let mut path: Vec<Position> = Vec::new();
        let mut visited: HashSet<Position> = HashSet::new();
        let mut curr_position = start_position;
        path.push(curr_position);
        visited.insert(curr_position);
        while curr_position != end_position {
            if map[curr_position.x - 1][curr_position.y] == '.' {
                let next_position = Position {
                    x: curr_position.x - 1,
                    y: curr_position.y,
                };
                if !visited.contains(&next_position) {
                    path.push(next_position);
                    visited.insert(next_position);
                    curr_position = next_position;
                    continue;
                }
            }
            if map[curr_position.x + 1][curr_position.y] == '.' {
                let next_position = Position {
                    x: curr_position.x + 1,
                    y: curr_position.y,
                };
                if !visited.contains(&next_position) {
                    path.push(next_position);
                    visited.insert(next_position);
                    curr_position = next_position;
                    continue;
                }
            }
            if map[curr_position.x][curr_position.y - 1] == '.' {
                let next_position = Position {
                    x: curr_position.x,
                    y: curr_position.y - 1,
                };
                if !visited.contains(&next_position) {
                    path.push(next_position);
                    visited.insert(next_position);
                    curr_position = next_position;
                    continue;
                }
            }
            if map[curr_position.x][curr_position.y + 1] == '.' {
                let next_position = Position {
                    x: curr_position.x,
                    y: curr_position.y + 1,
                };
                if !visited.contains(&next_position) {
                    path.push(next_position);
                    visited.insert(next_position);
                    curr_position = next_position;
                    continue;
                }
            }
        }

        let mut lookup: HashMap<Position, usize> = HashMap::new();
        for (i, position) in path.iter().enumerate() {
            lookup.insert(*position, i);
        }

        (path, lookup)
    }

    fn read_input<R: BufRead>(reader: R) -> (Vec<Vec<char>>, Position, Position) {
        let mut map = Vec::new();
        let mut start_position: Position = Position { x: 0, y: 0 };
        let mut end_position: Position = Position { x: 0, y: 0 };
        let mut x = 0;
        let mut y;
        for result in reader.lines() {
            let line = result.unwrap().trim().to_string();
            let mut chars_vec = Vec::<char>::new();
            y = 0;
            for c in line.chars() {
                if c == 'S' {
                    start_position = Position { x, y };
                }
                if c == 'E' {
                    end_position = Position { x, y };
                }
                chars_vec.push(if c == 'S' || c == 'E' { '.' } else { c });
                y += 1;
            }
            x += 1;
            map.push(chars_vec);
        }
        (map, start_position, end_position)
    }

    fn print_map(map: &Vec<Vec<char>>) {
        println!("");
        for line in map {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }

    Ok(())
}
