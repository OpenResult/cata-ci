use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_mentions_describe() {
    Command::cargo_bin("kata-ci-exercise-step-05")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("--describe"));
}

#[test]
fn describe_json_outputs_valid_shape() {
    let output = Command::cargo_bin("kata-ci-exercise-step-05")
        .unwrap()
        .args(["--describe", "--format", "json", "build", "all"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"name\": \"build-all\""));
    assert!(stdout.contains("\"steps\""));
}
