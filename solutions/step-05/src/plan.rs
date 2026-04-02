use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Plan {
    pub name: String,
    pub summary: String,
    pub required_tools: Vec<String>,
    pub required_env: Vec<String>,
    pub tags: Vec<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Step {
    pub label: String,
    pub action: FakeAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FakeAction {
    pub program: String,
    pub args: Vec<String>,
    pub human: String,
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
