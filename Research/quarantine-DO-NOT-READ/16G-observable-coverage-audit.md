# 16G — adversarial coverage-audit of the observable/replace matrix + the in-spike build

> **Status (2026-06-05): spike, round-16 — coverage audit + implementation scope.**
> Ran the `adversarial-crosscheck` skill (clean-context neutral + disowned-adversarial
> pair, un-seeded by the 16C–16F synthesis) to ensure `observable_matrix.rs` covers
> *all the angles* of the replace-a-converged-leaf problem. Both converged; every
> surviving gap **verified by tracing the real pipeline** (not relayed). This note
> records the audit + the bounded in-spike build it licenses. Append-only (round 16:
> …16F → 16G). Confidence-marked.

## 0. The model survives; the *coverage* did not
The pair (independently of 16C–16F) re-derived the observable model and **confirmed
16F**: a converged leaf is replaced by a `true`-stub; the stub defaults every
observable (effect→none, status→0, stdout/stderr→empty); a default is acceptable iff
*dead or vouched*; **effect←convergence, status←the `establishes` contract (free),
stdout/stderr←nothing**. The matrix's A/B (consumed-status fine, consumed-stdout not)
is right. **But the matrix only tested the *stdout* face.** The same unvouched-output
gate has several sinks the suite never exercised — that is the coverage hole.

## 1. Confirmed gaps (both agents; traced through `dorc` cli, `--has package:nginx`)
- **g-stderr (HIGH):** `apt-get install -y nginx 2> /tmp/e` then `cat /tmp/e` ⇒ install
  **REPLACED today**; the stub drops fd 2 → `/tmp/e` empty → `cat` diverges. The suite
  has *no* stderr case. fd 2 is unvouched exactly as fd 1 is.
- **g-fddup (HIGH):** `apt-get install -y nginx 2>&1 | grep -q nginx` ⇒ **REPLACED**;
  merged stream to grep lost. `2>&1`, `>&3`, `1>&3` are dup sinks the suite never tests.
- **g-redir-effect (HIGH):** `apt-get install -y nginx > /etc/marker` (no later read)
  ⇒ **REPLACED**; the `> /etc/marker` *itself* (create/truncate — `haz-redir-as-mutation`)
  is dropped with the line. Root cause: `command_effect` reads only `words`, ignoring
  `redirs`; `classify` maps `CfgNodeKind::Redir → Pure`. So a redirection is invisible
  to both the effect map and the replace gate. The matrix's
  `spec_…redirected_then_read` touches the *content* face; this is the *file-mutation*
  face, broader (fires with no reader).
- **g-toptop / hole-5 (MED, distinct mechanism):** `apt-get install -y nginx &` ⇒ the
  `&` ⊤-rejects (loud `syntax-unsupported` + `cfg-top-node`) **yet the install is still
  REPLACED**. `build_plan` never consults diagnostics, so a ⊤ in a leaf's own statement
  doesn't inhibit replacing it — an `inv-top-reject` breach at the plan layer. Benign
  for a converged no-op, latently unsound (a ⊤-context can change observability —
  `&` → async/`$!`/`$?`). NOT the observable gate; a separate ⊤-containment fix.

