I've read every source file in the crate. Here's the review.

---

## Review: `errorloom` — pre-publication engineering audit

**Overall:** The crate is genuinely small, dependency-free, and honest about its scope. The transport kernel (word-diff → span-attribution → re-hole → refusal) is well-factored from the orchestration layer; the seeded round-trip property test is the right kind of correctness anchor; the `FakeGit` + `SubprocessGit` pattern is clean; every refusal class has a corresponding test case. The problems are concentrated in a few specific areas — mostly around API completeness for external consumers and subprocess robustness.

---

### Finding 1 — MUST-FIX: `Region<K>` cannot be constructed by external consumers

`src/span.rs:66-67`

`Region<K>` is `#[non_exhaustive]` with no public constructor functions. The `impl<K> Region<K>` block (lines 102-122) provides only `key()` and `instance()` accessors. In Rust, `#[non_exhaustive]` on an enum prevents external crates from constructing variants using struct-literal syntax (`Region::TemplateLiteral { key, ... }`).

The `Consumer` trait (which is `pub`) requires implementors to return `TaggedRender<K>` from `tagged_render()`, which in turn requires constructing `Span<K>` values containing `Region<K>` values. An external consumer cannot do this — the trait is unimplementable from outside the crate.

The integration tests in `tests/toy_consumer.rs:237` and `tests/toy_consumer.rs:245` construct `Region` variants directly, but integration tests are separate crates and should be subject to the same restriction. If these tests currently compile, it may be a toolchain-specific behavior that shouldn't be relied upon.

Fix: either (a) make `Region` exhaustive and commit to the current variant set for 1.0, or (b) add public constructor functions (e.g., `Region::new_template_literal(key, paragraph, instance)`) for each variant and keep it non-exhaustive for future growth. Option (b) is better — it preserves forward-compat while making the trait implementable.

---

### Finding 2 — MUST-FIX: `SubprocessGit` silently swallows git failures

`src/bless.rs:150-153`, `src/bless.rs:160-165`

In `head_version_of`, when git exits nonzero, the method returns `Ok(None)` — conflation of "file is not tracked" with "git crashed / repo is corrupt / disk is full":

```rust
if !output.status.success() {
    return Ok(None);   // line 152: git segfault looks like an untracked file
}
```

In `dirty_paths`, the exit status is never checked at all — a crashing git produces empty stdout, which parses as "no dirty files." A transient git failure is indistinguishable from a genuinely clean tree.

Fix: `GitError` needs a `NonZeroExit { stderr: String }` variant. Both methods should check `status.success()` and return `Err(GitError::NonZeroExit { ... })` on failure (except possibly for `git show` on a genuinely untracked path — but that case should be distinguished, e.g., by parsing stderr for "does not exist" vs. generic failure).

There's a related fragility: `head_version_of` at line 143 does `path.to_string_lossy().replace('\\', "/")` to construct `HEAD:{rel}`. Paths containing `:` or `@` could interact with git's rev-parse syntax, though `git show HEAD:<path>` treats everything after the colon as a path. Low risk but worth noting.

---

### Finding 3 — MUST-FIX: `tokenize_command` silently drops unterminated-quoted content

`src/runner.rs:310-344`

The command-line tokenizer tracks an `open` flag (whether a token is being accumulated) and pushes on EOF if `open` is true (line 340-342). But it never checks whether `quote` is `Some` at EOF — an unterminated quote (`$ mytool "unfinished`) silently accumulates the content without the opening quote and pushes it as a normal token. The user gets a mysterious "command not found" or a wrong-argv execution rather than a clear parse error.

Fix: after the loop, if `quote.is_some()`, return an error (or use a `Result` return type). The crate claims "sharp edges are fine" but this is silent data corruption, not a sharp edge.

---

### Finding 4 — NICE-TO-FIX: `#[non_exhaustive]` on error enums weakens downstream correctness

`src/container.rs:125`, `src/runner.rs:133`, `src/promote.rs:113`, `src/bless.rs:109,233,344`, `src/span.rs:140`

Every public error/refusal enum is `#[non_exhaustive]`. This forces every downstream consumer to include a wildcard arm (`_ =>`), which means new error variants added in a minor release silently fall into the catch-all instead of producing a compile-time notification. For a library whose value proposition is "correctness-critical extraction of edits," the caller should be forced to explicitly handle new failure modes.

This is not universally wrong — `FrontmatterValue` and `Region` as non-exhaustive is defensible — but error types are the wrong place for it.

Fix: make error enums exhaustive (`CaseError`, `RunError`, `RefusalClass`, `BlessError`, `GitError`, `ModeRefusal`, `TaggedRenderError`). If forward-compat for error variants is desired, include a hidden `#[doc(hidden)] __NonExhaustive` variant instead, which makes the breakage opt-in but still visible.

---

### Finding 5 — NICE-TO-FIX: No timeout on child process execution

`src/runner.rs:278`

`let _status = command.status()?;` blocks indefinitely if the child process hangs. There is no `wait_timeout`, no `Drop`-on-panic kill, no wrapper that sets a deadline. A hung test command stalls the caller forever.

