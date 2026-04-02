use crate::cli::{BuildCliCommand, CheckCliCommand, CliCommand, DeployCliCommand, ImageCliCommand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Toolchain(ToolchainCommand),
    Check(CheckKind),
    Verify,
    Build(BuildTarget),
    Image(ImageCommand),
    Deploy(DeployTarget),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolchainCommand {
    Java { version: Option<String> },
    Node { version: Option<String> },
    Maven { version: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckKind {
    Lint,
    Test,
    Format,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildTarget {
    Java,
    Node,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageCommand {
    Build,
    Publish,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeployTarget {
    Sandbox,
}

impl From<CliCommand> for Command {
    fn from(value: CliCommand) -> Self {
        match value {
            CliCommand::Java { version } => Command::Toolchain(ToolchainCommand::Java { version }),
            CliCommand::Node { version } => Command::Toolchain(ToolchainCommand::Node { version }),
            CliCommand::Maven { version } => {
                Command::Toolchain(ToolchainCommand::Maven { version })
            }
            CliCommand::Check { command } => Command::Check(match command {
                CheckCliCommand::Lint => CheckKind::Lint,
                CheckCliCommand::Test => CheckKind::Test,
                CheckCliCommand::Format => CheckKind::Format,
            }),
            CliCommand::Verify => Command::Verify,
            CliCommand::Build { target } => Command::Build(match target {
                BuildCliCommand::Java => BuildTarget::Java,
                BuildCliCommand::Node => BuildTarget::Node,
                BuildCliCommand::All => BuildTarget::All,
            }),
            CliCommand::Image { command } => Command::Image(match command {
                ImageCliCommand::Build => ImageCommand::Build,
                ImageCliCommand::Publish => ImageCommand::Publish,
            }),
            CliCommand::Deploy { target } => Command::Deploy(match target {
                DeployCliCommand::Sandbox => DeployTarget::Sandbox,
            }),
        }
    }
}
