use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "kata-ci",
    bin_name = "kata-ci",
    about = "A fake CI-inspired CLI for Rust exercises"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum CliCommand {
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
        command: CheckCliCommand,
    },
    Verify,
    Build {
        #[command(subcommand)]
        target: BuildCliCommand,
    },
    Image {
        #[command(subcommand)]
        command: ImageCliCommand,
    },
    Deploy {
        #[command(subcommand)]
        target: DeployCliCommand,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum CheckCliCommand {
    Lint,
    Test,
    Format,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum BuildCliCommand {
    Java,
    Node,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ImageCliCommand {
    Build,
    Publish,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum DeployCliCommand {
    Sandbox,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_deploy_sandbox() {
        let cli = Cli::parse_from(["kata-ci", "deploy", "sandbox"]);

        assert_eq!(
            cli.command,
            CliCommand::Deploy {
                target: DeployCliCommand::Sandbox,
            }
        );
    }
}
