# 21B — arch-6: the H2SaLS coverage dashboard (design-as-built + findings)

> Build agent note, round-21 arch-6. Append-only; this is the dashboard's design-as-
> built, the H2SaLS rollup, census discrepancies, the seam wishlist, and the
> adversarial hunt-list. Confidence-marked per the house rules (+SURE / ~SUSPECT /
> -GUESS / --WONDER). Charter: quarantined round-21 priming prompt; arcs restated in
> `211` §1 (arch-6) and `20V` §7 (the doors-decompose-the-north-star charter).

## §0 What landed (paths)

New crate `crates/coverage` (binary `dorc-coverage`), consuming the other crates as
libraries:
- `crates/coverage/Cargo.toml` — workspace member (the `Cargo.toml` `members` line is
  the only edit outside the crate + tools + this note).
- `crates/coverage/src/lib.rs` — the attribution core (pure; `Door`/`BlockReason`/
  `Rung`/`Analyzable`/`SiteRow`/`Report`, `build_report`). 18 unit tests incl. one
  per door pinned against a known disposition.
- `crates/coverage/src/weights.rs` — the criticality-weight adapter seam (line-count
  stand-in today; `from_line_scores` is the future-1A swap-point).
- `crates/coverage/src/main.rs` — the I/O-edge binary: aligned table + stable-column
  TSV.
- `crates/coverage/README.md` — invocation + the door vocabulary.
- `tools/coverage.sh` — the gate-set wrapper (NOT wired into `e2e/run.sh`; never fails
  a build — exits 0 even when the binary is missing).

Gate set green ×2 in the worktree: `cargo fmt --check` · `clippy --workspace
--all-targets -D warnings` · `cargo test --workspace` (all green incl. 18 coverage
tests) · `sh e2e/run.sh` ×2 (75/75 both) · `typos` (clean). +SURE.

## §1 Design-as-built

### What it answers (charter c1–c4, per command-site)
- **c1 analyzable-without-⊤** — a TRI-state `Analyzable {Yes, Indeterminate}` (NOT a
  bool — see seam-1). `Yes` iff a fact resolved; `Indeterminate` for a `MustRun`.
- **c2 oracled** — did an oracle `check()` + effect-map resolve a fact (any
  `Establish`/`Query`/`Members` class).
- **c3 probed-converged** — the site's Effect `Verdict` when probe-results supplied,
  else `-`.
- **c4 disposition + DOOR** — the `Door` enum, the heart of the instrument.