## 2. Confirmed *non*-gaps to PIN (lock the gate as a scalpel, not a hammer)
- **p-devnull:** `apt-get install -y nginx > /dev/null 2>&1` ⇒ REPLACED today and
  **must stay** — `/dev/null` is the discard sink; the gate must exempt it (else it
  over-runs, eroding the whole feature; 16F §5 / the neutral's gap-6 / adversarial mis-2).
- **p-oror:** `apt-get install -y nginx || systemctl start nginx` ⇒ REPLACED — status
  consumed by `||` (the dangerous dual of `&&`) is vouched for a conforming establish,
  so it stays replaced; the suite pinned `&&`/`$?` but not `||`.

## 3. The status boundary, re-derived (the 16E↔16F debate, settled by the suite)
The adversarial pushed mis-1: the status-vouch is an **unenforced** contract — the
engine never checks that an `establish`-declared `(provider,verb)` exits 0 when
converged (`mkdir d`, `useradd x` don't). True, and it **confirms 16F, not 16E**:
status is *not* an analyzer obligation (no "converged⇒nonzero" reasoning — 16F §4);
it is an oracle-contract claim, blast-radius-bounded (16D). The converged-nonzero cell
is genuinely un-exercisable in-spike (the spike never runs commands → the stub always
yields rc 0; the oracle model can't express converged-nonzero). So: **no status gate**;
the in-spike mitigation is the 16D *spotlight* — the `ReplaceLicense` records that its
status default is vouched-by-the-establish-claim (oracle-conditional, visible), so a
non-conforming oracle is a loud bounded bug, not a silent analyzer error.

## 4. The in-spike build (bounded; 16F §7 + this audit)
- **(A) observable-liveness gate** in `prove_replaceable` — generalize "stdout" to
  **any unvouched output sink**, conservatively + structurally (16F §5 surrogate; no
  value-plane, 16C brk-1): a leaf MustRun if it is a **non-last pipeline stage** OR has
  a **stdout/stderr redirection to a non-`/dev/null` target** (file, append, or fd-dup).
  This is the single gate behind g-stderr, g-fddup, g-redir-effect, and the three
  existing `spec_*stdout*`. Status consumers (`&&`/`||`/`$?`/`if`) do NOT trigger it.
  Computed during CFG lowering (pipeline-position + redirs are local there), exposed on
  `Cfg`, consumed by `prove_replaceable`. **Strong-typed** (the steer): model the
  leaf's consumed observables as a type so a license is *unmintable* when an unvouched
  one is consumed (prevent), and record each default's vouch-source in the license's
  derivation (spotlight, the 16D angle without `Grounded<T>`).
- **(B) builtin-pure effect fix** in `command_effect` — a small blessed set of
  target-state-pure builtins (`set`, `cd`, `:`, `true`, `false`, `[`/`test`, `echo`,
  `export`, `read`, …) ⇒ `Pure` not `Opaque`, so they don't poison system-state
  ambient-ness (fs-4; `spec_set_e`). Sound: they touch shell-env/stdout, never an
  oracle-modeled fact (and `echo`'s stdout is gate (A)'s job, orthogonal). Note: the
  poison is *broader* than `set -e` (every un-oracled token poisons); blessing the
  common pure builtins recovers the common case, the rest stays the safe (over-refuse)
  direction.
- **(C) rename** skip→replace, channel→observable (16F §1): `SkipLicense`→`ReplaceLicense`,
  `prove_skippable`→`prove_replaceable`, `Disposition::Skip`→`Replace`, `is_replaced`
  localizes the matrix check.
- **(D) hole-5 / ⊤-containment** — ADD the spec (coverage); FIX only if cheap, else
  leave `#[ignore]`d as a documented gap (it is a ⊂-handling/parser concern — a leaf
  whose statement contains a ⊤ should be MustRun — distinct from the observable gate).

## 5. New matrix cases (the audit's coverage delta)
- specs (fixed by gate A): `spec_converged_stderr_to_file_must_run`,
  `spec_converged_stderr_merged_piped_must_run`,
  `spec_converged_redirect_is_an_effect_must_run`.
- pins (lock the scalpel): `pins_converged_devnull_discard_replaced`,
  `pins_converged_status_via_oror_replaced`.
- spec (deferred-fix, ⊤-containment): `spec_topcontext_background_leaf_must_run`.

## 6. Method note (held)
Ran the `adversarial-crosscheck` skill verbatim: clean-context pair, no third
optimistic pass, disown-the-artifact + own-the-doubt, agents NOT given 16C–16F (so
they re-derived independently). Convergence treated as the trustworthy signal;
adversarial-only claims (hole-5, mis-1) traced before acceptance. Every "REPLACED
today" above was produced by running the built `dorc` binary, not relayed.

**NOTES INDEX:** …16D degradation lens · 16E state/CFG read-write · 16F observable/
replace model · 16G (this — coverage audit + the bounded in-spike build).
