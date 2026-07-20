# 285a — errorloom external-review dispatch bundle (the prompts)

The round-28 side-quest (human-directed 2026-07-20): a two-lane, DeepSeek-only outside-lineage
review of the *publishable* `errorloom` crate, before its eventual publish. One docID, four parts:

- **285a** (this file) — the dispatch bundle: both review packets, fenced for the `deepseek-reviewer`
  shim's `awk` extractor.
- **285b** — the DeepSeek report from the `deepseek-taste` lane (raw, unadjudicated).
- **285c** — the DeepSeek report from the `deepseek-swe` lane (raw, unadjudicated).
- **285d** — the conductor's adjudication + repair-plan (purity protocol: never quotes b/c wholesale;
  labels provenance; weights DeepSeek as a cheap third angle, not a peer), to be handed to a
  last-polish-pass builder.

Dispatch mechanics (`/foreign-models`): each lane is a `deepseek-reviewer` Sonnet shim, read-only, run
WITHOUT a worktree (ds-review is read-only; its cwd-subtree read-scoping to the errorloom directory IS
the isolation and the dorc-fence). Each shim is handed: this bundle's absolute path + its KEY; the
artifacts-root cwd = the errorloom crate dir; its durable output path (285b / 285c); the debug budget
(≤5) + errors-upward guards. The two lanes fan out in parallel; adjudication is one batched read after
both return. DeepSeek needs the 1Password key ack live (human present).

---

=== DISPATCH: deepseek-taste | mode=review ===
You are reviewing a small, standalone, to-be-published Rust library crate named `errorloom`. Your
working directory is the crate root; every file you can read under it (`src/`, `tests/`, `Cargo.toml`,
`README.md`) is the crate. This is a strictly READ-ONLY review: make NO edits, NO commits, NO file
writes of any kind, and do NOT run build/test commands that mutate anything. Do NOT spawn subagents or
sub-tasks — do the review yourself.

FAIL-FAST: if your file-reading tools (Read / Grep / Glob) do not work, STOP immediately and report
"file-read tools unavailable" — do NOT review from memory or guess at the code. You may use kagi web
search to check CURRENT Rust API-guideline / idiom specifics if useful; if you rely on any such fact,
cite it — do not assert recent-Rust specifics from stale training memory.

SCOPE — read ONLY files inside this directory. The crate's doc-comments carry terse internal
cross-references shaped like `NNN §N` or `NNNx:some-slug` (e.g. `282 §6`, `28A §1`) and a couple of
mentions of a consumer project by name. IGNORE ALL OF THESE COMPLETELY — they cite internal design
docs you cannot and must not see; do not speculate about them, do not hold their opacity against the
code, do not comment on them. Judge `errorloom` purely as a standalone Rust library on its own merits.

WHAT IT IS (orientation only, from its own README/lib.rs): a testing/tooling library — executable
"transcript case" files as the authoring surface for a CLI tool's user-facing prose, with a
cram/txtar-lineage runner plus a diff-driven prose-extraction engine. Zero external dependencies by
design; a hand-rolled word-level diff; `#![forbid(unsafe)]`.

YOUR REMIT — a NARROW, RUST-USER-TASTE code review. Adopt the posture of a demanding senior Rust
reviewer doing a pre-publication pass, focused on the taste concerns the r/rust community raises about
library code. Be adversarial and concrete; cite file:line for every finding; rank by severity.

Some *non-exhaustive examples* of the sorts of things you may be reviewing for; but your focus is
up to your own reasoning; step outside of these as you see fit:
- API ergonomics & derive hygiene: naming; public type surface; `Debug`/`Clone`/`PartialEq`/`Eq`/
  `Hash`/`Default` derives on public types; `#[must_use]` placement; `#[non_exhaustive]` on growable
  public enums; generous inputs (`impl AsRef`/`&str`/`&[T]` vs owned); newtype patterns; `Error` +
  `Display` impls.
- Error handling: `Result` discipline; NO `.unwrap()`/`.expect()`/indexing-panics on library paths
  (tests may); typed error enums vs stringly errors; `## Errors` doc sections.
- Ownership taste: gratuitous `.clone()`; needless allocation / `.collect()` churn; borrow-vs-own on
  hot vs cold paths — NOTE: materializing owned data only on error/dump/cold paths is GOOD, not a
  defect; do not flag that.
- Iterator/combinator idiom vs imperative loops; `?` propagation; match style.
- Doc quality on public items; doc-examples using `?` not `unwrap`; `#![warn(missing_docs)]`.
- "C/Java-shaped Rust" and over-abstraction / premature generics ("generic soup") — AND the opposite
  over-correction (idiomatic-maximalism that hurts readability). Simplicity is the taste; flag BOTH
  directions.

Output: a ranked findings list (file:line, severity, the concrete criticism, and the fix in words —
but do NOT make the fix). Brief positive notes are fine but your value is the critique. This is a
low-stakes advisory pass; if you hit a tooling snag, report it briefly and stop rather than
working around it.
=== END DISPATCH: deepseek-taste ===

=== DISPATCH: deepseek-swe | mode=review ===
You are reviewing a small, standalone, to-be-published Rust library crate named `errorloom`. Your
working directory is the crate root; every file you can read under it (`src/`, `tests/`, `Cargo.toml`,
`README.md`) is the crate. This is a strictly READ-ONLY review: make NO edits, NO commits, NO file
writes of any kind, and do NOT run mutating commands. Do NOT spawn subagents — do the review yourself.

FAIL-FAST: if your file-reading tools (Read / Grep / Glob) do not work, STOP and report "file-read
tools unavailable" — do NOT review from memory. You may use kagi web search to check CURRENT Rust
packaging / SemVer / API-guideline specifics if useful; cite anything you rely on rather than asserting
recent-Rust specifics from stale memory.

SCOPE — read ONLY files inside this directory. The doc-comments carry terse internal cross-references
shaped like `NNN §N` or `NNNx:slug` and a couple of consumer-project name mentions: IGNORE ALL OF THESE
COMPLETELY — internal design-doc citations you cannot see; do not speculate about them, do not comment
on them, do not hold their opacity against the code. Judge the crate purely as a standalone library.

WHAT IT IS (orientation only): a testing/tooling library — executable transcript-case files (txtar +
YAML frontmatter) as the authoring surface for a CLI tool's user-facing prose. Two layers: a generic
transport engine (tokenize prose to words/paragraphs, word-level diff, attribute edits through a span
map, re-hole parameter values, refuse ambiguous edits) and an orchestration layer (a case-file
container, a sequential replay runner over a controlled environment, and two "bless" modes gated by a
small git façade). Zero external deps; a seeded property test on the transport round-trip;
`#![forbid(unsafe)]`.

YOUR REMIT — a BROADER SOFTWARE-ENGINEERING quality review of a published library. Step back from
line-level taste (the other lane owns that) and judge it as an engineering artifact others will depend
on, extend, and trust. Be adversarial and concrete; cite file:line; rank by severity; and separate
"must-fix-before-publish" from "nice-to-have."

Output: ranked findings (file:line, severity, must-fix-vs-nice-to-have, the concrete issue, the fix in
words — do NOT make the fix). If the design is appropriately small and honest, say so with evidence; if
it over-reaches or under-delivers on its own claims, say that with evidence. Low-stakes advisory pass;
on a tooling snag, report briefly and stop.
=== END DISPATCH: deepseek-swe ===
