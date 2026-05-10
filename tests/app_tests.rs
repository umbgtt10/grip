// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::process::ExitCode;

use grip::app::App;
use grip::config::Config;
use tempfile::TempDir;

#[test]
fn run_on_empty_dir_errors() {
    let dir = TempDir::new().unwrap();
    let config = Config {
        path: dir.path().to_path_buf(),
        json: false,
        min_score: None,
    };
    let app = App::new(config);

    let result = app.run();

    assert!(result.is_err());
}

#[test]
fn run_on_valid_dir_succeeds() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("lib.rs"),
        "pub fn greet() -> &'static str { \"hello\" }\n",
    )
    .unwrap();
    let config = Config {
        path: dir.path().to_path_buf(),
        json: false,
        min_score: None,
    };
    let app = App::new(config);

    let exit_code = app.run().unwrap();

    assert_eq!(exit_code, ExitCode::SUCCESS);
}
