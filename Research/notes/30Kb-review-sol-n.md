# 30Kb - Effective-world-reach architectural review (Sol neutral)

> Tier: post-build architectural and product review of `ai/r30-static-loading` at
> `ab02a661`, against `notes/30K`, `plans/28Q`, the root product documents, and the
> current steering law. This is not a code-quality review. It concentrates on wrong
> execution decisions, authority widening, lost product value, missing explanation
> inputs, and deviations that would shape later product work. Grades:
> **[+SURE]** directly established from the implementation/design · **[~SUSPECT]**
> reasoned but not demonstrated by an executed specimen · **[-GUESS]**.
>
> Review method: read the complete work order and `30Ka`; inspect the changed
> `analysis`/`plan`/`cli` decision seats and their crate laws; trace each claimed
> deviation through its consumers; re-check probe/apply, admin/engineer, and
> reliable/unreliable-oracle cells. No branch gate was rerun for this review.

## 0. Review verdict

**[+SURE] `review-stage-is-not-architecturally-closed`** - the unification's main
direction is right, but the branch must not be treated as a completed
`28Q:stage-effective-world-reach` implementation yet. One aggregate path can elide a
needed mutation under `--risk-faultless-skips` even when every oracle claim is correct;
one execution-owner path fails to retire walls when its owner elides; and two new
decision causes are deliberately omitted from the explanation model. The first is a
correctness blocker. The others materially under-deliver the product theorem and the
aid architecture the work order required.

**[+SURE] `review-bless-does-not-close-findings`** - the enumerated golden movements
are mostly desirable, but blessing them cannot close any finding below. The drift says
what the current implementation does; it does not establish that the new authority
boundaries are correct. In particular, no moved golden exercises a correct upstream
footprint colliding with a non-representative aggregate member.

**[+SURE] `review-next-arc-remains-blocked`** - bundle/artifact work should not accrete
on the present settlement boundary until the aggregate authority and owner-retirement
questions are corrected. `28Q` currently records the stage as BUILT; that status is too
strong while the review-blocking paths remain.

## 1. Critical: aggregate survival checks one fact, then erases many

**[+SURE] `finding-aggregate-backing-underchecks`** - `30Ka`'s
`dev-aggregates-take-one-position-freshness` is not conservative as built. The
implementation asks `survival_subject` for exactly one fact: the first member of an
`EstablishMembers` family, or the first ambient establish inside an `InlineCall`
(`plan/src/settle.rs:539-556`). `WallPolicy::freshness` constructs one `Backing` from
that fact. `aggregate_outcome` then accepts `FreshSurvived` and attaches that one
`SurvivalWitness` to a replacement that erases the entire aggregate
(`plan/src/lib.rs:4139-4157`).

**[+SURE]** The failure can be grounded with an ordinary book shape:

```sh
hork refresh-package curl
for pkg in nginx curl; do
   apt-get install -y "$pkg"
done
```

Assume the following, all within the intended trusted-survival contract:

1. **[+SURE]** both member verdicts measured converged before apply;
2. **[+SURE]** every erased member carries its own reached vouch;
3. **[+SURE]** `hork refresh-package curl` really runs;
4. **[+SURE]** its oracle accurately claims a footprint containing only the `curl`
   package cell; and
5. **[+SURE]** the admin supplied `--risk-faultless-skips`.

**[+SURE]** If `nginx` is the aggregate's representative, the wall footprint is compared with the
`nginx` backing and proves disjoint. The implementation can therefore mint
`FreshSurvived`, replace the whole loop, and omit the needed `curl` establish even
though the author's footprint was exact. The same construction applies to an inlined
call whose first establishing body site is disjoint while a later one collides.

**[+SURE] `finding-aggregate-failure-is-engine-owned`** - this is not the survival
tier's deliberately bought oracle unsoundness. No author omitted a footprint cell, no
resolver split a referent, and no admin misunderstood the flag. Dorc quantified over
only one member of the aggregate it erased. It violates
`277:set-lifting-universal-meet`, which requires universal quantification over every
footprint-by-backing member, and it violates `30K` section 5.4's explicit requirement
that every erased establish carry effective freshness.

