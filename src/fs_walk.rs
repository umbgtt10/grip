// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::traits::walk::Walk;

#[derive(Debug, Clone)]
pub struct FsWalk {
    root: PathBuf,
}

impl FsWalk {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { root: path.into() }
    }
}

impl Walk for FsWalk {
    fn rust_files(&self) -> Result<Vec<(PathBuf, String)>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension() == Some("rs".as_ref()))
        {
            let path = entry.path();
            if self.is_excluded(path) {
                continue;
            }
            let source = fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            files.push((path.to_path_buf(), source));
        }
        Ok(files)
    }
}

impl FsWalk {
    fn is_excluded(&self, path: &Path) -> bool {
        let normalized = path.to_string_lossy().replace('\\', "/");
        if normalized.contains("/target/") || normalized.contains("/.git/") {
            return true;
        }
        let relative = match path.strip_prefix(&self.root) {
            Ok(r) => r.to_string_lossy().replace('\\', "/"),
            Err(_) => return false,
        };
        relative.starts_with("tests/")
            || relative.starts_with("examples/")
            || relative.starts_with("benches/")
            || relative == "build.rs"
    }
}
