fn part_a(data: &String) -> i32 {
    let mut count = 0;
    let wordsearch = make_wordsearch(&data);
    for (y, line) in wordsearch.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            count += match c {
                'X' => check_pattern_from_location(&wordsearch, "MAS", x, y),
                'S' => check_pattern_from_location(&wordsearch, "AMX", x, y),
                _ => 0,
            }
        }
    }

    count
}

fn make_wordsearch(data: &str) -> Vec<Vec<char>> {
    let mut wordsearch: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        wordsearch.push(line.chars().collect());
    }
    wordsearch
}

fn check_pattern_from_location(wordsearch: &[Vec<char>], pattern: &str, x: usize, y: usize) -> i32 {
    // starting from the specified x,y location, check how many times the remaining characters are found in any direction
    let mut count: i32 = 0;
    let max_y = wordsearch.len() - 1;
    let max_x = wordsearch[0].len() - 1;

    // check east
    if x + pattern.len() <= max_x {
        if wordsearch[y][x + 1..x + pattern.len() + 1]
            .iter()
            .cloned()
            .eq(pattern.chars())
        {
            count += 1;
        }
    }

    //check south
    if y + pattern.len() <= max_y {
        if (1..pattern.len() + 1)
            .map(|i| wordsearch[y + i][x])
            .eq(pattern.chars())
        {
            count += 1;
        }
    }

    //check southeast
    if x + pattern.len() <= max_x && y + pattern.len() <= max_y {
        if (1..pattern.len() + 1)
            .map(|i| wordsearch[y + i][x + i])
            .eq(pattern.chars())
        {
            count += 1;
        }
    }

    //check southwest
    if x as i32 - pattern.len() as i32 >= 0 && y + pattern.len() <= max_y {
        if (1..pattern.len() + 1)
            .map(|i| wordsearch[y + i][x - i])
            .eq(pattern.chars())
        {
            count += 1;
        }
    }
    count
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part_a(&data));
}

#[cfg(test)]
#[test]
fn test_part_a() {
    let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        .to_string();
    assert_eq!(part_a(&data), 18);
}
