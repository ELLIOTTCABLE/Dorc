# 27P — shim-materialization landing (the `274` §5 last mile) + residue

AI-authored (Opus builder, r27 shim-materialization session, 2026-07-17). Records what landed for
the shim-materialization last mile — the per-run PATH shim that makes an entry-composed probe
EXECUTE for real (`274` §5 / `27L` task-14 / `27N` disposition-shim-materialization-deferred, the
block-stdlib package's first errand). Authority: root docs + `spike/CLAUDE.md` rulings +
`271`/`274`/`27C`/`27D`/`27L`/`27N`/`27O` outrank this. Companions: `274` §5 (THE shim design + DST
story) · `27L` §5 (the pure/DST-clean `plan::shim` model + the fixture constraints) · `27N` (the
entry-composed emission this executes; §6 shim disclosure) · `27O` (the carry lane whose fixtures
also ride the probe path — its Carry checks cross NO boundary, so they materialize NO shim).

## Branch / fold state (READ FIRST — the conductor must reconcile)

- Branch `ai/r27-shim-materialization`, based on `ai/r27-fallback-carry` @ **`81092f1`** (verified at
  step-zero; the lineage ref `ai/spike3-r27` is FROZEN at `1aecaa3` — the human away, ref-moves
  hook-reserved). STACKS on the fallback-carry chain per the `27D` stacked-fold push-through directive.
- Tip **`bd6cd88`**. 4 commits (`3dabad5..bd6cd88`), oldest→newest:
  1. `3dabad5` (new plan) — `ProbePlan::shim_files` (the pure per-run shim FILE SET).
  2. `cb07875` (new cli) — `--shim-dir` writes the shim files at the cli edge.
  3. `da3cf36` (new e2e) — thread the per-run shim dir into gate-1's probe exec.
  4. `bd6cd88` (re e2e) — babby-elides/diverges probe runs REAL (`-n`-keyed pass-through sudo mock,
     diverges argument-sensitive `hork`, drop `PROBE_RESULTS=authored`).
- NOT rebased (hook-reserved to the human). HOLD at tip. Behind-count vs `ai/r27-fallback-carry` = **0**
  (a pure fast-forward when folded).

## Acceptance summary (all green at the tip)

- **Four gates clean** on the whole workspace: `cargo fmt --check` · `clippy --workspace
  --all-targets -D warnings` (0 warnings) · `cargo deny check licenses bans sources` · `typos spike`.
- **Unit: 884** (was 881; +3: the `shim_files` battery). No failures.
- **e2e: 91/91** foreground (fresh binary). The **89 non-babby cases are BYTE-STABLE** (rung-0). The
  two acceptance cases (`context-entry-babby-elides`, `context-entry-babby-diverges`) now run the
  REAL probe — `PROBE_RESULTS=authored` DROPPED, gate-1 parity + vouch-closure + gate-6 dual-rail all
  active — with ZERO golden churn (no `expected.out` / `expected.ran` changed; see delta classes).
- **No new trust surface**: this lane is execution PLUMBING only — no licensing, no verdict paths, no
  analyzer changes. Every shim-resolution failure lands exactly where it does today: rc 127 ⇒
  cant-tell ⇒ can't-say ⇒ run (proven by the anti-masking check below).

## The mechanism (what "materialize" means here)

The `27N` entry-composed probe ships `sudo__enter hork__is_converged 'install' 'frob'`, where
`sudo__enter() { sudo -n "$@"; }` EXECs its guest as a fresh process. A shell function does not
survive `exec`, so `sudo -n hork__is_converged install frob` cannot resolve `hork__is_converged` at
the guest position — under stock/mock sh it 127s (`27N`: can't-say ⇒ run, degrade-safe). The `274`
§5 answer, now built:

1. **`ProbePlan::shim_files()` (pure, `plan`)** — the per-run shim FILE SET as a
   `BTreeMap<name, content>`. For each check whose `entry` has NON-EMPTY `enter_defs` (a real entry
   form that execs — `WrappedProbe::Enter`), it materializes every EXEC'd guest: the inner check, plus
   every enter form AFTER the outermost (the outermost runs as a funcdef in the probe shell). Each
   shim = `#!/bin/sh\n<oracle stripped funcdef verbatim>\n<fn> "$@"`. The funcdef is the SHIPPED
   `inner_sh`/`enter_defs` bytes (`271:rul-only-oracle-bytes-ship`); the shebang + argv-dispatch are
   synthesized scaffolding (`probe-composition-walls` — never book bytes). Deterministic content +
   `BTreeMap` ordering (`inv-determinism`). EMPTY for a wrapper-free / Carry-only run
   (`empty-world-byte-identical` — no shim dir).
2. **`--shim-dir=DIR` (cli edge)** — `materialize_shim_dir` writes the set into DIR after
   `compile_probe` (`io-at-edges-only`; the kernel stays pure). A pure side-effect: stdout is
   UNCHANGED (`two-surfaces`). On unix it `chmod 0755`s each file (cfg-gated `PermissionsExt`); on
   other platforms the write suffices and the session supplies the bit.
3. **e2e harness (session-establishment)** — the main round-trip runs with `--shim-dir=$_shimdir` (a
   per-case `mktemp -d`), so it ALSO writes the shims (empty for non-entry cases). `probe_exec_check`
   runs the probe with `PATH="$_mocks:$_shimdir"` — **MOCKS-FIRST** (mocked tools keep winning; the
   shim adds only the DISJOINT oracle-check names `*__is_converged`), `chmod +x` the shim dir (msys
   safety net), cleaned up at case end.

### PATH-and-mocks composition (`-n`-keyed sudo mock — the load-bearing fixture design)

The probe and apply share ONE `mocks/sudo`. An always-pass-through sudo mock would inject the peeled
guest into the BARE run (`sudo hork install frob` → `hork install frob`), which gate-6's
surface-argv ledger cannot attribute ⇒ false-fail. The fix: the mock passes through **iff `-n`** — the
`27C` non-interactive PROBE entry form (`sudo__enter() { sudo -n "$@"; }`) EXECs its guest so the
in-context check runs for real; a plain `sudo <guest>` (the APPLY's real escalation) logs and exits,
identical to every other sudo mock. So the apply/bare run-set is BYTE-IDENTICAL to the authored shape
while the probe resolves the shim. (Disclosed spike model — see gaps.)

## Per-case golden delta classes

- **89 non-babby e2e**: NO delta (byte-stable; `empty-world-byte-identical` — `shim_files` is empty
  for them, `--shim-dir` writes nothing, PATH unchanged). Includes `context-entry-wrapped-guard` and
  the three `wrapper-*` cases — their probes either ship no entry form or stay `PROBE_RESULTS=authored`
  (gate-1 b/c skipped), so the shim on PATH changes no emitted record.
- **`context-entry-babby-elides`**: delta = **fixture-infra only** — `mocks/sudo` → `-n`-keyed
  pass-through; `PROBE_RESULTS=authored` DELETED. `expected.out` + `expected.ran` (empty — both
  elide) + `probe-results.txt` UNCHANGED (zero golden churn).
- **`context-entry-babby-diverges`**: delta = **fixture-infra only** — `mocks/sudo` pass-through;
  `mocks/hork` argument-sensitive (`query frob` ⇒ rc 1); `PROBE_RESULTS=authored` DELETED.
  `expected.out` + `expected.ran` (`ran: sudo hork install frob`, preserved by the `-n` keying) +
  `probe-results.txt` UNCHANGED (zero golden churn).

## Which cases went real vs stayed authored (and exactly why)

- **WENT REAL** (the lane's acceptance): `context-entry-babby-elides` (site 0 ambient `hork query
  wombat` ⇒ holds; site 1 wrapped `sudo -n hork__is_converged install frob` ⇒ shim ⇒ `hork query frob`
  ⇒ holds — both elide) and `context-entry-babby-diverges` (site 0 holds; site 1 ⇒ shim ⇒ `hork query
  frob` ⇒ rc 1 ⇒ absent ⇒ runs). The REAL probe records now DRIVE the plan; they match the former
  authored fixtures exactly.
- **STAYED AUTHORED** (honest, disclosed): `context-entry-unvouched-runs` and
  `context-entry-noescalation-runs` ship NO entry-composed probe — site 1 degrades to run per the
  consent trace (unvouched / `--no-probe-escalation`), so their probe is site-0-ambient only and the
  shim is NOT exercised. `context-entry-wrapped-guard` ships the entry form but is a `27O`
  guard-render fixture (`PROBE_RESULTS=authored`); its probe now materializes the shim (its artifact
  carries `__enter`) but its sudo mock is unchanged (log-and-exit), so the shim file is present-but-
  unexec'd and every record is unchanged — it stays byte-stable and authored. These three are
  ORTHOGONAL to the shim lane; converting them is a trivial non-shim follow-up, out of this acceptance.

## Acceptance evidence (real-probe execution, verbatim)

Running each case's rendered probe under `env -i PATH=<mocks>:<shim> dash`, deframed:

- `context-entry-babby-elides` → `site 0 effect=holds rc=0` · `site 1 effect=holds rc=0`
- `context-entry-babby-diverges` → `site 0 effect=holds rc=0` · `site 1 effect=absent rc=1`

Both match the (former) authored `probe-results.txt` byte-for-byte. **Anti-masking (shim is
load-bearing, `anti-masking-tests`):** re-running the elides probe under `PATH=<mocks>` WITHOUT the
shim yields `site 1 effect=cant-tell rc=127` — the unresolved guest across the exec boundary, which
gate-1 (c) vouch-closure catches loudly. So the passing result is the REAL check resolving through
the materialized shim, never a lying mock.

## Comment budget (`24P` §8 rider) — numbers + commands

- Denominator (added lines, `git diff 81092f1..HEAD --numstat` summed): **225**.
- Numerator (added Rust `//` non-doc + authored sh `#` non-shebang, EXCLUDING `///` and generated
  goldens/copied logs): **22 ⇒ 9.8%** (rust `//` = 8; sh `#` = 14). Commands:
  `git diff 81092f1..HEAD -- '*.rs' | grep '^+' | grep -v '^+++' | grep -cE '^\+[[:space:]]*//[^/]'` = 8;
  `git diff 81092f1..HEAD -- 'spike/e2e/run.sh' 'spike/e2e/cases/*/mocks/*' | grep '^+' | grep -v '^+++' | grep -cE '^\+[[:space:]]*#[^!]'` = 14.
- `///` doc-comments (mandated on public items — `shim_files`, `shim_dispatch_script`, the
  `--shim-dir` field, `materialize_shim_dir`, the `EntryComposed` shim-seam doc): **33**, excluded
  from the numerator per the `27N` disposition-comment-budget correction; checked by eye at tip.

## tc-flags (flagged UP, NOT settled)

- **`tc-shim-materialization-in-harness-not-executor`** — in the spike the e2e HARNESS is the
  "session" that materializes the shim (`--shim-dir` + `chmod +x` + PATH-weave). The `274` §5 design
  homes materialization at real session-establishment (atomic write-then-rename, smoke-test,
  PATH-prepend, cleanup, the hostsim command-registration seam). The `plan::shim` model (`27L`) and
  this `shim_files` product are the reusable pure halves; the real-executor I/O choreography (which
  process writes, when, the smoke-degrade, cleanup-on-crash) is a real-executor-era judgment, not
  settled here.
- **`tc-sudo-mock-passthrough-keyed-on-dash-n`** — the pass-through-iff-`-n` fixture model cleanly
  separates probe entry (measure in-context) from apply escalation (log the mutation), but it keys on
  the specific entry-form spelling `sudo -n "$@"`. A different authored entry form (e.g. `sudo -k`,
  or an entry that omits `-n`) would not pass through under this mock. Fine for the babby-sudo
  fixture; the stdlib mock-authoring template should state the coupling. Flagged, not settled.

## Disclosed gaps / deferred pieces (`ru-26` / `churn-avoidance-disclosure`)

- **`dorc:sh` reentry shim (the colon case) — STILL DEFERRED.** This lane materializes the
  entry-composed `*__enter`/`*__is_converged` shims (valid filenames, resolve on msys — verified). The
  `274` §5 `dorc-sh` evaluator shim for `dorc:sh` reentries remains the `27L`
  finding-dorc-colon-unmockable-on-windows constraint (a colon is illegal in a Windows filename) — a
  separate follow-on, untouched here.
- **Env-scrubbing sudo (the `274` §9 PATH-weave soft spot).** The mock sudo preserves PATH so the
  shim resolves; a REAL env-scrubbing `sudo` (`env -i PATH=…` in its authored body, or a secure_path
  that excludes the shim dir) would 127 the guest ⇒ can't-say ⇒ run (safe, but value-zero for that
  reentry). Disclosed in `274` §9; the spike mock deliberately preserves PATH to exercise the happy
  path. The hint-owed (a marked delegation silently 127ing under a PATH scrub) is a real-executor lint
  item, not built.
- **Multi-link chains.** `shim_files` materializes every exec'd guest of an N-link chain
  (`chroot__enter` + inner check for a `sudo→chroot→pipx` chain — unit-pinned), but the babby
  acceptance is single-link; multi-link entry composition + cross-link ρ-threading stay `27N`-deferred.
- **Windows exec-bit.** Rust `fs::write` cannot set a unix exec bit on Windows; the harness `chmod +x`
  covers the msys probe run. A real Windows executor would need its own exec-bit story (out of spike
  scope — the whole `274` §5 Windows/noexec story is the session-level shimless-degrade).

## What the stdlib era must maintain

- **The shim is oracle-bytes-only.** `shim_files` wraps the SHIPPED `inner_sh`/`enter_defs` verbatim
  (`271:rul-only-oracle-bytes-ship`); the stdlib's `<tool>__is_converged` bodies become shim scripts
  as-authored. A body that is not self-contained (reads an ambient var the entry form scrubs) will
  127/misbehave across the exec boundary — the same PATH-weave soft spot; the quality bar should note
  it.
- **The entry form's `-n` (or its non-interactive equivalent) is the probe seat.** The mock keys
  pass-through on it; the stdlib `sudo__enter`/`su__enter` authoring template already spells
  `sudo -n "$@"` (`27N`). A stdlib entry form that changes that spelling must update the mock model.
- **`--shim-dir` is the reusable edge seam.** The real executor's session-establishment should call
  the same pure `ProbePlan::shim_files()` and materialize identically (deterministic content +
  ordering); only the WRITE choreography (atomicity, smoke-test, cleanup) is executor-specific.
- **`empty-world-byte-identical` holds by construction.** No wrapper oracle ⇒ no entry-composed check
  ⇒ empty `shim_files` ⇒ no shim dir ⇒ byte-identical probe/apply. Every future brief touching probe
  emission keeps this.
