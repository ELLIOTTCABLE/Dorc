# 24L — the typeless-oracle floor + the flat graduation ladder (PROPOSAL)

> Tier: AI-authored (Fable, the `r24-language-review` design-synthesis session), 2026-07-07.
> **PROPOSAL — pending the human's final review; do not build until stamped.** Captures the
> post-24Kc design dialogue on cluster-authored-surface. Trust order: root docs +
> `spike/CLAUDE.md` rulings > this document > everything it supersedes. Confidence-marked
> per house discipline. Human-typed inputs from the dialogue are cited as such; everything
> else is conductor-synthesis. KNOBS slugs are referenced, never redefined.
>
> Position in the decision-package: `24Kc`'s verdict demanded the dq-kOOB/kTYANNOT
> authored-surface weld be ruled before P5. The human's direction in this dialogue
> (typed, 2026-07-06/07): gradual-enhancement gently trumps off-ramp; eol-comment
> annotations rejected on ergonomic experience (Flow→TypeScript, typed); the "just sh"
> identity re-grounded as **erasure-semantics, not surface-purity** (the TypeScript
> bargain, named). This document specs the two load-bearing members of that package —
> the graduation LADDER and the typeless FLOOR — and pointers the siblings (§9), which
> are separate work items.

> **⚠ SUPERSESSION ANNOTATION (2026-07-10, r24 close-out; ruling record `24M` §1, casualty
> ledger `24O` §2):** the human's typed rulings landed OVER this proposal. Three components
> died or reversed — **location-gating is DEAD** (typed constructs are NOT illegal in books;
> the per-file `# dorc-lang/v0.1` version-comment is the gate — `24M:rul-typeless-floor` +
> `24M:rul-version-comment`); **"share-a-file dies" is REVERSED** (share-a-file lives,
> marker-gated); **the `dorc_` name-prefix is DEAD** (bare `munged_cmd__role()` names —
> `24M:rul-bare-dorcism-names`). The load-bearing mechanism SPEC — §2–§7's auto-cell, four
> privacy fences, entity-free floor, monotonicity audit — SURVIVES as the build spec, plus
> the `24C` §24L-gating-errand touch-point map (the verdict-unaware-kernel seam; the
> probe-emission fourth touch-point §7 under-specifies). Build home: `270:block-rebuild`,
> the typeless-floor stage. Read §2–§7 as live, §1's gating mechanics as history.

## §0. Summary

One language, location-gated: books admit only valid POSIX; oracle files admit the same
plus the typed constructs. One opt-in spelling everywhere (reserved-prefix function
names; the dot dies). A **typeless verdict-function oracle becomes fully functional** —
guard tier, own-line elision, the no-shadow cascade — via a synthetic, entity-free,
per-provider singleton cell (**the auto-cell**) that is *private*: it never participates
in any cross-namespace comparison and can never license anything for anyone. Types then
buy exactly what they semantically add — within-command entity precision and cross-tool
coordination (kinds, footprints, survival) — so every rung of the admin→engineer ladder
teaches one concept and pays immediately, and no rung removes a safety. The minimum
functional oracle drops from five concepts (dotted name, argparse, bind, mark, minted
kind) to two (a name-shape, the rc partition), and is pure sh.

## §1. The ladder (the flat gradient this exists to produce)

Design target (human-typed value): the admin→engineer cliff shaved flat; dorc-as-teaching
— specifically teaching the other half of the userbase to contribute back — is a primary
goal, and it gently trumps off-ramp purity.

- **Rung 0 — book + stdlib.** Learn nothing. Paid: stage-1 elisions; the admin's own
  hand-guards lifted (Half-B).
- **Rung 1 — typeless verdict function, in the book.** Learn two things: the reserved
  name-shape, and "0 = yes, 1 = no, anything else = can't-say, and can't-say runs."
  Paid: guards on the named wall, own-line elide when converged+ambient, downstream
  un-walled on elide (the stage-3 cascade). Testable raw in the author's own terminal —
  it is plain sh.
- **Rung 2 — extract to a sibling `.oracle.sh` file.** Learn nothing (same bytes;
  `dorc extract` performs it). Paid: reuse across books.
- **Rung 3 — first typed construct, in the file.** Learn one construct. Paid: the
  specific cross-tool coordination the hint quantified (entity splits; footprint
  survival; shared kinds).
- **Rung 4 — publish.** Learn the kind-ownership rule + stamp the dialect version.
  Paid: fleet effects.

