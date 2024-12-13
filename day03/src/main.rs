use regex::Regex;
use std::fs::read_to_string;

fn part_a(data: &str) -> i32 {
    let matcher = Regex::new(r"mul\((?<num1>[0-9]+),(?<num2>[0-9]+)\)").unwrap();
    let products: Vec<i32> = matcher
        .captures_iter(data)
        .map(|caps| caps["num1"].parse::<i32>().unwrap() * caps["num2"].parse::<i32>().unwrap())
        .collect();
    products.iter().sum()
}

fn part_b(data: &str) -> i32 {
    //let matcher = Regex::new(r"(mul\([0-9]+,[0-9]+\))").unwrap();
    let matcher = Regex::new(r"(do\(\))|(don't\(\))|(mul\([0-9]+,[0-9]+\))").unwrap();

    // First let's capture all the valid instructions in a vector:
    //   do()
    //   don't()
    //   mul(num1, num2)
    let instructions: Vec<String> = matcher
        .captures_iter(data)
        .map(|caps| caps[0].to_string())
        .collect();

    // Now go through our instructions and only add the multiples if
    // do() is enabled
    let mut enable_mul = true;
    let mut total: i32 = 0;
    for instruction in instructions {
        match instruction.as_str() {
            "do()" => {
                enable_mul = true;
            }
            "don't()" => {
                enable_mul = false;
            }
            _ => {
                if enable_mul {
                    let nums: Vec<&str> = instruction.split(',').collect();
                    let num1: i32 = nums[0][4..].parse::<i32>().unwrap();
                    let num2: i32 = nums[1][..nums[1].len() - 1].parse::<i32>().unwrap();
                    total += num1 * num2;
                }
            }
        }
    }

    total
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}
