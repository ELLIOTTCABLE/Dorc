# 28Ra — Context-kernel unification (28Q): neutral review, Fable lane

> Tier: adversarial-crosscheck NEUTRAL leg (Fable, clean context, 2026-08-01). Assesses
> `Research/plans/28Q-context-kernel-unification.md` for soundness, internal consistency,
> fit to the root docs and both user classes, and consistency with prior design law.
> Findings ordered by importance. Certainty marks per the house convention
> (+SURE / ~SUSPECT / -GUESS / --WONDER). Not a ruling surface; everything here is
> review-input for the conductor/human.

## §0 — Verdict in one paragraph

The three-pillar direction is sound and the plan is unusually well-disciplined: the
graded ack-ledger is accurate where I spot-checked it (supersession markers claimed in
§4 are really placed — `20V`:69, `28P` around `fnd-a-split-family-elides-on-two-authors`;
the cited invariants exist in the crate steering files), the staging separates the one
deliberate behavior change (stage-0) from byte-gated refactor stages, and the central P1
argument — factoring rows by DefinitionId makes the identity/cells chimera
*unrepresentable*, which cures the exact objection that forced
`28P:dec-the-gate-is-agreement-not-re-resolution` to choose withholding — is +SURE
correct as stated. The design also survives the exclusion-check I ran on P3's floor:
under an unreliable creator-oracle, availability only licenses *attempting* probe entry
and reality gates it dynamically, so both error directions land on can't-say ⇒ guard/run.
The problems I found are not direction-level; they are (1) one definitional ambiguity in
P2 with a fence-dissolving misreading available to builders, (2) a silently-dropped
safety property in stage-i whose named gate is vacuous exactly where the new machinery
acts, (3) a real conflict between the ACKED host-entry ruling and a *ruled* 27C entry-shape
law that the plan's "touching none of its consent machinery" framing papers over, and
(4) several "for free" claims that contradict the cited ledgers' own pricing.

## §1 — find-closure-rooting-dissolves-the-fence (P2's definition is lossily compressed)

Location: `28Q` §2 opening definition vs. `28Q` §2 "Diamond loading" bullet; source law
`28M` §10 `dir-ownership-is-transitive-inclusion` [TYPED].

§2's headline defines THE entry-closure as "the transitive closure of literal
`.`-sourcing from an entry file (a CLI-named positional, or the book)" and says custody
"becomes closure-membership... One identity, consumed everywhere 'owner' appears." Read
literally, a book that sources two strangers' oracle files (`. repoA/tools.sh` then
`. repoB/tools.sh`) puts both inside *the book's* entry-closure — one closure, one
speaker — and the committee fence (live role members spanning >1 closure ⇒
sparing-inert) can never fire for in-book-sourced strangers. That is precisely the
cross-author sparing configuration the fence exists to catch, and it would also mint a
spelling-dependent trust cliff: the same two files loaded as CLI positionals are two
closures (fenced), but loaded by book-sourcing they are one (unfenced).

I'm +SURE this is *not* the intended semantics: `28M` §10 [TYPED] is explicit that the
not-co-author machinery "binds only SIBLING/COUSIN edges in the include-tree — where the
authorized minting-site's entry is below/adjacent to another usage, never above it," and
`28Q` §2 itself repeats the sibling/cousin carve one paragraph later. But the two
statements in §2 contradict each other as written: "one identity per entry file" cannot
coexist with "sibling subtrees under one entry remain mutually fenced," because the
latter requires closure-identity to be *rooted per sourced subtree*, not per top entry.
The plan never states the rooting rule (what roots a custody unit; how the book's own
definitions key; what happens at a shared-grandchild diamond that dedups by bytes). A
builder implementing custody as membership-in-the-top-entry's-closure implements the
fence-dissolving reading and every stage-ii gate still passes, because the gates only
re-verify single-closure worlds plus the six stage-E cells.

Ask: one paragraph in §2 pinning closure-identity as a *function* (which node roots the
unit; sibling edges fence, ancestor edges take custody), plus one stage-ii fence cell for
the book-sources-two-strangers shape. Mitigating today: top-level book `.` still walls
(`28P:res-book-sourcing-wall-gates-this-item's-payoff`), so the hole only opens when the
§2 payoff-gate dot-blessing lands — which is exactly when nobody will be re-reading this
plan's fence text.

