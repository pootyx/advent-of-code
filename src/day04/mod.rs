use std::fs;
use std::io;
use std::path::PathBuf;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

pub fn run() -> io::Result<()> {
    println!("Running day 4 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day04/input.txt");
    let text_contents = fs::read_to_string(path)?;
    let grid: Vec<Vec<char>> = text_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    part1(&grid);
    part2(&grid);
    Ok(())
}

fn part1(grid: &[Vec<char>]) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let word = "XMAS".chars().collect::<Vec<char>>();
    let mut count = 0;

    let is_valid = |x: i32, y: i32| -> bool {
        x >= 0 && x < rows && y >= 0 && y < cols
    };

    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] == word[0] {
                for &(dr, dc) in DIRECTIONS.iter() {
                    let mut valid = true;
                    let mut x = r;
                    let mut y = c;

                    for i in 1..word.len() {
                        x += dr;
                        y += dc;
                        
                        if !is_valid(x, y) || 
                           grid[x as usize][y as usize] != word[i] {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("part1: {}", count);
}

fn part2(grid: &[Vec<char>]) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    let is_valid = |x: i32, y: i32| -> bool {
        x >= 0 && x < rows && y >= 0 && y < cols
    };
    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] == 'A' {
                let diagonals = [
                    ((-1, -1), (1, 1)),
                    ((-1, 1), (1, -1)),
                ];

                let mut found = 0;

                for (dir1, dir2) in diagonals {
                    let (x1, y1) = (r + dir1.0, c + dir1.1);
                    let (x2, y2) = (r + dir2.0, c + dir2.1);
                    
                    if !is_valid(x1, y1) || !is_valid(x2, y2) {
                        continue;
                    }

                    let c1 = grid[x1 as usize][y1 as usize];
                    let c2 = grid[x2 as usize][y2 as usize];

                    if (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M') {
                        found += 1;
                    }
                }

                if found == 2 {
                    count += 1;
                }
            }
        }
    }

    println!("part2: {}", count);
}
