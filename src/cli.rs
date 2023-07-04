use anyhow::Result;
use clap::{Parser, command};
use crate::command::{Commands, Command};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

impl Cli {
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        Cli::exec_command(args)?;
        Ok(())
    }

    fn exec_command(cli: Cli) -> Result<()> {
        match cli.cmd {
            Commands::Hash(cmd) => cmd.run(),
            Commands::HashCmp(cmd) => cmd.run(),
        }
    }
}