## §2 — find-resolve-regime-loses-the-withhold-safety (stage-i's gate is vacuous where the machinery acts)

Location: `28Q` §1 (the factoring + "the veto retires"; the `never_live` "SUBSUMED"
bullet); `28Q` §8 stage-i gate; source law `28P:dec-the-gate-is-agreement-not-re-resolution`
and `28P:adj-never-live-exactness-accepted`.

`28P`'s (B)-withhold choice had *two* independent virtues: it made the chimera
impossible, and it made every resolution-machinery bug fail in the value-losing
direction ("withholding is the one direction that cannot widen a license"). The
factoring genuinely retires the first reason (+SURE — one DefinitionId anchors identity
and cells, so option (A)'s pope-sin premise is gone). The plan is silent about
surrendering the second: under true frame-keyed resolution, a funcenv bug — a wrong SCCP
fold decision, a wrong subshell-scope boundary, a wrong `unset -f` window — now directly
*selects whose judgment governs a site*, with no agreement veto behind it. This is the
same character `28P:adj-never-live-exactness-accepted` flagged for the never-live
subtraction ("removal SHIFTS the winner... a wrong answer here grants a license rather
than losing one"), whose standing consequence ("decidable-set growth is
license-review-tier, never a convenience patch") was stamped into `analysis/CLAUDE.md`.
`28Q` §1's wording — the exact-subtraction hazard "dissolves into the general mechanism"
— reads as the hazard going away; in fact the general mechanism *inherits* the
winner-shifting character at every seat. ~SUSPECT the wording alone would mislead a
stage-i builder into treating funcenv precision work as ordinary value-add rather than
license-surface work; the standing consequence needs restating for the frame solver as
a whole, not deleting with the subtraction seat.

The verification gap that makes this matter: the stage-i gate is
`syn-single-frame-byte-identical` — but the corpus is single-definition and
define-before-use (stage G's respell), so in every gated world the frame lookup and the
old ambient answer coincide and byte-identity is *trivially* satisfied; it exercises
nothing plural. The worlds where the new machinery actually decides something (blessed
override above `unset -f`, subshell re-source, polyfill orders) arrive only as "new
corpus cells, not churn" — authored by the same lane that builds the feature, i.e. not
an independent check. The independent ground truth already exists: bitem8's
differential sentinel-body battery under the real two-binary floor
(`28P` bitem8, LANDED). Ask: stage-i's brief should commission the plural idioms as
*differential* cells (sentinel bodies; dash∩posh agreement with the analyzer's frame
answer), and say so in §8, so the flip from withhold-to-resolve lands with an
adversarial net rather than a vacuous one. `pattern-carry-the-answer`
(wrong-but-consistent) remains the backstop, but it only bounds the damage to
consistent-wrongness; it does not detect it.

## §3 — find-ssh-entry-conflicts-with-ruled-entry-shape (and stage-iii's second gate)

Location: `28Q` §3 first bullet (`rul-host-entry-is-ordinary-entry` + the re-parse
rider); conflicting law `27C` §3/§10 — `"$@"` verbatim in command position is "the ONLY
entry shape," the in-guest-preamble variant explicitly PUNTED (human-acked 2026-07-17).

The plan's header claims §3 generalizes `27C`'s availability behavior "touching none of
its consent machinery." True for consent — but the ssh rider touches a *ruled shape law*
instead. An ssh entry form cannot be `"$@"`-verbatim and survive the remote-shell
re-parse: ssh joins argv with spaces and hands the string to the login shell, so argv
boundaries are destroyed for any operand with metacharacters — the exact
measured-in-the-wrong-world/wrong-argv class the siting vouch exists to refuse. The plan
knows the obligation exists (it names the re-parse round-trip as the entry-siting vouch
discharge, and notes the engine's own transport ships on stdin per `260` §5), but a
stdin-shipping or quoting-wrapping ssh entry form is structurally the punted in-guest
preamble variant. So `rul-host-entry-is-ordinary-entry` [ACKED] quietly requires either
un-punting that 27C decision or minting ssh a carved-out entry shape — a real conflict
with a ruled law that the plan should name in §6/§7 rather than leaving to be
rediscovered at the stage-iii brief. (-GUESS on resolution: the carve is probably right —
ssh's entry form is closer to the engine's own transport than to sudo's exec — but that
is a ruling, not an implementation detail.)

Related sizing honesty: stage-iii's ssh-as-entry also needs the ssh oracle
(`ack-connection-dance-oracles-core`: "the ssh oracle's own arms mint reachability
facts"), and oracle authoring "rides the stdlib revival" which is itself gated on the
dialect-reach decision (§9 pin 3, `28O:fnd-dialect-tests-admit-only-string-comparison`).
So stage-iii sits behind *two* unscheduled design acts — the reserved §10 surface (named
in §8) and the stdlib revival (named only for the `command` oracle). Worth one sentence
in §8 so the stage's horizon reads honestly.

## §4 — find-for-free-claims-contradict-the-cited-ledgers

Location: `28Q` §1 consequences bullet ("the whyworld/survival seats... both are
oracle-only-vector coincidences that the one-lookup design deletes"; "The seventh seat
... unifies with the other six for free").

Two overclaims, both against `28P`'s own text:

- The whyworld/survival unification was *priced* by 28P and declined as out-of-scope:
  "Full unification was priced and declined: it means re-lifting that seat's whole
  world, which is a dispatch and not a rename"
  (`28P:res-the-why-world-cut-is-now-visible`; likewise
  `res-whyworld-and-survival-do-not-withdraw` — those seats build their *own* lifted
  sets from the oracle-only vector). Deleting the coincidence means threading the
  source-wide vector and the frame map through `WhyWorld` and the survival lifts — real
  work the plan absorbs into stage-i without a line item, while §7 simultaneously names
  stage-i's touch-count as the expense center. Not a soundness issue (both seats
  currently fail toward withhold); a sizing-honesty issue in the exact place the plan
  claims to be sizing.
- `tc-wrapped-lane-drops-a-case-bodied-in-book-verdict` was measured and explicitly NOT
  diagnosed ("~SUSPECT it is the same res-why-world-lifts-no-book-definitions family...
  NOT diagnosed further" — `28P`). `28Q` asserts the cause ("oracle-only-vector
  coincidences") as fact. If the assertion is wrong, stage-i lands, the coverage hole
  persists, and nothing trips: the loss is value-shaped (site runs), invisible to
  byte-identity and to outcome gates. Cheap fix: carry the case-bodied in-book wrapped
  fixture as a stage-i *expected-to-flip* cell, so the asserted cause is tested rather
  than trusted.

(The "seventh seat" count vs `28P`'s "SIXTH seat" is counting-frame drift, not an
error — noted only so nobody burns time reconciling it.)

## §5 — find-frame-relative-dialect-unspecified (the P1×P2×survival corner)

Location: `28Q` §2 committee-fence bullet ("the fence becomes frame-relative under P1")
+ §5 ("closure-keyed dialects are its eventual hook"); consumers `an-selector-dialect`,
`an-backing-set-meet`.

The sparing comparison is inherently *two-position*: a wall's at-most claim minted at
plan position p is intersected against a downstream fact's backing minted at position q,
and the dialect consulted is "the backing family's." Once liveness, custody, and
(eventually) dialects are frame-relative, the plan never says which position's
environment governs that comparison when p and q sit in different regions (e.g., the
fact minted inside a subshell re-source region, the wall outside it — the family
single-closure at q, plural at p). Every resolution the plan does spell is per-site;
the sparing meet is the one consumer that reads *across* sites, and it is exactly the
naked-trust tier. ~SUSPECT the practically-right answer is "collide unless both
positions agree on the family's closure and dialect" (conservative, cheap), but it needs
one ruled line before stage-ii, or the fence's "frame-relative... correct reading of
'live members'" sentence will get implemented per-seat by whoever reaches it first.
Bites only in plural worlds under the flag — which per §1 above is also where nothing
gates.

## §6 — find-time-slot-citation-misread (272 §6's "time" is the Linux time namespace)

Location: `28Q` §3 facts bullet: "the lifetime axis consumes `272` §6's reserved time
slot."

`272` §6's axis roadmap row reads "pidns / utsns / ipc / cgroup / time — representable
in the substrate frame; rare-tail; rows land if/when their axes ever do." That list is
the Linux kernel namespace inventory; its "time" entry is the time *namespace* (timens —
per-container CLOCK_MONOTONIC/BOOTTIME offsets), a sibling of pidns and cgroup, not a
temporal/lifetime concept. +SURE of the list's reading; the plan's sentence spends that
reserved slot on a semantically different axis. Mechanically harmless today (the
`undivided-by-transit-across <axis>` mark grammar carries any token), but it conflates
namespace-crossing with lifetime-crossing in the one place the crossing vocabulary is
being fixed "for this territory," and it forecloses the timens row's name. Cheap fix:
mint the lifetime axis as a new axis value and leave `272` §6's row alone.

## §7 — find-wait-transparency-derivation-is-compressed

Location: `28Q` §3 lifecycle bullet: "wait-loops with pure-delay bodies mint no events ⇒
wall-transparency derives."

As compressed, the inference runs absence-of-events ⇒ transparent — which is the inverse
of `silence-licenses-nothing` (an *unmodeled* body also "mints no events," and must
wall). The sound derivation routes through positively-modeled purity: the loop body is
transparent iff its commands are modeled event-free/Pure (a stdlib `sleep`
verdict/predict, a modeled condition probe), and anything unmodeled in the body keeps
today's wall. I'm +SURE the sitting meant the latter (the `26K` ratification language was
"pure-delay body + modeled condition"), but the plan's one-line form is the one a
stage-iii builder will implement from. One clause ("modeled-pure bodies; an unmodeled
body walls as ever") closes it.

## §8 — find-cross-host-entry-consent-disclosure (UX, not security)

Location: `28Q` §3 entry bullet + residual-host-specialness bullet.

Uniform entry means the probe phase now opens connections to hosts *named inside books*,
beyond the CLI-named target — a materially larger observable footprint than sudo/chroot
entry on the target itself. The hostile-host/custody half is properly fenced to the
security lane, but the *consent legibility* half is this plan's UX territory and goes
unmentioned: `27C:render-authority-disclosure` discloses contexts entered under what
authority; it needs a host coordinate ("probe will connect: web1 (target), db2 (line
14, ssh)") before stage-iii, or the first admin whose book mentions a production host
discovers the widened footprint from their auth logs. Fits priority-3 (attention is
trust); one line in §3 or the stage-iii gate suffices.

## §9 — find-stage-shape-and-outstanding-notes (smaller items, batched)

- **note-stage-zero-lost-measurements**: at a vouched site the predict stops shipping;
  a multi-cell predict's *other* minted cells stop being measured there. The stage-0
  gate pins site outcomes but expects "probe-artifact bytes/records" churn — so a
  vanished fact-record is inside the expected-churn envelope and won't trip anything.
  Ask: the stage-0 fold enumerates lost-measurement cells explicitly (a records-diff,
  not just an outcomes-diff), since backing sets and why-surfaces consume those records.
- **note-user-story-guard-anatomy-wording**: USER_STORY stage 2 says "The predict body
  is the oracle author's own sh... the same bytes the probe phase already ran."
  Stage-0 makes the *bytes-agree* half more true (verdict ships in both lanes), but the
  "predict body" naming becomes wrong at vouched sites. Human-edit-someday candidate;
  the plan lists USER_STORY edits for other rulings but not this one.
- **note-survival-closure-lanes-absent-from-ledger**: `28P:res-survival-lanes-still-
  ship-closure-less` (helpers don't travel with `disturbs`/`resolve`/`reaches` bodies;
  `cli/CLAUDE.md one-helper-index-two-lanes` names the cheap extension) is absent from
  §5's anti-piecemeal ledger, though P2 makes closures the custody substrate and the
  atomicity gates only contain the failure to value-loss. Name it in (stage-ii rider) or
  name it out; an anti-piecemeal ledger that omits a named sibling item invites exactly
  the piecemeal it forbids.
- **note-host-conditional-frames-vs-literal-plane**: §5 floats the eventual
  `res-host-conditional-loading` story as "per-host frames keyed by decidable host
  facts" while §1 preserves `funcenv-reads-source-literal-plane-only` ("probe-provenance
  values never site a load decision"). Any per-host frame keying is a deliberate
  amendment of that law, not an extension; the eventual-story sentence should say so to
  keep the law's edge crisp.
- **note-two-planes-weld-is-vocabulary-not-mechanism**: `syn-one-context-two-planes`
  unifies *discipline*, and §1/§3 correctly keep the mechanisms separate (frames are
  ProgramText-graded; availability consumes probe reachability). Worth guarding: the
  "ONE discipline" framing is exactly the kind of sentence a later lane reads as a
  mandate to share implementation, and the load plane must never grow a probe-data
  input. A single fence sentence in §6 would immunize it.
- **note-availability-has-better-prior-art-than-claimed**: the plan grounds
  "incarnation" well (TCP RFC 9293 — correct usage) but misses that "availability" is
  the project's own strongest precedent: available-expressions/PRE (README's lazy-code-
  motion framing) is literally piecewise truth over program points with kill-events —
  the world plane is available-expressions over world state. Citing it would strengthen
  the no-coinage story and give stage-iii builders the right intuition for free.
- **note-multi-phasic-shape-tension**: four stages, the last design-gated on a reserved
  sitting (§10) plus the stdlib revival (§3 above), under a standing anti-piecemeal
  order. AGENTS warns that large multi-phasic plans have been a curse here. Mitigation
  is real — stages 0–ii are fully specified, independently gated, and independently
  valuable; P3's mechanics are honestly deferred rather than hallucinated — so I read
  this as acceptable-but-watch: the risk is not the plan's structure but the standing
  order ("no more piecemeal work on this territory") turning the reserved §10 sitting
  into a bottleneck that pressure will route around.

## §10 — What holds up well (verified, not vibes)

- The P1 factoring argument is the strongest piece: it addresses
  `dec-the-gate-is-agreement-not-re-resolution`'s objection at its premise (merged
  whole-unit indices), not at its conclusion, and the one-indirection design reuses the
  seam bitem0/2/3 actually built (`answers_at`/`source_of`/custody are in the tree —
  verified in `28P`). "Structurally chimera-free" is the correct claim.
- Stage-0-first is the right discipline: the one behavior inversion is taken *before*
  the byte-gated refactor, in the old world, with an outcomes-stable gate — so the
  refactor stages never smuggle a behavior change. The inversion itself is well-founded:
  `28P:fnd-a-split-family-elides-on-two-authors` measured a two-author license, and
  verdict-primacy restores the monologue at the license tier while leaving the
  cross-author residue to the fence sitting where it belongs.
- P3's floor survives the four-cell exclusion-check: wrong-converged oracle ⇒ entry
  attempted-or-skipped, both land can't-say ⇒ guard/run; arriving-context guards sound
  in-sequence; destroyed-context sites run and fail as the book's own bug; unmodeled
  creators wall. Availability licenses *attempting*, reality gates — the kFAIL phase
  posture is preserved by construction.
- The honesty artifacts are genuinely honest: the unconditional destroy-recreate
  admission (no downstream convergence until the correlation door is designed), the
  `an-atmost-completion-signal` §9.8 listing "so nobody reads §3's atomicity inheritance
  as closing it," and the §6 preserved-invariant wall all check out against their
  sources.
- The vocabulary retirement (epoch/pivot/transit → incarnation/lifecycle/availability)
  discharges `26K` §0b's typed terminology-unification rider, including its "prefer
  standard terms over coinages" lean — the load-plane claim ("scoped environments over
  a program order; no coinage needed") is accurate, and the one miss is the uncited PRE
  precedent noted above.

## §11 — Asks, collected (for the conductor's convenience; all argued above)

1. §2: pin the closure-rooting rule + a book-sources-two-strangers fence cell (§1 here).
2. §8 stage-i: commission the plural idioms as *differential* sentinel cells; restate
   the winner-shifting/license-review-tier consequence for the whole frame solver (§2).
3. §6/§7: name the ssh-entry vs `"$@"`-only-entry-shape conflict as a ruling owed; add
   the stdlib-revival gate to stage-iii's stated dependencies (§3).
4. §1: re-price the whyworld/survival unification against `28P`'s dispatch sizing; carry
   the case-bodied wrapped-verdict fixture as an expected-to-flip stage-i cell (§4).
5. Pre-stage-ii: one ruled line on which position's closure/dialect governs the
   two-position sparing comparison (§5).
6. §3: re-cite the lifetime axis as a new axis value; leave `272` §6's timens row (§6).
7. §3: add "modeled-pure bodies" to the wait-loop transparency sentence (§7).
8. §3/stage-iii gate: host coordinate in the authority-disclosure line (§8).
9. Batched small items per §9 (records-diff at stage-0 fold; USER_STORY wording;
   survival-closure lanes into or out of the §5 ledger; the literal-plane law-change
   sentence; the two-planes fence sentence).
