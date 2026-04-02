use crate::command::{
    BuildTarget, CheckKind, Command, DeployTarget, ImageCommand, ToolchainCommand,
};
use crate::config::Config;
use crate::plan::{Plan, Step};

pub fn plan(command: &Command, config: &Config) -> Plan {
    match command {
        Command::Toolchain(tool) => plan_toolchain(tool, config),
        Command::Check(check) => plan_check(check),
        Command::Verify => Plan {
            name: "verify".to_string(),
            summary: "Run the fake quality gates for the project".to_string(),
            required_tools: vec![
                "fake-linter".to_string(),
                "fake-test-runner".to_string(),
                "fake-formatter".to_string(),
            ],
            required_env: vec![],
            tags: vec![
                "verify".to_string(),
                "quality".to_string(),
                "local".to_string(),
            ],
            steps: vec![
                Step::new("lint", "run fake lint", &["check", "lint"]),
                Step::new("test", "run fake test suite", &["check", "test"]),
                Step::new("format", "run fake format check", &["check", "format"]),
            ],
        },
        Command::Build(target) => plan_build(target),
        Command::Image(command) => plan_image(command),
        Command::Deploy(target) => plan_deploy(target),
    }
}

fn plan_toolchain(tool: &ToolchainCommand, config: &Config) -> Plan {
    match tool {
        ToolchainCommand::Java { .. } => {
            let version = config.resolve_toolchain_version(tool);
            single_step_plan(
                "java",
                &format!("Select Java {version}"),
                vec!["toolchain".to_string(), "java".to_string()],
                "fake-toolchain-selector",
                Step::new(
                    "select-java",
                    &format!("select Java {version}"),
                    &["java", &version],
                ),
            )
        }
        ToolchainCommand::Node { .. } => {
            let version = config.resolve_toolchain_version(tool);
            single_step_plan(
                "node",
                &format!("Select Node {version}"),
                vec!["toolchain".to_string(), "node".to_string()],
                "fake-toolchain-selector",
                Step::new(
                    "select-node",
                    &format!("select Node {version}"),
                    &["node", &version],
                ),
            )
        }
        ToolchainCommand::Maven { .. } => {
            let version = config.resolve_toolchain_version(tool);
            single_step_plan(
                "maven",
                &format!("Select Maven {version}"),
                vec!["toolchain".to_string(), "maven".to_string()],
                "fake-toolchain-selector",
                Step::new(
                    "select-maven",
                    &format!("select Maven {version}"),
                    &["maven", &version],
                ),
            )
        }
    }
}

fn plan_check(check: &CheckKind) -> Plan {
    match check {
        CheckKind::Lint => single_step_plan(
            "check-lint",
            "Run fake lint",
            vec!["quality".to_string(), "lint".to_string()],
            "fake-linter",
            Step::new("lint", "run fake lint", &["check", "lint"]),
        ),
        CheckKind::Test => single_step_plan(
            "check-test",
            "Run fake test suite",
            vec!["quality".to_string(), "test".to_string()],
            "fake-test-runner",
            Step::new("test", "run fake test suite", &["check", "test"]),
        ),
        CheckKind::Format => single_step_plan(
            "check-format",
            "Run fake format check",
            vec!["quality".to_string(), "format".to_string()],
            "fake-formatter",
            Step::new("format", "run fake format check", &["check", "format"]),
        ),
    }
}

fn plan_build(target: &BuildTarget) -> Plan {
    match target {
        BuildTarget::Java => single_step_plan(
            "build-java",
            "Build the fake Java artifact",
            vec!["build".to_string(), "java".to_string()],
            "fake-builder",
            Step::new("build-java", "build fake java artifact", &["build", "java"]),
        ),
        BuildTarget::Node => single_step_plan(
            "build-node",
            "Build the fake Node bundle",
            vec!["build".to_string(), "node".to_string()],
            "fake-builder",
            Step::new("build-node", "build fake node bundle", &["build", "node"]),
        ),
        BuildTarget::All => Plan {
            name: "build-all".to_string(),
            summary: "Build every fake artifact".to_string(),
            required_tools: vec!["fake-builder".to_string()],
            required_env: vec![],
            tags: vec!["build".to_string(), "all".to_string()],
            steps: vec![
                Step::new("build-java", "build fake java artifact", &["build", "java"]),
                Step::new("build-node", "build fake node bundle", &["build", "node"]),
            ],
        },
    }
}

fn plan_image(command: &ImageCommand) -> Plan {
    match command {
        ImageCommand::Build => single_step_plan(
            "image-build",
            "Build the fake image",
            vec!["image".to_string(), "build".to_string()],
            "fake-image-builder",
            Step::new("image-build", "build fake image", &["image", "build"]),
        ),
        ImageCommand::Publish => single_step_plan(
            "image-publish",
            "Publish the fake image",
            vec!["image".to_string(), "publish".to_string()],
            "fake-image-publisher",
            Step::new("image-publish", "publish fake image", &["image", "publish"]),
        ),
    }
}

fn plan_deploy(target: &DeployTarget) -> Plan {
    match target {
        DeployTarget::Sandbox => Plan {
            name: "deploy-sandbox".to_string(),
            summary: "Deploy the fake release to sandbox".to_string(),
            required_tools: vec!["fake-deployer".to_string()],
            required_env: vec!["FAKE_ENV=sandbox".to_string()],
            tags: vec!["deploy".to_string(), "sandbox".to_string()],
            steps: vec![Step::new(
                "deploy-sandbox",
                "deploy fake release to sandbox",
                &["deploy", "sandbox"],
            )],
        },
    }
}

fn single_step_plan(
    name: &str,
    summary: &str,
    tags: Vec<String>,
    required_tool: &str,
    step: Step,
) -> Plan {
    Plan {
        name: name.to_string(),
        summary: summary.to_string(),
        required_tools: vec![required_tool.to_string()],
        required_env: vec![],
        tags,
        steps: vec![step],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolchain_plan_uses_config_defaults() {
        let config = Config::default();
        let plan = plan(
            &Command::Toolchain(ToolchainCommand::Java { version: None }),
            &config,
        );

        assert_eq!(plan.steps[0].action.human, "select Java 21");
    }

    #[test]
    fn build_all_contains_two_steps() {
        let config = Config::default();
        let plan = plan(&Command::Build(BuildTarget::All), &config);

        assert_eq!(plan.steps.len(), 2);
    }
}
