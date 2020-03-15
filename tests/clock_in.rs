extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn clock_in_with_help_args() {
    let mut cmd = Command::cargo_bin("miteras").unwrap();
    cmd.arg("clock-in")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock in with today's condition"));
}
