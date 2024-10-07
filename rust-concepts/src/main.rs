mod ch01_ownership;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(arg) if arg == "ch01_ownership" => {
            println!("Demonstrating Rust ownership!");
            println!("--------------");
            ch01_ownership::demo();
        }
        _ => {
            println!("Please specify a module to run. Example: cargo run -- ch01_ownership");
        }
    }
}
