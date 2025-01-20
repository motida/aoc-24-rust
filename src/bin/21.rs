use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "21"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

/* 

+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

*/

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer: usize = 0;
        let mut codes_summary: Vec<(usize, usize)> = Vec::new();
        let codes = read_input(reader);
        //println!("codes: {:?}", codes);
        let numeric_keypad_moves = calc_numeric_keypad_moves();
        //println!("numeric_keypad_moves: {:?}", numeric_keypad_moves);
        let directional_keypad_moves = calc_directional_keypad_moves();
        //println!("directional_keypad_moves: {:?}", directional_keypad_moves);

        for code in codes {
            //let code = "0".to_string();
            let min_path_len: usize = calc_code_presses(
                code.clone(),
                &numeric_keypad_moves,
                &directional_keypad_moves,
                2,
            );
            let code_number = code_to_number(code.clone());
            codes_summary.push((code_number, min_path_len));
        }
        //println!("codes_summary: {:?}", codes_summary);

        for (code_number, presses) in codes_summary {
            answer += code_number * presses;
        }
        Ok(answer)
    }

    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer: usize = 0;
        let mut codes_summary: Vec<(usize, usize)> = Vec::new();
        let codes = read_input(reader);
        //println!("codes: {:?}", codes);
        let numeric_keypad_moves = calc_numeric_keypad_moves();
        //println!("numeric_keypad_moves: {:?}", numeric_keypad_moves);
        let directional_keypad_moves = calc_directional_keypad_moves();
        //println!("directional_keypad_moves: {:?}", directional_keypad_moves);

        for code in codes {
            //let code = "0".to_string();
            let min_path_len: usize = calc_code_presses(
                code.clone(),
                &numeric_keypad_moves,
                &directional_keypad_moves,
                25,
            );
            let code_number = code_to_number(code.clone());
            codes_summary.push((code_number, min_path_len));
        }
        //println!("codes_summary: {:?}", codes_summary);

        for (code_number, presses) in codes_summary {
            answer += code_number * presses;
        }
        Ok(answer)
    }

    assert_eq!(154115708116294, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn code_to_number(code: String) -> usize {
    code[0..code.len() - 1].parse::<usize>().unwrap()
}

fn calc_code_presses(
    code: String,
    numeric_keypad_moves: &HashMap<(char, char), HashSet<String>>,
    directional_keypad_moves: &HashMap<(char, char), HashSet<String>>,
    depth: usize,
) -> usize {
    let code = format!("A{}", code);
    let mut num_presses = 0;
    for i in 0..code.len() - 1 {
        let c1 = code.chars().nth(i).unwrap();
        let c2 = code.chars().nth(i + 1).unwrap();
        let n = calc_keypad_presses_recursive(
            c1,
            c2,
            depth,
            true,
            &numeric_keypad_moves,
            &directional_keypad_moves,
            &mut HashMap::new(),
        );
        num_presses += n
    }
    num_presses
}
fn calc_keypad_presses_recursive(
    c1: char,
    c2: char,
    depth: usize,
    is_numeric: bool,
    numeric_keypad_moves: &HashMap<(char, char), HashSet<String>>,
    directional_keypad_moves: &HashMap<(char, char), HashSet<String>>,
    cache: &mut HashMap<(char, char, usize, bool), usize>,
) -> usize {
    let keypad_moves = if is_numeric {
        numeric_keypad_moves
    } else {
        directional_keypad_moves
    };

    if depth == 0 {
        return *(&keypad_moves[&(c1, c2)].iter().nth(0).unwrap().len()) + 1;
    }
    let mut num_presses = usize::MAX;

    for moves in &keypad_moves[&(c1, c2)] {
        let mut temp_num_presses = 0;
        let m = format!("A{}A", moves);
        for i in 0..m.len() - 1 {
            if cache.contains_key(&(
                m.chars().nth(i).unwrap(),
                m.chars().nth(i + 1).unwrap(),
                depth - 1,
                false,
            )) {
                temp_num_presses += cache
                    .get(&(
                        m.chars().nth(i).unwrap(),
                        m.chars().nth(i + 1).unwrap(),
                        depth - 1,
                        false,
                    ))
                    .unwrap();
            } else {
                temp_num_presses += calc_keypad_presses_recursive(
                    m.chars().nth(i).unwrap(),
                    m.chars().nth(i + 1).unwrap(),
                    depth - 1,
                    false,
                    numeric_keypad_moves,
                    directional_keypad_moves,
                    cache,
                )
            };
        }
        if temp_num_presses < num_presses {
            num_presses = temp_num_presses;
        }
    }
    cache.insert((c1, c2, depth, is_numeric), num_presses);
    num_presses
}

fn calc_numeric_keypad_moves() -> HashMap<(char, char), HashSet<String>> {
    let keys = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
    let mut moves: HashMap<(char, char), HashSet<String>> = HashMap::from([
        (('A', '0'), HashSet::from(["<".to_string()])),
        (('A', '3'), HashSet::from(["^".to_string()])),
        (('0', '2'), HashSet::from(["^".to_string()])),
        (('0', 'A'), HashSet::from([">".to_string()])),
        (('3', 'A'), HashSet::from(["v".to_string()])),
        (('3', '6'), HashSet::from(["^".to_string()])),
        (('3', '2'), HashSet::from(["<".to_string()])),
        (('2', '0'), HashSet::from(["v".to_string()])),
        (('2', '1'), HashSet::from(["<".to_string()])),
        (('2', '5'), HashSet::from(["^".to_string()])),
        (('2', '3'), HashSet::from([">".to_string()])),
        (('1', '2'), HashSet::from([">".to_string()])),
        (('1', '4'), HashSet::from(["^".to_string()])),
        (('4', '1'), HashSet::from(["v".to_string()])),
        (('4', '7'), HashSet::from(["^".to_string()])),
        (('4', '5'), HashSet::from([">".to_string()])),
        (('5', '2'), HashSet::from(["v".to_string()])),
        (('5', '4'), HashSet::from(["<".to_string()])),
        (('5', '8'), HashSet::from(["^".to_string()])),
        (('5', '6'), HashSet::from([">".to_string()])),
        (('6', '3'), HashSet::from(["v".to_string()])),
        (('6', '5'), HashSet::from(["<".to_string()])),
        (('6', '9'), HashSet::from(["^".to_string()])),
        (('7', '4'), HashSet::from(["v".to_string()])),
        (('7', '8'), HashSet::from([">".to_string()])),
        (('8', '5'), HashSet::from(["v".to_string()])),
        (('8', '7'), HashSet::from(["<".to_string()])),
        (('8', '9'), HashSet::from([">".to_string()])),
        (('9', '6'), HashSet::from(["v".to_string()])),
        (('9', '8'), HashSet::from(["<".to_string()])),
    ]);
    for key in keys.clone() {
        moves.insert((key, key), HashSet::from(["".to_string()]));
    }
    for i in keys.clone() {
        for j in keys.clone() {
            if moves.contains_key(&(i, j)) {
                continue;
            } else {
                moves.insert((i, j), HashSet::from(["..........".to_string()]));
            }
        }
    }
    for k in keys.clone() {
        for i in keys.clone() {
            for j in keys.clone() {
                if moves[&(i, j)].iter().nth(0).unwrap().len()
                    > moves[&(i, k)].iter().nth(0).unwrap().len()
                        + moves[&(k, j)].iter().nth(0).unwrap().len()
                {
                    moves.get_mut(&(i, j)).unwrap().clear();
                    for x in moves[&(i, k)].clone() {
                        for y in moves[&(k, j)].clone() {
                            moves
                                .get_mut(&(i, j))
                                .unwrap()
                                .insert(format!("{}{}", x, y));
                        }
                    }
                }
                if moves[&(i, j)].iter().nth(0).unwrap().len()
                    == moves[&(i, k)].iter().nth(0).unwrap().len()
                        + moves[&(k, j)].iter().nth(0).unwrap().len()
                {
                    for x in moves[&(i, k)].clone() {
                        for y in moves[&(k, j)].clone() {
                            moves
                                .get_mut(&(i, j))
                                .unwrap()
                                .insert(format!("{}{}", x, y));
                        }
                    }
                }
            }
        }
    }
    moves
}

fn calc_directional_keypad_moves() -> HashMap<(char, char), HashSet<String>> {
    let keys = vec!['^', 'v', '<', '>', 'A'];
    let mut moves: HashMap<(char, char), HashSet<String>> = HashMap::from([
        (('A', '^'), HashSet::from(["<".to_string()])),
        (('A', '>'), HashSet::from(["v".to_string()])),
        (('^', 'v'), HashSet::from(["v".to_string()])),
        (('^', 'A'), HashSet::from([">".to_string()])),
        (('v', '^'), HashSet::from(["^".to_string()])),
        (('v', '<'), HashSet::from(["<".to_string()])),
        (('v', '>'), HashSet::from([">".to_string()])),
        (('>', 'v'), HashSet::from(["<".to_string()])),
        (('>', 'A'), HashSet::from(["^".to_string()])),
        (('<', 'v'), HashSet::from([">".to_string()])),
    ]);
    for key in keys.clone() {
        moves.insert((key, key), HashSet::from(["".to_string()]));
    }
    for i in keys.clone() {
        for j in keys.clone() {
            if moves.contains_key(&(i, j)) {
                continue;
            } else {
                moves.insert((i, j), HashSet::from(["..........".to_string()]));
            }
        }
    }
    for k in keys.clone() {
        for i in keys.clone() {
            for j in keys.clone() {
                if moves[&(i, j)].iter().nth(0).unwrap().len()
                    > moves[&(i, k)].iter().nth(0).unwrap().len()
                        + moves[&(k, j)].iter().nth(0).unwrap().len()
                {
                    moves.get_mut(&(i, j)).unwrap().clear();
                    for x in moves[&(i, k)].clone() {
                        for y in moves[&(k, j)].clone() {
                            moves
                                .get_mut(&(i, j))
                                .unwrap()
                                .insert(format!("{}{}", x, y));
                        }
                    }
                }
                if moves[&(i, j)].iter().nth(0).unwrap().len()
                    == moves[&(i, k)].iter().nth(0).unwrap().len()
                        + moves[&(k, j)].iter().nth(0).unwrap().len()
                {
                    for x in moves[&(i, k)].clone() {
                        for y in moves[&(k, j)].clone() {
                            moves
                                .get_mut(&(i, j))
                                .unwrap()
                                .insert(format!("{}{}", x, y));
                        }
                    }
                }
            }
        }
    }
    moves
}

fn read_input<R: BufRead>(reader: R) -> Vec<String> {
    let codes = reader.lines().flatten().collect::<Vec<String>>();
    codes
}
