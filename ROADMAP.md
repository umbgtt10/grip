# grip — Roadmap

**Crate:** `cargo-grip`  
**License:** MIT  
**Last updated:** 2026-05-10  
**Current status:** Phase 0 — ✅ Complete (v0.1.1 published)  

---

## Vision

`grip` measures how much tests can grab onto a Rust codebase.

Testability is the most important quality property of a software system. It is also
the least measured. Lines of code, cyclomatic complexity, and test coverage tell you
what exists and what was tested. None of them tell you how easy it is to test — how
many clean entry points exist, how many pure functions are available, how many trait
boundaries provide seams for test doubles, how many hidden dependencies are buried
in production logic making it impossible to test a function in isolation.

`grip` measures all of this. It produces a single score, a per-module breakdown,
and — across phases — a trend line across git history that turns testability from an
intuition into a measurement.

**The core question `grip` answers:**

*"How much can tests grab onto this codebase?"*

High grip: many pure functions, many trait boundaries, few hidden dependencies,
clear public surface. Tests can reach every behavior without heroic mocking,
global state manipulation, or test-only code paths.

Low grip: logic buried behind concrete types, hidden I/O, side effects mixed with
computation, no seams for test doubles. Tests either cannot reach the behavior at
all, or require the kind of setup that makes the test harder to understand than the
code it covers.

---

## Final target

A production-grade Rust static analysis tool that:

- Measures testability across four dimensions: public surface, pure function density,
  trait boundary ratio, and hidden dependency density
- Produces a single grip score (0–100) with a per-module breakdown
- Computes the Testability Index when combined with `braintax` output:
  `TI = grip / braintax`
- Tracks grip score across git history, producing a trend line that shows whether
  testability is improving, stable, or degrading as the codebase grows
- Emits structured output (JSON, SARIF, human-readable) suitable for CI integration,
  editor tooling, and downstream dashboards
- Runs in under 5 seconds on a 100K LOC codebase

---

## Design constraints (non-negotiable)

| Constraint | Rationale |
|---|---|
| Static analysis only | No instrumentation, no test execution, no runtime dependency |
| `syn`-based AST walking | Same approach as `braintax` — consistent vocabulary, composable output |
| Per-file, per-module, per-crate granularity | Actionable at the level where refactoring happens |
| Structured output | JSON output from day one — downstream tools depend on it |
| Heuristic purity detection | Perfect purity detection requires type inference; heuristics are fast, good enough, and honest about their limitations |
| No false precision | Scores are indices, not percentages. `grip` measures direction and magnitude, not exact values. |

---

## What grip measures

### Dimension 1 — Public surface

The count of items that tests can reach without heroics.

- `pub fn` — callable from tests, benchmarks, integration test crates
- `pub struct` — constructible and inspectable from test code
- `pub trait` — implementable as a test double
- `pub enum` — matchable in test assertions
- `pub(crate)` — reachable from within-crate tests (counted separately)

A codebase with zero public surface cannot be tested at all. Every item added to
the public surface is a potential test entry point. The ratio of tested public items
to total public items is the test gap — measured by `test-gap-gate`, not `grip`.
`grip` measures what is reachable. `test-gap-gate` measures what is reached.

### Dimension 2 — Pure function density

The fraction of functions that are probably pure — deterministic, side-effect-free,
testable by calling them with inputs and asserting on outputs.

A function is classified as probably pure if:
- No `&mut self` or `&mut T` parameters
- Returns a non-`()` value
- Does not call known side-effectful functions (see anti-pattern list in Phase 2)
- Contains no `unsafe` blocks

This is a heuristic. It will produce false positives (functions classified as pure
that have hidden side effects) and false negatives (functions classified as impure
that are actually deterministic). The heuristic is useful directionally: a module
with 80% probably-pure functions has more grip than one with 20%, regardless of the
classification errors at the margin.

The limitation is stated explicitly in `grip`'s output. `grip` does not claim to
detect purity. It estimates it.

### Dimension 3 — Trait boundary ratio

The fraction of `impl` blocks that implement a trait rather than inherent methods.

