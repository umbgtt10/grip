// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use anyhow::Result;

pub trait Walk {
    fn rust_files(&self) -> Result<Vec<(PathBuf, String)>>;
}
