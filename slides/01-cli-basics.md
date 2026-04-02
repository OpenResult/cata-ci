# Step 1: CLI Basics

## Why This Step Exists

We need a working command-line interface before we can improve its structure.

This step stays intentionally simple:

- use `clap`
- define subcommands
- accept a few arguments
- let `--help` do useful work for us

## Starting Point

We want a fake tool named `kata-ci`.

Examples:

```text
kata-ci java 21
kata-ci check lint
kata-ci verify
```

## Core Idea

At this stage, parsing and behavior are still close together.

That is acceptable for a very small CLI. It becomes a problem only when the command surface grows.

## Example

```rust
#[derive(Debug, Parser)]
#[command(name = "kata-ci")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Java { version: Option<String> },
    Verify,
}
```

## Discussion Prompt

- What becomes awkward if we put all behavior directly behind these `clap` types?
- What parts already feel like “real domain logic” instead of “argument parsing”?

## Teaching Notes

- Show `cargo run -- --help`
- Show one happy path
- Point out how much behavior `clap` gives us with very little code

## Recap

- `clap` is a strong starting point for a Rust CLI
- nested subcommands are easy to express
- this is a good baseline, but not yet a good architecture
