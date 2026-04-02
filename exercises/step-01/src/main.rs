mod cli;

use clap::Parser;
use cli::{CheckCommand, Cli, Command};

fn main() {
    let cli = Cli::parse();
    println!("{}", render_command(&cli.command));
}

fn render_command(command: &Command) -> String {
    match command {
        Command::Java { version } => {
            format!("select Java {}", version.as_deref().unwrap_or("default"))
        }
        Command::Node { version } => {
            format!("select Node {}", version.as_deref().unwrap_or("default"))
        }
        Command::Check { command } => render_check(command).to_string(),
        Command::Verify => [
            render_check(&CheckCommand::Lint),
            render_check(&CheckCommand::Test),
        ]
        .join("\n"),
    }
}

fn render_check(command: &CheckCommand) -> &'static str {
    match command {
        CheckCommand::Lint => "run fake lint",
        CheckCommand::Test => "run fake test suite",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_verify_keeps_a_stable_order() {
        assert_eq!(
            render_command(&Command::Verify),
            "run fake lint\nrun fake test suite"
        );
    }
}
