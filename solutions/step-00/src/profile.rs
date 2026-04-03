use crate::model::BuildProfile;

pub fn normalize_name(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

pub fn owner_label(owner: Option<&str>) -> String {
    match owner.map(str::trim).filter(|value| !value.is_empty()) {
        Some(owner) => format!("owner: {owner}"),
        None => "owner: unassigned".to_string(),
    }
}

pub fn add_tag(tags: &mut Vec<String>, tag: &str) {
    let normalized = normalize_name(tag);

    if normalized.is_empty() {
        return;
    }

    if !tags.iter().any(|existing| existing == &normalized) {
        tags.push(normalized);
    }
}

pub fn rename_profile(profile: &mut BuildProfile, new_name: &str) -> Result<(), String> {
    profile.name = validate_name(new_name)?;
    Ok(())
}

pub fn parse_retry_limit(raw: &str) -> Result<u8, String> {
    let trimmed = raw.trim();
    let value = trimmed
        .parse::<u8>()
        .map_err(|_| format!("invalid retry limit: {trimmed}"))?;

    if value > 10 {
        return Err(format!("retry limit must be 10 or less: {value}"));
    }

    Ok(value)
}

pub fn build_profile(
    name: &str,
    owner: Option<&str>,
    retries: &str,
) -> Result<BuildProfile, String> {
    let name = validate_name(name)?;
    let retries = parse_retry_limit(retries)?;
    let owner = owner
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    let mut tags = vec!["pre-work".to_string()];
    add_tag(&mut tags, &name);

    Ok(BuildProfile::new(name, owner, tags, retries))
}

pub fn summarize(profile: &BuildProfile) -> String {
    let tags = if profile.tags.is_empty() {
        "no tags".to_string()
    } else {
        profile.tags.join(", ")
    };

    format!(
        "profile {} | {} | tags: {} | retries: {}",
        profile.name,
        owner_label(profile.owner.as_deref()),
        tags,
        profile.retries
    )
}

pub fn into_profile_name(profile: BuildProfile) -> String {
    profile.name
}

fn validate_name(input: &str) -> Result<String, String> {
    let normalized = normalize_name(input);

    if normalized.is_empty() {
        return Err("profile name cannot be empty".to_string());
    }

    Ok(normalized)
}