Fix: wrap command execution with a configurable timeout (even if the default is "no timeout" for now, the seam should exist). At minimum document that `run_case` blocks indefinitely and the caller is responsible for wrapping it.

---

### Finding 6 — NICE-TO-FIX: `to_text` may produce non-round-trippable output for sections without trailing newlines

`src/container.rs:322-328`

`to_text` serializes sections as `-- name --\n` + content with no trailing-newline guarantee between a section's last content byte and the next `-- marker --`. If section content lacks a terminating `\n`, the next section marker fuses onto the same line, which `parse_txtar` would interpret as part of the prior section's content. A following parse fails with `ReplayNotLast` or a wrong section parse.

The `SAMPLE` constant in `container_tests.rs:6` tests the common case (sections ending with `\n`), so the round-trip test doesn't cover this edge. A programmatic consumer that constructs a `Case`, mutates a section to content without a trailing newline, and calls `to_text` would produce a broken case file.

Fix: either guarantee that `to_text` inserts a `\n` between section content and the next marker when the content doesn't end with one, or validate on `Case::parse` that all section content ends with `\n` (or at least that the last line isn't a marker). The former is safer.

---

### Finding 7 — NICE-TO-FIX: `RunError::Io(String)` discards the original `io::Error`

`src/runner.rs:186-188`

The `From<std::io::Error>` impl converts to `RunError::Io(error.to_string())`, losing the `ErrorKind`. A caller that wants to distinguish `PermissionDenied` from `DiskFull` must string-match the message.

Fix: either store the `io::ErrorKind` as an additional field, or wrap the original `io::Error`. The type already carries a `String` payload so adding `kind: std::io::ErrorKind` (or a custom enum) is a compatible change if `Io` is made `#[non_exhaustive]` (it already is).

---

### Finding 8 — NICE-TO-FIX: No `debug_assert!` guards on caller-owned invariants

`src/prose.rs:17-18`, `src/prose.rs:35-36`, `src/span.rs:47-48`

`Word::new`, `ParamName::new`, and `ArrangementSlug::new` document caller-owned invariants (no interior whitespace for `Word`, etc.) but enforce nothing. `debug_assert!` checks would catch consumer bugs in tests without affecting release builds.

Fix: add `debug_assert!(!text.contains(char::is_whitespace))` or similar to the relevant constructors.

---

### Finding 9 — NICE-TO-FIX: `Refusal` Display impl violates the `Display`-is-one-line convention

`src/promote.rs:197-222`

`Display for Refusal<K>` prints a multi-line dump with `--- baseline ---` / `--- edited ---` / `--- regions ---` sections. The `std::fmt::Display` trait is conventionally a single-line human-readable summary; multi-line structured output is `Debug` territory. Tools that print errors (like `anyhow`, `eyre`, or test harnesses) may render this poorly.

The dump is useful, but it should be on the `Debug` impl, with `Display` providing a one-line summary.

---

### Nits (not ranked, just noted)

- `src/diff.rs` — The exclusive use of `saturating_add` in the LCS table means very large inputs (beyond the documented "tiny paragraph" assumption) would silently produce wrong diffs. Defensible given the documented constraint, but worth a debug-assert or a hard cap.

- `src/promote.rs:204` — The fixed `24`-character width in the refusal display will misalign for words longer than 24 characters. Probably fine for English prose words.

- `src/runner.rs:416-427` — `unique_base` uses a `loop` with a monotonic counter; after 2^64 collisions the loop would spin forever. Astronomically unlikely (pid + counter), so this is purely a note.

- Lack of a changelog or versioning policy — `README.md` says "Pre-1.0, experimental," which is sufficient. The crate doesn't need more.

---

### What's good (evidence)

- **Zero dependencies.** The hand-rolled LCS diff and txtar parser are justified: keeping `cargo deny` clean and the network surface zero for a correctness kernel is worth ~200 lines of dead-simple code.

- **The round-trip property test** (`tests.rs:257-295`) with 500 seeded random inputs, covering both explicit-instance and structural-inference grouping modes, is the right kind of quality anchor. It exercises `promote` → catalog insertion → re-render → tokenize-compare.

- **The `FakeGit` + real-git-smoke separation** (`bless.rs:183-218`, `tests/subprocess_git.rs`) means orchestration correctness is proven hermetically and the subprocess adapter is tested against real git in isolation.

- **Every refusal class has a test** (`tests.rs:350-420`). No silent-failure surface.

- **`forbid(unsafe)`** is upheld workspace-wide.

- **The `Consumer` + `Git` trait split** is well-scoped: exactly two abstractions, each with a crisply defined responsibility. The library isn't over-abstracted.

- **Honest about sharp edges.** The crate documents that it's internal tooling, that refusals are blunt, that `v1` has no paragraph restructuring. No false polish.

---

### Summary

The crate is appropriately small and well-tested for its claims. The transport kernel is solid. The three must-fix items are all in the "external consumer can't use this without hitting a wall" or "silent failure" categories — they don't question the design, just the publish readiness of the surface. The nice-to-fix items are real but not blockers for an initial `0.1.0` release of internal tooling.