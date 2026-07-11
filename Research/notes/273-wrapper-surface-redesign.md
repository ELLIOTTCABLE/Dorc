# 273 — The wrapper surface: predict absorbs wrapper modeling, `cmd__lend_map()`, and probe-form composition

AI-authored (Fable, the `270:block-settle` rubber-duck sittings, 2026-07-11), minted at
the human's direction as the comprehensive durable of the task-5 arc (wrapper
context-function spelling, né `24S` §2b) — the `notes/272` precedent: the arc was
extensive redesign, large enough for its own document. Note-tier, kept-current through
block-settle. Authority: root docs and the `plans/271` rulings ledger outrank this;
ratification is MIXED and marked per-section (§12 is the status table — read it before
citing anything as settled). Companions: `plans/271` (the rulings; its task-5 entries
summarize and point here) · `notes/272` (the kind-side: address-derived topology this
design composes with) · `plans/24S` (the wrapper keystone this arc partially supersedes
— its §2b/§6b mechanisms are replaced below; its impossibility ledger §0 stands).
The full design-dialogue chronology lives in `plans/271`'s git history.

## §0 — The redesign in one view (what died of `24S`)

`24S` proposed one four-job context-function per wrapper (peel + axes + ρ-transform +
self-vouch) plus a kind-side declared trichotomy. The block-settle sittings dissolved
all of it into fewer, stronger pieces:

- the kind-side trichotomy → **derived** topology (`notes/272`:
  `kind__state_stored_only_in()` + the carried-by table).
- the four-job context-function → **two members**: the wrapper-shaped
  `cmd__predict()` (§2) and `cmd__lend_map()` (§3).
- the engine-built whole-ρ replication closure (`24S` §6b) → the **authored predict
  body itself is the shipped stand-in** (§6; ratification task-14-gated).
- wrapper self-vouch/self-effects → **no new spelling**: the wrapper's own
  `cmd__disturbs()` arms + the standing oracle-vouches-for-itself rule.
- `24S` §2a stage-D's "disjointness dividend" (`cron(root)` ⊥ everything) → **DEAD**
  under `272` §4 never-derive-separation: keying is license-free re-indexing; nothing
  here may resurrect it.
- vocabulary: "axes" → **dimensions**; "tail" → **guest** (authored-surface prose);
  "moves", "context" (as authored surface), "vantage", "probe stand-in" (as a distinct
  concept), `cmd__simulate_env()` (as a member name) — all retired.

## §1 — The surface at a glance

A wrapper family authors at most: **`cmd__predict()`** (the model — same member, same
contract as every tool on earth), **`cmd__lend_map()`** (the dimension member), plus
ordinary `cmd__disturbs()` / `cmd__is_converged()` where meaningful. There is no
wrapper-specific member species. **Wrapper-ness is detected, never declared**
(`271:rul-predict-absorbs-wrapper-modeling`, typed): a predict body whose
command-position `"$@"` runs its argument-slot is a peeling wrapper by tautology — a
wrapper IS a command whose behavior-model contains its argument-slot in command
position. The argparse path to that `"$@"` is the peel; the env-idioms along it are
the ρ-claims; the whole body, shipped, is the probe-lane stand-in.

The family definition bounding all of this (assumption-tail-suffix, spike-adopted):
a peeling wrapper parses a prefix of its own argv, then execs the REMAINDER verbatim
— same bytes, same order, nothing added/removed/substituted — once, locally. The
escapes map to other families: tail-as-string = carriers (task 6); substituted/
repeated/runtime argv (`find -exec {} ;`, `xargs`) = runtime-data (`24T:imp-P3`);
argv-transformers (GNU `env -S`) = out-of-family, per-shape decline. Safety
asymmetry: an inexpressible shape is unspellable ⇒ decline ⇒ HEAD wall — corpus
evidence prices only value, never soundness. Reliance registry: the coverage
argument, the single-command-position-`"$@"` contract, and peel-boundary detection
all rest on the suffix assumption; a counterexample forces a value-redesign
(tail-slice spelling), never a correctness cliff.

## §2 — `cmd__predict()` for wrappers

One uniform contract: *the best read-only sh model of your command.* A wrapper's
honest model transforms the environment and runs its guest:

```sh
sudo__predict() {                            # STRAWMAN body; the member name is settled
   while :; do case "${1-}" in
      -u) target="${2-}"; shift 2 ;;
      -i|-s) return 2 ;;                     # login-shell shapes: rho unclaimable, decline
      --) shift; break ;;
      -*) shift ;;
      *)  break ;;
   esac; done
   env -i TERM="${TERM-}" PATH="${PATH-}" HOME=/root USER=root LOGNAME=root "$@"
}
```

