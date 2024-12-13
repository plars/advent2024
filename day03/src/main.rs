use regex::Regex;
use std::fs::read_to_string;

fn part_a(data: String) -> i32 {
    let matcher = Regex::new(r"mul\((?<num1>[0-9]+),(?<num2>[0-9]+)\)").unwrap();
    let products: Vec<i32> = matcher
        .captures_iter(data.as_str())
        .map(|caps| caps["num1"].parse::<i32>().unwrap() * caps["num2"].parse::<i32>().unwrap())
        .collect();
    products.iter().sum()
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    println!("{}", part_a(data));
}
