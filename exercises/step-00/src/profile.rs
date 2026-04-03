use crate::model::BuildProfile;

pub fn normalize_name(input: &str) -> String {
    input.trim().to_lowercase().replace(' ', "-")
}

pub fn owner_label(owner: Option<&str>) -> String {
    match owner {
        Some(owner) => format!("owner: {}", owner.trim()),
        None => "owner: unassigned".to_string(),
    }
}

pub fn add_tag(tags: &mut Vec<String>, tag: &str) {
    let normalized = normalize_name(tag);

    if !normalized.is_empty() {
        tags.push(normalized);
    }
}

pub fn rename_profile(profile: &mut BuildProfile, new_name: &str) -> Result<(), String> {
    let normalized = normalize_name(new_name);

    if normalized.is_empty() {
        return Err("profile name cannot be empty".to_string());
    }

    profile.name = normalized;
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
    let normalized_name = normalize_name(name);

    if normalized_name.is_empty() {
        return Err("profile name cannot be empty".to_string());
    }

    // Lab 00 intentionally leaves one panic-prone line in starter code.
    // Participants replace this with `?` so invalid input returns `Err`.
    let retries = parse_retry_limit(retries).unwrap();

    let mut tags = vec!["pre-work".to_string()];
    add_tag(&mut tags, &normalized_name);

    Ok(BuildProfile::new(
        normalized_name,
        owner.map(|value| value.trim().to_string()),
        tags,
        retries,
    ))
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
