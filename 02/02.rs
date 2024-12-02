use std::fs;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let text_contents = fs::read_to_string("input.txt")?;
    
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
    
    println!("{}", similarity);

    Ok(())
}
