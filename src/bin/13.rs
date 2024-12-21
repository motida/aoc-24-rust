use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const BUTTON_A_COST: u128 = 3;
const BUTTON_B_COST: u128 = 1;

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
fn main() -> Result<()> {
    start_day(DAY);

    const SIZE: usize = 100;

    struct Button {
        x: u128,
        y: u128,
    }
    struct Prize {
        x: u128,
        y: u128,
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let machines = read_input(reader);
 
        for matchine in machines {
            let (button_a, button_b, prize) = matchine;
            let (a, b, e) = (button_a.x, button_b.x, prize.x);
            let (c, d, f) = (button_a.y, button_b.y, prize.y);
            let solution = solve_equation(a, b, c, d, e, f);

            answer += match solution {
                Some((x, y)) => x * BUTTON_A_COST + y * BUTTON_B_COST,
                None => 0,
            }
        }
        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // //endregion

    // //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let machines = read_input(reader);

        for matchine in machines {
            let (button_a, button_b, prize) = matchine;
            // part-2
            let prize = Prize {
                x: prize.x + 10000000000000,
                y: prize.y + 10000000000000,
            };
            //
            let (a, b, e) = (button_a.x, button_b.x, prize.x);
            let (c, d, f) = (button_a.y, button_b.y, prize.y);
            let solution = solve_equation(a, b, c, d, e, f);

            answer += match solution {
                Some((x, y)) => x * BUTTON_A_COST + y * BUTTON_B_COST,
                None => 0,
            }
        }
        Ok(answer as usize)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    fn solve_equation(
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        e: u128,
        f: u128,
    ) -> Option<(u128, u128)> {
        // a + b = e and c + d = f
        let det: i128 = (a as i128) * (d as i128) - (b as i128) * (c as i128);
        if det == 0 {
            None
        } else {
            let x: i128;
            let y: i128;
            let x_numerator = (d as i128) * (e as i128) - (b as i128) * (f as i128);
            let y_numerator = (a as i128) * (f as i128) - (c as i128) * (e as i128);
            if x_numerator % det == 0 && y_numerator % det == 0 {
                x = x_numerator / det;
                y = y_numerator / det;
                if x >= 0 && y >= 0 {
                    return Some((x as u128, y as u128));
                }
                return None;
            } else {
                return None;
            }
        }
    }

    fn read_input<R: BufRead>(mut reader: R) -> Vec<(Button, Button, Prize)> {
        let button_a_re = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
        let button_b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)$").unwrap();
        let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let mut machines: Vec<(Button, Button, Prize)> = Vec::new();

        loop {
            let mut button_a: Button = Button { x: 0, y: 0 };
            let mut button_b: Button = Button { x: 0, y: 0 };
            let mut prize: Prize = Prize { x: 0, y: 0 };
            let mut line = String::new();
            let _ = reader.read_line(&mut line);
            line = line.trim().to_string();

            for (_, [x, y]) in button_a_re.captures_iter(&line).map(|c| c.extract()) {
                button_a = Button {
                    x: x.parse::<u128>().unwrap(),
                    y: y.parse::<u128>().unwrap(),
                };
            }

            let mut line = String::new();
            let _ = reader.read_line(&mut line);
            line = line.trim().to_string();

            for (_, [x, y]) in button_b_re.captures_iter(&line).map(|c| c.extract()) {
                button_b = Button {
                    x: x.parse::<u128>().unwrap(),
                    y: y.parse::<u128>().unwrap(),
                };
            }

            let mut line = String::new();
            let _ = reader.read_line(&mut line);
            line = line.trim().to_string();

            for (_, [x, y]) in prize_re.captures_iter(&line).map(|c| c.extract()) {
                prize = Prize {
                    x: x.parse::<u128>().unwrap(),
                    y: y.parse::<u128>().unwrap(),
                };
            }

            machines.push((button_a, button_b, prize));
            let mut line = String::new();
            line = line.trim().to_string();
            let result = reader.read_line(&mut line);
            if result.is_err() || result.unwrap() == 0 {
                break;
            }
        }
        machines
    }

    Ok(())
}
