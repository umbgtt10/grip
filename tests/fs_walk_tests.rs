// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;

use grip::fs_walk::FsWalk;
use grip::traits::walk::Walk;
use tempfile::TempDir;

#[test]
fn finds_rust_files() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("main.rs"), "fn main() {}").unwrap();

    // Act
    let walk = FsWalk::new(dir.path());
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}

#[test]
fn skips_tests_directory() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    let tests = dir.path().join("tests");
    fs::create_dir_all(&src).unwrap();
    fs::create_dir_all(&tests).unwrap();
    fs::write(src.join("lib.rs"), "").unwrap();
    fs::write(tests.join("integration.rs"), "").unwrap();

    // Act
    let walk = FsWalk::new(dir.path());
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
    assert!(files[0].0.ends_with("lib.rs"));
}

#[test]
fn skips_examples_and_benches() {
    // Arrange
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("examples")).unwrap();
    fs::create_dir_all(dir.path().join("benches")).unwrap();
    fs::write(dir.path().join("examples/demo.rs"), "").unwrap();
    fs::write(dir.path().join("benches/bench.rs"), "").unwrap();

    // Act
    let walk = FsWalk::new(dir.path());
    let files = walk.rust_files().unwrap();

    // Assert
    assert!(files.is_empty());
}

#[test]
fn skips_build_rs() {
    // Arrange
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("build.rs"), "").unwrap();

    // Act
    let walk = FsWalk::new(dir.path());
    let files = walk.rust_files().unwrap();

    // Assert
    assert!(files.is_empty());
}
