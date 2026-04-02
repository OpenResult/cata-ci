# Step 5: Describe, Config, And Tests

## Why This Final Step Exists

A good CLI should be inspectable as well as executable.

Now that we have a plan, we can render it in more than one way.

## Goal

Add:

- `--describe`
- `--format text|json`
- minimal config defaults
- unit and integration tests

## Example

```rust
pub enum OutputFormat {
    Text,
    Json,
}

pub fn describe(plan: &Plan, format: OutputFormat) -> Result<String, serde_json::Error> {
    match format {
        OutputFormat::Text => Ok(render_text(plan)),
        OutputFormat::Json => serde_json::to_string_pretty(plan),
    }
}
```

## Why This Is A Good Finish

- rendering stays separate from planning and running
- JSON output is almost free once the plan is serializable
- config can supply defaults without muddying the command model

## Discussion Prompt

- What tests are pure unit tests now?
- Which behaviors are worth one or two integration tests?

## Recap

- the architecture now supports run, dry-run, and describe
- the fake domain stays safe and deterministic
- the final shape is small enough to teach and strong enough to reuse
