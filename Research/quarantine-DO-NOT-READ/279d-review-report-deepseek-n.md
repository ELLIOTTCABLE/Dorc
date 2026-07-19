# 279d — review report: the 270–278 block-settle design package

AI-authored (deepseek-n review agent), 2026-07-13. Critical review of the nine-document
block-settle package produced by the round-27 design sittings (2026-07-10 through
2026-07-12). The package comprises: `plans/270` (charter), `plans/271` (rulings ledger),
and `notes/272`–`notes/278` (the seven arc durables). Assessed against the root grounding
documents (README, DESIGN, IMPLEMENTATION, USER_STORY, KNOBS, TODO-ADDTL).

**Weighting applied:** HIGHEST value → internal contradictions, reasoning that does not
hold, decisions hard to undo once built. LOW value → code-quality complaints, market-fit
questions, findings from earlier review rounds. Working notes committed separately as
`279d-review-working-notes.md` (not for citation — the final record is this document).

**Overall assessment:** The package is substantively sound. The block-settle sittings
resolved all 15 charter items with typed rulings, dissolved several long-standing design
tensions (the wrapper context-function, the carrier declaration, the capture-claim fork),
and produced a coherent entity algebra. The six findings below are real but none
threatens the package's viability; the top two are sequencing/ratification risks, not
logic errors.

---

## Findings (most severe first)

### F1 — CRITICAL: Block-context stages build on a probe-composition mechanism that is confirmed debt and gated on a deferred task

**Severity:** critical &emsp; **Confidence:** medium

The package designs block-context machinery — wrapper-sudo probe-outside licensing,
eval'er reentry shipping, and the capture lane's delegation pipeline — on the assumption
that oracle `predict()` bodies can compose transitively as shipped probe stand-ins. The
mechanism to do this (probe-form composition, `notes/273` §6) is explicitly DRAFTED and
gated on task 14 ("re-derive the structural-vouch hard law in a fresh session"). The
current mechanism — shipping raw book bytes licensed by engine-side shape-matching — is
confirmed as "STANDING-LAW DEBT" against the round-20 structural-vouch ruling
(`notes/24J` header; `LIVING_STATUS` 2026-07-11).

The charter (`plans/270` §2) sequences block-context after block-rebuild, and task 14 is
"DEFERRED past the Fable window" with a re-entry pointer of "Opus-conductable"
(`plans/271` task map item 14). The deferral's scope clause says task 14 "Gates
block-context lanes only; block-rebuild never waits on it." This is correct about
block-rebuild but the clause understates the dependency: block-context IS the consumer,
and building it against the debt-confirmed mechanism risks baking structural assumptions
(about how composed probes ship, how per-channel coverage gates substitution, how wrapper
predict bodies compose with inner-tool bodies) that task 14's resolution may invalidate.

The mitigating factors are real. The wrapper-peel stage (W1) regression is "wrapper-free
corpus goldens byte-stable" — it may not need composition at all. The eval'er bare lane
(`notes/274` §5) ships real bytes, not composed bodies. The read-value-slice captures
real stdout. So the dependency is narrower than "all of block-context"; it concentrates
in wrapper-sudo (W2), the eval'er transform lane, and composed capture pipelines. But
those are the headline-value stages — the ones the charter justifies block-context's
existence with.

**Recommendation:** Before block-context dispatch, either resolve task 14 or explicitly
scope block-context's first dispatch to stages that do not depend on composition (W1
wrapper-peel, bare-lane eval'er shipping, single-command read-value captures), leaving
W2+ and the transform lane for a post-task-14 follow-on.

**Citations:**
- `notes/273` §6 (probe-form composition is DRAFTED, task-14-gated): lines 217–244
- `notes/274` §5 (transform lane task-14-gated; bare lane is independent): lines 160–170
- `plans/271` task map item 14 (DEFERRED past Fable window): lines 87–92
- `notes/24J` header (current mechanism is standing-law debt): lines 12–13
- `plans/270` §2 block-context stages: lines 130–164
- `LIVING_STATUS` (task 14 "gating block-context lanes only"): lines 42–45


### F2 — MAJOR: The formal spine's generator registry is conductor-proposed, not ratified, yet the entity-algebra rebuild ships against it