### The door vocabulary (`20V` §3–§4, §7 — full-elisions and transforms NEVER blurred)
Mapping from engine state to door (all matches `_ =>` into an honest `Unattributed`
bucket per the charter's evolution-survival rule):

| door | engine signal | charter |
|---|---|---|
| `fold` | `Disposition::Omit` | door-1: dead control-flow under a measured guard rc |
| `dead-invariant` | `Disposition::Replace` + site consumes `Channel::StatusInvariant` | door-3: `cmd \|\| true` |
| `replace-converged` | `Disposition::Replace` via `LicenseVia::{ConvergedEstablish,MembersLoop}`, NOT invariant | plain converged-establish elision |
| `query-substituted` | `Disposition::Replace` via `LicenseVia::QueryGuard` | read-only guard value-substituted to its probed rc |
| `guard-transform` | — (door-4 unbuilt) | **column reports 0**; kept stable |
| `static-declared` | — (door-2 unbuilt) | **column reports 0**; kept stable |
| `runs` | `Disposition::Run` + dominant `BlockReason` | the residue |
| `unattributed` | unrecognised `Disposition`/`LicenseVia` | the blind-spot bucket |

The **door-3 discriminator is load-bearing and subtle** (+SURE, validated against the
e2e `door3-or-true-elides`): a door-3 site is a `Replace` whose `LicenseVia` is
`ConvergedEstablish` (the engine mints it that way), so `via` alone can't distinguish
it from a plain `replace-converged`. The distinguisher is the *consumed channel set*
containing `StatusInvariant`. I read `cfg.consumed_observables(node)` and check it
**before** dispatching on `via`, so the `|| true` shape is never mis-bucketed as a
plain elision. (This is exactly the `20V` §7 "never blur" requirement made mechanical.)

`query-substituted` is its OWN column, distinct from both full-elision and
guard-transform (`20V` §7 spirit): a Query guard mutates nothing, so its substitution
is neither a mutation-elision (it has no mutation to be already-done) NOR door-4
(which *inserts* a guard before a bare mutator). Blurring it into either would
mis-state the north-star. I count it as a full-elision for the run-set-shrink fraction
(the guard's run is genuinely removed) but it is reported separately.

### The dq-2 rung split (`20V` §6)
Per the charter: r-2/r-4-shaped (elision reachable via a readable guard/wrapper idiom)
vs r-3-shaped (a bare mutator needing an oracle declaration). My mapping:
- `Rung::GuardReadable` ← `fold` / `query-substituted` / `dead-invariant` /
  `replace-converged` (all reachable from idioms or the engine's own value-substitution,
  zero declaration needed).
- `Rung::NeedsDeclaration` ← a `Runs(ConsumedStatusTop)` site that is an
  `EstablishAmbient`/`EstablishMembers` (the bare-mutator-under-`set -e` shape door-2/
  door-4 would move), plus `guard-transform`/`static-declared` when they land.
- `Rung::NotApplicable` ← everything else that runs (diverged/unprobed must run
  regardless; ⊤-trigger/no-oracle/output/loop are not declaration-rescuable).

### Criticality weighting (line-count stand-in)
Weight = the site's source-span line-count (a one-liner = 1; a heredoc-bearing or
multi-line leaf weighs its lines), per the charter "criticality = line-count
stand-in". `weights::Weights::from_line_scores` is the CLEARLY-MARKED adapter seam for
the future 1A matrix: hand it `line → score` and nothing else changes. Today
criticality-weight ≈ count-weight (most leaves are one line), so the two rollup columns
agree except where heredocs/multi-line commands inflate weight (e.g. the H2SaLS
`block_body=$(cat <<'EOF' …)` weighs 42).

### Determinism + evolution-survival
Pure kernel (`inv-determinism`): `BTreeMap` everywhere, sites sorted by `LeafId.0`, no
clock/RNG. The binary is the only I/O edge. EVERY engine-enum match ends `_ =>
Unattributed(<debug-repr>)` with an `#[expect(unreachable_patterns)]` (the arms are
unreachable TODAY because the enums are closed; the `#[expect]` makes the crate warn
LOUDLY the moment a new variant lands — `SkipClass::InlineCall`, a new `Channel`, a new
`LicenseVia` — so the gap surfaces instead of silently miscounting). This was the
charter's hard constraint and it cost real friction: the workspace `-D warnings` gate
treats the catch-all as `unreachable_pattern`, so each is silenced with a reasoned
`#[expect]` citing the charter. **Priority-tension surfaced** (per the global rules):
charter-mandated future-proofing (priority: maintainability/correctness-of-the-tool)
directly conflicts with the workspace's zero-dead-code lint posture. I resolved toward
the charter (the dead arms stay), which is the right call for an instrument meant to
outlive the engine's current enum shape.

## §2 The H2SaLS rollup (the headline result)

Run: `dorc-coverage --book=harden.sh -o <each oracles/*.oracle.sh>`. 195 sites, 11
oracles. Identical result with NO probe-results and with an ALL-CONVERGED synthetic
probe (the convergence ceiling) — convergence changes nothing here, which is itself the
finding.

```
## per-door rollup (count | criticality-weight)
door               count  crit-wt
fold               0      0
dead-invariant     0      0
replace-converged  0      0
query-substituted  0      0
guard-transform    0      0
static-declared    0      0
runs               195    271
unattributed       0      0

## north-star (full-elision vs guard-transform — kept separate)
   full-elision     :   0 sites · crit-wt    0
   guard-transform  :   0 sites · crit-wt    0   (door-4; 0 until it lands)
   total sites      : 195 sites · crit-wt  271
   crit-weighted full-elision coverage : 0.0%

## dq-2 rung split
rung               count  crit-wt
guard-readable     0      0
needs-declaration  1      1
not-applicable     194    270
```

### Why 0% — and why that is the RIGHT answer (the decomposition the charter wanted)
The north-star ceiling here is `oracle-coverage × declaration-coverage`, NOT engine
quality or guard-idiom density (`20V` §7 / `211` §1). The 0% decomposes into four
separately-ownable, all-correct causes (+SURE, cross-validated against the cli — see §3):

1. **Oracle coverage is thin against this deliberately-un-annotated book.** Only **4 of
   195 sites are oracled** (c2=yes): installs of `sudo`/`gpg`/`lynis` and the `groupadd`
   loop. The README states H2SaLS is "deliberately NOT annotated for Dorc"; the 11
   oracle seeds cover a handful of the 33 distinct external commands. 191/195 sites are
   `no-oracle` → run. This is the thesis's intended dependency working as designed.
2. **The 3 oracled installs are `written-upstream`, not ambient.** `apt-get update`
   (lines 51/126/637/659/662) is INTENTIONALLY unmodeled by the H2SaLS `package.oracle.sh`
   (its header: "um-pkg-3 … `update` therefore resolves no effect ⇒ runs"; there is no
   `pkgindex` oracle in the set). So `apt-get update` → Opaque → `Reach::Top` → every
   downstream install is `EstablishWritten` (stale resting probe) → runs regardless of
   convergence. This is the DESIGN "opaque poisons ambient-ness" cost, correctly surfaced.
3. **The 1 oracled converged-establish (`groupadd`) is `consumed-top-status` under
   `set -eu`.** Even all-converged, its rc is `StatusRelaxable`-consumed by errexit and ⊤
   (`fork-mutator-rc`), so it blocks — the EXACT errexit-elision headline cost the doors
   program exists to address. It is the lone `needs-declaration` (r-3) site: door-2/door-4
   would move it. This is the dashboard's single most pointed signal.
4. **Every `cmd || true` door-3 site is on an UN-ORACLED command.** H2SaLS has 5
   `|| true` sites (`rkhunter --update`, `rm`, `wget`, `lynis update info`, `lynis audit
   … | ansi2html`), all on tools with no oracle ⇒ no `EstablishAmbient` ⇒ no convergence-
   elision license ⇒ door-3 can't fire. door-3's payoff REQUIRES oracle coverage of the
   wrapped command. ~SUSPECT this is the single highest-leverage corpus lesson: door-3 is
   "free" only once the wrapped tool is oracled.

The `unattributed` bucket is **0** here AND **0 across all 75 e2e cases** (§3) — the
attribution is complete against the current engine; the 0% is not an attribution blind
spot, it is a real oracle-coverage-bound ceiling.

## §3 Cross-validation (the attribution is not lying)

- **vs the cli, on H2SaLS**: I ran `dorc --book=harden.sh -o … < all-converged-probe`
  and counted elided lines: **0** — byte-for-byte agreeing with the dashboard's 0%
  full-elision. The dashboard reads the same `build_plan` dispositions the cli renders.
  +SURE.
- **vs the cli, per-door on `headline-guarded-realistic`**: the dashboard attributes
  site 1 (`dpkg -s ca-certificates`, holds) `query-substituted`, site 2 (`dpkg -s nginx`,
  absent) `query-substituted`, sites 3–11 `runs`; the cli's apply substitutes exactly
  those two guards to `true`/`false` and runs the rest. Exact match. +SURE.
- **corpus-wide door totals across all 75 e2e cases** (via `tools/coverage.sh`):
  `fold=4 dead-invariant=1 replace-converged=37 query-substituted=6 guard-transform=0
  static-declared=0 runs=128 UNATTRIBUTED=0`. 48 sites elide, every one door-attributed,
  zero unattributed. +SURE. The per-door unit tests pin one known disposition each
  (door-3 elides-vs-diverged-runs, door-1 fold, members→replace-converged, the errexit
  consumed-top-status, written-upstream, query-substituted).

## §4 Census discrepancies (charter (b): spot-verify ≥5 commands)

The census (`Research/corpora/H2SaLS/census/commands.tsv`, built partly under a
reduced-capability agent) is a **command-TOKEN frequency** count (174 tokens with
line-lists); my report is **plan/apply SITES** (195). The granularities differ by
design, so where they diverge it is a finding about WHAT each measures, not a bug to fix.

Five-command line-number spot-verify (census line-list vs my site enumeration):

| command | census lines | my sites | verdict |
|---|---|---|---|
| `apt-get` | 51,54,126,127,130,637,638,640,659,662,663,665 | all 12 ✓ | EXACT |
| `groupadd` | 58 | 58 ✓ | EXACT |
| `useradd` | 65 | 65 ✓ | EXACT |
| `visudo` | 102 | 102 ✓ | EXACT |
| `ufw` | 183,184,409,410,413,414,417,421 | all 8 ✓ | EXACT |
| `service` | 188,628,686,689,692,695 | all 6 ✓ | EXACT (bonus) |

Two commands diverge — **both correctly, both my analyzer being honest, NOT census
errors** (~SUSPECT the census is right about the tokens; my report is right about the
sites):

- **`getent`** census 58,64,**109** → mine 58,64 (missing 109). `getent` at line 109 is
  inside `$(getent passwd "$USER_NAME" | cut …)` — a command-SUBSTITUTION body. My
  analyzer correctly does NOT count it as a plan/apply leaf (`is_expansion_internal`);
  it is effect-bearing but not a wrappable leaf (the dn-3 leaf-seam). Granularity
  difference.
- **`sed`** census 76,98,173,242,249,289,327,**432**,433,**575**,**593** → mine has
  76,98,173,242,249,289,327,433 (missing 432,575,593). Two sub-causes:
  - **432**: `esc=$(printf … | sed …)` — cmdsub-internal, as `getent` above.
  - **575, 593**: inside `while read -r pattern line; do … ; continue ; done <<EOF`
    loops. These ⊤-REJECT for two reasons the engine reports loudly
    (`error[syntax-unsupported]`): `continue` in a loop body (un-modeled early exit) AND
    a `done < file` trailing redirect (un-modeled construct I/O). The whole `while`
    construct collapses to one ⊤ node, so its body leaves (the sed, grep, printf, the
    `[ -z ]` test, `read`, `continue`) all vanish from my site set. **This is the
    biggest single census↔site gap and it is a correct, loud under-modeling boundary**
    (the `while-read-file-rejects` e2e case pins exactly this rejection). 8 such loop
    bodies × several leaves each accounts for much of the token↔site arithmetic.

Net reconciliation (-GUESS, not exhaustively audited): my 195 sites EXCLUDE cmdsub-
internal commands and ⊤-rejected loop bodies, and INCLUDE plain assignments + `[` tests
as leaves (which the census buckets as constructs, not its 174 "command tokens"). The
two numbers measure different things; the spot-verify confirms they agree wherever both
see a top-level leaf.

## §5 Seam wishlist (what attribution cannot see from public surfaces)

The dashboard reads only public crate APIs (it may not edit other crates). Where that
costs precision, the seam I wanted:

- **seam-1 (the big one): a public per-site `CommandEffect` / ⊤-reason readout.** A
  `SkipClass::MustRun` conflates Opaque(⊤) with pure-builtin / kill / unreachable — the
  underlying `CommandEffect` is not on any public surface. So c1 cannot say WHY a
  `MustRun` isn't analyzable, and I report it `Indeterminate` (honestly — never a false
  `TOP` assertion for `set -e`/`echo`). A `classify` that also returned each site's
  `CommandEffect` (or a `⊤-reason` enum: opaque-word / no-check / multi-operand-refused /
  unmodeled-construct) would let c1 split "genuinely ⊤" from "pure builtin, not an
  elision candidate". This is the single highest-value seam. ~SUSPECT it is a 1-line
  return-type widening on `classify`.
