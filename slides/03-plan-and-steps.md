# Step 3: Planner And Plan

## Why Add A Planner

Some commands are no longer single actions.

`verify` is really a small workflow.
`build all` is a sequence.

We need a place to decompose intent into steps.

## Goal

Turn a `Command` into a `Plan`.

The plan should include:

- a name
- a summary
- tags
- required tools
- required environment
- ordered steps

## Example

```rust
pub struct Plan {
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub steps: Vec<Step>,
}
```

## Why This Helps

- planning becomes testable without execution
- describe mode becomes possible later
- run and dry-run can share the same input

## Discussion Prompt

- How much metadata is enough for a teaching example?
- What is the difference between a `Command` and a `Plan`?

## Recap

- commands express intent
- plans express ordered work
- a planner gives us a clean seam between the two
