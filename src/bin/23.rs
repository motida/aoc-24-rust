use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "23"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let pairs = read_input(reader);
        let nodes = get_nodes(&pairs);
        let nodes_starts_with_t: HashSet<String> =
            HashSet::from_iter(nodes.clone().into_iter().filter(|x| x.starts_with("t")));
        let edges = get_edges(&pairs);
        let triplets = calc_triplets(&nodes, &edges, &nodes_starts_with_t);
        Ok(triplets)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let pairs = read_input(reader);
        let nodes = get_nodes(&pairs);
        let edges = get_edges(&pairs);
        let mut max_clique = calc_max_cliques(&nodes, &edges);
        max_clique.sort();
        let joined = max_clique.join(",");

        Ok(joined)
    }

    assert_eq!("co,de,ka,ta", part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn get_edges(pairs: &[(String, String)]) -> HashSet<(String, String)> {
    let mut edges = HashSet::new();
    for pair in pairs {
        edges.insert((pair.0.clone(), pair.1.clone()));
        edges.insert((pair.1.clone(), pair.0.clone()));
    }
    edges
}

fn get_nodes(pairs: &Vec<(String, String)>) -> Vec<String> {
    let mut nodes = Vec::new();
    for pair in pairs {
        nodes.push(pair.0.clone());
        nodes.push(pair.1.clone());
    }
    nodes.sort();
    nodes.dedup();
    nodes
}

fn calc_triplets(
    nodes: &Vec<String>,
    edges: &HashSet<(String, String)>,
    nodes_starts_with_t: &HashSet<String>,
) -> usize {
    let mut triplets = 0;
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                if edges.contains(&(nodes[i].clone(), nodes[j].clone()))
                    && edges.contains(&(nodes[j].clone(), nodes[k].clone()))
                    && edges.contains(&(nodes[i].clone(), nodes[k].clone()))
                    && (nodes_starts_with_t.contains(&nodes[i])
                        || nodes_starts_with_t.contains(&nodes[j])
                        || nodes_starts_with_t.contains(&nodes[k]))
                {
                    triplets += 1;
                }
            }
        }
    }
    triplets
}

fn calc_max_cliques(nodes: &Vec<String>, edges: &HashSet<(String, String)>) -> Vec<String> {
    let mut clique = Vec::<String>::new();
    let mut max_clique_size = 0;
    let mut max_clique = Vec::<String>::new();
    for start_node in nodes {
        clique.clear();
        clique.push(start_node.to_string());
        for node in nodes {
            if node == start_node {
                continue;
            }
            let mut add_to_clique = true;
            for clique_node in &clique {
                if clique_node == node {
                    continue;
                }
                if !edges.contains(&(clique_node.clone(), node.clone())) {
                    add_to_clique = false;
                    break;
                }
            }
            if add_to_clique {
                clique.push(node.to_string());
            }
        }
        if clique.len() > max_clique_size {
            max_clique_size = clique.len();
            max_clique = clique.clone();
        }
    }
    max_clique
}

fn read_input<R: BufRead>(reader: R) -> Vec<(String, String)> {
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let nodes: Vec<&str> = line.split("-").collect();
        pairs.push((nodes[0].to_string(), nodes[1].to_string()));
    }
    pairs
}
