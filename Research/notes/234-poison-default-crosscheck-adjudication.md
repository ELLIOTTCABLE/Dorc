# 234 — poison-default design-hole: adversarial-crosscheck adjudication (r23)

Adjudication of a 4-agent adversarial crosscheck of `notes/233` ("rubber-ducking the oracle
contract" — the poison-default design hole). Two firewalled pairs, clean-context, each member
isolated (no sibling reports, no 23x notes, only `233` + pre-r23 corpus):

- **Pair 1 — "is the problem-space complete?"** `p1-neutral` (enumerate framings 233 omits) +
  `p1-adversarial` (find where the *setup* forecloses regions).
- **Pair 2 — "internal holes in approach-4 (kind-scoped poisoning)"** `p2-neutral` (assess on its
  own terms) + `p2-adversarial` (assume flawed, prove where, concede misses).

Raw reports: `.tmp-234-crosscheck/{p1-neutral,p1-adversarial,p2-neutral,p2-adversarial}.md`
(temp; relocate or bin per human). This note is the **conductor adjudication** — I held the
positive position, cross-compared *within each pair before* concluding (per the method), and
**de-biased the two adversarial passes** (the human's explicit ask: mollify the negativity the
hostile framing instills). Everything here is process-evidence over fallible agents, NOT a
correctness claim; confidence-marked (+SURE/~SUSPECT/-GUESS/--WONDER). The one load-bearing
*code* claim I verified myself (§1); everything else is agent-surfaced and human-adjudicable.

Slug map back to the raw reports: `p1a fr-N`, `p1a opt-X`, `p1n §N` / `234-angle-*`, `p2n nk-N` /
`claim-X` / `cost-N`, `p2a AN` / `da-core` / `dn-survivor`.

---

## 0. The result in one paragraph

The crosscheck is unusually *convergent* (convergence = the signal; lone findings flagged as such).
**All four** agents independently land the same structural verdict and the same factual correction;
the two *adversarial* agents, on different tasks, independently arrive at the **same salvage**. The
adjudicated bottom line: approach-4's **goal** (scale the author's enumeration burden to *kinds you
touch*, not the universe — cheap for pure/peripheral tools) is sound and worth keeping; but its
**mechanism** — read an oracle's *silence about a kind* as proof the command can't touch that kind
(`da-core`) — is unsound *in Dorc's own terms*: it inverts the welded `kFAIL-perform` default on the
omission case, makes the cheapest oracle the dangerous one, and silently fails `story-1` across kind
boundaries. The defect is real and survives de-biasing. The salvage is also convergent: **never read
silence; buy precision with an explicit, positive, attributable kind-frame declaration** — and the
deepest framing unifies the whole thing: *silence-⇒-clean is sound iff the kind-universe is
sealed/owned/versioned, which is exactly what `wish-C` (no maintained type library) refuses.* That
is the fork to put to the human (§3).

---

## 1. The one VERIFIED factual correction — recalibrate the baseline (+SURE, I read the code)

233's "Dangerous, but easy" section says that stance (silence ⇒ poison nothing) "is close to how
the spike currently operates." **All four agents flagged this as backwards**, citing
`analysis/src/effect.rs`. I verified directly:

- An un-modeled command (no provider / no check / no effect-map row / *any* ⊤ argv word) ⇒
  `vec![CommandEffect::Opaque]` (`effect.rs:235`, `inv-top-reject`).
- `reach_transfer` (`effect.rs:747-753`): `CommandEffect::Opaque => … state.join(&Reach::Top(cause))`,
  comment verbatim: *"An Opaque ALWAYS poisons to ⊤ (the correctness floor — never lose the poison)."*
- The poison predicate (`effect.rs:576`): `Reach::Top(_) => true` — once ⊤, every downstream fact
  reads as mutated ⇒ ambient-ness lost ⇒ no elision.

