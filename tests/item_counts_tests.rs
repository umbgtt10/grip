// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::item_counts::ItemCounts;

fn full_counts() -> ItemCounts {
    ItemCounts {
        total_functions: 5,
        pure_functions: 3,
        public_functions: 2,
        pubcrate_functions: 1,
        public_structs: 1,
        public_traits: 1,
        public_enums: 1,
        total_items: 9,
        public_items: 6,
    }
}

#[test]
fn merge_adds_all_fields() {
    let a = full_counts();
    let b = full_counts();

    let merged = a.merged(&b);

    assert_eq!(merged.total_functions, 10);
    assert_eq!(merged.pure_functions, 6);
    assert_eq!(merged.public_functions, 4);
    assert_eq!(merged.pubcrate_functions, 2);
    assert_eq!(merged.public_structs, 2);
    assert_eq!(merged.public_traits, 2);
    assert_eq!(merged.public_enums, 2);
    assert_eq!(merged.total_items, 18);
    assert_eq!(merged.public_items, 12);
}

#[test]
fn merge_with_default_is_identity() {
    let a = full_counts();
    let empty = ItemCounts::default();

    let merged = a.merged(&empty);

    assert_eq!(merged.total_functions, 5);
    assert_eq!(merged.total_items, 9);
}
