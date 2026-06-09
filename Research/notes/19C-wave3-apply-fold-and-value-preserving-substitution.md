# 19C — Wave-3 build: the apply-side fold + value-preserving substitution (19B build-1)

> AI-authored strain-log. Confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust
> R/D/I/K and `19A §5`/`19B` (the human's rulings) over this. Continues `19A`/`19B`. This is the
> **decision-independent, INJECTED-observables** half (build-1); the real oracle-contract side that
> *produces* observables (`19B` build-2: command-keyed `.check()` + the OOB verdict-lane + opt-B) is a
> separate later build and was NOT built here.
>
> Landed (this worktree, `ai/round19-keystone`): `1ef05ce` (core+fold+plan engine), `b95ec86`
> (cli+hostsim wiring), `d97dbd9` (xfail promotion + e2e re-bless + new fold-omit case). Baseline and
> result both GREEN: workspace tests pass, clippy clean, **42/42 e2e** (was 41; +1 new case; the one
> pre-existing `render-case-arm-oneliner-wrong` xfail unchanged).

## 1. What landed (the three pieces, `19B §5` build-1)

- **rc-as-a-concrete-value** (`core`): `Observed { verdict: Verdict, rc: Option<Rc> }` + `Rc(i32)`. The
  injection `verdict_of: Fn(FactKey)->Verdict` became `observe: Fn(FactKey)->Observed`. `rc == None` ⇒ ⊤
  for the fold (no fold ⇒ run). The cli stdin grammar gained an optional `rc=N` (the OOB lane as plain
  data, `19B §2`): a converged fact defaults to the conforming `rc=0`, diverged/unknown carry no rc.
- **the fold** (`plan/fold.rs`, ~410 lines incl. tests): abstract-interpretation over the **AST** (not
  the CFG — which leaf controls which branch is syntactic; the flattened CFG blurs it) computing a
  per-node `AbstractRc ∈ {Known(rc), Top}` and a per-leaf dead-set. Folds `&&`/`||`/`if`/`!`/`case`:
  `Known(0) || R` ⇒ R dead; `Known(n≠0) || R` ⇒ R live (the under-execute fix); `&&` dual; `if`
  known-true/false picks the branch and kills the rest; `!` inverts a known status; **`case` does NOT
  fold** (string scrutinee, not an rc — see strain-C). Fold OMITs only from KNOWN observables
  (`inv-kfail`); ⊤ ⇒ no fold ⇒ live.
- **observable-preserving substitution** (`plan`): `Disposition::Replace(license, StandIn)` where
  `StandIn ∈ {True, False, Exit(i32)}` reproduces the **exact** observed rc — `true` (rc 0, the human's
  pick over `:`), `false` (rc 1), `(exit n)` (other; a subshell so it sets `$?` without aborting). NOT
  always `:`. `Disposition::Omit{controller: AstId}` is the new fold-dead disposition.

## 2. The headline outcomes the prompt asked for — status

- **`&&`/`||` under-execute FIXED; the disposition xfail PROMOTED.** `observable_matrix`'s
  `xfail_nonconforming_establish_andor_left_operand_wrongly_replaced` → renamed/rewritten
  `nonconforming_establish_andor_left_operand_substitutes_exact_rc`, now a **passing** assertion. +SURE
  this is the correct promotion, but it is a *test rewrite*, not an auto-XPASS — see strain-A.
- **The `andor-rc-vouch-wrong` e2e tripwire now shows `mkdir` running** via the fold + exact-rc — but
  only after the fixture was corrected to inject `rc=9` (strain-B). It is no longer "render-masked
  accidentally safe"; it is *correctly* safe.
- **New `fold-oror-guard-omits` e2e** demonstrates the omission: `command -v nginx || apt-get install`,
  nginx present ⇒ the fold OMITs the install and collapses the line to its value-preserving `true`.
  Nothing runs (`expected.ran` empty). The canonical idempotency idiom (DESIGN `dpkg -s || install`).
- **F1 `if`/`elif` `mark_status` retirement: PARTIAL — KEPT, by render necessity.** See §3 (the central
  finding).

## 3. Where it strained (the deliverable)

### strain-A — the xfail can't auto-promote; "safe" changed mechanism (+SURE)
The old xfail asserted `!is_replaced(useradd)` — the only SAFE option when the substitution was a rc-0
lie (`:`). Under value-preserving substitution the *correct* disposition is "**replaced, but by
`(exit 9)`**" — so `is_replaced(useradd)` stays **true**, and a bare `#[should_panic]` would NOT flip to
XPASS. The honest promotion *rewrites the assertion*: the stand-in is `Exit(9)` (value-preserving) AND
`mkdir` is `Run`. The lesson: a "promote the xfail" instruction can require changing *what is asserted*,
not just deleting `#[should_panic]` — the fix made the old safe-criterion (not-replaced) the wrong
criterion. ~SUSPECT this is a general shape: when a fix changes the *mechanism* of safety, xfails pinned
to the old mechanism's observable need rewriting, not flipping.

### strain-B — a converged establish's default rc is the CONFORMING assumption; non-conforming MUST inject (+SURE, sharp)
The `andor-rc-vouch-wrong` fixture originally said `user#present converged` with no rc. The cli defaults
converged-without-rc to `rc=0` (conforming — the established `:`/`true` behaviour). But `useradd` is
**non-conforming** (rc 9 when converged), so the default rc-0 made the fold think `useradd` *succeeded*
⇒ omit `mkdir` ⇒ **the exact under-execute the case exists to catch.** The fix: the fixture injects
`rc=9`. This is `19B` opt-B made concrete: *a non-conforming establish's converged rc is a separate
declaration the oracle must produce* — the default-to-rc-0 is sound ONLY for conforming establishes.
+SURE this is the single most load-bearing correctness seam for **build-2**: if the real oracle-contract
side fails to emit a non-conforming establish's true rc, the fold silently under-executes. The default
must stay conforming (rc 0 is the common case and the old behaviour), so the burden is *entirely* on
build-2 to inject the non-conforming rc. (`inv-kfail` does NOT save us here: a converged verdict + a
*wrong* default rc 0 is not an "unknown" — it is a confident wrong value. The protection is that the rc
is a *declared observable*, not inferred; an un-declared rc on a non-conforming establish is an
oracle-quality defect, like a lying probe — `tc-reliability`.)

### strain-C — `case` does not rc-fold (the other construct; expected, recorded) (+SURE)
`case` switches on a **string** value, not an exit status, so the rc-fold cannot resolve which arm
matches — every arm stays live (the `case_does_not_fold_rc_all_arms_live` test pins this). Folding `case`
needs string-value abstract-interpretation (track the scrutinee's literal/`$(…)` value and match it
against patterns), which is a *different* abstract domain, out of `19B` build-1 scope. This is a real
fold-coverage gap: the `pi-webhost` book's `case "$(hostname)" in …` is unfoldable here (and the
scrutinee `$(hostname)` is `Opaque` anyway). --WONDER whether string-value folding is worth it; the
hostname-case is the motivating real example, but its scrutinee is a command-substitution ⇒ ⊤ regardless,
so string-folding only helps **literal** scrutinees (`case $LITERAL in`), which are rarer. Deferred.

