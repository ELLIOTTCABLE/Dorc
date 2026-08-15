# minispec — CLAUDE.md

Conductor-authored (r30, per `301`). This directory is the project's reviewable
statement of the few kernel laws it opts to verify — internal instrument, never
user-facing. Read `Research/notes/301-minispec-and-dorc-verify.md` for the full spec;
this file carries what binds anyone standing in this directory.

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

## The remit (`301` §4)

The standing remit is the ABSOLUTE MINIMUM provable surface: the three flat-lattice
join axioms (`JoinIsCommutative` · `JoinIsIdempotent` · `JoinIsAssociative`) — basic,
zero-controversy mathematics with no Dorc design content, chosen so the process,
praxis, gates, and habits get built on terrain that cannot generate design
emergencies. Enrichment (real algebra-dependent laws) is a STANDALONE human-led
work-item whose tabled menu lives in `notes/300` §6; nothing here grows toward it as
a side-effect of any other lane.

## The unit contract (what the binder expects; STRAWMAN names rename freely)

Per unit `<Slug>.lean`: a module docstring carrying the English-authoritative law
text · `def <Slug> : Prop` over the `Generated/` definitions · `theorem
<Slug>_nonvacuous` (the anti-vacuity probe: a positive witness where the law does
non-trivial work) · a boundary battery of `example`s (⊥ · element · ⊤ shapes,
concretely evaluated). Proofs live at `Minispec/Proofs/<Slug>.lean` as
`theorem <Slug>_holds : Minispec.<Slug>`, importing the unit — the tactic-churn zone,
structurally unable to touch a statement. Generic-dictionary hypotheses
(`LawfulClone`/`LawfulEq`) are NAMED TRUSTED-BASE entries in
`Minispec/TrustedBase.lean` — governed shared vocabulary; changing what "lawful"
means changes every unit's claim, so edits are ceremony. Unit files stay hard-minimal
and diff-quiet: no metadata (the catalogue's), no churny material, byte-budget
advisory 8KB.

## What is builder-space here, and what is not

- `Generated/` is machine-produced: regenerated only by `mise run verify:translate`, never
  hand-edited. It is committed so a regeneration diff is a reviewable drift alarm.
- `Minispec/` and `Minispec/Proofs/` are spec surface. Unwritten units are marked and are a
  legal resting state.
- The harness and tooling around all of it — `spike/verify/`, the mise lane, the catalogue
  lock, the report — is ordinary engineering and ordinary builder-space.