- **seam-2: a public `site → Observable` re-key mirroring the cli's `facts_from_sites`
  firewall.** The cli's wrong-concrete firewall (establish-rc never folds; valid-Query-rc
  folds) lives in `crates/cli/src/main.rs` (private). I re-derive a coarsening in
  `observe_from_sites` (Effect verdict exact; valid-Query status reconstructed from the
  verdict as holds⇒0/absent⇒1). For the Effect-gated doors (replace-converged,
  dead-invariant) this is EXACT; for fold/query-substituted against a REAL probe-results
  file carrying raw rcs it could under-count vs the cli (I reconstruct the rc from the
  verdict rather than reading the wire rc). On the e2e fixtures the reconstruction
  matched the cli everywhere I checked, but a hostile probe-results with rc≠{0,1} on a
  valid Query would expose the gap. A public `cli::facts_from_sites`-equivalent would let
  the dashboard mirror the cli byte-for-byte. ~SUSPECT.
- **seam-3: in-loop-floor vs consumed-status precision.** An in-loop Members site that
  runs under `set -e` surfaces as `consumed-top-status` (the binding blocker — correct,
  since even all-converged it blocks on the consumption gate), but the *also-true*
  in-loop-floor reason is masked by precedence. Not wrong, just coarse. A public "why did
  the license refuse" enum on the plan step would sharpen the block-reason. -GUESS low
  value.

