// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use crate::item_counts::ItemCounts;
use crate::module_stats::ModuleStats;

#[derive(Debug, Clone, Default)]
pub struct Scorer;

impl Scorer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn score_counts(&self, counts: &ItemCounts) -> (u32, f64, f64) {
        let pure_ratio = if counts.total_functions > 0 {
            counts.pure_functions as f64 / counts.total_functions as f64
        } else {
            0.0
        };
        let public_ratio = if counts.total_items > 0 {
            counts.public_items as f64 / counts.total_items as f64
        } else {
            0.0
        };
        let grip = ((pure_ratio * 0.6 + public_ratio * 0.4) * 100.0).round() as u32;
        (grip, pure_ratio, public_ratio)
    }

    #[must_use]
    pub fn agg_modules(
        &self,
        files: Vec<(String, ItemCounts)>,
    ) -> (ItemCounts, BTreeMap<String, ItemCounts>) {
        let mut overall = ItemCounts::default();
        let mut modules: BTreeMap<String, ItemCounts> = BTreeMap::new();
        for (module, counts) in files {
            overall = overall.merged(&counts);
            modules
                .entry(module)
                .and_modify(|existing| *existing = existing.clone().merged(&counts))
                .or_insert(counts);
        }
        (overall, modules)
    }

    #[must_use]
    pub fn module_stats(&self, modules: BTreeMap<String, ItemCounts>) -> Vec<ModuleStats> {
        modules
            .into_iter()
            .map(|(path, counts)| {
                let (grip_score, pure_ratio, public_ratio) = self.score_counts(&counts);
                ModuleStats {
                    path,
                    grip_score,
                    pure_ratio,
                    public_items: counts.public_items,
                    total_functions: counts.total_functions,
                    pure_functions: counts.pure_functions,
                    public_ratio,
                }
            })
            .collect()
    }
}
