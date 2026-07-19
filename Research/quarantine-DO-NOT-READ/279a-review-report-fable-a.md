# 279a — Adversarial review of the 270-era block-settle corpus (Fable, review-a)

Reviewer: Fable conductor, clean-context adversarial pass commissioned 2026-07-13 against
the review-point `9431ccb9` (branch `ai/spike3-r23-fable-review-a`). Charge: find where the
270/271/272–278 arc is **wrong** — self-contradictory, unsound, or simply a bad call — not
where it is ugly. Kill it if it deserves killing; concede plainly where an attack does not land.

Authority order honored throughout: root human docs (README, DESIGN, IMPLEMENTATION) and
human-managed docs (USER_STORY, KNOBS, TODO/-ADDTL) outrank the corpus under review; within
`Research/`, newer supersedes older. Two hard exclusions respected: (1) nothing
security-flavoured — no threat models, hostile hosts, secrets, or privilege-as-attack-surface;
where a scenario needed a context boundary I recast it onto the team's own de-privileged
version-manager worlds (`nvm`/`mise`) rather than `sudo`. (2) No entry into
`quarantine-DO-NOT-READ` or `corpora`.

**Structure.** Part 1 is my own independent reasoning, written and committed BEFORE reading any
subagent output (the human's directive: an unspoilt pass grounded in doc-reads and reasoning, not
in delegated conclusions). Part 2 (appended later) adjudicates that pass against the subagent and
foreign-model lanes, promoting, demoting, or killing each finding.

---

# Part 1 — Independent pass (pre-subagent; unspoilt)

Confidence tags per the house convention: +SURE / ~SUSPECT / -GUESS / --WONDER.

## Finding F1 — the razor's flag-boundary is drawn at *pinnability*, not blast-radius (headline)

**Severity: design-judgment challenge to a last-day ruling. ~SUSPECT the boundary is mis-drawn;
+SURE the contradiction with USER_STORY's stated framing is real.**

