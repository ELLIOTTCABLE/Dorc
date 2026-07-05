# 24G — the kind-owner family: a live design round (identity, reach, typed emission)

AI-authored (Opus conductor), 2026-07-04, round 24. The RECORD of a multi-turn live design
dialogue with the human that settled the Stage-5 surface — the thought-process and the
arguments that decided each fork, not the surface itself (that lives in **USER_STORY stages
6–7**, minimized and human-audited; the two documents deliberately do not repeat each other).
Supersedes `24F §10`'s edge-annotation sketch where they conflict. Confidence-marked.

## §1 How the round went (the shape of the dialogue, worth recording in itself)

Five volleys, each overturning or sharpening the previous position — all at the
prose/walkthrough altitude, none caught by gates or tests:

1. The human challenged `24F §2`'s "bridges collapse into derivation" → dug the corpus →
   half-wrong: the OWN-oracle case collapses; the CROSS-AUTHOR case is the bridge's whole
   point (`23M` types-to-types; `ORACLE_PROVIDES` vocabulary-relations). → Part B minted.
2. The human challenged sh-spelling itself ("sh is a terrible language; the API doubled
   silently; argue these SHOULD be sh") → the razor emerged: **sh-spell exactly what must
   execute on a host; declare what is a relation between names** — and my first cut split
   the bridge into a static edge-declaration + a dynamic sh valuation.
3. The human asked whether edge/valuation are one family (the touches() graduation
   argument) → the split DIED under the graduation test (§3 below): one kind-keyed sh
   function, both maturities.
4. The human proposed the kind not be stringly-typed → typed emission (§4): kind as trailing
   annotation, entities as raw output; and ruled the error-posture (§5).
5. Naming (§6): `manifest()` → `reaches()`, and the name-as-contract principle articulated.

The meta-lesson, second time this round: real design errors surface at walk-me-through-it
altitude. Budget for it at every stage boundary.

## §2 The two families (settled)

The authored surface is TWO families keyed by different things — not five role-siblings:

- **per-TOOL** (keyed by command, invoked with a site's argv; every oracle author):
  `predict()` / `is_converged()` / `touches()`.
- **per-KIND** (keyed by vocabulary, invoked with an entity; kind-owners only —
  stdlib-concentrated, <10% of authors, high community-effect): `resolve()` (identity,
  1→1) / `reaches()` (reach, 1→N).

All five optional; each silence degrades to a named floor (the monotonic contract). A
command-keyed identity/reach function is a mis-keying (it would mint vocabulary no
coordinate uses) — enforced as a loud diagnostic, since the conductor made exactly this
mistake in a builder brief (provider-collision = warning; duplicate-kind = refuse-both
error).

## §3 Why ONE function per question, not edge-declaration + valuation-function (the graduation test)

The strongest argument in the round, the human's own (it is what carries `touches()`): if
two spellings hold the same information at different maturities, authors must rewrite across
strata on the day the information graduates — and learn two languages for one question. The
decisive test: take the flagship static edge, `package ⇝ service` (postinst-enables), and
apply its FIRST value-refinement — "only if the package ships a unit file" — which is
`dpkg -L "$1" | grep -q '\.service$' && printf '%s\n' "$1"`: **dynamic**. The canonical
static edge graduates on day two; `package ⇝ file` is BORN dynamic. Same question ("what
does touching one of my entities reach?"), same maturity axis, live traffic across it ⇒ same
family ⇒ one sh function, static arms traced / dynamic arms escalated, exactly the
`24E §14` machinery re-used at kind-key.

What the unified spelling costs, and what buys it back: an escalating body has a
statically-unknowable co-domain — a real loss for a static-analysis-is-the-whole-game
product — recovered by §4's typed emission (the co-domain is readable off the annotations
with zero tracing). The steelman for the split (declaration-time cycle checks;
eyeball-inertness) mostly dissolved: single-step expansion moots cycles, and static arms
never ship anyway. A free-floating `: package ~> service` line also had worse `kOOB`
standing than I first claimed — every existing mark anchors to adjacent code; a
freestanding edge-line is config-about-the-world, drifting toward the verboten sidecar pole.

## §4 Typed emission (kind = annotation, entity = runtime value)

The human's move, and it is the existing house pattern applied one level up: the dialect
already splits value-plane from type-plane everywhere (`dest : fb.Certs = "$1"`; trailing
establish-marks). Emission gets the same split: an emitting arm's KIND rides a trailing
annotation; its stdout lines are raw entities.

```sh
package.reaches() {
   printf '%s\n' "$1"    : service     # static arm — traced, ships nothing
   dpkg -L "$1"          : file        # dynamic arm — escalates, runs read-only
}
```

What it buys: (a) the **co-domain** statically, even when every arm escalates — checker
food, the §3 cost repaid; (b) the **vocabulary fence hardened** — at HEAD the derived-
footprint readback interns kind-strings out of host stdout, i.e. a host can mint vocabulary
at probe time (hygiene hole, not soundness — an alien-kind coord intersects nothing — but
the type-plane riding the runtime data-plane is the wrong direction); typed emission closes
the vocabulary at lift; (c) the `| sed 's|^|file:|'` wart dies — it existed only to staple
the kind in-band; raw tool output becomes the entity list; (d) reverse-DNS kind names typed
once per arm, not per line. `resolve()` needs no annotation slot: its co-domain is its own
kind by construction.

Costs, named: per-arm capture in the shipped derivation scaffold (one annotated command =
one kind = one capture unit; a genuinely mixed-kind single command must split its stream);
per-ROLE mark semantics (a trailing mark means establish in `predict()`, emission-typing in
`reaches()` — coherent, but a language decision to document loudly); pipelines must be able
to CARRY a trailing mark in reaches-bodies (a deliberate carve-out from `24E §14`'s
pipelines-carry-no-mark, which was right for probe/verdict bodies).

## §5 The error-posture (human ruling, binding on the whole family)

**Emission without typing means NOTHING** — an un-annotated emitting arm contributes no
coordinates and draws a smell-diagnostic, never a refusal. The lift's hard-error set stays
exactly the two standing categories: (1) syntax, (2) statically-derivable pure logic
conflicts that would genuinely break probing (two resolvers claiming one kind is in this
category; a resolver keyed to a known provider name is merely a smell). Everything else:
draw what we can from the author's sh-as-written. (This overruled the conductor's earlier
"required annotation / hard-refuse the lift" lean — silence licenses nothing, but silence
is never punished.)

## §6 Naming (`reaches()`, not `manifest()`; the name-as-contract principle)

`manifest` was `23M`'s example-name for the part-whole CASE, wrongly promoted to the
mechanism (the same error-shape as the keying bug: instance-name → mechanism-name). Two
principles decided the rename, both worth keeping:

- **In this dialect the function name is nearly the entire contract surface** — a
  period-function carries no signature; `is_converged`'s name already IS its license
  (USER_STORY: authoring the function that answers the question in its name is the vouch).
- **On completeness-shaped claims, the name biases what authors include** — and omission is
  `reaches()`'s dangerous direction (an omitted reach silently under-executes someone
  else's line). "Manifest" reads as *file list* and recruits omission of the causal edges.
  `reaches` is the corpus's own verb, scopes the full question, and conjugation-pairs with
  `touches` (a command *touches*; a touched entity *reaches*). `resolve` KEEPS its name on
  the same test: ops-native canonicalization verb whose connotation (reduce to the one true
  form) biases toward merging — that function's SAFE error direction.

## §7 Sequencing (human-set) + opens

- Part A (aliasing/`resolve()`) — LANDED + merged (`24C` §Stage-5A; three lying-nets green).
- **Part B next: `reaches()`** with typed emission + means-nothing posture (task #11).
- **`touches()` migration to typed emission: LAST** — after `reaches()` proves the shape;
  a conscious cleanup, not a blocker (the FIXME rides USER_STORY stage 5). The human
  independently dislikes the stringly-typed emission; it WILL be fixed, sequenced last.
- Then Stage 6 (measure/conclude/extract).
- OPEN (human, unpressured): whether the own-coordinate boilerplate (the leading
  `printf 'package:%s'` that exists only to satisfy the coherence check) dissolves into an
  engine-supplied default — deliberately left for the human's read.
- OPEN (post-trial): the co-reference contract — Part A's strain intel (`24C`
  strain-coreference-crosskind: the kind-fence short-circuits before canonicalization;
  `CanonicalCoord` cannot carry a target kind) is the design seed.
- Parked watch-item (raised in-round, not actioned): the razor retroactively questions
  `touches()`'s static half (Stage 2 built a Rust interpreter to statically read sh-spelled
  static relations — the cost of sh-spelling them); a line for the Stage-6 extraction, not
  a relitigation.

