# cargo-grip4rust

**How much can tests grab onto your Rust codebase?**

`cargo-grip4rust` is a static analysis tool that measures **testability** — how many clean entry points, pure functions, and trait seams a codebase exposes for testing. It produces a single grip score (0–100) with a per-module breakdown.

---

## The problem

Most engineering orgs measure test *coverage* — how much code was exercised. But coverage tells you nothing about *how hard it was to write those tests*. A codebase can have 95% coverage and still be a nightmare to test:

- Functions that smuggle hidden I/O (time, filesystem, randomness) as ambient dependencies
- Concrete types everywhere, no trait seams for test doubles
- Side effects mixed with computation, so you can't test logic without mocking the world
- Everything private — zero public surface for test entry points

`grip` measures the root cause, not the symptom.

---

## The formula (Phase 0)

```
pure_ratio   = probably_pure_functions / total_functions
public_ratio = public_items / total_items

grip = (pure_ratio × 0.6 + public_ratio × 0.4) × 100
```

**A function is classified as probably pure** when:
- No `&mut` parameters
- Returns a non-`()` value
- Contains no `unsafe` blocks

**A declaration is public** when it's marked `pub` or `pub(crate)` — reachable from test code.

> `grip` uses a heuristic for purity. It will produce false positives and false negatives. The heuristic is useful *directionally*: a module scoring 80 has more testability grip than one scoring 20. The limitation is stated explicitly — `grip` does not claim to detect purity, it estimates it.

---

## Installation

```sh
cargo install cargo-grip4rust
```

## Usage

```sh
cargo grip4rust [PATH]
```

**Arguments:**

| Argument | Description |
|---|---|
| `[PATH]` | Path to Rust crate or workspace root (default: `.`) |

**Options:**

| Option | Description |
|---|---|
| `--json` | Emit structured JSON output |
| `--threshold N` | Exit non-zero if overall grip score < N (CI gate). Alias: `--min-score` |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

---

## Output

```
cargo-grip4rust 0.1.3 -- my-crate
══════════════════════════════════════════════════════

Overall grip score:    71 / 100
Public surface:        10 items
Total functions:       20
Probably pure:         12 / 20  (60.0%)

Per module:
  consensus                      grip:  78   pure: 71.4%   pub:  5
  transport                      grip:  83   pure: 78.9%   pub:  3
  timer                          grip:  44   pure: 31.2%   pub:  1  ❌
  state                          grip:  91   pure: 88.3%   pub:  6

Offenders (score < 50):
  timer                          grip:  44  ❌
```

JSON output (`--json`) includes the full breakdown and `offenders` list for CI pipelines, dashboards, and editor tooling.

---

## What the score means

| Range | Meaning |
|---|---|
| 80–100 | **High grip.** Tests can reach most behavior through public surfaces and pure functions. |
| 50–79 | **Moderate grip.** Some modules are easy to test, others need refactoring. |
| 20–49 | **Low grip.** Most logic is hidden behind private items or mixed with side effects. |
| 0–19 | **Minimal grip.** The codebase resists testing at every level. |

---

## Roadmap

`grip` is being built in phases:

| Phase | What it adds | Version |
|---|---|---|
| **0** ✅ | Public surface + pure function ratio | v0.1.3 |
| 1 | Trait boundary ratio (seams) | v0.2.0 |
| 2 | Hidden dependency detection | v0.3.0 |
| 3 | Testability Index (`grip / braintax`) | v0.4.0 |
| 4 | Git history tracking + Quality Index trend | v1.0.0 |

See [`ROADMAP.md`](ROADMAP.md) for the full plan.

---

## Limitations

- **Purity is a heuristic.** `grip` classifies functions by signature patterns, not by type inference. It will make mistakes at the margin.
- **No runtime analysis.** `grip` never executes code, runs tests, or instruments builds.
- **No coverage data.** `grip` measures *testability*, not *testing*. Use a coverage tool alongside it.

---

## License

MIT — see [`LICENSE`](LICENSE).
