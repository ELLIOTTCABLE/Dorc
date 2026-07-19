# 279b — Round-27 design-package review (solution N)

Reviewer-authored, 2026-07-13. Scope: `Research/plans/270`,
`Research/plans/271`, and `Research/notes/272`–`278`, read against the root
authority documents, `spike/CLAUDE.md`, the predecessor designs cited by the
package, and the spike seams the rebuild would replace. This report changes no
reviewed document.

## Review verdict

**Hold `block-rebuild` dispatch.** The package has a sound conservative spine in
several places, especially its ternary safe-bottom and its refusal to derive
separation from mere context keying. But two new rules manufacture knife-tier
licenses from speech-acts that do not actually say what those licenses require.
Those are silent-under-execution risks, not missing polish. The capture and
language designs also leave high-lock choices to builders while describing the
work as settled enough to gate a rebuild.

## Findings, most severe first

### 279b-fd1 — Backing disclosure is consumed as an unspoken completeness claim

**Severity: BLOCKER · Confidence: HIGH**

The value design derives each backing from the union of `:?`-marked reads and
says the mark asserts exactly one thing: “this read reads X”
(`Research/notes/275-value-predictions-and-the-capture-lane.md:68-75`). It then
licenses arm-killing, operand concretization, and locator classification when the
declared backing is fresh
(`Research/notes/275-value-predictions-and-the-capture-lane.md:97-115`). Its
cross-context proof starts with a stronger premise: “the value is a pure function
of the state its backing names,” hence same backing state implies same value
(`Research/notes/275-value-predictions-and-the-capture-lane.md:140-159`).

That premise is neither derived nor authored. The predecessor ruling says the
opposite: a backing is a declaration-scope, not a computed read-set, and “carries
no completeness burden”
(`Research/notes/24D-stage3-claim-tier-and-guard-typespec.md:156-169`). The live
type preserves that exact contract
(`spike/crates/plan/src/survival.rs:398-407`). A positive disclosure that a command
reads X does not say that it reads *only* X, nor that the output is deterministic
given X.

Ground it in ordinary shell:

```sh
choice=$(tool read-choice)    # oracle discloses the shared DB cell
case "$choice" in
  apt) apt-get install p ;;
  brew) brew install p ;;
esac
```

If `tool read-choice` also consults `$HOME`, locale, a process-local cache, or the
clock, an honest `:? shared-db` mark remains honest while the value differs between
alice and root—or after an earlier apply command changes the hidden input. Dorc can
then fold the wrong arm even though every named backing transported and survived.
This is distinct from the accepted probe→apply TOCTOU residual: the omitted input
can already differ at probe time, or be visibly mutated inside the patrolled apply
window.

The package makes the exposure broader while understating its authoring cost. It
promises no new speech-act for captures
(`Research/notes/275-value-predictions-and-the-capture-lane.md:200-208`) and
hard-defers never-settled inputs
(`Research/notes/275-value-predictions-and-the-capture-lane.md:81-95`). “Honesty of
the mark” cannot close the gap without retroactively changing `:?` from positive
read disclosure into an `only-depends-on` plus hermeticity vouch. That would be a
much larger, much less obvious author contract.

**Required before rebuild:** keep `:?` positive-only and prevent it from licensing
value folds/transport, except for engine-known register values, until Dorc has
either (a) a complete dependency proof for an analyzable body or (b) an explicit,
owned, sh-spelled completeness/hermeticity speech-act. The latter must price hidden
environment, time, process, and other never-settled inputs; silence must remain
unknown. Whichever rule is chosen must govern both same-context wall survival and
cross-context transport.

### 279b-fd2 — Selector “dialects” create pairwise separation vouches nobody wrote

**Severity: BLOCKER · Confidence: HIGH**

A family is a name-derived coherence unit that is explicitly not guaranteed to
have one author (`Research/plans/271-block-settle-rulings-ledger.md:267-274`). In
the selector design, every verdict or observe mark mints its selector; all selectors
minted anywhere in the family become the family dialect; and unequal selectors in
that dialect are treated as provably disjoint
(`Research/notes/277-entity-algebra-design.md:151-177`). The resulting answer is
classified as a vouch-tier generator whose `provably-disjoint` result licenses
survival (`Research/notes/277-entity-algebra-design.md:118-139`).