So the live spike is **poison-all for unmodeled commands** (stance-A behaviour, `p1a §8`/`p1n §2e`),
NOT poison-nothing. The precise, fair picture (de-biasing the agents' slightly-too-clean "the map is
backwards"): the spike is **`D∪A` keyed on whether the command is modeled at all** —

- **modeled command, narrow oracle** ⇒ poisons only the declared `FactKey{kind,entity,selector}`
  cells, leaves un-mentioned cells alone. *This* is the "dangerous-but-easy"-shaped behaviour the
  human's "≈ current spike" intuition correctly points at (a narrow oracle doesn't poison what it
  didn't model).
- **unmodeled command** ⇒ `Opaque ⇒ ⊤ ⇒ poison-all`. Safe floor. The human's "poison-nothing" is
  wrong *here*.

**Why this recalibration matters for the whole round:** approach-4's actual *delta over the status
quo* is narrow and precise — it extends the "trust silence" behaviour the spike already has *for
un-mentioned cells inside a modeled kind* outward *to entire un-named kinds*. The status quo is
already safe on the unmodeled floor; approach-4 proposes to make that floor *unsafe* (a name-nothing
floor-oracle stops poisoning). So the design question is not "improve from the dangerous floor" (233's
framing) — the floor is already safe — it is "**is it worth trading the safe unmodeled floor for the
verbosity win, by trusting kind-silence?**" That is the `kFAIL-perform` weld question §2 lands on.

---

## 2. Pair-2 verdict — approach-4 on its own terms, de-biased

Cross-comparing `p2-neutral` and `p2-adversarial` first (per method): they **converge** on the core
and the costs; they **usefully disagree** on the probe phase, which resolves the alarm (below). The
adversarial pass over-reaches in three named places that I down-weight.

### 2.1 The core defect — SURVIVES de-biasing (+SURE, convergent all-4)

`da-core` = "non-mention of a kind in an oracle body is a sound, complete proof the command can't
mutate that kind" (`p2a §0`). Both pair-2 agents, and *both* pair-1 agents, independently reduce
everything to this. It is the **frame problem** (`099` W3: "non-effects can't be enumerated") re-hidden
one granularity up and no longer charged for. Run round-23's own CRITERION and the welds:

- **It inverts `kFAIL-perform` on the omission case** (`p2n §4`, the strongest single framing): under
  the safe baseline, a forgotten declaration ⇒ *lost performance* (priority-3, safe); under approach-4,
  *not mentioning* a kind ⇒ *auto-clear* ⇒ **wrong elision** (priority-1 under-execute — the one thing
  the weld forbids), reached by *doing less*, which is what a "be lazy" tool's users do. "Safe exactly
  when verbose, cheap exactly when unsafe" — `p2n` could not find a cell that is both wrong-elision-free
  *and* delivers the verbosity win. +SURE this is structural, not a fixable detail.
- **`hard-1`+`hard-2` are a standing impossibility proof against it** (`p2a A2`): `da-core` needs
  "name-what-you-touch" to be *complete*; mutation-unanalyzability + oracle-aging guarantee it never is.

This is the human's instinct in `233` ("hard-1") turned on approach-4 itself. It is not adversarial
bias — the *neutral* pair-2 agent reaches the identical place independently. **Do not mollify this
away.** What I *do* de-bias is the rhetoric around it (below).

### 2.2 The floor-inversion — REAL but the human already knew; the sharp part is the pincer

`p2a A1` ("kill-shot"): adding `hork.predict(){ hork --dry-run "$@"; }` flips `hork` from
poison-all-safe to poison-nothing-dangerous — the gradient runs *backwards* at `wish-B`'s entry point.

**De-bias:** this is not a discovery — `233` *states it itself* ("the dangerous floor (while an opaque
hork with NO `.predict()` would poison all, and therefor be safe)"). Calling it a kill-shot over-credits
the find. The genuinely-new and load-bearing part is `p2a A1'` (the **forced-mitigation pincer**, +SURE):
the obvious patch — "name-nothing ⇒ poison-all, like no-oracle" — re-imposes the enumerate-the-world
ceiling on *exactly the pure tools approach-4 wanted to support cheaply* (a `logger` that honestly
touches nothing now poisons-all and is useless). So **safety and value read the same signal
(kind-mention) in opposite directions; you cannot satisfy both by tuning silence.** The wart is not
patchable cheaply — that is the real finding, and it points straight at §3's salvage.

### 2.3 `story-1` holds only intra-kind — REAL refutation of a claimed benefit (downgrade "kill"→"conditional")

`p2a A2` / `p2n nk-4b` / `p1n §2`: 233 claims approach-4 handles `story-1` (the `scan_cve` retroactive
enforcement). It does — **only because the human modeled `cve_clean` as a selector *under* `apt.Package`**
(a kind `apt-get` names). Re-key the CVE fact to its own kind (`cve.Report:nginx.clean`, which a
third-party scanner would naturally declare) and `da-rule1` reads `apt`'s silence about `cve.Report` as
proof-of-no-effect ⇒ the version-bump doesn't poison the stale "clean" ⇒ the exact under-execute
`story-1` exists to prevent. `an-orientation-coercion` is *bypassed* — there's no degraded belief to
refuse-coercing; the clean fact is *minted from silence* (`p2a` reverse-prop check).

**De-bias / calibration:** down from "kill-shot" to "**the claimed `story-1` benefit is conditional on
intra-kind modeling, and `wish-C`/`wish-D` actively push authors toward the cross-kind modeling where it
silently vanishes.**" ~SUSPECT (mine): this is entangled with `an-require` (cross-command/cross-datum
edges) — which `232 §2` records the human *de-prioritized* ("degenerates to per-command edges"). So the
cross-kind `story-1` case is the unbuilt `an-require` problem wearing a poison costume; approach-4 doesn't
*create* it but does *rely on its absence* to claim a win it only half-delivers.

