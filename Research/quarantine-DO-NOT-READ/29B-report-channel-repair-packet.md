# 29B - Frozen phase packet: the owned report channel

The successor to `297`'s `phase-one-replace-pathname-report-plumbing`, which landed as a
DISABLE rather than a repair. This packet re-enables runtime report capture on a channel
Dorc exclusively owns. Design authority: `29A` section 4 (the mechanism, the residual
costs, the exclusion-check cells). Construction law: `AGENTS.for-builders-only.md`.
Build plan: `297`. This packet is the compression boundary - after it the work is
mechanical, and a question it does not answer is a STOP, not a judgment call.

Written by the security conductor 2026-07-24, against `ai/r29-catchup`@`396746e2`.

## 1 - What is wrong, in one paragraph

The deleted scaffold (`adef70d3`, recoverable in full from that diff) named a host
pathname, truncated it with `: >`, bound `DREP_V1` to it, read it back, and removed it.
Dorc owned none of that: a pre-positioned symlink turned the truncate into
arbitrary-file-destruction during the phase that promises no mutation, the read-back
accepted substituted content, and the removal could unlink an attacker's chosen target.
The oracle's own `>>` append is NOT part of the problem and does not change. The fix is
to make Dorc own the container before anything is written into it.

## 2 - The frozen shell shape

Exact. Do not improvise; if a construct will not pass `dash -n` or the two-binary floor,
STOP and report rather than substituting your own.

**Scratch root.** A module constant, `/tmp`, composed into
`/tmp/dorc-drep.<nonce>`. It is a controller literal. It is NOT read from the host
environment - no `TMPDIR`, no `HOME`, no `XDG_*` - because a host-supplied parent defeats
the ownership property (`29A` `rul-scratch-root-never-read-from-host`). An admin override
flag is explicitly OUT of scope; leave a one-line note where the constant is defined.

**Prologue.** Emitted ONCE, immediately after `records::header_line(...)` in
`ProbePlan::render_sh`, and ONLY when `self.checks.iter().any(|c| c.emits_report)`:

```sh
_dsc="/tmp/dorc-drep.<nonce>"; mkdir -m 700 "$_dsc" 2>/dev/null || _dsc=
```

`mkdir` is the whole repair. It creates exclusively and does not resolve a symlink at the
final component, so a pre-positioned anything makes it FAIL rather than clobber. `-m 700`
applies the mode at creation, with umask not applied, so there is no group- or
other-readable window. Failure empties `_dsc`, which is the degradation signal every later
site reads. Never retry, never fall back to another name, never remove what is already
there.

**Per-site**, replacing the deleted `record_scaffold_draining` (same call site, same
`emits_report` gate):

```sh
if [ -n "$_dsc" ]; then DREP_V1="$_dsc/<key>"; : >"$DREP_V1"; else DREP_V1=/dev/null; fi
<invocation>; _rc=$?
if [ "$_rc" -eq 0 ]; then _e=holds; elif [ "$_rc" -eq 1 ]; then _e=absent; else _e=cant-tell; fi
printf '<effect frame>\n' "$_e" "$_rc"
if [ -n "$_dsc" ]; then while IFS= read -r _dl; do printf '<report frame>\n' "$_dl"; done <"$DREP_V1"; rm -f "$DREP_V1"; fi
```

The `: >"$DREP_V1"` truncate is SAFE here and only here: the target is inside a directory
Dorc exclusively created at mode 700 this run. Pre-creating keeps the read-back simple
(a body that emits nothing yields an empty file rather than a missing one). `DREP_V1`
stays a plain shell variable - the check runs in the same shell, so no export is needed.
Each drained line remains ONE `printf` with the payload value-passed as `%s`, exactly as
before: a `%` or a space in an author's emission cannot corrupt the frame.

**Epilogue.** Emitted once, under the same `any(emits_report)` condition, at the end of
`render_sh`'s output:

```sh
if [ -n "$_dsc" ]; then rmdir "$_dsc" 2>/dev/null || :; fi
```

`rmdir` only removes an empty directory, so it cannot cascade. The `|| :` keeps a failed
cleanup off the script's exit status. Residue on a weird host is acceptable and disclosed;
a failed cleanup is never an error and never retried.

**Forbidden, permanently, in this lane:** `rm -rf` anywhere (an empty `$_dsc` would make
`rm -rf "$_dsc"/` catastrophic - `rmdir` plus per-file `rm -f` is the only cleanup);
any host environment read for siting; any second pathname operation after a failure; any
write outside `$_dsc`.

## 3 - Files and symbols in scope

- `spike/crates/plan/src/render.rs`, `mod probe` - re-add the draining scaffold under its
  previous name, plus the two new prologue/epilogue emitters. Read `adef70d3`'s diff for
  the shape that was deleted; the doc-comment there is largely reusable, but its
  scratch-path paragraph is now wrong and must be rewritten to describe ownership.
- `spike/crates/plan/src/lib.rs`, `ProbePlan::render_sh` - the `any(emits_report)`
  condition, the prologue/epilogue pushes, and restoring the per-check branch that chooses
  the draining scaffold over the plain one.
