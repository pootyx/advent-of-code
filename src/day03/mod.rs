use std::fs;
use std::io;
use regex::Regex;
use std::path::PathBuf;

fn find_all_multiplications(text: &str) -> i32 {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    let mut sum = 0;
    let mut dont_encountered = false;
    
    for captures in re.captures_iter(text) {
        let full_match = &captures[1];
        
        match full_match {
            "do()" => {
                dont_encountered = false;
                continue;
            },
            "don't()" => {
                dont_encountered = true;
                continue;
            },
            _ if !dont_encountered => {
                let num1: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                let num2: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
                sum += num1 * num2;
            },
            _ => {}
        }
    }
    
    sum
}

pub fn run() -> io::Result<()> {
    println!("Running day 3 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day03/input.txt");
    let text_contents = fs::read_to_string(path)?;
    
    let result = find_all_multiplications(&text_contents);
    println!("Sum of multiplications: {}", result);
    
    Ok(())
}
