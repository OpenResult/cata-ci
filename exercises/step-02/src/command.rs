use crate::cli::{
    BuildCliCommand, CheckCliCommand, Cli, CliCommand, DeployCliCommand, ImageCliCommand,
};

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

impl Command {
    pub fn from_cli(cli: Cli) -> Self {
        cli.command.into()
    }

    pub fn describe(&self) -> String {
        match self {
            Command::Toolchain(ToolchainCommand::Java { version }) => {
                format!(
                    "typed command: select Java {}",
                    version.as_deref().unwrap_or("default")
                )
            }
            Command::Toolchain(ToolchainCommand::Node { version }) => {
                format!(
                    "typed command: select Node {}",
                    version.as_deref().unwrap_or("default")
                )
            }
            Command::Toolchain(ToolchainCommand::Maven { version }) => {
                format!(
                    "typed command: select Maven {}",
                    version.as_deref().unwrap_or("default")
                )
            }
            Command::Check(CheckKind::Lint) => "typed command: run fake lint".to_string(),
            Command::Check(CheckKind::Test) => "typed command: run fake test suite".to_string(),
            Command::Check(CheckKind::Format) => "typed command: run fake format check".to_string(),
            Command::Verify => "typed command: verify the fake project".to_string(),
            Command::Build(BuildTarget::Java) => {
                "typed command: build fake java artifact".to_string()
            }
            Command::Build(BuildTarget::Node) => {
                "typed command: build fake node bundle".to_string()
            }
            Command::Build(BuildTarget::All) => {
                "typed command: build all fake artifacts".to_string()
            }
            Command::Image(ImageCommand::Build) => "typed command: build fake image".to_string(),
            Command::Image(ImageCommand::Publish) => {
                "typed command: publish fake image".to_string()
            }
            Command::Deploy(DeployTarget::Sandbox) => {
                "typed command: deploy fake release to sandbox".to_string()
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use clap::Parser;

    #[test]
    fn maps_verify_into_internal_command() {
        let cli = Cli::parse_from(["kata-ci", "verify"]);

        assert_eq!(Command::from_cli(cli), Command::Verify);
    }

    #[test]
    fn maps_image_publish_into_internal_command() {
        let cli = Cli::parse_from(["kata-ci", "image", "publish"]);

        assert_eq!(
            Command::from_cli(cli),
            Command::Image(ImageCommand::Publish)
        );
    }
}
