# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/CLAUDE.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
> findings get a durable numbered-note home; this file carries pointers.
>
> **How to maintain:** update judiciously — direction-changes, discoveries, refutations,
> deferments; never per-turn chatter. The bar: nothing lost to a context-collapse. Density is
> **vaguely logarithmic in age**: the newest work sits at the TOP, rich enough for a new
> conductor to skill up on; day-old context compresses to a paragraph; older history decays to
> a line, a pointer (a note slug, `git log`), or deletion. Reverse-chronological, always.

---

## NOW (2026-07-02 — end of the round-23 crisis arc)

**Direction-crosscheck COMPLETE:** a Fable neutral+adversarial pair reviewed the whole r23
pivot (incl. the human's `a92ad31` root-doc rewrite + the new USER_STORY.md); both verdicts:
*the pivot is sound; the seam is docs-narrating-ahead-of-pins.* Reviews `23Ia`/`23Ib`,
adjudication **`23J`** (headline product: the **rc-soundness cluster** — the guard consumes
aggregate in-band rc while design claims ride per-mark engine-interpreted semantics; three
facets, one gap; chartered into the vouch-spelling round). Repairs → task #15
(post-closeout); the human's root-doc queue → task #16.

**State:** the `plans/233` design-crisis is **formally closed**. The verdict architecture is
ternary — **{elide, guard, run}** — where a `guard` inserts the oracle's own stripped check in
front of the original bytes (`check || command`), silence licenses *nothing* (it only fails to
upgrade guard→elide), and the frontloaded trade stands (attention surrendered where the world
is undescribed; guards rescue perf/safety/monotonicity at new cost). Rulings are welded in
**`spike/CLAUDE.md`'s round-23 block** (rul-ternary-verdict · rul-guard-license ·
rul-attention-honesty · rul-divergence-proceed) + the TOCTOU identified-cause clarifier + the
inv-probe-sourced-values guard carve-out; KNOBS carries the kELISION caution + kSILO shove.
Guard-tier behaviour is **pinned before build** — 24 `guard23-*` e2e cases (9 xfail + floors;
registers `notes/23A` + `23G`), reviewed by a neutral+adversarial pair (`23B`/`23C`,
adjudicated **`23F`**), repaired, conductor-verified **123 round-trips / 9 xfail / 0 XPASS /
0 red**. The compilation tree is corpus-clean (coverage crate opacified; historical
corpus-touching notes quarantined).

**The atomic re-spelling session — PARTIAL LANDING 2026-07-02, honest STOP** (record + the
mechanical remainder-recipe: **`notes/23H`**; conductor re-verified 123/9/0/0 + gates green).
LANDED, all additive/always-green: the **R2 derivation core** (`check/derive.rs` — structural
walk over inline `case $verb` arms + marks reproduces the retired-marker effect-map;
**ValueClaim{Establish, EstablishInverted, Observe} replaces Polarity — no kill/creation axis**,
per the cov-q4 FINAL ruling in `23F` Addendum 2; 5 differential tests prove derive == old-lift;
`derive` is deliberately dead code until wiring); an R1 lexer-gap fix (mark after fd-dup
redirect); **146/151 fixtures** converted additively (markers RETAINED for the differential
two-step; golden-stable at every batch). SECOND SESSION (finisher, 2026-07-02, conductor-verified on
the merged tree — e2e 123/9/0/0, all 23 workspace suites green): **the SEMANTIC CORE IS
LANDED** — the 5 pkgindex Singletons via the ruled empty-entity mark (`pkgindex:.fresh`,
deliberate parse + near-miss-typo tests); the corpus-wide `derive==markers` flip-gate over
all 151 fixtures (it CAUGHT a real R4a bug — an OBSERVE-vs-ESTABLISH mark on one tool
oracle); **Polarity retired workspace-wide** (`ValueClaim`; the transitional-freeze ru-26
code-note lives at `analysis/src/effect.rs::cell_effect` on the EstablishInverted arm); and
the WIRING FLIP (cli+coverage effect-maps now derive from check bodies, not markers).
REMAINING, blocked on **ask-probe-divergence** (human ruling): R3's lane-swap revealed that
the authored check-body probe COMMANDS textually diverge from the retired `oracle_probe_*`
bodies (`--` operands, dropped `--quiet`, firewall's old probe is a PIPELINE the mark grammar
cannot annotate — a real dialect gap, flag `jc-mark-on-pipeline`), so completing R3 means
re-authoring mocks + re-blessing goldens together — the finisher refused to co-author
ground-truth and prediction in one unverifiable pass (anti-masking) and reverted its working
R3 code to stop at a green boundary. Options + the refined recipe: `23H §7`. THEN: R3
re-land (~1 pass), P5 marker retirement (+ the ruled `dpkg.check()`), guard23 vouch-marks,
big-bang re-bless with case-by-case inspection + per-xfail lens-verify.