### strain-D — the central tension: F1 `mark_status` cannot retire under the line-granular render (+SURE, the big one)
The prompt's model is right: exact-rc substitution makes F1's `if`-guard status-block **unnecessary at
the disposition layer** — substituting a guard leaf by its exact rc preserves the branch decision (a
conforming guard rc 0 ⇒ `if true; then B` ⇒ B runs, matching reality; a guard rc 9 ⇒ `if (exit 9)` ⇒ B
dead). So in principle the fold subsumes F1 uniformly. **But the line-granular `render_apply` cannot
express an in-situ substitution of a guard-condition leaf** that shares its source line with the `if`/
`while` keyword (`if ! command -v nginx`): commenting that line + emitting the stand-in produces
`# if ! command -v nginx` / `true` / `then` / … — a `dash -n` failure (`then`/`fi` without `if`). The
only safe line-render is to keep the guard verbatim — which is exactly what `mark_status` forces (block
the guard's license ⇒ `Run` ⇒ line verbatim).

So I **KEPT** `mark_status` for `if`/`elif`, and added an **omit-safety gate** in `render_apply`: an
`Omit` (fold-dead) body is neutralised *only if its controlling guard is itself neutralised* (Replace/
Omit). For an `if`-guard, the guard is `Run` (mark_status) ⇒ the gate keeps the dead body **verbatim**
(it runs; the runtime guard re-decides — the F1 floor, safe). For a `||`-guard (never `mark_status`'d),
the guard is `Replace` ⇒ the gate omits the dead body and collapses the line. **This is the clean
reconciliation**: F1's `if` floor and the fold's `||` omission coexist, gated on render-expressibility.

The retirement is therefore **partial and the partition is render-driven, not model-driven**:
- `&&`/`||` operand (the `tc-mint` gap that left the under-execute): **CLOSED** by the fold + exact-rc.
  This was never `mark_status`'d, so nothing was "retired" — the fold *added* the missing handling.
- `if`/`elif` guard-condition: **`mark_status` KEPT** — retiring it breaks `dash -n` under line-render.
  The disposition layer no longer *needs* it (exact-rc substitution suffices), but the render does. The
  full retirement waits on the **leaf-exact / structural render** (`C-5`/`seam-prov`, `19A` C-5,
  OUT-of-scope this spike): substitute `command -v nginx` → `true` *inside* the `if !`, then omit the
  body. Once that render exists, `mark_status` can go and the fold covers `if` uniformly.

I did NOT regress `kFAIL-perform`: the kept `mark_status` + omit-safety gate keeps `guard-status-blocks-
elision` byte-identical (install still runs). ~SUSPECT keeping `mark_status` is the *right* call for this
spike (don't trade correctness/render-validity for the optimization of omitting an `if`-body); the
finding is that the render layer, not the analysis, is the blocker — which re-confirms `C-5`'s priority.

### strain-E — omit-coherence: never omit a body while keeping a live guard (the stale-guard hazard) (+SURE)
The omit-safety gate is not just for `dash -n`; it is a **soundness** gate. Omitting a body while keeping
its guard verbatim is incoherent: the kept guard re-evaluates at apply-time and may *disagree* with the
fold's probe-time deadness (a stale/changed guard), so the kept-guard + omitted-body artifact could
under-execute. The gate forbids exactly this (omit only when the guard is also neutralised, so the
artifact does not re-evaluate a guard whose decision we already baked in). +SURE this generalises: **a
fold omission and the substitution of its controller must travel together** — you cannot bake one side of
a branch decision (omit the body) while leaving the other side (the guard) dynamic.

### strain-F — `tc-reliability`: the fold trusts the probe rc exactly as elision trusts the verdict (~SUSPECT, consistent)
The fold omits a branch from a probed rc; a stale/lying probe ⇒ wrong omission. But this is **no worse**
than the existing convergence-elision, which already replaces a converged establish with `true` on a
trusted verdict — both trust the probe (DESIGN's "by-probing, not by-stale-central-state"; the probe is
fresh per-run; `kFAIL-withhold` keeps probes conservative ⇒ ⊤ ⇒ no fold). So the fold does not *add*
reliability exposure beyond what Dorc already accepts. ~SUSPECT this is fine; the real adversarial test
(`an-host-as-adversary`: a forged-Converged with a forged rc) is `hostsim` fault-injection work, not
built here. NOTE the strain-B subtlety lives here too: a confident-but-wrong rc (not Unknown) is the
forged-verdict shape, and `inv-kfail` does not catch it — only `inv-must-may` (a `Must` fact) + the
sandbox do.

## 4. The exclusion-check on the fold (AGENTS four axes) — run, recorded

- **other phase (probe vs apply):** +SURE the fold is the **apply** collapse (`inv-superposition`). It
  lives in `plan` (the phased apply caller), driven by the *injected* apply-side observations; the engine
  (`analysis`/`core`) emits nothing phase-baked — `Observed`/`Rc` are phase-agnostic vocabulary. A probe
  must NOT run an apply fold (a probe withholds on ⊤; it does not omit apply branches). The fold engine
  takes no `Phase` and bakes none. ✓ (`tc-phase`/`tc-collapse` honoured: emit phase-agnostic, collapse in
  the caller.)
- **other reliability (unreliable/lying oracle):** see strain-F + strain-B. A ⊤ (un-probed) controller
  folds nothing (`unknown_guard_folds_nothing_kfail_perform` test). A *lying* converged-rc is the build-2
  contract's exposure (strain-B). ✓ for the un-probed case; ⚠ flagged for the forged-rc case (build-2).
- **reverse / backward direction:** the fold is forward (it replays the shell's forward `&&`/`||`/`if`
  semantics). It does NOT touch the never-instantiated backward/`Must`/apply-3 tower (`ch-scope`). The
  `Omit` disposition is a forward-fold output; a backward apply-3 slice would *additionally* prune for
  relevance — orthogonal, not in tension. ~SUSPECT no backward interaction; not load-tested (no backward
  caller exists, `16P T4`).
- **other user (admin vs engineer):** the fold serves the **admin** (we-infer): it reads the admin's
  control-flow + injected observations and omits/substitutes. The engineer's lever is the oracle that (in
  build-2) *produces* the rc — `opt-B` (a non-conforming establish's rc declaration). The fold's
  correctness *depends* on the engineer declaring non-conforming rcs (strain-B), so the admin/engineer
  seam is exactly the build-2 contract. ✓ identified, deferred to build-2.

## 5. `tc-*` flags raised (never settled in isolation — flagged up)

- **`tc-collapse` (the fold IS a `tc-collapse`):** I placed the fold in `plan` as the apply-phase collapse
  and kept the engine phase-agnostic (`inv-superposition`). +SURE this is the right home, but it is a
  genuine collapse-orientation call — flagging it. The fold collapses "known rc + control structure" into
  "this leaf is dead." It folds ONLY in the apply direction; I did **not** add a probe-phase fold (a probe
  has no apply branches to omit). If a future probe-projection wants its own fold, it must be a *separate*
  collapse — do not reuse this one (the doc-comment says so).
- **`tc-mint` (the `&&`/`||` gap):** the deferred `tc-mint` gap (a `&&`/`||` left operand's status, the
  post-condition-vs-guard ambiguity) is **dissolved**, not resolved-by-blocking: the fold reads the
  operand's *actual* rc and replays the operator, so there is nothing to "decide" — `install && start`
  (rc 0 ⇒ start runs) and `cmd || install` (cmd rc 0 ⇒ install dead) both fall out of the same fold. The
  human's `19A §5` "the polarity problem was a non-problem" is +SURE correct in practice. The residual
  `tc-mint` judgment (is this consumed status a license-blocker?) is **no longer needed for `&&`/`||`** —
  the fold subsumes it. (`if`/`elif` still uses the `mark_status` block, but for *render* reasons —
  strain-D — not because the `tc-mint` decision is unresolved.)
- **`tc-phase`:** taken the safe default — the fold is apply-only, phase-agnostic engine. Flagged (above).
- **NOT raised / out of scope:** `tc-uniqueness`, `tc-taint`, `tc-user` (build-2's), `tc-reliability`
  (build-2's forged-rc — strain-F).

## 6. What build-2 (the real oracle-contract side) MUST maintain (hand-forward)

- **+SURE, the load-bearing one (strain-B):** build-2 MUST produce a **non-conforming establish's exact
  converged rc** (`useradd`→9, `mkdir` w/o `-p`→1, `ln` w/o `-f`, `docker network create`, …) via opt-B
  (`19B §1`: a mutator's converged observables are a *separate* declaration, NOT the check mimicking the
  mutator's rc). If build-2 omits it, the fold's conforming `rc=0` default silently under-executes the
  fallback. The injection point is exactly `observe: Fn(FactKey)->Observed`'s `rc` field; build-2 fills it
  from the oracle's `fact-state→observables` declaration.
- The rc must ride the **OOB lane as structured data** (`19B §2`), never an exit code — `unknown` stays a
  distinct token; the real rc is `rc=2` data. The cli stdin grammar (`rc=N`) is the build-1 stand-in for
  that lane; build-2 replaces the stand-in with `$DORC_VERDICT` records.
- The check is **command-keyed, full-args** (`19B §1`/`C-1`); the fold consumes a *fact-keyed* rc here
  (build-1 injection), but build-2's per-command `.check()` is what *produces* it. ~SUSPECT a single fact
  probed by two different commands could yield two rcs — build-1's fact-keyed injection collapses that;
  build-2 must decide the keying (likely command-keyed production → fact-keyed consumption, per `19A §5`'s
  three-layer split: command-keyed invocation, named-kind identity, fact-keyed license).
- **The leaf-exact / structural render (`C-5`) is the unlock for full F1 retirement** (strain-D). Until
  it lands, `mark_status` stays for `if`/`elif`. Build-2 does not need it, but the render work (a separate
  C-5 item) is what lets `mark_status` finally go.

## 7. Honest scope notes / what I did NOT do

- Did NOT build the oracle-contract side (build-2) — injected observables only, as instructed.
- Did NOT retire `mark_status` for `if`/`elif` (strain-D) — kept, render-gated, noted. The `&&`/`||`
  handling is *added*, not a retirement of an existing special-case (there was none for `&&`/`||`).
- Did NOT touch the backward/`Must`/apply-3 tower (`ch-scope`) — the fold is forward.
- Did NOT add string-value `case` folding (strain-C) — out of build-1's rc-fold scope.
- The line-granular render stays the artifact renderer; the fold's omission is only rendered where the
  controller is neutralised (omit-safety, strain-E) — so a multi-leaf line mixing a live `Run` leaf with a
  dead `Omit` leaf stays verbatim (the `Omit` runs; safe). The `render-case-arm-oneliner-wrong` xfail
  (pre-existing, render-fidelity) is unchanged — same `C-5` render limitation, different trigger.