## Confidence

+SURE: the two-family split; the graduation test's verdict; typed emission's four buys; the
error-posture ruling (human-typed). ~SUSPECT: the per-arm capture cost stays modest; the
may-alias default (§3a of `24F`) still awaits fire-rate data. -GUESS: how far the
name-as-contract principle generalizes beyond this family (it should gate future
sibling-names regardless).

## §8 — the own-coordinate question: RESOLVED-ADOPT (2026-07-04, human; reservations noted)

§7's open item closed after a full interrogation (chat, this date). The design: the engine
UNIONS a wall-site's own effect-coordinate(s) — its establish cell, and its killed cell via
the kill-coords side-map — into the site's lifted footprint, **only when the author's own
emission is non-empty** (empty = no claim-act = wall; the engine never manufactures a claim
from silence — the anti-233 boundary). The AUTHORED lane keeps the coherence check as a
**pre-union canary** (cross-lane contradiction ⇒ refuse ⇒ wall, unchanged — it has real teeth
there); the DERIVED lane drops the own-membership requirement — the boilerplate
`printf 'package:%s' "$1"` dies, and deservedly: it was a separate trivial expression the
check validated *instead of* the derivation (the check was testing the decoy). Union coords
carry their own provenance (why-lens: own-effect, engine-supplied). Failure directions, swept:
a union coord only ever adds hit-surface (demotes; cannot license a survival); under
effect-lane error, union over-poisons (safe) where status-quo refuses (safer-but-costlier) —
union ≥ status quo in every swept cell. The load-bearing assumption — **effect-entails-touch**
(declaring "I establish X" while claiming "I don't touch X" is a contradiction in the
claim-language, not a choosable position) — survived interrogation; the two nearest honest
wants are selector-granularity (alt6 territory, orthogonal, unforeclosed) and same-cell
downstream facts (already `EstablishWritten` before the survival walk runs).

**rul24-boilerplate-cargocult (human rule-of-thumb, minted in the same exchange — a
surface-design/lint razor, keep it):** boilerplate required ~90% of the time that *actively
bites and must be removed* the other ~10% is acceptable when safety-critical — safety trumps
ergonomics. Boilerplate required ~70% of the time that does *not* noticeably and quickly harm
in its superfluous cases is the danger zone: it cargo-cults to 100%, produces no signal, and
retains its full ergonomics cost. The own-coordinate printf was precisely the second kind
(never bit when superfluous, near-universally required). Applies forward to the smell-layer's
design (a mostly-spurious smell decays the same way: ignored, valueless, still noisy) and
retroactively validates §5's means-nothing-not-refusal posture.
