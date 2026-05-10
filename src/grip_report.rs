// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::function_info::FunctionInfo;
use crate::module_stats::ModuleStats;
use crate::offender::Offender;
use crate::overall_stats::OverallStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GripReport {
    pub version: String,
    pub target: String,
    pub overall: OverallStats,
    pub modules: Vec<ModuleStats>,
    pub offenders: Vec<Offender>,
    pub offender_threshold: u32,
    pub functions: Vec<FunctionInfo>,
}
