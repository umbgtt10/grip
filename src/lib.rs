// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod app;
pub mod args;
pub mod cache;
pub mod collector;
pub mod config;
pub mod default_scorer;
pub mod fs_walk;
pub mod grip_report;
pub mod item_counts;
pub mod module_stats;
pub mod overall_stats;
pub mod stdout_reporter;
pub mod traits;
pub mod unsafe_finder;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::args::Args;
use crate::config::Config;

pub fn run() -> Result<ExitCode> {
    let args = Args::parse();
    let config = Config::from_args(args);
    App::new(config).run()
}

pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = Args::parse_from_args(args);
    let config = Config::from_args(args);
    App::new(config).run()
}
