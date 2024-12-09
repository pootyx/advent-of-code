use std::fs;
use std::io;
use std::path::PathBuf;

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let (target, nums) = line.split_once(':').unwrap();
    let target = target.trim().parse().unwrap();
    let nums = nums
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (target, nums)
}

fn is_reachable_part1(target: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    
    let (&last, rest) = nums.split_last().unwrap();
    
    if target % last == 0 && is_reachable_part1(target / last, rest) {
        return true;
    }
    
    if target > last && is_reachable_part1(target - last, rest) {
        return true;
    }
    
    false
}

fn is_reachable_part2(target: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    
    let (&last, rest) = nums.split_last().unwrap();
    
    if target % last == 0 && is_reachable_part2(target / last, rest) {
        return true;
    }
    
    // Try addition
    if target > last && is_reachable_part2(target - last, rest) {
        return true;
    }
    
    let last_len = last.ilog10() + 1;
    let magnitude = 10usize.pow(last_len);
    let target_len = target.ilog10() + 1;
    let ending = target % magnitude;
    
    if target_len > last_len && last == ending && is_reachable_part2(target / magnitude, rest) {
        return true;
    }
    
    false
}

pub fn run() -> io::Result<()> {
    println!("Running day 7 solution");
    
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day07/input.txt");
    let input = fs::read_to_string(path)?;
    
    let equations: Vec<_> = input.lines().map(parse_line).collect();
    
    let sum_part1: usize = equations
        .iter()
        .filter(|(target, nums)| is_reachable_part1(*target, nums))
        .map(|(target, _)| target)
        .sum();
    
    let sum_part2: usize = equations
        .iter()
        .filter(|(target, nums)| is_reachable_part2(*target, nums))
        .map(|(target, _)| target)
        .sum();
    
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
    
    Ok(())
}
