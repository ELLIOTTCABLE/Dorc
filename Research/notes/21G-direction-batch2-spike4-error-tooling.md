# 21G — Direction batch 2: spike-4 shape; the Merlin/Pottier error-tooling intent; q-2 re-spec

> Orchestrator note; human-direction ledger (conversational relays, 2026-06-10/11).
> Companion to 212 (batch 1) and 21F (the r1A report). Commit at next quiesce.

## §1 The spike-4 aside (relayed earlier, ledgered now)

Assuming round-21 closes with "real stuff really elided, sound modulo Perfect Oracle
Competence," the human expects spike-4 to focus on: (a) forcing the
perfect-oracle-competence bar's permeability DOWN until it meets authorship —
re-eliminating that cliff; (b) particularly, the detailed provenance and
error-handling that requires. Round-21 artifacts that are spike-4 seed material: the
disclosure floor, provenance comments carrying originals, named refusal diagnostics,
site N.M records, the dashboard's per-site why-not attribution, the dq-2
wrapper-reachable population split.

## §2 The error-tooling intent (human, vague-by-his-own-label; recorded not designed)

Two-layer ambition for the REAL codebase (mostly out-of-spike-scope):
- layer-1: a dislocated index of error messages (slug → catalog entry; aids
  localization) PLUS mechanical CI/pre-commit support asserting every give-up
  code-path carries a slugged error-ID with a corresponding catalog entry.
- layer-2: extend the same mechanical posture to provenance — (A) any value reaching
  a user/UI surface as text must be wrapped in a provenance-preserving type; (B)
  every transformation point must extend the provenance chain.

Corpus reconciliation (plans/111 + notes/110, the round-11 error/provenance
synthesis): the human's "Merlin approach" is two adjacent round-11 findings —
[A-bour-merlin-2018] (every stage yields result × accumulated-diagnostics, never
throws; fail-fast detection ≠ fail-fast reporting) which the spike ALREADY welded as
inv-no-throw/Carrier<T>; and [A-pottier-reachability-2016] (separate diagnostic
catalog + a mechanical COMPLETENESS GATE — Menhir's enumerate-every-error-state,
maintain a .messages file, CI-check coverage) which is the unbuilt piece his layer-1
names. Layer-2 (A)/(B) is the PROV derivation-DAG direction (111 §0, owed-prov) plus
the Racket transplantable-metadata finding (a transform grafts input-provenance onto
output), restated as TYPE-level enforcement.

Orchestrator observation (for spike-4 seeding, not built now): much of the
"self-flow-analyzer over our own code" ambition is approximable by the project's
existing make-bad-states-unrepresentable posture — diagnostics constructible only via
a registered DiagCode; a user-text newtype constructible only from
provenance-carrying values — shrinking the genuinely-mechanical residue to (i) a
catalog-completeness test and (ii) a no-raw-text-to-user-surfaces lint. The full
Pottier-style path-enumeration stays future.

## §3 What changes in-round (small): the q-2 diagnostics slice re-spec

The queued q-2 slice (note 219: 2-3 diagnostic codes for silent `$()` degradation;
not yet dispatched, lands post-arch-2 with y-1/#11) gains three shape-requirements so
it seeds rather than fights the future tooling:
- rq-1: codes registered in ONE catalog location, message-TEMPLATE separated from
  emit sites (emit sites pass structured params; the catalog owns phrasing) — the
  dislocated-index embryo.
- rq-2: a completeness unit test — every DiagCode has a non-empty catalog entry (the
  embryonic Pottier gate; trivial now, load-bearing later).
- rq-3: no free-text-only emissions on the new paths.
Existing codes fold into the same shape only where trivial; no analyzer is built.

## §4 Fork-leans this direction supplies (kLOCKIN-reserve, not welds)

- fork-cmdsub-top-cause (219): the cause-TAGGED ⊤ option is the provenance-wrapped
  pattern in miniature — prefer it where cost-comparable; the generic option remains
  the floor if the ValueOf reshape is expensive this round.
- capture-provenance (219 q-4.c): route-through-site-keyed-class vs tag-every-ValueOf
  — same lean, provenance-shaped where cheap, behind a seam.
- dashboard seam-1 (21B): exposing per-site CommandEffect/⊤-reason is the same
  family; candidate for the post-arch-2 small task.

## §5 Round-close obligation added

The wrap report carries a spike-4 inventory: which round-11 primitives are
already-welded (Carrier/never-throw; ⊤-poison cascade-suppression; Span
origin-handles) vs embryonic (DiagCode catalog; artifact provenance comments; OOB
verdict lane) vs absent (the derivation DAG; catalog completeness gate;
provenance-typed user surfaces) — the r22 seeding document the human's lean needs.
