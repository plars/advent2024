use std::collections::HashMap;
use std::fs::read_to_string;

fn part_a(data: &String) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in data.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut total: u32 = 0;
    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }
    total
}

fn part_b(data: &String) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    for line in data.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        *right.entry(r.parse::<u32>().unwrap()).or_insert(0) += 1;
    }

    let mut total: u32 = 0;
    for i in left.iter() {
        total += i * right.get(i).unwrap_or(&0);
    }
    total
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}
