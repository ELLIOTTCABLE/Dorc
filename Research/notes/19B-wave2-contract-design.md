# 19B — Wave-2 contract design: command-keyed checks, the OOB verdict-lane, opt-B, and the defensive `command` oracle

> Re-seed orchestrator note; continues `19A §5` (the corrected abstract-interp-over-probed-observables
> model). AI-authored, confidence-marked; the *decisions* below are the human's rulings this session.
> Trust R/D/I/K over this.

## 1. Decisions locked (human, this session)

- **opt-B** — a *mutator*'s converged observables are a **separate oracle declaration**, not the check
  mimicking the mutator's rc. Aligns with the OOB model (§2): observables are *declared/emitted data*,
  never smuggled through an exit code.
- **Checks are command-keyed, invoked with the book's FULL args** (`q-1a`/`C-1`); the *oracle*
  argparses. Confirmed `q-1b`: command-keyed checks + named-kind *coordination* + fact-keyed *license*
  are orthogonal layers (`19A §5`).

## 2. The OOB channel — resolves the exit-code-collision (human point, this session)

Problem (human): no exit code can mean "unknown / can't-probe," because *every* code may be a meaningful
observable of the probed command. Resolution: **the check's exit code is never the channel.** Verdicts go
out-of-band on `$DORC_VERDICT` (the kCOMMS lane, `plans/142`) as **structured** records — `unknown` is a
distinct token, and the command's real rc rides the lane as *data* (`rc=2`). No collision (structured,
not an exit code). User-facing diagnostics go on the *separate* per-leaf freeform files — **signalling
never shares a lane with freeform** (the GitHub `set-output` injection CVE; kCOMMS P6).
- **No `.valid()` needed** (human floated it, was meh re: 2× execution). The single `.check()`
  **self-guards** — inspects args and withholds *before* executing anything mutative — so no double-run;
  and the real enforcement against a mis-firing check is the `kFAIL-withhold` **sandbox**
  (`an-withhold-monitor`), not trust in the check's branching.

## 3. The defensive `command` oracle (strawman — the flag-handling stress case)

`command`'s read-only-ness is **entirely flag-determined**: `command -v|-V NAME` is a read-only query;
`command [-p] NAME args` (no `-v`/`-V`) **executes** NAME (mutative). So the check may only probe the
`-v`/`-V` forms; the execute form withholds.

```sh
command__check() {                      # invoked: command__check <book's FULL args>, e.g. -pv nginx
   q=
   for a in "$@"; do
      case $a in
         --)       break ;;             # end of options → NAME next → execute form
         -*[vV]*)  q=1; break ;;        # option-group containing v/V → query form (handles -pv, -Vp)
         -*)       ;;                    # other option (-p): keep scanning
         *)        break ;;             # first operand, no -v/-V → execute form
      esac
   done
   [ -n "$q" ] || { printf '%s\tunknown\tnot-a-query-form\n' "$DORC_LEAF" >>"$DORC_VERDICT"; return 0; }
   out=$(command "$@" 2>/dev/null); rc=$?         # read-only; full args passed through untouched
   case $out in /*) v=present ;; *) v=shadowed ;; esac   # R2-SHADOW: real file vs shadowing fn/alias
   printf '%s\trc=%d\tstdout=%s\tverdict=%s\n' "$DORC_LEAF" "$rc" "$out" "$v" >>"$DORC_VERDICT"
}
```

Defensive against `-v`/`-V`/`-pv`/`--`/execute-form. The exact sh-spelling is the oracle's to choose
(`ch-shape-anno`); this is a strawman, not a committed syntax.

## 4. Stretch-goal finding (noted; NOT pursued — human ruled "move on")

The opt-A/B question was really **"do leaves need their own declaration-language?"** — and since **any**
command may be mutative, it is *not* mutator-specific. The latent insight: for some command-classes it is
cheaper to compute **"does this need to mutate?"** (a convergence check) than to compute **"its full
observable outputs."** A convergence-check-vs-full-output-production split. Real, but a **stretch goal**;
recorded as a round finding, deferred.

## 5. Wave-2 build sequence

- **build-1 (apply-side; DECISION-INDEPENDENT — uses *injected* observables):** rc-as-a-concrete-value
  in the apply domain + abstract-interpret/constant-fold the apply-CFG over `&&`/`||`/`if`/`!`/`case` +
  **observable-preserving substitution** (reproduce the exact rc — `true`/`false`/`(exit N)` — and
  consumed stdout, not always `:`). **Supersedes gw-1's F1 if/elif special-case** (the fold handles
  guards uniformly — F1 dissolves) and **fixes the `&&`/`||` under-execute** (gw-5's disposition xfail
  should XPASS → promote; the tripwire shows `mkdir` running via the fold + exact-rc substitution). Fits
  the worklist substrate (domain = probed-observable-or-⊤).
- **build-2 (oracle-contract side; needs §1–§3):** command-keyed `.check()` (full-args) + the OOB
  verdict-lane + opt-B observable-production declaration; wires *real* probe output into build-1's fold.