That makes this apparently local edit nonlocal:

```sh
is_active "$svc"    :? sm.dorc.Service:"$svc"#active
```

Once loaded into a family that already mentions `#enabled`, the new line changes an
existing `#active` footprint versus `#enabled` backing from collide to spare. Yet
the mark is also defined as a read-disclosure
(`Research/notes/277-entity-algebra-design.md:258-266`), and the grammar says a
single marked line asserts exactly one thing
(`Research/notes/277-entity-algebra-design.md:246-255`). Neither the author of
`#active` nor the possibly different author of `#enabled` wrote “these cells cannot
overlap.” Co-loading the lines manufactures that pairwise claim.

The package recognizes the physical failure mode—overlapping cells within one
dialect are “inherent to narrowing”—but treats differential testing as sufficient
containment (`Research/notes/277-entity-algebra-design.md:173-181`). It is not.
With *n* selectors, the occurrence set silently asserts up to *n(n−1)/2* separation
relationships. Loading a new oracle can therefore make an old plan more
aggressive, and line-level attribution points at a line that never expressed the
failed relationship. Under the admin's survival flag, a false spare is silent
under-execution.

This also falsifies the package's “noise fails safe on both sides” claim at
`Research/notes/277-entity-algebra-design.md:173-177`: a noisy but syntactically
valid selector occurrence can enlarge the dialect and mint new separation
licenses.

**Required before rebuild:** default unequal selectors to `unknown` unless an owned
positive speech-act defines their partition or pairwise separation. If the intended
contract really is “mentioning selector X vouches that X is disjoint from every
other selector this family may ever load,” say that plainly, give the coherence unit
one accountable owner, and remove the contradictory “read-disclosure only” story.
An admin risk flag is not a substitute for the missing oracle-author claim; the
package's own rule says the flag may permit acting on a separation claim, never
manufacture one (`Research/notes/277-entity-algebra-design.md:120-126`).

### 279b-fd3 — The scheduled capture slice omits mechanisms required by even its floor

**Severity: HIGH · Confidence: HIGH**

The charter schedules a first slice in which a vouched inner command ships in the
probe and captured single-line stdout folds back as a probe-provenance literal
(`Research/plans/270-round27-charter.md:156-162`). The source design had already
shown that this is a multi-wave program, not a value-recipe field addition. It needs:

- promotion of a vouched expansion-internal command into a probe site;
- a capture site/disposition;
- a value-plane←probe-result back-edge or a second value pass;
- probe-source provenance;
- a value-bearing wire; and
- replacement of `v=$(cmd)` with a faithfully quoted literal assignment.

Those requirements and their dependency order are explicit at
`Research/notes/219-arch4-cmdsub-design.md:154-183`,
`Research/notes/219-arch4-cmdsub-design.md:187-206`, and
`Research/notes/219-arch4-cmdsub-design.md:210-221`. The current spike still marks
commands inside `$()` as expansion-internal non-leaves
(`spike/crates/analysis/src/cfg.rs:1426-1444`; coverage exclusion at
`spike/crates/analysis/src/effect.rs:1143-1153`).

Round 27 carries only three representation reservations—pipeline order, literal
provenance, and per-channel backing—and explicitly says to reserve, not build, the
post-probe rebind
(`Research/plans/271-block-settle-rulings-ledger.md:822-835`). The value note also
postpones artifact-entering substitution and merely flags that eliding a capture
assignment unbinds the variable
(`Research/notes/275-value-predictions-and-the-capture-lane.md:97-115` and
`Research/notes/275-value-predictions-and-the-capture-lane.md:117-138`). The
human-managed TODO calls post-probe rebind “past the first slice” even though it is
the mechanism by which the first slice can affect downstream analysis at all
(`TODO-ADDTL.md:19-21`).

