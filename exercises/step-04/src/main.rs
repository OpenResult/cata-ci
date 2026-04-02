mod cli;
mod command;
mod executor;
mod plan;
mod planner;
mod runner;

use clap::Parser;
use cli::Cli;
use command::Command;
use executor::FakeExecutor;
use runner::{ExecutionMode, Runner};

fn main() {
    let cli = Cli::parse();
    let mode = if cli.dry_run {
        ExecutionMode::DryRun
    } else {
        ExecutionMode::Run
    };
    let command: Command = cli.command.into();
    let plan = planner::plan(&command);

    let mut runner = Runner::new(FakeExecutor::default());
    let report = runner.run(&plan, mode);
    println!("{}", report.render());
}
