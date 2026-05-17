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

fn fixture_path(name: &str) -> String {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
        .to_string_lossy()
        .to_string()
}

#[test]
fn injected_outperforms_monolith() {
    // Arrange & Act
    let clean = run_from_args(vec![
        "cargo-grip4rust",
        &fixture_path("dep_injected"),
        "--threshold",
        "70",
    ]);
    let mono = run_from_args(vec![
        "cargo-grip4rust",
        &fixture_path("dep_monolith"),
        "--threshold",
        "50",
    ]);

    // Assert
    assert_eq!(clean.unwrap(), ExitCode::SUCCESS, "injected should score >= 70");
    assert_ne!(mono.unwrap(), ExitCode::SUCCESS, "monolith should score < 50");
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