There is no correct middle behavior. Re-run the original `$(cmd)` during apply and
the bytes that selected the planned arm may differ; elide the assignment and the
variable vanishes; leave downstream uses at top and the promised fold did not
happen. Reserving a type slot does not choose the pipeline architecture that avoids
those failures.

The claimed wire upgrade is also incorrect. The imported multi-host contract leaves
the inner site grammar unchanged and grants last-to-token parsing specifically to
`coord=` and future free-content fields
(`Research/plans/262-round26-build-spine.md:108-142`). The incumbent `stdout=` slot
is tokenized with `split_whitespace`, so a value containing spaces is truncated
(`spike/crates/cli/src/main.rs:2968-3012` and
`spike/crates/cli/src/main.rs:3084-3100`). The charter promises single-*line*; the
available lane carries, at best, a single *token*.

**Required before block-context dispatch:** choose the re-entry architecture; specify
capture-site promotion and probe inclusion; specify the exact assignment replacement
and quoting/observable rules; and define a produced, validated stdout record whose
framing really accepts the promised line class. Pin the design with DST cases for
spaces, empty output, nonzero rc, merged stderr, hidden walls, downstream splitting,
and a probe/apply value disagreement. Otherwise the rebuild is most likely to bake
the current one-way value pipeline precisely when changing it is cheapest.

### 279b-fd4 — The executable language “spec” neither defines parser membership nor preserves the promised off-ramp

**Severity: HIGH · Confidence: HIGH**

The package says no written language spec will exist and defines a valid base text as
one that “parses and runs identically” under pinned posh and dash
(`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:192-213`;
`Research/notes/278-dorc-lang-v0-1-reference.md:16-36`). That is a useful conformance
differential, but it does not tell a static parser which texts belong to the
language. “Runs identically” has no quantifier over inputs, environment, filesystem,
external commands, timing, fds, or side effects. Running arbitrary candidate text is
not a safe membership procedure; running a finite fixture battery only specifies the
fixtures. Debian Policy supplies a strong human portability target, not the missing
grammar and semantic equivalence relation. Even Dorc-minted coordinate quoting is
left to parser-build time
(`Research/notes/277-entity-algebra-design.md:230-244`).

The document tries to distinguish a portable base floor from extensions above it,
but then spends “dorc-lang v0.1” and “off-ramp” as though they were one guarantee:

- Bare `set -o pipefail` is legal dorc-lang even though both selected floor binaries
  reject it; the portable gated spelling is blessed, not required
  (`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:152-187` and
  `Research/notes/278-dorc-lang-v0-1-reference.md:67-81`). Strip does not rewrite a
  bare `set`.
- Direct `dorc-sh` is legal runtime text and is deliberately untouched by strip,
  leaving a documented post-uninstall dangle
  (`Research/notes/278-dorc-lang-v0-1-reference.md:116-131` and
  `Research/notes/278-dorc-lang-v0-1-reference.md:173-191`).

Those choices may each be defensible as explicit executor-only escape hatches, but
they contradict the higher-authority promise that the immediate off-ramp is simply
running the script with dash (`DESIGN.md:75-81`) and the maintained story that after
strip “every artifact is exactly the standard thing it was before”
(`USER_STORY.md:994-995`). A documented exception is still an exception; it cannot
also serve as proof of the universal guarantee.

The underweighted cost is lock-in. The rebuild's parser, strip pass, diagnostics,
stdlib, and published oracle surface will all encode whatever builders infer from
this prose. A binary differential can catch deviations from a stated contract; it
cannot substitute for the contract without making acceptance depend on an unsafe,
environmental experiment.

**Required before parser rebuild:** publish a finite syntactic base and its relevant
semantic obligations; retain pinned posh/dash as the conformance oracle. Separately
name (1) base-dialect portability, (2) Dorc-executor-only extensions, and (3) the
post-strip artifact guarantee. Then either require the guarded pipefail idiom and
repair direct-`dorc-sh` stripping, or obtain a human change to the root off-ramp
promise that scopes those texts out. Do not leave builders to decide which meaning of
“valid dorc-lang” wins.

### 279b-fd5 — The formal relation stops before generator reduction and set lifting

