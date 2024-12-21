use std::{fs, path::Path};

use clap::Parser;
use serde_json::Value;

#[derive(Debug)]
pub enum CLIArgsError {
    InvalidConfigFile(String),
    NoSuchConfigFile(String),
    OutputDirectoryNotEmpty(String),
}

impl CLIArgsError {
    pub fn get_message(&self) -> &str {
        match self {
            CLIArgsError::InvalidConfigFile(msg) => msg,
            CLIArgsError::NoSuchConfigFile(msg) => msg,
            CLIArgsError::OutputDirectoryNotEmpty(msg) => msg,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub stdlib: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub add_jpp: bool,

    #[arg(short, long)]
    pub config: String,

    #[arg(short, long)]
    pub outpath: String,
}

impl CLIArgs {
    pub fn build() -> Result<CLIArgs, CLIArgsError> {
        let mut args = CLIArgs::parse();

        if !Path::new(&args.config).exists() {
            return Err(CLIArgsError::NoSuchConfigFile(format!(
                "No such config file: {}",
                args.config
            )));
        }

        if args.verbose {
            log::info!("Config file exists: {}", args.config);
        }

        let file_content = fs::read_to_string(&args.config).expect("Failed to read config file");

        if serde_json::from_str::<Value>(&file_content).is_err() {
            return Err(CLIArgsError::InvalidConfigFile(format!(
                "Invalid config file: {}",
                args.config
            )));
        }

        if args.verbose {
            log::info!("Config file is valid: {}", args.config);
        }

        if args.outpath.ends_with('/') {
            args.outpath.pop(); // remove the / at the end
        }

        // If the outpath exists, must be empty
        if Path::new(&args.outpath)
            .read_dir()
            .map(|mut i| i.next().is_some())
            .unwrap_or(false)
        {
            return Err(CLIArgsError::OutputDirectoryNotEmpty(format!(
                "Output directory is not empty: {}",
                args.outpath
            )));
        }

        Ok(args)
    }
}
