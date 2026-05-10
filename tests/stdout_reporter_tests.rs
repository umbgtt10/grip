// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::grip_report::GripReport;
use grip::module_stats::ModuleStats;
use grip::overall_stats::OverallStats;
use grip::stdout_reporter::StdoutReporter;
use grip::traits::reporter::Reporter;

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
        offenders: vec![],
        offender_threshold: 50,
    }
}

fn reporter(json: bool) -> StdoutReporter {
    StdoutReporter::new(json)
}

#[test]
fn human_output_contains_score() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("71 / 100"));
}

#[test]
fn human_output_contains_module_lines() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn json_output_is_valid() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(true);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["overall"]["grip_score"], 71);
    assert!(parsed.get("modules").is_some());
}

#[test]
fn json_output_has_version() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(true);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["version"], "0.1.0");
}

#[test]
fn human_output_shows_offenders_section() {
    // Arrange
    let report = GripReport {
        modules: vec![ModuleStats {
            path: "bad_mod".to_string(),
            grip_score: 30,
            pure_ratio: 0.3,
            public_items: 1,
            total_functions: 5,
            pure_functions: 1,
            public_ratio: 0.2,
        }],
        offenders: vec![grip::offender::Offender {
            path: "bad_mod".to_string(),
            grip_score: 30,
        }],
        offender_threshold: 50,
        ..dummy_report()
    };
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("Offenders"));
    assert!(out.contains("bad_mod"));
}
