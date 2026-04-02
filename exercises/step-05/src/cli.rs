use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "kata-ci",
    bin_name = "kata-ci",
    about = "A fake CI-inspired CLI for Rust exercises"
)]
pub struct Cli {
    #[arg(long, help = "Print the plan without executing it")]
    pub dry_run: bool,

    #[arg(long, help = "Render the plan instead of running it")]
    pub describe: bool,

    #[arg(long, value_enum, default_value_t = FormatArg::Text)]
    pub format: FormatArg,

    #[arg(long, help = "Optional path to a kata-ci TOML config file")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FormatArg {
    Text,
    Json,
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