**The bindings (hold these; full text at the cited homes):**
- The four round-23 rulings — `spike/CLAUDE.md` (guard sourcing: the check IS the oracle,
  strip-only, whole-body both lanes; lifted forms byte-identical substrings; two nevers).
- Two-halves doctrine + anti-creep — `notes/239` §1 (full elision is THE goal, never
  aspirational-tier; guard-half = sister + permanent fallback, equal attention).
- Oracle ground-truth — `notes/23D` §1 (strip-only; arbitrary sh; analyzer-trick-not-language).
- Plan-surface, attention-chronology, atomic-command axiom, can't-serve rulings — `23D` §2–§4
  (plan-is-the-code render; no late attention-demands ever; no command disassembly).
- Interim rc-consumer posture — guards mint only where NO explicit status reader exists;
  errexit-implicit is OPEN, unpinned both ways, needs build-phase experimentation (`23D` §3).
- The vocabulary law + elide-half seeds — `23D` §5 (~SUSPECT tier; adversarial pass OWED).
- Coverage/re-spelling rulings — `23F` Addendum 2. Guard-license + h1–h5 rulings — `23F`.
- Statuses: `plans/233` STAMPED (end-annotation = correction channel). KNOBS was
  conductor-editable-with-review THIS session only — re-confirm. DESIGN/IMPLEMENTATION are
  human-only (his rewrite pending, low-spoons — do not nag).

