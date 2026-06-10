# 19A — Round-19 wave-1 wrap + the five human design-corrections (the rc-modeling root)

> Re-seed orchestrator note. Wave-1 (gw-1 Half-B/F1, gw-4 corpus, the adversarial-crosscheck, gw-5
> fixes) outcomes, then the **five design corrections the human gave this session** — the durable,
> high-value part. AI-authored, confidence-marked. Trust R/D/I/K over this; the corrections below are
> the **human's rulings** (treat as authoritative direction, though their phrasing here is mine).
> Continues 197/198/199.
>
> **⚠ DESIGN CHANGED DURING THE SESSION — read §5 first.** The C-2/C-3/C-4 framing in §3 was *refined
> and partly superseded* later the same session (§5). In short: C-1 and C-5 stand; C-2/C-3's "track
> rc-polarity per consumer" **dissolves** into plain abstract-interpretation over probed values; and
> C-4 is refined from "oracle-contract the rc *directionality*" to "oracle-contract the *observables a
> command produces* (incl. its exact rc value)." §3 is kept as the record of how the understanding got
> there; §5 is the corrected model.

## 1. Wave-1 outcomes (all green: 41/41 e2e, workspace + clippy clean)

- **gw-1** (`fd9d0e0` F1 stopgap, `85c48a4` Half-B.1, `c408afd` notes/198): F1 under-execute fixed
  (guard-status blocks elision; `guard-elision-wrong` xfail → passing `guard-status-blocks-elision`);
  Half-B.1 = operand-bound FLAT interceptors; Half-B.2 (subsumption) NOT built, wall mapped to 3
  deferrals (query-category/`inc-7`, occurrence-typing/`inc-6`, structural-render). **Note §3 below
  reframes gw-1's F1 + interceptor as *misframed special-cases* per the human.**
- **gw-4** (`d057797`: 13 non-guard strain cases + notes/199): corpus 26→39. **Frame-inversion
  finding:** the non-guard surface is mostly GREEN — `notes/197 §5`'s "gaps" were test-*coverage*
  gaps, not engine-*capability* gaps. Genuine strains: `strain-R` (line-render mangles a one-line
  `case`-arm — render-fidelity, distinct from F1's classification layer), `strain-E` (the whole
  errexit precise-edge subsystem is e2e-invisible in apply-2 until the backward slice consumes it),
  `strain-S` (lifter checks probe-completeness per-file not per-kind ⇒ spurious `oracle-missing-probe`),
  and `. /etc/profile` is `Opaque` (sourcing a helper defeats downstream elision).
