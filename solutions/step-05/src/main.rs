mod cli;
mod command;
mod config;
mod describe;
mod executor;
mod plan;
mod planner;
mod runner;

use clap::Parser;
use cli::{Cli, FormatArg};
use command::Command;
use config::Config;
use describe::OutputFormat;
use executor::FakeExecutor;
use runner::{ExecutionMode, Runner};

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let config = Config::load(cli.config.as_deref())?;
    let command: Command = cli.command.into();
    let plan = planner::plan(&command, &config);

    if cli.describe {
        let format = match cli.format {
            FormatArg::Text => OutputFormat::Text,
            FormatArg::Json => OutputFormat::Json,
        };
        let rendered = describe::describe(&plan, format).map_err(|error| error.to_string())?;
        println!("{rendered}");
        return Ok(());
    }

    let mode = if cli.dry_run {
        ExecutionMode::DryRun
    } else {
        ExecutionMode::Run
    };
    let mut runner = Runner::new(FakeExecutor::default());
    let report = runner.run(&plan, mode);
    println!("{}", report.render());
    Ok(())
}
