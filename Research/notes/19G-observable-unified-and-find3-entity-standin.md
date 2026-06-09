# 19G — one coherent Observable (landed) + the find-3 entity-resolution stand-in

> Append-only round-19 note (continues 193→19F). Priority-1 (the clean rip-and-replace to
> ONE `Observable`, 19F) landed its first half; a human steer surfaced `find-3`, the
> engine-infers-entity-identity stand-in. AI-authored, confidence-marked. Trust
> R/D/I/K + the human rulings over this.

## 1. Commit A — the one-Observable unification (landed, behavior-preserving)

+SURE (committed `f148a31`; 137 cargo tests + 43 e2e green). The three incoherent types
(19F §1) — `cfg::Observable` (consumption-liveness enum), standalone `core::Verdict`,
`Observed{verdict, rc}` (the bolted rc) — are now ONE `core::Observable{effect: Verdict,
status: Predicted<Rc>}` over a **closed** `Channel` vocabulary `{Effect, Status,
AndOrStatus, Stdout, Stderr}`. `Predicted<T>{Value, Top}` replaces `Option<Rc>` (and is
the fold's former `AbstractRc` by another name). The fold reads only `Observable.status`;
`prove_replaceable` gates on the Effect (verdict) + Status (`Predicted<Rc>`) + consumed
channels. cli/hostsim do not fabricate rc (already true pre-A; preserved).

Deviations from 19F's literal `{Effect, Status, Stdout, Stderr}`, recorded:
- **`AndOrStatus` kept as a 5th `Channel` variant** (not collapsed into `Status`). The
  collapse 19F implies would move the if-guard render-floor (a `Status` consumed in an
  `if`/`elif` blocks the license unconditionally — the line-granular render can't
  substitute a guard in-situ) OUT of channel-identity and INTO the render. ~SUSPECT
  cleaner, but a real behavior move (regression-risky), so A keeps both variants
  behavior-identical; the collapse is deferred to a deliberate pass (the `ch-wrong`
  bake-and-see: did it work, or is render-expressibility genuinely a channel-adjacent
  property?).
- `Verdict` survives as the Effect channel's *value type* (not a separate probe-reported
  concept) — satisfies 19F's "convergence is the derived Effect-channel state" without
  deleting the enum.

The under-execute (strain-B) was ALREADY closed pre-A (the `19D` gate + un-fabricated rc);
A is a coherence refactor, not a bugfix. "Done" for the rewrite = preserve that, with the
rc flowing from the oracle (Commit B), not a fixture.

## 2. find-3 — the engine-infers-entity-identity STAND-IN (human-surfaced)

+SURE: `effect::command_effect` (`verb = word-1`) + `effect::resolve_entity` (flag-strip
on `-`-prefix + the one-non-flag-operand rule) are the engine INFERRING entity identity.
Two problems:
- it mis-parses no-verb providers — `useradd deploy` ⇒ verb=`deploy` ⇒ Opaque (find-3
  proper); and, more deeply,
- it **breaks a welded principle**: *identity is declared, never inferred* (SF-1 /
  `an-entity-uniqueness` / `17N F3`) and `inv-referent-agnostic` (the engine reading
  argument *structure*).

**The settled mechanism (human-confirmed; `ch-shape-anno` + the foundational
compiled-probe).** The engine does ZERO argparse. An oracle writes a mini-argparse in our
*constrained* oracle-contract dialect — explicitly NOT arbitrary sh; a middle-ground that
feels/works like sh but is reliably liftable — and **inline-annotates** the operand's kind
(`pkg : com.debian.apt.Package = "$1"`). The engine **flow-tracks the book's constant**
(`nginx`) *through the oracle's argparse* (the `while`/`shift`/`case`) to the annotation,
binding the opaque token → kind. The engine never decides `-y` is a flag — the oracle's
argparse does.

**Why it's a stand-in, not built (+SURE, traced):** the spike has (a) **no value-plane**
(`16C` refuted analyzer-side value-synthesis; the flow-track is a *narrow, decidable*
re-introduction — constants + simple `$n` only, NOT the refuted general case), and (b)
**detached oracle bodies** (`lower_funcdef` builds them with no call-edges / param-binding
— `seam-interproc`), so there is no path from the book's call into the oracle's `$1`. The
crude inference fakes the result.

**Human ruling:** keep the stand-in for THIS spike ("the stand-in needs to stand"); but
**annotate every inference site as temporary identity-taint** (done — greppable `find-3
STAND-IN` markers in `effect.rs`), so its temporariness + the value-plane cause are
explicit. The proper fix (annotation-parse + bounded constant-flow through the oracle
argparse) is the keystone's *input* side — ~SUSPECT more central than the Observable
layer's tail, deferred here.

## 3. Forward (the Observable layer, on the stand-in)

- **Commit B** — the oracle declares a **converged-rc** (the rc a command returns when its
  fact already holds; default 0 conforming, e.g. 9 non-conforming), threaded
  effect-map → `command_effect` → `SkipClass` → `build_plan` → the Status channel, so the
  rc *flows from the check* (19F §6 anti-masking). The acceptance suite is written
  RED-first; no test hand-injects an rc the check should predict. The canonical case uses
  verb-shaped `wombat`/`hork` stubs (the stand-in cannot resolve `useradd`'s no-verb
  shape — recorded here, not worked around in the engine).
- **Commit C** — taint (`Grounded`/`OracleConditional`) on the oracle-claimed Status
  channel (and the find-3 inferred entity), per `ch-priority` #2 / `16P T12`.
- Then resume corpus + Half-B on the coherent base.
