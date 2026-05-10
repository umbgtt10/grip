// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::process::ExitCode;

use grip::run_from_args;
use tempfile::TempDir;

fn write_project(dir: &TempDir, source: &str) {
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), source).unwrap();
}

#[test]
fn run_from_args_empty_dir_errors() {
    let dir = TempDir::new().unwrap();

    let result = run_from_args(vec!["cargo-grip", &dir.path().to_string_lossy()]);

    assert!(result.is_err());
}

#[test]
fn run_from_args_valid_dir_succeeds() {
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    let result = run_from_args(vec!["cargo-grip", &dir.path().to_string_lossy()]);

    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}

#[test]
fn run_from_args_min_score_passes() {
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    let result = run_from_args(vec![
        "cargo-grip",
        &dir.path().to_string_lossy(),
        "--min-score",
        "0",
    ]);

    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}

#[test]
fn run_from_args_min_score_fails() {
    let dir = TempDir::new().unwrap();
    write_project(&dir, "fn greet() -> &'static str { \"hello\" }\n");

    let result = run_from_args(vec![
        "cargo-grip",
        &dir.path().to_string_lossy(),
        "--min-score",
        "100",
    ]);

    assert_eq!(result.unwrap(), ExitCode::FAILURE);
}