**[+SURE] `finding-aggregate-failure-misattributes`** - the resulting why-chain is
worse than merely incomplete. It can truthfully display the representative backing and
the correct upstream footprint as disjoint, then direct suspicion toward the footprint
author even though the omitted later member was Dorc's error. That lands in the
mis-attributed-error tier of `271:rul-sin-ordering`, the project's worst failure class.

**[+SURE] `required-aggregate-universal-proof`** - the durable correction is a private,
non-empty aggregate freshness proof carrying the exact ordered erased-establish set and
universally checking each establish's backing against every reaching wall. Its identity
and cardinality should align with `AllEstablishesVouched`, without conflating the two
proofs. A vouch proves the author's judgment exists; effective freshness proves that
the measurement still applies at this plan position.

**[+SURE] `required-aggregate-running-floor`** - if that proof is not built in this
fold, any aggregate facing a non-empty reaching-wall set must run. Merely forbidding
`FreshSurvived` for aggregate outcomes is the safe short floor: wall-free
`FreshClean` aggregate replacement remains available, while survival value is
explicitly forfeited rather than unsafely approximated. Only after that floor exists
would a `FORFEITS` row describing lost aggregate survival be truthful. The proposed
`forfeit-aggregate-single-position-freshness` text is not truthful against the current
implementation because the current implementation can grant rather than only withhold.

**[+SURE] `required-aggregate-regression-cell`** - the acceptance corpus needs both
aggregate species under the risk flag, with a correct footprint that is disjoint from
the first member and collides with a later member. The expected outcome is whole-
aggregate Run until universal freshness exists, then replacement only when every
member is spared. Existing aggregate tests prove vouch cardinality and convergence;
existing sparing tests prove universal meet inside one `Backing`; neither composes the
two surfaces.

## 2. High: an elided inline call keeps its body walls

**[+SURE] `finding-inline-owner-wall-persists`** - the new `ExecutionOwner` relation is
recorded correctly at lowering, but the no-execution ledger cannot currently consume
the key inline-call case. An inlined CALL node is intentionally classified `Pure`; its
spliced body nodes carry the actual establishes/kills/opaque effects
(`analysis/src/effect.rs:1174-1216`). The CALL can nevertheless receive an aggregate
replacement after every body establish is measured and vouched.

`DecideSite::invalidator` is populated with
`classification.invalidators.contains(call_node)` (`plan/src/settle.rs:462-477`). Since
the CALL node itself is `Pure`, `site_act` returns `NoMutationProof::NotEffective`
before inspecting its `Disposition::Replace` (`plan/src/lib.rs:3889-3905`).
`NoExecutionLedger::record_round` deliberately drops `NotEffective`. Consequently,
`effective_invalidators` sees no proof against the CALL owner and retains every
spliced body wall it owns (`plan/src/world.rs:271-283`).

**[+SURE] `finding-inline-owner-costs-product-value`** - this is conservative rather
than a wrong elision, but it breaks a central product theorem: an elided command casts
no wall. A fully converged helper call can disappear from the plan while every modeled
mutation inside it still forces downstream lines to guard or run. That is exactly the
attention/value recovery effective ownership was meant to provide, and it contradicts
`30Ka`'s claim that a spliced body retires with the CALL owner.

**[+SURE] `required-owner-effectiveness-is-derived`** - the decision seat needs the
question "does this render unit govern any effective invalidator?", not only "is this
owner node itself an invalidator?" A replacement-death proof for an owner must retire
all and only invalidators whose `ExecutionOwner` names that owner. The proof still
depends on the aggregate's complete vouches/freshness and on rendered death; ownership
does not manufacture authority, it only scopes the consequence of an authority already
minted.

