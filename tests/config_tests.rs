// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::args::Args;
use grip::config::Config;

#[test]
fn from_args_preserves_path() {
    let args = Args::parse_from_args(vec!["cargo-grip", "my-project"]);
    let config = Config::from_args(args);

    assert_eq!(config.path.to_string_lossy(), "my-project");
}

#[test]
fn from_args_preserves_json() {
    let args = Args::parse_from_args(vec!["cargo-grip", "--json"]);
    let config = Config::from_args(args);

    assert_eq!(config.json, true);
}

#[test]
fn from_args_preserves_min_score() {
    let args = Args::parse_from_args(vec!["cargo-grip", "--min-score", "42"]);
    let config = Config::from_args(args);

    assert_eq!(config.min_score, Some(42));
}