## §6 Adversarial hunt-list (attribution-lying risks)

The charter named three; I hunted each and add others.

- **hunt-1 (charter): a site counted elided whose artifact actually runs it.** Defence:
  the dashboard reads `build_plan`'s `Disposition` directly — the SAME structure
  `render_apply` renders — so "elided" means the plan elided it. Cross-checked the
  H2SaLS run-set against the cli (0 elisions both, §3) and the headline case per-door
  (exact match). RESIDUAL RISK (~SUSPECT): the render-capability REFUSAL (a heredoc-
  bearing `Replace` runs verbatim despite the disposition being `Replace`, `20V` §4 d-6).
  The dashboard counts that as `replace-converged` (elided) but the artifact RUNS it. I
  do NOT currently read `render_refusal_diagnostics`. On H2SaLS this never bites (0
  elisions), but on a converged book with a heredoc-carrying converged mutator the
  dashboard would over-count by 1 per such leaf. **FLAGGED as the top residual lie-risk**;
  the fix is to consult `plan.render_refusal_diagnostics` and demote a refused leaf to a
  `runs(render-refusal)` door. Deferred (no corpus case exercises it; would need a
  render-crate read I judged out-of-scope for the first cut).
  <!-- /* DONE 2026-06-11 (task #13 fix-1, commit 60e04f3; note 217 §2 m-6): build_report
  now consults Plan::render_refusal_diagnostics and demotes refused leaves to
  runs(render-refusal) (BlockReason::RenderRefusal), unit-pinned. This residual is
  CLOSED — do not re-implement. */ -->
