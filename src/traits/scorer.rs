// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use crate::item_counts::ItemCounts;
use crate::module_stats::ModuleStats;

pub trait Scorer {
    fn score_counts(&self, counts: &ItemCounts) -> (u32, f64, f64);
    fn agg_modules(
        &self,
        files: Vec<(String, ItemCounts)>,
    ) -> (ItemCounts, BTreeMap<String, ItemCounts>);
    fn module_stats(&self, modules: BTreeMap<String, ItemCounts>) -> Vec<ModuleStats>;
}
