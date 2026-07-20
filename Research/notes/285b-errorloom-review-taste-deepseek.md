I've read every file. Here are my findings, ranked by severity.

---

## Findings

### 1. `src/runner.rs:185-189` — `From<std::io::Error>` blanket impl flattens all I/O errors to string

```rust
impl From<std::io::Error> for RunError {
    fn from(error: std::io::Error) -> Self {
        RunError::Io(error.to_string())
    }
}
```

This is a blanket `From` impl on a public error type. Every `?` on an I/O operation in `runner.rs` converts the `io::Error` into a string, permanently discarding the `ErrorKind`. A consumer that wraps `RunError` (e.g., to retry on `WouldBlock` or to distinguish `NotFound` from `PermissionDenied`) cannot do so. The `RunError::Io(String)` variant carries a human-readable message only — no programmatic access to the underlying error.

The fix: either store the `io::Error` directly in `RunError::Io(std::io::Error)` (the standard approach), or at minimum store `io::ErrorKind` alongside the message. Since `std::io::Error` already implements `Display`/`Debug`/`Clone`, storing it whole costs nothing in API surface.

**Severity: High** — This is the kind of error-type mistake that gets called out immediately in r/rust review. The standard idiom is `Io(#[source] std::io::Error)` with `#[from]` from `thiserror`, or the manual equivalent.

---

### 2. `src/diff.rs:38-45` — Saturating arithmetic in `Lcs::new` silently produces wrong alignments on overflow

```rust
fn new(rows: usize, cols: usize) -> Self {
    let size = rows.saturating_add(1).saturating_mul(cols.saturating_add(1));
    Lcs { cols, cells: vec![0; size] }
}
fn index(&self, i: usize, j: usize) -> usize {
    i.saturating_mul(self.cols.saturating_add(1)).saturating_add(j)
}
```

When `(rows+1)*(cols+1)` overflows `usize`, both `size` and every `index` computation saturate silently. The `Vec` is allocated at the saturated size (too small), and index computations produce wrong positions that stay spuriously in-bounds. The DP algorithm then populates the wrong cells and the backtrack produces a garbage alignment — with no panic, no error, no diagnostic. The module's own doc says word streams are "tiny," so this won't fire in practice, but a published, dependency-free kernel module shouldn't contain a silent-corruption path.

The fix: use `checked_add`/`checked_mul` and return `Result` from the public diff entry point (or panic, since the preconditions are caller-controlled — a `debug_assert!` on reasonable max lengths plus a `checked` fallback that returns an error or truncates safely).

**Severity: High** — Silent wrong output from a correctness-critical kernel function. The likelihood is negligible given the input sizes, but the presence of a corruption path in a library that advertises deterministic correctness is a review red flag.

---

### 3. `src/runner.rs:280` — `String::from_utf8` error discards the raw bytes

```rust
let bytes = fs::read(&capture_path)?;
String::from_utf8(bytes).map_err(|_| RunError::NonUtf8Output { block: index })
```

`String::from_utf8` returns a `FromUtf8Error` whose `.into_bytes()` or `.as_bytes()` recovers the original `Vec<u8>`. The error mapping discards those bytes entirely. A consumer debugging why block N produced non-UTF-8 output gets only a block index — no hex preview, no length, nothing. For a tool that exists to inspect command output, this is self-defeating.

The fix: store at minimum the first N bytes (or a lossy `String::from_utf8_lossy` rendering) in the error variant. Storing the whole `Vec<u8>` is fine since this is the error path.

**Severity: Medium** — Not a correctness bug, but a sharp usability regression on the error path of a library whose entire purpose is inspecting tool output.

---

### 4. `src/bless.rs:77-80` — `Consumer::apply_field_edits` takes `&BTreeMap` forcing clones

```rust
fn apply_field_edits(
    &mut self,
    edits: &BTreeMap<Self::Key, FieldTemplate>,
) -> Result<(), Self::Error>;
```

The caller at `bless.rs:458` owns the `edits` map and drops it after the call:

```rust
consumer.apply_field_edits(&edits).map_err(consumer_err)?;
```

By taking `&BTreeMap`, the trait forces every consumer implementation to clone entries it needs to store (the toy consumer at `toy_consumer.rs:78` does exactly this). Since the caller transfers ownership anyway, the trait should take `edits: BTreeMap<Self::Key, FieldTemplate>` by value. This is a zero-cost change at the call site and removes mandatory clones on the impl side.

The fix: change the parameter to owned `BTreeMap<Self::Key, FieldTemplate>`. The call site becomes `consumer.apply_field_edits(edits)`.

**Severity: Medium** — API design; forces allocation on a trait method where the caller already has ownership. Classic r/rust review catch.

---

### 5. `src/runner.rs:416-427` — `unique_base` has an unbounded spin-loop

```rust
fn unique_base() -> std::io::Result<PathBuf> {
    let pid = std::process::id();
    loop {
        let nonce = COUNTER.fetch_add(1, Ordering::Relaxed);
        let candidate = std::env::temp_dir().join(format!("errorloom-{pid}-{nonce}"));
        match fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
}
```

