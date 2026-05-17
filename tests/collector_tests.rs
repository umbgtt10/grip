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

#[test]
fn inherent_io_connect_is_impure() {
    // Arrange
    let source = r#"
use std::net::TcpStream;

pub struct Client;

impl Client {
    pub fn connect(&self) -> std::io::Result<()> {
        TcpStream::connect("127.0.0.1:8080")?;
        Ok(())
    }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.inherent_methods, 1);
    assert_eq!(counts.inherent_impure, 1);
}

#[test]
fn inherent_io_writeln_is_impure() {
    // Arrange
    let source = r#"
use std::fs::File;

pub struct Logger;

impl Logger {
    pub fn log(&self) -> std::io::Result<()> {
        let mut f = File::create("/tmp/log.txt")?;
        writeln!(f, "hello")?;
        Ok(())
    }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.inherent_methods, 1);
    assert_eq!(counts.inherent_impure, 1);
}

#[test]
fn local_trait_impl_is_not_foreign() {
    // Arrange
    let source = r#"
mod inner {
    pub trait MyTrait {
        fn do_thing(&self) -> i32;
    }
}

struct MyStruct;

impl inner::MyTrait for MyStruct {
    fn do_thing(&self) -> i32 { 42 }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.local_trait_methods, 1);
    assert_eq!(counts.inherent_methods, 0);
}

#[test]
fn test_attr_is_skipped_in_local_trait_impl() {
    // Arrange
    let source = r#"
trait Helper {
    fn do_thing(&self) -> i32;
}

struct Impl;

impl Helper for Impl {
    fn do_thing(&self) -> i32 { 42 }

    #[test]
    fn test_helper(&self) -> i32 { 99 }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.local_trait_methods, 1, "test_helper should be skipped");
    assert_eq!(counts.total_functions, 1, "only do_thing should be counted");
}

#[test]
fn known_foreign_trait_is_excluded() {
    // Arrange
    let source = r#"
struct MyStruct;

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct")
    }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.local_trait_methods, 0);
    assert_eq!(counts.inherent_methods, 0);
}

#[test]
fn hidden_dep_uppercase_type_constructor() {
    // Arrange
    let source = r#"
struct Handler;
impl Handler {
    pub fn handle() { TcpStream::connect("127.0.0.1:8080").unwrap(); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "TcpStream::connect should be a hidden dep");
}

#[test]
fn hidden_dep_std_constructor_not_counted() {
    // Arrange
    let source = r#"
struct Builder;
impl Builder {
    pub fn build() -> String { String::new() }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "String::new should not be a hidden dep");
}

#[test]
fn hidden_dep_vec_new_not_counted() {
    // Arrange
    let source = r#"
struct Collector;
impl Collector {
    pub fn collect() -> Vec<i32> { Vec::new() }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "Vec::new should not be a hidden dep");
}

#[test]
fn hidden_dep_box_new_not_counted() {
    // Arrange
    let source = r#"
struct Wrapper;
impl Wrapper {
    pub fn wrap(x: i32) -> Box<i32> { Box::new(x) }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "Box::new should not be a hidden dep");
}

#[test]
fn hidden_dep_constructor_new_not_flagged() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn process() { MyDatabase::new("prod:5432"); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "MyDatabase::new constructs a concrete dependency");
}

#[test]
fn hidden_dep_behavioral_method_is_flagged() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn process() { MyDatabase::query("SELECT 1"); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "MyDatabase::query should be a hidden dep");
}

#[test]
fn hidden_dep_third_party_type_is_detected() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn charge() { StripeGateway::charge(100); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "StripeGateway::charge should be a hidden dep");
}

#[test]
fn hidden_dep_self_call_not_counted() {
    // Arrange
    let source = r#"
struct Factory;
impl Factory {
    pub fn create() -> Self { Self::new() }
    pub fn new() -> Self { Self }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "Self::new should not be a hidden dep");
}

#[test]
fn hidden_dep_macro_println_is_detected() {
    // Arrange
    let source = r#"
struct Logger;
impl Logger {
    pub fn log() { println!("hello"); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "println! should be a hidden dep");
}

#[test]
fn hidden_dep_multiple_calls_accumulate() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn run() {
        TcpStream::connect("127.0.0.1:8080").unwrap();
        MyDatabase::query("SELECT 1");
        File::create("/tmp/test.txt").unwrap();
    }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 3, "should detect all 3 concrete type calls");
}

