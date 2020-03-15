extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn clock_out_with_help_args() {
    let mut cmd = Command::cargo_bin("miteras").unwrap();
    cmd.arg("clock-out")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock out with today's condition"));
}
