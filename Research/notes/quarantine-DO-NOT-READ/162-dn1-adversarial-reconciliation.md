# 162 — dn-1 adversarial reconciliation: command-centric → fact-centric pivot

> **Status (2026-06-05): spike ph-1 follow-up.** Reconciles a neutral +
> disowned-adversarial cross-check (the `adversarial-crosscheck` method) of the
> 161 dn-1 strawman, both clean-context, both grounded in 099/09A/KNOBS, both with
> counterexamples run under `/bin/dash`. **Convergence is the signal**; tone/verdicts
> discarded. Each finding kept here is either run-by-me, run-by-an-agent-and-
> reproducible, or true-by-inspection — noted per item. **Supersedes 161 §2/§4/§6**
> (the command-centric mechanism); keeps 161 §1 (constraints) and §7 (alternatives).

## 0. The pivot (the load-bearing correction)
161's strawman made the probe a **dry-run of the mutator** (`apt-get --simulate
"$@" | grep …`). That is *command-centric* and it is the wrong abstraction. The
deepest finding (adv break-12, true by inspection): `oracle_kind`/`oracle_verb`/
the entity appear **nowhere** in `oracle_probe`'s logic — the skip rides on a
command-local dry-run, so the cross-oracle **kind-index (the dn-1 deliverable) is
unused by the mechanism that actually elides.** 099 §4 C1/C2 + 160 occurrence-
triple demand the opposite: the oracle supplies a **fact-predicate** (a latent
proposition over a *named kind*), and mutators **declare effects** on that fact.

**v2 is fact-centric:** an oracle declares (a) a named **kind**, (b) a read-only
**fact-probe** that observes whether `kind:entity` holds (three-outcome), and (c)
an **effect map** — which (provider, verb) establishes/kills the fact. The skip is
licensed by *the fact already holding* (per the probe) **and** the analyzer's
ambient∧invariant gate — never by re-running the mutator in dry-run.

```sh
# oracle: package (Debian dpkg/apt).            [STRAWMAN v2, fact-centric]
oracle_kind=package                              # the named kind (W4; literal; one per file)

# FACT-PROBE: does `package:<entity> = installed` hold? read-only, THREE-outcome
# (rc 0 = holds/converged · 1 = absent/diverged · 2 = unknown/can't-tell).
# Note: NOT a pipe-into-grep (that idiom structurally cannot emit rc 2 — see F-1);
# it captures the tool's own status and a missing tool => Unknown.
oracle_probe_package() {                         # $1 = entity
   command -v dpkg-query >/dev/null 2>&1 || return 2
   st=$(dpkg-query -W -f='${Status}' "$1" 2>/dev/null) || return 1
   case $st in 'install ok installed') return 0 ;; *) return 1 ;; esac
}