**The arc ahead:**
1. **Integrate the in-flight atomic session** (verify skeptically: gates, harness 123/9/0/0,
   lens-checks, differential evidence; then task #14 closes).
2. **THE BUILD SLICE — the next conductor's spine:** flip the xfails green honestly. ENTRY
   GATE (23J, direction-crosscheck): refresh ANALYZER-NEEDS.md FIRST (stale rounds 19–23;
   rows contradict the polarity-retirement ruling) and honor the **rc-soundness requirement**
   — the guard-witness gains a structural rc-soundness component (aggregate rc-0 ⇔ the
   vouched establish-set holds on the runtime-reached path; refuse-to-guard loudly
   otherwise), chartered into the vouch-spelling round BEFORE any vouch-consuming xfail
   flips (full cluster: `23J` conv-rc-soundness — the mark-`!` inversion trap, refuse-path
   rc-0, rc-vacuous fact-reporting bodies). Then sequence per `23A` §5: widen gate-6 FIRST
   (no license class yet for apply-only checks / guard-suppressed mutators; selftest
   confounds landed in `23G`); the guard emitter per the round-21 door-4 mechanics
   (`notes/218a`), the GuardLicense witness (mind **hz-refusepath** — the reached-path
   component is load-bearing), the strawman vouch lift. During: the rc-consumer experiment;
   guard render-forms.
3. **Placement-spectrum design round (task #11, commissioned):** per-site guard ↔ hoisted
   post-wall wave. Wall-density cost-model FIRST (density endogenous; NO corpus route);
   single-approval, no-late-attention, no mid-apply re-planning; quiescence named;
   adversarial-crosscheck before ANY re-weld. Concludes with the wave re-welds + wave pins.
4. **Elide-half design round** (equal standing; when the human wants it): seeds = `238`
   (horizon, derivation gradient) + `23D` §5 (footprints, demand-disjointness, namespace
   convention, grounding bridges, entity-aliasing). All ~SUSPECT until its own hostile pass.
   NB (23J / 23Ib-fd10): this arc OWNS the repair of 233 §0's original unsoundness, which is
   design-closed but STILL LIVE AT HEAD (the dangerous-middle; `23A` hz-ambient-hole goldens
   encode it) — the missing pin lands in the 23J repair pass (task #15), but the FIX rides
   here; scheduling this arc is scheduling that repair.
5. **Parked, human-keyed:** escape-hatches (`235`; un-park signals: admin-recourse pressure or
   bump-mode work); check-cost banding (needs a sanctioned data source; corpus QUARANTINED);
   the vouch-spelling family (dq-kOOB/kTYANNOT cluster); the 22H live-plan arc (see
   Research/README's note: guards pin apply to book-order — compose, don't collide).
6. Loose ends: a frozen-doc annotation on `notes/093` (round-9's closed-world axiom consciously
   revoked for the elide tier); walk-throughs are explain-on-demand.

**Conduct fences (standing; bind any successor):** word-slugs only, explain prior-art inline
(the human is often on his phone); silence ≠ ack — keep an ack-ledger, only what he TYPED
counts; **HARD QUARANTINE on corpus/H2SaLS topics** (never route sizing through them; the
`quarantine-DO-NOT-READ/` dir and `Research/corpora/` stay unread; do not ask why); crosscheck
adjudication under maximum skepticism (convergence = signal; adversarial-only =
suspect-until-checked); adversarial framing = exclusions-not-inclusions, strip authors'
worry-lists from hostile agents' reading; Fable dispatch = ask-first, human dictates or reviews
prompts (goals/desires, never instruction-lists — Opus gets the inverse: full enumeration);
watch the noted Fable firewall-breach tendency (lead with the breach, price containment, offer
the non-breaching cousin); strawman sh liberally in conversation, never as durable design;
never edit README/DESIGN/IMPLEMENTATION/TODO/AGENTS/root-CLAUDE; notes are append-only —
EXCEPT this file; echo the TaskList each round; the method is xfail-first → design →
adversarial-crosscheck → build; tc-shaped judgment calls flag UP, never settle silently.

**On-ramp order for a fresh conductor:** root docs → `spike/CLAUDE.md` → `plans/233` whole
(incl. end-annotation) → `notes/239` → THIS FILE → `23D` → `23F` (both addenda) → `23A`+`23G` +
`spike/e2e/run.sh` → `237` → `238` → `235`. Build reality: `plans/16P` §3 + `16Q` + closes
`20K`/`21W`/`22W` + the `guard23-*` cases.

---

## Yesterday-scale (2026-07-01/02, compressed — the crisis arc itself)

The human's `plans/233` showed the oracle effect/poison contract broken (silence-as-vouch
unsound / silence-as-poison valueless — the frame problem); the ternary reshape + guard-license
were settled in-conversation, stamped into 233, crosschecked by three clean-context agents
(`236a/b/c`, adjudicated `237` — mechanism validated 3-way; vouch-tier de-centered then
superseded by the two-halves doctrine; re-observation re-priced into the placement-spectrum
round), the ceiling mapped (`238`), the closure signed (`239`), the fork resolved + hatches
parked (`235`), the pin-set authored/reviewed/repaired (`23A`/`23B`/`23C`/`23F`/`23G`), and the
spike partially reconciled to the design (`23E`; R1 landed, atomic session in flight).

## Ancient (pre-round-23)

See `Research/README.md`'s per-round map (rounds 1–22) — the spine there is curated and
current through round 23.
