use crate::file;
use std::path::PathBuf;
use clap::{Parser, Subcommand};


#[derive(Debug)]
pub enum CommandError {
    FileError(file::FileError),
}

impl From<file::FileError> for CommandError {
    fn from(error: file::FileError) -> Self {
        CommandError::FileError(error)
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Hash(HashArgs),
}

#[derive(Parser, Debug, Default)]
#[group(required = true)]
pub struct HashArgs {
    #[arg(short, long)]
    pub path: PathBuf,
    #[arg(short, long)]
    pub algorithm: String,
}

pub fn hash_file(args: HashArgs) -> Result<(), CommandError> {
    let file = file::File::new(args.path);
    let hash = file.hash(&args.algorithm)?;
    println!("{}", hash);
    Ok(())
}