- **User-authored, never engine-generated** (`271:rul-simulate-env-user-authored`,
  typed): the body IS the per-variable env-claim (traced) and, pending §6's
  ratification, the shipped replication vehicle — no translation gap.
- **Per-channel claim/decline vocabulary** (merge-rider, acked): a delegation line =
  faithful all-channel claim · printf = asserted output claim · explicit `return` =
  rc claim · redirect-to-null = per-channel DECLINE (that channel ⊤ for consumers ⇒
  wall) · `return 2` = whole-shape decline. Engine-side discards at the invocation
  site (unconsumed-channel tidiness) are a distinct act from author-side
  redirect-declines.
- **Per-shape decline is load-bearing inside the flagship**: plain `sudo` has a
  claimable scrub-ρ; `sudo -i`/`-s` (and `su -`'s whole family) install login-file
  environment — host state, statically unclaimable — and decline. su authors NO
  predict at all; its value rides `cmd__lend_map()` alone (keying + the admin's own
  ambient guard lifting through the boundary).
- **Merge-riders** (typed, acked as build obligations): line-level attribution — the
  `24S` §4a chain's first link cites the LINE within the body (argparse lines = peel;
  env-idiom line = ρ; printf lines = output; marked lines = facts); opt-downs stay
  additive (rul24M-rungs-default pattern; kCONTRACT-RUNGS unmoved).
- **The licensure audit found no expressivity loss** in the merge: the elide/guard
  license stays `cmd__is_converged()`'s alone; hybrid tools needing both a
  measurement-rc and a passthrough-rc use per-line mark rc-capture (marks bind the
  annotated statement's rc; the body's terminal rc is the passthrough).
