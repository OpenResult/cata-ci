# Repository Agent Instructions

## Repo Layout

- `slides/` contains presenter-facing Markdown decks.
- `labs/` contains participant lab instructions.
- `exercises/step-0N/` contains starter code for each workshop step.
- `solutions/step-0N/` contains the clean reference implementation for the same step.
- The root `Cargo.toml` defines the workspace and shared dependency versions.

## Build And Test Commands

Use these commands from the repository root:

```bash
cargo fmt --all
cargo build --workspace
cargo test --workspace
```

To run a specific step:

```bash
cargo run -p kata-ci-exercise-step-03 -- build all
cargo run -p kata-ci-solution-step-05 -- --describe verify
```

## Teaching Material Conventions

- Keep file names aligned across `slides/`, `labs/`, `exercises/`, and `solutions/`.
- Each slide deck should stay concise enough for live delivery.
- Each lab should include goal, tasks, acceptance criteria, hints, commands, expected behavior, and a stretch goal.
- The code examples shown in slides should match the actual Rust module names in the step crates.

## Starter Code Conventions

- Starter code should feel incomplete in the right places, but still be readable and structured.
- Prefer starter code that compiles and passes a small baseline test set.
- If an exercise intentionally leaves work for participants, explain that in the lab document instead of leaving vague placeholders.

## Reference Solution Conventions

- Solutions should be clean, minimal, and ready to demo.
- Do not leave `todo!`, placeholder comments, or dead code in solutions.
- Prefer small functions, explicit names, and approachable module structure over clever abstractions.

## Architecture Rules

- Prefer clarity over abstraction.
- Keep `clap` parsing types at the edge when a typed internal model exists.
- Keep fake execution deterministic and local. No real shelling out.
- Avoid async, macros beyond mainstream derive usage, and unnecessary generics.

## Documentation Rules

- Keep code and docs aligned.
- If implementation changes, update `slides/`, `labs/`, `README.md`, and `FACILITATOR_GUIDE.md` in the same change.
- If a command name or CLI shape changes, verify all examples still match the repo.
