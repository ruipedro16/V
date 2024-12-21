use crate::args::CLIArgs;
use crate::impls::Impl;
use std::error::Error;

pub fn run(args: &CLIArgs) -> Result<(), Box<dyn Error>> {
    let _impl_info = match Impl::build(&args.config, &args.outpath) {
        Ok(i) => i,
        Err(e) => {
            return Err(
                format!("Failed to build implementation info : {}", e.get_message()).into(),
            );
        }
    };

    // TODO:

    Ok(())
}
