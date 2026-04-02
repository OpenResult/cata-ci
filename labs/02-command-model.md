# Lab 2: Typed Command Model

## Goal

Separate CLI parsing from the internal command representation.

## Starting Point

Open `exercises/step-02/`.

The starter still feels close to parser-driven behavior. Your job is to introduce and use a typed internal `Command`.

## Tasks

1. Inspect `src/cli.rs`, `src/command.rs`, and `src/main.rs`.
2. Add or complete the mapping from CLI types to the internal `Command` model.
3. Keep `clap` types out of the rest of the code as much as possible.
4. Add tests that prove the mapping works for a few commands.

## Acceptance Criteria

- `cargo test -p kata-ci-exercise-step-02` passes
- `cargo run -p kata-ci-exercise-step-02 -- build all` produces a typed-command-oriented output
- `main.rs` does not need to inspect raw nested `clap` enums deeply

## Hints

- `From` or `TryFrom` can be a nice boundary
- use small enums for `check`, `build`, `image`, and `deploy` targets
- explicit names are better than over-generic data structures

## Commands To Run

```bash
cargo test -p kata-ci-exercise-step-02
cargo run -p kata-ci-exercise-step-02 -- verify
cargo run -p kata-ci-exercise-step-02 -- image publish
```

## Expected Behavior

- the CLI still behaves the same from the outside
- the internal code reads more clearly because it works with typed intent

## Stretch Goal

Add a display helper that formats internal commands in a friendlier way than `Debug`.
