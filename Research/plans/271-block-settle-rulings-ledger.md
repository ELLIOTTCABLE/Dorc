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

### rul-rc-partition-stands  (task 10a; 2026-07-10; TYPED — "your analysis of the rc
seems correct. I buy your arguments"; riders r1–r3 bundle-acked therein)
TODO.md:23 ("systemd's 255-as-exit-fail? should we replace our >=2 to match?") answered:
**keep `rul-rc-partition` unchanged** (0 = named sense holds / 1 = complement / ≥2 =
confused-runs; spike/CLAUDE.md).

**rul-zero-one-inversion-pair** (the human's sharpening, typed same date; his coinage
"ruling-0/1-inversion-pair", slug-normalized): the verdict-bearing statuses are exactly
{0, 1}, an inversion-pair — a positive-sense member licenses on 0, a sense-flipped
member licenses on 1 via the mechanical flip. These two are the ONLY statuses that can
ever carry a verdict — hence the only statuses that can ever produce a skip (elide /
guard-pass / any non-run) — in any current or future decision table. ≥2 is
meaningless/error/warn/anything-else, is NEVER inverted under any future design, and
can never license anything: a ≥2 answer always falls through to run, un-verdicted.
systemd's `ExecCondition` ternary (0 proceed / 1–254
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

### rul-effect-check-home-typeless-floor  (task 10b; 2026-07-10; TYPED — "I ack your
10b arguments; let's roll it into the typeless-floor")
The owed static effect-check of verdict-function bodies (TODO.md:43; `24R` §0a
"machine-assist the contract") homes as a **typeless-floor brief rider**
(`270:block-rebuild`), not wrapper-peel: same seam (verdict-bearing providers threaded
into classify as data — the licensing chokepoint; the effect-check is one more predicate
at that gate), lands BEFORE block-context widens the shipped-probe surface
(probe-outside + whole-ρ replication + per-inner payload checks), and keeps the
referendum-carrying wrapper-peel brief lean. **Sizing rider (typed): low stakes — build
only if it falls out nearly-free at that seam; punt-empowered, zero guilt** ("not worth
the brainspace" beyond that; DX defaults out-of-scope for the spike). Trust framing
(three layers): the static check catches the shell-visible accident class ONLY (stray
writes/redirects/mutators in the body — the paranoid-author bug class); the
tool-invocation interior stays vouch-tier forever (frame problem — the oracle self-vouch
is not eliminable); the `plans/077` runtime observe backstop stays reserved as the
someday third layer, unaffected. Rider details: the sanctioned OOB write lanes
(`$DORC_REPORT`-class breadcrumbs) carve out as non-mutative-for-this-purpose or every
stage-4 refusal idiom fails the check; tmpfiles NOT blessed in v1; the rider brief
verifies against `24C`'s actual classify/effect shape rather than assuming it.

### rul-no-mutating-guards  (task 10b; 2026-07-10; TYPED, emphatic)
One bar, permanently: a verdict-function body that fails the non-mutation proof lifts
NOWHERE — not as a probe and **not as an apply-time guard**. The human, verbatim: "We
*do not* insert *mutation* that we just proved to be mutation, out-of-order, in
not-user-spelling, into apply bodies. Extremely nope." (The harm-0 argument: apply-phase
mutation is the user's own lines; a guard is Dorc-inserted code, and a mutating guard is
mutation Dorc authored into the apply that the bare book never had.) Kills the two-bar
option ("good enough to guard but not to probe") forever.

