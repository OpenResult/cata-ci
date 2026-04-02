# Lab 1: CLI Basics

## Goal

Use `clap` to build a small fake CLI and make `--help` useful.

## Starting Point

Open `exercises/step-01/`.

The starter already parses a few commands and prints deterministic fake output.

## Tasks

1. Read `src/cli.rs` and `src/main.rs`.
2. Add support for `maven [version]`.
3. Add support for `check format`.
4. Make sure `verify` includes every fake check you want to model at this stage.

## Acceptance Criteria

- `cargo run -p kata-ci-exercise-step-01 -- --help` shows the available commands
- `cargo run -p kata-ci-exercise-step-01 -- maven 3.9.6` prints a fake selection message
- `cargo run -p kata-ci-exercise-step-01 -- check format` prints a fake format message

## Hints

- `clap` derive supports nested subcommands cleanly
- `Option<String>` is enough for an optional version argument
- keep the output simple and deterministic

## Commands To Run

```bash
cargo test -p kata-ci-exercise-step-01
cargo run -p kata-ci-exercise-step-01 -- java 21
cargo run -p kata-ci-exercise-step-01 -- check lint
```

## Expected Behavior

- install-style commands print a fake tool selection
- check commands print a fake action
- `verify` prints multiple fake actions in a stable order

## Stretch Goal

Improve the command descriptions in `clap` help output.
