use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_mentions_check_and_verify() {
    Command::cargo_bin("kata-ci-exercise-step-01")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("check"))
        .stdout(contains("verify"));
}
