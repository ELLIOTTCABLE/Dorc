# minispec — CLAUDE.md

> **STUB — conductor-authored at fold.** The access-law text for this directory is a
> conductor act, not a builder one. What stands below is the minimum that must be true from
> the moment the directory exists; the full remit (`301` §4) lands with it.

## The two access laws (`301` §0)

- **law-spec-touch-frontier-human-only** — ONLY frontier-class models touch minispec content,
  and only with explicit human authorization. No exceptions; no hot-loop edits, ever.
  Builders may surface chafe against the spec and propose refinements; every change routes
  through the human. This works precisely because minispec is an EXTERNAL check rather than
  an acceptance gate its own maintainers own — LLMs are extremely prone to gaming acceptance
  criteria, and an acceptance surface the worker cannot write to cannot be gamed by the
  worker.
- **law-spec-leads-the-build** — plan; decide whether the plan touches the spec; if so,
  modify the spec FIRST through the authorized lane; then build toward spec-green. A builder
  whose build looks right while the pre-modified spec disagrees REPORTS and stops. Further
  spec massaging is a very, very last resort.

## What is builder-space here, and what is not

- `Generated/` is machine-produced: regenerated only by `mise run verify:translate`, never
  hand-edited. It is committed so a regeneration diff is a reviewable drift alarm.
- `Minispec/` and `Minispec/Proofs/` are spec surface. Unwritten units are marked and are a
  legal resting state.
- The harness and tooling around all of it — `spike/verify/`, the mise lane, the catalogue
  lock, the report — is ordinary engineering and ordinary builder-space.
