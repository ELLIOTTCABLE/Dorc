# 23O — Round-23 closeout: the oracle-contract crisis, its resolution, and the turn to empiricism

**The durable, frozen historical record of round 23**, written 2026-07-03 at round close. It
covers everything from `plans/230` through the golden-hill design floor as a single narrative,
for a future reader who needs to understand *what happened and what it means* without reading
the thirty-odd `23x` notes that hold the blow-by-blow. Where a claim is welded, its authoritative
home is cited. This note also **replicates the deferred-work ledger** (formerly `TODO-ADDTL.md`,
which is machine-rewritten and rots) so the deferrals survive durably. It does not repeat the
live task state (see the round-24 handoff) or the round-24 plan (see `plans/240`).

Confidence marks where a claim is not settled: `+SURE / ~SUSPECT / -GUESS`.

---

## 0. What round 23 was, in one paragraph

Round 23 opened as the "best-effort / collapsed-gradients" round (`plans/230`) and became, by
necessity, the round where Dorc's central correctness contract — how an oracle's partial
knowledge licenses skipping a command — **broke and was rebuilt from the studs.** It absorbed
five rounds' worth of work under one number: the crisis and its resolution (the ternary
verdict), a redesign of how oracles communicate verdicts (the role-split), a corpus-wide
rename, the realignment of the implementation spike to the new design, and a design-dialogue on
the hardest remaining piece (eliding past a running command). That ballooning is itself why the
round is being closed and renumbered here.

---

## 1. The narrative

### 1.1 The opening, and the interception (`230` → `233`)

Round 23 began on the two foundational arcs flagged at round-22 close: `plans/230` (best-effort
degradation / "collapsed gradients") and `plans/22H` (the live-plan streaming engine, deferred).
Work started on 230 — a state-space sweep (`notes/231`) and a design dialogue (`notes/232`).
That work was **intercepted** by the human's own crisis-log, `plans/233`, which showed that the
thing 230 was trying to gradually degrade was itself unsound. Everything pivoted to 233.

### 1.2 The crisis (`233`)

The pre-233 design gave each command a **binary** per-site verdict: `{elide, run}`. An oracle
declared the effects it knew about; the analyzer poisoned (invalidated) the facts an interfering
command touched. The break: **what does an oracle's *silence* about a piece of state mean?**
- If silence means *trusted* ("I didn't mention `fs.Path`, so I don't touch it"), a *partial*
  oracle becomes **worse than none** — `apt-get install nginx` silently writes
  `/etc/nginx/nginx.conf`, a downstream `[ -f /etc/nginx/nginx.conf ]` guard stays "ambient" and
  **wrongly elides**. This is under-execution, the cardinal sin. The spike actually shipped this
  ("safe floor, dangerous middle": a fully-unmodeled command poisons all — safe; a
  *modeled-but-partial* command poisons only what it declared — unsound).
- If silence means *poisoned* ("assume I touch everything I didn't rule out"), then every oracle
  must enumerate the entire universe of non-effects, which is unbounded and re-invalidates on
  every new kind. Nothing ever elides in a real book.

`+SURE` (confirmed against the literature in `23N`): **this is the frame problem** (McCarthy &
Hayes 1969), and it is *permanent* — no default over silence escapes it. The whole
modifies-clause / separation-logic / dynamic-frames field exists precisely *because* this is
true. 233 was not a bug to be fixed; it was a fundamental condition to be designed *around*.

### 1.3 The resolution: the ternary verdict (`233` update → `239`)

The escape was to enrich the *output vocabulary* from binary `{elide, run}` to **ternary
`{elide, guard, run}`**:
- A **guard** is an observable-preserving *insertion*: the oracle's own convergence check,
  compiled **in-sequence** immediately ahead of the original command's untouched bytes
  (`( check ) || <original command>`). It re-verifies convergence at runtime, *after* any
  interfering command has already run — so it is **frame-free by construction** and needs no
  knowledge of what the interferer did.
- **Silence licenses nothing.** It neither vouches (the unsound reading) nor collapses everything
  to run (the valueless reading); it merely fails to *upgrade* a `guard` to an `elide`. This is
  the load-bearing move: silence stops *meaning* anything.
