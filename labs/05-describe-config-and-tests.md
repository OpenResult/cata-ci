# Lab 5: Describe, Config, And Tests

## Goal

Render a plan in text or JSON, add minimal config defaults, and close the loop with tests.

## Starting Point

Open `exercises/step-05/`.

The starter has most of the architecture in place. This final step makes the CLI more inspectable and slightly more realistic.

## Tasks

1. Inspect `src/describe.rs`, `src/config.rs`, and `src/main.rs`.
2. Add `--describe` and `--format json`.
3. Use config defaults when `java`, `node`, or `maven` are called without a version.
4. Add unit tests around describing and one integration test around the CLI.

## Acceptance Criteria

- `cargo test -p kata-ci-exercise-step-05` passes
- `cargo run -p kata-ci-exercise-step-05 -- --describe verify` prints a readable plan
- `cargo run -p kata-ci-exercise-step-05 -- --describe --format json build all` prints valid JSON
- `cargo run -p kata-ci-exercise-step-05 -- java` uses a default version

## Hints

- keep describe rendering pure
- serialize the plan instead of re-deriving JSON manually
- config can stay tiny and optional

## Commands To Run

```bash
cargo test -p kata-ci-exercise-step-05
cargo run -p kata-ci-exercise-step-05 -- --describe verify
cargo run -p kata-ci-exercise-step-05 -- --describe --format json build all
```

## Expected Behavior

- text describe is concise and readable
- JSON describe is deterministic
- config defaults do not affect commands with explicit versions

## Stretch Goal

Add one integration test that checks `--help` mentions `--describe`.