```
trait boundary ratio = (impl Trait for Type) / (total impl blocks)
```

A trait boundary is a seam. It is a place where the caller can substitute a test
double — a mock, a stub, a fake — without modifying production code. High trait
boundary ratio means tests can control the dependencies of the unit under test.
Low trait boundary ratio means tests must either accept all real dependencies or
resort to global state manipulation.

This is Michael Feathers' seam concept made measurable. `grip` counts seams.

### Dimension 4 — Hidden dependency density (Phase 2)

The fraction of functions that contain hidden dependencies — inputs the test cannot
control because they are not parameters.

A hidden dependency is a call to an ambient source of non-determinism or side effect
from within a function that does not receive it as a parameter:

- `std::time::Instant::now()` — time is an input the test cannot set
- `rand::random()` — randomness is an input the test cannot seed
- `std::fs::*` — filesystem state the test cannot control cleanly
- `println!`, `eprintln!` — output the test cannot capture without global redirection
- `std::env::var()` — environment state the test cannot isolate
- `std::process::exit()` — uncatchable termination

Each hidden dependency reduces grip. A function with three hidden dependencies
requires the test to either accept non-determinism or set up global state — both
of which reduce the value of the test.

---

## Phase 0 — Public surface and pure function ratio

**Status:** ✅ Complete  
**Target:** 2 hours  
**Deliverable:** `cargo-grip` v0.1.1 on crates.io  

**The question Phase 0 answers:**

*"How much of this codebase is reachable by tests, and how much of what is
reachable is probably pure?"*

### Scope

- Walk all `.rs` files in the target directory recursively using `walkdir`
- Parse each file with `syn` using the `Visit` trait
- Collect per-file counts:
  - `pub fn` count
  - `pub(crate) fn` count
  - `pub struct` count
  - `pub trait` count
  - `pub enum` count
  - Probably-pure function count (heuristic: no `&mut`, non-`()` return)
  - Total function count (all `fn` items, not just `pub`)
- Aggregate per module (directory) and overall
- Compute:
  - Pure ratio: `probably_pure / total_fn`
  - Public ratio: `pub_items / total_items`
  - Phase 0 grip score: `(pure_ratio * 0.6 + public_ratio * 0.4) * 100`

### Output format

Human-readable default:

```
grip v0.1.0 — etheram-ibft
══════════════════════════════════════════════════════

Overall grip score:    71 / 100

Public surface:        142 items  (pub: 89, pub(crate): 53)
Total functions:       201
Probably pure:         127 / 201  (63.2%)

Per module:
  ibft/consensus       grip: 78   pure: 71.4%   pub: 18
  ibft/recovery        grip: 61   pure: 52.1%   pub: 9   ⚠️
  ibft/transport       grip: 83   pure: 78.9%   pub: 22
  ibft/timer           grip: 44   pure: 31.2%   pub: 6   ❌
  ibft/state           grip: 91   pure: 88.3%   pub: 31
```

JSON output (`--json` flag):

```json
{
  "version": "0.1.0",
  "target": "etheram-ibft",
  "overall": {
    "grip_score": 71,
    "public_items": 142,
    "total_functions": 201,
    "pure_functions": 127,
    "pure_ratio": 0.632,
    "public_ratio": 0.706
  },
  "modules": [
    {
      "path": "ibft/consensus",
      "grip_score": 78,
      "pure_ratio": 0.714,
      "public_items": 18
    }
  ]
}
```

### CLI interface

```
grip [OPTIONS] [PATH]

Arguments:
  [PATH]    Path to Rust workspace or crate root [default: .]

Options:
  --json          Emit JSON output
  --min-score N   Exit with non-zero if overall grip score < N (CI use)
  --module PATH   Restrict analysis to a specific module path
  -h, --help      Print help
  -V, --version   Print version
```

### Dependencies