On a `PermissionDenied` or read-only temp dir, this returns immediately (correct). But if something continuously creates colliding directory names, it spins forever. The `COUNTER` is a `u64` and will eventually wrap — at which point it reuses old names and loops indefinitely even without external collision. At one million iterations per second, wrap takes ~585,000 years, so this is *cosmically* unlikely. But the code is one line away from being obviously correct: a max-retries guard with a distinctive error.

The fix: a `for _ in 0..1_000` loop that returns a distinctive error on exhaustion, or at minimum a comment explaining why the unbounded loop is acceptable.

**Severity: Low** — Theoretically unbounded; practically fine. Worth a comment.

---

### 6. `src/bless.rs:167-176` — `SubprocessGit::dirty_paths` parses `--porcelain` with fragile slicing

```rust
let Some(rest) = line.get(3..) else { continue };
let rest = rest.rsplit(" -> ").next().unwrap_or(rest);
let rest = rest.trim().trim_matches('"');
```

Git's `--porcelain` format uses two status characters, a space, and then the path. The `3..` slice assumes exactly characters (not bytes) and no leading whitespace. This is reliable for the documented `--porcelain` v1 format but assumes ASCII status characters (they are). More importantly, the `trim_matches('"')` on line 172 handles quoted paths with spaces, but only trims one layer of quoting — a path containing literal quotes at its boundaries would be mangled. These are edge cases, but git path parsing has known sharp edges and this is a hand-rolled implementation.

The fix: either use `--porcelain=v1 -z` with null-separated output (more robust), or add a comment noting the format assumptions. The `-z` variant is the standard way to avoid quoting ambiguities.

**Severity: Low** — Works for all realistic paths. A comment would suffice.

---

### 7. `src/prose.rs:67-109` — `tokenize_located` state machine has no inline comments

This is a correctness-critical function: it feeds the span map and the diff engine. The state machine juggles four mutable variables (`word`, `gap_start`, `gap_newlines`, character iteration) across whitespace/non-whitespace transitions with a paragraph-break heuristic (≥2 newlines). The logic is correct on inspection, but at ~40 lines with zero inline comments, a reader must re-derive the state transitions from first principles.

The fix: three short comments marking the state-entry points. Not a bug; purely a readability concern for the most subtle function in the prose pipeline.

**Severity: Low** — Taste; the code is correct, but maintainability suffers.

---

### 8. `src/lib.rs` — No `#![warn(missing_docs)]` / `#![deny(missing_docs)]`

Every public item *is* documented, so this is a belt-and-suspenders issue. Adding the lint prevents future public items from landing undocumented.

**Severity: Trivial** — The crate is already fully documented. A one-line hardening.

---

### 9. `src/prose.rs:48` — `Token` derives `Hash` but is never used in a `HashMap`

```rust
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Token { ... }
```

The crate uses `BTreeMap` exclusively. The `Hash` impl is vestigial — it doesn't hurt, but it signals an intent (hashability) that the crate doesn't exercise. If a future change puts `Token` in a `HashSet` and the `Hash` impl isn't consistent with `BTreeMap` ordering assumptions, that's a latent bug (unlikely for word/break, but the pattern exists).

**Severity: Trivial** — Cosmetic. Remove or keep; either is defensible.

---

### 10. `src/runner.rs:263-265` — Silent `PATH` skip when `std::env::join_paths` fails

```rust
if let Ok(joined) = std::env::join_paths(&env.path) {
    command.env("PATH", joined);
}
```

On platforms where `join_paths` fails (e.g., a path entry containing a null byte, or a non-UTF-8 path on Unix), the child process gets no `PATH` variable at all. All commands then fail with "not found" rather than surfacing the configuration error. This is arguably correct for a `RunEnv` that starts empty and only contains caller-validated paths, but the silent swallowing makes the failure mode hard to diagnose.

**Severity: Trivial** — The `env.path` entries come from the caller's own `RunEnv::path_dir()` calls, so they're under the caller's control.

---

## Positive notes

- The `#[non_exhaustive]` placement on every growable public enum (`CaseError`, `RunError`, `TaggedRenderError`, `RefusalClass`, `ModeRefusal`, `BlessError`, `GitError`, `FrontmatterValue`, `Region`) is exactly right for a pre-1.0 published crate.
- `#[must_use]` on `ReplayCapture`, `RunReport`, `PromoteOutcome`, and `BlessResult` correctly prevents silent discard of results.
- The error-path-only cloning pattern in `promote.rs` (`RefusalCtx::refuse` materializes owned data only on refusal) is the correct ownership posture.
- Zero `.unwrap()`/`.expect()` on library paths (outside tests). The `unwrap_or`/`unwrap_or_default` discipline is consistent.
- `ConsumerKey` as a blanket-impled supertrait of `Clone + Ord + Debug` is a clean pattern for an opaque key.
- `FakeGit` alongside `SubprocessGit` behind the `Git` trait is the right testability seam.
- The word-diff, prose model, and bless orchestration are genuinely novel and well-factored — each module has a clear, single responsibility.
- Test coverage is strong: the round-trip property test (500 seeded runs across both instance-id modes), the per-refusal-class table, the toy-consumer end-to-end bless loop, and the real-git smoke test with clean skip-on-absent.