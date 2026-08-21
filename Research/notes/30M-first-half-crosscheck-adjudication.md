# 30M — r30 first-half crosscheck: conductor adjudication

> Tier: conductor adjudication (Fable, session `r30-fable-adversarial-review-conductor-first-half`,
> 2026-08-21) of the five-lane review of the landed r30 range `68709783..3011daae`. Kit:
> quarantine `30Ma`; raw reports `30Mb` (Fable adversarial) · `30Mc` (Sol neutral) · `30Md`
> (Sol adversarial) · `30Me` (DeepSeek neutral) · `30Mf` (DeepSeek adversarial). Adjudicated
> under maximum skepticism: every finding credited below was either verified by the conductor's
> own read, re-run mechanically, or is credited-on-report with that status stated. Grades:
> **+SURE** / **~SUSPECT** / **-GUESS** are mine unless attributed.
>
> Companion: **`30Mg`** is the just-fix-it builder remit (obviously-wrong, zero
> design-question items only). Items there get one line here (§6) and no more ink — they
> are expected to be fixed within days and are recorded durably in `30Mg` + git.

## §0 — Verdict in one screen

**No live wrongly-minted elision exists on the production CLI path at the reviewed tip.**
All five lanes, from three lineages, converged on that; the two lanes that went deepest
(Fable, Sol-neutral) each independently rated the effective-world settlement kernel — after
its `30Kb` repairs — the best-defended surface in the tree. The `30Kb` criticals are
genuinely repaired in code (verified by Fable trace AND Sol-neutral's independent read; I
spot-confirmed the aggregate universal-crossing seats).

The exposure the review actually surfaced sits in three places, none of them the seat the
arc worried about most:

1. **The static-loading edge** (`30I` steps 1–5a) carries the round's only demonstrated
   priority-1-class route: CLI pre-source acquisition promotes transitive dependencies to
   synthetic *roots*, replaying their load programs after the authored program finished —
   engine-created authority that can restore a definition the author `unset -f`'d
   (§1). Two further account-integrity defects (discarded dot-locals; the guarded-source
   value conjunct dropped against TYPED text) make the one load account — which the
   just-landed step-5b bundle projection now CONSUMES — unfaithful to sh in three distinct
   ways. This cluster is urgent precisely because of that consumption.
2. **The certifier-trip terminal cleanup** (`302:rul-certifier-trip-guard-only`) runs in
   exactly two drivers. The public `plan::build_plan` entry and all three instrument
   drivers (hostsim — the DST backbone — sweep, coverage) construct a throwaway
   `CertifierTrip`, discard it, and project: cross-window trips retain elisions there,
   demonstrated by a committed red test (§2). No production apply artifact ships from
   those paths today; the law violation, and the instrument blindness, are real.
3. **A governance layer**: conductor/builder deviations sitting unratified on welded or
   TYPED law — the wall-formation narration still flag-gated against a twice-REJECTED
   review verdict and the `kWARN` weld (§3); Spine records whose fields state falsehoods
   at the exact boundary `30L`/`30I`-7/8 are directed to consume (§4); and a
   representational conflict between two TYPED rulings (grade-at-mint vs
   authored-before-contact) that the landed one-grade-per-Spine type cannot express (§4).

Convergence map (the strongest signal in a crosscheck): the trip-cleanup gap was found
independently by three lanes (Fable abstractly; Sol-adversarial and DeepSeek-adversarial
concretely, different lineages); the wall-narration deviation by two lineages (Fable,
DeepSeek-neutral) atop Sol's original two rejections in `30Kb`; the loading-edge cluster
is Sol-neutral's alone but carries committed, mechanically-verified demonstrations; the
settlement kernel's health is unanimous.

## §1 — CRITICAL, loading edge: pre-source replay mints engine-created authority

**`30M:adj-pre-source-replay-is-the-rounds-worst-defect`** — crediting
`30Mc:finding-transitive-pre-source-replays-as-root` in full, at its filed severity.