# EFFECT MAP: accumulating calls (NOT reassigned vars — multi-verb without clobber).
oracle_effect apt-get install establish          # `apt-get install <ents>` establishes package:<ents>
oracle_effect apt-get reinstall establish
oracle_effect apt-get purge   kill               # `apt-get purge   <ents>` kills it
oracle_effect dpkg    -i      establish
```
The analyzer lifts: `kind=package`; the `oracle_probe_package` body (shipped as
the read-only probe, per-entity); and an accumulating set of
`(provider, verb, polarity)` effect-tuples. A book mutator `apt-get install -y
nginx` ⇒ effect `establish package:nginx` ⇒ skip iff `oracle_probe_package nginx`
== converged **and** `package:nginx` is ambient∧invariant at that point.

## 1. What the pivot dissolves (verified breaks, by theme)
- **F-1 verdict-channel can't emit Unknown** (neutral gap-toolfail-converged + adv
  break-4; **run by me** under bash, semantics POSIX-identical): `cmd | grep -q …
  && return 1; return 0` maps *tool-failure* → `Converged` (grep's empty-stream rc
  1 masks the tool's 127; `pipefail` does **not** save it — grep is the rightmost
  non-zero, so the pipeline is still 1, and `&&` still falls to `return 0`). The
  `cmd|grep -q` idiom is *structurally* two-outcome. → v2's fact-probe captures the
  tool's own rc and returns 2 (Unknown) when it can't run. *Dissolved.*
- **F-2 probe ≠ mutator-with-a-flag** (neutral gap-no-probe-flag/gap-no-readonly;
  adv break-3/5): most tools have no parseable dry-run; the read-only check is a
  *different command* (`dpkg-query`, `systemctl is-active`), and `--simulate`'s
  changeset vocabulary is verb-specific (a `start` verb prints `Started`, not
  `Inst`, → no grep match → false-Converged, break-3). → v2 declares the probe
  *separately* from the mutator. *Dissolved.*
- **F-3 multi-verb / opposite polarity + the §6 clobber** (neutral gap-multiverb;
  adv break-11): one `oracle_verb` + `by_provider: Map<ProviderId, ProviderDecl>`
  is the **same 1-place-namespace clobber** §7 rejected naming-conventions for —
  `apt-get install` and `apt-get purge` collide on key `apt-get`. → v2's effect map
  is keyed by `(provider, verb)` and accumulates. *Dissolved.*
- **F-4 kind-index decorative** (adv break-12, **inspection**): the dry-run probe
  never consults the named kind. → v2's skip is licensed by a fact keyed on the
  named kind; the index is load-bearing. *Dissolved.*
- **F-5 cross-kind false-link** (adv break-8/9; break-9 **run** via a hand-built
  `./fakebin/nginx`): 161 §2.1 claimed the oracle's effect-decl "links the guard's
  `nginx` to the package entity." But `command -v nginx` observes kind `tool`
  (PATH-executable), `apt-get install nginx` establishes kind `package` — *different
  kinds*. Propagating one as the other is unsound (a binary on PATH ≠ apt package
  installed). → **correction:** the named-kind model already forbids this — `tool`
  ≠ `package`, so there is **no cross-kind fact propagation**; the book's own
  `command -v` guard is honored locally as the user's best-effort intent (DESIGN),
  but its fact does **not** discharge a `package`-kind establish elsewhere. 161
  §2.1's "links … for cross-statement reasoning" prose was wrong; delete it.
  *Dissolved by applying W4 strictly* (not by new machinery).

## 2. What the pivot does NOT fix — remaining open problems (the real ph-1 output)
- **O-1 the ambient∧invariant gate is the ANALYZER's job, and 161 omitted it**
  (adv break-10, the 099-W5 "*the* wrong skip"; design-logic): `apt-get purge -y
  nginx; …; apt-get install -y nginx` — probing `install` against *initial* state
  (nginx present) ⇒ Converged ⇒ elide the install, but the upstream same-run purge
  removes it by apply-time. The oracle cannot see this; the **skip must be gated by
  reaching-defs / gen-kill over the fact** (160 hoist-predicate: hoist iff ambient
  ∧ invariant ∧ no in-script gen/kill reaches it). v2's effect map *is* the gen/kill
  the analyzer needs — so this is buildable, but it lands in `analysis`, and the
  contract must state the skip rule *gated*, never unconditional.
- **O-2 kFAIL-withhold is NOT enforced by the frame** (adv break-1/2, break-1 is the
  apt `-o` re-arm, break-2 a `docker create` "probe"): a frame-clean oracle can ship
  a probe body that *mutates*. The verdict frame machine-checks the *anchors*, not
  that the probe body is inert. The real defense is a **separate** mechanism (prove
  the probe calls only declared-inert ops, or sandbox/observe it — 077 seccomp
  backstop) — distinct from the verdict channel, and unbuilt. The contract must
  stop claiming "machine-enforceable non-mutation"; it enforces the *shape*, not
  inertness. (For the spike with no host: the `hostsim` can *detect* a probe that
  attempts a modeled mutation and fail the test — a DST check standing in for the
  real sandbox.)
- **O-3 entity-extraction is sound-XOR-useful** (adv break-6/7; neutral gap-flagstrip-
  eats-operand): ⊤-on-unknown-flag is *safe* but ⊤s the fixture's own common lines
  (`systemctl enable --now nginx`: is `--now` arg-taking? `apt-get install -y
  --reinstall nginx`), gutting the value-prop; and the precision lever that fixes it
  (declare the flag grammar) re-arms break-1's `-o` mutation. There is no
  free-and-sound generic splitter — per-provider flag grammar is required and is a
  real authoring burden (X3 "footprint is a mini-parser"). Spike stance: ⊤-conservative
  default (accept the lost skips), measure how often it bites on the fixtures (that
  measurement *is* a finding).
- **O-4 partial convergence** (neutral gap-partial-convergence): `apt-get install
  nginx curl jq` is one leaf over a *set*; `core::Verdict` is one tri-value. Per-entity
  verdicts need the leaf to fan out per entity (the leaf-seam can do this) or the
  verdict to carry a per-entity map. Reserve; don't pretend one bit covers a set.
- **O-5 kOOB ruling still needed** (adv break-13, neutral gap-koob; 161 find-1): is
  `oracle_kind=package` "config-in-disguise"? The adversarial sharpening: it *is*
  isomorphic to a YAML key, carries no control-flow, and taints nothing — so AGENTS'
  "intent from control-flow-tracing/tainting" doctrine cuts against it; and the
  off-ramp story is "delete the anchors," i.e. a Dorc-specific spelling. **Genuinely
  needs the human's ruling** (lean: allowed-as-a-pragmatic-spike-choice, but flag it
  as the weakest point against the no-cliff/off-ramp thesis). The lift binds ONLY to
  the assignment/`oracle_effect`-call AST, **never** the `# oracle:` comment
  (comment-parsing would be a clear kOOB violation — break-13 t-comment).
