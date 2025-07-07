use clap::{Parser, Subcommand};

use crate::commands::build::Build;

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "build", about = "Build current project")]
    Build(Build),
}

#[derive(Parser)]
#[command(name = "just", version, about = "Java project manager written in rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(Command::Build(cmd)) => cmd.run(),

            None => todo!(),
        }

        Ok(())
    }
}