**Severity:** major &emsp; **Confidence:** medium

`notes/277` §2 defines the formal spine of the entity algebra: one ternary comparison
relation, a generator registry mapping every authored surface (disturbs claims, invariance
lines, lend entries, selector-dialect comparison, keying, carried-by rows) to the
comparison verdicts each may produce, and a consumer map (which verdict feeds transport vs
survival). This is the load-bearing abstraction that makes the whole design cohere — every
other component feeds into this chokepoint.

The status table (`notes/277` §9) records: "the generator registry stays
conductor-proposed, on the table for the adversarial pass." The individual generators are
ratified (coordinate shape, selector dialect, invariance speech-act, lend-map entries are
all typed), but the composition of them into the registry — which verdicts compose how,
which generator outranks which, the precise interaction of keying with lend entries — is
not.

The entity-algebra-rebuild brief (`notes/277` §7b) implements against this spec. If the
human's adversarial pass finds issues with the registry's composition rules, the rebuild's
chokepoint implementation changes — and the chokepoint is what every other component feeds
into. The risk is bounded: the individual pieces are typed, and the registry is "just"
their composition. But "just composition" is where design-level bugs hide (the
safety-inversion in `notes/273` §4, which the ternary relation correctly encodes, is
exactly the kind of thing a wrong composition would break).

**Recommendation:** The adversarial pass should explicitly test the generator registry
against the three hardest cross-generator interactions: (a) a mapped lend + a keyed kind
producing transport within the mapped world, (b) a full lend + an invariant kind producing
transport across the boundary, and (c) a disturbs claim + a dialect-scoped selector
producing survival sparing. If these hold, the registry is sound enough for rebuild.

**Citations:**
- `notes/277` §2 (the formal spine + generator registry): lines 111–149
- `notes/277` §9 (status: conductor-proposed): lines 457–476
- `notes/273` §4 (the safety inversion the registry must encode): lines 171–203
- `notes/272` §1 (the comparison relation the registry feeds): lines 39–57


### F3 — MAJOR: `kind__state_stored_only_in()` carries a wider blast-radius than other at-most claims, priced identically

**Severity:** major &emsp; **Confidence:** high

`notes/272` §2 defines `kind__state_stored_only_in()` as complete-by-contract: the
kind-owner must survey the kind's stores totalistically before authoring. A wrong claim
here causes under-execution across context boundaries — an omitted per-user store means a
sudo'd line elides that needed to run. Unlike other at-most claims (a `cmd__disturbs()`
omission affects only the author's own tool's line; a wrong `cmd__lend_map()` entry
affects only sites under that wrapper), a wrong store-member claim affects every tool that
interacts with the kind across context boundaries. The `dpkg` kind's stores affect every
package-querying oracle; the `cron` kind's stores affect every scheduled-task tool.

The design prices this identically to other at-most claims: "Attributed to the member's
line; same tier as the rest of the at-most family" (`notes/272` §8). The pricing is honest
in the sense that wrongness is attributed and the `only` contract is explicit, but
"same tier" understates the blast-radius difference. The razor-conversion
(`plans/271:rul-invariance-speech-act`) moved the transport license from the derivation's
negative space to a typed line — this converts the failure from cardinal-sin
(unattributable) to vouch-tier (attributable) but does not reduce the fire frequency
(priced honestly in `plans/271`).

The differential harness (`notes/272` §9) can falsify wrong invariance after the fact but
cannot verify the survey was done upfront. This is consistent with the vouch-tier
philosophy — the design has never claimed to prevent author error, only to attribute it.
But the combination of wide blast-radius, no upfront verification, and "same tier" pricing
is a sharp edge worth frontloading more prominently than it currently is.

**Recommendation:** The `kind__state_stored_only_in()` authoring documentation should
frontload the blast-radius warning at the same prominence as the `only` contract itself.
The stdlib quality-bar checklist should add a specific item for this member: "Have you
audited every store this kind's tools can reach from every context?"

**Citations:**
- `notes/272` §2 (the member, the `only` contract): lines 59–103
- `notes/272` §8 ("The knife" — blast-radius description): lines 237–265
- `plans/271:rul-invariance-speech-act` (razor-conversion, bite-rate unchanged): lines 601–619
- `plans/271:rul-at-most-family-names` (the `only`-rule): lines 323–334


