# 27Xf — Adjudication of the round-28 crosscheck (reports 27Xa–27Xe)

AI-authored (Fable conductor, 2026-07-17). Batched adjudication of the five round-28
crosscheck reports against the post-adjudication repair layer at review-point `de22017`
(branch `ai/r27-review-base` — the non-durable cherry-pick constraining the lanes to the
full design record + root-doc fixes, all in-flight builder code excluded). The layer
under review: the `279f` self-adjudication, the in-place spec amendments (`277` §§3/5/6 ·
`275` §6), the two self-proposed-then-ratified rulings (`271:rul-only-oracle-bytes-ship` ·
`276:rul-pipefail-emit-never`), the replacement mechanism (`27A`→`27B`→`plans/27C`), and
the current-truth steering-law compression (`spike/CLAUDE.md` + seven crate files).

Authority: root docs and human-TYPED rulings outrank this; this note records adjudication
*verdicts* — nothing here closes by silence, and repairs are proposals until human-typed.
Ran under standing maximum-skepticism from the pro-position (the conductor is the
churn-guard: bar for crediting is *verified wrong in the artifact*, not *a reviewer said
so*). Lane weighting: 27Xa (Fable, in-lineage, adversarial, steered off `sudo`→chroot/mise
+ hard security exclusion) > 27Xb/27Xc (GPT-5.6-Sol, neutral/adversarial) > 27Xd/27Xe
(DeepSeek, neutral/adversarial). All five lanes landed (no coverage gap this round; the
sol-adversarial silence of the 270-era round did not recur).

## §1 — Verdict in one paragraph

**No kill; block-rebuild is untouched; the primary mechanism is confirmed sound from five
independent angles.** Measure-in-the-site's-denoted-context (`27C`'s default lane) drew a
withdrawal or a clean-clearance from every lane that examined it (27Xa
cleared-measure-in-context-core-move · 27Xb withdrawn-primary-entry-renames-transport ·
27Xc withdraw-measurement-placement-category · 27Xd/27Xe core-sound): nobody killed the
central move, and the completeness gap really is dissolved wherever entry succeeds because
nothing crosses a boundary. What the round *did* find concentrates almost entirely in the
one tier `279f` §3 already fenced and deferred — **`27C`'s cross-context fallback lane
(§4)** — plus a **steering-law staleness that feeds rebuild agents a hard-deleted role
name**, plus cheap spec-text hygiene. The single highest finding is real and verified:
`27C` §4's one *unflagged* fallback bullet (the engine-warranted carried-by row)
re-introduces the exact completeness gap the whole arc was built to close, in the
cardinal-sin direction. It lands in **block-context**, not block-rebuild — matching the
human's standing prediction that critical findings + their repair ride a later arc.

## §2 — Convergence map (strongest signal first)

Cross-lane agreement, my job's most-trustworthy input:

- **is_diverged in steering** — 27Xb + 27Xc, independently, cited. VERIFIED.
- **27C §1 authority predicate-vs-rule contradiction** — 27Xa + 27Xb, independently,
  cited. VERIFIED in-text.
- **the cross-context fallback lane re-opens the completeness gap** — 27Xb (unflagged
  structural row, sharpest), 27Xc (sudo guest non-equivalence), 27Xd/27Xe (tolerance-vouch
  completeness-shape) circle the same joint from three distinct angles. The 27Xb form is
  the one that bites unflagged; VERIFIED.
- **the layer's own stale-text discipline (applied to `275`) skipped for a sibling spec**
  — 27Xa (`277`), 27Xd (`275` §6 residue). VERIFIED for `277`.

One place a lane's clean bill was wrong-by-omission: **27Xd's "21/21 CLAUDE.md bullets
FAITHFUL"** checked each bullet against its *cited* source; it did not check the role-menu
bullet against the `24C` deletion ruling, so it missed the is_diverged divergence two other
lanes caught. "Compression is faithful" holds for what it sampled, is not a clean bill.

## §3 — Credited findings (verified; ranked; repairs are proposals)

