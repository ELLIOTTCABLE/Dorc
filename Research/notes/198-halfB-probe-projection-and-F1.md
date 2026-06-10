# 198 — Half-B probe-projection + the F1 stopgap: what landed, and where Half-B's wall actually is

> Round-19 spike-2, the gw-1 (Half-B / F1) track. Append-only; AI-authored;
> confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust root
> `README`/`DESIGN`/`IMPLEMENTATION`/`KNOBS`/`AGENTS` + `plans/191` over this, and
> `16P`/`16Q`/`17N`/`195`/`196`/`197` over the prose here where they conflict. This
> continues 195 §F1 (the F1 trace) and 196 §2 / 197 §2 (the Half-B model).
>
> Baseline going in: green (workspace `cargo test`, 26/26 e2e, one honest xfail
> `guard-elision-wrong`). Baseline going out: green (workspace, 26/26 e2e, **xfail
> promoted to a passing executed case** — the F1 defect is fixed).

## 0. TL;DR — what landed, what didn't, where the wall is

- **(a) F1 stopgap — LANDED** (commit "F1 stopgap: branch-consumed status blocks
  elision"). A command whose exit status is consumed by an **`if`/`elif` guard
  condition** now blocks the `ReplaceLicense` (it RUNS, not `:`-stubbed). Errexit-
  consumed status stays vouched (still elides). The xfail `guard-elision-wrong`
  flipped to a passing, executed case `guard-status-blocks-elision`.
- **(b) Half-B.1 — LANDED** (commit "Half-B.1: FLAT operand-bound probe
  interceptors"). `compile_probe`'s render now emits the FLAT interceptor model
  (197 §2 variant A): `package__check() { … }; package__check nginx` — the book's
  operand **bound**, not the Half-A `$1`-empty stub. Both legs oracle-anchored.
- **(b) Half-B.2 (guard SUBSUMPTION) — NOT BUILT; wall mapped.** Subsuming a
  guarded branch from the probe verdict (instead of running the guard) is blocked on
  **three** unbuilt things, two of which are charter-deferred and one of which is a
  `tc-*` trade I must not settle alone (§4). The F1 stopgap ("RUN the guard") is the
  prompt's stated safe floor, and it is what's shipped. This is the honest
  "abandon-the-direction-at-the-wall-and-log-it" outcome the charter sanctions.

The single most useful finding (§4, +SURE): **Half-B's hard part is not the probe
render (that was easy); it is that *subsumption needs a "query/probe" effect category
the oracle model does not have* (queries like `command -v`/`dpkg -s` are mismodeled as
`establish`), AND occurrence-typing (`inc-6`, unbuilt) to know the guard gates the
body, AND a structural apply rewrite (variant B). F1 works *precisely because* the
stopgap needs none of those — it only blocks elision of a branch-status consumer.**

---

## 1. The F1 stopgap — the safe floor (a)

### 1.1 What it is, and the locus that makes it correct

F1 (195 §F1): a command whose exit status is consumed by a guard was classified
`EstablishAmbient` and `:`-stubbed when its cell converged — destroying the branch
decision (a `kFAIL-perform` under-execute; the block-`if` form also orphans `then`,
caught by `ap-2`). Root cause (16G §66 / 16P-T10): the round-16 model decided "no
status gate" because rc-0 is vouched by the establishes-contract. **That is sound for
a post-condition / errexit consumer, unsound for a guard / pre-condition consumer**
(a *different branch* runs on the rc).

The fix reuses the existing observable-consumption machinery (the `Cfg.consumed`
set + the block in `prove_replaceable`). The engine already records, per leaf, the
unvouched observables its context consumes (`Stdout`/`Stderr`); the fix adds
`Observable::Status` to that set **only in an unambiguous-guard condition region**:
- `analysis/src/cfg.rs` `lower_condition_region` grew a `mark_status: bool`; the
  `if`/`elif` path (`lower_if_chain`) passes `true`, marking every command in the
  test region as branch-status-consuming.
- `plan/src/lib.rs` `prove_replaceable` now blocks on `Observable::Status` too.

**The locus IS the whole correctness argument** (+SURE): the errexit pass NEVER marks
`Status`, so errexit-consumed status simply never enters the set ⇒ stays vouched ⇒ a
converged install under `set -e` still elides. Branch-status (marked by
`lower_condition_region`) and errexit-status (never marked) are separated *by where
the mark is applied*, not by a runtime flag. This is the cleanest possible cut: no new
type, no new field, one `bool` parameter.

### 1.2 The A/B contrast (pinned)

`plan/tests/observable_matrix.rs` (the round-16 observable regression suite) gained
the branch-vs-errexit A/B:
- `f1_status_consumed_by_if_guard_blocks_replacement` — `if apt-get install …; then …`
  ⇒ NOT replaced (the guard runs). (195: reproduces with ONLY the package oracle,
  install-as-guard — no new oracle needed.)
- `f1_status_consumed_by_errexit_stays_vouched` — `set -e; apt-get install …` ⇒ STILL
  replaced (errexit-status vouched).
- `f1_andand_left_operand_stays_replaced_tc_mint_gap` — the deferred gap (§1.3).

Engine-side facts pinned in `analysis/tests/cfg.rs` (`consumed_if_guard_marks_status`,
`consumed_negated_if_guard_marks_status`, `consumed_errexit_does_not_mark_status`,
`consumed_andand_left_operand_does_not_mark_status`).

### 1.3 The `&&`/`||` ambiguity — a `tc-mint` gap I deliberately did NOT resolve

**`tc-mint` (FLAGGED, not resolved).** A `&&`/`||` LEFT operand is a status consumer,
but it is **ambiguous**:
- `install && start` — a POST-condition use ("did my install succeed? then start").
  rc-0 is vouched by the establishes-contract ⇒ stays replaceable. The matrix
  *deliberately pins this* (`pins_converged_status_via_andand_replaced`, from 16G §66
  `p-oror`).
- `cmd || install` (cmd a guard) — a PRE-condition use that SHOULD block (195 lists
  it as a guard consumer), **structurally identical at the CFG**.

The safe default for an ambiguous case is to BLOCK — but blocking here would regress
the matrix's deliberate post-condition pin, eroding real value. Disambiguating needs
the F3 co-reference judgment (does the consumed status gate a *different* branch's
body, or is it a post-condition on the same establish?) — the Half-B work. **So the
stopgap leaves `&&`/`||` UNMARKED** (`mark_status=false`, with a `TODO(tc-mint)` at
the call site) and marks only the unambiguous `if`/`elif` guard. This is a *narrowed*
fix: it provably fixes the headline F1 shape (`if ! command -v`) with zero matrix
regression, and pins the residual visibly.

~SUSPECT this is the right call for a stopgap, but the orchestrator owns it: the
alternative (block `&&`/`||` too, lose the post-condition pin) is *also* defensible
and *safer*. Two faces of `tc-mint`; I took the value-preserving one because the
prompt said "don't over-gate, or nothing elides," and flagged the residual loudly.

### 1.4 `while`/`until` are vacuous here (a parser fact worth knowing)

+SURE: the prompt lists `while` as a branch context, but the spike's parser
**⊤-rejects all loops** (`for`/`while`/`until` → `UnsupportedReason::Loop`,
`syntax/CLAUDE.md`). A `while`-guard never reaches the elision path — the whole loop
is a `Top` node, already blocked via `has_top_successor`/`MustRun` (over-conservative,
safe). So `lower_condition_region`'s `mark_status` covers only `if`/`elif` *today*; if
loops are ever modeled, the loop-condition path must pass `mark_status=true` too. Left
a note in the code; no live surface to test now.

### 1.5 Exclusion-check (the four axes) — +SURE the stopgap holds

- **reverse / apply-3 (backward):** the block is in `prove_replaceable` (the elision
  mint), which a backward apply-3 slice still routes through ⇒ the block carries.
  ~SUSPECT holds; gw-3 should confirm against the real backward caller.
- **other phase (probe):** the block is phase-agnostic (it forbids the mint); only
  what a blocked leaf *becomes* is phase-keyed (the caller's collapse). Holds in both.
- **other user:** helps the engineer (guards run safely); doesn't change the admin's
  bare-mutation elision (no condition region ⇒ no status mark). Holds.
- **unreliable oracle:** a lying oracle that declares `command -v` as `establish
  present` can no longer cause the guard to elide (branch-status blocks regardless) ⇒
  the guard RUNS ⇒ the real rc decides. **More** robust than before. Holds (a
  robustness *gain*).

---

## 2. Half-B.1 — FLAT operand-bound interceptors (the sound part of (b))

### 2.1 What changed

`ProbePlan::render_sh` now emits, per kind, one `<kind>__check()` function (the
strawman's `id__check` shape) wrapping the oracle's `oracle_probe_<kind>` body, then
**one invocation per fact with the book's operand bound**:

```sh
# probe: package:nginx#installed
package__check() { dpkg-query -W "$1" >/dev/null 2>&1; }
package__check nginx                 # operand BOUND (was: $1 empty — Half A)
# probe: firewall:80/tcp#allowed
firewall__check() { ufw status 2>/dev/null | grep -q "$1"; }
firewall__check 80/tcp              # one fn def, invoked per entity (dedup)
firewall__check 443/tcp
# probe: pkgindex#fresh
pkgindex__check() { … }
pkgindex__check                      # Singleton ⇒ no operand
```

Both legs are oracle-anchored (197 §2): ordering + operand from the **book** (the
fact's `EntityRef`), body from the **oracle**'s declared check. FLAT = independent
interceptors (concurrent-dispatchable). +SURE this is the corrected-model variant A,
and it kills the 16P-T16 "$1 unbound" degeneracy. Resolving the operand into the
*shipped artifact* is referent-agnostic (passed through to the check, never branched
on — same latitude `fact_label` uses for display).

### 2.2 Strain finding S1 — the ≥enum floor: one probe per KIND can't distinguish selectors

+SURE, surfaced directly by the headline render (§2.1): `service:nginx#enabled` and
`service:nginx#active` BOTH render `service__check nginx` — *identical* invocations —
because the oracle model has **one probe per kind** (`oracle_probe_service`), but the
selectors are independent cells. The single body (`systemctl is-active --quiet "$1"`)
checks `#active` only; it cannot answer `#enabled`. This is `F-BLESSED` (oracle
CLAUDE.md: "an honest `service` probe is TWO commands — `is-enabled` AND `is-active`")
made concrete in the probe output. The probe over-claims: it ships the same check for
two selectors, so a host that answers it answers `#active` for both ⇒ if `#enabled`
diverged but `#active` converged, the apply could wrongly elide an `enable`. **The
probe model needs per-(kind,selector) bodies, not per-kind** — a real strain on the
oracle representation. (Not fixed here: it is an oracle-shape change, and the corpus
sidesteps it by having `enable`/`start` both diverged or both converged in the
fixtures.) Hand to the oracle/corpus tracks.

### 2.3 Note — the probe stays `sh -n`-only, not executed (still correct)

195 C-4 kept the probe at `dash -n` (not executed) because the Half-A stub was
non-faithful. Half-B.1 makes it faithful (operand bound), BUT executing it is still
out of scope: read-only enforcement is the `hostsim` `kFAIL-withhold` monitor's job
(`an-withhold-sandbox`, unbuilt), not the e2e harness's. Running the probe under the
mock harness would prove "it parses + exits," not "it's inert" — a malicious oracle
body could mutate; only the sandbox catches that. So the harness still `-n`-checks the
probe and executes only the *apply*. `exec_check` is artifact-agnostic — point it at
the probe once the sandbox exists (`F-FW3`). Unchanged from 195.

---

## 3. The body-elision-inside-a-guarded-branch hazard (S2 — sharp, pre-existing)

~SUSPECT this is the sharpest *un-flagged* hazard I found, distinct from F1. With the
F1 stopgap, the GUARD runs, but the BODY inside the `then`-branch is still elided on
its OWN cell's verdict:

```sh
# book: if ! command -v nginx; then apt-get install -y nginx; fi
# probe: tool:nginx#present converged, package:nginx#installed converged
# →
if ! command -v nginx >/dev/null 2>&1
then
# apt-get install -y nginx   # dorc: elided (already converged)
   :
fi
```

Is the body `:`-stub a wrong-elision? **Analysis (~SUSPECT sound under a consistent
host, unsound under an inconsistent one):**
- The body elision is licensed by `package:nginx#installed` converged ⇒ `apt-get
  install nginx` is a genuine no-op ⇒ `:` is behavior-equivalent **regardless of the
  guard** (if the package is installed, install does nothing whether or not we enter
  the branch).
- It breaks ONLY if the host is *inconsistent* across cells: `tool:nginx#present`
  diverged (binary absent → guard true → enter then) while `package:nginx#installed`
  converged (DB says installed). Then: guard runs → enters then → `:` → install
  skipped → binary stays absent. The admin's intent ("install if binary missing") is
  defeated — but only because the host lied (DB ≠ reality), which is the **universal
  `tc-reliability` probe-accuracy risk**, not a guard-specific bug.

**`tc-reliability` + `tc-mint` (FLAGGED, not resolved).** Whether to block body-
elision *inside a guarded branch* is a real orientation call: blocking is safer
(over-execute) but kills the common idempotent-install-inside-a-guard win; not
blocking is sound under a consistent host (the universal assumption) but trusts cross-
cell consistency. I did NOT change this (it is pre-existing, unchanged by F1, and a
`tc-*`). The orchestrator should decide whether "a leaf inside a conditional whose
gating fact ≠ the leaf's fact" warrants a block. +SURE it is a genuine state-space
cell the corpus should pin (gw-4), currently uncovered.

---

## 4. Half-B.2 (guard SUBSUMPTION) — the wall, mapped concretely

The corrected model (197 §2): "the apply then SUBSUMES the guarded branch from the
probe verdict instead of `:`-stubbing it." I attempted to scope a sound version and
hit three distinct walls. **None is incidental; each is a real, named deferral.**

### 4.1 Two DIFFERENT operations the docs conflate (the clarifying frame — +SURE)

Disentangling "subsumption" was the key analytical step. There are two:
- **(A) Branch resolution** — use the guard's probe verdict to decide which branch
  runs, and replace the guard accordingly (e.g. `if ! command -v nginx; …` with
  `tool#present` converged → the `then` is dead → subsume it). **This is SOUND**: it
  is the admin's *own* branch logic, computed from the probe instead of run live. The
  admin wrote "install iff binary absent"; the probe says present; skipping is the
  admin's intent.
- **(B) Body-elision-via-guard** — use the guard's probe verdict to license eliding
  the *body's mutation* as already-done. **This is the F3 TRAP** (17N F3 / SF-1):
  `if ! dpkg -s conflicting_pkg; then apt-get install something` — the guard's cell
  (`package:conflicting_pkg#installed`) ≠ the body's cell (`package:something#…`).
  Linking them is may-grade co-reference promoted to must — a priority-1 under-execute
  if the body is wrongly elided. **The engine does NOT do (B)** (it elides the body on
  the body's OWN cell — that is S2, §3, a different thing), and must never.

F3's "subsuming a guarded branch is licensed only when the probed fact actually gates
the body" is about **NOT sliding (A) into (B)**: branch resolution (A) is fine; using
the *guard's* fact to elide the *body's* establish (B) is the trap.

### 4.2 Wall-1 — subsumption (A) needs a "query/probe" effect category the model lacks

+SURE, the deepest finding (echoes 196 §2 "the engine has no guard/check category").
To resolve a branch (A) from a probe verdict, the engine must know the guard command
is a **read-only query of fact F** and how its rc maps to F's state. But the oracle
model has only `Polarity{Establish, Kill}` — and **queries are mismodeled as
`establish`** (`command -v` → `oracle_effect command -v establish present`; `dpkg -s`
likewise would be). An `establish` *mutates* F to hold; a *query* observes F without
mutating. They are opposite. Subsumption-by-branch-resolution needs "this command
PROBES F (read-only), and rc-0 ⇒ F holds" — a third effect category (`Probe(F)` /
`Query(F)`), which does not exist. This is the same conflation that *caused* F1 (195:
"`command -v` … establishes nothing … the establish-contract vouch is vacuous").
**Until the oracle model distinguishes query from establish, the engine cannot
soundly resolve a branch from a guard's verdict** — it cannot even tell a guard from a
mutator. (`inc-7` "effect-map ≡ typestate transition" is the place this lands: a query
is the *identity* transition / a pure read; charter-deferred richness.)

### 4.3 Wall-2 — subsumption (A) needs occurrence-typing (`inc-6`, charter-deferred)

Even given a query category, resolving the branch needs to connect "the if-condition's
fact verdict" to "which branch is dead," tracking `!`-negation and the establish
polarity. That is **occurrence-typing / branch narrowing (`inc-6`)**, explicitly
unbuilt (charter §3 "occurrence-typing narrowing — unbuilt"; `analysis/CLAUDE.md`).
The engine today stores the `!`-negation only transiently (cfg.rs `lower_pipeline`
clears errexit-fallibility on it; it is not retained per-leaf for branch resolution).
Confirmed in source: there is no per-leaf "this guard, when its fact is converged,
resolves the branch to skip." Building it is the charter-deferred narrowing analysis.

### 4.4 Wall-3 — the apply rewrite: variant-B forcing OR a `tc-reliability` trade

Suppose Walls 1+2 were cleared. The apply must now *render* the subsumed branch. Two
options, **both strain findings**:
- **Variant B (CFG-preserved / structural rewrite):** remove the `if`/`then`/`fi` +
  body lines as a unit. The current `render_apply` is **line-granular** (it elides
  individual lines); eliding a whole `if`-branch line-by-line orphans `then`/`fi` (the
  exact `ap-2` empty-clause break). So branch-subsumption **forces variant B** — the
  CFG-preserved render the prompt said to build "only if a case forces it, and the
  forcing IS a strain finding." +SURE: it is forced. This is the `seam-prov` /
  render-fidelity tension (`plan/CLAUDE.md` `an-render-modes`) made load-bearing.
- **Guard-constant trade (leaf-granular, avoids variant B):** replace just the guard
  *leaf* with `false`/`true` reflecting the probe-resolved outcome (`if ! command -v
  nginx; then …` → `if false; then …; fi` when present). `sh -n`-clean, leaf-granular,
  skips the dead branch. BUT this **replaces the F1 stopgap's "run the guard"
  (safe, live-rc) with "trust the probe" (recovers value, `tc-reliability` risk if the
  probe is stale/inaccurate)** — exactly the trade F1's stopgap was the conservative
  side of. **`tc-reliability` + `tc-phase` (FLAGGED, not resolved):** making that
  trade is a phase/orientation/reliability judgment (it is sound on apply only if the
  probe is trusted; the probe phase withholds), and I must not settle it alone.

### 4.5 Net: subsumption is NOT built; the F1 floor is shipped (the sanctioned outcome)

Subsumption (A) is blocked on: a query effect-category (Wall-1, `inc-7` richness), an
occurrence-typing analysis (Wall-2, `inc-6`, charter-deferred), and a render/`tc-*`
choice (Wall-3, variant-B-forced or a `tc-reliability` trade). Two are charter-
deferred analyses; one is a `tc-*` I cannot resolve in a single crate. **The prompt's
stated safe floor is "RUN the guard," which is the F1 stopgap — shipped.** Per the
charter's "abandon a direction at a wall to log it is equally valid," this is the
honest stopping point: I landed the sound capability (Half-B.1 interceptors), the safe
floor (F1), and mapped the subsumption wall to three named deferrals instead of
grinding a rabbit-hole or making an unsound `tc-*` call.

~SUSPECT durable: **F1 and Half-B are the same thread from two ends** (197 §2). F1
(block guard elision) is the cheap safe end; Half-B subsumption (resolve the branch
from the probe) is the value end — and the value end needs the query-category +
narrowing + render machinery the spike has deliberately deferred. So "the corpus is
thin on guarded books because the engine can't do them yet" (196 §2) is now precise:
it can RUN them safely (F1) and PROBE them (Half-B.1 ships the interceptor), but it
cannot yet ELIDE the guarded branch (subsumption) — and the gap is three named pieces,
not a vague "more work."

---

## 5. `tc-*` flags raised (for the orchestrator — do NOT let these be settled in a single crate)

- **`tc-mint` (§1.3):** `&&`/`||` left-operand status — post-condition establish
  (vouched, stays replaceable — matrix pin) vs guard (block). Left UNMARKED by the
  stopgap to preserve the pin; `TODO(tc-mint)` in `cfg.rs` `lower_and_or`. Decide
  whether the safer block-everything is worth losing the post-condition win.
- **`tc-reliability` + `tc-mint` (§3, S2):** body-elision *inside a guarded branch*
  keyed on the body's own cell — sound under a consistent host, unsound under an
  inconsistent one (guard-cell ≠ body-cell). Pre-existing, unchanged. Decide whether a
  leaf inside a conditional whose gating fact ≠ its own fact warrants a block.
- **`tc-reliability` + `tc-phase` (§4.4):** the guard-constant subsumption trade —
  "run the guard live" (F1 floor, safe) vs "replace with the probe-resolved constant"
  (recovers value, trusts the probe). The core Half-B `tc-*`.
- **F3 co-reference (§4.1, the priority-1 redline):** branch resolution (A) is sound;
  body-elision-via-guard (B) is the may-grade-promoted-to-must trap. Engine does NOT
  do (B); any future subsumption work must keep (A)/(B) separate.

## 6. Context other tracks must maintain

- **gw-4 (corpus):** S1 (§2.2, per-selector probe over-claim) and S2 (§3, body-in-
  guarded-branch elision) are uncovered state-space cells — pin them (xfail where
  unbuilt). The probe render changed (FLAT interceptors) — any e2e the corpus adds
  must bless against the new `<kind>__check` shape (all 26 goldens re-blessed here).
  gw-4 owns the non-guard surface; the guard cluster (`guard-status-blocks-elision`)
  is gw-1's — coordinate before adding guard cases.
- **gw-3 (backward/apply-3):** confirm the F1 block (`prove_replaceable`'s
  `Observable::Status` gate) carries through the real backward caller (§1.5 reverse
  axis is ~SUSPECT-only until a backward caller exercises it).
- **recursion-smoke / oracle tracks:** Wall-1 (§4.2, the query-vs-establish effect
  category) is the `inc-7` typestate work; the per-selector probe (S1) is the same
  oracle-shape pressure. These are where subsumption's prerequisites live.
- **adversarial-crosscheck:** the sharpest targets are S2 (§3 — is the body-in-branch
  elision *actually* sound, or did I miss an under-execute?) and the §1.3 `&&`/`||`
  narrowing (did leaving it unmarked leave a live wrong-elision via `cmd || install`?).

## 7. Code touched (absolute paths)

- `spike/crates/analysis/src/cfg.rs` — `Observable` doc; `lower_condition_region`
  gained `mark_status: bool`; `if`/`elif` path marks, `&&`/`||` does not
  (`TODO(tc-mint)`).
- `spike/crates/analysis/tests/cfg.rs` — four `consumed_*` engine-side F1 pins.
- `spike/crates/plan/src/lib.rs` — `prove_replaceable` blocks on `Observable::Status`;
  `ProbePlan::render_sh` = FLAT operand-bound interceptors; `check_fn_name` helper;
  `probe_render_binds_operand_flat_interceptor` test.
- `spike/crates/plan/tests/observable_matrix.rs` — the branch-vs-errexit A/B + the
  `tc-mint` gap pin.
- `spike/e2e/cases/guard-status-blocks-elision/` — the promoted (was xfail) executed
  case; `spike/e2e/cases/guard-elision-wrong/` removed; all 26 goldens re-blessed for
  the new probe render.