Every rung optional; no rung respells a prior rung's work; living permanently at any rung
is a supported steady state (the single-book homelabber lives at rung 1 forever, fine).

The four mechanisms that keep it flat:

- **flat-one-language.** One grammar; typed constructs (binds, trailing marks, kind
  functions) are *illegal in books* — a location-gated feature-permission (the `unsafe`
  shape), not a second dialect. Prior art: TypeScript `allowJs` — .js files are
  first-class citizens of a typed project; graduation is a file move, not a port.
  Consequence, stated plainly: **the share-a-file blessing dies** (USER_STORY stage 3
  rewrites — the book may *carry* oracles, typeless ones; typed enrichment lives in the
  extracted file). This repairs the settled-layer contradiction 24Kc found (share-a-file
  vs the kTYANNOT books-stay-runnable containment) in the promise-keeping direction.
- **flat-one-spelling.** The opt-in semaphore is a reserved-prefix plain function name,
  identical in book and file; the dotted form is retired. Sketch (final ceremony
  human-reserved, §8): `dorc_<munged-cmd>__<role>()`, e.g. `dorc_apt_get__is_converged()`.
  Human-typed nits folded: the hyphen munge is *unavoidable* (dash rejects hyphenated
  names) — the gain is only that authored-name and shipped-name become one string,
  written once by the author's own hand, visible, greppable, collision-lintable at
  authoring time (the `apt_get`/`apt-get` non-injectivity keeps its lint); and `__` is
  the standard generated/private/don't-conflict separator, adopted. Bonus: the prefix
  makes typos *detectable* — any prefix-bearing function that does not parse as
  `<cmd>__<role>` draws a loud diagnostic (kWARN-rich), where bare naming conventions
  fail silent. The capture-rake is closed by construction (the recognizer keys on the
  reserved prefix; ordinary user functions are never captured).
- **flat-extraction.** `dorc extract book.sh:N` writes the sibling file, same bytes.
  Sibling `*.oracle.sh` discovery is automatic-with-advisory (a loud `loaded N oracles
  from ./` line) plus the refuse-to-lose rule: coverage that existed last plan must never
  vanish silently because a sibling file went unloaded — that is a hard hint. (Interacts
  with 24H ack-6's explicit-loading lean; the refuse-to-lose rule is the reconciliation:
  magic still never *loads*, but forgetting becomes impossible to miss.)
- **flat-hint-curriculum.** Hints teach the *next rung only*, quantified, and emit
  paste-ready scaffolds: the firstwall-hint (built, `339189a`) grows a skeleton emission —
  the `case`, the `*) return 2`, the arity gate — so the author writes only the one line
  they uniquely know (their read-only check) and the safety ceremony arrives correct
  instead of learned. Cargo-cult razor check: every scaffold line bites when wrong
  (safety-critical class, acceptable). `dorc why` attribution keeps naming the author's
  own function and line.

## §2. The typeless floor — mechanism

**Status at HEAD (the alternative right now):** ~SUSPECT at the exact-code level, +SURE at
the law level — a mark-free verdict function is *inert dead weight*. Effects derive from
marked bodies; no marks ⇒ no cells ⇒ the site never classifies establish-bearing ⇒ no
guard (the mint keys on an establish-bearing class), no elision (the weld's arm is
`EstablishAmbient`), and the command still ⊤-walls when it runs. Today's minimum
*functional* oracle is therefore the full five-concept stage-3 ceremony — the entire
oracle value-curve, including the guard tier, is gated behind the typed dialect. The
corpus already strains against this: the pipe-guard fixture had to improvise a valueless
bind (`v : otelcol`) and an empty-entity singleton cell (`otelcol:.v0155`) to express
"otelcol's own state, shared with nobody" — a hand-spelled auto-cell, pure boilerplate by
rul24-boilerplate-cargocult (required ~always, bites ~never). The 255 dry-run's
hand-oracle was likewise written typeless. The floor formalizes what the corpus is
already contorting to express. (**Verification errand, gating:** confirm the inert-at-HEAD
reading on a mark-free fixture before building — if HEAD already grants more, this spec
shrinks.)

**The proposal:** for any provider bearing a verdict-function, classify mints a synthetic
establish-cell — **the auto-cell**:

- kind: the provider name, in a reserved namespace no author can mint into
  (spelled illustratively `cmd:foobar` herein);
- entity: **none** — a per-provider singleton. This is mandatory, not tuning (§3);
- property: opaque/auto.

