// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::args::Args;

#[test]
fn default_path_is_dot() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust"]);

    // Assert
    assert_eq!(args.path.to_string_lossy(), ".");
}

#[test]
fn json_flag_is_false_by_default() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust"]);

    // Assert
    assert_eq!(args.json, false);
}

#[test]
fn threshold_is_none_by_default() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust"]);

    // Assert
    assert_eq!(args.threshold, None);
}

#[test]
fn path_arg_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "some/path"]);

    // Assert
    assert_eq!(args.path.to_string_lossy(), "some/path");
}

#[test]
fn json_flag_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--json"]);

    // Assert
    assert_eq!(args.json, true);
}

#[test]
fn threshold_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--threshold", "50"]);

    // Assert
    assert_eq!(args.threshold, Some(50));
}

#[test]
fn min_score_alias_still_works() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--min-score", "30"]);

    // Assert
    assert_eq!(args.threshold, Some(30));
}