- `spike/crates/plan/src/lib.rs` tests - invert the pin (section 4).
- `spike/e2e/cases/report27-decline-static-classed/` and
  `spike/e2e/cases/decline27-tier3-dynamic/` - re-bless, and inspect the diff by eye.

NOT in scope, and a change to any of them is a STOP: `render_sh`'s signature;
`records::Framing`; the CLI; the ingestion/deframing side; `oracle::report`'s static
tiers; the entry path (`cli/main.rs` deliberately ignores `emits_report` under
entry-composition - that carve STAYS, per the human's ruling that the context-entry
machinery owns the in-context lane's permission story).

## 4 - The test inversion, specified exactly

This is the artifact that must not be got wrong. `emitting_auto_cell_never_constructs_a_report_path`
currently forbids the whole vocabulary; the repair brings most of it back, so a
too-permissive rewrite re-opens the hole with a green suite. Rename it
`emitting_auto_cell_owns_every_path_it_writes` and assert ALL of the following against a
rendered `emits_report` probe:

Must be present:
1. `mkdir -m 700 ` occurs exactly ONCE (the prologue; per-site creation would be a
   different, unowned-parent shape).
2. The mkdir's failure branch is present and empties the guard variable.
3. Every `DREP_V1=` assignment is one of exactly two spellings: rooted at `"$_dsc/`, or
   the literal `/dev/null`. Count them; assert nothing else matches.
4. `rmdir ` occurs, and `rm -f ` occurs.

Must be absent, permanently:
5. `TMPDIR`, and any other environment expansion in a scratch path (assert `TMPDIR`,
   `HOME`, `XDG_` are absent from the whole render).
6. `rm -rf` - anywhere, in any form.
7. Any redirect whose target is neither `/dev/null`, `"$DREP_V1"`, nor `$_dsc`-rooted.
   Implement this by scanning the rendered text for `>` targets rather than by listing
   known-bad strings: a denylist is what the old test was, and it is why the old test
   could not survive the repair.

Preserved from the old test:
8. The effect record still ships, byte-for-byte as asserted today.

Plus, unchanged and load-bearing: a probe with NO `emits_report` check renders
byte-identically to today (`empty-world-byte-identical`). Add that as its own assertion if
it is not already pinned - it is the guarantee that keeps golden churn to two cases.

## 5 - The one obligation this packet cannot discharge itself

`mkdir` failing `EEXIST` on a pre-existing SYMLINK rather than following it is the
property the whole repair rests on. The conductor verified refusal against a pre-existing
regular file and directory, with the canary intact, but the symlink leg could not be
verified on the development machine (msys copies rather than links). Verify it on a real
POSIX host - a three-line shell check is sufficient - and record the result in `29A`. If
it does NOT hold on some target family, STOP: the design is wrong there, not the code.

Do NOT encode that OS guarantee as a permanent test. It tests the operating system, not
Dorc, and it is flaky across the platforms this repo builds on. Our permanent net is
section 4's structural assertions, which pin that we USE `mkdir` first and degrade on
failure - the parts that are ours.

The happy path is already covered behaviorally for free: the two e2e cases execute the
rendered probe for real under the harness's sandbox, so a broken prologue or a broken
degradation surfaces as missing report records.

## 6 - Invariants that bind this work

From `spike/CLAUDE.md`: `rul-probe-writes-only-what-it-owns` (this lane IS its
enforcement), `two-phases-opposite-fail-directions`, `rul-only-oracle-bytes-ship`,
`probe-composition-walls`, `empty-world-byte-identical`, `inv-determinism` (the constant
is a constant; no clock, RNG, or env in the kernel), `two-binary-floor`,
`skip-banned`. From the quarantine: `sinv-owned-probe-channel` is the governing rule and
this packet is its construction; `sinv-integrity-failure-mutation` (a failed channel
degrades the LANE and never fails a plan or an apply).

## 7 - Order of work

1. Read `adef70d3`'s diff for the deleted shape.
2. Write the prologue/epilogue/scaffold emitters against section 2. Do not touch tests yet.
3. Invert the pin against section 4. Run it against the NEW render and confirm it passes,
   then deliberately break each of the four "must be present" properties in a scratch edit
   and confirm the test catches each one. Revert those scratch edits. A test that cannot
   catch its own property is worse than no test.
4. `cargo test -p dorc-plan`, then the full workspace.
5. Run e2e. Inspect the two churned cases by eye BEFORE blessing; confirm the only delta
   is the scratch lines. Any other case churning is a STOP.
6. The full gate set from `spike/CLAUDE.md`, cold clippy included.
7. Report: what you changed, what you verified, what you could not verify, and any
   judgment call you had to make (which should be none - if you made one, name it).

## 8 - Stop conditions

Stop and report rather than deciding, if: `emits_report` is not reachable at render time
in the shape section 2 assumes; the epilogue's position relative to the other record lanes
matters in a way this packet did not anticipate; any case outside the two named ones
churns; a construct fails `dash -n` or the floor shells; or you find yourself wanting to
change a signature, add a flag, read an environment variable, or use `rm -rf`.
