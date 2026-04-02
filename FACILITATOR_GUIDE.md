# Facilitator Guide

## Suggested Timebox

- Intro and repo orientation: 15 min
- Step 1, CLI basics: 25 min
- Step 2, typed command model: 25 min
- Step 3, planner and plan: 30 min
- Step 4, runner and dry-run: 30 min
- Step 5, describe, config, and tests: 35 min
- Wrap-up: 20 min

## Teaching Throughline

The workshop should keep returning to one idea: each architectural layer exists to make the next change easier.

- Step 1 shows how quickly a CLI can become crowded.
- Step 2 introduces a typed internal command boundary.
- Step 3 introduces planning as a separate concern.
- Step 4 separates execution strategy from planning.
- Step 5 shows that rendering and testing become easier once the design is decomposed.

## Likely Stumbling Points

### Step 1

- `clap` derive syntax
- nested subcommands
- optional positional arguments such as `java [version]`

Recovery:

- remind participants that the derive macros remove a lot of boilerplate
- point them to the provided tests for concrete examples

### Step 2

- deciding what belongs in `cli.rs` versus `command.rs`
- naming internal enums cleanly

Recovery:

- ask “would you want the rest of the app to depend on `clap`?”
- encourage a small, explicit `Command` enum instead of generic wrappers

### Step 3

- deciding how detailed a `Plan` should be
- over-modeling metadata

Recovery:

- keep the plan model intentionally small
- explain that metadata exists to support explainability and validation, not complexity

### Step 4

- mixing execution logic back into planning
- testing dry-run without calling the executor

Recovery:

- insist that dry-run is still about the same plan, only a different execution mode
- use a fake executor with recorded calls

### Step 5

- text vs JSON rendering boundaries
- config defaults versus explicit arguments
- when to write integration tests

Recovery:

- keep describer pure
- make config optional and tiny
- use one or two integration tests, not an entire second test strategy

## Talking Points

- `clap` is a parser, not your domain model.
- A `Command` expresses intent better than raw parser structs.
- A `Plan` gives you a stable object to inspect, render, test, and execute.
- Dry-run is easiest when execution is already separated behind a `Runner`.
- JSON describe is almost free once the plan is a serializable data structure.

## Expected Solutions

- Step 1: a working `clap` CLI with nested subcommands and readable output
- Step 2: a typed `Command` enum and mapping layer
- Step 3: a planner that returns metadata and deterministic steps
- Step 4: a runner with run and dry-run behavior plus fake executor tests
- Step 5: describe output in text and JSON, minimal config defaults, and integration tests

## Demo Suggestions

Step 1:

```bash
cargo run -p kata-ci-solution-step-01 -- --help
cargo run -p kata-ci-solution-step-01 -- check format
```

Step 2:

```bash
cargo run -p kata-ci-solution-step-02 -- build all
```

Then show how the typed command is printed rather than raw `clap` structs.

Step 3:

```bash
cargo run -p kata-ci-solution-step-03 -- verify
```

Then open `planner.rs` and `plan.rs` side by side.

Step 4:

```bash
cargo run -p kata-ci-solution-step-04 -- --dry-run image publish
cargo run -p kata-ci-solution-step-04 -- deploy sandbox
```

Step 5:

```bash
cargo run -p kata-ci-solution-step-05 -- --describe --format json build all
cargo run -p kata-ci-solution-step-05 -- --config solutions/step-05/kata-ci.toml java
```

## If Participants Get Stuck

- move them to the next locally stable milestone, not the complete solution
- show a smaller match expression or one enum variant instead of the whole file
- prefer asking “what should this layer own?” over correcting syntax immediately
- if needed, let them jump to the matching `solutions/step-0N/` directory and work backward
