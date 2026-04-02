mod cli;
mod command;
mod plan;
mod planner;

use clap::Parser;
use cli::Cli;
use command::Command;

fn main() {
    let cli = Cli::parse();
    let command = Command::from_cli(cli);
    let plan = planner::plan(&command);
    println!("{}", plan.render_text());
}
