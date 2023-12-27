
use std::{ fs::read_to_string, io, path::Path};

fn read_lines(filename: &Path) -> Result<Vec<String>, io::Error> {
    let mut result = Vec::new();
    let lines = read_to_string(filename)?;
    for line in lines.lines() {
        result.push(line.to_string())
    }
    return Ok(result);
}


const LUT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line_1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line_2).sum())
}

fn parse_line_1(line: &str) -> u32 {
    let first = line.chars().find_map(|c| c.to_digit(10));
    let last = line.chars().rev().find_map(|c| c.to_digit(10));
    10 * first.unwrap() + last.unwrap()
}

fn parse_line_2(line: &str) -> u32 {
    let first = find_pattern(0..line.len(), line);
    let last = find_pattern((0..line.len()).rev(), line);
    10 * first + last
}

fn find_pattern(mut it: impl Iterator<Item = usize>, line: &str) -> u32 {
    it.find_map(|i| compare_slice(&line[i..])).unwrap()
}

fn compare_slice(slice: &str) -> Option<u32> {
    LUT.iter()
        .enumerate()
        .find(|(_, pattern)| slice.starts_with(*pattern))
        .map(|(i, _)| i as u32 + 1)
        .or_else(|| slice.chars().next().unwrap().to_digit(10))
}

fn main() {

    let path = Path::new("./src/day_1/input.txt");
    let res = read_lines(path);
    let input = match res {
        Ok(res) => res,
        Err(e) => {
            println!("Failed to read lines: {}", e);
            return;
        }
    };

    // let words_to_digits = [("one", 1), ("two", 2), ("three", 3), ("four", 4), 
    // ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), 
    // ("nine", 9)].iter().cloned().collect::<HashMap<&str, i32>>();
    let mut total = 0;
    for line in input {

        let t = part_two(&line);
        total += t.unwrap();
    }

    println!("{:?}", total)
      
}


