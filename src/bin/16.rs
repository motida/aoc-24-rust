use anyhow::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};


use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "16"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const MOVE_SCORE: usize = 1;
const TURN_SCORE: usize = 1000;

const TEST_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    #[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
    enum Direction {
        EAST,
        NORTH,
        WEST,
        SOUTH,
    }

    #[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
    struct Position {
        x: usize,
        y: usize,
        direction: Direction,
    }

    #[derive(Debug, Hash, Copy, Clone)]
    struct Distance {
        position: Position,
        distance: usize,
    }

    impl Eq for Distance {}

    impl PartialEq for Distance {
        fn eq(&self, other: &Distance) -> bool {
            self.distance == other.distance
        }
    }

    impl Ord for Distance {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.distance.cmp(&other.distance)
        }
    }
    impl PartialOrd for Distance {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (map, start_pos, end_pos) = read_input(reader);
        //print_map(&map);

        let (answer, _) = find_path(&map, start_pos, end_pos);

        Ok(answer)
    }

    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut map, start_pos, end_pos) = read_input(reader);
        print_map(&map);

        let (min_distance, min_distance_map) = find_path(&map, start_pos, end_pos);
        let tiles_count = count_shortest_paths_tiles(
            &mut map,
            start_pos,
            end_pos,
            min_distance,
            min_distance_map,
        );
        print_map(&map);

        Ok(tiles_count)
    }

    assert_eq!(45, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    //endregion
    fn find_path(
        map: &Vec<Vec<char>>,
        start_pos: (usize, usize),
        end_pos: (usize, usize),
    ) -> (usize, HashMap<Position, usize>) {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut min_distance = usize::MAX;

        let (mut min_scores_map, mut min_scores_heap) = create_min_scores(map);

        while min_scores_heap.len() > 0 {
            let s_d = min_scores_heap.pop().unwrap().0;
            let current_pos = s_d.position;
            let current_distance = s_d.distance;
            if current_pos.x == end_pos.0
                && current_pos.y == end_pos.1
                && current_distance < min_distance
            {
                min_distance = current_distance;
                //break; /* part 1 can break here */
            }

            if visited.contains(&current_pos) {
                continue;
            }
            visited.insert(current_pos.clone());

            calc_neighborhood_distances(
                map,
                &mut min_scores_map,
                &mut min_scores_heap,
                &current_pos,
            );
        }

        (min_distance, min_scores_map)
    }

    fn count_shortest_paths_tiles(
        map: &mut Vec<Vec<char>>,
        start_pos: (usize, usize),
        end_pos: (usize, usize),
        min_distance: usize,
        min_scores_map: HashMap<Position, usize>,
    ) -> usize {
        let mut stack: Vec<Position> = Vec::new();
        let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();

        let p = Position {
            x: end_pos.0,
            y: end_pos.1,
            direction: Direction::EAST,
        };
        if min_scores_map[&p] == min_distance {
            stack.push(p);
        }
        let p = Position {
            x: end_pos.0,
            y: end_pos.1,
            direction: Direction::WEST,
        };
        if min_scores_map[&p] == min_distance {
            stack.push(p);
        }
        let p = Position {
            x: end_pos.0,
            y: end_pos.1,
            direction: Direction::NORTH,
        };
        if min_scores_map[&p] == min_distance {
            stack.push(p);
        }
        let p = Position {
            x: end_pos.0,
            y: end_pos.1,
            direction: Direction::SOUTH,
        };
        if min_scores_map[&p] == min_distance {
            stack.push(p);
        }

        while stack.len() > 0 {
            let cur = stack.pop().unwrap();
            visited_tiles.insert((cur.x, cur.y));
            match cur.direction {
                Direction::EAST => {
                    let prev = Position {
                        x: cur.x,
                        y: cur.y - 1,
                        direction: Direction::EAST,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + MOVE_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::NORTH,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::SOUTH,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                }
                Direction::WEST => {
                    let prev = Position {
                        x: cur.x,
                        y: cur.y + 1,
                        direction: Direction::WEST,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + MOVE_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::NORTH,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::SOUTH,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                }
                Direction::NORTH => {
                    let prev = Position {
                        x: cur.x + 1,
                        y: cur.y,
                        direction: Direction::NORTH,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + MOVE_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::WEST,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::EAST,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                }
                Direction::SOUTH => {
                    let prev = Position {
                        x: cur.x - 1,
                        y: cur.y,
                        direction: Direction::SOUTH,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + MOVE_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::WEST,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                    let prev = Position {
                        x: cur.x,
                        y: cur.y,
                        direction: Direction::EAST,
                    };
                    if min_scores_map.contains_key(&prev)
                        && min_scores_map[&prev] + TURN_SCORE == min_scores_map[&cur]
                    {
                        stack.push(prev);
                    }
                }
            }
        }
        for tile in visited_tiles.iter() {
            map[tile.0][tile.1] = 'O';
        }
        visited_tiles.len()
    }

    fn calc_neighborhood_distances(
        map: &Vec<Vec<char>>,
        min_scores_map: &mut HashMap<Position, usize>,
        min_scores_heap: &mut BinaryHeap<Reverse<Distance>>,
        cur: &Position,
    ) {
        let (x, y, direction) = (cur.x, cur.y, cur.direction);
        let distance = min_scores_map[&cur];
        match direction {
            Direction::EAST => {
                if map[x][y + 1] != '#' {
                    let p = Position {
                        x: x,
                        y: y + 1,
                        direction: Direction::EAST,
                    };
                    if distance + MOVE_SCORE < min_scores_map[&p] {
                        *min_scores_map.get_mut(&p).unwrap() = distance + MOVE_SCORE;
                        min_scores_heap.push(Reverse(Distance {
                            position: p.clone(),
                            distance: min_scores_map[&p],
                        }));
                    }
                }
                //
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::NORTH,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::SOUTH,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
            }

            Direction::WEST => {
                if map[x][y - 1] != '#' {
                    let p = Position {
                        x: x,
                        y: y - 1,
                        direction: Direction::WEST,
                    };
                    if distance + MOVE_SCORE < min_scores_map[&p] {
                        *min_scores_map.get_mut(&p).unwrap() = distance + MOVE_SCORE;
                        min_scores_heap.push(Reverse(Distance {
                            position: p.clone(),
                            distance: min_scores_map[&p],
                        }));
                    }
                }
                //
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::NORTH,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::SOUTH,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
            }
            Direction::NORTH => {
                if map[x - 1][y] != '#' {
                    let p = Position {
                        x: x - 1,
                        y: y,
                        direction: Direction::NORTH,
                    };
                    if distance + MOVE_SCORE < min_scores_map[&p] {
                        *min_scores_map.get_mut(&p).unwrap() = distance + MOVE_SCORE;
                        min_scores_heap.push(Reverse(Distance {
                            position: p.clone(),
                            distance: min_scores_map[&p],
                        }));
                    }
                }
                //
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::WEST,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::EAST,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
            }
            Direction::SOUTH => {
                if map[x + 1][y] != '#' {
                    let p = Position {
                        x: x + 1,
                        y: y,
                        direction: Direction::SOUTH,
                    };
                    if distance + MOVE_SCORE < min_scores_map[&p] {
                        *min_scores_map.get_mut(&p).unwrap() = distance + MOVE_SCORE;
                        min_scores_heap.push(Reverse(Distance {
                            position: p.clone(),
                            distance: min_scores_map[&p],
                        }));
                    }
                }
                //
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::WEST,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
                let p = Position {
                    x: x,
                    y: y,
                    direction: Direction::EAST,
                };
                if distance + TURN_SCORE < min_scores_map[&p] {
                    *min_scores_map.get_mut(&p).unwrap() = distance + TURN_SCORE;
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: min_scores_map[&p],
                    }));
                }
            }
        }
    }

    fn create_min_scores(
        map: &Vec<Vec<char>>,
    ) -> (HashMap<Position, usize>, BinaryHeap<Reverse<Distance>>) {
        let mut min_scores_heap: BinaryHeap<Reverse<Distance>> = BinaryHeap::new();
        let mut min_scores_map: HashMap<Position, usize> = HashMap::new();
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                if map[x][y] == '#' {
                    continue;
                }
                if map[x][y] == 'S' {
                    let p = Position {
                        x,
                        y,
                        direction: Direction::EAST,
                    };
                    min_scores_heap.push(Reverse(Distance {
                        position: p.clone(),
                        distance: 0,
                    }));
                    min_scores_map.insert(p.clone(), 0);
                } else {
                    let p = Position {
                        x,
                        y,
                        direction: Direction::EAST,
                    };
                    // min_scores_heap.push(Reverse(Distance {
                    //     position: p.clone(),
                    //     distance: usize::MAX,
                    // }));
                    min_scores_map.insert(p.clone(), usize::MAX);
                }
                let p = Position {
                    x,
                    y,
                    direction: Direction::WEST,
                };
                // min_scores_heap.push(Reverse(Distance {
                //     position: p.clone(),
                //     distance: usize::MAX,
                // }));
                min_scores_map.insert(p.clone(), usize::MAX);
                //
                let p = Position {
                    x,
                    y,
                    direction: Direction::NORTH,
                };
                // min_scores_heap.push(Reverse(Distance {
                //     position: p.clone(),
                //     distance: usize::MAX,
                // }));
                min_scores_map.insert(p.clone(), usize::MAX);
                //
                let p = Position {
                    x,
                    y,
                    direction: Direction::SOUTH,
                };
                // min_scores_heap.push(Reverse(Distance {
                //     position: p.clone(),
                //     distance: usize::MAX,
                // }));
                min_scores_map.insert(p.clone(), usize::MAX);
            }
        }
        (min_scores_map, min_scores_heap)
    }

    fn read_input<R: BufRead>(reader: R) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
        let mut map = Vec::new();
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        let mut x = 0;
        let mut y;
        for result in reader.lines() {
            let line = result.unwrap().trim().to_string();
            let mut chars_vec = Vec::<char>::new();
            y = 0;
            for c in line.chars() {
                chars_vec.push(c);
                if c == 'S' {
                    start_pos = (x, y);
                }
                if c == 'E' {
                    end_pos = (x, y);
                }
                y += 1;
            }
            x += 1;
            map.push(chars_vec);
        }
        (map, start_pos, end_pos)
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
