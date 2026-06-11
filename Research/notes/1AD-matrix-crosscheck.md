# 1AD — D4 matrix crosscheck: reconciliation (task #6)

> **Disclosure:** LLM-generated (round-1A closing agent). Part of the intentionally
> quality-varied ARTIFICIAL N-of-1 testing-corpus effort (1A1 rul-1A-llm-disclosure);
> the corpus and seeds are frozen evidence, never executed (`dash -n` only). This note
> reconciles the two D4 crosscheck audits of `Research/plans/1AA-capability-matrix.md`
> and records which corrections were applied there. Append-only. Subject = THE
> ANALYZER throughout; no security-domain claims anywhere.

## §1 Dispatch record

Per 1AB §2–§3: two clean-context Fable subagents, prompts copied verbatim from 1AB §3
(SAFETY block composed to its spec — frozen-evidence / no exec / no git / read-only —
since the prior block's verbatim text lives only under quarantine), plus an
operational footer (worktree path, note filenames, and the 1AC §1 taken-on-word list
as a priority target). Both cleared the window; the compiler framing held (1AB §4
confirmed again). Neutral: ~279k tokens, 34 tool-uses. Adversarial: ~295k tokens, 37
tool-uses. Both independently read the full §4-disclosed unread zone
(`oracle`/`plan`/`solve` crates).

Reconciliation rule (1AB §2 / the adversarial-crosscheck discipline): convergent =
trustworthy; single-audit = ~SUSPECT until verified. EVERY correction applied below
was re-verified by the reconciler against the engine source / corpus lines / notes —
none was applied on an auditor's word alone.

## §2 Convergent findings (both audits; reconciler-verified; all APPLIED)

- **conv-1 — §3's "tc-F2 … is now LIVE" is wrong, twice over.** (= neutral wrong-3 +
  adversarial brk-3; verified effect.rs:114 + 301-302 + 145-151,
  crond.oracle.sh:54-56.) (a) `test`/`[` are blessed-pure and short-circuit BEFORE any
  check-set lookup, so the first-resolves-wins seam is unreachable for `test`-keyed
  providers — even with two claimants loaded, no book `test` site ever consults them.
  Corollary both audits drew independently: the seeds' `test`-keyed query cells are
  INERT at HEAD (fetched's um-validator-2 L650 guard included) — consistent with A12's
  engine grade (an-tier-a-forms/D), which the matrix already had right. (b) crond's
  `test` resolver was already commented out by the tc-F2 adjudication (1AB §1); only
  fetched keys `test` at HEAD, so no two-claimant case exists anyway. tc-F2 is
  PROSPECTIVE — it arms when grep/cmp-class read-providers collide, or when
  test-guards get routed into Queries (tc-F3) — not live. → 1AA §3 rewritten.
- **conv-2 — head-3's parenthetical is backwards; "declared-effect-no-probe" is not a
  zero-noise dialect shape at HEAD.** (= neutral wrong-2 + adversarial brk-2; verified
  eval.rs:387-398, oracle/lib.rs:234-242 + 592-605, service.oracle.sh:20, and
  `spike/e2e/cases/exec-singleton-update/pkgindex.oracle.sh`.) The service seed does
  the OPPOSITE of the claimed exemplar: its own comment says there is deliberately NO
  restart effect ⇒ restart lands Opaque ⇒ runs AND poisons. The shape's actual HEAD
  semantics, all three gates verified: (i) annotation-reached-but-no-command ⇒
  `Top(NoProbeReached)` ⇒ Opaque — the literal no-probe spelling KEEPS the poison;
  (ii) declared effect + inert check-arm command + no `oracle_probe_*` fn ⇒ the effect
  binds (poison stops) and the site runs (`resolve_probe`→None ⇒ skip-unresolvable),
  but the lift error-diagnoses MISSING_PROBE (fail-soft, loud); (iii) the
  single-selector kind-default fallback can silently wire a WRONG probe onto a
  one-selector kind. The spike's own pkgindex exemplar is the PROBE-FUL shape (real
  mtime-freshness probe; its check arm even carries the inert `test -n fresh` that
  dodges gate (i)). Net: poison-stopping-without-licensing is expressible only in the
  loud-error posture; a sanctioned zero-noise cell is missing machinery (f-1AD-1).
  → 1AA head-3 + the y-3 cheapest-path rewritten.
- **conv-3 — §4's "`cd`→PWD at 766" is mis-placed.** (= neutral wrong-5 + adversarial
  nit-1; verified value.rs:408-484 [`_ => return None`; no `cd` arm] + 762-766.) The
  `cd`→PWD/OLDPWD recognition lives in `simple_writes_var` — the Members-pass
  body-writer scan — not the transfer-clobber family; the main value plane carries a
  stale concrete `PWD` across a straight-line `cd` (latent here: no top-level PWD
  consumer in this book). → 1AA §4 line corrected.
