mod cli;

use clap::Parser;
use cli::{CheckCommand, Cli, Command};

fn main() {
    let cli = Cli::parse();
    println!("{}", render_command(&cli.command));
}

fn render_command(command: &Command) -> String {
    match command {
        Command::Java { version } => render_tool("Java", version.as_deref()),
        Command::Node { version } => render_tool("Node", version.as_deref()),
        Command::Maven { version } => render_tool("Maven", version.as_deref()),
        Command::Check { command } => render_check(command).to_string(),
        Command::Verify => [
            render_check(&CheckCommand::Lint),
            render_check(&CheckCommand::Test),
            render_check(&CheckCommand::Format),
        ]
        .join("\n"),
    }
}

fn render_tool(tool: &str, version: Option<&str>) -> String {
    format!("select {tool} {}", version.unwrap_or("default"))
}

fn render_check(command: &CheckCommand) -> &'static str {
    match command {
        CheckCommand::Lint => "run fake lint",
        CheckCommand::Test => "run fake test suite",
        CheckCommand::Format => "run fake format check",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_verify_includes_every_check() {
        assert_eq!(
            render_command(&Command::Verify),
            "run fake lint\nrun fake test suite\nrun fake format check"
        );
    }
}
