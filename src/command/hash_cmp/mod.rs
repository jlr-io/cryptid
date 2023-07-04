use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use crate::command::Command;
use crate::file;

#[derive(Parser, Debug, Default)]
#[command(about = "Compare a file's hash to a given digest")]
pub struct HashCmpCommand {
    #[arg(index = 1, required = true)]
    pub digest: String,
    #[arg(index = 2, required = true)]
    pub path: PathBuf,
    #[arg(index = 3, required = false, default_value = "sha256")]
    pub algorithm: String,
}

impl Command for HashCmpCommand {
    fn run(&self) -> Result<()> {
        let file = file::File::new(&self.path);
        let hash = file.hash(&self.algorithm)?;
        if hash.to_lowercase() == self.digest.to_lowercase() {
            println!("Hashes match");
        } else {
            println!("Hashes do not match");
        }
        Ok(())
    }
}
