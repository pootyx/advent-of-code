mod day01;
mod day02;
mod day03; 

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("1") => day01::run(),
        Some("2") => day02::run(),
        Some("3") => day03::run(),
        _ => {
            println!("Please specify a valid day number (1-3)");
            Ok(())
        }
    }
}
