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
        .join("clean_calc");
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
fn scores_good() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let grip_score = parsed["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        grip_score >= 80,
        "expected good score >= 80, got {}",
        grip_score
    );
    assert!(
        grip_score < 100,
        "expected imperfect score < 100, got {}",
        grip_score
    );
}

#[test]
fn has_many_public_items() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let public_items = parsed["overall"]["public_items"].as_u64().unwrap();
    assert!(
        public_items >= 10,
        "expected many public items, got {}",
        public_items
    );
}

#[test]
fn high_pure_ratio() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let pure_ratio = parsed["overall"]["pure_ratio"].as_f64().unwrap();
    assert!(
        pure_ratio >= 0.9,
        "expected high pure ratio, got {}",
        pure_ratio
    );
}

#[test]
fn scores_higher_than_sloppy() {
    // Arrange & Act
    let clean_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("clean_calc");
    let sloppy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sloppy_calc");
    let clean = analyze_at(&clean_path);
    let sloppy = analyze_at(&sloppy_path);

    // Assert
    let clean_score = clean["overall"]["grip_score"].as_u64().unwrap();
    let sloppy_score = sloppy["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        clean_score > sloppy_score + 30,
        "expected clean ({}) to be > sloppy ({}) + 30",
        clean_score,
        sloppy_score,
    );
}

fn analyze_at(fixture_path: &PathBuf) -> serde_json::Value {
    let config = Config {
        path: fixture_path.clone(),
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
