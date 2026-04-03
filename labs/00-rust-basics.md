# Lab 00: Rust Basics

## Goal

Practice the Rust fundamentals that the main workshop relies on: ownership, borrowing, mutation through references, and `Result`-based error handling.

This step is optional pre-work.
It does not use the `kata-ci` CLI theme yet.

## Starting Point

Open `exercises/step-00/`.

This starter is a small library for building and updating a fake `BuildProfile`.

## Tasks

1. Read the function signatures in `src/profile.rs` and identify which ones take ownership, which ones borrow, and which ones mutate through `&mut`.
2. Implement or improve `owner_label` so `None` becomes a readable fallback.
3. Implement or improve `rename_profile` so it updates an existing struct through a mutable reference.
4. Replace the `unwrap` in `build_profile` with proper `Result` propagation using `?`.
5. Add tests for invalid retry input and empty profile names.
6. Use `into_profile_name` to observe a by-value consuming function.

## Acceptance Criteria

- `cargo test -p kata-ci-exercise-step-00` passes
- `build_profile("release", Some("alex"), "3")` succeeds
- invalid retry input returns `Err` instead of panicking
- `rename_profile` updates the existing profile in place
- you can explain why `summarize(&profile)` can be called more than once

## Hints

- `&str` is borrowed text, `String` is owned text
- `Option<&str>` can become `Option<String>` with `.map(str::to_string)`
- use `?` when a function already returns `Result`
- `&mut` lets you change a value without taking ownership of it

## Commands To Run

```bash
cargo test -p kata-ci-exercise-step-00
cargo test -p kata-ci-solution-step-00
```

## Expected Behavior

- names are normalized into lowercase, dash-separated strings
- optional owner information is handled without crashes
- invalid retry values are reported as `Err`
- borrowed values remain usable after shared-reference functions run

## Stretch Goal

Make `add_tag` skip duplicates so calling it twice with the same normalized tag only stores one value.