- **adversarial-crosscheck** (kp-1, un-seeded pair, both self-corrected a stale-base worktree to assess
  `c408afd`): **convergent** — F1 is a *structurally-safe tightening* (only ever adds to the block-set,
  read `May` ⇒ can only block, never skip; pre-fix counterfactual wrongly-elided, post-fix runs); S2
  (body-in-guarded-branch) sound; the interceptor render works. **The adversarial pass OVERTURNED the
  "&&/|| safe-for-conforming-oracles" disposition** (gw-1's, the neutral pass's, AND the orchestrator's):
  it proved via the CLI that a non-conforming establish (`useradd` rc 9, `mkdir`/`ln`/`docker network
  create`) as an `&&`/`||` left operand wrongly gets `Replace`-d ⇒ `useradd || mkdir` skips `mkdir` (a
  priority-1 `kFAIL-perform` under-execute). New convergent find **F-QUOTE** (unquoted operand in the
  interceptor render — wrong-entity + a `kFAIL-withhold` probe-injection). Adversarial-only (~SUSPECT,
  unverified): the subsumption wall may be *softer* than gw-1 mapped (same-cell narrow case buildable on
  existing primitives) — flagged, not banked.
- **gw-5** (`820b730` F-QUOTE shell-quote, `d42876f` doc, `ecf48da` under-execute pin): F-QUOTE fixed
  (single-quote-always, 28 goldens re-blessed render-only); the &&/|| under-execute pinned at the
  *disposition* level (`observable_matrix.rs` `#[should_panic]` xfail) + a render-coupled e2e tripwire
  (`andor-rc-vouch-wrong`), because the line-render *masks* the under-execute (keeps the `||` line
  verbatim). Harness limit surfaced: `cli parse_results` keys verdicts by first whitespace token ⇒ a
  spaced operand folds to `Unknown ⇒ Run` (safe).

## 2. Process datum — the practice earned its cost again

Three optimistic reads (gw-1's strain-log, the neutral crosscheck, the orchestrator's relay to the
human) all bought the same false assumption — "converged ⟹ rc 0" — and the *adversarial* pass, told to
distrust it, found the common trigger (`useradd`/`mkdir`) and proved it. Near-exact replay of the
round-19 `strain-8` datum. **`an-probe-shape`/`DP-3` ("capture the tool's *own* rc") was in the corpus
all along; the spike's elision still assumed rc-0-success.** Keep aiming `/adversarial-crosscheck` at
*invested* conclusions, un-seeded.

## 3. The five human design-corrections (this session) — the durable deliverable

These are the human's rulings; they reframe wave-1 and set wave-2's shape.

- **C-1 · the probe interceptor must pass the command's FULL argument string** (human, +SURE
  ruling). Dorc must NOT extract a subset of a command's argv (`command -v nginx` → `…check nginx`
  drops `-v`; `apt-get install -y nginx` → `package__check nginx` drops `install -y`) — that assumes
  the command's private arg-grammar Dorc cannot know (the `F7` argparse-zoo). The oracle's `check()`,
  which alone knows its command, is invoked with the **full** argv and does any extraction. The
  strawman already shows this (`id__check -nG deploy`; `id__check(){ command id "$@"; }`). **gw-1's
  kind-keyed, Dorc-extracted-entity render (`package__check nginx`) VIOLATES this** — it is the
  dangerous assumption. *Pending arg-grammar tooling (`q1-flaggrammar`), full-args is the only safe
  form.* Open: **q-1a** (does cell-keying / `resolve_entity` also stop relying on extraction, or only
  the probe invocation? — ~SUSPECT only the probe invocation for now); **q-1b** (mutator case: the
  check becomes provider-keyed `apt_get__check install -y nginx` with the oracle parsing apt's grammar
  to reach `dpkg-query` — confirm the shape).
- **C-2 · `if/then` ≡ `&&`; the real axis is rc-polarity, tracked in the CFG** (human, +SURE).
  `if cmd; then B` ≡ `cmd && B`; `if ! cmd; then B` ≡ `cmd || B`. gw-1's F1 fix special-cased
  `if`/`elif` (block status) vs `&&`/`||` (unmarked) — that *split of one phenomenon* is exactly what
  left the &&/|| under-execute. The difference is **boolean polarity** (`!`, `&&`-vs-`||`), which
  belongs in the CFG as a per-consumer **rc-expectation** (success-expecting vs failure-expecting);
  eliding a status-consumed command must honor its expected rc. One uniform rule subsumes F1 + the
  &&/|| finding. gw-1's two special-cases are a band-aid for this general model.
- **C-3 · errexit is honored, not special-cased-as-vouched** (human, +SURE). Under `set -e` every
  command's rc is consumed (non-zero ⇒ abort) = an implicit success-expecting guard per line.
  gw-1's "errexit-status stays vouched, still elides" carries the identical rc-vouch unsoundness
  (eliding `useradd` to `:`/rc-0 hides the abort it would raise). Currently *masked* (`set -e` is
  `Opaque` ⇒ poisons downstream ⇒ nothing reaches the status question — gw-4 `strain-E` / neutral
  pass) so it is a latent design error, not a live bug — but the framing is wrong. errexit is just
  another rc-consumer under C-4.
