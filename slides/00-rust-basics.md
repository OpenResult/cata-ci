# Step 00: Rust Basics

## Why This Exists

This step is optional pre-work for participants who are new enough to Rust that ownership and error handling would otherwise distract from the CLI architecture workshop.

The goal is not to teach all of Rust.
The goal is to make `01 -> 05` easier.

## Owned Vs Borrowed Data

Rust makes us be explicit about who owns data and who only borrows it.

```rust
fn normalize_name(input: &str) -> String {
    input.trim().to_lowercase()
}
```

- `&str` means borrowed input
- `String` means owned output

## Move Vs Copy

Some values are copied cheaply.
Some values move.

```rust
let retries = 3_u8;
let again = retries; // Copy

let name = String::from("release");
let moved = name; // move
```

Why it matters:

- moves prevent accidental double ownership
- borrowing lets us read without consuming

## Borrowing By Reference

Use shared references when a function only needs to read.

```rust
fn summarize(profile: &BuildProfile) -> String {
    format!("profile {}", profile.name)
}
```

Why this is useful:

- the caller keeps ownership
- the same value can still be used afterward

## Mutable Borrowing

Use `&mut` when a function should update a value in place.

```rust
fn add_tag(tags: &mut Vec<String>, tag: &str) {
    tags.push(tag.to_string());
}
```

This is the simplest way to explain "pass by reference, but writable".

## Result Instead Of unwrap

`unwrap` is easy, but it turns bad input into a panic.

```rust
fn parse_retry_limit(raw: &str) -> Result<u8, String> {
    raw.parse::<u8>()
        .map_err(|_| format!("invalid retry limit: {raw}"))
}
```

Why this matters for real programs:

- invalid input is expected
- crashes are rarely the best API

## Propagating Errors With `?`

```rust
fn build_profile(name: &str, retries: &str) -> Result<BuildProfile, String> {
    let retries = parse_retry_limit(retries)?;
    Ok(BuildProfile::new(name.to_string(), None, vec![], retries))
}
```

`?` keeps error-handling readable without hiding failure.

## Discussion Prompt

- When should a function take ownership instead of borrowing?
- Where is `unwrap` acceptable in learning code, and where does it become a design smell?

## Recap

- `String` owns, `&str` borrows
- `&T` reads, `&mut T` mutates
- `Result` is better than panicking for expected failures
- these are exactly the skills needed before the CLI work starts