### rul-proven-mutation-fails-fast  (task 10b; 2026-07-10; TYPED, gentle-lean →
consistency)
Disposition table for the effect-check, superseding the earlier draft's uniform
no-lift: **proven-mutates ⇒ genuine fail-fast** (plan-time, before network, loud —
dictate-tier), *whatever the provability source* — structural (shell-level: a real-path
write-redirect; permanent, corpus-independent) or oracle-sourced (a loaded oracle
declares a body command mutative; corpus-RELATIVE). Conductor-framing within the
ruling: this mints no third hard-error category — a function whose __role name declares
read-only-answering and whose body provably mutates is an instance of the standing
"declarations that genuinely contradict each other" category. The corpus-flip edge
(a previously-green book hard-errors when a newly-loaded oracle proves a latent breach)
is ACCEPTED for consistency — justification on record: the flip discloses a standing
violation (the book was always mutating in the probe lane), not a regression. The two
provenance classes stay SEPARATED in diagnostics/docs (the human's gut, typed):
structural proofs are permanent and near-warrantable; oracle-sourced proofs name the
proving oracle in the dq and are never marketed as coverage. ~~Residual class: unprovable
stays NO-LIFT~~ — **SUPERSEDED 2026-07-10 (human soft-hold: "every command in an oracle
must also have an oracle" is chicken-and-egg AND gradual-enhancement hostile) by
`rul-unprovable-rides-the-vouch` below.**

### rul-unprovable-rides-the-vouch  (task 10b; 2026-07-10; TYPED — "Okay, acked.
Durable" — replaces the superseded no-lift residual)
The **unprovable** region of a verdict-function body (a foreign command with no loaded
effect-knowledge; a ⊤-valued redirect target) **ships on the authored vouch, exactly as
today** — probe AND guard, no lift-refusal, no error. Grounds: the oracle lane's
probe-license was always the VOUCH, not proof (DESIGN: "vouched-safe-to-run … either by
the author, or by provable Dorc-provenance" — two license sources; the under-approximate
"better to ship no probe at all" mandate governs the Dorc-provenance path, i.e. lifting
admins' in-book guards, which stays proof-gated and is NOT loosened by this ruling). The
effect-check is therefore **falsification-first, never a completeness gate** (`24R` §0a
says machine-ASSIST): it fail-fasts on proven contradiction
(`rul-proven-mutation-fails-fast`), it may positively report fully-verified bodies, and
third-party silence never revokes a first-party license. Riders: (r1) each unprovable
command earns a hint-tier note (author-facing lint surface primarily, kWARN-rich):
"outside machine verification; your vouch carries it" — with the standard
enhancement-pointer shape (a read-blessing for that tool would verify the region);
(r2) verified-vs-vouch-carried is a REPORTABLE distinction per body — the honest answer
to watch-machinery-raises-disclaim-burden is "Dorc verifies what it can and tells you
which parts rest on your word," never warranty language; (r3) the filter-tool cohort
(jq/awk/sed/cut in check pipelines) is the expected common unprovable case — the
stdlib's existing read-blessing species (the stage-1 "vouched probe-safe reads") covers
it opportunistically, but is never a precondition for an oracle to lift;
(r4) **vouch-scope-is-the-body-never-the-tool** (the human's sharpening, typed same
date, né "temporary-other-vouches(???)"): the body-vouch is a claim about a REGION OF
SH — "everything I typed inside this contracted non-mutative block" — never a claim
about the command families it happens to invoke. It therefore mints NO command-family
fact: an unmodeled command carried in one oracle's body stays ⊤ everywhere else — in
books it remains a full poison-wall (unmodeled-drives-to-⊤ law unchanged), and it gains
nothing in any other oracle's body. Non-transitivity is automatic rather than ruled,
because no fact about the tool was ever asserted to transit. Exactly one vouch reaches
OUT of its file: a tool's own oracle reaching that tool's book-sites — and it reaches
precisely because it IS a claim about the command family, argv-keyed. Two claim-species,
two scopes; they never convert into each other.

*(entry format for future additions:)*

<!--
### rul-<full-word-slug>  (task N; YYYY-MM-DD; typed | drafted-awaiting-ack)
One-paragraph ruling. Surviving strawman sh inline where spelling-relevant.
Supersedes / amends: <docID:slug refs>. Consumers: <which block/brief this binds>.
-->

## Direction & open threads

*(non-ruling steers, rejected alternatives worth remembering, and threads cut short by
a rewind — so the next context can resume mid-thought.)*

- **watch-machinery-raises-disclaim-burden** (human observation, 2026-07-10, task 10b —
  "not a nack"): the effect-check "leans perilously close to promising non-mutation";
  every layer of machinery *supporting* the user here raises the documentation/education
  bar for disclaiming that the machinery isn't *enough*. Standing education posture:
  warranty only the structural class; frame everything else as lint-against-accidents;
  the authored vouch remains THE contract. Same shape as the `--trust-footprints`
  "marketing at best, theatre at worst" honesty note — watch it wherever new
  contract-supporting machinery lands.
- Conversational-strawman discipline (conductor note-to-self): function names in
  strawmen use the bare munged `tool__role` form per `24M` (`foobar__is_converged`),
  not single-underscore.