Eligibility plumbing then lets the site into the *existing* arms: probe ships (the
verdict body, strip-only — for a typeless body the strip is the identity transform on
everything but nothing), verdict evaluated per-site per-reached-path as today, guard
mints past walls (vouch + converged), the elide-weld consumes the vouch + measurement +
ambience as today. No new license type, no new tier, no new verdict vocabulary.

**floor-privacy-rule (the load-bearing fence):** auto-coordinates are *private*. They
never assert disjointness against any foreign coordinate; in every cross-namespace
comparison — above all the survival tier's `footprint ∩ backing` test — an
auto-coordinate reads as **may-touch (⊤)**, never as "provably elsewhere." Mirrors
`resolve()`'s can't-resolve ⇒ may-alias ⇒ run. Consequences: a typeless fact can never
*receive* survival past a running wall; and since a typeless oracle has no `touches()`,
its running sites are un-footprinted total walls that can never *grant* survival.
Typeless oracles sit entirely outside the survival tier, on both sides; the double-gated
cell (`kSURVIVAL`) is untouched.

**What the floor is NOT (the three escaped sins, checked):**
1. *Not engine-side entity-guessing* (find-3): there is no entity (§3). Identity does not
   exist at the floor, declared or otherwise; it enters the system only with the first
   authored bind.
2. *Not identity-by-inference*: the auto-kind is a fresh per-command nominal that can
   never link to anything. The escaped sin was inferring *sameness*; the auto-cell
   asserts only *distinctness*, and distinctness is demoted to *incomparability* wherever
   it could license (the privacy rule).
3. *Not derived world-knowledge*: the auto-cell says nothing about what the command
   touches — no extent, no footprint, no reads. It says only "the fact this author's own
   probe measured lives at an address named after this command" — a tautology wearing a
   coordinate. All unclaimed knowledge stays unclaimed, therefore walled (silence=wall,
   owncoord's empty-emission = no-claim boundary, both unchanged).

House-pattern lineage: this is the fourth instance of "name what the author gave you, ⊤
what they didn't" — after opaque=⊤ (the ur-auto-type), `resolve()`'s may-alias floor, and
owncoord's no-claim boundary.

## §3. The entity question (the caught referential-abstraction break — entity-free is mandatory)

An earlier draft of this proposal had the auto-cell carry an entity "resolved through the
author's own argparse" (`cmd:nginxctl:mysite`). **The human caught this as a referential-
abstraction violation, and it is one:** a typeless oracle has no bind, and the bind is
precisely the author's act of *selecting which value is the referent*. The argparse
resolves values through control flow; it never declares which value is identity-bearing.
An engine that promotes an operand to a referent without that act has re-committed
find-3 one layer up. Therefore: **auto-cells carry no entity, ever** (fence-no-entity,
§7). All of a command's sites share one singleton cell.

Direction-check of what singleton coarseness costs: same-command multi-site books get
*more conservative* (more same-cell staleness hits ⇒ more forced runs/guards), never
less. Verdicts were always per-site (`inv-site-keyed-results` — each site's verdict
function runs under its own argv), so elision decisions still ride per-site measurements;
the shared cell only mediates cross-site invalidation. And the coarseness hands the first
bind its teaching moment: "Dorc treats all your `foobar` lines as one blob of state; bind
entities to split them."

## §4. Soundness, first principles (consumer × direction)

Frame: the cardinal sin is under-execution; over-execution/guarding is the safe
direction. Per consumer of a cell, the obligation is: *can an auto-cell make this
decision more permissive toward skipping, beyond what an authored, priced act licenses?*

- **The total wall (fd10) — the backstop theorem.** Any command that will actually run —
  foreign, opaque, modeled-diverged, typeless-diverged — walls everything downstream,
  cell-blind, at the unflagged floor. There is no cell input to weaken. This is why cell
  precision is nearly irrelevant to safety at the floor.
- **Ambience / same-cell staleness.** Auto-cells miss cross-command same-state links
  (different commands, same real state ⇒ different cells). The miss matters only if the
  interferer (a) runs — covered by the total wall regardless of cells — or (b) elides —
  and an elided command *did not execute*: the world it would have changed is unchanged,
  so the downstream site's own probe (which measured reality, never anyone's claim)
  remains valid. This is the **not-run theorem**, the same one the entire typed stage-3
  story stands on; it is not trust flowing downstream.