**[+SURE] `required-inline-owner-regression-cell`** - add an inline call with at least
two modeled body mutations followed by a separately converged site. When the call runs,
the later site guards/runs; when the call validly replaces, its body walls disappear and
the later site may elide. This must be tested independently of the critical aggregate
survival cell: the owner test is wall-free at the call and checks downstream retirement,
while the aggregate test checks incoming-wall authority.

## 3. High product gap: honest walls lose their account

**[+SURE] `finding-honest-walls-lose-narration`** - settled wall-formation narratives
are still gated on `WallPolicy::RiskAccepted`. `one_round` pushes a wall leaf only when
`accounts_survival` is true and the leaf decision says `MayMutate`
(`plan/src/settle.rs:441-486`). Honest-mode mutations therefore form decision-bearing
walls without leaving the `WallFormation` account `30K` section 7 explicitly required.

**[+SURE] `finding-render-chafe-is-not-mint-policy`** - `30Ka` justifies the omission by
the three `[unnarrated: WallFormation]` lines that appeared in deepest why transcripts.
That is opposite the project doctrine. `AID-NEEDS:law-collapse-mints-narrative` requires
the mint at the safety-narrowing; `289:seam-narrative-render-unconsumed` records that
minting outruns consumption; and the spike-era `kWARN` posture deliberately pays noisy
output now to preserve the architecture that can tune it later. A missing render
consumer may change selection, never whether the semantic account exists.

**[+SURE] `finding-nonleaf-walls-have-no-account-seat`** - simply removing the flag
condition is insufficient. The `walls` vector is produced from ordered leaf decisions,
while effective reach also contains command-substitution internals, redirection writes,
and always-at-node invalidators. Those handles may force downstream stale facts without
ever appearing in `ProvisionalEffectiveRound::walls`. The final account must come from
the settled effective wall population, mapping each wall through `ExecutionOwner` and a
source/participant identity where one exists; it cannot be reconstructed from leaf
`EffectiveAct`s alone.

**[+SURE] `required-wall-account-stays-decision-inert`** - retain compact wall handles
and their owner/participant mapping through settlement, mint `WallFormation` on the
settled round in both policies, and let arrangement/selection decide whether default,
deep, or `--all` surfaces display it. No durable schema change is required: whylog replay
can re-derive the in-memory account from its frozen inputs.

## 4. High product gap: replacement cascades have no cause

**[+SURE] `finding-replacement-cascade-unattributed`** - effective settlement creates a
new valid causal chain: an upstream mutation is replaced, its wall disappears, a
downstream Query becomes valid, and the Query's measured status proves another branch
dead. `attribute_dead_branch_cascades` intentionally filters `NoExecutionLedger` to
`DeadBranch` entries and omits `Replaced` entries (`cli/src/fixpoint.rs:108-178`). The
downstream decision can therefore depend on the replacement while `dorc why` has no
record of that dependency.

**[+SURE] `finding-cause-model-precedes-prose`** - `30Ka` says this needs a render shape
that does not exist and classifies inventing one as prose work. The model and the words
are separate obligations. The ledger already has the cause species, source site, fact,
and first-proof round. A typed `CascadeCause` can preserve
`DeadBranch { controller, controller_rc, ... }` and
`Replacement { site, fact, ... }` without writing one word of user prose. The renderer
may initially select nothing or show an existing unwritten/component placeholder; the
cause must not be discarded because its sentence is not authored.

**[+SURE] `finding-cascade-gap-bears-on-product-design`** - this is not polish. Dorc's
attention product removes commands, and its recovery product must answer which earlier
decision made that removal legal. Replacement-driven cascades are likely to become more
common as effective-world reach and later context availability recover more precision.
Leaving the species absent now biases later product-surface work toward a dead-branch-
only causal vocabulary that the engine has already outgrown.