#[test]
fn hidden_dep_zero_deps_on_clean_function() {
    // Arrange
    let source = r#"
struct Calc;
impl Calc {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "pure function should have 0 hidden deps");
}

#[test]
fn hidden_dep_self_field_trait_object_not_counted() {
    // Arrange
    let source = r#"
struct Service {
    db: Box<dyn Database>,
}
impl Service {
    pub fn query(&self, sql: &str) { self.db.query(sql); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "self.db on trait object should not be a hidden dep");
}

#[test]
fn hidden_dep_self_field_concrete_type_is_flagged() {
    // Arrange
    let source = r#"
struct DataStore {
    conn: String,
}
impl DataStore {
    pub fn query(&self, sql: &str) -> String { sql.to_string() }
}

struct Service {
    db: DataStore,
}
impl Service {
    pub fn run(&self) { self.db.query("SELECT 1"); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[1].hidden_deps, 1, "self.db on concrete DataStore should be flagged");
    assert_eq!(fns[0].hidden_deps, 0, "DataStore::query is a free impl method");
}

#[test]
fn hidden_dep_self_field_ref_dyn_not_counted() {
    // Arrange
    let source = r#"
struct Service<'a> {
    handler: &'a dyn Handler,
}
impl Service<'_> {
    pub fn handle(&self) { self.handler.process(); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "self.handler on &dyn Handler should not be a hidden dep");
}

#[test]
fn hidden_dep_free_function_is_detected() {
    // Arrange
    let source = "fn query_db() { Database::query(\"SELECT 1\"); }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "free function with concrete call should be flagged");
}

#[test]
fn hidden_dep_free_function_clean_not_flagged() {
    // Arrange
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "pure free function should have 0 hidden deps");
}

#[test]
fn hidden_dep_eprintln_is_detected() {
    // Arrange
    let source = r#"
struct Logger;
impl Logger {
    pub fn log() { eprintln!("error"); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "eprintln! should be a hidden dep");
}

#[test]
fn hidden_dep_arc_dyn_field_not_counted() {
    // Arrange
    let source = r#"
struct Service {
    db: Arc<dyn Database>,
}
impl Service {
    pub fn query(&self, sql: &str) { self.db.query(sql); }
}
"#;
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "self.db on Arc<dyn Database> should not be a hidden dep");
}

#[test]
fn hidden_dep_input_argument_not_counted() {
    // Arrange
    let source = "struct Service;\nimpl Service {\n    pub fn run(&self, db: &Database) { db.query(\"SELECT 1\"); }\n}\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "db.query on input argument should not be a hidden dep");
}

#[test]
fn hidden_dep_print_macro_is_detected() {
    // Arrange
    let source = "struct Logger;\nimpl Logger {\n    pub fn log() { print!(\"hello\"); }\n}\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "print! should be a hidden dep");
}

#[test]
fn hidden_dep_thread_sleep_is_detected() {
    // Arrange
    let source = "fn pause() { std::thread::sleep(std::time::Duration::from_secs(1)); }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "thread::sleep should be a hidden dep");
}

#[test]
fn hidden_dep_light_weight_vs_heavy() {
    // Arrange
    let source = "fn light() { println!(\"start\"); Instant::now(); }\n\
fn heavy() { Database::new(\"prod\"); StripeGateway::charge(100.0); }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    let light_contr = grip::contribution_schedule::contribution(fns[0].is_pure, fns[0].has_trait_seam, fns[0].dep_weight);
    let heavy_contr = grip::contribution_schedule::contribution(fns[1].is_pure, fns[1].has_trait_seam, fns[1].dep_weight);
    assert!(light_contr > 0.0, "light deps should have positive contribution, got {light_contr}");
    assert_eq!(heavy_contr, 0.0, "heavy deps should have zero contribution, got {heavy_contr}");
}

#[test]
fn hidden_dep_labels_are_recorded() {
    // Arrange
    let source = "fn run() { Database::new(\"prod\"); println!(\"done\"); }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(source, &_file);

    // Assert
    assert!(fns[0].hidden_dep_labels.contains(&"Database::new".to_string()), "should contain Database::new label");
    assert!(fns[0].hidden_dep_labels.contains(&"println".to_string()), "should contain println label");
}
