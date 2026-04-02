use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_mentions_describe_and_config() {
    Command::cargo_bin("kata-ci-solution-step-05")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("--describe"))
        .stdout(contains("--config"));
}

#[test]
fn describe_json_outputs_valid_shape() {
    let output = Command::cargo_bin("kata-ci-solution-step-05")
        .unwrap()
        .args(["--describe", "--format", "json", "build", "all"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"name\": \"build-all\""));
    assert!(stdout.contains("\"steps\""));
}

#[test]
fn config_file_changes_default_version() {
    let config_path = format!("{}/kata-ci.toml", env!("CARGO_MANIFEST_DIR"));

    Command::cargo_bin("kata-ci-solution-step-05")
        .unwrap()
        .args(["--describe", "--config", &config_path, "java"])
        .assert()
        .success()
        .stdout(contains("select Java 17"));
}
