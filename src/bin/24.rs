use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "24"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST2: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

#[derive(Debug, Clone, PartialEq, Eq)]
enum GATE {
    AND,
    OR,
    XOR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    in1: String,
    in2: String,
    gate: GATE,
    out: String,
}
impl Gate {
    fn activate(&self, wires: &mut HashMap<String, Option<bool>>) -> bool {
        if wires[self.in1.as_str()].is_some()
            && wires[self.in2.as_str()].is_some()
            && wires[self.out.as_str()].is_none()
        {
            wires.insert(
                self.out.clone(),
                match self.gate {
                    GATE::AND => Some(wires[&self.in1].unwrap() && wires[&self.in2].unwrap()),
                    GATE::OR => Some(wires[&self.in1].unwrap() || wires[&self.in2].unwrap()),
                    GATE::XOR => Some(wires[&self.in1].unwrap() ^ wires[&self.in2].unwrap()),
                },
            );
            true
        } else {
            false
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let (wires, gates, _, _) = read_input(reader);
        let output = calculate(gates.clone(), wires.clone());
        Ok(output)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let (wires, gates, _, _) = read_input(reader);
        let (full_adder, mut full_adder_wires, _, _) = gen_full_adder();
        for w in wires.iter().filter(|(k, v)| v.is_some()) {
            full_adder_wires.insert(w.0.clone(), w.1.clone());
        }
        gen_dot_file(&full_adder);
        gen_dot_file(&gates);
        // used graphviz to find wrong wiring
        //
        let (input1, input2) = calc_input(&wires);
        let output = calculate(full_adder.clone(), full_adder_wires.clone());
        println!("{} + {} = {}", input1, input2, output);

        let swaps: [(String, String); 4] = [
            ("ckj".to_string(), "z15".to_string()),
            ("kdf".to_string(), "z23".to_string()),
            ("rpp".to_string(), "z39".to_string()),
            ("dbp".to_string(), "fdv".to_string()),
        ];
        let mut gates = gates.clone();
        for gate in gates.iter_mut() {
            for swap in &swaps {
                if gate.out == swap.0 {
                    gate.out = swap.1.clone();
                } else if gate.out == swap.1 {
                    gate.out = swap.0.clone();
                }
            }
        }

        let output = calculate(gates.clone(), wires.clone());
        println!("{} + {} = {}", input1, input2, output);

        let mut answer = Vec::<String>::new();
        for swap in swaps {
            answer.push(swap.0);
            answer.push(swap.1);
        }
        answer.sort();
        Ok(answer.join(","))
    }

    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn gen_dot_file(gates: &Vec<Gate>) {
    let mut dot = String::new();
    dot.push_str("digraph G {\n");
    for gate in gates {
        let color = match gate.gate {
            GATE::AND => "red",
            GATE::OR => "green",
            GATE::XOR => "blue",
        };

        dot.push_str(&format!(
            "{} -> {} [color={}];\n",
            gate.in1, gate.out, color
        ));
        dot.push_str(&format!(
            "{} -> {} [color={}];\n",
            gate.in2, gate.out, color
        ));
    }
    dot.push_str("}");
    println!("{}", dot);
}

fn gen_full_adder() -> (
    Vec<Gate>,
    HashMap<String, Option<bool>>,
    HashMap<String, Gate>,
    HashMap<String, Gate>,
) {
    let mut gates = Vec::<Gate>::new();
    let mut gates_lookup_by_output = HashMap::<String, Gate>::new();
    let mut gates_lookup_by_input = HashMap::<String, Gate>::new();
    let mut wires_set: HashSet<String> = HashSet::new();
    let mut wires = HashMap::<String, Option<bool>>::new();
    let z00 = Gate {
        in1: "x00".to_string(),
        in2: "y00".to_string(),
        gate: GATE::XOR,
        out: "z00".to_string(),
    };
    gates.push(z00);
    let c00: Gate = Gate {
        in1: "x00".to_string(),
        in2: "y00".to_string(),
        gate: GATE::AND,
        out: "c00".to_string(),
    };
    gates.push(c00);
    let r01: Gate = Gate {
        in1: "x01".to_string(),
        in2: "y01".to_string(),
        gate: GATE::XOR,
        out: "r01".to_string(),
    };

    for i in 1..=44 {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        let z = format!("z{:02}", i);
        let r = format!("r{:02}", i);
        let prev_c = format!("c{:02}", i - 1);
        let c = format!("c{:02}", i);
        let a = format!("a{:02}", i);
        let b = format!("b{:02}", i);
        let g_r = Gate {
            in1: x.clone(),
            in2: y.clone(),
            gate: GATE::XOR,
            out: r.clone(),
        };
        gates.push(g_r);
        let g_z = Gate {
            in1: r.clone(),
            in2: prev_c.clone(),
            gate: GATE::XOR,
            out: z.clone(),
        };
        gates.push(g_z);
        let g_a: Gate = Gate {
            in1: x.clone(),
            in2: y.clone(),
            gate: GATE::AND,
            out: a.clone(),
        };
        gates.push(g_a);
        let g_b: Gate = Gate {
            in1: r.clone(),
            in2: prev_c.clone(),
            gate: GATE::AND,
            out: b.clone(),
        };
        gates.push(g_b);
        let g_c: Gate = Gate {
            in1: a.clone(),
            in2: b.clone(),
            gate: GATE::OR,
            out: c.clone(),
        };
        gates.push(g_c);
    }
    let mut g: Gate = gates.pop().unwrap();
    g.out = "z45".to_string();
    gates.push(g);

    for g in gates.iter() {
        gates_lookup_by_output.insert(g.out.clone(), g.clone());
        gates_lookup_by_input.insert(g.in1.clone(), g.clone());
        gates_lookup_by_input.insert(g.in2.clone(), g.clone());
        wires_set.insert(g.in1.clone());
        wires_set.insert(g.in2.clone());
        wires_set.insert(g.out.clone());
    }
    for wire in wires_set {
        wires.insert(wire, None);
    }

    (gates, wires, gates_lookup_by_output, gates_lookup_by_input)
}

fn calculate(gates: Vec<Gate>, mut wires: HashMap<String, Option<bool>>) -> usize {
    while gates
        .iter()
        .map(|x| x.activate(&mut wires))
        .fold(false, |acc, did_activate| acc || did_activate)
    {
        continue;
    }
    let output = calc_output(&wires);
    output
}

fn calc_input(wires: &HashMap<String, Option<bool>>) -> (usize, usize) {
    let max_z = wires.keys().max().unwrap()[1..].parse::<u8>().unwrap() - 1;
    let mut input1 = 0;
    let mut input2 = 0;
    for i in (0..=max_z).rev() {
        let digit1 = wires[format!("x{:02}", i).as_str()].unwrap();
        input1 = input1 << 1;
        input1 |= digit1 as usize;
        let digit2 = wires[format!("y{:02}", i).as_str()].unwrap();
        input2 = input2 << 1;
        input2 |= digit2 as usize;
    }
    (input1, input2)
}

fn calc_output(wires: &HashMap<String, Option<bool>>) -> usize {
    let max_z = wires.keys().max().unwrap()[1..].parse::<u8>().unwrap();
    let mut output = 0;
    for i in (0..=max_z).rev() {
        let digit = wires[format!("z{:02}", i).as_str()].unwrap();
        output = output << 1;
        output |= digit as usize;
    }
    output
}

fn read_input<R: BufRead>(
    reader: R,
) -> (
    HashMap<String, Option<bool>>,
    Vec<Gate>,
    HashMap<String, Gate>,
    HashMap<String, Gate>,
) {
    let mut wires = HashMap::<String, Option<bool>>::new();
    let mut gates = Vec::<Gate>::new();
    let mut gates_lookup_by_output = HashMap::<String, Gate>::new();
    let mut gates_lookup_by_input = HashMap::<String, Gate>::new();
    let lines = reader.lines().flatten().collect::<Vec<String>>();
    let mut index = 0;
    for i in 0..lines.len() {
        index += 1;
        let line = &lines[i];
        if line.is_empty() {
            break;
        }

        let pair: Vec<&str> = line.splitn(2, ": ").collect();
        wires.insert(
            pair[0].to_string(),
            Some(pair[1].parse::<u8>().unwrap() == 1),
        );
    }
    for i in index..lines.len() {
        let line = &lines[i];
        let quintet: Vec<&str> = line.splitn(5, " ").collect();
        let (in1, in2, gate, out) = (quintet[0], quintet[2], quintet[1], quintet[4]);

        wires.entry(in1.to_string()).or_insert(None);
        wires.entry(in2.to_string()).or_insert(None);
        wires.entry(out.to_string()).or_insert(None);
        let gate_type = match gate {
            "AND" => GATE::AND,
            "OR" => GATE::OR,
            "XOR" => GATE::XOR,
            _ => panic!(),
        };
        let gate = Gate {
            in1: in1.to_string(),
            in2: in2.to_string(),
            gate: gate_type,
            out: out.to_string(),
        };
        gates.push(gate.clone());
        gates_lookup_by_output.insert(out.to_string(), gate.clone());
        gates_lookup_by_input.insert(in1.to_string(), gate.clone());
        gates_lookup_by_input.insert(in2.to_string(), gate.clone());
    }
    (wires, gates, gates_lookup_by_output, gates_lookup_by_input)
}
