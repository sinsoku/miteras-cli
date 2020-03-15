extern crate assert_cmd;
#[macro_use]
extern crate clap;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn with_version_args() {
    let mut cmd = Command::cargo_bin("miteras").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(format!("miteras {}\n", crate_version!()));
}