**Severity: MEDIUM · Confidence: MEDIUM-HIGH**

The entity note calls its ternary signature, consumer table, and generator registry
the formal spine
(`Research/notes/277-entity-algebra-design.md:111-149`). It does not specify:

- how simultaneous generator answers reduce;
- whether and how `same(coord)` closes transitively;
- which contradictions refuse both answers; or
- how the relation lifts from coordinates to footprint and backing sets.

The last omission becomes immediate in this rebuild. The current backing is one
coordinate (`spike/crates/plan/src/survival.rs:398-425`), while the new design makes
it a set (`Research/notes/277-entity-algebra-design.md:350-361`). For backing
`{a,b}` and footprint `{x}`, `disjoint(x,a)` plus `unknown(x,b)` must not license
survival. Survival requires every footprint×backing pair to be proved disjoint;
cross-context value transport likewise requires every dependency to transport. An
accidental existential in either consumer silently under-executes.

The selector problem in 279b-fd2 is one concrete bad generator composition, but
repairing it does not fill in the general algebra. The package itself says the
generator registry and verification posture remain conductor-proposed and are on
the table for this adversarial pass
(`Research/notes/277-entity-algebra-design.md:439-466`).

**Required before the related representation/API freezes:** add the reduction,
conflict, closure, and set-lifting laws to the one-page spec, including attribution
aggregation. Pin the mixed-result cases in the pure DST kernel. The ternary codomain
is the right start; callers still need one unambiguous way to consume it.

## Suspicions checked and withdrawn

### 279b-wd1 — Wrapper prediction does not presently breach probe inertness

The wrapper strawman ends in `env … "$@"`, which would execute an arbitrary guest if
shipped naively
(`Research/notes/273-wrapper-surface-redesign.md:60-106`). I did not promote that to
a finding because the package does not claim naive shipment is valid. It explicitly
keeps probe-form composition draft-only behind task 14, requires every participant to
be replaced by its oracle prediction, and withholds un-oracled compounds
(`Research/notes/273-wrapper-surface-redesign.md:217-244`). It also records that
`env` cannot directly execute shell functions and routes the materialization/lowering
choice to implementation planning
(`Research/plans/271-block-settle-rulings-ledger.md:871-878`). The danger is real;
the fence is honest and holds.

### 279b-wd2 — A single context slot need not collapse a multi-axis context

I tested whether the representation had baked “context” into one scalar dimension.
It has not: whole-coordinate comparison is behind a relational chokepoint and the API
explicitly refuses per-axis pointwise decomposition
(`Research/notes/277-entity-algebra-design.md:91-109`). A slot can carry a product
context. No finding without an implementation that narrows it.

### 279b-wd3 — The safe half of the entity trichotomy holds up

The package correctly identifies `unknown` as the only shared safe bottom: it blocks
transport and collides for survival
(`Research/notes/277-entity-algebra-design.md:118-126`). It also correctly refuses
to infer separation merely because two values are keyed by different users; the
docker socket counterexample is apt
(`Research/notes/272-address-derived-topology.md:154-173`). Findings 279b-fd2 and
279b-fd5 concern how positive answers are minted and combined, not the ternary
codomain or the never-derive-separation carve.

### 279b-wd4 — Bare-`sh` descent is adequately fenced from licenses

I checked whether parsing an opaque `sh -c` payload could accidentally license inner
elisions. The design distinguishes hint-only bare `sh` from licensed `dorc:sh` and
requires the distinction at the type level
(`Research/notes/274-evaler-surface-and-reentry-token.md:38-62`). That is the right
failure direction. The off-ramp problem in 279b-fd4 is the separate direct-`dorc-sh`
dangle and the overloaded language guarantee.

### 279b-wd5 — Accepted TOCTOU, market-fit, and local algorithmic cost are not review findings

The probe→apply race is an explicit standing trade, and the package does not newly
hide it. The prompt also correctly excludes another market/corpus referendum. I
found no controller-local complexity issue whose cost approaches the remote-command
and network costs identified by the root design. None of those axes is used here to
inflate the findings.
