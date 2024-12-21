use init_jasmin_impl::{args, worker};

use log::LevelFilter;

fn main() {
    // Initialize the logger
    env_logger::builder().filter_level(LevelFilter::Warn).init();

    let args = match args::CLIArgs::build() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("CLI Arguments Error: {}", e.get_message());
            std::process::exit(1);
        }
    };

    worker::run(&args).unwrap(); // TODO: Remove this unwrap and handle the errors properly
}
