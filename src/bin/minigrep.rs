use rust_scratch_pad::{Config, run};

fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        std::process::exit(1);
    });

    if let Result::Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

