// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::process::ExitCode;

use grip::app::run_from_args;
use tempfile::TempDir;

fn write_project(dir: &TempDir, source: &str) {
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), source).unwrap();
}

#[test]
fn run_from_args_empty_dir_errors() {
    // Arrange
    let dir = TempDir::new().unwrap();

    // Act
    let result = run_from_args(vec!["cargo-grip4rust", &dir.path().to_string_lossy()]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn run_from_args_valid_dir_succeeds() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let result = run_from_args(vec!["cargo-grip4rust", &dir.path().to_string_lossy()]);

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}

#[test]
fn run_from_args_threshold_passes() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let result = run_from_args(vec![
        "cargo-grip4rust",
        &dir.path().to_string_lossy(),
        "--threshold",
        "0",
    ]);

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}

#[test]
fn run_from_args_threshold_fails() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let result = run_from_args(vec![
        "cargo-grip4rust",
        &dir.path().to_string_lossy(),
        "--threshold",
        "100",
    ]);

    // Assert
    assert_eq!(result.unwrap(), ExitCode::FAILURE);
}
