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
        }
        // the first level is assumed to be safe, so check that len-1 of the rows are safe
        if safe_levels == row.len() - 1 {
            total += 1;
        }
    }
    total
}

fn part_b(data: &str) -> u32 {
    /*
    For this one, we need to actually generate all the possible versions of the row with one element missing
    This could certainly use some cleanup, but it works
     */
    let ascending = |a: &u32, b: &u32| -> bool { a < b };
    let descending = |a: &u32, b: &u32| -> bool { a > b };
    let safe_distance = |a: &u32, b: &u32| -> bool { a != b && a.abs_diff(*b) <= 3 };
    let mut total: u32 = 0;

    for line in data.lines() {
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let rows_to_check = get_row_with_one_missing(row);
        for check_row in rows_to_check {
            let direction_checker: fn(a: &u32, b: &u32) -> bool;
            match check_row[0] < check_row[1] {
                true => direction_checker = ascending,
                false => direction_checker = descending,
            }
            let mut safe_levels = 0;
            for i in 0..check_row.len() - 1 {
                if direction_checker(&check_row[i], &check_row[i + 1])
                    && safe_distance(&check_row[i], &check_row[i + 1])
                {
                    safe_levels += 1;
                }
            }
            if safe_levels >= check_row.len() - 1 {
                total += 1;
                break;
            }
        }
    }
    total
}

fn get_row_with_one_missing(row: Vec<u32>) -> Vec<Vec<u32>> {
    let mut row_with_one_missing: Vec<Vec<u32>> = Vec::new();
    for i in 0..row.len() {
        let mut new_row = row.clone();
        new_row.remove(i);
        row_with_one_missing.push(new_row);
    }
    row_with_one_missing
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}