### 2.4 The costs — real, re-weighted

- **`p2a A3` — whole-kind POISON is too coarse (fs.Path).** REAL but **not specific to approach-4**
  (de-bias): a package install touches a data-dependent file set the engine can't name statically, so
  *any* approach either poisons all `fs.Path` (re-erects the poison-wall on the highest-frequency ops
  action) or accepts an unsound ACK. This is the pre-existing entity-granularity limit (`an-entity-
  uniqueness`), surfaced by but not caused by approach-4. The adversary concedes it's a cost, not a kill.
  Moderate weight: it guts approach-4's *headline verbosity win* on the common `install`-then-file-guard
  shape, which is where leverage is wanted (`effort-allocation`, `p2n nk-6`).
- **`p2a A4` — soundness rides the *unenforced* cross-oracle kind-identity string** (`apt.Package` vs
  `dpkg.Package`, typo/drift). REAL but **not independent** (de-bias): the adversary itself says "A1, A2,
  A4 are three faces of one defect." A4 is `da-core` reached via a mis-spelled name. Fold it in; it adds
  one true thing — approach-4 *promotes* the open/unbuilt `an-provide-equivalence` / `an-cross-oracle-
  coherence` layer from "precision-nice" to "**correctness-critical**" (a `kLOCKIN-commit` escalation the
  lean signs up for silently).
- **`p2n cost-1` — the lattice change is high-lock, not a knob** (+SURE): shattering the single absorbing
  `Reach::Top` into a per-kind poison map *plus* a retained global ⊤ for the no-kind case, and the engine
  must decide "did this engage kind K?" *inside the transfer function* — which is the undecidable
  `da-core` predicate (`p2n nk-1c`) dragged into the kernel. Comparable to the `FactKey` re-key the
  spike already paid once (`16Q §1`).
- **`p2n cost-2` / `p2a A3-other-user` — `kSILO` pull** (~SUSPECT): the four verbs
  (ESTABLISH/ACK/OBSERVE/POISON) are an oracle-only dialect with no in-book analogue, and as inline `:`-puns
  are the off-ramp-hostile non-inert sh `17O` F-OFFRAMP flagged (`kTYANNOT`/`dq-kOOB` territory). Safety
  migrates book→oracle; the book-alone off-ramp degrades.
- **`p2n nk-7` — POISON and "forgot a cell" are observationally identical** in the lattice, so tooling
  (`an-enrichment-nudge`) cannot lint the dangerous omission without also flagging deliberate conservatism.
  The baseline's explicit `: T~` at least made "considered" syntactically distinct from "never thought
  about it." Real, low-weight, and *fixed for free by §3's explicit frame*.

### 2.5 The probe-phase alarm — RESOLVED by the within-pair disagreement (de-bias: fence-to-state, not a break)

