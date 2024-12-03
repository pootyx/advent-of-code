use std::fs;
use std::io;
use std::path::PathBuf;

fn parse_line(line: &str) -> Vec<i32> {
    let readings = line.split_whitespace();
    let mut numbers: Vec<i32> = Vec::new();
    for reading in readings {
        if let Ok(number) = reading.parse::<i32>() {
            numbers.push(number);
        }
    }
    numbers
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for pair in numbers.windows(2) {
        let diff = pair[0].abs_diff(pair[1]);
        if diff == 0 || diff > 3 {
            return false;
        }
        
        if pair[0] > pair[1] {
            is_increasing = false;
        }
        if pair[0] < pair[1] {
            is_decreasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn is_sequence_safe_with_removal(numbers: &[i32]) -> bool {
    if is_valid_sequence(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut new_sequence: Vec<i32> = numbers.to_vec();
        new_sequence.remove(i);
        if is_valid_sequence(&new_sequence) {
            return true;
        }
    }

    false
}

pub fn run() -> io::Result<()> {
    println!("Running day 2 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day02/input.txt");
    let text_contents = fs::read_to_string(path)?;

    let lines: Vec<_> = text_contents.lines().collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let numbers = parse_line(line);

        if is_valid_sequence(&numbers) {
            part1 += 1;
        }
        if is_sequence_safe_with_removal(&numbers) {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    
    Ok(())
}