### F4 — MINOR: "Invariant" names two distinct mechanisms with different provenance

**Severity:** minor &emsp; **Confidence:** low

The token `invariant` appears in two roles: (a) the `invariant:<axis>` speech-act on a
colon-line inside `kind__state_stored_only_in()` (`notes/277` §4e) — vouch-tier, typed,
authored by the kind-owner; and (b) the "invariant" outcome of the derivation's carried-by
table (`notes/272` §3 r1) — structural/engine-warranted, derived from substrate marks. Both
produce transport/probe-outside licenses, but with different provenance and different
failure modes (wrong authored invariance = vouch-tier knife; wrong carried-by row =
engine bug).

The status table (`notes/272` §12) distinguishes them, and
`plans/271:rul-invariance-speech-act` is explicit that "substrate-borne carried-by rows
stay engine-warranted structural tier, never author-owed." The naming collision is mild
and the design documents are clear when read carefully. An implementor reading top-to-bottom
could briefly conflate them, but the distinction is recoverable.

**Recommendation:** Consider renaming the derivation's outcome to "carried-invariant" or
"structural-invariant" in the rebuild brief to make the distinction grepable. Low urgency.

**Citations:**
- `notes/277` §4e (the authored invariance line): lines 293–315
- `notes/272` §3 r1 (the derived invariance from carried-by table): lines 109–114
- `notes/272` §12 (status: authored is TYPED, carried-by is conductor-proposed): lines 341–356
- `plans/271:rul-invariance-speech-act` (substrate-borne rows stay engine-warranted): lines 601–619


### F5 — MINOR: `notes/272` §3 describes r2 with pre-amendment semantics in its body text

**Severity:** minor &emsp; **Confidence:** low

`plans/271:rul-invariance-speech-act` re-roled the emission-set non-interference derivation
(`notes/272` §3 r2) from a license-generator (transport licensed from silence) to a
contradiction-checker plus keying/conjecture/hints. The amendment is recorded in a
prominent block at `notes/272` lines 139–143, but the main body text of §3 (lines 105–153)
still describes the pre-amendment semantics: the three outcomes (invariant, keyed, ⊤) are
listed with "invariant" described as "buys BOTH the identity bridge … AND the
probe-outside license" (line 135). The amendment note changes which mechanism actually
issues that license, but a top-to-bottom reader who hasn't yet reached the amendment may
internalize the old semantics.

**Recommendation:** Rewrite §3's outcome descriptions to state the post-amendment
semantics directly, with the amendment history in a footnote rather than an interrupting
block. The rebuild brief should cite the amended text, not reconstruct from the old body
+ amendment.

**Citations:**
- `notes/272` §3 body (pre-amendment semantics): lines 105–153
- `notes/272` §3 amendment block (post-amendment re-role): lines 139–143
- `plans/271:rul-invariance-speech-act` (the typed ruling): lines 601–619


### F6 — MINOR: "Incorrectness-inexpressible" typesystem claim for invited-rooms typing

**Severity:** minor &emsp; **Confidence:** low

`notes/274` §1 states that descend-don't-license enforcement lives at the TYPESYSTEM tier:
"incorrectness-inexpressible type-differentiation between invited-room analysis (may mint
licenses) and hint-only rooms (may not)." This is achievable in Rust with newtype wrappers
and module visibility boundaries (a `HintFact` type that lacks the trait required by
license-consuming code paths), but the claim of "incorrectness-inexpressible" is strong
for what amounts to a module-boundary discipline. If the type barrier has gaps —
particularly in generic code that operates uniformly over facts regardless of provenance —
hint-lane data could reach survival licensing code.

**Recommendation:** The invited-rooms typing should be pinned by a specific
compilation-failure test: attempt to pass a hint-derived fact to a license-consuming
function and verify the compiler rejects it. This converts the claim from prose to
machine-checked.

**Citations:**
- `notes/274` §1 (invited-rooms typing, enforcement tier): lines 56–57
- `notes/274` §6 (invited-rooms typing, typed direction): lines 195–196


