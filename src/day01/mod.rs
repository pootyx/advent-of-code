use std::fs;
use std::io;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn run() -> io::Result<()> {
    println!("Running day 1 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day01/input.txt");
    let text_contents = fs::read_to_string(path)?;
    
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();
    
    for line in text_contents.lines() {
        let mut iter = line.split_whitespace();
        let col1 = iter.next();
        let col2 = iter.next();
    
        if let (Some(col1), Some(col2)) = (col1, col2) {
            if let (Ok(num1), Ok(num2)) = (col1.parse::<i32>(), col2.parse::<i32>()) {
                column1.push(num1);
                column2.push(num2);
            }
        }
    }
    
    column1.sort();
    column2.sort();

    let mut count = 0;
    let length = column1.len();

    for i in 0..length {
        count += (column1[i] - column2[i]).abs();
    }

    println!("part 1: {}", count);

    let mut similarity = 0;
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    
    for &num in &column2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    
    for &num in &column1 {
        if let Some(&count) = freq_map.get(&num) {
            similarity += num * count;
        }
    }
    
    println!("part 2: {}", similarity);

    Ok(())
}
