use std::fs;
use std::io;

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

fn main() -> io::Result<()> {
    let text_contents = fs::read_to_string("input.txt")?;

    let lines: Vec<_> = text_contents.lines().collect();

    let mut count = 0;

    for line in lines {
        let readings = line.split_whitespace();
        let mut numbers: Vec<i32> = Vec::new();
        for reading in readings {
            if let Ok(number) = reading.parse::<i32>() {
                numbers.push(number);
            }
        }
        if is_sequence_safe_with_removal(&numbers) {
            count += 1;
        }
    }

    println!("Count: {}", count);

    
    Ok(())
}



