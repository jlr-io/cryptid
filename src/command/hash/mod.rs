use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use crate::command::Command;
use crate::file;

#[derive(Parser, Debug)]
pub struct HashCommand {
    #[arg(index = 1, required = true)]
    pub path: PathBuf,
    #[arg(index = 2, required = false, default_value = "sha256")]
    pub algorithm: String,
}

impl Command for HashCommand {
    fn run(&self) -> Result<()> {
        let file = file::File::new(&self.path);
        let hash = file.hash(&self.algorithm)?;
        println!("{}", self.path.display());
        println!("{}", hash.to_lowercase());
        Ok(())
    }
}