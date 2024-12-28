use anyhow::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Cursor};
use std::io::{BufRead, BufReader};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "15"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


const TEST_PART2_SMALL: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

const TEST_PART1_SMALL: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<>^<v<^
";

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut map, start_pos, moves) = read_input(reader);
        print_map(&map);
        simulate(&mut map, start_pos, moves);
        print_map(&map);
        let answer = calc_sum_gps_coordinates(&map);
        Ok(answer)
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (map, _, moves) = read_input(reader);
        print_map(&map);
        let (mut map, robot_start_pos) = widen_map(map);
        //println!("Movements: {:?}", moves);
        print_map(&map);
        map = simulate_2(&mut map, robot_start_pos, moves);
        print_map(&map);
        let answer = calc_sum_gps_coordinates(&map);
        Ok(answer)
    }

    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    //endregion
    fn simulate(map: &mut Vec<Vec<char>>, start_pos: (usize, usize), moves: Vec<char>) {
        let mut pos = start_pos.clone();

        for movement in moves {
            make_move(movement, map, &mut pos);
        }
    }

    fn simulate_2(map: &Vec<Vec<char>>, start_pos: (usize, usize), moves: Vec<char>) -> Vec<Vec<char>> {
        let mut robot_pos = start_pos.clone();
        let mut current_map = map.clone();
        for movement in moves {
            //println!("Movement: {}", movement);
            //println!("Robot position Before move{:?}", robot_pos);
            //let mut name = String::new();
            //std::io::stdin().read_line(&mut name);
            let mut work_map = current_map.clone();
            if make_move_2(movement, &mut work_map, &mut robot_pos) {
                current_map = work_map.clone();
            }
            //print_map(&current_map);
        }
        current_map
    }

    fn calc_sum_gps_coordinates(map: &Vec<Vec<char>>) -> usize {
        let mut sum_gps_coordinates = 0;
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                if map[x][y] == 'O' || map[x][y] == '[' {
                    sum_gps_coordinates += 100 * x + y;
                }
            }
        }
        sum_gps_coordinates
    }

    fn make_move_2(
        movement: char,
        map: &mut Vec<Vec<char>>,
        robot_pos: &mut (usize, usize),
    ) -> bool {
        let deltas: HashMap<char, (isize, isize)> =
            HashMap::from([('>', (0, 1)), ('<', (0, -1)), ('v', (1, 0)), ('^', (-1, 0))]);
        let (dx, dy) = deltas[&movement];

        let mut next_pos = (
            ((robot_pos.0 as isize + dx) as usize),
            ((robot_pos.1 as isize + dy) as usize),
        );
        let next_tile = map[next_pos.0][next_pos.1];
        match next_tile {
            '#' => false,
            '.' => {
                move_robot(map, robot_pos, dx, dy);
                true
            }
            '[' | ']' => match movement {
                '>' | '<' => {
                    if move_box_horirzontly(movement, map, next_pos) {
                        move_robot(map, robot_pos, dx, dy);
                        true
                    } else {
                        false
                    }
                }
                '^' | 'v' => {
                    let mut visited = HashSet::new();
                    if next_tile == ']' {
                        next_pos = (next_pos.0, next_pos.1 - 1);
                    }
                    if move_box_vertically(movement, map, next_pos, &mut visited) {
                        move_robot(map, robot_pos, dx, dy);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn move_robot(map: &mut Vec<Vec<char>>, robot_pos: &mut (usize, usize), dx: isize, dy: isize) {
        map[robot_pos.0][robot_pos.1] = '.';
        robot_pos.0 = (robot_pos.0 as isize + dx) as usize;
        robot_pos.1 = (robot_pos.1 as isize + dy) as usize;
        map[robot_pos.0][robot_pos.1] = '@';
    }

    fn move_box_vertically(
        movement: char,
        map: &mut Vec<Vec<char>>,
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        let dx: isize = if movement == 'v' { 1 } else { -1 };
        let next_pos = ((pos.0 as isize + dx) as usize, pos.1);
        let (next_tile_left, next_tile_right) =
            (map[next_pos.0][next_pos.1], map[next_pos.0][next_pos.1 + 1]);
        let mut can_move_box;
        match (next_tile_left, next_tile_right) {
            ('.', '.') => {
                can_move_box = true;
            }
            ('#', _) | (_, '#') => {
                can_move_box = false;
            }
            ('[', ']') => {
                if move_box_vertically(movement, map, next_pos, visited) {
                    // map[next_pos.0][next_pos.1] = map[pos.0][pos.1];
                    // map[pos.0][pos.1] = '.';
                    // map[next_pos.0][next_pos.1 + 1] = map[pos.0][pos.1 + 1];
                    // map[pos.0][pos.1 + 1] = '.';
                    can_move_box = true;
                } else {
                    can_move_box = false;
                }
            }
            (']', '[') => {
                can_move_box =
                    move_box_vertically(movement, map, (next_pos.0, next_pos.1 - 1), visited);
                if can_move_box {
                    can_move_box =
                        move_box_vertically(movement, map, (next_pos.0, next_pos.1 + 1), visited);
                }
            }
            (']', _) => {
                can_move_box =
                    move_box_vertically(movement, map, (next_pos.0, next_pos.1 - 1), visited);
            }
            (_, '[') => {
                can_move_box =
                    move_box_vertically(movement, map, (next_pos.0, next_pos.1 + 1), visited);
            }
            _ => {
                can_move_box = false;
            }
        }
        if can_move_box {
            map[next_pos.0][next_pos.1] = map[pos.0][pos.1];
            map[next_pos.0][next_pos.1 + 1] = map[pos.0][pos.1 + 1];
            map[pos.0][pos.1] = '.';
            map[pos.0][pos.1 + 1] = '.';
        }

        can_move_box
    }

    fn move_box_horirzontly(movement: char, map: &mut Vec<Vec<char>>, pos: (usize, usize)) -> bool {
        let dy: isize = if movement == '>' { 1 } else { -1 };
        let next_pos = (pos.0, ((pos.1 as isize) + (2 * dy)) as usize);
        match map[next_pos.0][next_pos.1] {
            '.' => {
                map[pos.0][((pos.1 as isize) + (2 * dy)) as usize] =
                    map[pos.0][((pos.1 as isize) + dy) as usize];
                map[pos.0][((pos.1 as isize) + dy) as usize] = map[pos.0][pos.1];
                map[pos.0][pos.1] = '.';
                true
            }
            ']' | '[' => {
                if move_box_horirzontly(movement, map, next_pos) {
                    map[pos.0][((pos.1 as isize) + (2 * dy)) as usize] =
                        map[pos.0][((pos.1 as isize) + dy) as usize];
                    map[pos.0][((pos.1 as isize) + dy) as usize] = map[pos.0][pos.1];
                    map[pos.0][pos.1] = '.';
                    true
                } else {
                    false
                }
            }
            '#' => false,
            _ => false,
        }
    }

    fn make_move(movement: char, map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
        let deltas: HashMap<char, (i32, i32)> =
            HashMap::from([('>', (0, 1)), ('<', (0, -1)), ('v', (1, 0)), ('^', (-1, 0))]);
        let (dx, dy) = deltas[&movement];

        let next_tile = map[(pos.0 as i32 + dx) as usize][(pos.1 as i32 + dy) as usize];
        match next_tile {
            '#' => return,
            '.' => {
                map[pos.0][pos.1] = '.';
                pos.0 = (pos.0 as i32 + dx) as usize;
                pos.1 = (pos.1 as i32 + dy) as usize;
                map[pos.0][pos.1] = '@';
                return;
            }
            'O' => {
                let mut next_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
                while map[next_pos.0][next_pos.1] == 'O' {
                    next_pos = (
                        (next_pos.0 as i32 + dx) as usize,
                        (next_pos.1 as i32 + dy) as usize,
                    );
                }
                match map[next_pos.0][next_pos.1] {
                    '.' => {
                        map[pos.0][pos.1] = '.';
                        pos.0 = (pos.0 as i32 + dx) as usize;
                        pos.1 = (pos.1 as i32 + dy) as usize;
                        map[pos.0][pos.1] = '@';
                        let mut curr_pos = pos.clone();
                        while curr_pos != next_pos {
                            curr_pos.0 = (curr_pos.0 as i32 + dx) as usize;
                            curr_pos.1 = (curr_pos.1 as i32 + dy) as usize;
                            map[curr_pos.0][curr_pos.1] = 'O';
                        }
                    }
                    '#' => return,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn widen_map(map: Vec<Vec<char>>) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut new_map = Vec::new();
        let mut start_pos = (0, 0);
        let mut x = 0;
        let mut y;

        for row in map {
            let mut chars_vec = Vec::<char>::new();
            y = 0;
            for c in row {
                if c == '@' {
                    chars_vec.push('@');
                    chars_vec.push('.');
                    start_pos = (x, y);
                    y += 2;
                } else if c == 'O' {
                    chars_vec.push('[');
                    chars_vec.push(']');
                    y += 2;
                } else {
                    chars_vec.push(c);
                    chars_vec.push(c);
                    y += 2;
                }
            }
            x += 1;
            new_map.push(chars_vec);
        }
        (new_map, start_pos)
    }

    fn read_input<R: BufRead>(reader: R) -> (Vec<Vec<char>>, (usize, usize), Vec<char>) {
        let mut map = Vec::new();
        let mut start_pos = (0, 0);
        let mut moves = Vec::new();
        let mut map_section: bool = true;
        let mut x = 0;
        let mut y;
        for result in reader.lines() {
            let line = result.unwrap().trim().to_string();
            if line.is_empty() {
                map_section = false;
                continue;
            }
            if map_section {
                let mut chars_vec = Vec::<char>::new();
                y = 0;
                for c in line.chars() {
                    chars_vec.push(c);
                    if c == '@' {
                        start_pos = (x, y);
                    }
                    y += 1;
                }
                x += 1;
                map.push(chars_vec);
            } else {
                for c in line.chars() {
                    moves.push(c);
                }
            }
        }
        (map, start_pos, moves)
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
