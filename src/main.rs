use cryptid::{
    cli::{Cli},
    command,
};

fn main() -> Result<(), command::CommandError>{
    Cli::run()
}