- **O-6 oracle/book recognition + anchor-namespace collision** (neutral gap-multi-decl,
  gap-oracle-and-book, gap-anchor-namespace): "a file is an oracle iff it sets the
  anchors" mis-handles (a) a file that's both library + top-level mutators, (b) a
  benign script that happens to use `oracle_kind` for its own purpose, (c) two
  providers in one file. Spike stance: an oracle is a file whose top level is
  *only* anchors + probe/effect declarations + function defs (no top-level
  mutators); one kind per file; collision accepted as a known false-positive risk.

## 3. Held-up (don't re-litigate)
- 161 §2's blessed-anchor idiom genuinely passes `dash -n` and runs inertly
  (both agents confirmed; the dotted `apt-get.check()` control fails as X4 said).
  v2 keeps plain assignments + plainly-named functions; **off-ramp survives** (now
  *better* than 161: `oracle_probe_package`/`dpkg-query` are hand-callable, partly
  answering find-5 — though `oracle_effect` still needs a 2-line no-op shim).
- §1 constraints (3-place relation, in-band, off-ramp, machine-frame-not-discipline)
  and §7 alternatives (naming-convention/registration/infer-from-body) stand.
- The named-kind model is *correct* and, applied strictly (F-5), is what makes the
  whole thing sound; the failure was the command-centric probe, not the kind model.

## 4. Build implications (what `oracle` + `analysis` must now carry)
- `core`: add `KindId(Symbol)`, `ProviderId(Symbol)`; add a **selector** to `Fact`
  (`svc:nginx#enabled` ≠ `#active`; 099 §8 / 160 fact-pair — 161 §6 wrongly dropped it).
- `oracle`: `KindDecl { kind, probe_body }`, an `EffectMap` of `(ProviderId, verb)
  → (KindId, Polarity{Establish,Kill})`, accumulating; lift from the v2 idiom;
  ⊤/diagnostic on non-literal anchors or top-level mutators.
- `analysis`: the skip rule is **fact-holds (probe) ∧ ambient∧invariant (gen/kill
  over the effect map) ∧ MUST-grade** — the O-1 gate lives here, not in the oracle.
- `hostsim`: must be able to (a) answer a fact-probe deterministically against a
  modeled system-state, and (b) **detect a probe that attempts a modeled mutation**
  → fail the test (the O-2 kFAIL-withhold check, standing in for the real sandbox).