```toml
[dependencies]
syn     = { version = "2", features = ["full", "visit"] }
walkdir = "2"
serde   = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Validation target

Run against `etheram-ibft` before publishing. The output must:
- Produce scores that agree with intuition about which modules are most and least testable
- Flag `ibft/timer` (or equivalent) as low-grip — it is known to have the most
  side-effectful logic
- Flag `ibft/state` (or equivalent) as high-grip — it is known to be mostly pure

If the scores contradict known intuition about the codebase, the metric is wrong
and must be adjusted before publishing.

### Gate

- Builds with `--release`
- Runs in under 2 seconds on `etheram-ibft`
- Produces valid JSON output under `--json`
- `--min-score 0` exits 0
- `--min-score 100` exits non-zero
- Published on crates.io as `grip` v0.1.0

---

## Phase 1 — Trait boundary ratio

**Status:** Planned  
**Target:** 4–6 hours  
**Depends on:** Phase 0 complete  
**Deliverable:** `grip` v0.2.0 on crates.io  

**The question Phase 1 adds:**

*"How many seams does this codebase expose for test doubles?"*

### Scope

- Extend the `syn` visitor to collect:
  - Total `impl` blocks per module
  - `impl Trait for Type` blocks per module (trait implementations)
  - `impl Type` blocks per module (inherent implementations)
- Compute trait boundary ratio: `trait_impls / total_impls`
- Update grip score formula:
  `grip = (pure_ratio * 0.4 + public_ratio * 0.3 + trait_ratio * 0.3) * 100`
- Add trait boundary ratio to human-readable and JSON output

### Output addition

```
Per module:
  ibft/consensus   grip: 81   pure: 71.4%   pub: 18   traits: 6/9  (66.7%)
  ibft/transport   grip: 74   pure: 78.9%   pub: 22   traits: 2/11 (18.2%)  ⚠️
```

The transport module having low trait ratio despite high purity is actionable:
the functions are pure but the dependencies are concrete. Refactoring toward
traits would raise grip without touching the logic.

### Gate

- Phase 0 gate conditions still pass
- Trait ratio appears in JSON output
- Score changes from Phase 0 are reasonable — no module should change by more
  than 20 points
- Published on crates.io as `grip` v0.2.0

---

## Phase 2 — Hidden dependency detection

**Status:** Planned  
**Target:** 6–8 hours  
**Depends on:** Phase 1 complete  
**Deliverable:** `grip` v0.3.0 on crates.io  

**The question Phase 2 adds:**

*"How many hidden, uncontrollable inputs does this codebase contain?"*

### Scope

Extend the `syn` visitor to detect calls to known hidden dependency sources within
non-test function bodies:

**Time:**
- `std::time::Instant::now()`
- `std::time::SystemTime::now()`
- `chrono::Utc::now()`
- `chrono::Local::now()`

**Randomness:**
- `rand::random()`
- `rand::thread_rng()`
- Any call path containing `::rng()` or `::random()`

**Filesystem:**
- `std::fs::read`
- `std::fs::write`
- `std::fs::File::open`
- `std::fs::File::create`

**Environment:**
- `std::env::var()`
- `std::env::args()`

**Output:**
- `println!`
- `eprintln!`
- `print!`
- `eprint!`

**Process:**
- `std::process::exit()`
- `std::process::abort()`

**Unsafe:**
- Any `unsafe` block within a non-`unsafe fn`

Each detected hidden dependency reduces the function's grip contribution. A function
with one hidden dependency contributes at 50% of its pure-function grip value.
A function with two or more contributes at 0%.

This is intentionally aggressive. A function with two hidden dependencies is
nearly untestable without global state manipulation. Zero contribution is correct.

### Per-function output (new)

Phase 2 introduces per-function detail output under `--verbose`:

```
grip v0.3.0 — etheram-ibft — verbose
══════════════════════════════════════════════════════

ibft/timer.rs
  fn schedule_round_timeout()    grip: 0    hidden: Instant::now(), thread::sleep()  ❌
  fn compute_timeout_ms()        grip: 100  pure, no hidden deps                     ✅
  fn reset_timer()               grip: 50   hidden: Instant::now()                   ⚠️