- **C-4 · return-code must be FULLY MODELED; its directionality is oracle-contracted** (human, +SURE
  — THE ROOT). rc is not bool 0/1, and which rc value means "converged" is **declared by the oracle
  per check/command**, never assumed. This subsumes C-2 + C-3 and gw-1's Wall-1: a *query*
  (`command -v`) is a check whose rc maps to a verdict *without mutating*; a non-conforming establish
  (`useradd` rc 9) declares rc-9 = converged; the CFG tracks each consumer's **expected** rc
  (polarity), the oracle declares each command's **actual** rc→verdict mapping, and elision is sound
  iff they reconcile. **This is also Half-B subsumption** (branch-resolution = reconcile the guard's
  declared rc→verdict with the probe). Nearest corpus: `an-probe-shape` / `DP-3` ("capture the tool's
  own rc") + the three-valued verdict `T5`; C-4 sharpens them from "capture rc" to "model rc + contract
  its directionality." This is wave-2's center of gravity (high-lock — re-keys the oracle/effect
  contract).
- **C-5 · rendering: structural + reconstruction-metadata, two outputs** (human; OUT-OF-SCOPE this
  spike, docs-notation). The render becomes AST-structural but carries enough metadata (line/col,
  whitespace tokens) to reconstruct the original; reconstructions must be *minimal*; one AST serves
  two outputs — (a) a **minimal by-AST print** (ship only the active commands/expressions over ssh)
  and (b) a **full textual reconstruction** with ANSI grey-out escapes progressively wrapping the
  components/sub-expressions being eliminated/substituted *as probe data streams back onscreen*. This
  is the resolution of the line-vs-structural tension (gw-4 `strain-R`, `seam-prov`/`an-render-modes`);
  the current line-granular render is the disposable interim. The leaf-exact render will flip gw-5's
  `andor-rc-vouch-wrong` tripwire (desired signal).

## 4. Reframed wave-2 + open decisions (for the human)

The corrected core is **C-4 (model rc + oracle-contract its directionality) + C-1 (full-args to the
check) + C-2/C-3 (unified rc-consumer polarity in the CFG)** — one contract that unifies F1, the &&/||
under-execute, errexit, and Half-B subsumption. High-lock. Nothing reverted from wave-1; gw-1's F1 +
interceptor stand as interim special-cases superseded *in design* by C-1..C-4.

Open (human):
- **q-1a / q-1b** (C-1 interceptor shape — gate a clean rebuild).
- **wave-2 go:** build the C-4 rc-modeling contract next (unblocks everything, high-lock) vs the
  orthogonal lower-lock tracks (gw-3 backward/apply-3 — ch-scope-locked, and gw-4 found errexit is
  dead-weight until it lands; recursion-smoke/seam-finite). Cheap precursor: **fork-B** (verify the
  adversarial's ~SUSPECT "same-cell subsumption is soft" claim) to size the C-4 build.

## 5. Corrected realization (supersedes §3's C-2/C-3/C-4 framing) — it's just abstract interpretation over probed observables

The human dissolved most of §3's complexity. The clean model (+SURE, human-ruled):

**Probe → concrete observables → abstract-interpret the apply-CFG → omit what can't run.** The probe
ships the (command-keyed, read-only) checks and returns a set of **concrete observables** — actual rc
*values*, stdout, stderr, fds. The apply phase abstract-interprets the apply-CFG with those concrete
values substituted, over the *known* semantics of `&&`/`||`/`if`/`!`/`case`, and omits what provably
can't run. **rc is opaque to Dorc** — we hold the value (`9`), never interpret its meaning; `9 || mkdir`
→ `mkdir` runs, by the shell's own `||` semantics. The author already encoded the meaning by choosing
`!`/`&&`/`||`; Dorc just replays it over the probed value. *(This is concrete partial-evaluation where
probed, ⊤ where unknown — SPA ch.12 made concrete, not a fixpoint over abstractions.)*

- **The "polarity problem" was a non-problem (corrects C-2/C-3).** Nothing to *track*; no directionality
  for Dorc to *interpret*. gw-1's F1 "block status-consumption in `if`/`elif`" is a **floor that exists
  only because the spike doesn't yet track rc-as-value or execute the probe** (it injects a
  convergence-bit verdict). The real design *probes the guard, reads its rc, substitutes it, folds the
  branch* — which **is** Half-B subsumption, and *simpler* than the band-aid. errexit (C-3) isn't
  special either: it's the shell evaluating each command's concrete rc — abstract-interpretation gets it
  for free.
