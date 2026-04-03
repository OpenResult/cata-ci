use kata_ci_solution_step_00::{
    add_tag, build_profile, into_profile_name, normalize_name, owner_label, parse_retry_limit,
    rename_profile, summarize, BuildProfile,
};

#[test]
fn normalize_name_returns_an_owned_string() {
    assert_eq!(normalize_name(" Release Candidate "), "release-candidate");
}

#[test]
fn owner_label_handles_missing_owner() {
    assert_eq!(owner_label(None), "owner: unassigned");
}

#[test]
fn add_tag_normalizes_and_deduplicates() {
    let mut tags = vec!["existing".to_string()];

    add_tag(&mut tags, " Nightly Build ");
    add_tag(&mut tags, "nightly build");

    assert_eq!(
        tags,
        vec!["existing".to_string(), "nightly-build".to_string()]
    );
}

#[test]
fn parse_retry_limit_accepts_small_numbers() {
    assert_eq!(parse_retry_limit("3"), Ok(3));
}

#[test]
fn parse_retry_limit_rejects_invalid_input() {
    assert_eq!(
        parse_retry_limit("abc"),
        Err("invalid retry limit: abc".to_string())
    );
}

#[test]
fn parse_retry_limit_rejects_large_values() {
    assert_eq!(
        parse_retry_limit("99"),
        Err("retry limit must be 10 or less: 99".to_string())
    );
}

#[test]
fn rename_profile_updates_the_existing_struct() {
    let mut profile = BuildProfile::new(
        "starter".to_string(),
        Some("alex".to_string()),
        vec!["pre-work".to_string()],
        2,
    );

    rename_profile(&mut profile, "Release Prep").unwrap();

    assert_eq!(profile.name, "release-prep");
}

#[test]
fn build_profile_returns_err_instead_of_panicking() {
    assert_eq!(
        build_profile("release", Some("alex"), "11"),
        Err("retry limit must be 10 or less: 11".to_string())
    );
}

#[test]
fn summarize_borrows_the_profile() {
    let profile = build_profile("Release Candidate", Some("alex"), "3").unwrap();

    let first = summarize(&profile);
    let second = summarize(&profile);

    assert_eq!(first, second);
    assert!(first.contains("release-candidate"));
}

#[test]
fn into_profile_name_consumes_the_profile_value() {
    let profile = build_profile("Release Candidate", Some("alex"), "3").unwrap();

    let name = into_profile_name(profile);

    assert_eq!(name, "release-candidate");
}
