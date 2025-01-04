use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "17"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const TEST_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let (registers, program) = read_input(reader);
        let outputs = run_program(&program, registers);

        Ok(outputs
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        /*
        Program: 2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0

        0: (2,4) : B = A MOD 8
        2:  (1,2) : B = B XOR 2
        4:  (7,5) : C = A >> B
        6:  (4,5) : B = B XOR C
        8:  (1,3) : B = B XOR 3
        10: (5,5) : output B MOD 8
        12: (0,3) : A = A >> 3;
        14: (3,3) : if A > 0 GOTO 0
         */

        let (_, program) = read_input(reader);

        let answer: u64 = 0;
        let a = find_a(answer, &program);

        println!("A={:?}", a.unwrap());

        Ok(a.unwrap() as usize)
    }

    //assert_eq!(117440, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    fn find_a(a_reg: u64, program: &Vec<u8>) -> Option<u64> {
        if program.is_empty() {
            return Some(a_reg);
        }
        for d in 0..8 {
            let a = (a_reg << 3) + d;
            let mut b = a % 8;
            b = b ^ 2;
            let c = a >> b;
            b = b ^ c;
            b = b ^ 3;
            if b % 8 == program[program.len() - 1] as u64 {
                let mut p = program.clone();
                p.pop();
                let t = find_a(a, &p);
                match t {
                    None => continue,
                    Some(s) => return Some(s),
                }
            }
        }
        None
    }

    fn read_input<R: BufRead>(mut reader: R) -> (HashMap<char, u64>, Vec<u8>) {
        let mut registers: HashMap<char, u64> = HashMap::new();
        let mut program: Vec<u8> = Vec::new();

        for result in reader.lines() {
            let line = result.unwrap().trim().to_string();
            if line.starts_with("Register") {
                let words: Vec<&str> = line.trim().split(" ").collect();
                registers.insert(
                    words[1].chars().nth(0).unwrap(),
                    words[2].parse::<u64>().unwrap(),
                );
            } else if line.starts_with("Program") {
                let words: Vec<&str> = line.trim().split(" ").collect();
                program = words[1]
                    .split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
            }
        }
        (registers, program)
    }

    fn run_program(program: &Vec<u8>, registers: HashMap<char, u64>) -> Vec<u8> {
        let mut outputs = Vec::new();
        let mut registers = registers.clone();
        let mut instruction_pointer = 0;
        while instruction_pointer < program.len() {
            let (opcode, operand) = (
                program[instruction_pointer],
                program[instruction_pointer + 1],
            );
            let literal_operand = operand as u64;
            let combo_operand = match operand {
                0 | 1 | 2 | 3 => operand as u64,
                4 => registers[&'A'],
                5 => registers[&'B'],
                6 => registers[&'C'],
                _ => 0,
            };

            match opcode {
                0 => {
                    // adv
                    registers.insert('A', registers[&'A'] >> combo_operand);
                }
                1 => {
                    // bxl
                    registers.insert('B', registers[&'B'] ^ literal_operand);
                }
                2 => {
                    // bst
                    registers.insert('B', combo_operand & 7);
                }
                3 => {
                    // jnz
                    if registers[&'A'] != 0 {
                        instruction_pointer = literal_operand as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc
                    registers.insert('B', registers[&'B'] ^ registers[&'C']);
                }
                5 => {
                    // out
                    outputs.push((combo_operand & 7) as u8);
                }
                6 => {
                    // bdv
                    registers.insert('B', registers[&'A'] >> combo_operand);
                }
                7 => {
                    // cdv
                    registers.insert('C', registers[&'A'] >> combo_operand);
                }
                _ => {}
            }
            instruction_pointer += 2;
        }
        outputs
    }

    Ok(())
}