- The license to *elide* (not just guard) is the **converged-vouch**: an explicit, sh-spelled,
  path-scoped, judgment-tier, attributed claim by the oracle author that "when this check's
  establish-set holds, I judge skipping acceptable; whatever else the command would do is noise I
  accept." Fallible, never a fact; blamed to its author when wrong.

This was crosschecked by three clean-context agents (`236a/b/c`), adjudicated in `237`. One of
them (`236b`) explored a *different* mechanism — re-observing facts at a runtime "barrier" past
the wall — which was **de-centered** into a separate placement-spectrum round (see §4). The
ceiling of what elision can achieve was mapped in `238`. The crisis was formally **closed** by
the signed package `notes/239`, which also established:

**The two-halves doctrine** (`239` §1, the governing frame): *full elision is THE goal* — the
golden hill, the attention-conserving product Dorc exists for. The *guard-half is its sister and
permanent fallback* (for un-oracled tools, lazy days, platforms nobody wrote the deep oracle
for), owed **equal design attention**. Neither is aspirational; no "someday, if possible"
language about the elision goal belongs in the corpus, and no guard-half decision may silently
discard a constraint the elide-half needs.

Authoritative homes: `plans/233` (stamped, with a correcting end-annotation), `plans/239`
(signed), and the round-23 rulings block in `spike/CLAUDE.md`.

### 1.4 The interface: how oracles speak a verdict (`23J` → `23L`)

The direction-review (§1.6) surfaced the **rc-soundness cluster**: the guard consumes a check
body's *aggregate, in-band exit code*, but the design's claims ride *per-mark, engine-interpreted*
semantics (the `!`-inversion, which-property, nothing-claimed) that are **stripped from the
shipped bytes**. A vouched inverted arm would mint a guard that skips *exactly* when the world
drifted to needing the command. Resolved, in-conversation, by three welded rulings
(`notes/23L`, `spike/CLAUDE.md`):
- **rul-role-split.** A rich oracle is a family of role-sibling functions: **`predict()`** (facts
  + prediction; its aggregate exit status keeps the incumbent predicted-rc meaning — the round-20
  elision-substitution machinery is untouched) and **`is_converged()` / `is_diverged()`** (the
  verdict function; sense declared by name; its body is the one authored place a tool exit-code
  becomes a convergence verdict). The whole oracle-to-Dorc surface is exactly two invocation
  contracts: `predict()` = "simulate the command," `is_*verged()` = "is system-state steady." No
  out-of-band verdict tokens — both have natural exit-code-shaped consumers, so forcing them OOB
  is rejected (but OOB/printf stays fine for everything else: facts, diagnostics, refusals).
- **rul-rc-partition.** A verdict function's exit status reads against one fixed table, the POSIX
  convention: **0 = the named sense holds, 1 = its complement, ≥2 = confused (always runs).** The
  minimal guard-capable oracle collapses to a one-line passthrough. Direct-sense glue is the
  welded `( f ) || cmd`; declared-dual glue is an engine-emitted lossless sense-flip
  (`( f_diverged; [ $? -eq 1 ] ) || cmd`), which *restores structural protection* for inverted
  logic. Author's negative contract: never collapse exit statuses out of a verdict function (no
  `!`, no `|| true`, mind pipeline tails).
- **strip-fidelity.** A bare-mark statement (a line that is only a mark) is an annotation-line,
  equivalent to a comment — the strip deletes the whole statement, never leaving a `:` null
  command that would clobber the body's exit code. The author's last substantive command must
  remain the last exit-status-affecting statement in the stripped body.

**"The oracle"** was defined precisely: the union of all `<cmd>.<interrogator>` bodies discovered
through complete abstract-interpretation of the constructed code-unit — it subsumes both members,
and is never a synonym for one of them. Expected authoring order: `is_*verged()` first, `predict()`
later on Dorc's hinting; docs center on `is_*verged()`.

This whole cell, worked from the literature in `23N`, has an important honesty split
(**survival vs adequacy**, §1.7) that a future reader must not lose.

### 1.5 The reconciliation: making the spike speak the design (`23E` → `23H`)

