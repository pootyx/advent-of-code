use std::fs;
use std::io;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run() -> io::Result<()> {
    println!("Running day 6 solution");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day06/input.txt");
    let text_contents = fs::read_to_string(path)?;
    let grid: Vec<Vec<char>> = text_contents.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}", grid);
    part1(&grid);
    Ok(())
}

fn part1(grid: &[Vec<char>]) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut x = 0;
    let mut y = 0;
    let mut direction: Option<Direction> = None;
    
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let is_valid = |x: i32, y: i32| -> bool {
        x >= 0 && x < cols && y >= 0 && y < rows
    };
    
    let is_obstacle = |x: i32, y: i32| -> bool {
        grid[y as usize][x as usize] == '#'
    };

    'find_start: for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] == '^' {
                y = r;
                x = c;
                direction = Some(Direction::Up);
                break 'find_start;
            } else if grid[r as usize][c as usize] == 'v' {
                y = r;
                x = c;
                direction = Some(Direction::Down);
                break 'find_start;
            } else if grid[r as usize][c as usize] == '<' {
                y = r;
                x = c;
                direction = Some(Direction::Left);
                break 'find_start;
            } else if grid[r as usize][c as usize] == '>' {
                y = r;
                x = c;
                direction = Some(Direction::Right);
                break 'find_start;
            }
        }
    }

    if !direction.is_some() { 
        println!("Invalid start position");
        return;
    }

    visited.insert((x, y));

    while is_valid(x, y) {
        let (next_x, next_y) = match direction {
            Some(Direction::Up) => (x, y - 1),
            Some(Direction::Down) => (x, y + 1),
            Some(Direction::Left) => (x - 1, y),
            Some(Direction::Right) => (x + 1, y),
            None => break,
        };

        if !is_valid(next_x, next_y) {
            break;
        }

        if is_obstacle(next_x, next_y) {
            direction = Some(match direction.unwrap() {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            });
        } else {
            x = next_x;
            y = next_y;
            visited.insert((x, y));
        }
    }

    println!("Distinct positions visited: {}", visited.len());
}
