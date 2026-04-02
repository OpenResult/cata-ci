# Lab 3: Planner And Plan

## Goal

Convert internal commands into explicit execution plans.

## Starting Point

Open `exercises/step-03/`.

The starter includes a command model and plan types. The planner is the missing architectural link.

## Tasks

1. Inspect `src/command.rs`, `src/plan.rs`, and `src/planner.rs`.
2. Implement planning for `verify`, `build`, `image`, and `deploy`.
3. Fill in plan metadata such as summary, tags, required tools, and required environment.
4. Keep the steps deterministic and easy to read.

## Acceptance Criteria

- `cargo test -p kata-ci-exercise-step-03` passes
- `cargo run -p kata-ci-exercise-step-03 -- verify` prints a clear plan summary
- `cargo run -p kata-ci-exercise-step-03 -- build all` contains both fake java and fake node build steps

## Hints

- a `Plan` is ordered work, not parser state
- keep the plan model small
- use helper constructors if the match expression gets noisy

## Commands To Run

```bash
cargo test -p kata-ci-exercise-step-03
cargo run -p kata-ci-exercise-step-03 -- verify
cargo run -p kata-ci-exercise-step-03 -- deploy sandbox
```

## Expected Behavior

- `verify` expands into multiple steps
- `build all` expands into multiple steps
- metadata explains the plan, not just the raw command

## Stretch Goal

Add a `required_env` entry for the fake deploy command.
