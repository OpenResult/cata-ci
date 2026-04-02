# Step 4: Runner And Dry-Run

## Why This Step Matters

Once we have a plan, execution strategy becomes its own concern.

Run and dry-run should not require different planners.

## Goal

Introduce:

- `Runner`
- `ExecutionMode`
- fake `Executor`

## Example

```rust
pub enum ExecutionMode {
    Run,
    DryRun,
}

pub trait Executor {
    fn execute(&mut self, step: &Step) -> String;
}
```

## Design Benefit

- dry-run is just another way to consume a plan
- the executor can be faked in tests
- planning stays pure

## Discussion Prompt

- Where should output formatting live?
- Should the runner know about `clap` flags?

## Recap

- plan first, then execute
- dry-run is an execution concern
- fake executors make deterministic tests straightforward
