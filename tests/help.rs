extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

static CLI_DESCRIBE: &str = "A command-line tool for MITERAS.";

#[test]
fn no_args() {
    let mut cmd = Command::cargo_bin("miteras").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(CLI_DESCRIBE));
}

#[test]
fn with_help_args() {
    let mut cmd = Command::cargo_bin("miteras").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(CLI_DESCRIBE));
}
