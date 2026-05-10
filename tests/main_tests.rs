// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;

use assert_cmd::Command;
use tempfile::TempDir;

#[test]
fn binary_prints_error_on_empty_dir() {
    let dir = TempDir::new().unwrap();

    let assert = Command::cargo_bin("cargo-grip")
        .unwrap()
        .arg(dir.path())
        .assert();

    assert.failure();
}

#[test]
fn binary_prints_score_on_valid_dir() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("lib.rs"),
        "pub fn greet() -> &'static str { \"hello\" }\n",
    )
    .unwrap();

    let assert = Command::cargo_bin("cargo-grip")
        .unwrap()
        .arg(dir.path())
        .assert();

    assert
        .success()
        .stdout(predicates::str::contains("grip score"));
}
