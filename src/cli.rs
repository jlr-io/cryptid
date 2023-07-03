use clap::{Parser, command};
use crate::command::{self, Commands, CommandError};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

impl Cli {
    pub fn run() -> Result<(), CommandError> {
        let args = Cli::parse();
        Cli::exec_command(args)?;
        Ok(())
    }

    fn exec_command(args: Cli) -> Result<(), CommandError> {
        match args.cmd {
            Commands::Hash(hash_args) => command::hash_file(hash_args),
        }
    }
}