- Blessed idiom set for closure bodies (v1 draft, unsettled): `env -i` scrub-base ·
  `VAR="$VAR"` survivor · `VAR=literal` · `VAR="$parsed"` · terminal `"$@"`; cwd
  idiom TBD; a read-only query arm (getent-class, for `sudo -u TARGET`
  HOME-retargeting) is a wanted extension. `env`-the-command is the spelling's own
  stress-test (its ρ-delta arrives in its argv ⇒ a loop-shaped export body) — work it
  before ratifying the set. Closure bodies never escalate: real `sudo` is not a
  blessed idiom (§6's emergent imp-1).

## §3 — `cmd__lend_map()` (typed, `271:rul-lend-map`)

The dimension member: **a function from the site's argv to fixed strings, one entry
per dimension** the dialect knows. The name's rationale (human): "lend" raises
functional-programmers' hackles usefully — *I can infect my descendant; my descendant
can touch my stuff* — and has no likely corpus competitors, so it affords shortness.

```sh
sudo__lend_map() {                           # STRAWMAN body; name + semantics settled
   target=root
   while :; do case "${1-}" in
      -u) target="${2-}"; shift 2 ;;
      -i|-s) return 2 ;;
      --) shift; break ;;
      -*) shift ;;
      *)  break ;;
   esac; done
   printf '%s\n' "$target"     : user        # contents  = MAPPED lend
   :                           : fs-view     # empty     = FULL lend (shared world)
   "$@"                                      # the peel boundary
}

nice__lend_map() {
   while :; do case "${1-}" in
      -n) shift 2 ;; --) shift; break ;; -*) shift ;; *) break ;;
   esac; done
   :                           : user
   :                           : fs-view
   "$@"
}
```

Semantics, each load-bearing:

- **Empty result for a PRESENT key = full lend** — the guest borrows the caller's
  world on that dimension wholesale; spelled as the colon-line (sh's nothing-command,
  marked; strips to a harmless bare `:`).
- **Contents = mapped lend** — the guest's dimension is the caller's through the map;
  the value is ρ-resolved argv (an unresolvable `sudo -u "$DBUSER"` yields a ⊤
  value: identifies-with-nothing, preserved from `24S`).
- **A MISSING key = ⊤** — unknown; walls; hint-tier nudge ("sudo's lend_map doesn't
  answer netns; one line unlocks N sites"). This is the **enumerate-every-dimension
  law** (human-moved: choose silence-semantics by elision danger, not convenience,
  for the small authoring cohort): the absent-key-means-full-lend reading is
  explicitly REJECTED — it would reinstate the negative space the move killed.
- **No `only` in the name**: the member is arm-incremental (each dimension-line an
  arm), so the family's only-rule excludes the quantifier. Version story: a new
  dialect dimension (netns) is a missing line in old members ⇒ ⊤ ⇒ walls scoped by
  the carried-by table to the kinds that dimension carries — no version-scoped
  totality semantics anywhere.
- **Rationale banked** (why explicit enumeration): razor-conversion — under
  totality-by-contract the member's knife was an omission-failure (razor-FAILING per
  razor-attributable-line); explicit entries convert every wrongness into a positive
  mis-assertion on a pointable line. Family re-homing — the member leaves the
  survey-total `only`-class (tolerated where domains are open; this domain is closed
  and tiny) for the arm-incremental disturbs-class. And it restores `24S`'s original
  silence-safe posture (opaques7-finding2) while filling its one grammar hole (no
  positive changes-nothing spelling).
- **wart-quiet-danger-line**: the mechanism's most consequential line is spelled as a
  no-op (the colon-line) — loud-friend iconicity inverts, unfixably (sh's "same" IS
  nothing-happens). Mitigations: the line is present and pointable; the member NAME
  carries the license.

## §4 — The license anatomy, and the safety inversion

What a lend-entry licenses: a **full lend** asserts live referent-sameness — outside
measurements are honored inside (the elision-transport license, `24S` §6a's
identity-bridge + probe-outside pair) and the guest's disturbances land on ambient
cells (kill-traffic routing). A **mapped lend** re-keys the guest's cells to the
mapped value — license-free re-indexing (`272` §3) — and directs transport *within*
the mapped world where the kind's stores permit.

**All entry-types are dangerous when wrong, in the same direction** (human
correction, conceded): a wrong full lend transports across a boundary that exists; a
wrong mapped lend transports to the wrong target's world; a *partially overlapping*
map is the fs-view Hard cell, deferred. The full-lend line is only the entry most
likely to be wrong-by-thoughtlessness (the cargo-cult default) — an error-likelihood
statement, not a mechanism asymmetry.

**The phase inversion** (human-surfaced; the design already encodes it):
believed-no-overlap is the SAFE belief for the transport/elision consumer
(keyed-apart ⇒ no transport ⇒ runs) and the DANGEROUS belief for the
kill-traffic/survival consumer (wrongly-separated cells let a disturbed fact
survive); believed-overlap inverts both. No single default is safe for both
consumers — which is exactly why the `272` §1 comparison relation is TERNARY (*same*
licenses transport only; *provably-disjoint* licenses sparing only; *unknown* is
safe for BOTH) and why never-derive-separation exists (keying feeds only the
transport-blocking direction). Task-8's survival-flag ruling is about the one tier
where believed-separation is permitted to do work.

Composition with the kind side: transport across a wrapper boundary requires the
wrapper's lend-entry × the kind's `272` derivation — a full lend extends transport
even to kinds whose stores hang on that dimension; a kind whose stores ignore the
dimension transports regardless of the entry. Both authors stay ignorant of each
other (the `24S` §2c no-wrapper-awareness referendum, extended: no tool oracle ever
mentions a wrapper; no wrapper oracle ever mentions a kind).

## §5 — Dual-peel coherence

The two members' argparse structures are fully independent — each entry-point may
accept/license/reject any argument-set differently; `return 2` from one and a peel
from the other are coherent together (declining adds no license). The single
requirement: **given both members answer the same book-invocation, their `"$@"` must
abstract-interpret to the same tail position** — inner-oracle dispatch consumes
exactly one tail head. Disagreement is genuine static incoherence: provable, clearly,
forever, from static context ⇒ **immediate loud fail-fast error** (dictate-tier,
pre-network; the rul-proven-mutation-fails-fast posture). Fail-fast governs until
first network-transit; best-effort begins where static analysis is exhausted.

## §6 — Probe-form composition (DRAFTED; ratification gated on task #14)

`271:rul-only-oracle-bytes-ship`, drafted: **a probed compound ships with every
participant replaced by its oracle's predict** — modeled commands as their bodies
(stream-faithful by delegation: a body that calls the real read-only thing produces
the real bytes); a consumed channel the body declines (redirect) ⇒ that participant
unshippable for that consumption ⇒ can't-say ⇒ run; un-oracled ⇒ nothing ships. The
connected pipe becomes `otelcol__predict '--version' | grep__predict '-q' '0.155.0'`;
the composed wrapper case `sudo__predict a2enmod__predict ssl | grep …` — sudo's
body transforming the environment under which the inner predict evaluates. Kernel
that survives an earlier retraction: **per-channel coverage** — a participant may be
model-substituted iff every channel the compound consumes from it is covered.
Consequences if ratified: the "read-blessing" species dissolves into ordinary
delegation arms; the capture lane ships the delegation member and captures REAL
bytes (byte-consumption demands real execution — the same rule, not an exception);
imp-1 ("probes never escalate") becomes EMERGENT — enforced by the oracle-lane
read-only contract + the task-10b effect-check, never by engine name-knowledge.

Why gated: the landed `24J` connected-probe ships raw book bytes licensed by
engine-side shape-matching — audited and CONFIRMED as standing-law debt against the
round-20 structural-vouch ruling, the round-23 strip-predict correction, and
inv-one-observable (corrections minted in `24J`'s header, `24C` §pipe-guard, and
LIVING_STATUS, 2026-07-11) — but the human wants the hard law itself re-derived in a
fresh session before the repair is ruled (task #14). Raw-ship also laundered
model-fidelity gaps (a book's `grep -vq` raw-ships while the arm models only `-q`;
function-ship surfaces the gap as an honest arity-gate decline). Transitive
delegation (`doas__predict() { sudo "$@" ;}`) stays its own corner: traceable
composition, never shippable (escalation) — own care or out-of-v1.

## §7 — Output prediction and the replace tier (direction; unstamped)

`cmd__predict()`'s designed contract is per-channel OBSERVABLE prediction — output,
not just rc (`inv-one-observable`; `Predicted`/`OutClaim` reserved at tip). The
guard-affecting idiom class that demands it: **changed-detection** —
mutative-idempotent tools reporting did-anything-change via stdout, grepped, guarding
a downstream disturbance (`sudo a2enmod ssl | grep -q 'already enabled' || sudo
systemctl restart apache2`; certbot-renew/reload; local `rsync -ai`). This family is
Ansible's `changed_when` spelled in native sh; folding it elides restarts — and
rc-only oracles structurally cannot fold it (the dataflow between the verdict and
the admin's grep runs through BYTES). **Guards select for predictability**: an admin
greps only convergence signals, and convergence signals are functions of
probe-readable state — the idiomatic guard-affecting cell is systematically the
predictable cell. Credential-gated reads are never rescued (prediction cannot
manufacture unknowable state; the `sudo crontab -l`/`iptables -S` guards stay
honest walls forever).

The safe authoring shape is the **conditional claim** — "when state S (probe-read),
output matches L; else `return 2`" — naturally partial, fails to run. Output claims
inhabit the long-named, never-inhabited non-degenerate **replace** tier (KNOBS named
mechanisms): the guard-fold substitutes the mutative head with its read-only
predictor inside the admin's own pipeline. Fenced ladder (**output-composition-
ladder**): concatenation-shaped wrapper-output models (sequential same-fd;
deterministic) ⊑ per-line stream-transforms (`| ts`; automata-adjacent, `24T` §5b
fence-hover) ⊑ stderr/stdout interleaving (never). Eyes-open (human's lede, banked):
positing stdout-prediction opens the full-behavioral-model tail ("rewrite the damn
command in sh") — not pushed, priced: totalistic byte-prediction invites version-skew
drift (MH2-adjacent); unmatched shapes must ⇒ ⊤ ⇒ run.

**Wrapper-own-output is uninhabited** in idiomatic ops and the ecosystem selects
against it (docker routes pull-noise to stderr; `script -q`; `chronic` exists to
delete it). The real classes: handle-emission (`docker run -d` ids — capture-lane,
value-⊤ shape-claimable, not guards) and the fenced ssh archetype. Idiomatic
wrappers' output-function is passthrough — free in the body's bare `"$@"` —
with banner/chronic-class decoration/suppression rare and expressible in the same
body when real.

## §8 — Fences, failure modes, frontloaded limitations

- Credential-gated state: plan-time unknowable forever (`24S:imp-1` unchanged);
  run-with-guard is the honest cap; no machinery in this design touches it.
- The 23J park stands: no Dorc-authored checks under wrappers in the apply lane;
  when it unparks, guard-eligibility of `"$@"`-bearing bodies is per-instantiation
  read-only licensing (own contribution contract-read-only ∧ instantiated tail
  covered).
- Licensure-bearing argparse is BODY-LOCAL by design (human, standing): shared
  helpers may carry only license-free subfunctionality — no abstracting the meaty
  argparse. watch-argparse-clone-pain: 2–3 clones per wrapper family; measure at
  block-stdlib; macros are be-very-not-sh surface, priced only on cross-design
  evidence.
- Token collision: dimension marks (`: user`, `: fs-view`) and substrate marks
  (`: fs`, `: net-kernel`) are two engine-owned closed vocabularies; keep `fs-view`
  vs `fs` distinct; ONE deliberate kOOB reading covers both (queued, `272` §11).
- The knife inventory: wrong full lend / wrong mapped value = misdirected transport,
  attributed to the entry's line; wrong peel = static-incoherence fail-fast where
  detectable, wrong-oracle dispatch where not (the lying-peel sweep); wrong ρ-claim =
  wrong-CONVERGED via incoherent replication, same tier as a wrong check body;
  omitted dimension = walls (value-loss, never wrongness).
- Identity wrappers pay two near-trivial members (lend_map with colon-lines +
  bare-`"$@"` predict); the passes-through shorthand bundle stays unruled
  (two-mechanisms risk flagged).

## §9 — Verification posture

Sweep axes: lying-peel · lying-lend (both entry-types) · lying-ρ-claim ·
lying-output-claim; the mutate-as-A/probe-as-B differential falsifies wrong full
lends (per `272` §9, the derivation's other half). Test-pinnable invariants:
rung-0 wrapper-free goldens byte-stable · silence never identifies · ⊤-valued
dimension identifies with nothing · every cross-boundary elision renders its
four-link chain with line-level first link · keying never feeds survival ·
boundary-disagreement fail-fasts · stand-in fidelity on consumed channels
(differential harness — our CI, never the product's rescue, per
rul-net-quality-u-curve).

## §10 — Vocabulary record

Live: **dimension** (né axis; the human's Minecraft evocation: changing one swaps a
world of un-enumerated associated state) · **guest** (né tail, authored-surface
prose) · **visit/stay** (teaching prose: "sudo's guest visits user=root, stays in
fs-view") · **full lend / mapped lend** · **peel** · **wrapper**. Exec-scoped
framing (human-sharpened): these wrappers are not mutate-then-undo — the guest is
BORN differently-situated; the caller's world never changes; that is why read-only
oracles can answer honestly. Retired: "moves" · "context" as authored surface ·
"vantage" · "probe stand-in" (= predict-as-shipped) · `cmd__simulate_env` ·
"read-only-blessed command" (a category error; the blessing was always an oracle
vouch-species) · "environment" for this member (the family contains `env`, whose
domain is the OTHER thing; "environment" belongs to ρ forever).

## §11 — Open couplings

- **task-6 (carriers):** working frame = a carrier is predict-with-a-code-operand;
  su's `-c` shape straddles both designs; the argparse seam is shared.
- **task-7 (capture-claim):** four customers from this arc — dynamic locator arms
  (`272` §11) · the axis-independence value-bound · predicted-output values (the
  guard-fold runs admin pipes against predicted byte-streams; Predicted and OutClaim
  are one value-plane species) · capture-ships-real-bytes-by-necessity (§6).
- **task-8 (survival-flag):** re-read its adjudicability condition against
  derived-not-declared topology; the §4 inversion is its sharpening.
- **task-14:** gates §6; the `24J`/`24C`/LIVING_STATUS corrections stand regardless.
- **block-context briefs:** the env-closure stress-test + blessed idiom set + getent
  query-arm; the `24S` §8 W1–W2 staging re-reads against this surface (wrapper-peel
  stage = lend_map + detection machinery; wrapper-sudo stage = predict closure +
  probe-outside + composition).
- **stdlib briefs:** wrapper oracles (sudo, su, env, nice, `ip netns exec`) authored
  against THIS surface; the no-wrapper-awareness referendum watch-item stands.

## §12 — Status table

| component | status |
|---|---|
| the merge: predict absorbs wrapper modeling; detection by command-position `"$@"` | TYPED (`271:rul-predict-absorbs-wrapper-modeling`) |
| merge-riders (per-channel vocabulary · line-level attribution · opt-downs-additive) | TYPED (acked as build obligations) |
| `cmd__lend_map()` name + entry semantics + enumerate-every-dimension | TYPED (`271:rul-lend-map`) |
| user-authored closure (never engine-generated) | TYPED direction (`271:rul-simulate-env-user-authored`; member name superseded into predict) |
| dual-peel: independence + boundary-equality + fail-fast on disagreement | settled in-dialogue (human ack + his fail-fast correction) |
| safety-inversion precision (all entries dangerous; ternary relation encodes it) | human-surfaced, conductor-conceded; recorded |
| probe-form composition / only-oracle-bytes-ship | DRAFTED; ratification = task #14 |
| output-prediction + replace-tier + changed-detection fold | direction; scope call (v1 vs ladder-reserve) unmade |
| blessed closure-idiom set; env stress-test; getent arm | unsettled; block-context implementation-planning |
| assumption-tail-suffix | spike-adopted (typed), reliance registry recorded |
| vocabulary record (§10) | dimensions typed-liked; guest unobjected; retirements conductor-recorded |
