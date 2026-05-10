// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::grip_report::GripReport;
use grip::module_stats::ModuleStats;
use grip::overall_stats::OverallStats;
use grip::reporter::Reporter;

fn dummy_report() -> GripReport {
    GripReport {
        version: "0.1.0".to_string(),
        target: "my-crate".to_string(),
        overall: OverallStats {
            grip_score: 71,
            public_items: 10,
            total_functions: 20,
            pure_functions: 12,
            pure_ratio: 0.6,
            public_ratio: 0.5,
        },
        modules: vec![
            ModuleStats {
                path: "alpha".to_string(),
                grip_score: 80,
                pure_ratio: 0.8,
                public_items: 5,
                total_functions: 10,
                pure_functions: 8,
                public_ratio: 0.5,
            },
            ModuleStats {
                path: "beta".to_string(),
                grip_score: 50,
                pure_ratio: 0.4,
                public_items: 5,
                total_functions: 10,
                pure_functions: 4,
                public_ratio: 0.5,
            },
        ],
    }
}

fn reporter(json: bool) -> Reporter {
    Reporter::new(json)
}

#[test]
fn human_output_contains_score() {
    let report = dummy_report();
    let reporter = reporter(false);

    let out = reporter.render(&report).unwrap();

    assert!(out.contains("71 / 100"));
}

#[test]
fn human_output_contains_module_lines() {
    let report = dummy_report();
    let reporter = reporter(false);

    let out = reporter.render(&report).unwrap();

    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn json_output_is_valid() {
    let report = dummy_report();
    let reporter = reporter(true);

    let out = reporter.render(&report).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["overall"]["grip_score"], 71);
    assert!(parsed.get("modules").is_some());
}

#[test]
fn json_output_has_version() {
    let report = dummy_report();
    let reporter = reporter(true);

    let out = reporter.render(&report).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["version"], "0.1.0");
}
