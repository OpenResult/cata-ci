mod cli;
mod command;

use clap::Parser;
use cli::Cli;
use command::Command;

fn main() {
    let cli = Cli::parse();
    let command = Command::from_cli(cli);
    println!("{}", command.describe());
}
