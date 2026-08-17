# spike/verify — the dorc-verify binder (steering law)

Governs the binder itself (the Rust under `spike/verify/`) and the artifacts it
generates: `minispec/REPORT.md` and `src/catalogue_lock.rs`. The spec content it reads
— `minispec/**/*.lean` — is NOT ours: it stays under `minispec/CLAUDE.md`'s
`law-spec-touch-frontier-human-only`. Registry discipline: one rule per bullet,
greppable slugs, APPEND to sections.

## Artifacts & verdicts

- **rul-published-paths-are-repo-relative** — no committed or byte-compared artifact
  ever carries an absolute path; portability is what makes the byte-comparison
  meaningful.
- **rul-verdict-leads-the-artifact** — a checking command prints its PASS/FAIL FIRST,
  on stderr, and keeps the artifact bytes pure on stdout. A verdict buried under its
  own artifact invites filtered reads.
- **rul-staleness-names-its-diff** — a freshness refusal names WHICH sections or cells
  diverged; a bare "stale — re-run" is a defect.

## The catalogue lock & promote

- **rul-promote-writes-only-what-it-looked-at** — a cheap-tier promote can neither mint
  nor withdraw an engine-tier claim. What a run did not recompute is carried forward
  untouched, never re-asserted and never dropped.
- **rul-lock-is-generated-not-edited** — `src/catalogue_lock.rs` is written only by
  `dorc-verify promote` (its byte-stable round-trip is pinned); the review artifact is
  the git diff. Never hand-edit it.

## Evidence & badges

- **rul-seat-citations-are-owner-scoped** — a seat citation resolves to the ONE
  owner-qualified declaration; ambiguity refuses loudly (seven `fn join` live in
  `analysis/src/lattice.rs` alone).
- **rul-badge-recompute-is-harness-scoped** — `report`/`promote --with-kani` drives only
  the PAIRED harnesses, never the full battery — that is `verify:kani`'s serialized
  lane, and the full battery has OOM'd the WSL VM.
- **rul-interrogated-needs-the-coupling** — the `interrogated` badge requires the
  `{slug}_specializes_at_u32` coupling theorem alongside the nonvacuity probe and the
  battery.
- **rul-derivation-digest-is-an-alarm-not-trust** — the recorded source digest detects
  drift of the `#[path]`-included inputs, and nothing more. A matching digest asserts
  NOTHING about correctness; an absent one renders `UNRECORDED` honestly rather than
  silently passing; and only a real `verify:translate` run closes it.
