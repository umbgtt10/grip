// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::Write;

use grip::collector::Collector;
use tempfile::TempDir;

fn write_file(dir: &TempDir, name: &str, contents: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let mut fh = File::create(&path).unwrap();
    fh.write_all(contents.as_bytes()).unwrap();
    path
}

#[test]
fn pure_function_is_counted() {
    // Arrange
    let source = "pub fn add(a: i32, b: i32) -> i32 { a + b }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 1);
    assert_eq!(counts.public_functions, 1);
    assert_eq!(counts.total_items, 1);
    assert_eq!(counts.public_items, 1);
}

#[test]
fn impure_function_is_not_counted_as_pure() {
    // Arrange
    let source = "pub fn impure(x: &mut i32) { *x += 1; }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn unit_return_is_not_pure() {
    // Arrange
    let source = "pub fn side_effect() { println!(\"hello\"); }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn unsafe_function_is_not_pure() {
    // Arrange
    let source = "pub fn raw() -> i32 { unsafe { 42 } }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn private_function_is_not_public() {
    // Arrange
    let source = "fn private() -> i32 { 42 }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.public_functions, 0);
    assert_eq!(counts.public_items, 0);
}

#[test]
fn mixed_items_are_counted() {
    // Arrange
    let source = r#"
pub fn a() -> i32 { 1 }
pub fn b(x: &mut i32) { }
fn c() -> i32 { 2 }
pub struct S;
pub enum E {}
pub trait T {}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 3);
    assert_eq!(counts.public_functions, 2);
    assert_eq!(counts.pure_functions, 2);
    assert_eq!(counts.public_structs, 1);
    assert_eq!(counts.public_enums, 1);
    assert_eq!(counts.public_traits, 1);
    assert_eq!(counts.total_items, 6);
    assert_eq!(counts.public_items, 5);
}

#[test]
fn test_attribute_is_skipped() {
    // Arrange
    let source = r#"
#[cfg(test)]
mod tests {
    pub fn test_helper() -> i32 { 42 }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 0);
    assert_eq!(counts.public_items, 0);
}

#[test]
fn pubcrate_is_public_item() {
    // Arrange
    let source = "pub(crate) fn internal() -> i32 { 42 }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pubcrate_functions, 1);
    assert_eq!(counts.public_items, 1);
}
