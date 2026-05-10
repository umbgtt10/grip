// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-grip4rust", version = "0.1.3")]
#[command(about = "Measure Rust testability")]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub json: bool,

    #[arg(long, alias = "min-score")]
    pub threshold: Option<u32>,
}

impl Args {
    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }
}