- **conv-4 — the 1AC §1 trust-gap (w-1/w-2/w-3) is CLOSED, in the matrix's favor.**
  Both audits independently confirmed every taken-on-word relay against the unread
  crates: check-dialect `TestOp`={Eq,Ne} and the `Word` ladder
  (Unmodeled-fails-everywhere); `resolve_probe`'s exact-then-single-selector-default-
  else-None; annotation-but-no-probe ⇒ Top; plan's StatusRenderFloor blocking
  unconditionally; solve's `n*1024+4096` cap + `converged` flag + states=IN-state. The
  breaks in this note live in INFERENCES built on that zone, not in the relayed facts
  — the disclosure discipline located the risk almost exactly. → 1AA §4 footnote
  upgraded from ~SUSPECT-confirmed to +SURE.
- **conv-5 — the citation mass holds.** ~30 file:line spot-checks per audit, zero
  substantive miscites (occasional ±1-3 line drift into adjacent doc-comments); all 19
  `an-*` statuses current against ANALYZER-NEEDS.md; census frequencies verified at
  both tables; y-1/y-2/y-5/y-6 and heads-1/-2 confirmed end-to-end. y-1's
  Redir⇒Pure-not-Opaque sharpening (the matrix's headline mechanism) survived both
  passes intact.

## §3 Single-audit findings — verification verdicts

Adversarial-only:

- **brk-1 (APPLIED — the headline correction): y-3's poison-wall geography.**
  Verified: harden.sh L38 carries `$(id -u)`; cfg.rs:996-1016 lowers cmdsub bodies
  onto the main path as expansion-internal Commands ("They remain in the effect
  dataflow"); effect.rs:577 keeps them in reaching-defs while denying leaf-status; no
  seed keys an `id` provider (grep over `oracles/*.oracle.sh` providers); Opaque⇒⊤-join
  at effect.rs:464. So `Reach::Top` opens at **L38**, thirteen lines before update —
  and the matrix's own B2/y-4 rows already count L38 as a poison source, so "from line
  51 onward" was an internal coherence break, not merely an overstatement (1AC c-5
  checked L38's `exit`-arm and missed the same line's cmdsub). The unmask-everything
  rationale fails with it: with id+pkgindex seeds the A1 showcase at L54 unmasks, then
  L58's in-loop getent guard (Opaque, tc-F3) re-walls four lines later; §1 stays
  saturated with seed-unfixable Opaques (the ⊤-arg useradd, A11 plumbing, getent ×3).
  NO single seed unmasks the book. y-3's grade (low) and poison mechanics stand; its
  rationale is corrected from "first domino unmasks everything" to "cheapest entries
  in the dominant economy; first-unmasked-WIN needs the pair". Rank kept at 3 —
  head-2's cheap-poison-levers-first sequencing survives both audits (the adversarial
  audit endorsed poison-dominance while breaking the geography). The id-seed
  prescription itself: recorded as f-1AD-4, NOT asserted in the matrix (its dialect
  fit shares conv-2's gate-(i) residue — ~SUSPECT).
- **nit-2adv (APPLIED): the in-loop floor is LIFTED for the Members shape at HEAD.**
  Verified plan/lib.rs:176-183 (`LicenseVia::MembersLoop` — "lifts the in-loop
  render-floor for exactly this shape") + the `disposition_for` doc (990-993: Members
  routed to `members_disposition` BEFORE the floor returns Run at 998-1000). Resolves
  the audits' one direct conflict (§4 below). B5's "never licensed here: in-loop floor
  + self-reach" mis-listed the floor as an operative blocker for the Members loops;
  operative blockers = tc-M1's pristine-empty strictness (+ the brk-1 wall + the
  all-members-Converged and consumption gates). Conclusion ("never licensed on this
  book") unchanged. cfg.rs:140-145 + 198-202's "the member-elision slice LATER lifts"
  doc-comments are stale against plan — flagged f-1AD-3.
- **nit-3adv (APPLIED as weld): y-7's havoc-narrowing revives y-2's
  wrong-concretes.** Mechanics convergently confirmed (the havoc-all at
  value.rs:325-331 is what currently ERASES the stale constants that call-transparency
  at value.rs:386-399 would otherwise leak into guards). Narrow the havoc without
  also addressing calls and helper-written flags read stale pre-call concretes —
  latent in this corpus only because `sshd_changed` is also branch-assigned at top
  level. Weld added to y-7's cheapest path (order with/after y-2), parallel to tc-M2's
  y-1↔y-2 weld.

Neutral-only:

- **wrong-1 (APPLIED): y-7's salvage premise.** Verified parser.rs:714-722 and
  746-754: both loop-jump ⊤-rejects pass `Vec::new()` as the salvage argument — the
  corpus loops' Unsupported nodes carry ZERO salvaged children, so salvage-scoped
  havoc would havoc nothing there. The bodies ARE parsed into the arena before the
  reject (orphaned), so the narrowing survives via span-containment scan (the
  value.rs:772 `node_within` idiom), not via `salvaged`. y-7 cheapest-path rewritten.
- **wrong-4 (APPLIED): A8's "worst break" attribution.** Verified 1A6 §1: the crown
  ("as assembled, the book could NEVER complete a run") belongs to
  err-shell-snippet-rc; err-handlers-endplay is the other §1 class. A8 cell corrected.
- **u-1 (RESOLVED → APPLIED): A6's "the D1 strain champion".** Verified 1A6 §3: the
  "highest-value impedance specimen" crown attaches to imp-blockinfile-TRUNCATE (row
  A5's entry), and "champion" appears nowhere in 1A2–1A6. A6's max grade stands on the
  anchoring entry's own content (xn-13/xa-13/xa-14, both-directions divergence); the
  unsupported superlative is corrected.
- **u-2 (RESOLVED, no change): A7's "fix-2"** exists at 1A2:30 and is exactly the ufw
  proto-less rewrite. Cite accurate.
- **nit-1n (APPLIED):** y-4's partition accounted 9/11 cmdsub sites; L84 (`su_err`,
  A10's co-consumption story) and L432 (`esc=$(printf|sed)`, um-pure-1-shaped, stays
  ⊤, detached-body context) added — 11/11.
- **nit-2n (recorded, NOT applied):** "L65" vs the `$(openssl …)` at L67 — the census
  attributes a continued command to its start line; convention, not error.
- **nit-3n (APPLIED):** B5's "both literal-list" — L420's list carries `"$MAIL_PORT"`
  (a quoted-var word; resolvable, no ⊤-trigger; the for-var JOIN was ⊤ regardless with
  six distinct words). Wording fixed.

## §4 The one direct conflict, resolved

Neutral read plan/lib.rs:998-1000 as "the floor stands for ANY in-loop leaf"; the
adversarial read the same lines as "Members are routed around the floor". BOTH are
real: `disposition_for` does return `Run` for every in-loop leaf it SEES — and its own
doc (990-993) says Members-shaped leaves never reach it. A function-local reading that
was literally correct and wrong by omission — a clean specimen of why the
reconciliation rule wants source-verification, not vote-counting. Resolved in the
adversarial's favor; applied as nit-2adv.

## §5 Net disposition

Matrix verdict: fit for D4 after corrections — no yikes rank moves; y-1/y-2 (ranks
1–2), heads-1/-2, the an-* statuses, the census numbers, and the citation appendix
survive two hostile passes. What fell: y-3's geography/"first domino" rationale
(brk-1), the declared-effect-no-probe remediation story (conv-2), §3's tc-F2 liveness
(conv-1), and four smaller attributions (conv-3, wrong-1, wrong-4, u-1). Pattern in
the failures: every substantive break traces to the 1AC-disclosed UNREAD zone, or to
one same-line oversight the author partially checked — the disclosed trust-boundary
predicted the fault-lines almost exactly. Process lesson promoted to 1AE seeding
feedback: a +SURE whose derivation passes through a taken-on-word fact must inherit
the hedge.

## §6 Flags recorded here (NOT fixed; rolled into 1AE's human-surface list)

- **f-1AD-1:** a sanctioned poison-stop-without-license cell shape (conv-2) is real
  missing machinery with no `an-*` row — registry candidate alongside tc-M4's three.
- **f-1AD-2:** the kind-default single-selector fallback (oracle/lib.rs:238-239) can
  silently arm a wrong probe when a second cell joins a previously-one-selector kind —
  um-svc-1's hazard generalized; dialect/lint question, tc-F2/F3 family.
- **f-1AD-3:** cfg.rs:140-145 + 198-202 doc-comments still call the member-elision
  lift future ("later lifts") — stale vs plan/lib.rs:176-183; trivial engine-side
  comment fix, out of round scope.
- **f-1AD-4:** an `id`-Query seed as the L38 companion to pkgindex (brk-1) — plausible
  zero-engine-code poison-stop, ~SUSPECT on dialect fit (needs a probe-ful arm per
  conv-2 gate (i); um-user-2's consumed-stdout limit binds the VALUE side regardless).
- **f-1AD-5:** 1AC tc-M3 repeats the "loading both makes routing file-order-dependent"
  premise that conv-1 refutes at HEAD. 1AC is append-only and stands as-written; this
  note is its correction of record.
