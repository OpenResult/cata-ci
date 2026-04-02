use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "kata-ci",
    bin_name = "kata-ci",
    about = "A fake CI-inspired CLI for Rust exercises"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum Command {
    Java {
        version: Option<String>,
    },
    Node {
        version: Option<String>,
    },
    Check {
        #[command(subcommand)]
        command: CheckCommand,
    },
    Verify,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum CheckCommand {
    Lint,
    Test,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_java_with_version() {
        let cli = Cli::parse_from(["kata-ci", "java", "21"]);

        assert_eq!(
            cli.command,
            Command::Java {
                version: Some("21".to_string()),
            }
        );
    }

    #[test]
    fn parses_nested_check_command() {
        let cli = Cli::parse_from(["kata-ci", "check", "lint"]);

        assert_eq!(
            cli.command,
            Command::Check {
                command: CheckCommand::Lint,
            }
        );
    }
}