- **Guards.** Mint from vouch + past-wall + probe-verdict; re-verify live; frame-free by
  construction. No cell input. The typeless floor gets the full guard tier with zero
  soundness delta.
- **The elide-weld (own line).** Vouch (authored — the priced act, identical exposure at
  every tier: adequacy/converged≠no-op bites the author's own line) + the probe's
  per-site measurement + ambience (above) + reproducible observables. The vouch never
  enters the fact-plane (welded), unchanged.
- **Survival.** The single tier where cell *comparisons* themselves carry permission
  (§6's license law) — which is exactly why auto-coordinates are excluded from it, both
  directions, by the privacy rule. The constructed near-miss
  that makes the rule mandatory rather than decorative: stdlib apt runs diverged with an
  honest authored footprint (`package:nginx` + reaches-expanded files); a converged
  typeless `nginxctl` site downstream has naive auto-backing `cmd:nginxctl` — a different
  string from every authored coordinate — so a *naive* disjointness test passes and the
  line wrongly survives a wall that plausibly touches its real state. Distinctness-as-
  license is the failure; the two coordinate systems were not disjoint, they were
  *mutually ignorant*. The privacy rule (auto ⇒ may-collide ⇒ demote) closes it, and is
  also simply the correct answer.
- **Kills.** A typeless oracle declares none. Destructive verbs are declined (run, safe)
  or wrongly vouched — the same own-line judgment exposure a typed author has. Others are
  protected from a running typeless kill by the total wall; from an elided one by
  not-run ⇒ nothing killed.
- **`resolve()` / `reaches()`.** Never meet auto-cells. `resolve()` canonicalizes within
  authored kinds; auto-kinds are resolver-less by construction and private besides.
  `reaches()` expands *footprint* coordinates of owned kinds; typeless sites have no
  footprints to expand, and their backings are incomparable rather than disjoint, so an
  owner's closure never wrongly clears them. Collaboration machinery operates entirely
  within the authored coordinate system, which auto-coordinates never enter.

Worked cases (kept for the implementing conductor's pins):
- *Foreign mutator:* `hork` between two sites — opaque ⇒ ⊤ poisons all cells, auto
  included; the typeless converged site downstream guards (vouched, probe ships), never
  elides. Byte-identical to today's stage-2 behavior.
- *Non-collaborative typed neighbor, both directions:* typeless `certfix` runs above
  typed `foobar` (fb.Certs, converged) ⇒ un-footprinted wall ⇒ foobar guards. Typed
  `foobar` runs (footprinted) above converged typeless `certfix` ⇒ unflagged: wall ⇒
  guard; flagged: auto-backing incomparable ⇒ may-collide ⇒ guard. No quadrant lets
  either party's ignorance of the other manufacture a skip.
- *Same command twice:* upstream diverged ⇒ runs ⇒ wall ⇒ downstream guards; upstream
  converged ⇒ elides ⇒ no shadow ⇒ downstream elides on its own probe of an unchanged
  world. Neither branch consults entity resolution.

## §5. Monotonicity audit (no grade-up removes a safety)

- **Unmodeled → typeless.** Exactly two new permissions: own-line elide when converged
  (vouch-priced, own-line exposure — the standard authored act) and downstream
  un-walling on elide (the not-run theorem, not a trust grant; downstream still elides
  only on its own vouches and measurements). Everything else byte-identical to opacity:
  running sites wall totally, nothing survives them, nothing is claimed for anyone.
  A conservative refinement of unmodeled.
- **In-book → file.** No semantic delta; same bytes, same lift.
- **Typeless → typed.** Authored coordinates replace private ones. Cross-oracle same-cell
  links can appear ⇒ only ever *adds* forced runs (safety-adding). Backing becomes
  comparable ⇒ survival becomes *possible* — the sanctioned double-gated exception
  (wall-author's clean at-most claim AND the admin's flag); typing yourself into
  comparability is the first half of an opt-in, never a unilateral grant. Entity binds
  split the singleton ⇒ *recovers* elision on multi-site books. Nothing auto-cells
  protected is lost: the wall is cell-blind and unchanged; own-measurement validity is
  unchanged; mis-typing same-referent entities apart is the standing within-kind aliasing
  exposure (resolve()/horizon), pre-existing and priced, not new to this ladder.
- One eyes-open removal to name: today's five-concept ceremony functions as an
  *accidental competence gate* on skip-power. The floor removes it — a two-line oracle
  can now elide its own line. The gate was never a designed safety (the vouch was always
  the load-bearing judgment; measured-converged/ambient/observable conditions all still
  apply), but the change in who can cause a skip is real and is hereby stated.

## §6. License vs address — why the elision gradient does not run backwards

(The human's direct question, 2026-07-07; kept here because the implementing conductor
can hit the same confusion. AMENDED same date after a human-prompted soundness walk: the
original opened with the slogan "cells never license, verdicts license" — true in this
section's scope, false as a general engine law. The scoped form follows; do not teach
the slogan.)

**The license law, scoped precisely.** Permission is *granted* only by authored
judgments and by measurements, three-membered: the **vouch** (authoring the
verdict-function) licenses acting on its own converged answer at its own sites; probe-
provenance **values** license read-reproduction — and thereby dead-branch omission
through ordinary sh control-flow (the fold lane, which omits mutators with no vouch
anywhere; licensed by proviso-read-erasure's reads-only rule, the mutator's non-execution
being a consequence of sh semantics, not a licensed skip); and, in the flagged survival
tier ONLY, authored **cell comparisons** — footprint ∩ backing disjointness — carry
permission across a running wall. Everywhere below that tier, cell reasoning is
demote-only: it adjudicates whether a measurement is still a valid witness (staleness,
walls, query-validity), and every possible cell mistake at the base tier errs toward
run — the wall is cell-blind for anything that runs, and the not-run theorem covers
everything that doesn't. That survival is the one place cells DO license is not
incidental: cell-granted permission is precisely the bought unsoundness, which is why
that tier is double-gated and why the floor's privacy rule (fence-no-disjoint) must bar
auto-coordinates from the comparison at the type level rather than by convention.

**This section's question — per-path gradualness — under that law:** the per-path
elision gradient never came from types. The path-license is the verdict function's
*reached-path* answer under the site's constant-propagated argv (rul-guard-license's
"reached"; the 24A §1b vocabulary fence), plus the rc partition. The argparse is
therefore not meaningless at the typeless floor — license-scoping is its first job and
is type-independent: a path answering 0 licenses; a path answering `return 2` or left
unhandled declines and runs. The auto-cell only moves the *site* from unaddressable
(⊤ ⇒ classify refuses everything) to addressed (⇒ the verdict machinery may consult the
author's per-path answers at all).

Typing adds **address precision, not license precision**, and runs forward:
- binds split the singleton into per-entity cells ⇒ *more* elision on multi-site books;
- marks make backing comparable ⇒ survival becomes possible (double-gated);
- a newly-shared kind can surface a genuine same-cell link with another oracle,
  converting an elide into a guard/run — direction-checked: if the interferer runs, the
  cell-blind wall had already demoted the site (no change); if it elides, the not-run
  theorem says the elision was sound anyway, so the forced run is conservatism, not
  corrected unsoundness. (~SUSPECT how much of that conservatism is paid depends on
  whether HEAD's same-cell staleness machinery is verdict-aware or static — an
  implementation question to settle during the build; both resolutions are safe, they
  differ only in over-run cost.)

The tier-independent hazard, named so it is not mistaken for a floor artifact: the
*laziest* body (`dorc_foobar__is_converged() { foobar --check; }` — no case, no declines)
answers for every verb, purge included. The identical no-decline body is writable in the
full typed dialect today; declines were never spelled with types. Mitigations are the
existing ones at every tier — author judgment, the R2-MULTIOP/arity quality bar (the P5
brief already carries it, `252 §9` memo-2) — plus the floor's own: the scaffold-emitting
hint makes `case` + `*) return 2` + the arity gate what an author gets by *accepting the
scaffold*, so decline-by-default precedes the first purge-bite.

## §7. Spec obligations (unrepresentability targets, claim-tier style) + engine delta

Fences, phrased as compile-error targets for the claim-tier layer (rul24-overtype; the
Stage-2 `TC` discipline — doc-comments carry the half the compiler cannot see):

- **fence-no-footprint** — no `Footprint` is constructible from an auto-coordinate (no
  at-most claim from silence; the 233 sin must not compile).
- **fence-no-disjoint** — `disjoint` across the auto/authored namespace boundary is
  unspellable; the comparison type returns may-touch (mirror `Resolution::MayAlias`).
- **fence-unnameable** — auto-kinds live in a namespace other oracles cannot mint into
  or reference.
- **fence-no-entity** — the auto-cell type has no entity slot (not an `Option` — no slot).

Engine delta, enumerated: classify mints the auto-cell for verdict-bearing providers
(instead of leaving them effect-⊤); eligibility plumbing into the existing guard/elide
arms; the four fences; hint/why-lens attribution lines for auto-cell-backed decisions
("licensed by your `dorc_foobar__is_converged`, book.sh:22"). Untouched by construction:
wall walk, survival machinery, guard anatomy, rc partition, vouch law, kills, derivation,
resolve/reaches.

Test obligations (24B filing rules): in-memory pins for the four fences + the worked
cases in §4 (foreign-mutator, non-collaborative-neighbor both directions, same-command
pair) + a sweep topology extension so the end-state differential exercises
typeless-oracle scenarios (no lying-net analogue is possible — typeless claims never
travel — which is itself a pin: assert nothing auto-derived reaches a survival decision);
at most one or two e2e rows (a typeless-floor yardstick case; the improvised
`v : otelcol` boilerplate retired in whatever fixture migrates first).

Gating verification errand (run before building): (a) confirm a mark-free
verdict-function fixture is inert at HEAD (no guard, no elide, still walls) — the §2
status claim; (b) confirm the scoping of rule-query-validity below a RUNNING mutator —
total vs same-cell. The §6 base-tier direction-law ("every cell mistake errs toward
run") has one unverified edge in the fold lane: if query invalidation below a running
mutator is same-cell-scoped rather than total, a missed cell-sameness there is a
*permissive* miss (a stale query value wrongly folding a mutator branch dead). ~SUSPECT
it is conservative/total, per the `door1-guard-below-mutators-invalid` /
`query_after_mutator_is_invalid` pin names — verify in code. This edge does not touch
the floor itself (typeless oracles never enter the fold lane) but is load-bearing for
the stated law.

## §8. Costs, exposures, casualties (the honest ledger)

- Share-a-file dies; USER_STORY stage 3 rewrites (human-owned doc queue): the appended
  oracle loses its binds and gains the prefix; typed enrichment moves to the extracted
  file at stage 4+.
- The stage-3 exemplar becomes ~5 lines of pure sh; the nine-liner's binds/marks move to
  the rung-3 telling.
- Singleton coarseness on multi-site same-command books (over-run direction; priced;
  first-bind teaching moment).
- The competence-gate removal (§5, last bullet).
- The hyphen munge stays (human-typed: unavoidable) with its non-injectivity lint; the
  authored/shipped name unification is the whole gain there, no more.
- Scope honesty: the floor serves *mutator-verdict* oracles. Read-value commands
  (`hostname`, `uname` — the r25 host-guard wall, `255 §5.1`) need the Query/predict
  lane (purity + value), which the floor does not provide; the P5 coverage requirement
  for `hostname` stands unchanged.
- Adequacy (converged≠no-op) is neither worsened nor improved — identical vouch exposure
  at every tier; the r25 A1 instrument measures it regardless.

## §9. Open forks (human-reserved) + package siblings

Forks this document deliberately does not close:
- **The exact semaphore ceremony** (`dorc_` vs `dorc__` prefix; role separator `__` per
  the human's convention nit) — human picks; nothing downstream depends on it.
- **Registration-call variant** — a `dorc_provides`-style explicit registration was the
  runner-up semaphore; noted because it is the natural sh-native home for
  `kCONTRACT-RUNGS` rung-selection and the dialect-version stamp if the ladder pole is
  ever chosen; the prefix spelling wins on zero-ceremony and set-eu-safety in books.
- **Verdict-aware vs static same-cell staleness** (§6) — implementation investigation;
  affects conservatism only.
- Whether rung-1 in-book oracles are counted/rendered distinctly in plan-summary
  (display question, 24H territory).

Package siblings (separate work items from the same dialogue, not specced here): the
**loud-friend law** (a raw-run authored file must fail loud, never lie — kills the
silent-wrong-argv class 24Kb executed); **`dorc strip --spanmap`** as first-class
versioned tooling + borrowed-lint adapters (shellcheck/`dash -n` run on the stripped
form, diagnostics mapped back — the tooling-economics answer; the non-janky quarter of
the human's strawman-2); the **erasure-invariant** doc language replacing the "just sh"
claims (human-voice queue); the **dialect version marker** (human TODO, pre-stdlib);
and the 24Kc small fixes (return-decline-inert, nounset idioms, munge-reservation,
on-ramp arity honesty), all pole-independent.
