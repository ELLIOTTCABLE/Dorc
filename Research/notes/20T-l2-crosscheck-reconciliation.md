# 20T — L2 member-precision crosscheck reconciliation

> Hostile single-agent pass (Fable) on commit 2676ad3, briefed from the builder's own
> hunt-list (20S §10) plus its own angles; every claim run through the built binary AND
> checked against real dash on pure-builtin analogues. Verdict first: **the self-reach
> fixed-point rationale is sound** (its words: "the one crack is not in the fixed-point
> logic at all — it's upstream, in the eligibility model"). One priority-1, fixed same-day.

- **find-cd-pwd (priority-1, demonstrated, FIXED):** `cd` rebinds `$PWD`/`$OLDPWD` but
  was modeled as writing nothing — correct on the fact-cell axis (it establishes no
  facts, so it doesn't poison self-reach) but a lie on the var axis. `for PWD in aaa
  bbb; do cd /tmp; apt-get install -y "$PWD"; done` probed `package:aaa`/`package:bbb`
  and elided while dash installs `/tmp` twice — a kFAIL-perform violation, exactly the
  member-override's one blind spot (a "pure" body command that rebinds the for-var).
  Realism near-nil (`for PWD in` is perverse); the hole was genuine regardless. **Fix
  (orchestrator, direct):** `simple_writes_var` gains `cd ⇒ {PWD, OLDPWD}`; swept the
  same class while there: `getopts` now writes `OPTIND`/`OPTARG` unconditionally (the
  doc-comment had claimed this; the code hadn't), and a dynamic `getopts` name-operand
  is a conservative write (mirroring the read-family arm). Three unit pins, including
  the non-degrade pole (`for PWD in …` WITHOUT `cd` stays a Members family — PWD is an
  ordinary var until something rebinds it). No e2e case: the corpus pins user-realistic
  shapes; the eligibility refusal IS the entire fix surface and the units pin both poles.
- **Did-not-survive (the valuable negatives, all genuinely constructed):** bare-`read`
  `$REPLY` (dash errors on bare `read` — the hole is bash-only); `eval`/brace-group/
  `$()`/function-call reassigns (⊤-reject, span-scan, or Opaque backstop each catch);
  accumulation vars and second-var perturbation (any pkg-dependent var is already ⊤ ⇒
  Opaque ⇒ all-or-nothing refuses); multi-command one-line bodies (fall to verbatim-run,
  safe); empty body / empty list (vacuous-but-harmless, render dash-clean); while-`$?`
  dash facts all validated incl. nested + zero-iteration; member rc firewall closed by
  construction (`Establish` hardcoded ⇒ rc withheld); duplicate members (per-SITE
  suppression means no self-staling, cli merges by FactKey); cross-leaf joint elision
  (pairwise refusal via back-edge visibility — the compose-unsoundly worry is moot);
  split-literal and quoted-glob member sources behave dash-faithfully.
- **Suite verified by the pass itself:** 347 tests, 66/66 e2e ×2 — and re-verified by
  the orchestrator post-fix (348+ with the new pins; 66/66 ×2; fmt/clippy/typos clean).
- Residual notes: the while/until branch of the `$?` fix gates nothing today
  (while-bodies stay floored) — its value is general correctness + forward-looking;
  the in-loop floor's remaining masked latents stay recorded in 20O find-6.
