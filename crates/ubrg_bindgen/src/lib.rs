pub mod cli;
pub mod ruby;

use anyhow::Result;

pub fn run(cli: cli::Cli) -> Result<()> {
    match cli.command {
        cli::Command::Generate(args) => ruby::generate_bindings(&args),
    }
}