## What the package gets right (non-exhaustive)

- **The razor discipline is uniformly applied.** The package's central design move —
  converting omission-failures into positive mis-assertions on typed, pointable lines —
  runs through every arc: lend_map's enumerate-every-dimension law (`notes/273` §3),
  the invariance speech-act (`plans/271:rul-invariance-speech-act`), the env-claim
  inversion (`notes/274` §2), the descend-don't-license posture (`notes/274` §1). No
  arc missed the pattern.

- **The kWHICHSH weld is well-grounded.** The posh∩dash two-binary floor
  (`notes/276:rul-spec-two-binary-floor`) converts the language-spec question from an
  unbounded design problem into an executable test. The evidence base (committed
  `kwhichsh-gcd/turn01`–`turn02` with binary empirics) is thorough for a design-pass
  artifact. The scope carve (weld binds oracles only; book-acceptance is a separate
  open question) is correctly fenced.

- **The selector-dialect algebra (`notes/277` §3) is carefully load-bearing.** The
  dialect-scoped, minting-by-mark, safe-default design correctly encodes the
  subscription semantics that `plans/233` identified as necessary, without a global
  per-kind vocabulary. The properties list (empty-world → byte-identical, noise fails
  safe on both sides, monotone under loading, no self-licensing) is test-pinnable.

- **The safety inversion (`notes/273` §4) is the package's sharpest insight and is
  correctly applied.** The observation that believed-no-overlap and believed-overlap are
  each dangerous to a different consumer, and that only the ternary relation's *unknown*
  bottom is safe for both, justifies the entire never-derive-separation carve and the
  survival flag's existence. This is a genuinely non-obvious piece of distributed-systems
  reasoning applied to language design.

- **The value-prediction species (`notes/275`) dissolves a long-standing fork**
  (`notes/219` q-5) with a clean representational/behavioural split: same reserved seams
  (OutClaim, cause-tagged ValueOf), new typed fields (provenance grade, per-channel
  backing sets). The authored-surface-empty punchline (§9 — "what they already wrote")
  is the correct kBURDEN answer.

- **The charter's block arc (`plans/270` §2) correctly sequences the churn.** Block-settle
  first (spellings settled), block-rebuild under single ownership (one fixture sweep),
  block-context after (inherits the settled surface), block-stdlib last (authored exactly
  once against the final algebra). The single-ownership decision for block-rebuild was a
  late correction that prevents the multi-pass fixture churn the earlier design would have
  caused.

- **Cross-document consistency is high.** The `touches` → `disturbs` rename, the class-
  prefixed role names, the `:` / `:!` / `:?` sigil family, the `only`-rule, and the
  rc-partition are uniformly applied across all seven arc durables. The naming discipline
  (`plans/270` §1) is followed throughout.

- **The stability ledger (`notes/278` §4) is honest.** "Syntax = marker-gated ·
  `__role` names = permanent · verdicts = unstable-and-improving, disowned" — three
  sentences that prevent the most common design-document failure mode (promising
  stability the implementation cannot deliver).


## Investigated and dropped

Each entry states what was investigated, why it was dropped, and the confidence that the
drop is correct.

### D1 — Whether any ruling contradicts root DESIGN.md or IMPLEMENTATION.md
**Dropped, high confidence.** Checked the key design-theorem touchpoints: DESIGN's
"never generates probes, lifts them" (upheld — all probes ship authored bodies),
DESIGN's "best effort" (upheld — vouch-tier attribution, survival flag's explicit
trust buy-in), IMPLEMENTATION's correctness band and probe/apply inequality
(upheld — imp-1 fences credential escalation, the survival flag gates the one place
trusted claims affect apply). No contradiction found.

### D2 — Whether the double opt-in (admin flag + author claim) correctly isolates risk
**Dropped, high confidence.** `plans/271:rul-flag-is-razor-residue` gates the survival
*consumer* of the comparison relation, not the claim types. Typed lines (invariance,
lend entries, disturb claims) ride the vouch economy un-flagged. The admin's consent
object is a risk-class (open-world at-most residue), not a mechanism. The surprise-
anatomy thread's coherence test (wrong-world-check class generalization) held. This is
well-specified.

