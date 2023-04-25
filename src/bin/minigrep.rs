use rust_scratch_pad::{Config, run};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        std::process::exit(1);
    });

    if let Result::Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

