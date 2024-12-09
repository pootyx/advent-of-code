use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

pub fn run() -> io::Result<()> {
    println!("Running day 9 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day09/input.txt");
    
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    
    let mut numbers = Vec::new();
    let mut unsorted = Vec::new();

    let mut i = 0;
    let mut is_id = true;
    
    for byte_result in reader.bytes() {
        let byte = byte_result?;
        let c = byte as char;
        
        if c.is_ascii_digit() {
            let num = c.to_digit(10).unwrap() as i32;
            numbers.push(num);
        }
    }
    
    println!("Total numbers read: {}", numbers.len());

    for num in numbers {
        if is_id {
            for _ in 0..num {
                unsorted.push(i.to_string());
            }
            i += 1;
            is_id = false;
        } else {
            for _ in 0..num {
                unsorted.push(".".to_string());
            }
            is_id = true;
        }
    }

    println!("Initial: {:?}", &unsorted[..10]);

    let mut current_pos = 0;
    while current_pos < unsorted.len() {
        if unsorted[current_pos] == "." {
            if let Some(last_num_pos) = unsorted[current_pos..].iter()
                .enumerate()
                .rev()
                .find(|(_, s)| **s != ".")
                .map(|(i, _)| i + current_pos) 
            {
                unsorted.swap(current_pos, last_num_pos);
            }
        }
        current_pos += 1;
    }

    println!("Sorted: {:?}", &unsorted[..10]);

    let mut sum: i64 = 0;
    for (pos, val) in unsorted.iter().enumerate() {
        if val != "." {
            if let Ok(num) = val.parse::<i64>() {
                sum += pos as i64 * num;
            }
        }
    }
    
    println!("Sum of positions * values: {}", sum);

    Ok(())
}
