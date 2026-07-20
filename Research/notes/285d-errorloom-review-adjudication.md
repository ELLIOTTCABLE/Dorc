# 285d — errorloom review adjudication + last-polish repair-plan

Conductor adjudication (Fable, 2026-07-20) of the two DeepSeek lanes (`285b` taste, `285c` swe).
Purity protocol: DeepSeek is a cheap third angle, NOT a peer — findings weighted, verified where
load-bearing, never credited raw. Provenance labelled. This is the spec for a last-polish-pass Opus
builder, who holds FINAL latitude on the judgment-tier items (human-directed: trust the builder).

Dispatch sequencing: this polish edits `errorloom` public API (ripples to the `dorc-loom` consumer),
so it lands AFTER `phase-5-backport` folds (phase-5 also edits `dorc-loom`) — serial, to avoid a
dorc-loom merge collision. Off the phase-5-folded tip.

## KILLED — do NOT act (verified false / rejected against standing ruling)

- **swe-F1 "Region unconstructable → Consumer unimplementable" — VERIFIED FALSE.** `Region<K>` is
  `#[non_exhaustive]` at the ENUM level (`span.rs:66`), which permits external construction of known
  variants and only forces non-exhaustive matching. `dorc-loom` (external crate) constructs every
  variant directly (`dorc-loom/src/lib.rs:64/74/79/82`) and compiles. DeepSeek conflated enum-level
  with variant-level `non_exhaustive` (its own "may be toolchain-specific" hedge was the tell). No fix.
- **swe-F4 "make the error enums exhaustive" — REJECTED.** Directly contrary to the standing taste
  ruling (`#[non_exhaustive]` on growable published enums is correct pre-1.0; the tasteful-rust
  research anchor agrees) — and the TASTE lane explicitly PRAISED the placement (`285b` positive
  notes). The two lanes disagree; I side with taste + the ruling. KEEP non_exhaustive on all public
  enums. (Publish-taste is ultimately the human's; flag for them, do not flip.)

## FIX — credited, worth the polish (error-richness + silent-failure clusters lead)

Error-richness (medium; the crate exists to inspect tool output, so losing detail is self-defeating):
- **taste-F1 / swe-F7** — `RunError::Io(String)` (`runner.rs:135/187`) flattens `io::Error`, discarding
  `ErrorKind`. Store the `io::Error` (or its `ErrorKind`) so consumers can distinguish NotFound /
  PermissionDenied. `Io` is already `non_exhaustive`-friendly.
- **swe-F2** — `SubprocessGit` swallows git failures: `head_version_of` returns `Ok(None)` on nonzero
  exit (conflates untracked-file with git-crash); `dirty_paths` never checks `status.success()`. Add
  `GitError::NonZeroExit { stderr }`; distinguish genuine "path not in HEAD" from failure. (Low blast
  radius — orchestrator-only bless path — but real for a published lib.)
- **taste-F3** — `String::from_utf8` map_err (`runner.rs:~280`) discards the raw bytes; keep a lossy
  preview or the bytes in the error variant.

Silent parse-failures (cheap; "sharp edges fine" does NOT license silent data corruption):
- **swe-F3** — `tokenize_command` EOF path (`runner.rs:~340`) pushes `if open` without checking
  `quote.is_some()`: an unterminated quote silently accumulates as a normal token. Return a parse
  error on `quote.is_some()` at EOF. VERIFIED.
- **swe-F6** — `to_text` (`container.rs:~322`) can fuse a section marker onto a prior section lacking a
  trailing `\n` (non-round-trippable). Guarantee a `\n` between section content and the next marker,
  or validate on parse. Add the round-trip test case the current `SAMPLE` misses.

Cheap hardening (low/trivial; each is a few lines):
- **taste-F2 / swe-nit** — `diff.rs` `Lcs` saturating arithmetic hides an overflow → silent-wrong
  alignment path. Inputs are tiny by design, but a correctness kernel shouldn't carry a silent-corruption
  path: `debug_assert!` a sane cap (or `checked_*` + a hard error). Cheap.
- **taste-F5 / swe-nit** — `unique_base` unbounded spin-loop (`runner.rs:~416`): bound the retries with a
  distinctive error (or a one-line comment justifying it).
- **swe-F8** — `Word::new`/`ParamName::new`/`ArrangementSlug::new` document caller invariants (no
  interior whitespace) but enforce nothing: add `debug_assert!`s.
- **taste-F8** — add `#![warn(missing_docs)]` (already fully documented; prevents regressions).
- **taste-F4** — `Consumer::apply_field_edits(&BTreeMap…)` forces impls to clone; the caller owns and
  drops the map. Take it BY VALUE. (Ripples to the dorc-loom Consumer impl — keep the workspace green.)

## BUILDER LATITUDE — judgment/marginal (decide during work; skipping any is fine, say which)

- **swe-F9** — `Display for Refusal` is a multi-line dump (Debug territory). More idiomatic: one-line
  `Display` + move the dump to `Debug`. BUT this is the deliberate blunt-dump sharp-edge posture
  (`282:rul-internal-tool-sharp-edges`); if you keep it, that's defensible — your call, note it.
- **swe-F5** — no child-process timeout (`runner.rs:~278` `command.status()?` blocks forever). A real
  seam but feature-ish for v0.1 internal tooling; a doc note on `run_case`'s blocking contract is the
  cheap floor. Latitude.
- **taste-F7** — `tokenize_located` state machine has no inline comments (~40 lines, subtlest function):
  ≤3 short comments if it helps a reader; respect the comment budget.
- Marginal cosmetics (skip unless trivial): taste-F9 (`Token: Hash` vestigial — remove or keep),
  taste-F10 (PATH `join_paths` silent skip — a note), swe-nit (promote.rs fixed-24 width misaligns for
  long words).

## What both lanes AGREED is good (no action; recorded so it stays)

Zero deps + the hand-rolled LCS/txtar justified; the 500-seed round-trip property test over both
instance-id modes; `FakeGit`/`SubprocessGit` split; every refusal class tested; `forbid(unsafe)`; the
two-trait split is right-sized (NOT over-abstracted); error-path-only cloning in `promote.rs`; zero
library-path `unwrap`/`expect`; honest sharp-edge documentation.

## For the human (publish-taste, not the builder's call)

The non_exhaustive-on-error-enums question (swe-F4 vs taste-praise) is a genuine publish-taste fork;
my adjudication keeps them non_exhaustive. Flag only. Also: the crate is `publish=false` pending your
LICENSE choice; none of the above changes that.
