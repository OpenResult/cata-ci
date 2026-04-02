use crate::plan::Step;

pub trait Executor {
    fn execute(&mut self, step: &Step) -> String;
}

#[derive(Debug, Default)]
pub struct FakeExecutor {
    pub calls: Vec<String>,
}

impl Executor for FakeExecutor {
    fn execute(&mut self, step: &Step) -> String {
        self.calls.push(step.action.human.clone());
        format!("executed: {}", step.action.human)
    }
}