The implementation spike had drifted: it modeled oracles through a **marker fiction**
(`oracle_effect`, `oracle_kind=`, `oracle_probe_*`, `oracle_vouch_converged=`) and an internal
"st-2" probe/check split that were never design truth. Over five agent sessions (recorded in
`23E`, `23H`) the spike was realigned to the human's ground-truth (*the oracle is just sh; the
check IS the oracle; strip-only; the stripped body ships in both lanes*): the inline dialect
parses (period-named funcs, trailing marks); effects **derive** from check bodies, not markers
(`ValueClaim{Establish, EstablishInverted, Observe}` — the create/destroy "polarity" axis
retired per the human's opaque-values ruling); all 151 fixtures converted; the probe lane ships
the stripped check body per-site; the markers were **retired entirely** (~2,500 lines of dead
machinery deleted). Verified green at every integration (**123 e2e round-trips / 9 designed
xfails / 0 red**). This is the expensive (~500k-token Opus) work that leaves the spike, *right
now*, freshly aligned and the best it has ever been positioned to build on.

### 1.6 The spec and the reviews (`23A`/`23B`/`23C`/`23F`/`23G`; `23I`/`23J`)

The guard tier was **pinned before building**: 24 `guard23-*` e2e cases (9 XFAIL "here is what
it must eventually do" + floors that catch wrong builds) form its executable specification
(`23A`, repairs in `23G`), reviewed by a neutral+adversarial crosscheck pair (`23B`/`23C`),
adjudicated in `23F` (h1–h5 rulings, composition holes found and pinned). A **direction
crosscheck** (`23I`, adjudicated `23J`) then read the whole pivot: verdict *the pivot is sound*,
with the one convergent finding that **the rewritten root docs and USER_STORY narrate ahead of
the pins** (end-state design presented as present mechanics). Its most valuable product was the
rc-soundness cluster (§1.4); its findings were routed to repairs and to the human's doc queue.

### 1.7 The golden-hill floor: the elide half, grounded (`23M`/`23N`)

The final design-dialogue attacked the hardest piece — **eliding *past a running command*** (the
golden hill / attention product). It reached a *floor*, not a mechanism, and the floor is
durable:

- **233 is permanent; cage it, don't fix it.** Sound past-a-wall elision *requires someone
  promising completeness over some vocabulary*; every such promise is human and fallible; no
  design removes the need. "Design around" has three moves: **CONCENTRATE** the naked completeness
  claim into its smallest attendable home (an owner's no-synonym coherence over its *own* bounded
  vocabulary); **OPT-IN** (silence means *wall*, never trust — the trusted claim is reached only
  by an explicit grounding act, never defaulted into); **PRICE** the residue honestly at the
  professed horizon.
- **The mechanism has a name** (`23N`, verified against source): the golden-hill move —
  *footprint × backing × disjointness ⇒ elide-past-a-running-command* — **is the separation-logic
  frame rule** applied through a **dynamic frame** (a footprint computed per-run, e.g. `dpkg -L`);
  the disjointness test is **Bernstein's independence condition**; "poison" is the compiler's
  **kill** set; the whole move is **lazy code motion / partial-redundancy-elimination** over an
  abstract system-state store. **Elision is a compiler-optimization problem; guarding is a
  gradual-typing problem.** This is not metaphor — it is the actual theory, and the field's
  40-year convergence on "you must *declare or derive* a frame, with silence⇒clobbered as the
  safe default" is the strongest evidence the 233 spine is right, not a Dorc tragedy.
- **The consent-wall** (load-bearing design filter): the user consents *once*, and attention is
  spent *at* consent, *before* any mutation runs. Anything learned *after* a mutation is
  chronologically post-consent, so its attention is already gone — you cannot reach into a
  consented mind and remove a line the user already read. **Therefore attention is conserved
  *only* by pre-mutation static analysis.** Any design that re-achieves elision *after* mutation
  is instantly DOA *for the attention product* (it can still buy *performance* — the command
  doesn't run — which is a different product; see §4).
- **Survival vs adequacy** (`23N` §5, conductor-adjudicated — do not lose this): an elision rests
  on two independent claims. **Survival** ("does the fact outlast the interfering command?") is
  *settled* — a fact is **self-framing** (it can't claim more than its own probe reads, by
  construction), so backing-completeness is a non-worry and disjointness is sound *modulo
  aliasing*. The suspect risk **relocates, it does not vanish**: it lands on **adequacy** ("does
  the probe faithfully stand in for the *elided command's* full effect?"), which is precisely the
  round-23 **converged-vouch** / the **converged≠no-op** case (`dpkg -s nginx` holds, yet
  `apt-get install nginx` would upgrade). Total 233-risk is *conserved*. `23N`'s "adequacy" is a
  literature re-derivation of the converged-vouch, not a new open problem; **keep our sharper
  term**. Net design steer: spend effort on *converged-vouch quality / calibration*, not on
  backing-completeness.
- **The synonym cell, scoped** (`23N` §6): cross-provider sameness (owl:sameAs / ontology
  alignment) is **closed by design** — reverse-DNS per-provider naming was chosen to bypass it.
  Within-kind *observable* aliasing (symlinks/mounts) is **alias analysis** — the "measurement
  crack" is **dynamic points-to**: disjoint iff must-not-alias, else wall. Within-kind *abstract*
  referents reduce to declare-or-wall. Once split, the "one unresearched frontier" was
  ~fully answered.

Design artifacts: `notes/23M` (the round's terminology + landmines + changed-vs-churned ledger)
and `notes/23N` (the prior-art grounding + reading list). Both are marked *terminology and
landmines, not a design* — the elide contract (footprint spelling, grounding bridges,
entity-identity) is deliberately **un-designed**, to be learned empirically (§1.8).

### 1.8 The turn: go empirical (now → round 24)

The conclusion, and the human's decision: **the theory is exhausted at its floor, so the work is
building, not more literature** (this is `23N` §10's own verdict, and the converged-vouch's
adequacy is *calibrated-never-proven* — measurable only by running). The elide half can be built
**first**, on strawman inputs, because in the spike the **DST exec-differential is the correctness
net** (run the elided plan under mocks, diff against the bare book; a wrong elision changes the
end-state and goes red). The guard is the *production* net (production has no bare-book to diff);
it is deferrable. Round 24 is *"head off 233 by building something and seeing what happens"* —
see `plans/240`.

---

## 2. The settled law (welded — a future reader must not relitigate these)

All homed in `spike/CLAUDE.md`'s rulings blocks + the cited notes:
- **rul-ternary-verdict** — `{elide, guard, run}`; guard = the oracle's own stripped body inserted
  `( check ) || cmd`; silence licenses nothing; two nevers (never engine-synthesized sh; never
  declared output in guard position). (`233`/`239`)
- **rul-guard-license** — a guard mints only from a matching (call-site, reached converged-vouch,
  probe-verdict); the vouch never enters the fact-plane; run-delta verbs never guard; no vouch ⇒
  run. (`239`/`23F`)
- **rul-role-split / rul-rc-partition / strip-fidelity** — §1.4 above. (`23L`)
- **rul-attention-honesty** — attention is saved ONLY by provable elision; the plan render is the
  whole book in order, execute-lines never hidden; "scrappy, but correct: never hide risk." (`239`)
- **rul-divergence-proceed** — apply-time divergence is proceed-and-flag; no abort, no strict
  mode; all decisions front-load into the single approval (the attention-chronology doctrine). (`239`)
- **the two-halves doctrine** — §1.3. (`239`)
- **the consent-wall** — attention is conservable only by pre-mutation static analysis;
  post-mutation elision is DOA for attention. (`23M`/`23N`)
- **silence = wall** — the anti-233 default: silence never licenses; the trusted claim is opt-in
  via an explicit grounding act. (`23M`)
- **the atomic-command axiom** — no command disassembly; multi-operand lines are whole-line;
  granularity comes from author-written loops. (`23D` §3)
- **opaque values / no polarity** — property values are opaque booleans; the engine knows no
  create/destroy; `!` is exit-code-inversion plumbing. (`23F` Addendum 2)
- **the oracle ground-truth** — the check IS the oracle; arbitrary sh; strip-only; lifted forms
  are byte-identical substrings. (`23D` §1)

Standing pre-23 welds untouched and still binding: `kFAIL` (phase-keyed fail-directions),
`inv-must-may`, `inv-referent-agnostic`, `inv-probe-sourced-values`, order-is-sacred (no
intra-host reorder/parallelize), `kLANG` (sh-is-the-product), the mutation-analysis-impossible
ruling.

---

## 3. The state of the implementation spike (2026-07-03)

- **Built and green** (123/9/0/0): the parser, the value-flow analysis, the probe phase, the
  basic disposition engine (elide converged commands *with no wall*), the exec-under-mocks
  differential harness, and — new this round — the realigned inline dialect (`predict`-named
  bodies, marker-free, effects derived from check bodies).
- **Specified but not built**: the guard tier — the 9 `guard23-*` XFAIL cases pin what it must do;
  the emitter, the GuardLicense witness, and the gate-6 widening are not yet built. (Round-24
  Stage 3.)
- **Not built**: the elide-past-a-running-command machinery (footprints, backing, disjointness,
  grounding). This is the golden hill — the whole point of round 24, and deliberately un-designed
  so it can be learned by building.
- **Quarantined / opacified**: the coverage crate (measures elision coverage) and the H2SaLS
  corpus are behind a standing quarantine; real-world coverage measurement is gated on lifting it.
  Strawman measurement is not.

The spike is disposable by charter. Its job is to *learn*, then have its conclusions extracted
into the human-authored docs, then die. Round 24 is the last major arc before that extraction is
possible.

---

## 4. The de-centered alternative: the barrier / placement-spectrum (performance product)

`236b` proposed, and round 23 de-centered into a future design round, the **barrier**: instead of
a per-site guard, hoist the past-wall re-checks into one batched, parallel "wave" fired right
after the wall (roughly one read per *kind* — a "generation-probe"), so downstream commands
don't re-run. This is a *performance* optimization (per-site-guard ↔ hoisted-wave is the
spectrum), and by the **consent-wall it offers the attention product nothing** — a command that
"elides off the wave" was still shown at consent and its attention already spent. It is a
legitimate round (cheaper past-wall verification), but it is *not* the golden hill and is
correctly parked. It also touches the TOCTOU weld and carries an open approval-contract question
(`236b`-oq1/F14). Home: `notes/236b` + `notes/237`.

---

## 5. Deferred-work ledger (replicated durably; formerly the rotting `TODO-ADDTL.md`)

Everything below is *deferred, not abandoned* — reserved with reasons. Sorted roughly by
lock-in / consequence.

- **Reactivity / live-plan (the analyzer's "time axis") — round-25-ish, high lock-in.** Probes
  streaming concurrently from N hosts, the per-host plan re-folding *live* as they report;
  reactivity threaded through the whole kernel. The unit is *replacement*, not run/not-run, and it
  is not provably stable; the concurrency / seeded-logical-clock / per-host-accumulator seam must
  be built (`hostsim` has none). Genuinely important, *completely unexplored* territory the human
  wants Fable-class attention on *soon* (before Fable gets expensive); may reuse the round-24
  spike if it hasn't grown crufty. Seed: `plans/22H`.
- **Error-reporting + provenance pass — critical-tier, reorderable.** One N-tier, per-host-forking
  derivation DAG (`notes/110` + `plans/111`), never firmed. Likely belongs on the round-24 spike
  (rich provenance is what makes the elision yardstick legible — "why did/didn't this line
  elide") but is not a round-24 *blocker*; sequence in when convenient.
- **The Dorc language proper (strict superset) + the `unsafe` escape-hatch — high lock-in.**
  DESIGN ranks the language most-critical-for-lock-in, yet nothing defines the superset vs
  POSIX-sh; the welded must-handle set (const-prop, interprocedural, variables, heredocs; only
  `eval` is punted); the strict-dash-vs-maximum-subset axis (r23 h3 leans dash-ish / POSIX2024,
  `local` expected); and the human-required Rust-`unsafe`-equivalent hatch that loses CFG-totality
  and taints its subgraph so it can never be performantly skipped. Canonical can't-unbake item.
- **MH2 — the version layer.** Version-aware oracle verbs over a cross-PM version *lattice* (no
  join across managers); a binary-content-hash grounding gate (fail-safe on `$PATH` mismatch); a
  version *coordinate* (purl-shaped, "same version string, different bytes"). Absent from DESIGN;
  shapes the fact-domain → retrofit-hostile. Human considers it deferrable behind the sh-spelling
  work.
- **Cross-host shared-state + `kSTATE` persist-vs-recompute — high lock-in to reserve.** Write-skew,
  memo-key soundness, unreachable≠converged; the verdict-shape `(verdict, content-key, freshness)`
  is retrofit-hostile; human leans stateless (recompute from host truth, rust-analyzer-style).
  Decide the *shape* now even if built later. (The probe→apply TOCTOU half is resolved — WONTFIX
  with the identified-cause clarifier; guards catch in-book causes, open-world drift stays banned.)
- **Oracle-author DX (linter / LSP / authoring harness) — highest-leverage, low lock-in.** DESIGN
  §5 "where the bulk of the work lies"; the #1 existential-risk lever (oracle quality). r23 grew
  its backlog: the don't-collapse verdict-function lint, the branch-aware coverage lints. Buildable
  later.
- **The `.diff` verb.** The strictly-stronger, opt-in, authors-must-earn-it verb; byte-mechanics
  deferred to jq/diff/patch, only the convergence predicate is Dorc's altitude. (The rest of the
  old verb-menu is absorbed into the r23/23L rulings.)
- **Deferred surfaces (reserve the seams, don't build):** real-time streaming of remote command
  output + the dual-render TUI (degrading to plain-text controls; why-elided/why-probed as the
  primitives); `serial:`/rolling-batch non-preclusion; `retries:`/`until:` convergence-by-retry
  as first-class syntax; `sudo`/`become` execution-context first-classing — and its r23 addendum,
  **lane-privilege** (`23J`): a vouch promotes oracle code into the book's elevated context once
  `become` exists — the unnamed cell in the guard posture-shift.
- **Parked, human-keyed:** the escape-hatches (`235`: hatch-isolate/executed-but-poison-suppressed,
  bump-exclude; un-park signal = admin-recourse pressure); check-cost banding (needs a sanctioned
  data source; corpus quarantined); the converged-vouch's concrete sh *spelling* (dq-kOOB;
  human-reserved — new interplay: whether authoring a verdict-function *is* partly the vouching
  act); kind collision + evolution semantics; bootstrap-kind curation.
- **README positioning (human's voice):** *"lazy code motion for shell"* — not a metaphor but the
  load-bearing framing (`23N`); confirm it lands for a general audience, decide how much
  frame-rule depth the README carries vs. defers to `23N`. Citations: Knoop–Rüthing–Steffen "Lazy
  Code Motion" (PLDI 1992); Morel–Renvoise PRE (CACM 1979); optionally O'Hearn–Reynolds–Yang frame
  rule + Kassios dynamic frames.

---

## 6. The vocabulary (for consistent future writing)

`23N` grounded the design's concepts in their literature names. **Policy: adopt the academic term
where it is *precise*; keep the Dorc-minted term where the academic one is vaguer or would mislead
an agent into treating a settled thing as open.** Adopt: **frame rule** (the disjointness-licenses-
survival theorem), **dynamic frame** (probe-derived footprint), **self-framing** (backing = probe
read-set, complete by construction), **kill** (poison), **lazy code motion / PRE** (the whole
elide move), **blame** (horizon attribution), **Bernstein's condition** (the disjointness test),
**footprint** (already ours). Keep our minted terms: **converged-vouch / converged≠no-op** (over
`23N`'s vaguer "adequacy"); **horizon** (with "blame" for the attribution within it); **spell**
(mechanical code-plane) vs **profess** (human-facing trust-plane); **guard / elide / run** (the
verdicts). The full terminology table and graded reading list live in `23N`.

---

## 7. Pointers (what this note deliberately does NOT hold)

- The **round-24 plan** (the five-stage elide-build ladder, goals, method): `plans/240`.
- The **live task state** and session-temporal conductor context: the round-24 handoff document
  (non-durable).
- The **blow-by-blow detail** of any thread: its numbered `23x` note, cited inline above.
