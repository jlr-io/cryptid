use anyhow::Result;
use clap::Subcommand;
mod hash;
mod hash_cmp;

pub trait Command {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Hash(hash::HashCommand),
    HashCmp(hash_cmp::HashCmpCommand),
}