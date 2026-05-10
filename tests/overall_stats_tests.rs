// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::overall_stats::OverallStats;

#[test]
fn overall_stats_serializes_to_json() {
    let stats = OverallStats {
        grip_score: 80,
        public_items: 10,
        total_functions: 20,
        pure_functions: 15,
        pure_ratio: 0.75,
        public_ratio: 0.5,
    };

    let json = serde_json::to_string(&stats).unwrap();

    assert!(json.contains("80"));
    assert!(json.contains("grip_score"));
}

#[test]
fn overall_stats_deserializes_from_json() {
    let json = r#"{"grip_score":60,"public_items":5,"total_functions":10,"pure_functions":5,"pure_ratio":0.5,"public_ratio":0.5}"#;

    let stats: OverallStats = serde_json::from_str(json).unwrap();

    assert_eq!(stats.grip_score, 60);
    assert_eq!(stats.total_functions, 10);
}
