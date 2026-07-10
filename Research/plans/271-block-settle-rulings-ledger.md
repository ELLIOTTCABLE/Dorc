# 271 — Block-settle rulings ledger (living)

AI-scribed, human-adjudicated (design-rubber-duck sessions, begun 2026-07-10). This is
the **single evolving durable** for the `270:block-settle` design-pass: rulings,
direction, and the surviving strawman spellings, accreted as the human works through the
session task list below. Explicitly sanctioned as a living document by the human
(2026-07-10); plan-tier per `plans/23D` precedent — update in place, annotate
supersessions, never silently delete a ruling.

Authority: root docs and human-TYPED rulings outrank this; this ledger *records* typed
rulings, it does not create them — an entry here without "(typed)" is conductor-drafted
direction awaiting ack (silence ≠ ack). Naming discipline per `270` §1: hyphenated
full-word slugs; entries minted here are referenced elsewhere as `271:<slug>`.

Sibling deliverable: the **entity-algebra design note** (the `24C:rul-selector-pre-stdlib`
obligation, tasks 1–2 below) is its own document when authored; this ledger carries the
rulings that shape it, not its body.

## Session task map (mirrors the conductor task list, 2026-07-10)

1. adj-entity-algebra, spelling half — the authored entity/selector mark grammar
   (gates block-rebuild; needs typed ack).
2. adj-entity-algebra, seams half — the two `24S:A7` reservations (context-qualifier
   slot; kind-owner per-axis room).
3. adj-trichotomy-spelling (né `24S:A3`) — kind-owner per-axis context topology;
   the deliberate kOOB-redline reading.
4. adj-axis-vocabulary (né `24S:A1`) — ratify v1 = {user, fs-view} + ρ,
   versioned-closed, engine-owned.
5. wrapper context-function spelling — strawman the `24S` §2b surface
   (peel/axes/ρ-transform/self-vouch) in actual sh.
6. carrier declaration spelling — `24T` pin1 (which-arg-is-code, stdin shapes,
   argv-binding, carrier context record).
7. adj-capture-claim (né `219` fork-capture-claim-type) — probe-captured stdout:
   OutClaim-observation vs new claim-type.
8. adj-survival-flag-outcome (né `24S:A2` + the `--trust-footprints` rename) — the
   central trust ruling: flag gates the OUTCOME; outcome-centric name.
9. adj-stopping-point (né `24T:P-A6`) — typed ratification of the derivable analyzer
   ceiling (const-resolvability × no-escalation × no-cross-host).
10. adj-small-homes — systemd rc=255 vs the ≥2 partition; the
    probe-bodies-proved-non-mutable effect-check's home.
11. adj-payload-pins pre-read (né `24T:P-A1`/`P-A2`) — optional tail; formally owed at
    block-context implementation-planning.

## Rulings

### rul-axis-vocabulary-v1  (task 4; 2026-07-10; typed)
`24S` §3a/§7-A1 ratified as posed: v1 coordinate axes {user, fs-view} + reserved-never
{host}; ρ components {env-vars per-variable, positionals, cwd}; vocabulary engine-owned,
dialect-versioned; users never mint axes; expressibility-clause decline for wrappers that
exceed it. Rider (the human's typed shape): **fs-view is soft-deferred** — explicitly
named-and-shamed as "a hard enough problem that it could take its own entire round to get
right"; the spike does NOT attempt to get it Correct. Re-entry condition: fs-view comes
back in scope only if *specifically* needed to exercise r27 work (a Dorc language
feature); failing that, the spike-minimum is a Very Simple Fs Stub — effectively
{user, fs-straw}. The `24S` §3b honest ladder stays reserved-not-built, unchanged.
Consumers: the entity-algebra design note (context-qualifier seam only), wrapper-peel/
wrapper-sudo briefs, stdlib kind declarations.

### rul-rc-partition-stands  (task 10a; 2026-07-10; drafted-awaiting-ack)
TODO.md:23 ("systemd's 255-as-exit-fail? should we replace our >=2 to match?") answered:
**keep `rul-rc-partition` unchanged** (0 = named sense holds / 1 = complement / ≥2 =
confused-runs; spike/CLAUDE.md). systemd's `ExecCondition` ternary (0 proceed / 1–254
skip-cleanly / 255 fail; `24R:repurp-finding89`) is cited as convergent-evolution
*validation* of the ternary check-contract shape, not adopted: its layout puts every
machinery-mintable status (126/127, 128+n, timeout 124/125) inside the verdict-bearing
complement region, which is exactly what our flat ≥2 sink exists to prevent — and any
sense-flipped family member (the mechanical-inversion glue in rul-rc-partition; future
name-extensions like is_noop) would turn a machinery-minted status into a licensed SKIP
under that layout. Riders: (r1) the ≥2 region stays semantically FLAT forever — no
status inside it may acquire distinct verdict meaning; decline-vs-crash distinctions
live in the report/diagnostic lanes (runner MAY narrate well-known tool-rc's in why-lane
diagnostics); (r2) exterior coherence is a delegation-body concern — ordinary
`case $? in` remap arms; stdlib quality-bar line: delegating to a tool with a documented
non-test-shaped exit vocabulary (LSB status, terraform -detailed-exitcode) requires an
explicit remap arm (lintable); (r3) Dorc-as-tool outward contracts (`dorc plan
--exit-code`, dorc-run) are separate per-surface decisions, unbound by this partition.

### rul-effect-check-home-typeless-floor  (task 10b; 2026-07-10; drafted-awaiting-ack)
The owed static effect-check of verdict-function bodies (TODO.md:43; `24R` §0a
"machine-assist the contract") homes as a **typeless-floor brief rider**
(`270:block-rebuild`), not wrapper-peel: same seam (verdict-bearing providers threaded
into classify as data — the licensing chokepoint), lands BEFORE block-context widens the
shipped-probe surface (probe-outside + whole-ρ replication), and keeps the
referendum-carrying wrapper-peel brief lean. Failure disposition (lean): a body that
fails the effect-check does not lift at all — no probe AND no guard, one bar — with a
dq diagnostic naming the offending line. The `plans/077` runtime observe backstop stays
reserved, unaffected.

*(entry format for future additions:)*

<!--
### rul-<full-word-slug>  (task N; YYYY-MM-DD; typed | drafted-awaiting-ack)
One-paragraph ruling. Surviving strawman sh inline where spelling-relevant.
Supersedes / amends: <docID:slug refs>. Consumers: <which block/brief this binds>.
-->

## Direction & open threads

*(non-ruling steers, rejected alternatives worth remembering, and threads cut short by
a rewind — so the next context can resume mid-thought.)*
