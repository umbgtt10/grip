// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::module_stats::ModuleStats;
use crate::overall_stats::OverallStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GripReport {
    pub version: String,
    pub target: String,
    pub overall: OverallStats,
    pub modules: Vec<ModuleStats>,
}
