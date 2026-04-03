# kata-ci Rust CLI Workshop

`kata-ci` is a half-day Rust workshop for developers who already know how to design software, but are still getting comfortable with Rust.

The main workshop starts at step `01`.
Step `00` is optional pre-work for participants who want a focused Rust fundamentals warm-up before the CLI material begins.

The workshop teaches a specific CLI architecture progression:

- parse arguments with `clap`
- map parser output into a typed internal `Command`
- turn a `Command` into a `Plan`
- execute a `Plan` with a `Runner`
- support run, dry-run, and describe modes
- keep the design deterministic and easy to test

The domain is deliberately fake. `kata-ci` feels inspired by CI/CD tooling, but it never touches real tools, networks, registries, or deployment targets.

## Audience

- Developers who are strong in general engineering
- Teams adopting Rust for internal tools or platform tooling
- Learners who want a practical example of separation of concerns in a CLI
- Learners who may want an ownership-and-`Result` refresher before the main workshop

## What Participants Learn

Main workshop:

- How to model a small but maintainable Rust CLI
- When to keep `clap` types at the edge of the system
- How typed enums and structs improve clarity
- Why planning is a useful layer between intent and execution
- How dry-run and describe modes shape architecture
- How to write deterministic tests without real shell commands

Optional pre-work in step `00` also covers:

- `String` vs `&str`
- pass by value vs pass by reference
- mutable borrowing with `&mut`
- `Result`, `unwrap`, and `?`

## Agenda

Recommended pre-work:

- 60-90 min: step 00, Rust basics, self-paced and optional

Approximate pacing for the live half-day workshop:

- 15 min: intro and repo orientation
- 25 min: step 1, CLI basics
- 25 min: step 2, typed command model
- 30 min: step 3, planner and plan
- 30 min: step 4, runner and dry-run
- 35 min: step 5, describe, config, and tests
- 20 min: debrief and Q&A

## Local Setup

Requirements:

- stable Rust toolchain
- `cargo`

Recommended checks:

```bash
rustc --version
cargo --version
```

## Repo Layout

```text
slides/      live presentation material
labs/        participant lab instructions
exercises/   starter code for each step
solutions/   reference solution for each step
```

Each step is a separate crate so it can compile and run independently.
Step `00` is a library crate.
Steps `01 -> 05` are binary crates.

## Workshop Flow

Optional pre-work:

1. `slides/00-rust-basics.md`
2. `labs/00-rust-basics.md`

Main workshop flow:

1. `slides/01-cli-basics.md`
2. `labs/01-cli-basics.md`
3. `slides/02-command-model.md`
4. `labs/02-command-model.md`
5. `slides/03-plan-and-steps.md`
6. `labs/03-plan-and-steps.md`
7. `slides/04-runner-and-dry-run.md`
8. `labs/04-runner-and-dry-run.md`
9. `slides/05-describe-config-and-tests.md`
10. `labs/05-describe-config-and-tests.md`

## Running Exercises

All commands below are run from the repository root.

Step 00, optional pre-work:

```bash
cargo test -p kata-ci-exercise-step-00
```

Step 1:

```bash
cargo run -p kata-ci-exercise-step-01 -- --help
cargo test -p kata-ci-exercise-step-01
```

Step 2:

```bash
cargo run -p kata-ci-exercise-step-02 -- verify
cargo test -p kata-ci-exercise-step-02
```

Step 3:

```bash
cargo run -p kata-ci-exercise-step-03 -- build all
cargo test -p kata-ci-exercise-step-03
```

Step 4:

```bash
cargo run -p kata-ci-exercise-step-04 -- --dry-run deploy sandbox
cargo test -p kata-ci-exercise-step-04
```

Step 5:

```bash
cargo run -p kata-ci-exercise-step-05 -- --describe verify
cargo run -p kata-ci-exercise-step-05 -- --describe --format json build all
cargo test -p kata-ci-exercise-step-05
```

## Running Solutions

All commands below are run from the repository root.

Step 00, optional pre-work:

```bash
cargo test -p kata-ci-solution-step-00
```

Step 1:

```bash
cargo run -p kata-ci-solution-step-01 -- maven 3.9.6
cargo test -p kata-ci-solution-step-01
```

Step 2:

```bash
cargo run -p kata-ci-solution-step-02 -- image build
cargo test -p kata-ci-solution-step-02
```

Step 3:

```bash
cargo run -p kata-ci-solution-step-03 -- deploy sandbox
cargo test -p kata-ci-solution-step-03
```

Step 4:

```bash
cargo run -p kata-ci-solution-step-04 -- --dry-run image publish
cargo test -p kata-ci-solution-step-04
```

Step 5:

```bash
cargo run -p kata-ci-solution-step-05 -- --describe --format text verify
cargo run -p kata-ci-solution-step-05 -- --config solutions/step-05/kata-ci.toml build all
cargo test -p kata-ci-solution-step-05
```

## Recommended Facilitator Pacing

- Assign step `00` ahead of time when the audience is new enough to Rust that ownership and error handling would otherwise slow down the live session.
- Keep the lectures short and code-focused.
- Demo the solution crate only after participants have worked the exercise.
- Prefer discussing tradeoffs over Rust trivia.
- Re-anchor the group on the same three questions each step:
  1. What is the CLI edge?
  2. What is the internal model?
  3. What became easier to test after the refactor?

## Final Summary

This workshop is intentionally about one narrow architectural story:

- `clap` owns parsing
- internal `Command` owns intent
- `Planner` owns decomposition
- `Runner` owns execution strategy
- `Describer` owns rendering

Optional step `00` prepares the Rust language basics.
If you run `01 -> 05` in order, participants will see the same fake CLI evolve from a simple parser into a small, testable application.
