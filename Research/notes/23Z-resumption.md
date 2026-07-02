# 23Z — r23 resumption / keep-alive (for the next conductor)

Resumption prompt for r23 (the CERTAINTY axis / best-effort-gradients round; charter `plans/230`).
The arc opened — the §1 sweep and the design synthesis landed — and the next build-step (xfail-first)
is **designed but unwritten**, living only in a conversation. This captures the status + the concrete
next-up work so a fresh conductor doesn't re-derive it from chat. AI-authored; trust the root docs and
`plans/230` / `notes/231` / `notes/232` over this where they conflict.

## On-ramp — read first (so this prompt stands alone)

If not already in context, get your bearings in this order (this mirrors `AGENTS.md`'s reading-guide):

1. **Human-authored ground truth** (repo root; do NOT edit — suggest fixes to the human): **`IMPLEMENTATION.md`**
   — especially the "Correctness vs. best-effort: a band" section, which *is* the r23 frame — and
   **`DESIGN.md`**; then **`KNOBS.md`** + **`ANALYZER-NEEDS.md`** as the shared vocabulary (reuse their
   `k*` / `an-*` slugs; don't re-derive a tension under a new name). `README.md` / `TODO.md` for completeness.
2. **`Research/README.md`** — the one onboarding always-read: current state + how to navigate the (deep,
   noisy) planning corpus.
3. **The r23 core, in order:** **`plans/230`** (the charter/seed — the whole round) → **`notes/231`** (the
   sweep walk-back map) → **`notes/232`** (the design synthesis; it *corrects* `231` §4 — heed the
   `<!-- … -->` pointer there).
4. **The build reality you act on:** **`spike/CLAUDE.md`** (the welded `inv-*` invariants + standing rulings
   — non-negotiable) → **`plans/16P` §3** (the built-vs-designed ledger) + **`16Q`** (the precision keystone)
   → the spike-3 / take-3 closes **`19I`** (hand-forward) / **`20K`** / **`21W`** / **`22W`** (the CURRENT
   spike state) → **`spike/e2e/run.sh`** (the harness contract you'll author xfail cases against).
5. **Deeper soundness grounding** (consult as the work demands): **`plans/055`** (the two phase-keyed
   soundnesses; the MAY/MUST lattice) + **`plans/099`** (MUST-vs-MAY; the decidable floor) + **`plans/111`**
   (the error/provenance spine — r23 is the *decision-side* counterpart to its *reporting-side*).

## Status of the arc (charter §6: xfail-first → design → adversarial-crosscheck → build)

- **DONE — §1 collapsed-gradient sweep** (8-agent fan-out) → **`notes/231`**. The walk-back map: 6
  gradient-clusters ranked by lock-in — `1a` trust-cell (HIGH; *build*, not un-collapse — 0 live
  decision-edges) · `1b` cardinality-strong-update (HIGH; the gate is *absent*, not boolean) · `1c`
  coverage-vouch-default (MED; the headline) · `1d` multicell-classify-cliff (MED; shovel-ready) · `1e`
  partial-member (MED; broadest convergence) · `1f` door3-recovery-dormant (LOW) — plus the
  must-stay-boolean fence, the **THIN** trust-decision-surface verdict (0 live edges / 2 latent), and the
  channel-vouch (23-c2) finding.
- **DONE — design synthesis** → **`notes/232`** (completeness-vouch + the oracle observable-declaration
  surface). It **corrects `231` §4** in three places (a `<!-- -->` pointer is in `231` §4): `oracle_effect`
  is an OPEN spelling strawman (not settled, not vetoed-and-dropped); `dc-elide-on-trusted-default` SPLITS
  by channel-nativeness (native sh-idiom vouches vs the invented effect-cell spelling); oracle-spelling ⊥
  book-spelling (central declaration, not a cross-actor gap).
- **NEXT (designed, NOT written) — the xfail-first executable spec** (the set below).
- **PENDING — design → adversarial-crosscheck → build**, walking back the `231` collapsed booleans.
  Multi-round per charter §7; r23 only *opens* the arc.

## Next-up: the xfail-first set (charter §6 — pin the desired gradient *behaviours* as FAILING tests before any type)

Harness contract (`spike/e2e/run.sh`): a case is `cases/<name>/` = `book.sh` + `*.oracle.sh` +
`probe-results.txt` (+ optional `mocks/` + `expected.ran` + `expected.out`), through 8 gates (`dash -n`,
apply/probe exec run-sets, the argv-echo + dual-rail license differentials, stderr-error floor, why-lens).
An **`XFAIL`** file (1st line = reason) pins the SAFE/desired behaviour and is expected-fail at HEAD; a
surprise pass is a loud **XPASS-to-promote**. There are **zero xfail cases at HEAD** (all promoted).

Three flavours: **XFAIL** (desired-not-built) · **PASS-guard** (the safe floor / recovery that already
works — pinned so a future gradient can't silently break it) · **Rust** (engine-internal, no clean e2e
surface).

### Buildable now (native channels + engine mechanics — not gated on the open oracle-spelling)
- **`pin-consumed-stdout-runs`** (PASS) — a book capturing an oracled mutator's stdout, no vouch → **runs**. The safe floor.
- **`xf-vouch-stdout-bodyredirect`** (XFAIL) — the oracle vouches stdout-empty via the body-redirect `…() { … } >/dev/null` → should **elide**. Native sh idiom (POSIX-standard, demonstrated); off-ramp-clean; NOT `dq-kOOB`-gated (`232` §3/§7).
- **`pin-ortrue-toplift-recovers`** (PASS) — `cmd || true` with a ⊤ left doesn't block (door-3 `StatusInvariant`; already works — `231` 1f).
- **`xf-ortrue-widen`** (XFAIL) — door-3 widened to `|| :`, `|| true >/dev/null`, `|| { :; }` (currently bare-`true`-keyword only, `cfg.rs` `right_is_bare_true`).
- **`xf-andor-both-agree`** (XFAIL) — `eval_and_or` recovers a known rc when both operands agree (currently drops to ⊤; `fold.rs` ~`:267`).
- **`pin-probe-safety-boolean`** (Rust PASS) — probe-construction never ships a probe lacking a declared-inert basis, regardless of any trust/confidence input. The **`dc-probe-NOT` red-XPASS tripwire** (must-stay-boolean; `rul-mutation-impossible`).
- **`xf-multicell-elide`** (XFAIL) — a `(provider,verb)` with ≥2 establish cells, all converged+ambient → **elide** (currently `_ => MustRun`, `effect.rs` ~`:1091`). Shovel-ready (`231` 1d).
- **`xf-partial-member-elide`** (XFAIL) — `apt install nginx curl jq`, one member diverged → elide the converged subset (currently runs all). **NB the BUILD needs a separate per-member self-reach analysis, not an in-place softening** (`231` 1e — the all-or-nothing self-reach is a fixed-point argument).
- **`cardinality`** (Rust, two pins) — strong-update on a provably-unique singleton (XFAIL; the gate is currently *absent*) + weak/⊤ on an aliased entity `for h in $hosts; install pkg:$h` (PASS-guard — the §5 danger-direction). `231` 1b.

### Wait on the oracle-spelling (behaviour stable; concrete oracle-syntax unsettled — `tc-vouch-surface` / the open `dq-kOOB` question, `232` §8)
- **`xf-vouch-effectcell`** (XFAIL) — a modeled-effect **completeness vouch** lets a converged leaf elide where an unvouched gap runs (the `231` 1c / `232` §4 headline). The *behaviour* is the r23 target; the *spelling* is the live design question — author this case against whatever the oracle-contract settles to. Do not pin it on a strawman spelling that will move.

### Hold for the design→crosscheck step (the riskiest rung)
- **`xf-disagreement-prefer-probe`** (XFAIL) — a trust-aware `merge_observable` prefers a probe-**OBSERVATION** over an oracle-**CLAIM** (currently collapses a same-cell disagreement to ⊤⇒run). This moves *away* from run — the `inv-kfail`-forbidden direction unless the source provably dominates. Pinning it presupposes a soundness-critical lattice ordering (`tc-disagreement-rung`, `231` §5); **design it adversarially before xfail-ing it.**

## Pending human decisions / tc-flags (surfaced, not resolved)
- **`tc-vouch-surface`** (`232` §8) — the effect-cell completeness-vouch spelling (the open `dq-kOOB` question; gates `xf-vouch-effectcell`).
- **`tc-disagreement-rung`** (`231` §5) — the probe-observed-vs-oracle-claimed lattice ordering (gates `xf-disagreement-prefer-probe`).
- **`tc-cardinality-strong-update-rung`** · **`tc-multicell-aggregate-grain`** · **`tc-partial-member-self-reach`** (`231` §5) — the soundness-critical rungs behind 1b/1d/1e.
- **`tc-one-observable-build-vs-spec`** (`231` §5) — `inv-one-observable`'s "check() predicts per-channel values" text vs the build (check() resolves identity only). A welded-invariant text-vs-code divergence — for the human.
- **unseeded-hunt recovery** (`231` §5) — the §1 sweep's `unseeded-hunt` agent was harness size-capped (1 of a claimed 8 candidates survived; the other 7 are lost in its transcript). Re-run with a smaller per-candidate budget if the design phase wants the full unseeded set.

## Method reminder (charter §6 + the standing process memories)
xfail-first → design the gradient(s)/lattice → **adversarial-crosscheck the design** (post-Fable: Opus
same-model crosschecks; lean *harder* on them since there's no higher tier to catch cross-cutting error;
frame with exclusions-not-inclusions) → build, walking back the `231` collapsed booleans. Re-test every
proposed gradient against the welds: **`inv-kfail`** (a gradient adds precision only in the SAFE direction —
toward run — never licenses less-safe-than-the-boolean-floor); **`ru-11`** (trust/taint that drives a
decision is a SEPARATE decision-plane cell, never the receipts); the **exclusion-check** (probe-safety
stays boolean — no confidence threshold ever ships a probe).

## Pointers
charter `plans/230` · walk-back map `notes/231` · design `notes/232` · harness `spike/e2e/run.sh` · build
reality `plans/16P` (§3 built-vs-designed ledger) + `16Q` + the spike-3 closes `20K`/`21W`/`22W` · the
welded invariants `spike/CLAUDE.md` (`inv-kfail`, `inv-must-may`, `inv-probe-sourced-values`,
`inv-one-observable`, `ru-11`, `rul-mutation-impossible`).
