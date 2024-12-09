use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

fn part2(numbers: &[i32]) -> i64 {
    let mut disk = Vec::new();
    let mut positions = Vec::new();
    let mut sizes = Vec::new();
    let mut file_id = 0;
    
    for (i, &num) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            let start = disk.len();
            for _ in 0..num {
                disk.push(file_id);
            }
            positions.push((start, disk.len()));
            sizes.push(num);
            file_id += 1;
        } else {
            for _ in 0..num {
                disk.push(-1);
            }
        }
    }

    for id in (0..sizes.len()).rev() {
        let (start, end) = positions[id];
        let size = sizes[id];
        
        let mut free_start = -1;
        let mut free_count = 0;
        
        for pos in 0..start {
            if disk[pos] == -1 {
                if free_start == -1 {
                    free_start = pos as i32;
                }
                free_count += 1;
            } else {
                free_start = -1;
                free_count = 0;
            }
            
            if free_count == size {
                let free_start = free_start as usize;
                for i in 0..size as usize {
                    disk[free_start + i] = id as i32;
                    disk[start + i] = -1;
                }
                break;
            }
        }
    }

    let mut sum = 0i64;
    for (pos, &val) in disk.iter().enumerate() {
        if val != -1 {
            sum += pos as i64 * val as i64;
        }
    }
    
    sum
}

pub fn run() -> io::Result<()> {
    println!("Running day 9 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day09/input.txt");
    
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    
    let mut numbers = Vec::new();
    
    for byte_result in reader.bytes() {
        let byte = byte_result?;
        let c = byte as char;
        
        if c.is_ascii_digit() {
            let num = c.to_digit(10).unwrap() as i32;
            numbers.push(num);
        }
    }
    
    let mut unsorted = Vec::new();
    let mut i = 0;
    let mut is_id = true;
    
    for num in &numbers {
        if is_id {
            for _ in 0..*num {
                unsorted.push(i.to_string());
            }
            i += 1;
            is_id = false;
        } else {
            for _ in 0..*num {
                unsorted.push(".".to_string());
            }
            is_id = true;
        }
    }


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


    let mut sum: i64 = 0;
    for (pos, val) in unsorted.iter().enumerate() {
        if val != "." {
            if let Ok(num) = val.parse::<i64>() {
                sum += pos as i64 * num;
            }
        }
    }
    
    println!("Part 1: {}", sum);

    let part2_sum = part2(&numbers);
    println!("Part 2: {}", part2_sum);

    Ok(())
}
