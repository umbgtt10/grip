// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::grip_report::GripReport;
use grip::overall_stats::OverallStats;

fn sample() -> GripReport {
    GripReport {
        version: "0.1.0".to_string(),
        target: "test".to_string(),
        overall: OverallStats {
            grip_score: 50,
            public_items: 1,
            total_functions: 2,
            pure_functions: 1,
            pure_ratio: 0.5,
            public_ratio: 0.5,
        },
        modules: vec![],
        offenders: vec![],
        offender_threshold: 50,
        functions: vec![],
    }
}

#[test]
fn report_serializes_to_json() {
    // Arrange
    let report = sample();

    // Act
    let json = serde_json::to_string(&report).unwrap();

    // Assert
    assert!(json.contains("grip_score"));
    assert!(json.contains("\"test\""));
}

#[test]
fn report_deserializes_from_json() {
    // Arrange
    let report = sample();
    let json = serde_json::to_string(&report).unwrap();

    // Act
    let parsed: GripReport = serde_json::from_str(&json).unwrap();

    // Assert
    assert_eq!(parsed.version, "0.1.0");
    assert_eq!(parsed.overall.grip_score, 50);
}
