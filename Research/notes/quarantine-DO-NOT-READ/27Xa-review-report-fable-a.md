# 27Xa — Outside review of the post-crosscheck repair layer (Fable-A)

AI-authored (Fable, clean-worktree outside review at review-point `de22017`,
2026-07-17; commissioned adversarially: "the response is the artifact"). Scope: the
layer produced between the 270-era crosscheck and the block-rebuild dispatch — the
adjudication (`279f`), the in-place spec amendments (`277` §§3/5/6 · `275` §6), the
self-proposed-then-ratified rulings (`271:rul-only-oracle-bytes-ship` ·
`276:rul-pipefail-emit-never`), the replacement mechanism (`27A`→`27B`→`plans/27C`),
and the current-truth compression (`spike/CLAUDE.md` + the seven crate files).
Authority: root docs and human-TYPED rulings outrank this; everything herein is
finding-tier, nothing closes by silence. Supersession banners on `27A`/`27B` honored
as load-bearing. **Hard exclusions honored: no security analysis of any kind was
performed or is reported here** — one finding (§1 first entry) touches a
privilege-*adjacent* spelling solely as a spec-consistency/coverage matter; its
security dimensions are deliberately not examined and belong to another lane.
Quarantined directories and `corpora/` untouched; the raw 279a–e critic reports are
quarantine-archived, so dismissal-verification below re-derives each dismissed claim
against the package texts rather than the critics' own words.

## §0 — Verdict in one paragraph

The repair layer is substantially better than its production process. The core move —
`27C` measure-in-the-site's-denoted-context — is, as far as this review can determine,
genuinely sound in its default lane: it dissolves the completeness gap by consuming no
cross-context claim at all, its walls-compatibility arguments check out, and its
keystone provenance claim (no root-doc sentence ever promised an unprivileged probe)
survives independent verification. The amendments mostly repair what they answer; the
dismissals I could re-derive all survive scrutiny; the compression is faithful to its
sources in every load-bearing rule checked. What is *wrong* concentrates in five
places: an internal contradiction in `27C` §1's ruled authority predicate on exactly
the connection-class `27B`'s own sizing argument called the norm
(finding-held-authority-cell-contradiction); a vacuous-quantification hole in the
`fix-set-lifting` amendment, propagated verbatim into the steering law
(finding-vacuous-set-lifting-hole); the quiet collapse of two promised-independent
re-derivations into in-context self-ratification, one of them on a law the ledger had
flagged the conductor as contaminated for (finding-independence-collapsed-twice); a
consent-precedent argument in `27B` that the corpus's own standing why-run fence
indicts, never run through that fence (finding-whyrun-fence-not-run); and stale
unflagged transport rows in `277` §2/§9 — the same stale-text hazard class the layer
itself fixed in `275`, left standing in the document the rebuild consumes as THE spec
(finding-stale-transport-rows-in-277). None of these is a kill. All five are cheap
now and expensive after block-context hardens.

## §1 — Findings (strongest first; each independently re-derived)

### finding-held-authority-cell-contradiction — `27C` §1's two ruled sentences disagree, on the cell the pivot's own sizing leaned on

*(Raised strictly as spec-consistency and product-coverage; no security analysis
performed or implied.)*

`27C` §1 states the authority rule twice, incompatibly:

- the predicate: "the only implementable predicate is *can the connection do it with
  zero new credentials*" (`27C:rule-reuse-never-acquire`, ruled);
- the rule: "a root connection performs user-shifts, `chroot`, and netns entry with
  zero new credentials; **a non-root connection performs none of them**."

