use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::stdin;
use std::io::{BufRead, BufReader};

const DAY: &str = "14"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const SIMULATION_SECONDS: u16 = 100;
const ACTUAL_SIMULATION_SPACE: (u64, u64) = (101, 103);
const TEST_SIMULATION_SPACE: (u64, u64) = (11, 7);
const TEST_SIMULATION: bool = false;
const SIMULATION_SPACE: (u64, u64) = if TEST_SIMULATION {
    TEST_SIMULATION_SPACE
} else {
    ACTUAL_SIMULATION_SPACE
};

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";


fn main() -> Result<()> {
    start_day(DAY);

    #[derive(Copy, Clone)]
    struct Robot {
        x: u64,
        y: u64,
        vx: i64,
        vy: i64,
    }

    impl fmt::Display for Robot {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "x: {}, y: {}, vx: {}, vy: {}",
                self.x, self.y, self.vx, self.vy
            )
        }
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        let mut robots = read_input(reader);
        //print_robots(&robots);
        let space = calc_space(&robots);
        //print_space(&space);
        run_simulation(&mut robots, SIMULATION_SECONDS);

        //print_robots(&robots);
        let space = calc_space(&robots);
        print_space(&space);

        answer = calc_safety_factor(&space);
        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    // assert_eq!(12, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        let mut robots = read_input(reader);
        for i in (1..10000) {
            
            let mut robots_clones = robots.clone();
            run_simulation(&mut robots_clones, i);

            let space = calc_space(&robots_clones);
            if test_for_line_segment(&space) {
                answer = i;
                print_space(&space);
            }
        }

        Ok(answer as usize)
    }

    //assert

    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result: usize = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    fn read_input<R: BufRead>(reader: R) -> Vec<Robot> {
        let re = Regex::new(r"^p=(\d+),(\d+) v=([-]?\d+),([-]?\d+)$").unwrap();
        let mut robots: Vec<Robot> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            for (_, [x, y, vx, vy]) in re.captures_iter(&line).map(|c| c.extract()) {
                robots.push(Robot {
                    x: x.parse::<u64>().unwrap(),
                    y: y.parse::<u64>().unwrap(),
                    vx: vx.parse::<i64>().unwrap(),
                    vy: vy.parse::<i64>().unwrap(),
                });
            }
        }
        robots
    }

    fn run_simulation(robots: &mut Vec<Robot>, seconds: u16) {
        for mut robot in robots {
            move_robot(&mut robot, seconds, SIMULATION_SPACE);
        }
    }

    fn move_robot(robot: &mut Robot, seconds: u16, space: (u64, u64)) {
        let delta_x = (robot.vx * (seconds as i64)) % (space.0 as i64);
        let delta_y = robot.vy * (seconds as i64) % (space.1 as i64);
        let old_x = robot.x as i64;
        let old_y = robot.y as i64;
        let mut new_x = old_x + delta_x;
        let mut new_y = old_y + delta_y;
        if new_x < 0 {
            new_x = (space.0 as i64) - new_x.abs();
        }
        if new_x >= space.0 as i64 {
            new_x = new_x - space.0 as i64;
        }
        if new_y < 0 {
            new_y = (space.1 as i64) - new_y.abs();
        }
        if new_y >= space.1 as i64 {
            new_y = new_y - space.1 as i64;
        }
        robot.x = new_x as u64;
        robot.y = new_y as u64;
    }

    fn calc_safety_factor(space: &Vec<Vec<u64>>) -> u64 {
        let mut quadrants: [u64; 4] = [0; 4];

        for x in 0..SIMULATION_SPACE.0 {
            for y in 0..SIMULATION_SPACE.1 {
                if x < SIMULATION_SPACE.0 / 2 && y < SIMULATION_SPACE.1 / 2 {
                    quadrants[0] += space[y as usize][x as usize];
                }
                if x < SIMULATION_SPACE.0 / 2 && y > SIMULATION_SPACE.1 / 2 {
                    quadrants[1] += space[y as usize][x as usize];
                }
                if x > SIMULATION_SPACE.0 / 2 && y < SIMULATION_SPACE.1 / 2 {
                    quadrants[2] += space[y as usize][x as usize];
                }
                if x > SIMULATION_SPACE.0 / 2 && y > SIMULATION_SPACE.1 / 2 {
                    quadrants[3] += space[y as usize][x as usize];
                }
            }
        }
        quadrants.iter().fold(1, |acc, x| acc * x)
    }

    fn test_for_line_segment(space: &Vec<Vec<u64>>) -> bool {
        for x in 0..SIMULATION_SPACE.0 {
            for y in 0..SIMULATION_SPACE.1 - 15 {
                if space[x as usize][y as usize..y as usize+14].into_iter().all(|c| *c > 0) {
                    return true;
                }
            }
        }
        false
    }

    fn calc_space(robots: &Vec<Robot>) -> Vec<Vec<u64>> {
        let mut space = vec![vec![0; SIMULATION_SPACE.0 as usize]; SIMULATION_SPACE.1 as usize];
        for robot in robots {
            space[robot.y as usize][robot.x as usize] += 1;
        }
        space
    }

    fn print_space(space: &Vec<Vec<u64>>) {
        println!("");
        for row in space {
            for i in row {
                if *i == 0 {
                    print!(".");
                } else {
                    print!("{}", i);
                }
            }
            println!();
        }
    }

    fn print_robots(robots: &Vec<Robot>) {
        println!("");
        for robot in robots {
            println!("{}", robot);
        }
    }

    Ok(())
}
