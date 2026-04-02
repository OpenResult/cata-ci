use crate::executor::Executor;
use crate::plan::Plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Run,
    DryRun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReport {
    pub lines: Vec<String>,
}

impl RunReport {
    pub fn render(&self) -> String {
        self.lines.join("\n")
    }
}

pub struct Runner<E> {
    executor: E,
}

impl<E> Runner<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl<E: Executor> Runner<E> {
    pub fn run(&mut self, plan: &Plan, mode: ExecutionMode) -> RunReport {
        let mut lines = vec![format!(
            "plan {} has {} step(s)",
            plan.name,
            plan.steps.len()
        )];

        for (index, step) in plan.steps.iter().enumerate() {
            let line = match mode {
                ExecutionMode::Run => {
                    format!("RUN {}: {}", index + 1, self.executor.execute(step))
                }
                ExecutionMode::DryRun => {
                    format!("DRY-RUN {}: {}", index + 1, step.action.human)
                }
            };
            lines.push(line);
        }

        RunReport { lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::FakeExecutor;
    use crate::plan::{Plan, Step};

    #[test]
    fn dry_run_keeps_executor_unused() {
        let plan = Plan {
            name: "verify".to_string(),
            summary: "Run fake checks".to_string(),
            required_tools: vec![],
            required_env: vec![],
            tags: vec!["verify".to_string()],
            steps: vec![Step::new("lint", "run fake lint", &["check", "lint"])],
        };
        let mut runner = Runner::new(FakeExecutor::default());

        let report = runner.run(&plan, ExecutionMode::DryRun);

        assert_eq!(report.lines[1], "DRY-RUN 1: run fake lint");
    }
}
