use std::fs::read_to_string;

fn part_a(data: String) -> u32 {
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

fn main() {
    let data = read_to_string("input.txt").unwrap();
    //let data = "1 2\n3 4\n5 6\n7 8".to_string();
    println!("{}", part_a(data));
}
