use init_jasmin_impl::{args, worker};

fn main() {
    let args = match args::CLIArgs::build() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("CLI Arguments Error: {}", e.get_message());
            std::process::exit(1);
        }
    };

    worker::run(&args).unwrap(); // TODO: Remove this unwrap and handle the errors properly
}