**[+SURE] `required-cascade-cause-union`** - generalize the post-settlement attribution
model over every `NoExecutionProof` species that can flip effective Query validity.
Preserve each species' genuinely different operands: a replacement has no controller
line and must never borrow one; a dead branch has one and should keep it. This remains
decision-inert and does not require durable growth.

## 5. Medium: the semantic act structurally reads the output disposition

**[+SURE] `finding-semantic-act-reads-disposition`** - the implementation does not meet
the structural claim recorded in `30K`/`28Q`/`30Ka` that `Disposition` never feeds
effective analysis. `decide_site` computes a `Disposition`, passes `&disposition` into
`site_act`, and `site_act` matches that public output type to decide which
`NoMutationProof` to mint (`plan/src/lib.rs:3868-3905`). There is no Rust `From` impl,
but `site_act` is semantically that conversion inside the sole constructor.

**[+SURE] `finding-replacement-proof-does-not-bind-license`** - the nearby witness is
weaker than its documentation. `ReplacementDeathProof::mint` receives `(site, fact,
renders_dead)`; it does not receive or consume the `ReplaceLicense` it claims to bind
(`plan/src/world.rs:113-145`, `plan/src/settle.rs:567-575`). The current lexical census
ensures one caller, and that caller happens to sit under a `Disposition::Replace` match.
That is co-location plus a fence, not possession of the authority.

**[~SUSPECT] `risk-current-constructor-is-locally-safe`** - the present sole caller and
render-feasibility check probably prevent an immediate independent mint. The concern is
architectural: the code and durable law disagree at precisely the authority boundary
later work will treat as established. A future disposition refactor can move the output
and act separately while every type continues to compile, because the shared proof the
design promised does not exist.

**[+SURE] `required-private-decision-proof-projects-twice`** - compute one private
decision conclusion/witness from the license, freshness, dead-branch proof, and render-
death predicate, then project both `Disposition` and `EffectiveAct` from it. The
replacement-death variant should hold or consume the actual replacement authority,
subject to whatever output-exempt payload must be stripped before ledger equality. The
effective side must not accept the public output enum as input.

## 6. Medium: backing placement is right, backing lifetime is not

**[+SURE] `finding-backing-authority-separation-is-right`** - `30Ka` is right that
`FactBacking` is not one of the admin/oracle authorities that inhabit
`WallPolicy::RiskAccepted`. Footprints, resolutions, and dialect belong in the closed
policy sum because their use is flag-gated. A fact backing is derived model data, not
consent.

**[+SURE] `finding-backing-should-remain-frozen`** - the report's further claim that
backings are per-round values that change as settlement erases is the wrong lifetime.
The backing says what the already-compiled, already-executed frozen probe read. Apply-
side proof that a book mutation cannot execute does not retroactively change the probe's
read-set. `30K` section 4.1 correctly listed backings among frozen inputs.

**[+SURE] `finding-current-backing-recompute-is-accidentally-stable`** - today,
`resolve_node_effects` collects backings before the `ErasedSites` overlay rewrites effect
cells to `Pure` (`analysis/src/effect.rs:1890-1910`). Reclassification therefore appears
to reproduce the same backing map each round despite threading it through
`RoundClassification`. This makes the deviation unnecessary now and leaves a future
trap: making backing collection respect erasure would silently narrow survival inputs
and could spare more.

**[+SURE] `required-backing-is-frozen-beside-policy`** - keep backings beside, not
inside, `WallPolicy`, but freeze them with the probe/model inputs before settlement.
Every round should consume the same backing account. Any future refinement from actual
probe path/outcome is a separate, explicitly measured authority change, not a side
effect of effect erasure.

## 7. Status and completion consequences

**[+SURE] `finding-built-status-is-premature`** - `plans/28Q` now marks
`stage-effective-world-reach` BUILT and points later work at the new settlement as a
completed boundary. The critical aggregate authority error and the incomplete owner
projection make that statement unsafe for successor builders. The stage should read
review-blocked/partial until those are repaired; otherwise later bundle and world-scope
work will correctly trust a false invariant.

