use std::{env, path::PathBuf};

use clap::{Args, Parser, Subcommand};

use crate::{commands::build::Build, config::Config};

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "build", about = "Build current project")]
    Build(Build),
}

#[derive(Parser)]
#[command(name = "just", version, about = "Java project manager written in rust")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,

    #[command(flatten)]
    global: GlobalArgs,
}

#[derive(Args, Debug)]
struct GlobalArgs {
    /// Optional path to project root
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let run_in = match &self.global.path {
            Some(p) => p,
            None => &env::current_dir()?,
        };

        let config = match Config::load() {
            Ok(c) => c,
            Err(_) => Config::initialize_in(run_in)?,
        };

        match &self.command {
            Command::Build(cmd) => cmd.run(run_in, config)?,
        }

        Ok(())
    }
}
