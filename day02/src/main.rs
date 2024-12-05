use std::fs::read_to_string;

fn part_a(data: &str) -> u32 {
    let ascending = |a: &u32, b: &u32| -> bool { a < b };
    let descending = |a: &u32, b: &u32| -> bool { a > b };
    let safe_distance = |a: &u32, b: &u32| -> bool { a != b && a.abs_diff(*b) <= 3 };
    let mut total: u32 = 0;

    for line in data.lines() {
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let direction_checker: fn(a: &u32, b: &u32) -> bool;
        match row[0] < row[1] {
            true => direction_checker = ascending,
            false => direction_checker = descending,
        }
        let mut safe_levels = 0;
        for i in 0..row.len() - 1 {
            if direction_checker(&row[i], &row[i + 1]) && safe_distance(&row[i], &row[i + 1]) {
                safe_levels += 1;
            }
            if safe_levels == row.len() - 1 {
                total += 1;
            }
        }
    }
    total
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    println!("{}", part_a(&data));
}
