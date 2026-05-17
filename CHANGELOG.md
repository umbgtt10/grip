# Changelog

All notable changes to `cargo-grip4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] — 2026-05-08

### Added
- Phase 1: Trait boundary ratio metric
- Method-level collection: `inherent_methods`, `local_trait_methods`, `trait_ratio`
  in `ItemCounts`, `ModuleStats`, `OverallStats`
- `IoCallFinder` — scans method bodies for I/O calls (TcpStream::connect,
  fs::write, writeln!, etc.) to detect impure methods lacking `&mut self`
- Known foreign trait exclusion list — `Display`, `Clone`, `Debug`, `Serialize`
  and 40+ other std/crate traits excluded from counting
- `#[test]` attribute skipping in both inherent and trait impl methods
- N/A display for modules with zero impl methods (clarifies vs 0.0%)
- `trait_check` fixture with 6 integration tests covering pure-inherent,
  impure-inherent, well-seamed, foreign-only, and mixed modules

### Changed
- Grip formula: `(pure_ratio * 0.6 + public_ratio * 0.4) * 100` →
  `(pure_ratio * 0.4 + public_ratio * 0.3 + trait_ratio * 0.3) * 100`
- `Scorer::score_counts` returns `(u32, f64, f64, f64)` — includes trait_ratio
- Human-readable output adds `Trait methods:` line and `traits:` column
- ModuleStats and OverallStats serialization includes new trait fields

### Fixed
- `has_mut_param` now detects `&mut self` receiver (was only checking typed
  parameters)
- Foreign trait detection covers multi-segment paths like `serde::Serialize`
  via last-segment check against known list
- Pure-function heuristic expanded with I/O call detection

## [0.1.4] - 2026-05-08

### Added
- Per-function tracking: `FunctionInfo` struct with `name`, `file`, `is_pure`, `is_public`
- `Collector::collect` returns `(ItemCounts, Vec<FunctionInfo>)` — per-function data alongside aggregates
- `GripReport.functions` field — all functions with file paths in JSON output
- `type CollectedFiles` alias for cleaner return types
- Private functions visible in JSON output (`is_public` field on each function entry)

### Changed
- Grip score simplified to `grip = pure_ratio × 100` (no public_ratio weighting)

---

## [0.1.3] — 2026-05-08

### Added
- `FunctionInfo` first version with per-function tracking
- Grip score formula refined

---

## [0.1.2] — 2026-05-08

### Added
- `Walk`, `Scorer`, `Reporter` traits in `src/traits/` — protocol dependency seams
- `FsWalk`, `DefaultScorer`, `StdoutReporter` concrete implementations behind traits
- `App::with_deps()` constructor for dependency injection (testing, alternate impls)
- Incremental file cache in `.grip_cache/cache.json` — re-parses only changed files
- `Cache` struct with mtime+size-based cache keys
- `cache_tests.rs` — hit and miss coverage

### Changed
- `visit_item` decomposed: 50-line match → 7-line dispatcher dispatching to `visit_fn`,
  `visit_struct`, `visit_trait`, `visit_enum`, `visit_mod`
- `App::run` decomposed: `collect_files`, `compute_report`, `handle_output` extracted
- `ItemCounts` derives `Serialize`, `Deserialize` for cache serialization

---

## [0.1.1] — 2026-05-08

### Added
- Phase 0: public surface + pure function ratio analysis
- CLI: `cargo grip [PATH]`, `--json`, `--threshold N`
- `#[cfg(test)]` and `#[cfg_attr(..., test)]` detection — test code excluded from analysis
- Purity heuristic: no `&mut` params, non-`()` return, no `unsafe` blocks
- JSON output with `--json` flag
- 46 integration tests across 13 source files, AAA-compliant
- `scripts/run_stage_1.ps1` and `scripts/run_stage_2.ps1` CI gates
- README with formula, score table, roadmap, and limitations
- ROADMAP with 4-phase plan

### Changed
- Struct consolidation: `Scorer`, `Reporter`, `App` as structs with methods
- `UnsafeFinder` extracted to own file
- Shortened qualifying paths (`args::Args` → `Args`, etc.)
- Crate renamed from `grip` to `cargo-grip`

### Fixed
- `cfg_attr` false positive — now checks `test` in token payload
- AAA blank-line separation across all tests

---

## [0.1.0] — 2026-05-08

### Added
- Initial publish on crates.io as `cargo-grip`
- Hello-world binary with cargo subcommand support
- `Cargo.toml` metadata, MIT license, README placeholder

[0.2.0]: https://github.com/umbgtt10/grip4rust/compare/v0.1.4...v0.2.0
[Unreleased]: https://github.com/umbgtt10/grip4rust/compare/v0.2.0...HEAD
[0.1.3]: https://github.com/umbgtt10/grip4rust/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/umbgtt10/grip4rust/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/umbgtt10/grip4rust/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/umbgtt10/grip4rust/releases/tag/v0.1.0
