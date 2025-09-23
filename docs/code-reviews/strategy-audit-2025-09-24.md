# Strategy Audit: Majority-Decided Top 5 Issues (Project-Scoped, No Claude CLI)

Date: 2025-09-24
Scope: time-tracker repository
Method: Performed 8 distinct strategy passes (performance, error-handling, API/UX, testing coverage, architecture/modularity, build/deps, security/validation, lint/tech-debt). Collected concrete evidence (file, line ranges, excerpts). Aggregated overlapping findings; prioritized by impact and confidence.

---

## Top 5 Issues

### 1) Performance: Avoid cloning ParseState on every line in parsing pipeline
- Category: performance
- Severity: high
- Confidence: 0.9
- Problem: `parse_content` uses a fold with `state.clone()` in multiple match arms (including errors). As inputs grow, this leads to repeated allocations and O(N²)-ish behavior on large files.
- Evidence:
  - src/parsing/parser.rs: 15–24, 39–56, 57–71
  ```rust
  // map + fold creates new state on each line
  let final_state = content
      .lines()
      .enumerate()
      .map(|(line_number, line)| ParsedLine { content: line.trim(), line_number: line_number + 1 })
      .fold(ParseState::default(), |state, line| {
          process_line(&line, &state, filter, file_name)
      });
  ```
  ```rust
  // cloned state in multiple branches
  Ok(LineType::Header(maybe_date)) => ParseState { current_date: maybe_date, ..state.clone() },
  Ok(LineType::Entry(entry)) if state.in_time_tracking_section() => {
      let mut new_state = state.clone();
      // push entry ...
      new_state
  }
  Err(error) => ParseState {
      errors: { let mut errors = state.errors.clone(); errors.push(...); errors },
      ..state.clone()
  }
  ```
- Suggested fix:
  - Convert to a mutable state loop and a `process_line_mut` helper that updates `&mut ParseState` in place (no cloning).
  - This eliminates repeated allocation and copying while preserving logic.

---

### 2) Build/Deps: devShell vs justfile tool drift (cargo-watch missing; rustup reduces reproducibility)
- Category: build-deps
- Severity: medium
- Confidence: 0.8
- Problem: The `justfile` references tools that aren’t in `flake.nix` devShell. `rustup` in devShell can cause drift vs pinned toolchains.
- Evidence:
  - justfile uses cargo-watch and llvm-cov:
  ```make
  check-w:
      cargo watch -c -x check

  test-coverage:
      cargo llvm-cov nextest
  ```
  - flake.nix devShell (no cargo-watch; llvm-cov commented as broken):
  ```nix
  buildInputs = with pkgs; [
    cargo-nextest
    rustup
    just
    python312
    uv
  ];
  ```
- Suggested fix:
  - Add `cargo-watch` to devShell.
  - Consider pinning toolchain via `rust-bin`/`fenix` (with clippy/rustfmt) instead of `rustup` for reproducibility.
  - If `cargo-llvm-cov` remains broken in your nixpkgs, gate `test-coverage` behind availability or document a fallback path.

---

### 3) UX Accuracy: Percentages may not sum to 100% (known TODO)
- Category: api-ux
- Severity: medium
- Confidence: 0.85
- Problem: Percentages shown can add up to more/less than 100% due to rounding.
- Evidence:
  - tests/acceptance/general.rs: 121–129
  ```rust
  .with_percentage("63")
  .taking("5h 00m")
  .expect_project("exercise")
  .with_percentage("38") // todo: sum of both percentages should be 100%
  ```
- Suggested fix:
  - Normalize rounding error: compute percentages for all but last category; set the last to `100 - sum(previous)`.
  - Alternatively, distribute rounding error to the largest bucket.

---

### 4) Error Handling: unwrap/expect in date range calculations can panic on edge cases
- Category: error-handling
- Severity: medium
- Confidence: 0.7
- Problem: Date helpers use `unwrap`/`expect`; extreme dates or unexpected calendar behavior could panic.
- Evidence:
  - src/domain/dates/range.rs: 204–210 (expects)
  ```rust
  let first_of_current_month = today
      .with_day(1)
      .expect("Failed to set day to 1st of current month");
  let last_day_of_previous_month = first_of_current_month
      .pred_opt()
      .expect("Failed to get previous day from first of month");
  ```
  - src/domain/dates/range.rs: 263–270 (unwraps)
  ```rust
  let first_day = date.with_day(1).unwrap().with_month(1).unwrap();
  let last_day = first_day
      .with_year(date.year() + 1)
      .unwrap()
      .pred_opt()
      .unwrap();
  ```
- Suggested fix:
  - Replace with checked flows (`if let` / `?` returning `ParseError`) where reachable by user input.
  - Keep panics only for unreachable states proven by earlier validation.

---

### 5) API/UX: `--limit` is a boolean mapping to fixed 90% threshold (not tunable)
- Category: api-ux
- Severity: medium
- Confidence: 0.75
- Problem: Users cannot choose a different threshold.
- Evidence:
  - src/cli/mod.rs: 147–154
  ```rust
  pub fn limit(&self) -> Option<OutputLimit> {
      if self.limit {
          Some(OutputLimit::CumulativePercentageThreshold(90.00))
      } else {
          None
      }
  }
  ```
- Suggested fix:
  - Introduce `--limit <0..=100>` (float or integer). Keep `--limit` boolean as shorthand mapping to default (90), or deprecate.
  - Validate and produce a helpful error on out-of-range values.

---

## Additional Observations

- Architecture/Modularity
  - The code reflects the intended hexagonal structure (cli/parsing/domain/reporting). Public API placement and domain modeling are coherent.

- Testing Coverage
  - Strong acceptance tests: periods, tags, details, markdown formatting, and general behaviors under `tests/acceptance`. Good breadth and realistic scenarios.

- Security/Validation
  - Directory walker confines processing to `md`/`txt` and does not follow symlinks. Reasonable defaults for a CLI tool with local files.

- Minor Tech-Debt
  - `TimeEntry::main_context()` panics if no tags, but parser guarantees tags for valid entries. This is acceptable given the invariant; document as such or convert to `Result` if external construction is expected.

---

## Proposed Next Steps (Minimal, High-Value)

1) Parsing performance refactor (mutable state)
   - Refactor `parse_content` to mutate `ParseState` in place and introduce `process_line_mut` (no `clone`).
   - Acceptance criteria: no behavioral changes; equal test pass rate; noticeable speedup on large files.

2) DevShell alignment
   - Add `cargo-watch`; consider pinned Rust toolchain (with clippy/rustfmt). Document or guard `cargo-llvm-cov` usage.

3) Percentage normalization
   - Ensure totals display as exactly 100% (either last-bucket correction or error distribution).

4) Safer date helpers
   - Remove reachable `unwrap`/`expect` in `range.rs`; return domain errors or provide safe fallbacks.

5) Configurable `--limit`
   - Add `--limit <0..=100>` with validation; map boolean to default.

---

Prepared by: Agent Mode (local analysis)
