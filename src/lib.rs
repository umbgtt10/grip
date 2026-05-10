// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod app;
pub mod args;
pub mod collector;
pub mod config;
pub mod grip_report;
pub mod item_counts;
pub mod module_stats;
pub mod overall_stats;
pub mod reporter;
pub mod scorer;
pub mod unsafe_finder;
pub mod walk;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<ExitCode> {
    let args = args::Args::parse();
    let config = config::Config::from_args(args);
    app::App::new(config).run()
}

pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = args::Args::parse_from_args(args);
    let config = config::Config::from_args(args);
    app::App::new(config).run()
}