### Tier 1 — the fallback lane re-opens the completeness gap (block-context; NOT block-rebuild)

> **RESOLVED 2026-07-17 (human-opted): `27C:mech-pure-predicate-carry`.** The finding
> below stands as diagnosed; the fix is neither flag-gating (moving-under-the-flag is a
> failure-mode) nor abandonment. The old unflagged engine-warranted carried-by row is
> RETIRED (it rested on tool-semantics the engine may not hold). Substrate-axis facts now
> travel unflagged iff (A) their marked backing kinds carry the owner's `invariant:<axis>`
> line AND (B) the engine proves the verdict body read-set-closed (everything influencing
> the verdict rc traces to argv or a marked read; no unmarked external input) — closing the
> open-world residue by structure rather than accepting it under the flag. Referent-agnostic
> (marks + sh-taint, never tool-semantics; it lifts the hermetic argument-driven predicate
> shape authors already write); substrate axes only (user excluded — access-flip); netns
> needs the namespaced-`net.*` model. The conservative closure pass is the spike's to build
> and prove in practice. Spec: `27C` §0/§4(a)/§9/§10; steering `spike/CLAUDE.md`
> pure-predicate-carry; need `ANALYZER-NEEDS` an-read-set-closure.

**cr-structural-carried-by-transports-a-measurement** (27Xb finding-structural-transport-
repeats-completeness-gap, CRITICAL/0.98) — **CREDITED, verified, highest.** `27C` §4's
first fallback bullet — the *engine-warranted carried-by row*, explicitly **unflagged** —
licenses an *unshifted* measurement to answer a wrapped site on the strength of
substrate-store invariance ("kernel state is not filesystem state"). But the engine
warrants only that the *store* is dimension-invariant; it cannot warrant that the measuring
*body* read only that store — and the design's own position (`279f` §3, credited;
`spike/CLAUDE.md` value-predictions "backing carries NO completeness burden") is that
backing is *not* complete. So a verdict function that honestly marks one kernel read while
also reading a filesystem path (27Xb's `policyctl` counterexample: reads `/etc/policy/$1`
AND `sysctl -n`, marks only the kernel cell, run under `chroot /target`) gets its ambient
verdict transported across the fs-view boundary and wrongly elided — silent under-execution,
unflagged, no false line, reliable oracle. This is the cardinal-sin shape the razor arc
exists to kill, re-entering through the one lane `27C` left unflagged. It is precisely the
`279f` §3 gap, which `27C` fixed for the *authored-invariance* lane (§4 bullet (b),
flag-gated) but left open for the *structural* lane on the reasoning that store-invariance
is engine-warrantable — conflating store-invariance with answer-invariance (the exact
conflation `27A` §1 named).

*Dimension scope (human-corrected 2026-07-17; sharpens the finding, does not dissolve it):*
the unflagged structural carried-by row is a **substrate-dimension mechanism only** —
`277` §2's generator row scopes it to "substrate-borne axes," and the design serves the
`user`/`sudo` dimension through the *other*, flag-gated bullet (§4(b), authored-invariance
line × flag). A user-shift changes *authority over* stores, not the stores themselves, so
there is no substrate partition for the engine to warrant and the unflagged row never fires
for it. The hazard is therefore confined to **fs-view (chroot) and netns**. Two distinct
unsoundnesses live there: (B) the *answer-completeness* hole above (a body straddling the
substrate partition — reads both the invariant store and a varying store, marks only the
invariant one), which bites both substrate dimensions; and, for **netns specifically**, an
(A) *store-invariance* hole the fs-view case does not have — `net.*` sysctls are
per-netns-namespaced, so the engine warrant "kernel state is netns-invariant" is *false*
for network sysctls even for a pure-substrate-read body. fs-view is the clean case (one
hole, B); netns is doubly delicate (A and B).

