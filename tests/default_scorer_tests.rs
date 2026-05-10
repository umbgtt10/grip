// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use grip::default_scorer::DefaultScorer;
use grip::item_counts::ItemCounts;
use grip::traits::scorer::Scorer;

fn scorer() -> DefaultScorer {
    DefaultScorer::new()
}

#[test]
fn perfect_grip() {
    // Arrange
    let counts = ItemCounts {
        total_functions: 2,
        pure_functions: 2,
        public_functions: 2,
        public_structs: 1,
        total_items: 3,
        public_items: 3,
        ..Default::default()
    };

    // Act
    let (score, pure, public) = scorer().score_counts(&counts);

    // Assert
    assert_eq!(score, 100);
    assert_eq!(pure, 1.0);
    assert_eq!(public, 1.0);
}

#[test]
fn zero_grip() {
    // Arrange
    let counts = ItemCounts {
        total_functions: 2,
        pure_functions: 0,
        total_items: 3,
        public_items: 0,
        ..Default::default()
    };

    // Act
    let (score, pure, public) = scorer().score_counts(&counts);

    // Assert
    assert_eq!(score, 0);
    assert_eq!(pure, 0.0);
    assert_eq!(public, 0.0);
}

#[test]
fn empty_module_gives_zero() {
    // Arrange
    let counts = ItemCounts::default();

    // Act
    let (score, pure, public) = scorer().score_counts(&counts);

    // Assert
    assert_eq!(score, 0);
    assert_eq!(pure, 0.0);
    assert_eq!(public, 0.0);
}

#[test]
fn module_aggregation() {
    // Arrange
    let files = vec![
        (
            "a".to_string(),
            ItemCounts {
                total_functions: 1,
                pure_functions: 1,
                public_items: 1,
                total_items: 1,
                ..Default::default()
            },
        ),
        (
            "b".to_string(),
            ItemCounts {
                total_functions: 1,
                pure_functions: 0,
                public_items: 0,
                total_items: 1,
                ..Default::default()
            },
        ),
    ];

    // Act
    let (overall, modules) = scorer().agg_modules(files);

    // Assert
    assert_eq!(overall.total_functions, 2);
    assert_eq!(overall.pure_functions, 1);
    assert_eq!(modules.len(), 2);
}

#[test]
fn module_stats_sorted() {
    // Arrange
    let mut map = BTreeMap::new();
    map.insert(
        "alpha".to_string(),
        ItemCounts {
            total_functions: 1,
            pure_functions: 1,
            public_items: 1,
            total_items: 1,
            ..Default::default()
        },
    );

    // Act
    let stats = scorer().module_stats(map);

    // Assert
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].path, "alpha");
    assert_eq!(stats[0].grip_score, 100);
}
