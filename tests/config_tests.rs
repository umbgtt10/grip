// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::args::Args;
use grip::config::Config;

#[test]
fn from_args_preserves_path() {
    // Arrange
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "my-project"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.path.to_string_lossy(), "my-project");
}

#[test]
fn from_args_preserves_json() {
    // Arrange
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--json"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.json, true);
}

#[test]
fn from_args_preserves_threshold() {
    // Arrange
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--threshold", "42"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.threshold, Some(42));
}
