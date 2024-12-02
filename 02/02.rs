use std::fs;
use std::io;

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
    
    column1.sort();
    column2.sort();

    let mut similarity = 0;
    let length = column1.len();

    for i in 0..length {
        for j in 0..length {
            if column1[i] == column2[j] {
                similarity += column1[i];
            }
        }
    }

    println!("{}", similarity);

    Ok(())
}
