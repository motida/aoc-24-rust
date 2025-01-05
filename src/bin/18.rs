use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const MEMORY_SIZE: usize = 71;
const MEMORY_SIZE_TEST: usize = 7;
const BYTES_SIZE: usize = 1024;
const BYTES_SIZE_TEST: usize = 12;

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, memory_size: usize, bytes_size: usize) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let bytes = read_input(reader);
        let answer = simulate(&bytes, memory_size, bytes_size);
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(
        22,
        part1(
            BufReader::new(TEST.as_bytes()),
            MEMORY_SIZE_TEST,
            BYTES_SIZE_TEST
        )?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, MEMORY_SIZE, BYTES_SIZE)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, memory_size: usize, bytes_size: usize) -> Result<(u8, u8)> {
        let bytes = read_input(reader);
        let mut index = 0;
        for i in bytes_size..bytes.len() {
            let answer = simulate(&bytes, memory_size, i);
            if answer == usize::MAX {
                index = i - 1;
                break;
            }
        }
        Ok(bytes[index])
    }

    assert_eq!(
        (6, 1),
        part2(
            BufReader::new(TEST.as_bytes()),
            MEMORY_SIZE_TEST,
            BYTES_SIZE_TEST
        )?
    );
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, MEMORY_SIZE, BYTES_SIZE)?);
    println!("Result = {},{}", result.0, result.1);
    //endregion

    Ok(())
}

fn simulate(bytes: &Vec<(u8, u8)>, memory_size: usize, bytes_size: usize) -> usize {
    let memory = create_memory(&bytes, memory_size, bytes_size);
    let (min_distance, _) = find_path(&memory);
    min_distance
}

fn create_memory(bytes: &Vec<(u8, u8)>, memory_size: usize, bytes_size: usize) -> Vec<Vec<char>> {
    let mut memory: Vec<Vec<char>> = vec![vec!['.'; memory_size]; memory_size];
    for i in 0..bytes_size {
        memory[bytes[i].0 as usize][bytes[i].1 as usize] = '#';
    }
    memory
}

fn create_min_distances_map(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), usize> {
    let mut min_scores_map: HashMap<(usize, usize), usize> = HashMap::new();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '#' {
                continue;
            }
            if map[x][y] == '.' {
                min_scores_map.insert((x, y), usize::MAX);
            }
        }
    }
    min_scores_map
}

fn calc_neighborhood_distances(
    map: &Vec<Vec<char>>,
    min_distance_map: &mut HashMap<(usize, usize), usize>,
    min_distance_heap: &mut BinaryHeap<Reverse<(usize, usize, usize)>>,
    cur_pos: (usize, usize),
) {
    let (x, y) = (cur_pos.0, cur_pos.1);
    let distance = min_distance_map[&cur_pos];
    let map_size = map.len();

    if y + 1 < map_size && map[x][y + 1] != '#' && distance + 1 < min_distance_map[&(x, y + 1)] {
        min_distance_map.insert((x, y + 1), distance + 1);
        min_distance_heap.push(Reverse((x, y + 1, min_distance_map[&(x, y + 1)])));
    }
    if y != 0 && map[x][y - 1] != '#' && distance + 1 < min_distance_map[&(x, y - 1)] {
        min_distance_map.insert((x, y - 1), distance + 1);
        min_distance_heap.push(Reverse((x, y - 1, min_distance_map[&(x, y - 1)])));
    }
    if x + 1 < map_size && map[x + 1][y] != '#' && distance + 1 < min_distance_map[&(x + 1, y)] {
        min_distance_map.insert((x + 1, y), distance + 1);
        min_distance_heap.push(Reverse((x + 1, y, min_distance_map[&(x + 1, y)])));
    }
    if x != 0 && map[x - 1][y] != '#' && distance + 1 < min_distance_map[&(x - 1, y)] {
        min_distance_map.insert((x - 1, y), distance + 1);
        min_distance_heap.push(Reverse((x - 1, y, min_distance_map[&(x - 1, y)])));
    }
}

fn find_path(memory: &Vec<Vec<char>>) -> (usize, HashMap<(usize, usize), usize>) {
    let mut min_distance_heap: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut min_distance = usize::MAX;

    let start_pos = (0, 0);
    let end_pos = (memory.len() - 1, memory[0].len() - 1);

    let mut min_distance_map = create_min_distances_map(&memory);
    min_distance_map.insert(start_pos, 0);
    min_distance_heap.push(Reverse((0, 0, 0)));

    while min_distance_heap.len() > 0 {
        let d = min_distance_heap.pop().unwrap().0;
        let (current_pos, current_distance) = ((d.0, d.1), d.2);
        if current_pos == end_pos && current_distance < min_distance {
            min_distance = current_distance;
            //break; /* part 1 can break here */
        }

        if visited.contains(&current_pos) {
            continue;
        }
        visited.insert(current_pos.clone());

        calc_neighborhood_distances(
            memory,
            &mut min_distance_map,
            &mut min_distance_heap,
            current_pos,
        );
    }

    (min_distance, min_distance_map)
}

fn read_input<R: BufRead>(reader: R) -> Vec<(u8, u8)> {
    let mut bytes: Vec<(u8, u8)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let pair: Vec<u8> = line.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
        bytes.push((pair[0], pair[1]));
    }
    bytes
}
