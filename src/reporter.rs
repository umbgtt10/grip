// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::{self, Write};

use anyhow::Result;

use crate::grip_report::GripReport;
use crate::module_stats::ModuleStats;

#[derive(Debug, Clone)]
pub struct Reporter {
    json: bool,
}

impl Reporter {
    #[must_use]
    pub fn new(json: bool) -> Self {
        Self { json }
    }

    pub fn render(&self, report: &GripReport) -> Result<String> {
        if self.json {
            Ok(serde_json::to_string_pretty(report)?)
        } else {
            Ok(self.render_human(report))
        }
    }

    pub fn write(&self, report: &GripReport) -> Result<()> {
        let out = self.render(report)?;
        io::stdout().write_all(out.as_bytes())?;
        io::stdout().write_all(b"\n")?;
        Ok(())
    }

    fn render_human(&self, report: &GripReport) -> String {
        let mut lines = Vec::new();
        let target = &report.target;
        lines.push(format!(
            "cargo-grip {} -- {}\n══════════════════════════════════════════════════════\n",
            report.version, target,
        ));

        let overall = &report.overall;
        lines.push(format!(
            "Overall grip score:    {} / 100",
            overall.grip_score
        ));
        lines.push(format!(
            "Public surface:        {} items",
            overall.public_items
        ));
        lines.push(format!(
            "Total functions:       {}",
            overall.total_functions
        ));
        lines.push(format!(
            "Probably pure:         {} / {}  ({:.1}%)",
            overall.pure_functions,
            overall.total_functions,
            overall.pure_ratio * 100.0
        ));

        lines.push("\nPer module:".to_string());
        for module in &report.modules {
            lines.push(self.render_module_line(module));
        }

        lines.join("\n")
    }

    fn render_module_line(&self, module: &ModuleStats) -> String {
        let marker = self.module_marker(module.grip_score);
        format!(
            "  {:<30}  grip: {:>3}   pure: {:>5.1}%   pub: {:>3}  {}",
            module.path,
            module.grip_score,
            module.pure_ratio * 100.0,
            module.public_items,
            marker,
        )
    }

    fn module_marker(&self, score: u32) -> &'static str {
        if score < 40 {
            "❌"
        } else if score < 70 {
            "⚠️"
        } else {
            ""
        }
    }
}