```

### Updated grip score formula

```
grip = (
  pure_ratio         * 0.30 +
  public_ratio       * 0.20 +
  trait_ratio        * 0.25 +
  clean_fn_ratio     * 0.25   ← new: functions with zero hidden dependencies
) * 100
```

### Gate

- Phase 1 gate conditions still pass
- At least one hidden dependency detected in `etheram-ibft` — known to exist
- No false positives in test files — hidden dependency detection must skip
  `#[cfg(test)]` blocks and `tests/` directories
- `--verbose` produces per-function output
- Published on crates.io as `grip` v0.3.0

---

## Phase 3 — Testability Index: `grip / braintax`

**Status:** Planned  
**Target:** 4–6 hours  
**Depends on:** Phase 2 complete, `braintax` v0.x JSON output stable  
**Deliverable:** `grip` v0.4.0 on crates.io  

**The question Phase 3 answers:**

*"How testable is this code per unit of cognitive complexity?"*

This is the Testability Index — the ratio that makes the metric actionable for
engineers and legible to managers.

### Scope

- Accept `braintax` JSON output as input via `--braintax PATH`
- Join on function/module path
- Compute per-function and per-module Testability Index:
  `TI = grip_score / braintax_score`
  where `braintax_score` is normalized to the same 0–100 range as grip
- Classify each function into one of four quadrants:

| Quadrant | Grip | Braintax | Meaning |
|---|---|---|---|
| ✅ Ideal | High | Low | Easy to understand, easy to test |
| ⚠️ Acceptable | High | High | Complex but testable — worth the complexity |
| ⚠️ Lazy | Low | Low | Simple but undertested — low-hanging fruit |
| ❌ Danger zone | Low | High | Complex AND hard to test — immediate refactoring priority |

- Produce a prioritized refactoring list: all danger-zone functions sorted by
  TI ascending — the functions that most urgently need structural improvement

### Output addition

```
grip v0.4.0 — etheram-ibft — Testability Index
══════════════════════════════════════════════════════

Overall TI:   1.34  ✅

Danger zone (refactor immediately):
  ibft/timer.rs::schedule_round_timeout    TI: 0.21   grip: 12   braintax: 58  ❌
  ibft/recovery.rs::import_recovered       TI: 0.34   grip: 21   braintax: 62  ❌
  ibft/consensus.rs::handle_view_change    TI: 0.51   grip: 38   braintax: 74  ⚠️

Ideal (protect these):
  ibft/state.rs::compute_quorum_threshold  TI: 4.20   grip: 84   braintax: 20  ✅
  ibft/state.rs::is_member                 TI: 6.50   grip: 91   braintax: 14  ✅
```

### Gate

- Phase 2 gate conditions still pass
- `--braintax` flag accepts valid `braintax` JSON output without error
- Danger zone list contains at least one function in `etheram-ibft` — known to exist
- Ideal list contains at least one function in `etheram-ibft` — known to exist
- TI values are stable across two consecutive runs on the same codebase
- Published on crates.io as `grip` v0.4.0

---

## Phase 4 — Git history tracking and QI trend

**Status:** Planned  
**Target:** 8–12 hours  
**Depends on:** Phase 3 complete  
**Deliverable:** `grip` v1.0.0 on crates.io  

**The question Phase 4 answers:**

*"Is testability improving, stable, or degrading as this codebase grows — and what
is the financial cost of the current trajectory?"*

This is the Quality Index — the three-dimensional metric that makes death marches
visible in currency before they become inevitable.

### Scope

**Git history walking:**
- Accept `--history` flag to enable git history mode
- Walk all commits on the current branch using `git2` crate
- Compute grip score at each commit (or at configurable intervals — `--every N`)
- Store results in a local `.grip-history.json` file (incremental — only recomputes
  commits not already in the cache)

**Code size tracking:**
- Count productive LOC at each commit (excluding test files, blank lines, comments)
- Store alongside grip score

**QI computation:**
- `QI = grip_score / (braintax_score × normalized_size)`
- `normalized_size = LOC / 1000` (per KLOC normalization)
- QI is meaningful only when `braintax` history is also available — gracefully
  degrades to grip-only trend when `--braintax-history` is not provided

