# 199 — Corpus: the non-guard strain-frontier (gw-4)

> Round-19 corpus track gw-4 (notes/197 §4/§5): map the strain-frontier across the
> analyzer's NON-GUARD surface — `e2e/`-only, never `crates/**`, never the guard cluster
> (gw-1 owns the F1 family). Continues 195 (the corpus + harness mechanism it builds on)
> and 197 (the coverage matrix this fills). Append-only; AI-authored; confidence-marked
> (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust root DESIGN/KNOBS/README/IMPLEMENTATION/
> AGENTS + plans/191 over this, and 195/197 over the prose here where they conflict.

## 0. Headline (the frame-inverting finding) — +SURE

**The non-guard frontier is mostly GREEN, not a strain-wall.** The coverage matrix in
197 §5 marked the non-guard rows "gap, gw-4 owns (almost none exercised)". That phrasing
is about *e2e exercise*, and is accurate — but it primed an expectation that these rows
are *unbuilt*. They are not: the ENGINE handles shell-execution-environment state,
scope-containment, enclosing observable-consumption, redirection-as-effect, the full
⊤-trigger set, command-substitution-non-leaf, and the multi-oracle Seam **correctly,
end-to-end through the rendered+executed artifact**. The round-16 CFG (cfg.rs find-1..8)
and effect classifier are far more complete on this surface than the matrix's framing
suggested. So this track's honest deliverable is *not* a wall of xfails proving "unbuilt";
it is a broad **green** corpus proving the built surface survives the full pipeline
(parse→cfg→effect→plan→render→**execute**, ap-2), plus the **three real, subtle strains**
the breadth surfaced — none of them a gross gap, all of them edge-precision:

- **strain-R (render-fidelity, the one genuine defect / new xfail):** the line-granular
  `render_apply` mangles a `case`-arm (and, by the same mechanism, any compound) when a
  structural token (`pat) … ;;`, an `if`/`then` keyword) shares a SOURCE LINE with an
  elidable leaf — `dash -n` syntax error, caught by ap-2. The non-guard twin of F1b's
  render-luck. Pinned `cases/render-case-arm-oneliner-wrong` (xfail).
- **strain-E (errexit e2e-invisibility):** the elaborate errexit precise-edge subsystem
  (cfg.rs find-1/2/3/5: `!`-pipeline, whole if/while cond, `&&`/`||`-left, `|| true`,
  failing-redir) has **no observable effect on the apply-2 rendered artifact** — it only
  refines `node→exit` edges, which the *backward* slice consumes, and the backward slice
  is unbuilt (ch-scope/gw-3). A large, carefully-built, well-unit-tested subsystem with an
  empty apply-2 projection. Not a defect; a *latency*. `cases/exec-errexit-elide-vouched`.
- **strain-S (Seam per-file probe-check):** a legitimately-SPLIT same-kind oracle pair
  (one file declares the probe, a sibling declares only another provider's effects) emits
  a **spurious `oracle-missing-probe`** diagnostic — the lifter checks probe-completeness
  per-FILE, not per-kind-across-files. The index is complete and elision works; the
  diagnostic is noise. `cases/seam-two-providers-one-kind`.

## 1. What landed (13 new cases, 26→39; +2 xfail total)

10 executable (ap-2 exec gate, run under PATH=mocks-only), 2 analysis-only (dash -n +
golden — real file-redirects can't be sandboxed, see §4 the C-1 analogue), 1 xfail. All
green/xfail; workspace untouched (no `crates/**`, no `run.sh` edit — see §6).

| case | cluster | mode | what it pins |
|---|---|---|---|
| `exec-subshell-establish` | A scope | exec | `( install )` body IS a leaf; its FS-effect escapes the subshell ⇒ converged install elides |
| `exec-enclosing-pipe-subshell` | B encl-liveness | exec | `( install ) | grep` Runs — consumption is the ENCLOSING pipeline's property, not the leaf's (16G kill-shot) |
| `enclosing-group-redir` | B encl-liveness | analysis | `{ install; } > f` Runs — enclosing group-redirect consumes inner stdout |
| `redir-as-effect` | A redir | analysis | `: > f` Runs (Redir is a first-class effect node, never an elidable Command leaf); upstream install still elides |
| `exec-errexit-elide-vouched` | A errexit | exec | eliding a converged install UNDER `set -e` is vouched (the `:`-stub rc-0 won't trip errexit) |
| `exec-multileaf-line-mixed` | C render | exec | `install; reload` on ONE line ⇒ whole line Runs (elision render-masked) — over-execution, SAFE |
| `render-multileaf-line-all-elide` | C render | exec | two converged installs on one line ⇒ single `:` (line-atomic elision) |
| `exec-subst-body-nonleaf` | A/C subst | exec | `echo "$(install nginx)"` — the `$()`-internal install is NOT a leaf (absent from probe); downstream curl elides |
| `top-eval` | D ⊤-trigger | exec | `eval "…"` ⊤ poisons downstream ⇒ converged install Runs (the safe ⊤-contagion) |
| `exec-top-arith-in-arg-ok` | D ⊤-precision | exec | `echo "$((1+1))"` is NOT ⊤-rejected (arith is a trigger only as the SOLE command word) ⇒ install elides |
| `exec-literal-unset-pure` | D ⊤-precision | exec | literal `unset FOO` is Pure (not ⊤, not poison) ⇒ install elides |
| `seam-two-providers-one-kind` | E Seam | exec | apt+yum both ground `package`, distinct entities both elide via the ONE shared probe (kind = cross-oracle anchor) |
| `render-case-arm-oneliner-wrong` | C render | **xfail** | strain-R: case pat+body+`;;` on one line ⇒ render mangles structure ⇒ dash -n error |

## 2. The filled coverage matrix (197 §5 non-guard rows; gw-4's)

Legend: **built+e2e** = engine handles it AND a new/existing e2e case exercises it through
the rendered (and where safe, executed) artifact; **built/invisible** = engine handles it
but it has no apply-2-observable projection; **gap** = genuinely unexercised/unbuilt;
**oos** = out of scope (charter / gw-3 / later).

### shell-execution-environment state (16P T9 / §B)
| sub-cell | status | evidence |
|---|---|---|
| errexit `!`-pipeline exempt (find-1) | built/invisible | cfg.rs `tests` + strain-E: no apply-2 effect |
| errexit whole if/while cond exempt (find-2) | built/invisible | cfg.rs + strain-E (cond is gw-1's *classification* turf; the *errexit-edge* part is invisible either way) |
| errexit `&&`/`||`-left exempt (find-3) | built/invisible | cfg.rs + strain-E |
| errexit `|| true` swallow | built/invisible | cfg.rs + strain-E |
| errexit failing-redir aborts (find-5) | built/invisible | cfg.rs + strain-E (redir IS fallible; edge invisible in apply-2) |
| `set -e` elide-vouched (the OBSERVABLE half) | built+e2e | `exec-errexit-elide-vouched` |
| scope-containment `( )` env/var/cwd don't escape | built (struct) | `ScopeEnter`/`ScopeExit` in cfg.rs; the *env-frame* pass is structural, no ShellEnvState dataflow instantiated yet (T9-lattice unbuilt) — see strain-E-adjacent gap below |
| scope-containment `( )` FS DOES escape | built+e2e | `exec-subshell-establish` (subshell install establishes ⇒ elides) |
| scope `{ }` mutations DO escape (no boundary) | built+e2e | `enclosing-group-redir` (+ `consumed_observables` over the group) |
| `$()`-body-as-non-leaf | built+e2e | `exec-subst-body-nonleaf` (e2e; was unit-only `substitution_internal_command_is_not_a_plan_leaf`) |
| redirection-as-effect `: >f` | built+e2e | `redir-as-effect` |
| `& wait` / background | built+e2e | existing `background-amp-runs` (`&`⇒⊤ node ⇒ has_top_successor blocks replace) |

### observable-liveness ENCLOSING variants (16P T10)
| sub-cell | status | evidence |
|---|---|---|
| `{ install; } > f` | built+e2e | `enclosing-group-redir` |
| `( install ) | grep` | built+e2e | `exec-enclosing-pipe-subshell` |
| `/dev/null` exempt | built+e2e | existing `exec-devnull-exempt` |
| leaf-local pipe (the contrast) | built+e2e | existing `consumed-output` (bare `install | tee`) |

### render fidelity / leaf-seam (16P T14)
| sub-cell | status | evidence |
|---|---|---|
| flat `render_sh` vs line-granular `render_apply` | built | render_sh is unit-only (plan.rs); the CLI emits render_apply — e2e only sees render_apply |
| empty-clause `:`-stub (multi-LINE arm) | built+e2e | strain-R case's contrast (the body-on-own-line form renders dash-clean) |
| **multi-leaf-line LeafId↔AstId blur** | **strain-R (xfail)** | `render-case-arm-oneliner-wrong`; + `exec-multileaf-line-mixed` (the safe over-exec variant) + `render-multileaf-line-all-elide` (the all-elide variant) |

### parser ⊤-triggers (16P T2 / inv-top-reject)
The full set, swept (each ⊤-rejects + poisons downstream — uniform behavior, so ONE
executed representative + this enumeration, not 9 redundant cases):
| trigger | ⊤? | e2e |
|---|---|---|
| `eval "…"` | yes (DynamicExecution) | `top-eval` (executed) |
| dynamic command name `"$cmd" …` | yes (DynamicExecution) | swept §3; unit `command_effect…dynamic` |
| `.`/`source` of non-literal `. "$x"` | yes (DynamicExecution) | swept §3 |
| `$(( … ))` as SOLE command | yes (ArithmeticExpansion) | swept §3 |
| `unset "$x"` (dynamic lvalue) | yes (DynamicLValue) | swept §3 |
| `printf -v VAR` | yes (DynamicLValue) | swept §3 |
| `test -v` / `[ -v ]` | yes (DynamicLValue) | swept §3 |
| `for`/`while`/`until` loop | yes (Loop) | existing `toprejected` |
| background `&` | localized ⊤ (⊤-containment, not poison) | existing `background-amp-runs` |
| **non-triggers (precision floor):** | | |
| `echo "$((1+1))"` (arith in ARG) | no | `exec-top-arith-in-arg-ok` |
| literal `unset FOO` | no (Pure) | `exec-literal-unset-pure` |
| literal `. /etc/profile` | no, but Opaque (poisons) | swept §3 (`.` not in pure-builtin list) |

### multi-oracle / the Seam (17N)
| sub-cell | status | evidence |
|---|---|---|
| two kinds (package + service) | built+e2e | existing `two-oracles` |
| **two providers ONE kind** | built+e2e (+strain-S) | `seam-two-providers-one-kind` |

### frontier (mark-don't-over-invest — gw-3/later)
| row | status |
|---|---|
| interproc / detached bodies | partial (existing `exec-detached-fn` pins the MustRun fold; real call-edges oos — cfg find-7) |
| ShellEnvState dataflow lattice (T9 env/var tracking) | **gap/oos** — `ScopeEnter`/`Exit` exist structurally but no env-frame *dataflow* is instantiated; only FS-effect (reaching-defs) and errexit run over the scope. The "env/var/cwd don't escape" claim is *structural intent*, not yet a checked dataflow property. Flag for whoever builds T9. |
| partial-convergence, volatile facts | oos |
| backward slice / apply-3 (would make strain-E observable) | oos (gw-3) |

## 3. The ⊤-trigger sweep (raw, for reproducibility) — +SURE

Each `<trigger>\napt-get install -y nginx` with `package:nginx#installed converged` and the
stock package oracle. Result uniform: a `syntax-unsupported` parse diag + a `cfg-top-node`
⊤ diag, and the downstream install renders **verbatim (Run)** despite converged — the ⊤
⇒ `CommandEffect::Opaque` ⇒ `Reach::Top` ⇒ install `EstablishWritten` ⇒ no license. Verified
for: `eval "…"`, `"$cmd" install …`, `. "$cfg"`, `$(( 1+1 ))`, `unset "$v"`, `printf -v out x`,
`test -v HOME`. The three non-triggers (`echo $((…))`, `unset FOO`, `. /etc/profile`) do NOT
emit a parse diag; the first two then ELIDE the install (pure/non-poisoning), the third RUNS
it (`.` is Opaque — sourcing can do anything — but not a ⊤-reject).

The `.`-asymmetry is worth a design eyebrow (~SUSPECT, not actioned): `cd`/`export`/`unset`/
`set` are blessed `is_target_state_pure_builtin` (don't poison), but `.`/`source` of a
*literal* file is Opaque (poisons). Defensible (a sourced file is arbitrary code; a `cd` is
not), but it means `. ./lib.sh; <converged mutators>` poisons the whole tail — a realistic
book idiom (sourcing a helper lib) defeats all downstream elision. Likely a real
coverage-erosion in practice; flagged for the oracle-coverage discussion, out of spike scope.

## 4. The strains, in detail

### strain-R — render-fidelity multi-leaf-line blur (the xfail) — +SURE; LOUD-ish
`render_apply` elides a SOURCE LINE iff a `Replace` leaf is on it and no `Run` leaf is, by
commenting the line + emitting a `:` stub. This is sound when the line is *only* a mutator.
It **breaks when a single source line also carries structural syntax**. The sharpest
non-guard case: a compact `case` arm `nginx) apt-get install -y nginx ;;`. The body install
is *legitimately* elidable (converged + ambient — the CLASSIFICATION is correct, unlike F1
where classification itself is wrong). But commenting the line deletes `nginx)` and `;;`
too, leaving `case nginx in` / `:` / `*) : ;;` / `esac` ⇒ `dash -n` exit 2 ("newline
unexpected, expecting `)`"). ap-2's dash -n gate catches it (so it's a caught wrong-RENDER,
not a silent ship). The SAFE behavior: a leaf-EXACT render commenting only the body token,
keeping `nginx)`/`;;` — and indeed the same arm written multi-LINE (body on its own line)
ALREADY renders dash-clean (verified). So the fix is leaf-exact render fidelity (seam-prov,
plan/CLAUDE.md's `an-render-modes` tension), NOT classification. This is the exact hazard
plan/CLAUDE.md predicted twice ("comment the lone body of an if/while/case arm and you
reproduce the empty-clause syntax error"; "a multi-leaf line … blurs the LeafId→AstId
back-map"). Now demonstrated end-to-end for the non-guard `case` half.

`exec-multileaf-line-mixed` shows the *other face* of the same line-atomicity: when a Run
leaf shares the line with an elidable Replace leaf (`install; reload`), the whole line runs
— the elision is render-masked (the install is in the probe, classified EstablishAmbient,
but never elided). This is SAFE (over-execution of an already-converged install, never a
wrong-elision) but is a real precision loss, and it is the *non-guard* generalization of
F1b's "render-luck" (F1b is the guard-status version; this is the plain-sequential version).
~SUSPECT a leaf-exact render fixes both strain-R and the masking at once.

### strain-E — errexit precise-edges are e2e-invisible in apply-2 — +SURE
cfg.rs spends find-1..8 (and a whole second forward pass, `materialise_errexit_edges`)
computing PRECISE conditional `node→exit` failure-edges, pruned where the shell never aborts
(negated pipeline, whole condition region, `&&`/`||`-left, `|| true`) and extended where it
does (failing redir). I could not produce ANY apply-2 rendered-artifact difference between a
book with and without `set -e`, across all those variants (the downstream establish elides
or runs identically). +SURE why: the edges only ADD reachability to `exit`; they never remove
forward reachability of a downstream node, and `classify`'s ambient gate keys on forward
reaching-defs + forward `reachable_from_entry`, neither of which the `→exit` edges perturb.
The edges exist *for the backward apply-minimization slice* (cfg.rs doc: "a spurious cmd→exit
edge is unsound BACKWARD") — and the backward slice is unbuilt (ch-scope, gw-3). So this is a
**built-but-latent** subsystem: correct, well-tested at the graph level (cfg.rs 31 tests), but
with zero observable projection until gw-3 lands the backward direction that consumes it. The
ONE errexit thing that IS apply-2-observable — that eliding a converged establish under
`set -e` is rc-0-vouched and doesn't trip errexit — is pinned by `exec-errexit-elide-vouched`.
Recommendation: when gw-3 builds the backward slice, the errexit precise-edges become its
first real consumer — that is where to add the precise-edge e2e cases (a spurious edge ⇒ a
wrong backward skip). Until then, the precise-edge unit tests in cfg.rs are the right home;
duplicating them as e2e would be theatre (they'd all render identically).

### strain-S — Seam per-file probe-completeness false-alarm — +SURE; minor
`oracle::lift` runs `lift_one` per file; `bind` diagnoses `MISSING_PROBE` if the file's
declared `oracle_kind` has no `oracle_probe_<kind>` *in that same file*. But the design's
own cross-oracle model (DESIGN inference-limitations; 17N Seam) is that the KIND is the
anchor and many provider-files share it. A natural split — `apt.oracle.sh` declares
`oracle_kind=package` + the probe + apt's effects; `yum.oracle.sh` declares
`oracle_kind=package` + only yum's effects — makes `yum.oracle.sh` (correctly probe-less,
the probe is apt's) emit a spurious `oracle-missing-probe`. The merged `KindIndex` is
COMPLETE (apt's probe covers `package`), and both providers' converged installs elide — so
the elision is correct and the diagnostic is pure noise. The e2e harness suppresses stderr,
so this does not redden the suite; but a real user splitting oracles this way would see a
confusing error. ~SUSPECT the right fix is to defer the probe-completeness check to AFTER
the whole `lift` (per-kind-across-files), not per-file — but that is an `oracle`-crate
change (out of gw-4's `e2e/`-only fence), so flagged not fixed. The case carries the note
inline so a reader isn't alarmed by the green-with-stderr-noise.

## 5. Cross-checks against the existing 26 (mis-marked "covered"?) — ~SUSPECT none

Audited the existing corpus against 195/197's coverage claims. Found **no mis-marks** — the
claims are honest. Nuances worth recording:
- `consumed-output` is leaf-LOCAL pipe-consumption (`install | tee`), correctly analysis-only
  (tee writes a real file). 197 §5 says T10 has "only the leaf-local case tested" — accurate;
  my `enclosing-group-redir` + `exec-enclosing-pipe-subshell` fill the ENCLOSING variants, so
  no redundancy.
- `exec-detached-fn`'s run-set (`apt-get install -y nginx` ran) proves the body executes via
  the `prov` CALL, not that the analyzer "chose to run the body" — the analyzer elides nothing
  there (body + call both MustRun ⇒ both verbatim). The case correctly pins "detached ⇒
  nothing elided"; just note the execution proves call-reachability, not an elision decision.
  Minor framing, not a mis-mark.
- `two-oracles`' service oracle has the documented selector mismatch (`enable` gates #enabled
  but the probe reads is-active/#active) — already flagged in-case + 195. Not my scope (gw-1/
  oracle-quality), left as-is.
- `background-amp-runs` correctly analysis-only (C-1 determinism boundary) and correctly does
  NOT elide the converged install (`&`⇒⊤⇒has_top_successor). Honest.

## 6. Harness & scope discipline (for the orchestrator) — +SURE

- **No harness change.** `e2e/run.sh` is byte-unchanged (verified: working-tree vs index diff
  empty). All 13 cases use the existing mechanism as-is (mocks/ + expected.ran for exec;
  dash -n + expected.out for analysis-only; XFAIL for the defect-pin). The existing C-2 mock
  shim and C-3 xfail support carried everything — no extension needed.
- **No `crates/**` change** (HARD SCOPE FENCE): verified working-tree vs index diff empty for
  `crates/`. The engine was probed as-is.
- **No guard-cluster collision**: I steered clear of `if`/`while`/`&&`/`||`/`!` status-consumed
  guards (gw-1's F1 family). `render-case-arm-oneliner-wrong` uses a `case` (a non-status
  construct) and the strain is render-layer, not the classification-layer F1 owns — disjoint.
- **Analysis-only rationale (the C-1 analogue):** 2 cases (`enclosing-group-redir`,
  `redir-as-effect`) are dash -n + golden because their rendered apply contains a real FILE
  redirect (`> /var/log/…`, `: > /etc/…`). The ap-2 exec gate runs the apply for real under
  PATH=mocks-only; a file-redirect is not a PATH lookup, so the shims cannot intercept it and
  executing would write outside the sandbox. This is the same shape as 195 C-1 (backgrounded
  `&` can't be executed): "verify by running it" is sound only for artifacts whose every
  side-effect is a PATH-resolvable command. The PIPELINE form of enclosing-consumption IS
  executable (`exec-enclosing-pipe-subshell` — a pipe is process-to-process, no FS sink), so
  the cluster is not left exec-less. New boundary, recorded for whoever extends the harness:
  **an executed case's rendered apply must contain no real file-redirect** (alongside C-1's
  no-backgrounding rule).
- **Dev-only scratch (NOT committed):** `mkmocks.sh`/`probe.sh`/`ran.sh` at the worktree root
  are my dev loop (stamp mocks; preview render; replicate exec_check's run-set). Left
  untracked; do not integrate (mkmocks.sh mirrors round19's existing root scratch).
- **BLESS note:** `BLESS=1 sh e2e/run.sh` is all-or-nothing; it regenerated every golden but
  produced BYTE-IDENTICAL output for the 26 existing cases (verified: `git diff` empty,
  `--ignore-all-space` empty) — so integrating only the 13 new case dirs is clean and safe.
