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
    Maven {
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
    Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_maven_with_version() {
        let cli = Cli::parse_from(["kata-ci", "maven", "3.9.6"]);

        assert_eq!(
            cli.command,
            Command::Maven {
                version: Some("3.9.6".to_string()),
            }
        );
    }

    #[test]
    fn parses_format_check() {
        let cli = Cli::parse_from(["kata-ci", "check", "format"]);

        assert_eq!(
            cli.command,
            Command::Check {
                command: CheckCommand::Format,
            }
        );
    }
}
