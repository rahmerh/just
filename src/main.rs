use clap::Parser;
use owo_colors::OwoColorize;

use crate::cli::Cli;

mod cli;
mod commands;

fn main() {
    env_logger::init();
    std::panic::set_hook(Box::new(|info| {
        eprintln!("{} {}", "Something unexpected happened:".red().bold(), info);
        std::process::exit(1);
    }));

    let cli = Cli::parse();

    if let Err(err) = cli.run() {
        eprintln!("{} {}", "Encountered a problem:".red().bold(), err);

        for cause in err.chain().skip(1) {
            log::debug!(" -> caused by: {}", cause);
        }

        std::process::exit(1);
    }
}
