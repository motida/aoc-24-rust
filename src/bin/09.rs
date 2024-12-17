use anyhow::*;
use itertools::{any, chain};
use std::arch::is_aarch64_feature_detected;
use std::collections::{HashMap, HashSet};
use std::fs::{read, File};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::{Filter, Successors};
use std::mem::discriminant;
use std::net::AddrParseError;
use std::path::Ancestors;
use std::slice::ChunksExactMut;
use std::{isize, usize};

use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let checksum: usize = 0;
        let disk_map: String = read_input(reader);
        let mut disk: Vec<Option<usize>> = create_disk(disk_map);
        compact_disk(&mut disk);
        let checksum: usize = calculate_checksum(&disk);
        Ok(checksum)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2

    println!("=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let checksum: usize = 0;
        let disk_map: String = read_input(reader);
        let (mut disk, mut files, mut free_space, max_file_id) = create_disk_2(disk_map);
        compact_disk_2(&mut disk, &mut files, &mut free_space, max_file_id);
        let checksum: usize = calculate_checksum(&disk);
        Ok(checksum)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion
    Ok(())
}

fn compact_disk_2(disk: &mut Vec<Option<usize>>, files: &mut HashMap<usize, (usize, usize)>, free_space: &mut[(usize, usize)], max_file_id: usize) {
    let mut file_id = max_file_id;
    while file_id > 0 {
        let (mut file_address, mut file_size) = files.get(&file_id).unwrap();
        for (i, (mut free_space_address, mut free_space_size)) in free_space.iter().enumerate() {
            if free_space_address >= file_address {
                break;
            }
            if free_space_size >= file_size {
 
                move_file(disk, file_id, file_address, free_space_address, file_size);
                free_space[i] = (free_space_address + file_size, free_space_size - file_size);
                files.insert(file_id,(free_space_address, file_size));
                break;
            }
        }
        file_id -= 1;
    }
}

fn move_file(disk: &mut Vec<Option<usize>>, file_id: usize, file_address: usize, free_space_address: usize, file_size: usize) {
    for i in 0..file_size {
        disk[free_space_address + i] = disk[file_address + i]; 
        disk[file_address + i] = None;
    }
}

fn calculate_checksum(disk: &Vec<Option<usize>>) -> usize {
    let mut checksum: usize = 0;
    for (i, b) in disk.iter().enumerate() {
        if b.is_some() {
            checksum += i*b.unwrap();
        }
    }
    checksum
}

fn compact_disk(disk: &mut Vec<Option<usize>>) {
    let mut left: usize = 0;
    let mut right = disk.len() - 1;
    loop {
        left = find_next_free_space(disk, left);
        right = find_next_block(disk, right);
        if left >= right {
            break;
        }
        disk[left] = disk[right];
        disk[right] = None;
        left += 1;
        right -= 1;
    }
}

fn find_next_free_space(disk: &Vec<Option<usize>>, left: usize) -> usize {
    let mut next = left;
    while next < disk.len() && disk[next].is_some()  {
        next += 1;
    }
    next
}

fn find_next_block(disk: &Vec<Option<usize>>, right: usize) -> usize {
    let mut next = right;
    while right >= 0 &&disk[next].is_none() {
        next -= 1;
    }
    next
}

fn create_disk(disk_map: String) -> Vec<Option<usize>> {
    let mut file_id: usize = 0;
    let mut is_file: bool = true;
    let mut disk: Vec<Option<usize>> = Vec::new();
    for c in disk_map.chars() {
        if is_file {
            disk.extend(vec![Some(file_id); c.to_digit(10).unwrap() as usize]);
            file_id += 1;
            is_file = false;
        } else {
            if c != '0' { 
                disk.extend(vec![None; c.to_digit(10).unwrap() as usize]);
            }
            is_file = true;
        }
    }
    disk
}

fn create_disk_2(disk_map: String) -> (Vec<Option<usize>>, HashMap<usize, (usize, usize)>, Vec<(usize, usize)>, usize) {
    let mut file_id: usize = 0;
    let mut is_file: bool = true;
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut files: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut free_space: Vec<(usize, usize)> = Vec::new();
    let mut address: usize = 0;
    let mut size: usize = 0;


    for c in disk_map.chars() {
        if c != '0' {
            size = c.to_digit(10).unwrap() as usize;
        } else {
            size = 0;
        }
        if is_file {
            disk.extend(vec![Some(file_id); c.to_digit(10).unwrap() as usize]);
            files.insert(file_id, (address, size));
            address += size;
            file_id += 1;
            is_file = false;
        } else {
            if size > 0 { 
                disk.extend(vec![None; c.to_digit(10).unwrap() as usize]);
                free_space.push((address, size));
                address += size;
            }
            is_file = true;
        }
    }
    (disk, files, free_space, file_id - 1)
}

fn print_disk(disk: &Vec<Option<usize>>) {
    println!("");
    for c in disk {
        let c1 = match c {
            None => '.',
            Some(n  ) => n.to_string().chars().nth(0).unwrap(),
        };
        print!("{}", c1);
    }
    println!("");
}

fn read_input<R: BufRead>(mut reader: R) -> String {
    let mut line = String::new();
    let len = reader.read_line(&mut line);
    line.trim().to_string()
}
