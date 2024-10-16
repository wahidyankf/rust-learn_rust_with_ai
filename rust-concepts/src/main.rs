mod ch01_ownership;
mod ch02_borrowing;
mod ch03_lifetime;
mod ch04_pattern_matching;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("01" | "ch01" | "ownership") => {
            println!("Demonstrating Rust ownership!");
            println!("--------------");
            ch01_ownership::demo();
        }
        Some("02" | "ch02" | "borrowing") => {
            println!("Demonstrating Rust borrowing!");
            println!("--------------");
            ch02_borrowing::demo();
        }
        Some("03" | "ch03" | "lifetime") => {
            println!("Demonstrating Rust lifetimes!");
            println!("--------------");
            ch03_lifetime::demo();
        }
        Some("04" | "ch04" | "pattern_matching") => {
            println!("Demonstrating Rust pattern matching!");
            println!("--------------");
            ch04_pattern_matching::demo();
        }
        _ => {
            println!("Please specify a module to run. Examples:");
            println!("cargo run -- 01 (or ch01, or ownership)");
            println!("cargo run -- 02 (or ch02, or borrowing)");
            println!("cargo run -- 03 (or ch03, or lifetime)");
            println!("cargo run -- 04 (or ch04, or pattern_matching)");
        }
    }
}