### D3 — Whether the charter's struck/deferred tasks indicate planning failure
**Dropped, high confidence.** Tasks 11, 13, 14, 15 were struck or deferred during the
Fable window triage (`plans/271` task map). The charter (`plans/270` §3) listed the
full agenda; the sitting's job was to triage it against a closing Fable window. The
triage is recorded with reasons (11: "always formally owed at block-context
implementation-planning"; 13: "demoted to one line in the entity-algebra crosscheck
packet brief"; 14: "DEFERRED past the Fable window"; 15: "PARKED … re-enters at stdlib
quality-bar authoring"). This is normal design-process refinement, not a flaw.

### D4 — Whether the never-settled-backed regime creates a classification gap
**Dropped, high confidence.** The human HARD-DEFERRED clock-backed values
(`notes/275` §3: "gargantuan meh — no thought or tokens until a real book's `date`
walls and it hurts"). The three regimes (register, world-cell, never-settled) are
clearly distinguished. Any boundary ambiguity would only surface when the
never-settled regime is unparked, which is explicitly not happening in v1.

### D5 — Whether 276's book-tolerance openness contradicts the off-ramp promise
**Dropped, high confidence.** `notes/276:rul-kwhichsh-oracle-scoped` explicitly carves
the weld to oracle/marked dialect text only; book-acceptance is a separate open
question. DESIGN.md's off-ramp promise applies to the stripped artifact (always
floor-legal portable sh). The open question is about analysis *quality* for non-POSIX
books (how many guards/elisions they get), not about whether they can *run*. This is
honest about limitations, not a contradiction.

### D6 — Whether the `--risk-faultless-skips` name is misleading
**Dropped, high confidence (that it's low-value).** The human acknowledged the
potential confusion ("faultless" ≠ "harmless") and demanded a help-text disclaimer.
This is a UX naming concern, not a design-logic flaw. Per the stated weighting rules,
naming nits are very low value.

### D7 — Whether `notes/278` (the reference) misrepresents any ruling
**Dropped, high confidence.** Spot-checked: `cmd__disturbs()` rename, `only`-rule, rc
partition, env-claim ladder, stability ledger all match their source rulings. The
reference correctly marks itself DRAFT and states "on any conflict between this page
and a cited ruling, the ruling wins" — the right disclaimer for an assembly document.

### D8 — Whether the package has internal contradictions between sitting durables
**Dropped, high confidence.** Traced the cross-document dependency chain:
`notes/272` §3 r2 amendment is recorded in both `notes/272` and `plans/271`;
`notes/273` §0 correctly enumerates what died of `notes/24S`; `notes/274` §0 correctly
enumerates what died of `notes/24T` pin1; `notes/275` §0 correctly dissolves the
`notes/219` fork; `notes/277` §4a correctly deprecates the `.prop` suffix. The arc
durables are consistent with each other and with the rulings ledger.

### D9 — Whether the specimen amendments (`notes/277` §7a) cover all grammar changes
**Dropped, medium confidence.** The amendment list covers the five mark-bearing
specimens. I did not walk every specimen in `notes/24P` to verify completeness — that
is the block-rebuild conductor's job at bless-checkpoint-one. The list appears
comprehensive for the grammar changes described in `notes/277` §4.


## Review metadata

- **Documents reviewed:** 17 total (8 grounding + 9 package + 1 context citation)
- **Citations followed into older Research/:** `notes/24J` (header), `LIVING_STATUS`
  (r27 onboarding section), `notes/24C` (referenced but not deep-read — the
  debt-confirmation question was answered by `24J`)
- **Documents NOT reviewed that may contain relevant context:** `notes/24C` full body
  (residue ledger — the pipe-guard section was cited but the full accretion tail was
  not read); `plans/24S`/`24T` full bodies (the wrapper/payload keystones — read only
  through their summaries in `notes/273` §0 and `notes/274` §0); `notes/219` full body
  (the capture-lane origin — read only through its summary in `notes/275` §0)
- **Earlier review rounds checked for overlap:** `notes/236a`, `notes/236c`,
  `notes/23Ia` — these review different rounds (233 crisis, 23I directional) and none
  of their findings apply to the 270–278 package
