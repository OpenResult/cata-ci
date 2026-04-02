use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::command::ToolchainCommand;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(default = "default_java")]
    pub java: String,
    #[serde(default = "default_node")]
    pub node: String,
    #[serde(default = "default_maven")]
    pub maven: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: Defaults::default(),
        }
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            java: default_java(),
            node: default_node(),
            maven: default_maven(),
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self, String> {
        match path {
            Some(path) => {
                let input = fs::read_to_string(path).map_err(|error| {
                    format!("failed to read config {}: {error}", path.display())
                })?;
                toml::from_str(&input)
                    .map_err(|error| format!("failed to parse config {}: {error}", path.display()))
            }
            None => Ok(Self::default()),
        }
    }

    pub fn resolve_toolchain_version(&self, command: &ToolchainCommand) -> String {
        match command {
            ToolchainCommand::Java { version } => version
                .clone()
                .unwrap_or_else(|| self.defaults.java.clone()),
            ToolchainCommand::Node { version } => version
                .clone()
                .unwrap_or_else(|| self.defaults.node.clone()),
            ToolchainCommand::Maven { version } => version
                .clone()
                .unwrap_or_else(|| self.defaults.maven.clone()),
        }
    }
}

fn default_java() -> String {
    "21".to_string()
}

fn default_node() -> String {
    "20".to_string()
}

fn default_maven() -> String {
    "3.9.6".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_supplies_versions() {
        let config = Config::default();

        assert_eq!(config.defaults.java, "21");
        assert_eq!(config.defaults.node, "20");
        assert_eq!(config.defaults.maven, "3.9.6");
    }

    #[test]
    fn explicit_version_wins_over_default() {
        let config = Config::default();
        let version = config.resolve_toolchain_version(&ToolchainCommand::Java {
            version: Some("17".to_string()),
        });

        assert_eq!(version, "17");
    }
}