*Repair (proposal), least-to-most conservative:* (i) fire the unflagged structural row
ONLY when the effect-analysis can **prove the body's read-set closed** to the invariant
store (a lone `sysctl -n "$1"` is closable and sound; a straddling body is not, and falls
to guard/run) — sound, unflagged, no new author vocabulary, degrades safely, and it is the
`279f` §3 disposition-(c) option; (ii) restrict structural rows to
cell-identity-for-keying, never verdict/value transport (280-tier consensus; keying is
license-free per `272` §4); (iii) flag-gate the whole row (simplest; loses the unflagged
attention win on genuinely-safe pure-substrate-read bodies); or take `27C`'s recorded
`honest-walls-for-worlds` v1-defer. netns additionally needs the engine's substrate model
to know which kernel state is namespaced, regardless of which option is chosen. Verified
against `27C` §0/§4, `277` §2, `279f` §3, and the value-predictions law. **Does not touch
block-rebuild** (which builds coordinates/chokepoints/dialect/seams — no transport license
minted there); lands squarely in block-context implementation-planning.

**cr-27C-0-says-nothing-travels-then-travels** (27Xb, sub-point) — **CREDITED, verified.**
`27C` §0 §2's fallback bullet reads "(a) an engine-warranted carried-by row (structural,
unflagged) … (b) … × `--risk-faultless-skips`. Silence walls. **Absent the flag, nothing
travels, ever.**" (a) is a literal unflagged travel inside the same bullet that closes with
"absent the flag, nothing travels, ever." Same joint as above; the contradiction is the
symptom. Repair rides the Tier-1 decision.

### Tier 2 — real spec/steering defects, cheap, some already self-healing in-build

