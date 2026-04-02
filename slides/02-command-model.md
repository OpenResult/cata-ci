# Step 2: Typed Command Model

## Why Refactor Now

The CLI has become bigger than a toy.

If the rest of the program depends directly on parser structs, parsing concerns leak everywhere.

## Goal

Introduce a typed internal model that represents intent.

Examples:

- `Command::Verify`
- `Command::Build(BuildTarget::All)`
- `Command::Image(ImageCommand::Publish)`

## Example

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    SelectJava { version: Option<String> },
    Check(CheckKind),
    Verify,
    Build(BuildTarget),
}
```

## Why This Helps

- the rest of the app stops depending on `clap`
- tests can target command mapping directly
- later refactors become local instead of cross-cutting

## Discussion Prompt

- What should stay in `cli.rs`?
- What should move into `command.rs`?
- Is a typed `Command` worth it for a small program?

## Recap

- parser types are edge types
- internal commands should express domain intent
- explicit enums are usually clearer than generic containers
