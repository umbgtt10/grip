// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::item_counts::ItemCounts;

#[derive(Debug, Clone)]
pub struct Cache {
    cache_dir: PathBuf,
    store: HashMap<PathBuf, CachedEntry>,
    dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedEntry {
    mtime_secs: u128,
    len: u64,
    counts: ItemCounts,
}

impl Cache {
    pub fn new(root: &Path) -> Self {
        let cache_dir = root.join(".grip_cache");
        let store = if cache_dir.exists() {
            Self::load(&cache_dir).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Self {
            cache_dir,
            store,
            dirty: false,
        }
    }

    pub fn get(&self, path: &Path) -> Option<ItemCounts> {
        let entry = self.store.get(path)?;
        let metadata = fs::metadata(path).ok()?;
        let mtime = metadata.modified().ok()?;
        let mtime_secs = mtime.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_secs() as u128;
        if entry.mtime_secs == mtime_secs && entry.len == metadata.len() {
            Some(entry.counts.clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, path: &Path, source: &str, counts: &ItemCounts) {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return,
        };
        let mtime = match metadata.modified() {
            Ok(t) => t,
            Err(_) => return,
        };
        let mtime_secs = match mtime.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u128,
            Err(_) => return,
        };
        self.store.insert(
            path.to_path_buf(),
            CachedEntry {
                mtime_secs,
                len: source.len() as u64,
                counts: counts.clone(),
            },
        );
        self.dirty = true;
    }

    pub fn flush(&self) {
        if !self.dirty {
            return;
        }
        let _ = fs::create_dir_all(&self.cache_dir);
        if let Ok(json) = serde_json::to_string(&self.store) {
            let _ = fs::write(self.cache_dir.join("cache.json"), json);
        }
    }

    fn load(cache_dir: &Path) -> Result<HashMap<PathBuf, CachedEntry>> {
        let path = cache_dir.join("cache.json");
        let json = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&json)?)
    }
}
