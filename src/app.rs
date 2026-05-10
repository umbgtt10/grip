// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;

use crate::cache::Cache;
use crate::collector::Collector;
use crate::config::Config;
use crate::grip_report::GripReport;
use crate::item_counts::ItemCounts;
use crate::overall_stats::OverallStats;
use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;
use crate::traits::walk::Walk;

#[derive(Debug)]
pub struct App<W: Walk, S: Scorer, R: Reporter> {
    walker: W,
    scorer: S,
    reporter: R,
    config: Config,
}

impl
    App<
        crate::fs_walk::FsWalk,
        crate::default_scorer::DefaultScorer,
        crate::stdout_reporter::StdoutReporter,
    >
{
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            walker: crate::fs_walk::FsWalk::new(&config.path),
            scorer: crate::default_scorer::DefaultScorer::new(),
            reporter: crate::stdout_reporter::StdoutReporter::new(config.json),
            config,
        }
    }
}

impl<W: Walk, S: Scorer, R: Reporter> App<W, S, R> {
    #[must_use]
    pub fn with_deps(walker: W, scorer: S, reporter: R, config: Config) -> Self {
        Self {
            walker,
            scorer,
            reporter,
            config,
        }
    }

    #[must_use]
    pub fn reporter(&self) -> &R {
        &self.reporter
    }

    pub fn run(&self) -> Result<ExitCode> {
        let mut cache = Cache::new(&self.config.path);
        let indexed = self.collect_files(&mut cache)?;
        cache.flush();

        if indexed.is_empty() {
            return Err(anyhow::anyhow!(
                "no Rust source files found in {}",
                self.config.path.display()
            ));
        }

        let report = self.compute_report(indexed);
        self.handle_output(&report)
    }

    fn collect_files(&self, cache: &mut Cache) -> Result<Vec<(String, ItemCounts)>> {
        let files = self.walker.rust_files()?;
        let mut indexed = Vec::with_capacity(files.len());
        for (path, source) in files {
            let module = self.module_from_path(&path);
            let counts = if let Some(cached) = cache.get(&path) {
                cached
            } else {
                let counts = Collector::collect(&source);
                cache.set(&path, &source, &counts);
                counts
            };
            indexed.push((module, counts));
        }
        Ok(indexed)
    }

    fn compute_report(&self, indexed: Vec<(String, ItemCounts)>) -> GripReport {
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
        let module_stats = self.scorer.module_stats(modules);
        let offender_threshold = self.config.threshold.unwrap_or(50);
        let offenders = module_stats
            .iter()
            .filter(|m| m.grip_score < offender_threshold)
            .map(|m| crate::offender::Offender {
                path: m.path.clone(),
                grip_score: m.grip_score,
            })
            .collect();
        GripReport {
            version: env!("CARGO_PKG_VERSION").to_string(),
            target,
            overall,
            modules: module_stats,
            offenders,
            offender_threshold,
        }
    }

    fn handle_output(&self, report: &GripReport) -> Result<ExitCode> {
        if let Some(min) = self.config.threshold {
            return Ok(if report.overall.grip_score >= min {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            });
        }
        self.reporter.write(report)?;
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