### The claim under attack
`271:rul-flag-is-razor-residue` (TYPED 2026-07-12): *"Claims own what lines can say; the flag owns
what no line can say. The admin-consent flag gates exactly the outcome whose wrongness cannot,
even in principle, be pinned to a line that says the false thing — the open-world at-most residue
… i.e. the survive tier … Everything line-sayable rides the vouch economy un-flagged."* The
admin's single consent object (`--risk-faultless-skips`, `271:rul-flag-named-risk-faultless-skips`)
therefore covers ONLY the frame-problem residue (a `disturbs`-claim's unsayable "and nothing else").

### The contradiction it walks into
USER_STORY sells the un-flagged tiers on an explicit locality promise. The "bought unsoundness"
section (USER_STORY:633–693) and stage 3 (USER_STORY:118–126, 505–516) repeatedly reassure the
admin that a wrong un-flagged vouch bites *locally*:

> "stage 3's vouch, when wrong, endangers its own author's own tool's line, and that price was
> stated where it was bought." (USER_STORY:638–640)

But three un-flagged, line-sayable mechanisms minted/settled in the 27x arc have **non-local**
blast radius — a wrong claim under-executes *someone else's* unguarded command:

1. **Cross-context transport via a wrong `invariant:<axis>` line** (`272` §3 invariant outcome;
   `277` §2 consumer map: `same → transport … never survival`; `275` §6 value transport). Transport
   licenses eliding a command in one context on a fact measured in another — and it is routed to
   the *transport* consumer, which `277` §2 explicitly marks **un-flagged** (only
   `provably-disjoint → survival` is flag-gated).
2. **`kind__disturbance_reaches_only()` missing an edge** (USER_STORY stage 7:582–615): a
   downstream file-fact survives a wall because the reach declaration under-covered — under-execution
   attributed to the kind-owner, not the admin, and (per rul-flag-is-razor-residue, being
   line-sayable) un-flagged.
3. **`kind__resolve()` over/under-merge** (USER_STORY stage 6:537–575).

### Worked instance that SURVIVES scrutiny (the invariance transport)
Recast onto the team's own de-privileged world (`271:thread-nvm-not-sudo-examples`: "the
wrong-world-check class is not about privilege — `nvm`/`mise` replicate it with no user and no
root"):

```sh
# dorc-lang/v0.1 — a kind-owner honestly but WRONGLY declares a version-manager-keyed store invariant
sm_dorc_NodeGlobal__state_stored_only_in() {
   printf '%s\n' "$(npm root -g)"      : fs
   :                                   : invariant:user     # WRONG: npm -g root is nvm-world-keyed
}
```

Admin book line (unguarded, ordinary): `npm install -g typescript`. The probe runs in the
analyzer-invoking world (call it world-A, `imp-1`: probes never escalate/re-context); it measures
`typescript` present in world-A's global root; the `invariant:user` line launders that measurement
to "same for every world" → the fact transports → `npm install -g typescript` **elides**. But the
book's intended target world (world-B — a different active `nvm`/`mise` node) has no typescript.
Under-execution. No wall, no `--risk-faultless-skips`, no admin opt-in of any kind — licensed
purely by a *distant kind-owner's* honest-but-wrong `invariant:user` line.

`imp-1` is load-bearing to the break: because the probe cannot measure world-B, its wrong-world
measurement is the ONLY measurement, and the invariance claim is what makes it stand in for
world-B. The command's own `is_converged` cannot save it (it too runs in world-A). This is exactly
the "pipx-in-`~/.local`" knife the corpus already names (`272` §8) — I am not claiming it is
undiscovered. I am claiming its **fencing is wrong**: it under-executes an admin's unguarded line
with non-local blast radius (a *different* author's wrong line, in a *different* file), yet the
design leaves it un-flagged while flag-gating the structurally-identical survival tier.

### Why "attributed" ≠ "adequately fenced"
The razor's whole defense (`271:rul-invariance-speech-act`, `thread-razor-unsoftened-worked`) is
that these bites are *attributable to a typed line*, therefore vouch-tier, therefore acceptable
un-flagged. The human even acked the honest price: *"bite-rate-monotonic … it doesn't reduce fire
frequency; it buys attribution."* But attribution is a **post-mortem** good — it tells you whose
line to fix *after* production silently didn't get its package. The survival tier has the identical
"attributed-by-name" property (USER_STORY:513–516 — "attributed by name in every elision it
licenses") and the design flagged it anyway, on the grounds that non-local, no-runtime-net
under-execution deserves explicit admin consent. Transport and reach have the same anatomy
(non-local, no runtime net, someone-else's claim) and were NOT flagged. The distinguishing line
the design drew — *pinnability* — does not track the thing that actually warrants consent, which
is **blast-radius × absence-of-runtime-net**. A line being sayable does not make its wrongness the
admin's to have foreseen.

### The honest caveats (where this softens)
- +SURE the design *considered and deliberately chose* this boundary (`thread-razor-unsoftened-worked`
  weighed the fallback "honest-walls-for-worlds" = no cross-context transport at v1, and rejected it
  to keep the `24S` §2 headline). So this is a challenge to a *reasoned* choice, not an unnoticed bug.
- My sharpest *plain-elision* version of this attack **DISSOLVES**: a wrong `resolve()` over-merge
  does not under-execute in plain elision, because the merged command's own `is_converged` re-probes
  its own entity directly and catches the divergence. resolve/reach/transport only bite where a fact
  stands in for a site that is NOT independently re-probed — i.e. survival (flagged) and cross-context
  transport (un-flagged, via `imp-1`). So the finding narrows to precisely the transport/invariance
  and reach consumers, not "the whole un-flagged core." USER_STORY stage 6's "a resolver that wrongly
  MERGES only over-verifies" is therefore *correct for the consumers it analyzed* (footprint/survival)
  — it simply never analyzed the transport consumer, where the safety of merge-vs-split is not
  re-derived.
- The admin does see the elided line in the plan with its reason (`rul-attention-honesty`), so it is
  *disclosable*, not hidden. But the plan reason will read "converged / survives (footprint disjoint)"
  or a transport note — it will NOT read "trusting a stranger's invariance claim you never consented
  to," which is the disclosure that would actually inform the choice.

**Disposition I'll defend:** the flag boundary should be re-cut at *non-local under-execution*
(a wrong claim skipping a line whose author is not the claim's author), not at *pinnability*. Either
those transport/reach outcomes join the flag, or the "cheap honesty / stays-local" language in
USER_STORY must be retracted for them. As it stands the human-managed root-narrative and the
last-day ruling disagree about where the naked trust lives.

## Finding F2 — `pipefail` reintroduces verdict nondeterminism on the `| grep -q` idiom

**Severity: material, unaddressed, in the last-day language sitting. +SURE the shell race is real;
~SUSPECT the corpus never accounts for the run-to-run flap (only the static 141-in-sink).**

`276:rul-pipefail-four-lanes` (`-dialect: IN`) makes `set -o pipefail` legal and the analyzer model
pipeline-rc first-class, on the argument that *"the verdict lane WANTS pipefail"*: without it a masked
upstream crash in `foo | grep -q x` mints a clean licensing 0/1 (unsafe); with it the crash surfaces
in rc → the flat `≥2` sink → can't-say → run (safe). The doc acknowledges one wart: *"the SIGPIPE wart
(early-exit consumer → rc 141) lands in the flat ≥2 sink ⇒ can't-say ⇒ run"* and adds a stdlib
quality-bar rider: *"prefer full-read forms (`grep x >/dev/null`) over early-exit `-q` where the
producer minds SIGPIPE."*

The corpus treats 141 as a *static* property of a pipeline. It is not — it is a **race**:

- `producer | grep -q pattern`. `grep -q` exits 0 on the FIRST match. If the producer has more to
  write than fits the pipe buffer (~64 KiB on Linux) and the match occurs before the producer
  finishes, the producer's next write hits EPIPE/SIGPIPE → the producer dies with 141 → under
  pipefail the pipeline rc is 141. If the producer finishes first (small output, or match at the very
  end), rc is 0.
- Same host, same converged state, +SURE **coin-flip** verdict: 0 (→ elide) vs 141 (→ `≥2` → run).

Two consequences the corpus does not close:

1. **Plan flap directly attacks the flagship.** The primary product is *attention* (DESIGN priorities
   3 & 5; USER_STORY throughout). A verdict that flips run↔elide across identical runs makes
   `dorc plan` output churn with no world change — the plan is no longer "a function of the probed
   world" (USER_STORY:138) but a function of a scheduling race. Worse, it collides with the
   *`dorc plan --exit-code`* contract that 276 itself builds: `276:rul-verdicts-never-stable`'s rider
   insists the exit code gate *"divergence-of-world, never plan shape."* A SIGPIPE-race verdict flip
   IS a plan-shape flip that the exit code will report as world-divergence — the very thing the
   contract promises it will not do. The two 276 rulings undercut each other.
2. **The quality-bar rider cannot reach the material it needs to.** The rider is a *stdlib oracle
   author* instruction. But the design's Half-B / anti-silo headline (`KNOBS:kHALVES-elide-half`,
   `KNOBS:kSILO`; USER_STORY stage 1:118–126, "the admin's own hand-written idempotence guard … is
   exactly the shape the analyzer lifts") makes the *admin's own* `| grep -q` guards first-class
   lifted verdict material — and `273` §7's changed-detection flagship is literally
   `sudo a2enmod ssl | grep -q 'already enabled' || …`. `grep -q`-piped convergence checks are the
   single most idiomatic form in real ops shell (`systemctl status | grep -q`, `docker ps | grep -q`,
   `dpkg -l | grep -q`, `ip route | grep -q` — several of which routinely exceed 64 KiB). The rider
   does not, and cannot, reach admin books.

Direction of failure is safe (→ run, never under-execute), so this is **not** a soundness bug. It is
a value/coherence bug that lands squarely on the two headline surfaces (attention-stability, drift
monitoring) and on the internal consistency of the language sitting. The bite is *scoped* (needs
producer output > pipe buffer AND early match AND producer minds SIGPIPE) but nondeterministic
heisenbugs are maximally corrosive to *trust*, which DESIGN priority 3 makes explicitly
subordinate-only-to-correctness. [Empirical rc distribution being confirmed by a verifier lane;
result folded in Part 2.]

## Finding F3 — the "replace"/changed-detection-fold breaches byte-for-byte plan-honesty

**Severity: latent breach in unstamped design; +SURE the collision exists; ~SUSPECT it is
under-flagged given it touches a human-written promise.**

DESIGN's plan-honesty promise is explicit and human-written:

> "it still includes all the same lines (byte-for-byte, where they're present at all). All *we* do is
> either remove lines entirely … or *additively* guard them" (DESIGN:243–248)

`spike/CLAUDE.md rul-ternary-verdict` restates it as an invariant: *"the original command's bytes
always survive verbatim (no code path removes them)."* Two operations only: remove, or additively
guard.

`273` §7 (the replace tier / changed-detection fold) introduces a **third**: *"the guard-fold
substitutes the mutative head with its read-only predictor inside the admin's own pipeline."* The
flagship is `sudo a2enmod ssl | grep -q 'already enabled' || sudo systemctl restart apache2` — the
mutative `a2enmod` head is *replaced* by its `predict` body so the convergence signal computes
read-only, eliding the restart. That is machine-composed bytes standing in a position the admin
authored, inside a pipeline that still runs — neither "remove" nor "additive guard," and not
"byte-for-byte."

The corpus does name a "replace" mechanism (`KNOBS:` Named-mechanisms: *"elide / replace — an
observable-preserving replacement of a proven-converged command"*; AGENTS.md terminology). But KNOBS'
`replace` is elision's degenerate cousin — substituting a *whole converged command* with an
observable-preserving stand-in (typically `true`, or reproducing captured values for a consumer). 273
§7's fold is different in kind: it splices a predictor *into a surviving pipeline* to manufacture a
convergence signal the admin's own `grep` then consumes. Whether the *rendered* plan shows original
bytes while the *shipped artifact* shows substituted bytes (per the `rec-1` two-surfaces split,
`spike/CLAUDE.md`), something the admin did not write executes in place of something they did — and
DESIGN's deeper promise ("there is always an original command, as-human-written, before we took any
action"; IMPLEMENTATION:364–371) is strained even though the predictor fragments do have an author.

Mitigations the corpus offers, fairly: 273 §12 marks the whole tier *"direction; scope call (v1 vs
ladder-reserve) unmade"*, and `274` §12 notes *"payload-v1 introduces ZERO new observable transforms."*
So this is a **latent** breach, not a shipped one. But it is unremarked *as a breach* in 273 — the doc
argues the fold's value without noting it crosses a human-written line — and the KNOBS `replace`
mechanism it leans on is in quiet tension with the higher-authority DESIGN. This is exactly the class
of thing that "gets built because the corpus already blessed the vocabulary."

## Finding F4 — the round-ordering bakes the highest-lock-in design before any outside push-back

**Severity: structural / process. +SURE the ordering is as described; the critique is a judgment,
partially exposed to the human's own YOLO-measurement stance.**

`270` sequences the round **block-settle → block-rebuild → block-context → block-stdlib**, then the
tabled field trial (né r25), then multi-host (né r26). block-settle + block-context are precisely the
highest-complexity, highest-lock-in machinery — the entity algebra, selector dialects, wrapper
`lend_map`, value-prediction species, the reentry token — and by USER_STORY's own reckoning they serve
the *rarest* cohort: *"Stages 6 and 7 … Fewer than one author in ten will ever write one"*
(USER_STORY:528). The only two mechanisms that put the design in contact with anything outside its own
corpus — the field trial and any corpus measurement — are gated **behind** all of it (`270` §5).

The stated justification is lock-in economy (`270` §0: "so the oracle stdlib is authored exactly once,
against the final entity algebra, the final dialect, and the real sudo mechanism"). But that economy is
**circular**: "author once against final shapes" only saves work if the shapes are *right*, and the
shapes in 272–278 are derived almost entirely by self-citation — every `271:` ruling cites other `271:`
/ `272`–`277` rulings; the graded external prior-art that does exist (`kwhichsh-gcd`, the versioning
round) is confined to the language floor and MH2, not to the entity-algebra/wrapper/capture core that
block-settle actually locked. If those shapes are wrong, "author the 40 oracles once" just means
authoring them once against wrong shapes, then re-authoring — the lock-in argument for doing this first
evaporates.

I concede this is partly the deliberate YOLO the human has already ruled acceptable (measurement
declined; PMF not chased). The distinct, still-standing point is narrower: the *lock-in* rationale used
to order block-settle first presupposes correctness it cannot have without the reality-contact it defers.
An equally-defensible ordering — a thin field trial on the stage-1–4 core *before* committing the
stage-5–7 algebra — would have let outside evidence shape the retrofit-hostile decisions while they were
still cheap. The chosen ordering is the echo chamber made operational: it structurally guarantees the
hardest-to-unbake design is baked before anything that isn't the corpus can object.

## Finding F5 — the attention product structurally anti-correlates with need (supporting)

**Severity: real tension, design half-owns it; underweighted, not novel. ~SUSPECT worth surfacing.**

Attention — the flagship — is saved ONLY by full elision (USER_STORY:203–204, "A guard makes the book
fast and safe; it does not make it shorter. Only proof does that"). Full elision requires every command
*upstream* of the target to be oracle-covered-and-converged (the poison-wall logic, IMPLEMENTATION:218–274).
Therefore:

- On a **fully converged** host, everything elides and attention is maximally saved — but a fully
  converged host is exactly the run where you least needed to look.
- On a **drifted** day, the thing that changed runs (a wall), and everything downstream demotes to
  guards (no attention saved) — unless you turn on the survival tier, which the design itself brands
  *"marketing at best … theatre at worst"* (USER_STORY:472–475).

So the headline currency is delivered in inverse proportion to demand, and the only recovery on the
high-demand (drifted) days is the naked-trust tier the design is most ashamed of. The corpus half-owns
this (stage 5 is literally this story) but frames it as a triumph of honesty rather than as the value
prop eating itself. I flag it as a lens on F1/F2 rather than an independent kill: it is why the survival
tier's fencing (F1) and the plan's stability (F2) matter more than their "1-in-10 / safe-direction"
framing suggests — they are load-bearing for the *only* days attention-saving is wanted.

## Finding F6 — "monotone under oracle-loading" is over-stated across the sparing/survival seam (supporting)

**Severity: over-claim in a human-managed doc + the buildable spec. ~SUSPECT (possible tier-confusion;
flagged for adjudication).**

USER_STORY states monotonicity unconditionally: *"Silence licenses nothing; more description is only
ever upgrade"* (USER_STORY:213), *"adding one never endangers someone else's line"* (USER_STORY:211).
`277` §3 restates *"monotone under oracle-loading."* This holds cleanly for the fact-plane licensing tier
(elide/guard/run of a command's *own* line). It is over-stated for the **selector-dialect sparing**
algebra, which feeds the *survival* consumer: loading a more-detailed oracle *adds tokens to a family's
`dialect(family, kind)`*, which can turn a previously-colliding (safe) cross-cell comparison into a
*sparing* (a fact survives a wall) — i.e. added description can *expand* the under-execution surface
under `--risk-faultless-skips`. "More description is only ever upgrade" is true for what a command does to
its own line; it is not true, in the dangerous direction, for what added cross-cell vocabulary does to
survival sparing. I hold this at ~SUSPECT pending the entity-algebra lanes — it may be that the intended
reading of "monotone" is fact-plane-only, in which case the over-claim is in the *wording* of a
human-managed doc rather than in the mechanism.

## Clean bills (attacks that did not land — stated plainly)

- **The ternary `compare` relation routing is coherent** (+SURE). `277` §2's map — `same`→transport
  (never survival), `provably-disjoint`→survival sparing (never transport), `unknown`→safe bottom for
  both — is internally consistent; `unknown` is genuinely the meet that is safe for both consumers. The
  soundness risk lives entirely in the *generators* (invariance/resolve/lend being wrong, per F1), not in
  the relation.
- **The `( check ) || original` guard is errexit-sound** (+SURE). The left operand of `||` is
  `set -e`-exempt, so a failing/confused check falls through to the verbatim original; `$?`/observable
  consumers are separately handled by `inv-one-observable`'s Status channels and the `GuardInsert`
  no-mint carve-out (`spike/CLAUDE.md`). USER_STORY:189–198 states this correctly.
- **The resolve-merge → plain-elision under-execution attack DISSOLVES** (+SURE). A merged command's own
  `is_converged` re-probes its own entity, so a wrong over-merge cannot skip a plainly-elided line; the
  merge only matters in the stand-in consumers (survival, cross-context transport). USER_STORY stage 6's
  "wrong merge only over-verifies" is correct for the footprint consumer it analyzed. This is why F1
  narrows to invariance/reach and does not indict the whole un-flagged core.

## Part-1 bottom line

No single clean kill-shot that ends the project on its own — but F1 is a genuine, reasoned challenge to a
*last-day* ruling that the human typed acks on, showing the admin-consent boundary is drawn at the wrong
place and that a human-managed root doc (USER_STORY's "stays-local" reassurance) is now false for the
transport/reach tiers. F2 is a concrete, unaddressed defect in the *most recent* work that corrupts both
headline surfaces. F3 is a latent breach of a human-written promise the corpus blessed the vocabulary for.
F4 is the echo-chamber structure made operational. Together they are the strongest case that the corpus is
"most convincing exactly where it is wrong" — the razor (F1) and the pipefail ruling (F2) are the most
elegant, most-acked recent artifacts, and both have holes their own elegance hid.

---

# Part 2 — Adjudication against the subagent / foreign-model lanes

*(Appended after Part 1 was committed unspoilt. Six lanes: a pipefail empirical verifier, an
off-ramp/strip auditor, a root-doc contradiction sweep, an entity-algebra adversary (all Opus),
a Codex/GPT-5.6 outside-lineage review of `277`, and a Sonnet process/chronology audit. Every
subagent claim was re-verified against the rules before crediting — the standing discipline
against credulous adversarial adjudication, MEMORY `crosscheck-adjudication-skepticism`. Verdicts
below are mine, not the lanes'.)*

## Adjudication summary

| # | finding | source | verdict | live-at-v1? | direction |
|---|---|---|---|---|---|
| F1 | razor flag-boundary drawn at pinnability, not blast-radius | my Part 1, reinforced | **stands, headline** | yes (transport un-flagged) | under-exec |
| A1 | `275` §6 value-transport still licenses from negative space task-8 revoked | Codex fd4 | **credited, strong** | yes | under-exec |
| A2 | "same state ⇒ same value" ignores observer-context dependence | Codex fd5 | **credited** | yes | under/over-exec |
| A3 | value-freeze patrol omits poison-wall (unmodeled-mutator) degradation | entity-agent F4 | **credited** | yes (fold TYPED) | under-exec |
| A4 | substrate-label trust-tier mislabel + differential-CI temporal blind spot | entity-agent F1 | **credited** | latent (netns/fs-view) | under-exec |
| A5 | §3 sparing predicate spares ⊤ (selector-less) *backings* | Codex fd1 | **credited, spec bug** | survival-gated | under-exec |
| A6 | family-scoped selector *meaning* fed into family-free token *equality* | Codex fd2 / entity F2 | **credited** (mis-classed as lint) | transport default-on | under-exec |
| F6 | "monotone under oracle-loading" & "attributable" false as written | my F6, confirmed | **promoted +SURE** | survival-gated | under-exec |
| F2 | pipefail `\| grep -q` verdict nondeterminism → plan flap + `--exit-code` breach | my F2, verified | **stands, scoped** | yes (narrow conjunction) | safe (→run) |
| F4 | round-order bakes highest-lock-in design before any outside push-back | my F4 + chronology | **promoted** | — | — |
| A7 | nested `lend_map` composition has no defined algebra | Codex fd6 | credited, medium | yes | under-exec |
| A8 | two-binary floor does not reach `sh -c` payload interiors; post-strip shell-shift unpriced | off-ramp agent | credited | yes | correctness |
| A9 | zsh "IN via discipline" — the discipline set is silently incomplete (u-curve dip) | off-ramp agent | credited, scope | yes | correctness |
| F3 | replace/changed-detection-fold vs byte-for-byte plan-honesty | my F3, softened | **narrowed to render-honesty** | no (unstamped) | — |
| A10 | binding-site elision unbinds runtime consumers | Codex fd8 / entity F5 | credited, **acknowledged-open** | v1 hazard, deferred | under-exec |
| A11 | row-3 `dorc-sh` in admin books breaks the "trivial off-ramp" promise | off-ramp / root-doc / entity F6 | credited, narrow | yes | off-ramp |
| A12 | USER_STORY (authority tier) now teaches dead spellings | root-doc agent | credited, disclosed | yes | doc-staleness |
| — | Codex fd3 (carried-by "generates invariance" contradiction) | Codex fd3 | **demoted/dissolved** | — | — |
| — | ternary relation *routing*; `( check )\|\|orig` errexit; resolve-merge in plain elision | my clean bills | **hold as clean** | — | — |

## The one finding that matters most — the transport cluster (F1 + A1–A6)

Read together, A1–A6 are not six scattered nits; they are one structural fact, and it is the fact
my F1 predicted from the design side. **Every default-on (un-flagged) soundness gap the review
found lives on the `same`/transport path, and the survival path — which the design *did* fence
behind `--risk-faultless-skips` — came out comparatively clean.** The entity-agent stated this
independently and I confirmed it: the selector dialect only ever feeds the flag-gated
`provably-disjoint` consumer, so its defects (A5, A6, F6) are opt-in and self-healing; the
transport/invariance path feeds `same`, fires by default, and is where A1–A4 concentrate.

That inverts the corpus's own risk story. USER_STORY sells the naked trust as living in one
fenced corner (the survival tier, stages 5–7, behind the flag; USER_STORY:633–693). The review
shows the *unfenced* corner — cross-context transport — carries genuine under-execution paths that
no flag gates:

- **A1 (+SURE it is a live contradiction).** `275` §6 step 4 still reads *"Invariance is already
  derived (`state_stored_only_in × carried-by × r2`)"* and concludes a value transports whenever
  its backing does. But `271:rul-invariance-speech-act` (typed the SAME DAY, task-8, AFTER task-7's
  `275`) re-roled that r2 derivation from license-generator to *contradiction-checker* and requires
  an explicit `invariant:` line — *"transport never licenses from negative space"* (`277` §4e).
  `275` §6 was never updated. Built as written, the value lane re-opens exactly the silence-licenses
  transport the design congratulated itself for closing (root-doc-agent Finding 9 documents the
  closure; Codex fd4 documents that it did not propagate). The `prefs`-socket example makes it
  concrete: a store with no `invariant:` line and no who-am-I ingredient derives "invariant" from
  negative space and wrongly transports one caller's value to another.
- **A2 (+SURE the assumption is unjustified).** Even with *correct* backing invariance, `275` §6's
  chain assumes "same backing state ⇒ same value." A read can honestly disclose `:? file` while its
  *output* also depends on observer context (`appctl current-mode` printing `alice:production` vs
  `root:production` off one invariant file). Backing-invariance proves *referent* sameness, not
  *observation* sameness; the value-transport proof conflates them. The fix requires ρ/register
  dependencies to enter the value recipe — machinery the design has but this chain does not invoke.
- **A3 (~SUSPECT, live at v1).** `275` §5's freeze patrol checks only whether anything *claims* to
  touch the captured backing in the window — an unmodeled (poison-wall) command claims nothing, so
  the fact-plane's poison-wall degradation is *not inherited* by the value freeze. A captured value
  can be frozen across a command that silently rewrites its source. `fold-into-analysis` is TYPED,
  so this is v1-live unless the rebuild inherits the fix by luck.
- **A4 (+SURE on trace, latent).** Substrate-borne invariance is branded *"engine-warranted,
  structural"* — the highest trust tier, no speech-act owed — but *which* substrate a store sits on
  (`kernel` vs `net-kernel`) is an author's token choice, untestable by the differential harness
  until the differentiating axis (netns) is built. `272` §3 r1's "strongest robustness property"
  (new axes consume existing declarations, *zero author rework*) guarantees a wrong label goes live
  *silently* when the axis lands. Structural warrant resting on an untestable authored choice is a
  trust-tier mislabel.

**Why F1's frame is the right lens on these.** The design's answer to all of A1–A6 will be the
razor: *the wrongness is attributable to a line, therefore vouch-tier, therefore acceptable
un-flagged* (`rul-flag-is-razor-residue`). But that is precisely the choice I challenge. These
bites are non-local (they under-execute a *different* author's command in a *different* file),
carry no runtime net, and the admin never consented — the exact anatomy the design flagged the
survival tier for. Drawing the consent boundary at *pinnability* rather than *non-local
blast-radius* is what leaves the entire transport path un-flagged. USER_STORY's "cheap honesty"
promise (a wrong un-flagged vouch "endangers its own author's own tool's line") is simply false
for transport/reach: those endanger *someone else's* line, un-flagged. Either transport joins the
flag, or that root-narrative reassurance must be retracted.

**Honest limits.** All of A1–A4 are traced through the *written spec*, not an implementation; an
implementer wiring values through the same CFG might inherit fact-frame safety for A3 by accident
(entity-agent flagged this, correctly). And the design *did* consider "honest-walls-for-worlds"
(no cross-context transport at v1) and reject it to keep the `24S` §2 headline — so this is a
challenge to a reasoned choice. But A1 is not a judgment call: it is a same-day contradiction
between two typed rulings, on the correctness-critical path, that would ship a revoked license if
built literally.

## The gating spec has concrete correctness bugs (A5, A6, F6)

`277` is the buildable spec that `270:block-rebuild` dispatches against. Three defects, all in §3,
all flag-gated (survival consumer) so blast is opt-in, but all real:

- **A5** — the sparing predicate (`spare iff claim-token ∈ dialect(backing-family) ∧ claim-token ≠
  backing-token`) special-cases selector-less *claims* (→ collide) but not selector-less
  *backings*: a whole-entity fact `sm.dorc.Service:foo` is spared by a `#enabled` disturbance
  because `enabled ≠ (absent)` and `enabled ∈ dialect` — so `systemctl disable foo` spares a
  whole-entity convergence fact and the needed re-enable elides. The predicate needs "either side
  ⊤ ⇒ collide."
- **A6** — selector *meaning* is `(family,kind)`-scoped (§3) but coordinates carry no family, and
  `same` is generated by raw token equality (§2). Two honest families both minting `#ready` with
  different meanings (config-exists vs health-ok) collide into `same` → one's fact licenses eliding
  the other's mutation. The doc parks this as *"adjudicability-list tier"* (a lint/UX concern);
  the review shows it is a live *correctness* path in the transport consumer, mis-classified.
- **F6** — `277` §3 lists *"monotone under oracle-loading"* and *"attributable"* as properties **to
  pin as tests** (§6). Both are false as written: loading a family's second arm (or *any* observe
  mark, entity-agent F3) grows `dialect(family)` and flips a cross-cell comparison from collide
  (run, safe) to spare (elide, dangerous) — monotone toward *more* under-execution; and cross-family
  sparing bites are not single-line-attributable (no line asserts the false proposition). Pinning
  these literally yields a vacuous or failing test.

## F2 — pipefail flap, verified and scoped

The verifier reproduced it: `producer | grep -q pat` under `set -o pipefail` returns rc 0
deterministically when output fits the pipe buffer, 141 deterministically when the match is early
in a bulky stream, and a genuine run-to-run **coin-flip** (measured ~12/88 to 15/85) in the
~48–66 KiB straddle band — same host, same converged state. Confirmed load-bearing: `24J`'s
connected-probe lift consumes the *runtime-measured* pipeline rc, so the racy value is the verdict
input; 141 → `≥2` sink → run. Two things the corpus does not close: (1) it models 141 as a *stable*
landing and never acknowledges the run-to-run flap, so `dorc plan` output churns with no world
change — attacking the attention product; (2) that flap collides with `276`'s own `dorc plan
--exit-code` contract ("gate divergence-of-world, never plan shape") — a converged host whose
producer size wobbles across 64 KiB (`ps aux`, `docker ps`, `dpkg -l`) flips its exit code with
zero world change. Always safe-direction (never a wrong skip; the `≥2⇒run` partition caps it at
lost value), and scoped to `{pipefail ∧ early-exit -q ∧ bulky all-covered producer}` — narrower
than "all `|grep -q` books flap," but real, unaddressed, and on both headline surfaces. Fix rides
an already-open item: the `--exit-code` contract must compute from world-divergence facts, not raw
probe-rc sink-landing, and/or the connected-probe compiler should prefer full-read consumer forms
(extending the oracle-scoped quality-bar rider into the *probe compiler*, where it reaches
book-lifted pipelines the stdlib rider cannot).

## F4 — the echo-chamber, made operational (with the chronology receipts)

`270` sequences the highest-lock-in machinery (block-settle: entity algebra, wrappers, capture —
USER_STORY's "fewer than 1-in-10" tail) *first*, gating the only outside-contact (field trial,
corpus) behind it. The lock-in justification ("author the stdlib once against final shapes") is
circular: it saves work only if the shapes are right, and the shapes are derived by self-citation.
The chronology audit put numbers on the risk: **6 design arcs closed in one ~13.5-hour session on
2026-07-12** (tasks 6/7/8/9/12 plus the `276` language sitting), the `271` ledger **compressed
1498→612 lines the same day** (deleting the dialogue that would let anyone audit *how* the rulings
were reached), and multiple **welds/permanents stamped that same day** (the kWHICHSH weld, the
two-binary floor, `rul-flag-is-razor-residue`). A1 is the smoking gun that this pace cost
correctness: task-7's `275` §6 and task-8's `rul-invariance-speech-act` — closed hours apart the
same day — ship a direct contradiction on the transport path, because the fix to one was never
propagated to the other before both were "CLOSED." The structure guarantees the hardest-to-unbake
design is baked, and its audit trail compressed away, before anything outside the corpus can object.

## Findings that soften, and honest dissolves

- **F3 → narrowed.** The root-doc sweep confirmed the replace/changed-detection-fold (`273` §7)
  reconciles further than a clean breach: "replace" is a blessed KNOBS mechanism, the rc/stdout
  substitution machinery pre-exists (`inv-one-observable`), and `rul-composed-bytes-defer-and-floor`
  pins folds to world-spoken provenance. The genuine residual is narrow and real: `273` §7 splices
  a predictor into a *surviving* pipeline and never reconciles the plan-**render** honesty against
  DESIGN's byte-for-byte promise or `rul-attention-honesty` — it raises honesty only re: version
  drift. Unstamped/v1-excluded, so a watch-item, not a shipped breach.
- **A10 (binding-site elision) → acknowledged-open, not novel.** `275` §5 itself flags that eliding
  a `PKG=$(...)` assignment unbinds the variable for runtime consumers; Codex/entity-agent gave the
  concrete mis-execution (`apt-get install -y ""`). Real v1 hazard, but the design named it and
  deferred it. Needs a hard gate in the block-context brief (never elide a binding with a live
  apply-time consumer outside the folded region), not a redesign.
- **A11 (row-3 off-ramp), A12 (stale USER_STORY spellings), A9 (zsh discipline gaps) → credited,
  disclosed, narrow.** Each is a real narrowing of a root-doc promise (trivial off-ramp; authority
  doc correctness; shareability-to-zsh), each is *disclosed in mechanism* somewhere in the corpus,
  and each is loosely or not scheduled for reconciliation against the human-written promise it
  touches. Worth the human's eye; none a kill.
- **A8 (payload floor-coverage) → credited, unacknowledged.** The two-binary floor validates the
  outer file but not `sh -c` payload interiors (opaque argv; run under host `/bin/sh`, not
  posh/dash), and the pinned→host-sh semantic shift post-strip is unpriced for payloads. The
  least-acknowledged of the off-ramp gaps.
- **Codex fd3 → DISSOLVED.** Its "carried-by generates invariance, contradicting keying" reads
  `277` §2's terse registry line literally; but `277` §2 explicitly defers the semantics to
  `272` §3 r1, which is precise (carried-by ⇒ keying for the carrying axis, invariance for the
  others). Terse-but-not-contradictory; an implementer is pointed at the correct source.
- **Clean bills hold.** The ternary `compare` *routing* is coherent (`unknown` is the safe meet);
  the `( check ) || original` guard is errexit-sound; `272` §4 never-derive-separation is sound in
  both directions (entity-agent verified the docker-rootless case); and my own resolve-merge attack
  stays dissolved (self-probe defeats it in plain elision). Crediting these matters: the danger is
  concentrated, not diffuse.

## Verdict

No single unanswerable kill-shot that ends the project on its own — and I will not manufacture one.
But the review does not come back empty, and it comes back pointed exactly where the human
predicted: **the corpus is most convincing precisely where it is most wrong.** The two most
elegant, most-acked, most-recent artifacts — the razor (`rul-flag-is-razor-residue`) and the
value/invariance transport chain — are where the real soundness gaps cluster, and they cluster
there *because* those arcs were closed fastest, last, and with their dialogue compressed away.

The strongest case for pumping the brakes before block-rebuild:

1. **The gating spec (`277`) has concrete correctness bugs** (A5) **and states false properties it
   instructs builders to pin as tests** (F6). That alone warrants a spec revision before dispatch,
   independent of any philosophy dispute.
2. **The default-on transport path carries genuine, un-flagged under-execution** (A1–A4), including
   a same-day self-contradiction (A1) that would ship a revoked license if built literally. This is
   novel, recent, on the correctness-critical path, and *not* dismissible as the disclosed
   "bought unsoundness" — that was sold as living behind the flag; this does not.
3. **The consent boundary is arguably mis-drawn** (F1): the razor gates *pinnable* wrongness, but
   the thing that warrants admin consent is *non-local, no-net* under-execution, and the two do not
   coincide. This is a design-judgment the human typed acks on under last-day pressure, and it
   deserves re-litigation in a fresh session — not because the razor is inelegant (it is elegant),
   but because its elegance hid that "attributable" was quietly substituted for "consented."

Recommended disposition: **hold block-rebuild dispatch** pending (a) a `277` §3 spec fix (A5, A6,
F6), (b) reconciliation of `275` §6 against `rul-invariance-speech-act` (A1) with an explicit
ruling on whether cross-context transport is flag-gated (F1), and (c) a poison-wall inheritance
clause in the value-freeze patrol (A3). None of these is a rewrite; all are cheap now and
retrofit-hostile later — which is exactly the property `270` invoked to justify doing block-settle
first, now turned against the settlement's own gaps.
