use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_mentions_maven_and_format() {
    Command::cargo_bin("kata-ci-solution-step-01")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("maven"))
        .stdout(contains("verify"));
}

#[test]
fn running_format_prints_a_fake_action() {
    Command::cargo_bin("kata-ci-solution-step-01")
        .unwrap()
        .args(["check", "format"])
        .assert()
        .success()
        .stdout(contains("run fake format check"));
}
