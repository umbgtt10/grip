// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;

use crate::collector::Collector;
use crate::config::Config;
use crate::grip_report::GripReport;
use crate::overall_stats::OverallStats;
use crate::reporter::Reporter;
use crate::scorer::Scorer;
use crate::walk::Walk;

#[derive(Debug)]
pub struct App {
    walker: Walk,
    scorer: Scorer,
    reporter: Reporter,
    config: Config,
}

impl App {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            walker: Walk::new(&config.path),
            scorer: Scorer::new(),
            reporter: Reporter::new(config.json),
            config,
        }
    }

    pub fn run(&self) -> Result<ExitCode> {
        let files = self.walker.rust_files()?;
        let mut indexed = Vec::with_capacity(files.len());
        for (path, source) in files {
            let module = self.module_from_path(&path);
            let counts = Collector::collect(&source);
            indexed.push((module, counts));
        }
        if indexed.is_empty() {
            return Err(anyhow::anyhow!(
                "no Rust source files found in {}",
                self.config.path.display()
            ));
        }
        let (overall_counts, modules) = self.scorer.agg_modules(indexed);
        let (grip_score, pure_ratio, public_ratio) = self.scorer.score_counts(&overall_counts);
        let overall = OverallStats {
            grip_score,
            public_items: overall_counts.public_items,
            total_functions: overall_counts.total_functions,
            pure_functions: overall_counts.pure_functions,
            pure_ratio,
            public_ratio,
        };
        let target = self
            .config
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_string();
        let report = GripReport {
            version: env!("CARGO_PKG_VERSION").to_string(),
            target,
            overall,
            modules: self.scorer.module_stats(modules),
        };
        if let Some(min) = self.config.min_score {
            return Ok(if grip_score >= min {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            });
        }
        self.reporter.write(&report)?;
        Ok(ExitCode::SUCCESS)
    }

    fn module_from_path(&self, path: &Path) -> String {
        let relative = path.strip_prefix(&self.config.path).unwrap_or(path);
        let s = relative.to_string_lossy().replace('\\', "/");
        let without_src = s.strip_prefix("src/").map(|s| s.to_string()).unwrap_or(s);
        if let Some(pos) = without_src.rfind('/') {
            without_src[..pos].to_string()
        } else {
            ".".to_string()
        }
    }
}
