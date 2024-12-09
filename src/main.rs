mod day01;
mod day02;
mod day03; 
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("1") => day01::run(),
        Some("2") => day02::run(),
        Some("3") => day03::run(),
        Some("4") => day04::run(),
        Some("5") => day05::run(),
        Some("6") => day06::run(),
        Some("7") => day07::run(),
        Some("8") => day08::run(),
        Some("9") => day09::run(),
        _ => {
            println!("Please specify a valid day number (1-5)");
            Ok(())
        }
    }
}
