use serde_json::Error;

use crate::plan::Plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn describe(plan: &Plan, format: OutputFormat) -> Result<String, Error> {
    match format {
        OutputFormat::Text => Ok(render_text(plan)),
        OutputFormat::Json => serde_json::to_string_pretty(plan),
    }
}

fn render_text(plan: &Plan) -> String {
    let tools = if plan.required_tools.is_empty() {
        "none".to_string()
    } else {
        plan.required_tools.join(", ")
    };
    let env = if plan.required_env.is_empty() {
        "none".to_string()
    } else {
        plan.required_env.join(", ")
    };
    let steps = plan
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| format!("{}. {}", index + 1, step.action.human))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "plan: {}\nsummary: {}\ntags: {}\nrequired tools: {}\nrequired env: {}\nsteps:\n{}",
        plan.name,
        plan.summary,
        plan.tags.join(", "),
        tools,
        env,
        steps
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Plan, Step};

    fn sample_plan() -> Plan {
        Plan {
            name: "verify".to_string(),
            summary: "Run fake checks".to_string(),
            required_tools: vec!["fake-linter".to_string()],
            required_env: vec![],
            tags: vec!["verify".to_string()],
            steps: vec![Step::new("lint", "run fake lint", &["check", "lint"])],
        }
    }

    #[test]
    fn text_output_is_readable() {
        let rendered = describe(&sample_plan(), OutputFormat::Text).unwrap();

        assert!(rendered.contains("plan: verify"));
        assert!(rendered.contains("1. run fake lint"));
    }

    #[test]
    fn json_output_contains_steps() {
        let rendered = describe(&sample_plan(), OutputFormat::Json).unwrap();

        assert!(rendered.contains("\"name\": \"verify\""));
        assert!(rendered.contains("\"steps\""));
    }
}