Mechanism (+SURE, Sol-demonstrated, conductor-verified mechanically — §7):
`cli/src/main.rs:372–403` recursively appends every marked file a named pre-source
sources; `read_book_sourced` then takes the ambient boundary AFTER expansion, so acquired
dependencies become `SourceRole::NamedLoad` roots; `run_ambient_prefix` executes every
root's complete load program in vector order. For `--pre-source entry.dorc.sh` where
`entry.dorc.sh` sources `verdict.dorc.sh`, the modeled program is `entry` (with its nested
load) *followed by a synthetic second run of `verdict`* — an act the author never wrote,
violating `30I:rul-pre-source-is-dot-prelude` [TYPED] and
`rul-static-loading-is-the-whole-model` [TYPED].

Why it is priority-1-class and not a precision loss: Sol-neutral's committed world (an
entrypoint that sources a dependency then `unset -f`s one of its definitions) ends, under
sh, with the verdict function ABSENT — and under the landed model, *restored* by the
synthetic root. A restored `is_converged` is a live vouch source: with an otherwise-valid
predict row and a converged probe answer, Dorc guards or replaces a book command that,
under the modeled shell, no live judgment answers for. That is engine-created authority
suppressing admin bytes — the cardinal direction — reachable from truthful oracle files
and an ordinary CLI invocation. The corpus never caught it because every committed
fixture's dependencies bind idempotently (`30Mc:test-idempotent-dependencies-hide-replay`,
a genuinely good observation about the whole fixture population).

Two aggravators, adjudicated:

- **The bundle projection now consumes the lie.** Since the review tip, the sibling lane
  landed `30I` step 5b on `ai/r30-conduct` (`e4076cb3` "Project every static load
  occurrence into bundles") — and the load-occurrence account it keys bundles from
  contains the synthetic `LoadSourcer::Invocation` roots
  (`30Mc:consequence-bundle-roots-are-already-corrupted`). I verified the sibling's diff
  touches `main.rs`/`snapshot.rs` only additively for bundling and does NOT touch
  `funcenv.rs` — the defect certainly survives at that tip (+SURE). The repair must land
  UNDER the fresh bundle work, before any golden promotion treats bundle-root population
  as expected shape.
- **It is entangled with a builder-recorded deviation Fable independently flagged as the
  lane's sharpest unadjudicated item** (`30Mb` §9: `30Ib:dev-ambient-include-guards-are-
  not-evaluated`): acquisition expands dependencies WITHOUT evaluating include-guard
  control flow, and the recorded fix ("run ambient programs at the `Entry` transfer")
  carries an open licensure question — what an unresolvable ambient load does — with
  `30Ib` §5.2's variable-rooted custody work blocked behind it. The replay repair and this
  deviation are the same seat.

Adjudication: **repair-now, with one narrow design decision attached.** The mechanical
half (`30Mc:required-root-occurrence-identity`: acquisition retains the explicit ordered
pre-source roots; `push_ambient` receives only invocation roots; dependencies are reached
at their authored `.` positions) is design-clean — it restores the TYPED rule as written.
The attached decision the human must give direction on, because it decides what the
Entry-transfer evaluation does at its edges: **`30M:ask-unresolvable-ambient-load-posture`**
— when a pre-source's own load program hits an unresolvable act (unknown operand, ⊤ cwd),
does the whole prelude floor (funcenv ⊤ from that point — maximally conservative,
matches today's book-side behavior), or does it suspend only the affected subtree? My
recommendation: whole-prelude floor at v0 — it is the sh-parity-shaped answer
(`rul-unsure-falls-toward-sh-parity`; sh would have run the failing `.` and its
consequences are unknowable from there on), it is strictly conservative, and nothing
downstream depends on prelude precision yet. This item is therefore in `30Mg` with the
floor posture stated as the default, human veto invited.

## §2 — HIGH, kernel edge: the trip cleanup is absent from every non-CLI producer

**`30M:adj-trip-cleanup-absent-from-four-producers`** — crediting
`30Md:fnd-discarded-trip-retains-elisions` (Sol, High) and `30Mf` Finding 1 (DeepSeek,
medium) as one convergent finding, with Fable's §5 `dec-trip-cleanup-is-still-must-remember`
as the design-shape half. Three lineages, independent.

The mechanical fact (+SURE, three independent reads agree, red test committed):
`plan::build_plan` (`lib.rs:3729–3733`), `hostsim` (`lib.rs:1511–1515`), `coverage`
(`lib.rs:592–596`), and `sweep` (`drive.rs:213–217`) each pass a fresh temporary
`CertifierTrip` into `build_plan_walled` and project without ever calling
`demote_on_trip`; `analysis::effect::classify`'s convenience path discards a further
latch upstream. This directly violates `plan/CLAUDE.md
certifier-trip-cleanup-runs-in-every-driver` ("EVERY plan-producing driver") and the
TYPED substance behind it (`302` §3: one boolean per analysis spine; terminal demotion
before plan emission). The primary CLI and `WhyWorld` are correct (both lanes verified —
the law's exemplar drivers are fine; every *other* producer is not).

Severity calibration, adjudicated between the two filings: DeepSeek's bounding is the
better-calibrated half — none of the four paths emits a production apply artifact today,
and same-window trips are floored mid-pipeline regardless (`302` §3's consumer floors,
which both lanes verified fire in place). Sol's escalation is the more important half —
the retained-elision case its red test demonstrates is the CROSS-WINDOW trip (a
different solve window's inconsistency, which only the terminal cleanup evicts), and the
paths that lack it include the DST harness and the sweep/coverage instruments, so **on
any tripped run the project's own regression instruments silently disagree with
production dispositions** — a green instrument is not evidence against this class
(`30Md`'s "poisons the gates' witnesses", which I endorse as the finding's real cost).
DeepSeek's sharpest contribution: the doc-comment on `demote_on_trip` itself
(`certifier_trip.rs:96–99`) claims the must-remember surface "the reification dissolves"
— while three drivers had already forgotten it. The reification moved the *record*, not
the *act*; the claim is false as written.

Adjudication: **repair-now (remit) + one design recommendation.** The remit adds the
missing calls and a fence (a test enumerating plan-producing paths, or a
projection-seat assertion) — no design question; the law already rules the behavior.
The design recommendation, for the human's stack rather than this remit:
**`30M:rec-dissolve-trip-must-remember-structurally`** — make plan-projection demand a
trip-disposition witness by type (Fable's shape: the way projection demands
`PlanAuthority`), so the surface actually dissolves. That belongs with the Spine/plan
boundary close (§4), not in a quick-fix lane.

## §3 — Design-sitting: the wall-narration deviation sits unratified on welded law

**`30M:adj-wall-narration-needs-a-typed-ruling`** — crediting `30Mb` §2
(`fnd-wall-narration-deviation-contradicts-welded-law`) and `30Me` Finding 1
(independent lineages; both +SURE on the state), atop the two standing `30Kb` REJECTs.

State (+SURE): `WallFormation` narratives mint only under `WallPolicy::RiskAccepted`,
and only for leaf decisions (`settle.rs:458,512–519` — the code comment self-discloses
the deviation). Honest mode — the default — and every non-leaf wall (command-substitution
internals, group-level redirections, unmodeled constructs) form decision-bearing walls
with no narrative record. `30K` §7's required-accounts list carries no policy qualifier;
Sol's review REJECTED the deviation twice on grounds rooted in human-welded law
(`KNOBS:kWARN` kWARN-rich — "the detectable moment is during construction";
`AID-NEEDS:law-collapse-mints-narrative` — the mint is demanded at the narrowing, not at
the consumer); and the downgrade from "product architecture blocker" to "non-blocking
residue" appears in no [TYPED]/[ACKED] record anywhere in `300`/`307`/`30Ka`. Under
silence-is-not-ack this is an open conductor-tier call sitting directly on a welded knob
— which is precisely the "locally-defensible default with design-weight" class this
review was commissioned to surface.

Adjudication: I decline to endorse either pole; the knob is welded HUMAN territory. What
I can say: the deviation's engineering argument (three `[unnarrated:]` lines in deep why
transcripts; golden churn) is exactly the cost the kWARN weld says to pay, and golden
churn is separately ruled a non-blocker; the counter-consideration is that the record is
consumed by nothing (`289:seam-narrative-render-unconsumed`) so the cost buys no visible
account today. **The ask is a typed one-line ruling**: either (a) mint on the settled
round in both policies, non-leaf walls included, per `30K` §7 as written — with the
`[unnarrated:]` churn accepted; or (b) ratify the defer-until-consumer stance as a scoped
exception to `kWARN`/`law-collapse-mints-narrative`, recorded where those laws live.

One rider either way (+SURE, cheap, Fable-caught): `30L:req-wall-narrative-gains-region-
operand` currently *implies* un-gating the mint but *spells* only the non-`LeafId`
operand; a `30L` builder implementing it literally leaves honest-mode minting nothing.
The `30L` stage-4 brief needs the one sentence that resolves per the ruling above.

## §4 — Design-sitting: the Spine account is not yet a safe substrate, and says so in false fields

**`30M:adj-spine-schema-truthfulness-before-consumers`** — crediting `30Mc` findings
3–5 (all +SURE mechanical reads; none corrupts today's plan — every defective record is
new-arm/debug-tier or unread) and Fable's §5
`dec-spine-reification-under-delivers-its-own-boundary` as one cluster.

The inventory, adjudicated into two tiers:

- **Flat falsehoods, design-free, fix now (remit, §6):**
  `SpineSiteClassification.invalidator` is documented "gens into reach as an invalidator"
  but written from `kills` alone — false for every ordinary establish and every opaque
  leaf; the `InlineCall` record maps its ordered member account to an empty `cells`;
  `SpineInvocation.mode` is hard-coded `"whylog-replay"` from a function unreachable on
  the actual replay branch — the durable field describes neither producing invocation.
  Each contradicts its own documented meaning; each is exactly what `30L`'s route work or
  a future durable lift would naively consume.
- **Representational conflicts needing a human direction (design list):**
  1. **`30M:ask-spine-grade-boundary`** (`30Mc:finding-spine-grade-is-object-global`):
     `Spine` carries ONE influence grade object-globally; load decisions and invocation
     framing are recorded after intake and therefore stamp `host-influenced` — but
     `30I:rul-load-decisions-are-authored-before-contact` [TYPED] and `309` §2's
     stamp-at-mint [ACKED] jointly require them `authored-before-contact`. The two typed
     directions are individually coherent and jointly unrepresentable in the landed type.
     Options (Sol's framing, which I endorse): mint authored records onto the Spine
     before intake and carry it through settlement, or permit record-local grades whose
     authored-provenance constructors demand the loader's typed witness. Continuing to
     add late recorders silently chooses wrongly. This should be ruled before any
     marking-frontier, influence-debug, or bundle-provenance work consumes grades.
     **CLOSED [HUMAN-RULED 2026-08-21]:** `306b` §10 is the conductor-facing
     authority and its quarantined mirror is `306a` §11. Influence is carried
     privately and non-optionally by stable semantic objects; their own mints join
     all influencing inputs. Spine preserves already-established record influence
     and never stamps an object-global grade. Views continue the same carriage into
     projection decisions and outputs; unimplemented seams are explicit untracked/
     maximally-influenced, never implicitly authored. Implementation remains owed at
     `30M:rec-own-the-309-boundary-close` before `30I` steps 7/8.
  2. **`30M:ask-certification-row-shape`** (`30Mc:finding-certification-window-replaces-
     passes`): the sole production writer emits one `whole-window` certification row
     derived from the run-wide latch, against a documented closed per-pass vocabulary and
     the `30E` census's "per-pass consistency + the latch". Conforming (one row per
     certification event + the latch as its own summary) is probably just census-fidelity
     — but pass identity for repeated settlement rounds has representation judgment in
     it, so it gets a direction line, not a remit line.
  3. **`30M:rec-own-the-309-boundary-close`** (Fable §5): the `309` ack pre-dates three
     disclosed as-built deviations (render still computes decisions `30F` §4.4; five
     render-time decisions outside the plane `30E` §3; the `DecidePlane` generic seam).
     None is wrong per se; together they mean the boundary `30I` steps 7/8 will freeze is
     not yet the boundary the ack described, and no lane owns closing the gap. Assign it
     (a small lane or a rider on step 7) before artifact forms land. Sol-neutral's
     assessment sentence is the right summary: the Spine is real but "unfinished schema,
     not ready substrate," and the census proves *types project*, not that *populations
     mean what their fields say* — a population-and-meaning audit belongs to that close.

## §5 — Design-sitting: the guarded-source value conjunct, and the reviewers' one real disagreement

> **SUPERSEDED 2026-08-21:** the binary ratify-dissolution-or-build framing below
> conflated two consumers. `30I:rul-load-semantics-stay-full-fidelity` requires
> the supported sh load model to preserve the literal comparison and branch;
> `30I:rul-guarded-source-speech-is-lossy` separately requires the authorship
> projection to retain only direct-constant origin, guarded-source, and helper
> co-resolution. The exact value is load-bearing for behavior and deliberately
> discarded for the speech act. This historical disagreement remains useful as
> the evidence that exposed the missing separation.

**`30M:adj-sentinel-literal-needs-ratify-or-build`** — crediting
`30Md:fnd-sentinel-literal-never-participates` (Sol, medium, red test committed) as the
finding, WITH Fable's contrary "inert" verdict (`30Mb` §9 did-not-hold) as a genuine
half-truth. This is the review's one direct cross-lane disagreement, and both halves
survive scrutiny:

- Fable is right about **custody and licenses**: the value question cannot move WHOSE
  judgment executes — `sole_populator`'s uniqueness census over the whole authored world
  plus the composition seat's suspension make a wrong-unit value inert for authority (its
  §9 audit is thorough, specimen-grounded, and I credit it fully).
- Sol is right about **the account and the TYPED text**: `30I` §3.4 [TYPED] requires "the
  guard-tested value that selected that route" to `Must`-originate in the target closure;
  the implementation deliberately never compares values (`30Ib` §10 records it as
  "dissolved"), so a version-mismatch world (`v1` assigned, `v2` guarded — sh takes the
  source arm and runs the dependency twice) records `LoadRoute::Reused`. The one load
  account states the wrong branch. That is harmless today and becomes concrete the moment
  emission reproduces branch decisions: a flattened artifact omitting a re-source sh
  performs is a behavior divergence in the emitted plan — and step 5b just started
  keying bundles from these occurrences.

Adjudication: a **builder default contradicting TYPED text, recorded as "dissolved"** —
exactly the unruled-decision class, at medium design-weight. The human owes one of:
(a) ratify the dissolution — amend `30I` §3.4 to name-census sufficiency, accepting the
account's branch infidelity as a documented limitation (cheapest; defensible for v0 given
the custody inertness); or (b) direct the value conjunct built (the guard-literal joins
`sole_populator`'s census; Sol's red test greens). My lean, weakly: **(b)** — the account
is about to become emission substrate, `rul-unsure-falls-toward-sh-parity` is the
controlling posture for exactly this class of linguistic behavior, and the check is
small; but (a) is genuinely tenable and this is the human's text to amend.

## §6 — The fix-now footnote (full treatment in `30Mg`; one line each)

Adjudicated obviously-wrong + design-free; all lanes' evidence commits referenced from
the remit. See `30Mg` for acceptance criteria, evidence pointers, and sequencing:

1. Pre-source root-occurrence repair (§1; with the stated floor-posture default).
2. Dot-locals threading — sourced top-level assignments reach the caller
   (`30Mc:finding-dot-locals-are-discarded`; demonstrated; sh-parity TYPED direction;
   funcenv = license-review-tier care).
3. `demote_on_trip` in all four missing producers + a producer-enumeration fence (§2).
4. Members-path certifier floor: typed `StaleCause::SolveInconsistent` replaces the
   sentinel wall; narration stops claiming `TotalWall` for our own defect; add the
   missing pin (`30Mb` §1; unreachable today, double-contained — verified).
5. Guard-redirect refusal disclosure: `refused_render_steps` adopts the full
   `guard_render_refused` predicate + a reason variant; conductor-verified (`30Mf` F2).
6. Spine flat falsehoods: `invalidator` from the real invalidator set; `InlineCall`
   cells populated; `SpineInvocation.mode` truthful (`30Mc` F3).
7. Hygiene batch: `pin28-variable-resolved-source-loads` re-head/re-shape (the named
   `30Ib` fix); `prove_inline_replaceable` doc-comment; `AbstractRc` doc + negative
   pins for the records-grounded fence (incl. the if-no-else-controller cell);
   the synthetic consumer-map test de-fake; the funcenv literal-plane red-first pin;
   an env-exported-sentinel containment cell (`30Mb` §9).

## §7 — Verification ledger (what was checked, and how)

- **Conductor-verified by own read**: the guard-redirect disclosure gap (`lib.rs:4740`
  vs `:4792`, narrative hardcoded Heredoc at `:4687`); the steering lag in
  `plan/CLAUDE.md` (its trip-cleanup and effective-reach bullets confirm `30Mb` §3 by
  inspection); the sibling-tip survival of the loading/trip/Spine findings (diff-stat:
  `funcenv.rs`, `certifier_trip.rs`, `spine.rs` untouched on `ai/r30-conduct`).
- **Mechanically re-run under conductor direction** (Sonnet scout, fresh worktree, both
  evidence branches, 2026-08-21): all four committed red reproducers CONFIRMED failing
  with exactly the filed outputs.
  - `a_pre_source_dependency_runs_only_at_its_authored_dot` (branch
    `worktree-agent-aae734f66adb32de5` @ `de3a01b9`): RED —
    `left: Live(DefinitionId { file: SourceFileId(1), … })` vs `right: Withheld`.
  - `a_sourced_assignment_sites_a_later_load` (same): RED —
    `left: ["root.sh", "entry.sh"]` vs `right: ["root.sh", "entry.sh",
    "vendored/common.sh"]`.
  - `a_tripped_plan_projected_without_cleanup_must_not_retain_elision`
    (`worktree-sol-adversarial-30M` @ `344dd38f`): RED — "a genuine certifier
    disagreement must reach the terminal demotion before projection".
  - `a_mismatched_sentinel_literal_must_take_the_source_arm` (same): RED —
    `left: ["common.sh"]` vs `right: []` ("the live v1 assignment cannot satisfy
    alpha's v2 comparison").
- **Credited-on-report** (self-checking lanes; consistent internally and across lanes):
  Fable's §1 members-floor structural analysis and §9 custody audit; Sol-neutral's seat
  reads for §1/§4's mechanisms; both DeepSeek `did not hold` closures. Nothing below
  rests on a DeepSeek-only claim except `30Mf` F2, which I verified myself.
- **The detritus branches**: `worktree-agent-aae734f66adb32de5` (sol-neutral: two
  evidence tests + a scratch synthesis ledger) and `worktree-sol-adversarial-30M` (two
  evidence tests). Disposition: KEEP until the `30Mg` builder cherry-picks the four test
  commits (`c304dc99` · `5e614861` · `1dbca1ab` · `176e0818` — see `30Mg`; the builder
  un-ignores each as its fix lands); the scratch ledger's content is subsumed by `30Mc`;
  both branches delete after the remit folds.

## §8 — What held, unified (so nobody re-derives it)

The union of all five lanes' positive verifications and did-not-holds, deduplicated —
these are now multiply-independently-checked and should be treated as settled unless new
evidence arrives:

- The `30Kb` repair set is real: aggregate survival walks every erased establish under
  one exact ordered identity shared with the vouch proof; the render-refused replacement
  retains its wall; `decide_site` projects disposition and semantic act from one private
  conclusion; replacement-death takes the `&ReplaceLicense`; inline-owner retirement
  works through `ExecutionOwner`. (Fable trace + Sol-neutral read + DS-neutral read.)
- The erasure mint (`prove_dead_branches`) demands all four conditions; fold-`Omit`
  behind a live guard renders verbatim (`is_neutralised`) — two lanes independently
  chased the same suspicion into the same fail-safe.
- The guarded-source recognition seat is clean for authority (Fable's §9 audit: both
  attack cells pre-pinned in the committed corpus; six withholding conditions each
  negatively tested; composition-seat suspension battery real).
- `rederivation-is-demote-only`, the trip cleanup's occupancy census independence, the
  `Grade::Must` hardcoding, speculative-loads-mint-no-speaker, loader cycle/depth ⇒ ⊤
  withhold, and `PlanAuthority`'s no-bypass are each verified at their seats by at least
  one lane with a specific read.
- Fable's eleven-entry and the two DeepSeek `did not hold` lists stand as filed; nothing
  in them was contradicted by another lane.

## §9 — Scheduling synthesis (for the human's sequencing, with the sibling in flight)

The sibling lane on `ai/r30-conduct` has landed `30La` + `30I` 5b/6 (verified by
diff-stat; the loading findings survive it). Recommended order, cheapest-safe first:

1. **Fold the sibling's work as planned**, then run the `30Mg` remit as one builder lane
   over the merged tip — its items are independent of 5b/6 except the pre-source repair,
   which must land BEFORE any bundle-shape golden promotion or step-7/8 artifact work
   treats bundle-root population as expected (§1). The remit builder verifies each
   finding still reproduces at its tip first (cheap; the four red tests make that
   mechanical).
2. **Three one-line human rulings** unblock everything design-tier here:
   wall-narration ratify-or-mint (§3, with the `30L` rider sentence); the sentinel-value
   ratify-or-build (§5); the Spine grade boundary direction (§4.1). None is large; all
   three sit on surfaces the next arcs freeze.
3. **Assign the `309` boundary close** (§4.3) before `30I` steps 7/8; fold
   `30M:rec-dissolve-trip-must-remember-structurally` (§2) into it.
4. The `307` §5 veto-sweep pile and the unowned precision-loss items (`30Mb` §5) remain
   round-close work, unchanged in urgency — listed here so the round-close checklist
   inherits them from one place.

Conductor-owed follow-through (mine, not the remit's): apply the `30Ka` §7 steering
texts (now overdue — `plan`/`analysis` crate law still describes the deleted machine;
`30Kb:finding-steering-law-lags-the-code` stands at tip); rewrite
`FORFEITS:forfeit-guard-tier-classed-decline` to current truth; close or restate
`30Ib:dev-the-probe-artifact-still-ships-under-a-refusal` against the suspension model;
drop a superseded-marker beside `trial/r26/predictions.md` §7; carry the
hand-authored-transcript disclosure (`30Ib` §12.3) into the step-8 bless review.

## §10 — Lane calibration (for future dispatch tuning)

- **Fable (adversarial, worker)**: the only lane that found the governance layer
  (unratified deviations on welded law; ownerless residue), the deepest self-checking
  (§9's second-sitting audit closed its own §7 flag), and the best did-not-hold
  discipline. Its one miss: the acquisition-side pre-source replay, in the very lane it
  audited deepest — it audited recognition/custody (funcenv-down), not acquisition
  (main.rs-down). Complementarity with Sol-neutral was near-perfect and worth engineering
  for rather than hoping for.
- **Sol-neutral (worker)**: the round's single most valuable report — two demonstrated
  criticals with committed reproducers in the least-reviewed lane, plus the Spine schema
  audit. The "neutral" framing did not blunt it.
- **Sol-adversarial (worker)**: efficient, narrow, both findings real, both with red
  tests; the `did not hold` list caught and closed six plausible attack lines.
- **DeepSeek (both, review-only)**: earned their seats. The adversarial lane's F1
  independently converged with Sol-adversarial (cross-lineage confirmation) and its F2
  was a genuine production find no other lane made (conductor-verified); its severity
  self-bounding was the best-calibrated in the stable. The neutral lane's three findings
  were small but none was wrong. Note `30Mf` was returned as a final message per its
  read-only rules and filed by the collector — the report's own note about not writing
  to disk is expected behavior, not a failure.