**Trend analysis:**
- Compute the QI derivative over the last N commits (configurable, default 10)
- Classify trend: `Improving`, `Stable` (±5%), `Degrading`
- Detect the inflection point — the commit where QI began declining
- Report the commit hash, author, date, and message of the inflection point

**Cost projection:**
- Accept `--team-size N` and `--daily-rate R` flags
- Compute estimated butchering duration from current QI deficit to asymptote:
  `estimated_days = (asymptote_QI - current_QI) / recovery_rate_per_day`
  where `recovery_rate_per_day` is estimated from the historical recovery rate
  in previous positive-derivative periods
- Compute cost: `butchering_cost = estimated_days × team_size × daily_rate`
- Compute opportunity cost: `opportunity_cost = estimated_days × features_per_day × value_per_feature`
  where `features_per_day` and `value_per_feature` are configurable inputs
- Compute early intervention cost: cost if addressed in the current sprint
  (assumes 2-week butchering at current team size)

### Output addition

```
grip v1.0.0 — etheram-ibft — Quality Index trend
══════════════════════════════════════════════════════

QI today:          1.34  (↓ from 1.87 at peak — 28 Jan 2026)
QI trend:          Degrading  (−0.12 / week over last 10 commits)
Asymptote (est.):  2.10

Inflection point:
  Commit:  a4f3c21
  Date:    2026-03-14
  Author:  Umberto
  Message: "feat: add recovery path for late-joining validators"

Cost projection (team: 5, rate: CHF 1,200/day):
  Estimated butchering duration:    9 weeks
  Butchering cost:                  CHF 270,000
  Opportunity cost (est.):          CHF 405,000
  ─────────────────────────────────────────────
  Total cost of current trajectory: CHF 675,000

  If addressed this sprint (2 weeks): CHF 60,000
  You are waiting:                    CHF 10,500 / day
```

**SARIF output (`--sarif` flag):**

Produce a SARIF file for IDE and CI integration. Each degrading module is a
diagnostic. Each danger-zone function is a warning. Each hidden dependency is
an informational note. Compatible with GitHub Code Scanning, VS Code, and Zed.

### Gate

- Phase 3 gate conditions still pass
- `--history` completes without error on `etheram-ibft` git history
- Inflection point detection identifies the correct commit (manually verified)
- Cost projection output is correct given known team size and rate inputs
- SARIF output is valid per the SARIF 2.1.0 schema
- Runs in under 30 seconds for a 500-commit history
- Published on crates.io as `grip` v1.0.0

---

## Timeline summary

| Phase | Deliverable | Key addition | Target | Status |
|---|---|---|---|---|
| 0 | v0.1.1 | Public surface + pure function ratio | 2 hours | ✅ Complete |
| 1 | v0.2.0 | Trait boundary ratio | 4–6 hours | Planned |
| 2 | v0.3.0 | Hidden dependency detection | 6–8 hours | Planned |
| 3 | v0.4.0 | Testability Index (`grip / braintax`) | 4–6 hours | Planned |
| 4 | v1.0.0 | Git history, QI trend, cost projection | 8–12 hours | Planned |

---

## Publication readiness checklist (v1.0.0)

- [ ] All four dimensions implemented and validated against `etheram-ibft`
- [ ] JSON output stable and versioned
- [ ] SARIF output valid per schema
- [ ] `--history` completes on at least one real project with 100+ commits
- [ ] Cost projection output verified against known team/rate inputs
- [ ] README written for a non-Rust audience — managers must understand the output
- [ ] CRAP score 0 across all `grip` source files (enforced by `crap4rust`)
- [ ] `braintax` integration documented with example workflow
- [ ] crates.io metadata complete

---

## Hard rules

- Every phase runs against `etheram-ibft` before publishing — intuition validation
  is mandatory, not optional
- Scores must be explainable: every number in the output must link to a concrete,
  named code artifact
- Heuristics are documented as heuristics — `grip` never claims more precision than
  it has
- JSON output is never broken between minor versions — downstream tools depend on it
- CRAP score 0 before any phase is declared complete
- No false positives in test files — `#[cfg(test)]` and `tests/` directories are
  always excluded from productive code analysis