`p1a fr-3` and `p2n nk-5` raise alarm: the probe (`kFAIL-withhold`) side is "absent," and ACK ("I checked,
I don't mutate") reads dangerously like the *banned* "vouch-makes-a-probe-safe" claim (`rul-mutation-
impossible`). But `p2a N1` **tried this attack and conceded it doesn't land**: probe-inertness comes
*only* from structural self-vouch, independent of the effect/poison model; approach-4 touches poison
(apply), not probe-shippability. **Adjudication (the cross-comparison pays off here):** there is **no actual
probe regression**. What's true is narrower and is a *documentation* obligation: `233` omits the guardrail
sentence, and its ACK gloss is ambiguous. Carry it as a **fence to state**, not a hole — *"poison-scoping is
an apply-only dial; probe-shipping stays self-vouch-only regardless of any completeness/ACK claim"* (`p1a`
weld-sentence). A reader who reaches for "the oracle vouched completeness, so trust it enough to ship its
declared-inert commands in the probe" has crossed `dc-probe-NOT` — the map should forbid that in one line.

---

## 3. The convergent salvage — CROSS-confirmed, the most important output (+SURE on the shape)

`p1-adversarial opt-K` (from "completeness") and `p2-adversarial dn-survivor` (from "attack approach-4")
**independently land the same fix** — two isolated agents, different tasks. Per the method, cross-pair
convergence on a *constructive* answer is the strongest trust signal in the whole exercise.

**Keep the spike's safe floor (silence / unmodeled ⇒ poison-all). Buy precision back with an *explicit,
positive, totalizing, attributable* kind-frame declaration — never from silence.** I.e. an oracle that
genuinely touches nothing writes *one honest line* ("I have surveyed the kind-universe; I touch exactly
{}" — or {K1,K2}), distinct from *incidental mention* and from *absence*. Then:

- the pure `logger` gets its elision benefit cheaply (one line) **and** safely (a declared, lintable
  contract, not a fact minted from silence) — defusing the A1'/A1 pincer;
- omission still fails toward poison (A1/A2 defanged — silence is never read as clean);
- the precision claim sits on the *same* `an-claimed-vs-proven` trust-tier as every other oracle claim
  (a contract a linter can challenge), instead of being promoted to a clean static fact (A4 defanged);
- the gradient's monotonicity is restored (no declaration ⇒ floor; declaration ⇒ strictly more skips,
  never fewer/less-safe) — `wish-B` honoured *honestly*.

This is `kBURDEN-user-declares` paid honestly, vs approach-4's attempt to get `kBURDEN-user-declares`
precision at `kBURDEN-we-infer`-from-silence cost. It **sacrifices approach-4's headline** ("`cp.check`
names nothing and auto-clears") — but that headline *is* the unsound part.

### 3.1 The deepest framing — the fork to put to the human (unifies pair-1 and pair-2)

Approach-4's "silence ⇒ clean" is **sound if and only if the kind-universe is sealed: every kind has an
owner who declares its *complete, versioned* selector vocabulary** (`p1a opt-I`/`opt-K`, `17N` opt-2
RAL-enum; `an-version-coord`/`an-content-hash`). In a sealed world, "I don't mention K" is a *complete*
statement relative to the closed vocabulary, and `story-1` arrival becomes an explicit **kind-version bump
that lapses prior completeness-claims to ⊤** (`222` m-3 "declaration lapses to door-1-only with a notice")
— sound, attributable, no enumerate-the-world. But **`wish-C` (no maintained type library; fully dynamic,
opaquely-declared kinds) is exactly what keeps the universe OPEN**, and an open universe is what makes
silence un-trustable (`p2n claim-E`, `p1n 234-fix-A`). So the genuine decision is:

> **FORK (tc-poison-default):** relax `wish-C` toward *owned / sealed / versioned* kinds — at which point
> silence-based clearing becomes sound and approach-4's intuition is largely recovered — **OR** keep
> `wish-C` open and dynamic, in which case silence can *never* be trusted and precision must be bought with
> the explicit positive frame (§3). You cannot have open-dynamic kinds *and* trust-silence soundly.

The corpus already leans toward a hybrid here that `wish-C` over-states away (`p1n 234-fix-A`):
`effort-allocation` endorses *bootstrapping the ~40-50 highest-frequency oracles centrally* and a blessed
core ontology (`055` "reserve the representation"; `an-fact-domain`). A small blessed/owned core + an open
tail is the sealed-where-it-matters middle; `233`'s flat `wish-C` forecloses it by fiat.

---

## 4. Pair-1 verdict — the problem-space is *one axis*; the foreclosed dimensions

Cross-comparing `p1-neutral` and `p1-adversarial`: strong convergence. Both find `233` competently maps a
single axis — *poison granularity × default-direction* — and presents it as the whole space; both caught
the §1 factual error; they offer complementary foreclosed dimensions. **On its declared axis the four
positions ARE the corners** (both agents say so plainly — no fifth corner on that axis; `p1a §7`). The gap
is everything the axis holds fixed. The dimensions the human should fold in, ranked by my read of leverage:

- **`234-D1` · poison is a property of a *site-in-a-book*, not of an *oracle in isolation*** (`p1a fr-1`,
  the highest-value reframe; corroborated by `p1n §2e` granularity + `inv-site-keyed-results`). The
  soundness obligation is `an-elision-predicate` over the book's reaching-defs, not an inventory of what
  every loaded oracle *might* touch. **Demand/consumer-anchored poisoning** (`opt-E`, `kFACTS-on-demand`):
  only poison a cell some consumer in *this* book actually reads. `story-1`'s burden then materializes
  *only in books that run the cve tool* — the "meaningless logging oracle must disclaim an ocean" nightmare
  is an artifact of refusing to condition on the book. **Honest bound (the agent's own, +SURE):** demand
  shrinks `story-1` hugely, `story-2` not at all (an opaque tool's victims are unbounded). Note for the
  positive position: demand relieves the *burden* but does **not** rescue silence-⇒-clean — a present
  consumer still needs the silent mutator to have poisoned, which silence won't do (§3 still required).
- **`234-D2` · the frame axiom is already WELDED, so "enumerate-the-world = correct" is mis-anchored**
  (`p1n §1`, `p1a opt-K`, both +SURE). `099` W3 (closed-world-over-effects: assume-unchanged-unless-declared)
  is the floor. Stance-A doesn't *exceed* it — it *fights* it (asks authors to declare non-effects, which W3
  says is unbounded). So `233`'s "correct vs best-effort" spine is miscut; the real axis is "*how much
  precision above the welded W3 floor, at what granularity, for what burden.*"
- **`234-D3` · the question is PHASE-KEYED** (`p1n §2d`, `p1a fr-3`). `wish-A` (apply, never wrongly elide)
  and `wish-F` (probe, never mutate) are the two opposite `kFAIL` directions sitting flat in one wish-list.
  Apply wants the *aggressive* poison default; probe is insensitive to under-poison (it cares about
  probe-inertness, a *separate self-vouch* mechanism — §2.5). The four stances silently answer only the
  apply question.
- **`234-D4` · `story-2` has existing machinery `233` doesn't reach for** (`p1n §2a`, `p1a opt-J`).
  `an-provide-equivalence` / `an-managed-vs-runnable` (`09A §2`, the m×n abdication) reframes `story-2` from
  a poison-default problem to a *grounding/identity* one (two providers, one declared-equivalent kind); and
  **runtime-traced footprint** (`an-undeclared-net` seccomp backstop, eBPF harness, `kDEPS`-trace) lets the
  system *observe* what an opaque `hork` touched instead of statically assuming it touches everything —
  collapsing "poison the universe" to "poison what we saw + ⊤-residue" (bound: post-first-run only).
- **`234-D5` · the claimed-vs-proven trust plane is collapsed** (`p1n §2c`): a *claimed* exclusion
  (`: …held~`) and a *proven* Tier-A fact (`[ -f X ]`) are treated as equally load-bearing; the natural
  best-effort gradient *of the exclusion itself* is unavailable (`an-claimed-vs-proven`). This is the same
  trust-cell r23's own `230 §2` exemplar is about.
- **`234-D6` · smaller, real:** the **admin/engineer exclusion-check** — `233` is written entirely from the
  oracle-author seat; the admin's Tier-A book-guards are a poison/anchor source and the admin is who silently
  de-optimizes under retroactive-tightening (`p1n §2b`/`§5 x1`, `kSILO`). And `233` reasons about
  *establish/ack* but the poison problem is fundamentally **kill-set computation** (`p1n §4`) — the
  vouch/poison metaphor hides that the real choice is "what does omission mean for the *kill* relation."

**The meta-move both pair-1 agents end on:** stop asking "what must an *oracle* declare?" and ask "**what
must the engine *prove* at an elision site?**" — at which point most of the four-stance tension stops being
load-bearing (the book's own CFG supplies much of the proof for free, no author obligation).

---

## 5. De-biasing ledger (explicit, per the human's ask)

What I down-weighted from the adversarial passes, and why — so the negativity doesn't over-read:

- `p2a A1` "kill-shot" → **known** (233 self-flags the floor-inversion); credit moves to the *pincer* A1'.
- `p2a A2` "kill-shot" → **conditional refutation**: real, but the `story-1` benefit only *fails* under
  cross-kind modeling; intra-kind (the human's own example) it genuinely *works*. Downgrade kill→conditional.
- `p2a A3` (fs.Path) → real cost but **not specific to approach-4** (pre-existing entity-granularity limit).
- `p2a A4` (kind-identity) → **not independent**; a face of `da-core` (the adversary concedes this). Adds one
  true thing (lock-in escalation), no new soundness hole.
- `p1a/p2n` probe-phase alarm → **does not land** as a regression (`p2a N1` concedes); it's a fence-to-state.
- General: the adversarial "the corpus already rejected this exact pattern" (`p2a §1`, `09A §4` "absence
  licenses nothing") is rhetorically strong and *directionally* correct, but `09A §4` was about a `trap`
  *detector*; approach-4 is a *declared default*, a softer relative. The analogy holds in spirit (silence
  licenses nothing), not as a verbatim re-commit of a ratified rejection. ~SUSPECT.

What I **refused** to mollify (it survives de-biasing, convergent across neutral+adversarial+code): the core
`da-core` defect (§2.1) and the `kFAIL-perform` inversion on omission (§2.2). De-biasing is *calibration*,
not flipping to positive — and here the calibrated verdict is still substantially negative on the *mechanism*
while positive on the *goal* and the *salvage*.

What I held for the **positive** position and could NOT rescue: I tried to use `234-D1` (demand) to save
silence-⇒-clean and it doesn't (§4 D1 note); the only world where silence is sound is the *sealed-kinds* one,
which `wish-C` forecloses (§3.1). So the positive case for approach-4 *as written* does not close; the
positive case for its *goal* routes entirely through §3's explicit-frame salvage.

---

## 6. What I'd surface as the human's decisions (tc-flags)

- **`tc-poison-default` (the fork, §3.1):** relax `wish-C` toward owned/sealed/versioned kinds (silence
  becomes sound; approach-4's intuition recovered, high `kLOCKIN`) **vs** keep `wish-C` open (silence never
  trusted; precision via explicit positive kind-frame, §3). The single load-bearing call.
- **`tc-explicit-frame-shape`:** if the salvage (§3), how is the positive "I touch exactly {…}" frame
  *spelled in sh* — and does it ride the same `kTYANNOT`/`dq-kOOB` surface as the §232 completeness-vouch?
  (It is the *same family* of "positive considered-this mark" — likely one mechanism, not two.)
- **`tc-demand-poison`:** adopt site/demand-anchored poisoning (`234-D1`) as the burden-relief lever
  (orthogonal to the fork; relieves burden either way). Low-med lock-in (query strategy over existing
  reaching-defs).
- **`tc-story2-mechanism`:** route `story-2` through `an-provide-equivalence` + runtime-trace (`234-D4`)
  rather than the poison-default — it is a grounding/identity problem, not a poison-granularity one.
- **`tc-probe-fence`:** add the one-line weld to `233`/DESIGN — *poison-scoping is apply-only; probe-shipping
  stays self-vouch-only regardless of completeness/ACK claims* (§2.5).
- **Doc fix:** `233`'s "≈ how the spike currently operates" mislabels the unmodeled floor — it poisons-all,
  not nothing (§1). Worth a `[REVISED]` note on `233` so the baseline isn't carried forward wrong.

---

## 7. Confidence + provenance

- **+SURE, verified by me:** §1 (the `effect.rs` poison-all-unmodeled mechanism).
- **+SURE, convergent across all 4 + code-cited:** the `da-core` core defect (§2.1); the `kFAIL-perform`
  omission-inversion (§2.2); the four-positions-are-one-axis completeness shape (§4).
- **+SURE, cross-pair-confirmed (2 isolated adversaries, different tasks):** the explicit-frame salvage (§3).
- **~SUSPECT (mine, adjudicated):** the sealed-vs-open fork as *the* unifying frame (§3.1) — it's my synthesis
  of `opt-K` + `dn-survivor` + the `wish-C` analysis, not a verbatim agent claim; I find it tight but the
  human should test it. The cross-kind `story-1`/`an-require` entanglement (§2.3).
- **Process caveats:** agents read live code beyond the reading-list (effect.rs, oracle/lib.rs) — useful but
  means some claims are spike-state-specific (line numbers drift; re-confirm before editing). I did not
  re-verify every cited line, only §1. No 23x notes reached the agents (anti-priming held). The human's own
  self-flagged footguns in `233` (the floor-inversion) DID prime the agents toward it (it's in the artifact) —
  hence my de-bias of the "kill-shot" framing on a known point.

*Standing reminder honored: this is AI process-evidence, not a correctness proof — only human
battle-testing settles whether the salvage actually holds. The crosscheck widens coverage; it does not
license a "the design is good/bad" verdict on its own.*
