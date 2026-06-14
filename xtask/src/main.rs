mod cli;
mod config;
mod pipelines;
mod tools;

use clap::Parser;

use cli::{Cli, Command};
use config::Config;

fn main() {
    let args = Cli::parse();

    let config_path = args.config.unwrap_or_else(Config::default_path);
    let config = match Config::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let pipeline = pipelines::select(config.build.target);

    match args.command {
        Command::Build => pipeline.build(&config),
        Command::Run => {
            pipeline.build(&config);
            pipeline.run(&config);
        }
    }
}
