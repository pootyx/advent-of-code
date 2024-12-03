use std::fs;
use std::io;
use regex::Regex;
use std::path::PathBuf;

fn find_all_multiplications(text: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut results = Vec::new();
    
    for captures in re.captures_iter(text) {
        let num1 = captures[1].parse::<i32>().unwrap();
        let num2 = captures[2].parse::<i32>().unwrap();
        results.push((num1, num2));
    }
    
    results
}

pub fn run() -> io::Result<()> {
    println!("Running day 3 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day03/input.txt");
    let text_contents = fs::read_to_string(path)?;
    let multiplications = find_all_multiplications(&text_contents);

    let mut part1 = 0;

    for (num1, num2) in multiplications {
        let result = num1 * num2;
        part1 += result;
    }

    println!("Part 1: {}", part1);
    
    Ok(())
}
