// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::path::PathBuf;

use anyhow::Result;

use grip::app::App;
use grip::config::Config;
use grip::default_scorer::DefaultScorer;
use grip::fs_walk::FsWalk;
use grip::grip_report::GripReport;
use grip::traits::reporter::Reporter;

struct CaptureReporter {
    captured: RefCell<String>,
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &GripReport) -> Result<String> {
        let json = serde_json::to_string_pretty(report)?;
        *self.captured.borrow_mut() = json.clone();
        Ok(json)
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        let json = self.render(report)?;
        print!("{}", json);
        Ok(())
    }
}

fn analyze() -> serde_json::Value {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sloppy_calc");
    let config = Config {
        path: fixture_path,
        json: true,
        threshold: None,
    };
    let reporter = CaptureReporter {
        captured: RefCell::new(String::new()),
    };
    let app: App<FsWalk, DefaultScorer, CaptureReporter> = App::with_deps(
        FsWalk::new(&config.path),
        DefaultScorer::new(),
        reporter,
        config,
    );
    let _ = app.run().unwrap();
    let captured = app.reporter().captured.borrow().clone();
    serde_json::from_str(&captured).unwrap()
}

#[test]
fn scores_bad() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let grip_score = parsed["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        grip_score < 50,
        "expected bad score < 50, got {}",
        grip_score
    );
}

#[test]
fn has_few_public_items() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let public_items = parsed["overall"]["public_items"].as_u64().unwrap();
    assert!(
        public_items < 6,
        "expected few public items, got {}",
        public_items
    );
}

#[test]
fn low_pure_ratio() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let pure_ratio = parsed["overall"]["pure_ratio"].as_f64().unwrap();
    assert!(
        pure_ratio < 0.7,
        "expected low pure ratio, got {}",
        pure_ratio
    );
}