- **hunt-2 (charter): double-counting under loops/Members.** Defence: a Members site is
  ONE `SkipClass::EstablishMembers` ⇒ ONE `SiteRow` ⇒ counted once (the members are
  sub-records, not sites). Verified the `loop-members-all-converged-elides` case reports
  exactly 1 site (site 0, `replace-converged`), not 2. The probe ships per-member checks
  (`site 0.0`, `site 0.1`) but those fold into the parent site for c3. No double-count.
  +SURE.
- **hunt-3 (charter): the `unattributed` bucket masking a systematic gap.** Defence: the
  bucket is reported as its own loud column and asserted 0 in tests on modeled books; it
  is 0 across all 75 e2e cases AND on H2SaLS. If a future engine variant lands, the
  `#[expect(unreachable_patterns)]` fires at compile time AND the bucket populates at
  runtime — double signal. The RISK is the inverse: a site mis-attributed to a WRONG real
  door (not unattributed). Mitigation: the per-door unit tests pin known dispositions, and
  the corpus-wide cross-check against the cli would catch a systematic mis-bucket. +SURE
  on the bucket; ~SUSPECT no wrong-door mis-bucket exists (the door logic is a thin,
  tested map over public enums).
- **hunt-4 (mine): the door-3 discriminator firing on the WRONG replace.** If a
  `replace-converged` site happened to ALSO consume `StatusInvariant` (some inner `|| true`
  on a converged ambient establish that ALSO has another reason to elide), I'd label it
  `dead-invariant` over `replace-converged`. Per the engine's mark-union (`20V` §4: "any
  OTHER blocking mark on the site wins"), a site carrying StatusInvariant AND a real
  elision is genuinely a door-3 shape (the `|| true` is what made the ⊤ status non-
  blocking), so labelling it `dead-invariant` is correct-by-construction. Verified
  `door3-or-true-elides` (the install is `EstablishAmbient` + StatusInvariant → I report
  `dead-invariant`, matching the case's stated door). +SURE this is right, but FLAGGED
  because it is the subtlest call in the crate.
- **hunt-5 (mine): the criticality weight double-counting overlapping spans.** A heredoc
  leaf's span and an enclosing construct don't both produce sites (only leaves do), so
  per-line weights aren't summed twice. But a multi-line `$()`-assigned leaf weighs its
  whole span (e.g. 42 for `block_body=$(cat <<EOF…)`), which is a generous stand-in. When
  the 1A matrix lands this is moot (real per-site scores). -GUESS low risk.
- **hunt-6 (mine): the convergence reconstruction (seam-2) fabricating a fold.** My
  `observe_from_sites` reconstructs a valid-Query rc from its verdict (holds⇒0/absent⇒1).
  If a real probe-results carried a valid-Query `rc=2` (a `cant-tell`-but-present guard),
  I'd reconstruct `Top` (Unknown verdict) and the fold would correctly NOT fire — safe
  direction (`kFAIL-perform`). The lie would only go the unsafe way if I fabricated a 0
  where the wire said non-zero; I never do (Unknown⇒Top). +SURE this errs safe.

## §7 Status + what I did NOT do

- Did NOT read `render_refusal_diagnostics` (hunt-1 residual) — first-cut scope; flagged.
- Did NOT wire the 1A weights (they don't exist yet) — the adapter seam is in place and
  tested (`from_line_scores`).
- Did NOT edit any other crate's source, the e2e tree, or the H2SaLS corpus (read-only).
  Only edits: `crates/coverage/**`, the `Cargo.toml` `members` line, `tools/coverage.sh`,
  this note.
- The `tools/coverage.sh` wrapper is NOT in `e2e/run.sh` and exits 0 even with no binary
  — it can never fail a build (charter: "runs in the gate set without becoming a gate").