**cr-is-diverged-lives-in-steering-and-ANALYZER-NEEDS** (27Xb finding-steering-restores-
deleted-diverged-role + 27Xc medium-deleted-role-remains-steering) — **CREDITED, verified,
scope widened.** `24C:734` is a HUMAN-TYPED hard-deletion ("`is_diverged` is REMOVED from
the dialect — hard, not soft"; removal inventory strikes the reserved suffix, the sense-flip
glue, the `VerdictSense` parameter); `278:150` lists only `cmd__is_converged()` as the
canonical verdict function. Yet `spike/CLAUDE.md:105` ("authoring `cmd__is_converged()` /
`cmd__is_diverged()` IS the vouching act") and `:356` (role menu "`__is_converged()`/
`__is_diverged()`") still name the dead role — a *permanent, unversionable compat surface*
per `271` — and `ANALYZER-NEEDS.md:336` (`an-verdict-function`) still lists it too (broader
than either lane caught). 27Xb additionally reports live code carrying it (reserved.rs,
verdict.rs dual-`VerdictSense`, a passing obsolete-role test) — HEAD-code, so block-rebuild's
respell is the correct home for the code strike, but the steering text feeds rebuild agents
NOW. *Repair:* strike `is_diverged` from both steering occurrences and `ANALYZER-NEEDS:336`;
add a negative acceptance pin (the deleted permanent name is neither reserved nor
recognized). Highest-priority Tier-2: steering is what rebuild agents read.

**cr-27C-1-authority-predicate-contradicts-rule** (27Xa finding-held-authority-cell-
contradiction + 27Xb finding-nonroot-authority-cells-do-not-partition) — **CREDITED,
verified.** `27C` §1 states the authority test twice, incompatibly: the *predicate* "the
only implementable predicate is *can the connection do it with zero new credentials*" vs
the *rule* "a non-root connection performs none of them." A held-authority non-root
connection (the passwordless-delegation cell `27B` §2 called "the norm") satisfies the
predicate but is classed by the rule as performing nothing, and routed into an
"acquisition" cell whose sketched mechanism is a no-op for it. `spike/CLAUDE.md:203`
compresses to the *predicate* reading; the `27C` four-cells text carries the *rule* reading
— steering and centerpiece disagree, and an implementing agent cannot build the entry gate
without choosing. *Repair:* one human sentence choosing a side + correcting whichever text
loses. (Raised strictly as spec-consistency/coverage; the privilege dimensions are
excluded-lane territory and not adjudicated here.)

**cr-set-lifting-vacuous-at-empty** (27Xa finding-vacuous-set-lifting-hole) — **CREDITED,
verified via build corroboration.** `279f:fix-set-lifting` (now `277` §5, compressed into
`spike/CLAUDE.md` set-lifting-universal-meet) states universal quantification over
backing-SETS without the non-emptiness side-invariant; universal-over-∅ is vacuously true,
so the law as written licenses sparing-every-wall / transport-everywhere for an empty
backing-set. An amendment whose entire purpose (`279b-fd5`) was closing quantifier holes
missed the third one and was acked + compressed in that state. Corroboration the finding is
real: the in-flight build branch subsequently had to mint exactly the missing precondition
(`git log` @ `1c086b8`: "minting-line precondition + same-kind limitation named",
post-review-point). *Repair:* state the two side-invariants (a fact's backing-set is
non-empty by construction — the minting line's own coordinate is always a member; ⊤/unknown
is never *encoded* as ∅) in `277` §5 + steering, matching what the build already did; DST
pins ∅-unrepresentable / ⊤-never-∅. Cheap; partly self-healed in-build already.

**cr-277-stale-transport-rows** (27Xa finding-stale-transport-rows-in-277) — **CREDITED,
verified.** The layer applied exemplary supersession discipline to `275` §6 (banner +
per-step comments) but not to `277`, which `Research/README` designates "THE spec" for the
entity algebra and which entity-algebra-rebuild consumes directly. Verified: `277:124`
consumer-map row "same | transport … the probe-outside license" (under-qualified — no 27C
flag-gate caveat for cross-context consumption); `277:547` "Riding the human's adversarial
pass, by design: the `275` ratifications (… transport chain)"; `277:566` status row "`275`
ratifications … | riding the human's adversarial pass" — all describing transport as a live
pending-ratification consumer, when `279f` refused it (07-13) and `27C` re-answered (07-16).
*Repair:* three `275`-pattern supersession annotations. NB the §2 relation row is not
false as *relation algebra* (same cells → a fact does transport); it is under-qualified as
to the cross-context-consumption flag gate — annotate, don't rewrite.

**cr-nested-wrapper-composition-rider-dropped** (27Xb finding-nested-wrapper-composition-
rider-was-dropped) — **CREDITED, verified against `279f`.** `279f` §2 (row 279a-A7) + §5
credited the nested-wrapper `lend`/ρ composition gap and *dispatched an explicit rule*
(pointwise lend composition, ⊤ propagates) to block-context planning. `27C` §3 says only
"chains compose recursively" and relies on one segment per `(host, context)`; neither `27C`
nor steering carries the pointwise-fold / ⊤-propagation / canonical-context-key algebra.
`27C` IS the block-context spec, so the dispatched rider has no home. *Repair:* add the
pointwise fold to `27C` §3 + the block-context brief (order, identity element, mapped/full
interaction, ⊤ propagation, context-key construction); pin nested permutations.

### Tier 3 — real but scoped / doc-sync / rationale (mostly human-owned root-doc queue)

**cr-flag-contract-widened-past-one-naked-trust** (27Xc high-risk-flag-contract-widens +
27Xa) — **CREDITED as doc-sync.** `27C` §4 extends `--risk-faultless-skips` to a *second*
transaction (fallback-lane foreign-measurement consumption) beyond USER_STORY's/KNOBS'
advertised "exactly one place / one naked trust / everywhere else you trust measurements."
Internally intelligible (`271:rul-flag-is-razor-residue` pre-authorized the flag absorbing
future unsayable residue by outcome-class — 27Xc concedes this); the gap is purely that the
*admin-facing* contract wasn't updated. `279f`'s root-doc-queue item mis-scoped it ("caveat
*if transport ships unflagged*" — it shipped *flagged* and still falsifies "exactly one
place"). *Repair:* human-owned root-doc edit; re-scope the queue item. Priority-3 honesty
territory the corpus is elsewhere scrupulous about.

**cr-entry-self-effects-carve-is-AI-asserted** (27Xc blocker-context-entry-mutates-plan) —
**CREDITED, re-framed (NOT a blocker).** `27C` §3 carves "entry self-effects modeled,
elide-alongside" (sudo: an auth-log line, a timestamp refresh) against the categorical
probe-never-mutates weld (`kFAIL-withhold`; DESIGN §194-226/§290-293). 27Xc overstates it as
a categorical contradiction; it is a *carve to a welded invariant currently asserted at
AI-tier*, and it is `sudo`-specific — `chroot`/`mise exec` entry writes nothing durable, so
the dimensions actually in play don't trigger it. *Repair:* explicit human ruling on
whether context-entry self-effects breach the probe-mutation weld (they are the ONE probe
write the design contemplates); scope it to the wrappers that log. Genuine open question,
not a defect.

**cr-sudo-entry-not-guest-insensitive** (27Xc critical-substituted-guest-changes-context) —
**CREDITED, scoped to sudo/policy-wrappers, STRAWMAN-tier.** sudo policy can match on the
guest command+args (per-command CWD/chroot/SELinux/AppArmor), so `sudo -n sh -c checker`
may inhabit a *different* policy-selected context than `sudo original` — measure-in-context
isn't context-equivalent for sudo specifically. Real, but: the entry form is STRAWMAN
(`27C` §10); the dimensions in play here (`chroot` fixes context by its argument, `mise
exec`/`env` by ρ) ARE guest-insensitive. *Repair:* the entry-form *contract* needs an
explicit guest-insensitivity requirement; sudo (and policy-matching wrappers) must satisfy
it or not qualify as a default entry form. Records a contract requirement for whoever builds
entry forms; not a design-frame break.

**cr-tolerance-vouch-rationale-unstated** (27Xd finding-1 + 27Xe F2; both *withdrew* the
"contradiction" framing) — **CREDITED as a rationale gap only.** The tolerance vouch is a
completeness-shaped negative-universal shipping on the default dial; both lanes, on
reflection, concede it is NOT the refused transport gap because it is *empirically
falsifiable* (two-user CI, tracer read-set diffing) where backing-completeness is not (27Xe
W1 explicitly withdraws; 27Xd "not a contradiction — the razor holds"). The design already
prices the risk (`27C` §7 hole-bad-oracle-blast). *Repair:* state the falsifiability
distinction in `27C` §2/§7 (why one completeness-shaped claim is acceptable at default and
the other was refused). Minor; do not let it drive churn.

## §4 — Dismissed / downgraded (churn-guard rejections, with reasons)

- **27Xe F3 / F4 / F6** (the adjudication is in-lineage / `27C` never externally reviewed /
  "four lanes" inflated) — **DOWNGRADED to near-moot.** These attack `279f`'s independence;
  they are substantially answered by the existence of THIS round — `27C` is now
  externally reviewed by three foreign-lane passes (27Xb/27Xc/27Xe themselves), which found
  real things. 27Xe F4 "27C never externally reviewed" is falsified by 27Xe reviewing 27C.
  The human already knowingly dissolved task-14 (the "clean-room" was clarified never
  required; `271:rul-only-oracle-bytes-ship` ratified by typed ack). Task weighted these
  LOW. Residue kept: one cheap process-hygiene rec (below).
- **27Xa finding-independence-collapsed-twice** as a *defect* — **DOWNGRADED to process
  note.** Both promised fresh-session re-derivations resolved in-context; but nobody claims
  the resulting law is unsound (27Xa itself: +SURE the only-oracle-bytes law is the safe
  direction), and the human ruled knowingly. Kept as hygiene: *a future
  "fresh-session/clean-room/triple-check" commitment should get a slugged ledger entry whose
  closure names who ran it and from what context.* No design substance.
- **27Xe F1 / F7 · 27Xd finding-2 / finding-3 · 27Xa finding-cascade-rescue-product-
  conflation** (27B's body still says "trilemma dissolved" / "transport dies" / attention-
  vs-execution; 275 §6 stale text ahead of corrections) — **DOWNGRADED.** `27A`/`27B` are
  SUPERSEDED historical notes carrying load-bearing banners; newest-wins governs and the
  corpus runs annotate-don't-rewrite, so a superseded note's overclaiming body is not a
  defect (the banner is the correction). The manufactured-adjacent class of this round:
  straining at a bannered historical doc. No action beyond the banners already present. (The
  ONE live-text instance — `277`'s rows — is credited above precisely because `277` is NOT
  superseded; it is THE spec.)
- **27Xb finding-residue-containment-weighted-before-designed** (conditional tails graded as
  containment while marked STRAWMAN) — **NOTED, low.** Fair observation that `27C` §5/§7 use
  a deferred mechanism to grade the non-root residue "contained"; but 27Xb found no
  counterexample that kills it and the mechanic is bounded by welded attention law (27Xe W2
  withdrew the determinism attack for that reason). Rides the placement-spectrum round
  already parked in `271`. Kept as a wording caution: separate ruled floor (guard/run) from
  the conditional-tail value projection.
- **Amendment-repairs re-confirmed** (fix-spare-top-backing, dialect-properties,
  pipefail-emit-never, the five `279f` §7 dismissals incl. errexit `v=$(false)`) — every
  lane that re-checked them confirmed they hold (27Xa §2 cleared-*, 27Xc withdraw-*, 27Xd
  §6). No action. Trivial residue: the "lint-clean" qualifier that `276` carries is absent
  from `279f` §6's ask-summary and KNOBS' weld prose (27Xa) — one-word doc-sync, optional.

## §5 — Disposition

1. **Block-rebuild is untouched.** Confirmed from the same four independent angles `279f`
   §8 claimed and re-verified here: nothing credited mints or depends on a transport
   license; the build builds coordinates, chokepoints, dialect sets, seams, fences. The
   Tier-1 finding lives in block-context. Proceed.
2. **Tier-1 RESOLVED 2026-07-17 (human-opted): `27C:mech-pure-predicate-carry`** — the
   unflagged engine-warranted carried-by row is retired; substrate-axis facts carry
   unflagged only under authored `invariant:<axis>` (A) + engine-verified read-set closure
   (B), closing the completeness residue by structure rather than under the flag. Landed:
   `27C` §0/§4(a)/§9/§10, `spike/CLAUDE.md` pure-predicate-carry, `ANALYZER-NEEDS`
   an-read-set-closure. Build-and-prove the conservative closure pass is the spike's
   load-bearing task. Does not gate block-rebuild.
3. **Cheap text repairs, do soon** (feed rebuild agents / THE spec): strike `is_diverged`
   from `spike/CLAUDE.md` ×2 + `ANALYZER-NEEDS:336` + negative pin (Tier-2, highest —
   *the human completed the is_diverged docs cleanup 2026-07-17*); the `277` transport-row
   annotations (DONE — §8/§9 markers); the `27C` §1 authority sentence; the `277` §5 +
   steering non-emptiness invariant; the nested-wrapper composition rule into `27C` §3.
4. **Root-doc queue (human):** the flag-contract widening (re-scope the mis-scoped `279f`
   queue item); the entry-self-effects carve wants an explicit human ruling.
5. **Housekeeping at fold-in:** rename the deepseek-adversarial lane's stray
   `27Xd`-prefixed working-notes file (collides with the neutral report's prefix); the
   deepseek-neutral self-commit used `(AI review)`, not an enumerated `.gitlabels` tag.

## §6 — The code-now weighing (counterbalance)

Five kill-mandated lanes, zero kills, on a mechanism produced in one day and never before
externally reviewed. The primary lane (measure-in-context) survived every angle. The
findings cluster exactly where `279f` §3 already pointed — the cross-context fallback lane —
which is both confirmation the refuse-and-defer instinct was right AND the warning that
`27C` §4 quietly walked one unflagged bullet back into the refused gap. That bullet is the
whole payload of this round: cheap to fix, lands in block-context, changes nothing about the
build now in flight. Everything else is spec-text hygiene the corpus already knows how to do
(it did it for `275`; it owes the same to `277` and the steering law). Dispatch continues;
schedule the Tier-1 fold into block-context planning as a named strike-condition, not a
footnote.
