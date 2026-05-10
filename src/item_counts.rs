// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCounts {
    pub total_functions: usize,
    pub pure_functions: usize,
    pub public_functions: usize,
    pub pubcrate_functions: usize,
    pub public_structs: usize,
    pub public_traits: usize,
    pub public_enums: usize,
    pub total_items: usize,
    pub public_items: usize,
}

impl ItemCounts {
    #[must_use]
    pub fn merged(self, other: &ItemCounts) -> Self {
        Self {
            total_functions: self.total_functions + other.total_functions,
            pure_functions: self.pure_functions + other.pure_functions,
            public_functions: self.public_functions + other.public_functions,
            pubcrate_functions: self.pubcrate_functions + other.pubcrate_functions,
            public_structs: self.public_structs + other.public_structs,
            public_traits: self.public_traits + other.public_traits,
            public_enums: self.public_enums + other.public_enums,
            total_items: self.total_items + other.total_items,
            public_items: self.public_items + other.public_items,
        }
    }
}
