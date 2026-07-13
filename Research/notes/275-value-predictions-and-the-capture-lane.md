# 275 — Value-predictions: the capture lane's species, fields, regimes, and validity

AI-authored (Fable, the `270:block-settle` rubber-duck sittings, 2026-07-12), minted as
the comprehensive durable of the task-7 arc (adj-capture-claim, né `219`
fork-capture-claim-type) — the `272`/`273`/`274` precedent. Note-tier, kept-current
through block-settle. Authority: root docs and the `plans/271` rulings ledger outrank
this; ratification is MIXED and marked per-section — §12 is the status table.
Companions: `plans/271` (the rulings; its task-7 entries summarize and point here) ·
`notes/219` (the origin analysis — round-21 vintage: concepts carry, file:line cites
and oracle-surface references are stale) · `notes/272`/`273`/`274` (the customer arcs)
· `plans/24T` (fences: imp-P1–P7 all stand). The design-dialogue chronology lives in
`plans/271`'s git history.

## §0 — The question, and what it dissolved into

`219` q-5 asked: is a probe-captured stdout a NEW claim-type (door-2-counterfactual
flavour) or an ordinary probe-observation (the reserved OutClaim channel finally
producing)? Answer (TYPED 2026-07-12, the assembled ruling —
`271:rul-value-prediction-species`): a named species, the **value-prediction**, that
is *representationally* the reserved seams producing (the one-Observable's stdout
seat; the cause-tagged ValueOf of the scheduled reshape; `inv-one-observable` intact)
and *behaviourally* a claim-type (typed fields; validity rules branch on them). The
fork's two positions were never opposed — the human's reframing ("do we allow the
validity-algorithm to DEPEND on claim-type, not do-we-have-one") was the real
question, answered by enumeration (§4). Three of `219`'s four forks died before this
arc opened: fork-cmdsub-top-cause → the value-recipe-reshape; fork-capture-probe-body
→ only-oracle-bytes-ship / task-14; fork-capture-wire → the single-line floor + the
wire-records-v1-import.

## §1 — The species and its name

**value-prediction** (name TYPED 2026-07-12): the object the analysis holds when it
believes an expression or variable will evaluate to bytes B at apply time.

Naming rationale (the human's): "claim" is the derived-license tier — `is_converged`
CLAIMS convergence; the oracles-may-lie buy-in lives there — while "prediction" is an
engine-internal shape-division, gently expected to be *less* (not zero) subject to
human interpretation. And it is literally what the object is: a forward-looking
assertion about an apply-time evaluation, justified by grounds. Conductor
verification, on record: (a) even delegation-captured REAL bytes are only ever useful
as a prediction of the apply-time expansion — the entire chronology apparatus exists
to justify exactly that inference, so "prediction" is *more* accurate than
"claim"/"known-value" (both over-promise certainty; "known" was the worst offender);
(b) it rhymes correctly with `cmd__predict()` (the species' chief producer) and with
the engine's reserved `Predicted<T>` shape; (c) register-resolved members (§3) are
certain-by-construction — calling them predictions is conservative, errs humble.

Coverage — ALL byte-shaped beliefs beyond program text: probe-captured stdout; stored
rcs (`rv=$?` — same species, per the travel concession: travel-distance is a property
of value-plane capture, not of a channel); composed output-predictions (`273` §7's
term now names one provenance grade of this species); register-resolved who-am-I
values.

Two implementor-facing boundary cares: (care-site-vs-stored) an rc consumed at its
site through branch structure (`check || act`) is the FACT machinery, not a
value-prediction — the boundary is *storage into the dataflow*, not the channel.
(care-outclaim-rename) the existing `OutClaim` newtype's name now clashes tier-wise
(it is channel *content*, not a claim) — rename rider on the value-recipe-reshape;
not urgent.

## §2 — The two fields, both derived, never declared

- **provenance**: a taint-style weakest-fragment grade over the value's recipe —
  `register` (the analyzer's own context record) / `world-spoken`
  (delegation-produced: the tool itself spoke at probe time) / `author-composed`
  (printf-produced: the author asserted it) / `⊤`. A value concatenated from a
  delegation read and a composed decoration grades as composed.
- **backing**: a SET of coordinates — the union of the producing reads' observed
  (`:?`) coordinates, computed **per-channel and recipe-granular**, not per-line.
  This granularity is the finding from `271:rul-orthogonality-counterexample-test`'s
  first run: the decorated-output oracle (rc depends on a state-read; stdout depends
  on state-read ∪ clock-read) genuinely needs "clock backs the VALUE and not the
  FACT" — which splits the mark's *consumers*, not the mark. The `:?` mark asserts
  exactly one thing ("this read reads X"); per-channel backings are derived by
  tracing which reads feed which observable.

Both fields ride the fragment-preserving, cause-tagged ValueOf/Recipe reshape
(`270:block-rebuild`); `271:rider-value-recipe-reshape-capture-seams` carries this
consumer plus the pipeline-order and literal-provenance seams.

## §3 — Three regimes, by backing-class

- **register-backed**: the read reads a context register the analyzer itself holds
  (the lend-mapped user value). Resolved analytically — no shipping, no probe; no
  staleness axis, because the register is region-constant by construction (the guest
  is born differently-situated, never mutated). `272` r2's blessed who-am-I list is
  simply the engine-known set of register-backed reads — a regime of the one rule,
  not a sibling mechanism.
- **world-cell-backed**: probe-measured; folds patrolled by the walls machinery
  (§5); transportable across contexts per backing invariance (§6).
- **never-settled-backed**: HARD-DEFERRED (human, 2026-07-12: "gargantuan meh" —
  no thought or tokens until a real book's `date` walls and it hurts). The candidate
  on the shelf, one line only: instability modeled as an at-every-site
  self-disturbing kind, expressible inside the existing ternary relation; the
  category belongs in this section of the engine when it is ever wanted.

## §4 — The validity table (which rules branch on which field)

- **fold-into-analysis** (arm-killing, operand concretization, locator
  classification): provenance ≥ world-spoken (register qualifies) ∧ backing fresh
  over the patrolled window (§5) ∧ no never-settled backing member ∧ shape within
  the single-line wire floor. The composed grade is excluded here by the interim
  reversible floor — `271:rul-composed-bytes-defer-and-floor`; field evidence
  decides whether that gate ever becomes permanent, in either direction.
- **warnings / why / hints**: every grade, every regime, maximal input, forever
  ungated (typed).
- **artifact-entering substitution**: postponed wholesale (the render steer); the
  floor pre-pins world-spoken-only for whenever it unparks.
- **license-plane participation**: value-predictions obey invited-rooms typing — a
  value learned by hint-lane descent (an unmarked `sh` payload) can never feed a
  fold regardless of provenance grade; hint-lane data widens scope, never narrows,
  never licenses (extends descend-don't-license and `274`
  finding-descent-edges-widen-only to values by construction).
- **cross-context transport**: by backing invariance, derived (§6) — no authored
  vocabulary member exists or is needed.

## §5 — Chronology

Freeze-at-binding is the language's own semantics: once bound, the bytes live in the
shell; consumers inherit the binding's value through already-modeled dataflow — no
per-consumer wall-checks. The patrolled window, stated with the human's care:
**apply-script-start → the apply-time binding line** — a closed world of VISIBLE
script-mutators; the probe→apply-start segment is the standing accepted TOCTOU
residual, same as every probed fact.

The co-valuation unfold (the admin-side derivation that re-grounded this design): a
dynamic-named guard (`PKG=$(cat /etc/pkg); dpkg -s "$PKG" || apt-get install -y
"$PKG"`) has TWO staleness axes — the ANSWER (ordinary walls on what the check
reads) and the QUESTION (the captured name itself). The fold freezes the question,
not just the answer; the repair is the same frame rule applied to the value's own
backing (did anything claim to disturb `/etc/pkg` in the window). Spoken admin-side,
no cells needed: "line N was dropped because line 1's read said X at plan time, X
checked converged, and nothing in this plan claims to touch either the file or the
checked state above line N."

Hazard flagged for mechanics time (the `219` silent-vanishing shape, new clothes):
eliding a capture-ASSIGNMENT line unbinds the variable for apply-time runtime
consumers — binding-site disposition needs its own care in block-context planning.

## §6 — Cross-context transport (the probe-outside chain for values)

> (STATUS 2026-07-13, per `279f` §3: **NOT RATIFIED — refused as posed at the
> crosscheck adjudication.** Premise 1 consumes the backing as a completeness claim
> the `:?` mark does not make (279b-fd1/279a-A2: a backing carries no completeness
> burden, `24D`); a body honestly marking one read while its output depends on an
> unmarked input transports a wrong value with no wrong line anywhere. The
> disposition — v1-defer / an authored completeness speech-act / effect-closed
> bodies only — is owed at block-context implementation-planning. Register-backed
> transport stands, analytic. Step 4 amended per `279f:fix-275-license-source`;
> the rest of the section is preserved as the proposal under adjudication.)

Fact-side refresher: probe-outside license = permission to treat an outside (alice)
measurement as being ABOUT the inside (root) object = same-object-whoever-asks =
derived user-invariance (`272`). The value-side chain, four steps:

1. The value is a pure function of the state its backing names
   (rul-measurement-is-authorship + honesty of the mark).
2. Same state ⇒ same value. (⇐ only — the converse is false and UNUSED: a keyed
   backing fails-to-license sameness, never proves difference;
   never-derive-separation's flavour at the value tier.)
3. "Same state whoever asks" on the user axis = user-invariance of the backing cell.
4. Invariance is established per `272` §3 AS AMENDED by
   `271:rul-invariance-speech-act`: the kind-owner's typed `invariant:<axis>` line
   (vouch-tier) plus engine-warranted carried-by rows; the r2 derivation
   contradiction-checks and never licenses. *(Amended 2026-07-13 — the prior
   wording, "invariance is already derived (state_stored_only_in × carried-by ×
   r2)", predated the task-8 re-role and read as derivation-licensed transport;
   279a-A1/Codex-fd4.)*

∴ **a value transports across the user boundary exactly when its backing does.**
This DISCHARGES `272` §11's expected "axis-independence value-bound in the
read-blessing vocabulary": derived, not spelled — the `$(brew --prefix)` locator arm
classifies through Homebrew's own store declarations. Counter-case runs the same
rails: `$(git config --global user.email)` is `$HOME`-keyed ⇒ same-context consumers
only; a root-context fold of an alice-measured value refuses, naming the keyed store.
The wrongness knife is the existing one (an omitted per-user store — pipx-shaped),
same tier, same attribution, no new blade.

## §7 — Imported doctrine (lives in `271`; applied here)

`271:rul-measurement-is-authorship` (world-knowledge = CFG + observables of authored
endpoints; the conductor's measured/predicted/analytic system-taxonomy is retracted)
· `271:rul-composed-bytes-defer-and-floor` (incl. judgments-not-facts: an oracle's
product is a judgment, deliberate helpful lies are the stdlib's founding transaction)
· `271:rul-sin-ordering` (the chain design's target: every wrong fold lands
attributed; the two cardinal/pope-sin threats — host-authored free text entering
program meaning, and engine string-semantics bugs — are floored and
postponed-with-fences respectively) · `271:rul-orthogonality-counterexample-test`
(method; first run produced §2's per-channel granularity).

Channel-uniformity (human suspicion, conductor-confirmed): text-observables ride the
IDENTICAL claim-side logic as rc-observables — license, backing, chronology,
transport, attribution; all prior logic holds. The channels differ only
consumer-side (bytes enter value-flow, string-semantics, naming, artifacts — each
individually fenced, floored, or postponed). The one standing channel novelty: rc =
the host answering multiple-choice among human-authored continuations; stdout = the
host AUTHORING content (the hostile-host hook, `plans/102`; host-minted bytes can
participate in cell-naming and, someday, artifacts — which is why the floor and the
fences exist).

## §8 — Customers, discharged or routed

- **read-value-slice** (`270:block-context`): inherits §4's table + the floor;
  single-line; its gating adjudication (this arc) is now delivered.
- **dynamic locator arms** (`272` §11): served by §6; the second-customer coupling
  closes.
- **predicted-output values** (`273` §7/§11): "Predicted and OutClaim are one
  value-plane species" CONFIRMED — output-predictions are the author-composed grade;
  the changed-detection fold's scope call remains unmade (not this arc's).
- **capture-ships-real-bytes** (`273` §6): consistent — the floor's world-spoken
  reading IS that sentence's disambiguation; task-14's gate unaffected.
- **stdin-code carriage** (`274` §11): rides the species without additions;
  UNWALKED in anger — flagged to block-context implementation-planning.
- **who-am-I** (`272` r2): the register regime (§3), not a separate route.

## §9 — The authored surface (the punchline: the empty set)

Admins: nothing — books unchanged; captures start working. Tool and stdlib authors:
nothing NEW — ordinary `cmd__predict()` arms, the ruled per-channel idiom vocabulary,
`:?` observe marks: all grammar that exists from tasks 1–6. The capture lane is
entirely a CONSUMER of existing speech-acts. The only prospective vocabulary in the
whole design: the never-settled bit's minting home, deferred with its candidate (§3).
For kBURDEN this is the arc's headline: the answer to "what must an author write to
get captures?" is "what they already wrote."

## §10 — Failure modes and honest residue

- **minting-tax**: fold-past-walls needs an observed coordinate on the producing
  arm; junk-cell cargo-cult pressure is real; a WRONG backing is knife-tier via
  survival. Bounded: absent marks cost only value (the positional floor — fold only
  when nothing effect-bearing interposes probe→binding — still rescues book-top
  captures, the flagship `case "$(hostname)"` included).
- **too-pretty meta-flag**: the backing candidate absorbed every objection raised
  (clock, who-am-I, two-jobs); earmarked for the adversarial crosscheck riding the
  entity-algebra note, exclusions-not-inclusions framing, conductor's self-flagged
  weak points stripped from the packet.
- **hint-descent quality**: hints derived from misparsed unlicensed payloads can
  mislead (kWARN-tier, never correctness).
- **single-line wire floor**: multi-line/binary captures refuse ⇒ site stays ⊤ ⇒
  runs.
- **binding-site elision hazard** (§5): owed care at mechanics time.
- **the composed gate**: deferred by ruling; the reversible floor holds it open
  without deciding it.

## §11 — Routed consumers

read-value-slice brief (the table, the floor, single-line) · value-recipe-reshape
brief (per-channel backing derivation; the provenance slot; pipeline-order +
literal-provenance seams — `271`'s rider) · task-12 entity-algebra design note
(the backing-inheritance candidate for formal ratification; the backing-SETS seam;
the `272` fence and carve unchanged) · task-8 (the invited-rooms-values coupling:
hint-lane values never feed survival; plus the standing re-read of adjudicability
against derived-not-declared topology) · the render-unpark fence (world-spoken pin)
· stdlib quality-bar (delegation-vs-composition as teaching, never gate).

## §12 — Status table

| component | status |
|---|---|
| the species + the name value-prediction | TYPED 2026-07-12 |
| fields: derived provenance grades; per-channel backing sets | TYPED (the assembled-ruling full ack; granularity via the orthogonality-test run) |
| three regimes by backing-class | direction-tier ack (hedged: "barely understanding, but satisfied"); formal ratification rides the entity-algebra note + its crosscheck |
| the validity table | TYPED (full ack); composed-gate row = `271:rul-composed-bytes-defer-and-floor` |
| chronology: freeze-at-binding; patrolled window apply-start→binding | TYPED (window terminology per the human's nit) |
| cross-context transport chain; `272`-customer discharge | direction-tier (same hedge); rides the note |
| never-settled/clock | HARD-DEFERRED (don't-even-mention tier; shelf entry only) |
| authored-surface-empty | consequence-tier, verified against the task-1–6 grammar |
| stdin-code carriage | routed to block-context planning; unwalked |