- **C-4 refined: the oracle contract is `fact-state → observables`, not `rc → verdict-directionality`.**
  Read-only guard (`command -v nginx`): run it, read the rc. Mutator (`useradd deploy`, can't run): the
  oracle declares the observables it *would* produce given the probed fact-state — "useradd, when the
  user exists → rc **9**" — so the substitution reproduces the exact value. An *observable-production*
  declaration, not a directionality interpretation.
- **Substitution = observable-value-MAINTAINING (DESIGN's observable/replace model, `16F`/`16P-T10`,
  diluted in the spike).** Replace a converged leaf with the cheapest stand-in that reproduces *the
  observables its consumers read* — the exact probed rc (`9`, not `false`/`1`, not `:`/`0`), captured
  stdout, etc. (`true` over `:` for readability going forward; the real thing reproduces probed
  observables, needing how-each-leaf-is-consumed + the oracle's `fact-state→observables`.)

**`q-1b` resolved — "command-keyed vs effect-keyed" is a layer-conflation, not an either/or** (dug the
corpus: `17N` F0/F1/§5, `16P` T7/DP-1). Three axes the "fact-centric not command-centric" slogan blurred:
1. the check/probe **invocation** is **command-keyed** — only the oracle can argparse the binary and
   decide which arg is a `Package`, whether the verb flips `#installed` or just reads it (`17N` F0: the
   engine is *referent-agnostic*; the oracle is the command-specific thing). +SURE: can't be otherwise.
2. cross-oracle **identity** is **named-kind**-keyed — two *different* commands (apt/brew/dpkg) touch
   one fact; a shared arg-token can't bridge them (`17N` §5). The named kind is a *coordination
   vocabulary* for command-keyed oracles, **not an alternative** to command-keying.
3. the elision **license** is **fact-converged**-keyed, not re-running the command. The only thing
   `DP-1`'s "command-centric → fact-centric pivot" rejected: the refuted strawman (`notes/161`) made the
   probe a *dry-run of the mutator* (`apt-get --simulate | grep`), making the named-kind "decorative in
   the skip path." "Fact-centric" fixed *that*, never the check invocation.
The tell: the corpus's own effect-map is the 3-place `(kind, provider, verb) → effect` — `provider`+
`verb` **is** the command, so it's command-*inclusive* by construction. Unified picture: **command-keyed
oracles declaring effects on named kinds.** (`q-1a`: only the probe *invocation* goes full-args now; the
cell-keying `resolve_entity` heuristic stays — flagged as an open design-hole, eventually oracle-mediated.)

**Reframed wave-2 (replaces §4's "rc-modeling contract"):** three pieces — **(1)** track observables
(esp. rc) as concrete values in the apply abstract-interpretation; **(2)** observable-preserving
substitution (reproduce exact rc/stdout, not `:`); **(3)** the command-keyed oracle-check contract
(full-args; declares the read-only check + the `fact-state → observables` it stands in for). One
mechanism for F1 + `&&`/`||` + errexit + subsumption. Fits the existing worklist substrate (lattice
values become probed-observables-or-⊤; no IFDS/Datalog change). +SURE more faithful to DESIGN's
observable/replace model than the spike's current `classify → prove_replaceable → :`-substitute.
