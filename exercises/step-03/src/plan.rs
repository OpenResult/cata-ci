#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub name: String,
    pub summary: String,
    pub required_tools: Vec<String>,
    pub required_env: Vec<String>,
    pub tags: Vec<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub label: String,
    pub action: FakeAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FakeAction {
    pub program: String,
    pub args: Vec<String>,
    pub human: String,
}

impl Plan {
    pub fn render_text(&self) -> String {
        let tools = if self.required_tools.is_empty() {
            "none".to_string()
        } else {
            self.required_tools.join(", ")
        };
        let env = if self.required_env.is_empty() {
            "none".to_string()
        } else {
            self.required_env.join(", ")
        };
        let steps = self
            .steps
            .iter()
            .enumerate()
            .map(|(index, step)| format!("{}. {}", index + 1, step.action.human))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "plan: {}\nsummary: {}\ntags: {}\nrequired tools: {}\nrequired env: {}\nsteps:\n{}",
            self.name,
            self.summary,
            self.tags.join(", "),
            tools,
            env,
            steps
        )
    }
}

impl Step {
    pub fn new(label: &str, human: &str, args: &[&str]) -> Self {
        Self {
            label: label.to_string(),
            action: FakeAction {
                program: "fake-ci".to_string(),
                args: args.iter().map(|value| value.to_string()).collect(),
                human: human.to_string(),
            },
        }
    }
}
