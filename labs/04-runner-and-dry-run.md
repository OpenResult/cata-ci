# Lab 4: Runner And Dry-Run

## Goal

Execute a plan with either real fake execution or dry-run output.

## Starting Point

Open `exercises/step-04/`.

The starter already knows how to build a `Plan`. Your job is to add a `Runner` and a fake `Executor`.

## Tasks

1. Inspect `src/runner.rs` and `src/executor.rs`.
2. Add `ExecutionMode` with `Run` and `DryRun`.
3. Make dry-run print clear step-by-step output without calling the executor.
4. Add tests that prove the fake executor is called only in run mode.

## Acceptance Criteria

- `cargo test -p kata-ci-exercise-step-04` passes
- `cargo run -p kata-ci-exercise-step-04 -- --dry-run image build` prints planned actions only
- `cargo run -p kata-ci-exercise-step-04 -- deploy sandbox` prints executed fake actions

## Hints

- the runner should consume a `Plan`, not raw CLI values
- record executor calls in memory for tests
- dry-run should still use the same planned steps

## Commands To Run

```bash
cargo test -p kata-ci-exercise-step-04
cargo run -p kata-ci-exercise-step-04 -- --dry-run verify
cargo run -p kata-ci-exercise-step-04 -- image publish
```

## Expected Behavior

- run mode executes every step in order
- dry-run mode prints every step in order
- tests remain deterministic and local

## Stretch Goal

Have the runner print a short completion summary with the number of steps.