**[+SURE] `finding-steering-law-lags-the-code`** - `30Ka` leaves the root and crate
steering updates as proposed residue. This is especially consequential here because the
branch intentionally changed which reach species owns final freshness and introduced
`ExecutionOwner`. Successor agents auto-load the old crate laws, including text that
still seats re-derivation in the deleted wall walk. The steering laws must be updated
after the review corrections firm, not before and not omitted.

**[+SURE] `finding-status-undercounts-deviations`** - the current status prose says four
open deviations while `30Ka` section 3 records six. More importantly, the aggregate
deviation is described as conservative rather than review-blocking. The durable report
should retain the historical builder claim, while the current plan/status must reflect
this review's correction rather than propagating that classification.

**[+SURE] `required-close-order-after-review`** - the minimum close order is:

1. repair or safely floor aggregate survival and add both aggregate regression cells;
2. make owner-level replacement proofs retire inline body walls and add the downstream
   un-walling cell;
3. make the semantic-act/disposition mint satisfy the chosen structural law;
4. freeze backings at the frozen-probe boundary;
5. retain honest/non-leaf wall accounts and replacement cascade causes as typed,
   decision-inert inputs;
6. re-run the decision/run-set differential before any new blessing;
7. update `28Q`, the needs/forfeits registries, and crate steering to the reviewed truth;
8. only then resume bundle/artifact projection over the corrected settlement boundary.

## 8. Deviation-by-deviation disposition

| `30Ka` deviation | review call | architectural reason |
|---|---|---|
| `30Ka:dev-replacement-death-does-not-erase-effects` | ACCEPT direction; rewrite the work-order expectation | replacement changes effective execution, not the authored command's described effect; rewriting the effect to `Pure` destroys the class needed to re-prove the replacement |
| `30Ka:dev-backings-ride-beside-the-policy` | ACCEPT placement; REJECT per-round lifetime | backing is not authority, but it describes the frozen probe and should not change under apply-side erasure |
| `30Ka:dev-aggregates-take-one-position-freshness` | REJECT; correctness blocker | the implementation uses one representative backing to license erasure of every aggregate member, violating universal meet and permitting wrong elision with correct claims |
| `30Ka:dev-wall-formation-account-stays-flag-gated` | REJECT; product architecture blocker | semantic mints must not depend on current render consumption or the risk flag; honest and non-leaf walls are decision-bearing causes too |
| `30Ka:dev-replacement-death-cascades-are-unattributed` | REJECT as stage completion | the typed cause is already derivable from the ledger; absence of prose does not license discarding causality |
| `30Ka:dev-effective-reach-component-prose-unminted` | ACCEPT as explicit prose residue | the reason variant and emission path exist; human/conductor authorship and a genuine defining case are correctly still owed |

## 9. Terse acknowledgements

**[+SURE] `ack-one-effective-world-is-correct`** - replacing origin validity plus two
late walks with one certified `ReachingWalls` answer is the correct architecture. Query
validity, establish freshness, total walls, and survival now share one CFG-aware world.

**[+SURE] `ack-guard-rung-restores-product`** - the broader Run-to-Guard movements are
desirable: a modeled running mutation should not punish an honest oracle by deleting the
guard rung. The two frame Guard-to-Replace movements also correctly realize the rule that
an elided upstream mutation casts no wall.

**[+SURE] `ack-nonleaf-wall-closes-real-hole`** - the command-substitution movement from
Replace to Guard closes a genuine wrong elision. Recording execution ownership at
lowering, defaulting unclaimed nodes to always-active, and keeping policy material out of
the lattice are strong choices worth preserving through the repairs above.

**[+SURE] `ack-certification-shape-is-sound`** - the effective solve uses the existing
certifier without weakening its question, latches failures across provisional rounds,
and takes a guard/run floor. The risk policy remains a closed inhabited type, and
survival re-derivation stays demote-only before a replacement proof can settle.
