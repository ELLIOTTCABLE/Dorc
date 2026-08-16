# 30B — The minispec review pair: adjudication

> Tier: conductor adjudication (Fable, 2026-08-16). Inputs: `30Ba` (Fable-class
> neutral) · `30Bb` (Sol adversarial, foreign lineage) — independent clean contexts,
> both static-only at `d31378e8`. Method: maximum skepticism; the scope-driving
> claims verified by the conductor's own eyes; the remaining concrete claims are
> verify-at-checkpoint items for whichever lane repairs them. The two reports are
> the detail record; this note is verdicts and disposition only.

## §1 — Conductor-verified (own eyes)

- **The corpus is broken at its root, today**: `minispec/Minispec.lean:13` imports
  `Minispec.LeqIsReflexive`, which does not exist (renamed to
  `JoinIsAssociative.lean`, which the root never imports). Both lanes found it
  independently; I read the file and listed the directory. A clean-cache
  `lake build` almost certainly fails; the every-day gates stayed green because the
  Lean tiers are opt-in — the exact composite `30Ba:fnd-earned-badges-are-unchecked-
  on-every-gate-that-runs` describes.
- `Minispec/Proofs/` holds only `README.lean` — consistent with `proved` never
  having been claimed; the text-grep trap (`30Bb` finding 4) is laid, not sprung.

## §2 — Credited on convergence (repair-lane verify-first items)

The full statements live in the reports; credited headline set:
`30Ba:fnd-lean-staging-never-removes-stale-files` (a SHARED cross-worktree build
root that only ever accretes — the mechanism by which §1's breakage survived a
green lane, and the reason **no banked `verify:lean` result is trustworthy until
it lands**, the `300` §2b "lake green" claims included) ·
`30Ba:fnd-battery-never-instantiates-its-own-law` (badges are evidence about the
seat function, never the law's own `Prop`; one-line-per-unit repair) ·
the seat citation is a filename+function grep and the catalogue names the TRAIT
where the laws bind `Flat::join` (`30Bb` finding 2 + `30Ba`'s render half) ·
`proved` must become engine-gated for "earned" · the committed REPORT is stale two
ways, worktree-path-dependent, un-gate-holdable, has no legend/remit/hypotheses,
and its axiom census undercounts 16→13 with a fence that explains 7 of 16
(`30Ba` verified the three missed names by hand) · the promote generator remains
absent (KNOWN — `300` §2b `fnd-promote-subcommand-missing`; the false part is the
lock header's `@generated` claim) · no Generated-freshness digest, no aeneas
rot-check · pinned renders "no paired harness" while the obvious pairings exist
(`flat_obeys_the_binary_laws` · `flat_is_associative` — three promotes plausibly
sitting free, a HUMAN call) · the four smaller pins (`Undeclared` untested ·
`Proofs/` uncensused · single-level unit walk · bare harness-name aliasing).

NOT credited as defects: the TrustedBase framing (both lanes independently call it
honest; the hypotheses are stronger than Rust's trait bounds but are SATISFIED at
both production instantiations, `Flat<String>`/`Flat<Binding>` — `30Ba` checked);
the derived-definitions linkage (genuinely real — the `#[path]`-include design is
the keep-list's headline). The forward risk both flag: an enrichment law over a
cause-excluding-`Eq` type (the `Reach` precedent) would be vacuous-in-practice and
badge-green — the enrichment charter must require production-instantiation
batteries.

## §3 — Disposition

- The repair surface SPLITS on `minispec/CLAUDE.md`'s own line: everything in
  `spike/verify/` is ordinary builder-space (`30Ba` §5's ranked list items 2–8,
  plus the smaller pins, plus the check-side half of item 1 — the
  units-vs-root-module census); the SPEC-CONTENT touches — the `Minispec.lean`
  import-list fix (the neutral's argument to DELETE the redundant hand-list, the
  lakefile glob being the real mechanism, is endorsed), the unit-prose
  `Result`-error softening, and the per-unit law-instantiation battery lines —
  are `law-spec-touch-frontier-human-only`: frontier + EXPLICIT human
  authorization, not dispatched by conductor fiat.
- Record correction banked NOW: the `300` §2b lake-green claims are marked
  suspect-until-restaged (this note is the marker; the ledger row points here).
- The repair lane is NOT auto-dispatched — the human capped new arcs; the plan
  above awaits their call (dispatch now, or successor/next-round work). Nothing
  burns: minispec is an instrument with zero product-code reach, and its two
  Earned badge claims are the only assertions currently outrunning reality.

## §4 — Process note

The pair converged from two lineages on every load-bearing finding, with the
neutral lane finding the causal mechanism the adversarial only inferred around —
the strongest crosscheck outcome this arc has produced, and a repeat of the
truncated-null lesson's converse: a REVIEWED-never-checked artifact rotted within
two days of its minting, invisibly, because its only checking lane was opt-in and
its cache accreted. Instruments need their own instruments; the `30Ba` §5 item-3
report-currency gate is the cheapest such.