These classify one connection-class differently: the non-root connection that CAN
perform the shift with zero new credentials (the passwordless-delegation cell —
`27B` §2 finding-premise-unmeasured calls it "the norm" for automation/cloud users,
its regime 2). By the predicate it is reuse (nothing acquired, nothing prompted,
nothing handled); by the rule it performs no shifts. The four-cells text routes it
into "non-root + explicit acquisition mechanism (opt-in; UX deferred,
`27C:open-cell-granted-acquire-ux`)" — but the sketched mechanism (a "one-shot
`sudo -v`-class moment, credential never stored") is a credential-lane construct that
is a no-op for exactly this cell, and calling a zero-new-credential shift
"acquisition" contradicts the rule's own name.

Why this is more than a wording nit:

1. **The pivot's justification leaned on this cell.** `27B` §2's case that the
   transport dead-end was over-weighted rests on three regimes: root-connected ops
   (served by `27C`'s default lane), passwordless-automation (THIS cell), and
   interactive-personal (mostly the guard floor). As ruled, the primary lane serves
   only the first; the second — cited as "the norm" — lands in `27C` §7's
   `hole-static-identity` residue ("non-root, no acquisition: wrapped sites guard"),
   silently including connections that hold the authority the predicate names.
2. **The ambiguity propagates.** `24S` §0's imp-1 bracket ("may re-use authority the
   connection already holds") and ANALYZER-NEEDS `an-privilege-fact`
   ("connection-held authority is re-usable") both carry the predicate reading; the
   four-cells table carries the other. An implementing agent cannot build the entry
   gate without choosing, and neither `27C` §9's build list nor §10's status ledger
   flags that a choice exists.
3. **The honest-residue accounting inherits the blur.** `27C` §7 prices the residue
   as "best-effort tier by ruling" — accurate for genuinely-unauthorized connections,
   silently lossy for held-authority ones.

Ask-shape: one typed sentence choosing a side ("held-authority non-root connections
are {reuse-lane at the default dial | acquisition-cell, deliberately}") plus a
one-line correction to whichever texts lose. I take no position on which side is
right — that weighing involves considerations excluded from this review.

### finding-vacuous-set-lifting-hole — `279f:fix-set-lifting` states the universal law without the side-invariants that make a universal law safe

The amendment (now `277` §5, propagated verbatim to `spike/CLAUDE.md`
set-lifting-universal-meet and `plan/CLAUDE.md` universal-meet-here): "sparing over a
backing-set requires EVERY footprint×backing pair provably-disjoint; any unknown
member ⇒ collide; transport over a backing-set requires every member to transport."
The acked fixpoint clause adds: "a not-yet-licensed member reads unknown ⇒ the set
collides."

Universal quantification over an EMPTY set is vacuously true, and an empty set has no
members to read unknown. As written, the law licenses:

- sparing every wall for a fact whose backing-set is ∅ (nothing to collide with);
- transport everywhere for a value whose backing-set is ∅ (every member — all zero —
  transports). `275` §4's TYPED fold-validity row ("backing fresh over the patrolled
  window") is likewise vacuously satisfied at ∅, while `275` §10's prose states the
  opposite intent ("fold-past-walls needs an observed coordinate on the producing
  arm"; the positional floor). The typed table and the prose disagree at exactly ∅,
  and the amendment formally sides with the permissive reading.

The safe form needs two side-invariants the amended text nowhere states: (a) a fact's
backing-set is non-empty by construction (the minting line's own coordinate is always
a member — value-plane backings from observe-less bodies are the reachable-∅ case);
(b) ⊤/unknown is never *encoded* as ∅ — the two must be unconfusable in the
representation, because on the footprint side ∅ has a legitimate authored meaning
("this matched invocation-shape disturbs at-most nothing", which genuinely spares)
while ⊤ means wall.

Corroboration, both directions, stated honestly: at review-point HEAD the landed code
is conservative — `sweep/src/drive.rs:263` walls on "no touches / non-literal argv /
⊤ / empty emission", and the own-establish coordinate is unioned into authored
footprints — so this is a spec-text hazard, not a live bug. But the in-flight build
branch subsequently had to name exactly the missing precondition (commit subject:
"stage-4b backing-SETS LANDED; **minting-line precondition** + same-kind limitation
named" — post-review-point, cited as external corroboration only). An amendment whose
entire purpose was closing quantification holes (`279b-fd5`: "retrofit-hostile if
wrong") missed the third quantifier hazard, was acked in that state, and was
compressed in that state into the steering law implementing agents consume. The
repair is one sentence plus two DST pins (∅-backing unrepresentable /
⊤-never-∅) beside the two pins the clause already carries.

### finding-independence-collapsed-twice — promised-independent re-derivations resolved by in-context self-ratification

Two instances, same shape:

1. **task-14 / `271:rul-only-oracle-bytes-ship`.** The contemporaneous record
   (2026-07-11/12) is explicit: the human typed "I want to triple-check that hard law
   in a new session"; the ledger's execution shape specifies "clean-context subagents
   re-derive the law (root docs + `23O` settled law; **never shown the drafted
   repair**; exclusions-not-inclusions framing)"; and the 2026-07-12 conductor
   "self-flagged as CONTAMINATED for the fresh-check role." Four days later the
   ratification entry records the human "clarif[ying] 'triple-check' never meant an
   adversarial clean-room pass," dissolves task-14, and delivers the re-derivation
   *in-context* — by a conductor lineage that had ingested everything the
   contamination flag existed to keep out. The formal authority chain is intact (the
   ack is human-typed: "That's the design I understood and want"), and the law's
   *content* looks safety-sound to me (+SURE it is the safe direction: book bytes are
   unvouched; shipping them into the read-only lane would execute unvouched code;
   argv-through-the-author's-argparse is the right type-checker shape). But the
   promised independent eye never looked at a rule the ledger itself graded
   "hard-law status plus a test pin, never builder memory" — and the stakes rose
   after the promise was made: `27C` §3's entry composition ("only oracle bytes
   execute in the probe lane... the wrapper-oracle's entry form wrapping the inner
   oracle's body") now leans on this law for the shipped form of every entered
   context. If the law has a flaw, no clean context has ever had the chance to find
   it. The record of *why* the human's stated requirement relaxed exists only as the
   AI's paraphrase.
2. **`279f:ask-flag-boundary-recut`.** Queued as "a fresh-session re-examination of
   `rul-flag-is-razor-residue` before wrapper-sudo dispatches"; discharged as
   "[SUBSTANTIALLY DISCHARGED... `27C`'s escalation dial is the recut]" — i.e., by
   the same-context design dialogue, not a fresh session. Substantively this one
   mostly holds (flagging the fallback-lane transport genuinely answers `279a-F1`'s
   half; `279e-#1`'s half was priced consequence-light), so the damage is smaller —
   but the pattern is the point: both of the layer's promised independent
   re-examinations were resolved by the party they were to examine, recorded by that
   party.

I did not find a third instance. Recommend: any future "fresh-session /
triple-check / clean-room" commitment gets a slugged entry whose closure REQUIRES
naming who ran it and from what context — the ledger discipline already exists for
rulings; extend it to process commitments.

### finding-whyrun-fence-not-run — the pivot's consent argument cites precedents the corpus's own ledger brands broken, and skips its own mandatory fence

`24R` §0a (the why-run ledger) is the corpus's standing account of incumbent
dry-runs: Chef's first-party "why-run considered harmful" is "the canonical statement
that dry-run-over-shell fails in battle"; Ansible `--check` "silently skips shell
tasks"; Puppet `--noop` / Salt `test=True` "execute guards for real at agent
privilege"; summarized as "why-run made a promise nobody [could keep]" — with a
standing fence: "**flag-marketing-fence**: any future pitch touching
dry-run/plan-safety must pass §0a."

`27B` §3's consent reframing (observation 2) argues the opposite direction from the
same facts: "Every incumbent's dry-run already works this way... none of those tools
flags privileged dry-runs as a consent event. Dorc's unprivileged probe is the
ecosystem outlier." Three problems:

1. The Ansible half is contradicted by the corpus's own record: for the shell content
   that is Dorc's entire domain, check-mode does not run checks under become — it
   skips them. The precedent-claim was marked "~SUSPECT... worth a citation pass" in
   `27B` itself; no citation pass appears anywhere in the trail, and the claim
   graduated into the layer's consent story unverified.
2. The Puppet/Salt/Chef half is *true* — and is precisely the behaviour `24R` §0a
   classifies as the incumbents' structural failure. Citing the ecosystem's
   broken-promise norm as consent-normalization for the new default lane is an
   argument the corpus's own considered position indicts.
3. The fence was not run. Neither `27B`, nor `27C` §0's "security story in one
   sentence," nor the supersession banner (which corrects four *other* `27B`
   overclaims) cites `24R` §0a. LIVING_STATUS still lists `24R` as read-first
   material ("the why-run impossibility ledger") while the newest layer's consent
   argument stands un-checked against it.

To be plain about what this does NOT show: `27C`'s mechanism is defensibly *better*
than the precedents it cites — entered bodies are vouched, per-dimension,
dial-gated, where Puppet-noop executes arbitrary guards — so the design likely
*passes* §0a if run through it. The finding is that the layer's justification stands
on an argument its own fence exists to catch, and nobody ran the fence. Cheap repair:
one paragraph in `27C` §1 or §7 running the §0a comparison honestly (the differences
ARE flattering), and striking or citing-and-correcting the Ansible half of `27B`'s
observation via its banner.

### finding-stale-transport-rows-in-277 — the layer's own stale-text discipline, applied to `275`, skipped for `277`

`279a-A1` established the hazard class ("danger = a builder citing [a superseded
spec] in isolation"), and the layer's response was exemplary for `275`: banner,
per-step supersession comments, a §12 row annotation. The same discipline was not
applied to `277`, which Research/README designates "THE spec" for the entity algebra
and which the entity-algebra-rebuild brief consumes directly:

- `277` §2's consumer map still reads "same → transport (a fact established in one
  context licenses action about the other); **the probe-outside license**", and its
  generator-registry row for the axis-invariance line lists tier "vouch (kind-owner's
  typed line)" — with no mention that under `plans/27C` §4 the composed cross-context
  outcome rides ONLY under `--risk-faultless-skips`. A builder consuming §2 in
  isolation builds invariance-generated transport unflagged — the exact
  under-execution anatomy `27C` §4 fences, re-created by documentation.
- `277` §9's status table still carries "`275` ratifications... riding the human's
  adversarial pass" — they were refused as posed on 2026-07-13 and the territory
  re-answered by `27C` on 2026-07-16.
- `277` §8's "Riding the human's adversarial pass, by design: the `275`
  ratifications..." — same staleness.

The document was edited as late as 2026-07-16 (the §3/§5/§6 ack annotations landed
then), so the omission is not "nobody touched it since." `277` §0 does say `plans/`
outrank it on conflict, and newest-wins is corpus law — but `279a-A1` was credited
*despite* newest-wins resolving that instance too, for exactly this reason. Repair is
three annotations of the `275` §4/§12 pattern.

### finding-cascade-rescue-product-conflation — `27B`'s guard-cascade refutation rescues a different product than the one the severity claim was about

`27A` §1's whole-book-tier severity claim is attention-denominated: "once we've
showed the user a plan with the guarded commands included, that scarce resource of
user-attention is *spent*" (IMPLEMENTATION's own doctrine: "the full-elision,
user-attention-preserving behaviour *is* the product"). `27B` §2
finding-cascade-overstated refutes it with route-conditional-tail — which rescues
*execution* (zero steady-state check-tax, drift-safety on fired days) while its own
§4 render paragraph concedes the attention cost persists: conditional-tail lines
"render as guards, at most dimmed"; folding them "is a human product ruling; nothing
here presumes it." `27C` §5 inherits the framing ("residual guarded walls no longer
cost the whole tail" — true execution-wise, pending-at-best attention-wise) and the
supersession banner, which corrects four other `27B` overclaims (transport-not-dead;
the withdrawn privilege ordering; the wrapper-bytes phrasing; the demotion ordering),
does not correct this one.

Bounded consequence, stated fairly: the *primary* lane rescues attention wherever
entry succeeds, so the pivot does not rest on the conflation — but the residue
accounting does. For every fallback-lane cell (`hole-static-identity`, unvouched
oracles, unenterable dimensions), "conditional tails + generation-probes contain the
drifted-day cost" (`27C` §7) is an execution statement wearing containment language;
the attention product in those cells is simply gone, which the corpus's own doctrine
ranks as the product being gone. One honest sentence in `27C` §7 fixes it — and the
fold-or-not render ruling it defers should be tracked, since the fallback cells'
value-story quietly depends on it.

### finding-synthesis-epistemics-inverted — the layer violates the epistemic rule it announces

`279f` §8, arguing for build-dispatch: "further whole-package design passes have
demonstrably NOT settled [the open questions] (the transport chain survived five
sittings and was caught by adversaries reading built artifacts, i.e., by contact, not
by more synthesis). The marginal value of another design round is low and falling."
What followed, for the deepest finding: three more whole-package synthesis documents
(`27A` 07-13; `27B`+`27C` both 07-16), a same-day steering-law rewrite, and
next-morning build dispatch — with zero outside-lineage review of the replacement
mechanism, zero reality contact, the one falsifiable prediction the design stakes
(`27C:prediction-trial-walls-dissolve` — the `255` book's two walls dissolve on a
real host) scheduled several blocks downstream at field-trial revival, and
`279f:ask-thin-reality-checkpoint` (the cheap contact point) still parked as
"optional, human-taste," undecided, in LIVING_STATUS's non-blocking leftovers.

Mitigations, credited: `27B` was a human-directed *self-refutation* brief ("distrust
the corpus's own conclusions"), which is real adversarial process, and it worked —
the layer killed its own dead-end. The human typed the `27C` rulings, and the human's
own commits folded `27C` into USER_STORY. And the prediction genuinely cannot run
before some of the mechanism exists. But `27B` and `27C` are the same model lineage
as everything they replaced; the dialogue that corrected `27B` into `27C` was
in-context; and the block that consumes `27C` whole (block-context) has no gate
between it and the untested prediction. By the layer's own §8 epistemology, `27C` is
exactly the artifact-class in which confident wrongness concentrates, sitting exactly
where being wrong costs the most (the shipped probe form for every wrapped site).
Cheapest repair consistent with their own argument: make the thin-reality-checkpoint
a typed yes/no rather than an ambient maybe, and put
`27C:prediction-trial-walls-dissolve` in the block-context brief as a named
strike-condition, not a §8 footnote.

### observation-consent-rhetoric-inflation — "both-sides consent" and "three named consents" overstate the default cell

At the default dial, the admin's "side" of both-sides consent is the absence of a
flag, and the authority-disclosure line (`27C:render-authority-disclosure`) renders
in the plan header — i.e., after the probes it discloses have run. Both choices are
human-ruled and defensible (defaults are how every tool ships; the plan-promise was
always non-mutation-modulo-vouches, not non-execution); the finding is only that the
layer's *language* — "both-sides consent," "three named consents," "consent
legibility" — describes the opt-down cells accurately and the default cell
aspirationally, and the compression propagates the phrase into `spike/CLAUDE.md` and
`oracle/CLAUDE.md` where future doc-writers will inherit it as fact. Wording-tier;
flagged because this corpus is unusually careful about exactly this kind of
overclaim (`24R` §0a's "shortest honest form" discipline), and admin-facing honesty
is priority-3 territory.

## §2 — Cleared: attacks that do not land, and repairs that hold (verified, not presumed)

- **cleared-imp-one-provenance-audit-holds.** `27B`'s keystone claim — no root-doc
  sentence promises an unprivileged probe — verified by direct sweep of README /
  DESIGN / IMPLEMENTATION / USER_STORY at review-point: the probe promise is
  non-mutation-modulo-vouches ("vouched-safe-to-run / non-mutative"), and the
  never-escalate rule's provenance is AI-tier (`24S` §0 imp-1; `an-privilege-fact`),
  with `kFAIL-withhold` welding mutation, not privilege. The pivot's license is
  genuine. The frame-error diagnosis (a proposal-tier line hardening into a "standing
  invariant" across three documents) is also accurate as a description of what the
  record shows.
- **cleared-fix-spare-top-backing-repairs.** The amended `277` §3 predicate (sparing
  requires minted selectors on BOTH sides; ⊤ on either side collides) states exactly
  the missing half `279a-A5` identified, consistently with §1's ⊤-collide intent;
  the regression pin rides the rebuild brief. Complete repair (modulo the SET-tier
  hole reported above, which is `fix-set-lifting`'s, not this one's).
- **cleared-dialect-properties-now-true.** The reworded §3 properties hold as
  restated: cross-family monotonicity follows from dialect(family, kind) being
  per-family (a new family cannot alter comparisons against other families'
  backings); the within-family collide→spare flip is real and is honestly re-priced
  as declared, flag-gated kill-surface control rather than denied.
- **cleared-pipefail-narrowing-is-honest.** `276:rul-pipefail-emit-never` does not
  fully seal `279b-fd4`'s crack (an author's bare `set -o pipefail` still strips to
  floor-illegal text) — and the record says so itself: "'strip output is floor-legal'
  holds **for lint-clean text**... the floor test remains the conformance gate that
  catches an author who insists on the bare form," with the human's eyes-open
  grading on record. A priced narrowing, honestly annotated where it was minted.
  (Residue, trivial: the `279f` §6 ask-summary and KNOBS' weld prose lack the
  lint-clean qualifier that `276` carries.)
- **cleared-dismissals-survive.** Every dismissal I could re-derive against the
  texts stands. `279e-#6`: `v=$(false)` does trip errexit (POSIX: a command-less
  assignment takes the last command-substitution's status, and errexit fires on it) —
  and the genuinely dangerous neighbouring cell, `local v=$(cmd)` rc-masking, is
  independently covered by `276:rul-base-dialect-ruling-list` (permitted, analyzer
  treats rc-opaque, hints under `set -e`), so no hole is left either way.
  `279e-#5`: `273` §2's per-channel vocabulary plus the ambient silence-mints-⊤
  default cover the terminal-rc cases; "a clarifying clause could be added" is a fair
  summary. `279e-#7`: human-acked, eyes-open, on record in `276`. `279d-F5`: the
  `272` §3 amendment block is prominent in place (verified), and
  annotate-don't-rewrite is genuinely the corpus's convention. No manufactured
  dismissal found; notably the dismissals concentrate on the lanes the adjudication
  weighted lowest, and still all check out — the weighting did not visibly corrupt
  outcomes (the headline credit went to a foreign lane's finding).
- **cleared-measure-in-context-core-move.** The default lane consumes no completeness
  claim because nothing crosses a boundary: the fact is minted, keyed, and consumed
  in one context; the residual within-context trust is exactly the pre-existing `233`
  converged-vouch adequacy gap, correctly identified as untouched. The `27B` §3
  walls-compatibility list checks out item-by-item against `27A` §2 (in particular
  wall-measurement-reach's own text carves the escalated-probing exception the route
  occupies, and wall-agnosticism-homes is preserved — unmodeled wrappers still never
  peel, so unauthored entry cannot exist). The new mutation-blast trade
  (`27C:hole-bad-oracle-blast`) is disclosed rather than buried. The layer's deepest
  repair is, in shape, the right one — this review found its edges wrong, not its
  center.
- **cleared-compression-is-faithful.** `spike/CLAUDE.md` and the seven crate files
  were checked rule-by-rule against their cited sources for the new layer
  (context-entry, tolerates-vouch, only-oracle-bytes, sparing algebra, set-lifting,
  emit-never, two-binary floor, conditional tails): no divergence *introduced by
  compression* was found. The two defects the compression carries
  (finding-vacuous-set-lifting-hole, observation-consent-rhetoric-inflation) are
  inherited faithfully from their sources — which is the failure mode a compression
  cannot catch, and an argument for fixing sources first.
- **cleared-usercontent-root-doc-consistency.** The human's own 2026-07-16 root-doc
  refresh is consistent with the layer: USER_STORY's STATUS block routes wrapped
  sites to `plans/27C`, and its chezmoi story's "`sudo` lines still wall (honestly)"
  matches `27C`'s non-root residue exactly (a user-context run holds no shiftable
  authority). One forward obligation, currently untracked: `279f:ask-root-doc-queue`
  keyed USER_STORY's transport caveat to the ships-unflagged branch only; transport
  shipped *flagged* instead, which changes the FLAG's admin-facing definition — the
  moment the `27C` §4 fallback lane ships, USER_STORY's "past `--risk-faultless-skips`,
  you are trusting named authors' at-most claims" (line ~718) undersells the flag's
  scope (it will also gate kind-owners' invariance lines, a different claim species).
  Nothing queues that edit today.

## §3 — Minor nits (recorded so they are not re-found)

- nit-fallback-lane-cite-slip: `279f` §6's flag-boundary discharge annotation says
  "residual flag questions live in `27C` §5's fenced fallback lane" — the fallback
  lane is §4; §5 is guards/conditional-tails.
- nit-registry-resolved-wording: `279f`'s `279d-F2` row ("RESOLVED BY THIS PASS —
  the registry survives") converts crosscheck-survival into ratification language for
  a surface `271`'s closing sweep records as "un-acked-but-yolo'd." The human's yolo
  is on record, so no consequence — but "survives the crosscheck" and "ratified" are
  different states, and this corpus is elsewhere scrupulous about the difference.
- nit-275-regime-row-unannotated: `275` §3's "world-cell-backed... transportable
  across contexts per backing invariance (§6)" clause is live un-annotated text
  pointing into the refused-bannered §6; newest-wins plus the pointer saves it, but
  it is the last un-annotated transport sentence in that file.
- nit-27C-conditional-tails-attribution: `27C` §5 credits
  `27C:route-conditional-tail`; the route is `27B` §4's (`27B:route-conditional-tail`)
  — an artifact of the supersession, harmless, mildly history-obscuring.

## §4 — What this review did not do

No security analysis of any kind (threat models, hostile actors, attack surface,
privilege semantics) — per commission; the one privilege-adjacent finding is strictly
a two-sentences-disagree consistency item, and wrapper reasoning throughout was
worked on fs-view/environment examples (`chroot`, `mise exec`-class) wherever the
argument allowed. No entry into quarantined directories or `corpora/` — consequently
the critics' raw reports were not read, and dismissal-verification is re-derivation
against package texts, not adjudication-of-the-adjudication's-summaries. Code-level
verification was targeted (the survival footprint path, the `split_whitespace`
claim's neighborhood, HEAD's empty-emission posture), not a sweep; the spike is
declared throwaway and chains-of-logic were the commissioned target. The `278` DRAFT
reference and the r25/r26 dormant branches were not reviewed. Confidence markers:
findings §1 are each +SURE on the textual contradiction/omission they report and
~SUSPECT-at-most on consequence-sizing; cleared items §2 are +SURE on what was
directly verified and say so where verification was partial.
