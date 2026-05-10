// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStats {
    pub grip_score: u32,
    pub public_items: usize,
    pub total_functions: usize,
    pub pure_functions: usize,
    pub pure_ratio: f64,
    pub public_ratio: f64,
}
