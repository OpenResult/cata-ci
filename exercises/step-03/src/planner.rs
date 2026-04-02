use crate::command::{
    BuildTarget, CheckKind, Command, DeployTarget, ImageCommand, ToolchainCommand,
};
use crate::plan::{Plan, Step};

pub fn plan(command: &Command) -> Plan {
    match command {
        Command::Toolchain(tool) => plan_toolchain(tool),
        Command::Check(check) => plan_check(check),
        Command::Verify => Plan {
            name: "verify".to_string(),
            summary: "Run the fake quality gates".to_string(),
            required_tools: vec![
                "fake-linter".to_string(),
                "fake-test-runner".to_string(),
                "fake-formatter".to_string(),
            ],
            required_env: vec![],
            tags: vec!["verify".to_string(), "quality".to_string()],
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

fn plan_toolchain(tool: &ToolchainCommand) -> Plan {
    match tool {
        ToolchainCommand::Java { version } => Plan {
            name: "java".to_string(),
            summary: format!("Select Java {}", version.as_deref().unwrap_or("21")),
            required_tools: vec!["fake-toolchain-selector".to_string()],
            required_env: vec![],
            tags: vec!["toolchain".to_string(), "java".to_string()],
            steps: vec![Step::new(
                "select-java",
                &format!("select Java {}", version.as_deref().unwrap_or("21")),
                &["java", version.as_deref().unwrap_or("21")],
            )],
        },
        ToolchainCommand::Node { version } => Plan {
            name: "node".to_string(),
            summary: format!("Select Node {}", version.as_deref().unwrap_or("20")),
            required_tools: vec!["fake-toolchain-selector".to_string()],
            required_env: vec![],
            tags: vec!["toolchain".to_string(), "node".to_string()],
            steps: vec![Step::new(
                "select-node",
                &format!("select Node {}", version.as_deref().unwrap_or("20")),
                &["node", version.as_deref().unwrap_or("20")],
            )],
        },
        ToolchainCommand::Maven { version } => Plan {
            name: "maven".to_string(),
            summary: format!("Select Maven {}", version.as_deref().unwrap_or("3.9.6")),
            required_tools: vec!["fake-toolchain-selector".to_string()],
            required_env: vec![],
            tags: vec!["toolchain".to_string(), "maven".to_string()],
            steps: vec![Step::new(
                "select-maven",
                &format!("select Maven {}", version.as_deref().unwrap_or("3.9.6")),
                &["maven", version.as_deref().unwrap_or("3.9.6")],
            )],
        },
    }
}

fn plan_check(check: &CheckKind) -> Plan {
    match check {
        CheckKind::Lint => single_step_plan(
            "check-lint",
            "Run fake lint",
            vec!["quality".to_string(), "lint".to_string()],
            Step::new("lint", "run fake lint", &["check", "lint"]),
        ),
        CheckKind::Test => single_step_plan(
            "check-test",
            "Run fake test suite",
            vec!["quality".to_string(), "test".to_string()],
            Step::new("test", "run fake test suite", &["check", "test"]),
        ),
        CheckKind::Format => single_step_plan(
            "check-format",
            "Run fake format check",
            vec!["quality".to_string(), "format".to_string()],
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
            Step::new("build-java", "build fake java artifact", &["build", "java"]),
        ),
        BuildTarget::Node => single_step_plan(
            "build-node",
            "Build the fake Node bundle",
            vec!["build".to_string(), "node".to_string()],
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
            Step::new("image-build", "build fake image", &["image", "build"]),
        ),
        ImageCommand::Publish => single_step_plan(
            "image-publish",
            "Publish the fake image",
            vec!["image".to_string(), "publish".to_string()],
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

fn single_step_plan(name: &str, summary: &str, tags: Vec<String>, step: Step) -> Plan {
    Plan {
        name: name.to_string(),
        summary: summary.to_string(),
        required_tools: vec!["fake-ci".to_string()],
        required_env: vec![],
        tags,
        steps: vec![step],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_has_three_steps() {
        let plan = plan(&Command::Verify);

        assert_eq!(plan.steps.len(), 3);
        assert_eq!(plan.steps[0].action.human, "run fake lint");
    }

    #[test]
    fn build_all_contains_java_and_node_steps() {
        let plan = plan(&Command::Build(BuildTarget::All));

        assert_eq!(plan.steps.len(), 2);
        assert_eq!(plan.steps[0].label, "build-java");
        assert_eq!(plan.steps[1].label, "build-node");
    }
}
