# 20H — task-O (the one-Observable completion): the channel rename, the tuple-in-shape, and the gate-5 disposition carve-out

> Round-20 spike note, append-only. Records task-O: the evidence-backed channel rename
> (item-1), the `19F` §3 tuple completed in SHAPE — `Observable` gains `stdout`/`stderr`
> (item-2), and the gate-5 `Omit`/`Replace` carve-out resolving strain-D3b-fold-vs-gate5
> (item-3), plus the 206 §3 supersession line (item-4). Behavior-preserving except where
> stated; ZERO golden churn. AI-authored, confidence-marked. Trust R/D/I/K + 19H/19I + the
> human rulings over this. Builds on 206 §3 (the rename rationale), 19F §3 (the tuple spec),
> 20G §5 (strain-D3b-fold-vs-gate5 + tc-gate5-omit), 20C (the record grammar), 20F (gate-5).

## §0 What landed (all green: cargo fmt/clippy -D warnings/test 64-lib + suite-wide 0-fail-1-ignore;
## `sh e2e/run.sh` 50/50 ZERO xfail; `typos spike` clean; ZERO golden bytes changed)

- **item-1 rename** (pure, zero behavior): `Channel::AndOrStatus` → `Channel::StatusRelaxable`,
  `Channel::Status` → `Channel::StatusRenderFloor`. Every reference/comment/test-name updated
  across `core`, `analysis` (`cfg.rs` + `tests/cfg.rs`), `plan` (`lib.rs` + `tests/observable_matrix.rs`),
  `spike/CLAUDE.md` (inv-one-observable), `plan/CLAUDE.md`. The 206 §3 axis-rationale is cited at
  the `core::Channel` enum.
- **item-2 tuple-in-shape**: `core::Observable` gains `stdout: Predicted<OutClaim>` +
  `stderr: Predicted<OutClaim>`, `OutClaim(pub Symbol)`. Defaulted `Predicted::Top` everywhere;
  `verdict_only` + all four struct-literal sites updated. The cli record grammar RESERVES
  `stdout=`/`stderr=` (parses-and-stores, emits nothing). Nothing produces a non-⊤ value.
- **item-3 gate-5 carve-out**: `--debug-argv` now tags `argv <leafid> <run|replace|omit> <words>`;
  `argv_echo_check` skips non-`run` sites. Demonstrated both directions (§3).
- **item-4**: one supersession line added to 206 §3 ("EXECUTED in task-O").

## §1 The rename mechanics — what resisted, what was clean (+SURE)

The rename is genuinely pure: the two-channel *distinction* (one `Powerset<Channel>` membership
test per variant in `consumption_ok` + three `mark`/`insert` sites in `cfg.rs`) is untouched; only
the spelling changed. `cargo test` flipped green with no logic edit — the +SURE proof it is
behavior-preserving.

What needed a judgment call, not just sed:
- **str-1 (the `Observable.status` field doc).** The field doc said "`Status` channel" — now
  AMBIGUOUS, because both `StatusRenderFloor` and `StatusRelaxable` consume *the status
  observable*. The status FIELD is the predicted *value*; the consuming-side CHANNEL decides
  usage. I reworded it to "the predicted exit status … the consuming side decides which status
  channel reads it." ~SUSPECT this is the right framing (it matches the prediction-vs-consumption
  split inv-one-observable already draws); a reviewer should confirm the field-doc isn't read as
  implying a single status channel still exists.
- **str-2 (the conceptual tuple in inv-one-observable stays `{Effect, Status, Stdout, Stderr}`).**
  The 19F §3 OBSERVABLE tuple is the PREDICTION side (the `Observable` struct fields). The rename
  is a CONSUMPTION-channel refinement (the `Channel` enum, the liveness set). I left the inv text's
  abstract tuple intact and ADDED a clause documenting the consumed-Status split by
  render-expressibility. +SURE the tuple and the consumption-enum are different things (19F §1's
  whole point); conflating them would re-fragment the very thing inv-one-observable guards.
- **str-3 (e2e fixture comments NOT renamed — golden-coupled).** Three e2e `book.sh` files and
  their `expected.out` carry the literal `AndOrStatus` in authored sh-comments
  (`exec-errexit-top-status-runs`, `exec-query-after-mutator-runs`, `exec-dollarq-blocks-elision`).
  The apply render echoes the book verbatim INCLUDING comments, so a `book.sh` comment edit forces
  an `expected.out` re-bless — a GOLDEN CHANGE, which the task forbids ("ZERO golden changes"). So
  I deliberately left those six fixture comments on the old name. The rename's source-tree scope
  (Rust + the two CLAUDE.md + the matrix doc) is complete; the fixture comments are a known stale-
  name residue, flagged not fixed (tc-fixture-comment-rename, §5). A future fixture re-bless can
  sweep them; doing it now would trip the zero-golden gate.

## §2 The OutClaim representation chosen (+SURE: cheapest deterministic `Copy`)

`OutClaim(pub Symbol)` — a newtype over the interned `core::Symbol`. The decisive constraint:
**`Observable` must stay `Copy`** (the cli's `record.map_or(…)`, `plan`'s `observed.map_or`, the
matrix `is_replaced`, and the fold all rely on `Observable: Copy`/`Clone` by value — `prove_replaceable`
even takes `consumed`/`status` by value). `Symbol` is a `Copy` `u32` newtype, so `Predicted<OutClaim>`
keeps `Observable: Copy` with zero ripple. Owned `String` was rejected: it is `!Copy`, which would
have forced `Observable` off `Copy` and rippled through ~8 by-value call sites — a behavior-shaped
churn the "shape only" mandate forbids.

Determinism (`inv-determinism`): the interner is order-of-interning, never hashed-into-output, so an
`OutClaim(Symbol)` is a deterministic handle. The engine NEVER decodes it (`inv-referent-agnostic`):
a substitution would compare/reproduce the claim; the analyzer does not branch on its text.

The cli threads `&mut Interner` into `parse_results` to mint the reserved-key claims (the `cli` is
the I/O edge, `inv-determinism`-exempt — no kernel determinism touched). ~SUSPECT this is the
right seam: the alternative (a separate owned-string lane bypassing the interner) would have been a
SECOND representation of captured text — exactly the representation-drift `19F` died to. One
interned vocabulary, one `OutClaim`.

**Anti-masking honored (item-2's load-bearing constraint).** NOTHING produces a non-⊤ `OutClaim`:
the rendered probe emits no `stdout=`/`stderr=` (PROBE_HEADER bytes unchanged — see §4), and
`consumption_ok` blocks a consumed `Stdout`/`Stderr` UNCONDITIONALLY without ever reading the
`Predicted<OutClaim>` value. So feeding a (hypothetical) claim into the tuple cannot relax that
block — the gate is value-blind on stdout/stderr by construction. The new unit test
`parse_results_reserves_stdout_stderr_keys_inert` pins BOTH halves: absent ⇒ `Top` (the live
default), present ⇒ stored as `Value(OutClaim)` while Effect/Status are unaffected. This is the
SHAPE assertion 19F mandated (a future stdout-producing probe is value-plumbing, not a
representation change), NOT a check predicting a value (which would be the masking 19I §2 strips).

## §3 The gate-5 carve-out — strain-D3b-fold-vs-gate5 resolved, both directions demonstrated

**Mechanism.** `--debug-argv` was `argv <leafid> <words>`; now `argv <leafid> <disposition> <words>`
where disposition ∈ {`run`, `replace`, `omit`} (`disposition_tag`). `argv_echo_check` pulls the 3rd
field and `continue`s unless it is `run`. The one-directional ⊆ assertion (engine-resolved-and-
shimmed ⊆ bare-logged) now reads engine-resolved-and-shimmed-**and-run** ⊆ bare-logged.

**Why this is the right cut (+SURE, traced).** A `replace`d or `omit`ted site is deliberately absent
from the APPLY run-set. For a guarded `omit` (the `dpkg -s X || install X` fold) the install is
absent from the BARE book too — a preceding guard short-circuits it — so asserting it ⊆ the bare log
is a FALSE failure. That false failure is exactly the structural exclusion 20G §5 traced: it confined
the fold/omit e2e demonstration to UN-shimmable BUILTIN guards (`command -v`, which fails in the
mocks-only bare book ⇒ the install runs there ⇒ the old ⊆ held by accident). The `run`-only filter
removes the exclusion without weakening the gate for sites that actually run.

**Direction 1 — the gate KEEPS its teeth (demonstrated, temp copy of `exec-diverged`).** That case
has a `run` site (`apt-get install -y nginx`). A wrapper rewriting the engine's readout to
`argv 0 run apt-get install -y apache` (the prefix-env wrong-concrete class 20F §1 demonstrated) ⇒
gate-5 FAILS loudly ("engine-resolved RUN argv not in bare log"), because the bare book logs
`apt-get install -y nginx`. So a wrong RESOLUTION on a run site is still caught. +SURE (run from
`/tmp`, repo untouched).

**Direction 2 — no longer structurally excludes shimmed-guard folds (demonstrated, temp).** A minimal
strain-D3b-fold-vs-gate5 shape — `dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx`, a
`pkgstate` query oracle on the EXTERNAL (shimmable) `dpkg`, a `dpkg` shim exiting 0 (nginx holds), a
`package` establish oracle, probe-results `site 0 effect=holds rc=0`. The engine folds: site 0
(`dpkg -s nginx` guard) ⇒ `replace` (line collapses to `true`), site 1 (`apt-get install`) ⇒ `omit`.
The bare book runs `dpkg -s nginx` (exit 0) ⇒ `||` short-circuits ⇒ the install is NOT logged. Result:
- OLD gate-5 (assert ALL dispositions ⊆ log): **FAIL** on `argv 1 omit apt-get install -y nginx` —
  the false failure 20G §5 names.
- NEW gate-5 (skip non-run): **PASS** — the `omit` site is skipped; the `replace`d guard is skipped
  too (correctly — a `replace`d site is substituted in the apply, not run there).
+SURE (run from `/tmp`, repo untouched). This means a future shimmed-Query-fold corpus case CAN now
demonstrate the omit under all gates green — the value-story 20G §5 found stranded at the harness
layer is un-stranded.

## §4 Zero-golden discipline (the item-2 stop-and-flag invariant; got zero)

The one real trap: documenting the reserved `stdout=`/`stderr=` keys "in the artifact header" (item-2)
collides with "ZERO golden changes" — the PROBE_HEADER string is EMITTED into stdout, captured in
every `expected.out`. Changing the emitted bytes would re-bless 50 goldens. Resolution: the
"production is future work" documentation lives in the Rust DOC-COMMENT on `PROBE_HEADER` (+ the
`parse_results` doc + the `SiteRecord` doc), NOT in the emitted string literal. The shipped artifact
bytes are unchanged. Verified: `git status` shows ZERO `expected.out`/`expected.ran`/`probe-results.txt`
touched; the e2e content golden-diff (which fails on any artifact byte delta) is green 50/50. +SURE.

## §5 The fold-oror-marker answer (item-3, REPORT ONLY — NOT done)

**`fold-oror-guard-omits` CANNOT drop its `PROBE_RESULTS=authored` marker.** +SURE, traced. The
marker's reason (read from the marker file + `tool.oracle.sh`) is a **gate-1** concern: the
load-bearing guard probes via `command -v` — a SHELL BUILTIN that `PATH=mocks-only` cannot shim, so
the mocked probe resolves the operand absent (rc 1) instead of the authored real-host `holds rc=0`,
breaking gate-1(b) PARITY. The marker is a *permanent* gate-1 opt-out (20G §1's "builtin" category),
NOT a gate-5 artifact. My carve-out touches gate-5 ONLY (the argv-echo differential). The two gates
are independent — gate-1 asserts the probe REPRODUCES the fixture; gate-5 asserts the engine's argv
matches the bare book. So the carve-out neither enables nor affects this marker. (And the case never
tripped the OLD gate-5 anyway: its builtin guard fails in the bare book ⇒ the install runs there ⇒
the old ⊆-assertion held.) The marker stays. Per the brief I did NOT touch it.

## §6 tc-* / judgment calls flagged (conservative defaults; flagged up, not settled)

- **tc-fixture-comment-rename** (§1 str-3): six e2e fixture comments (`book.sh` + `expected.out`)
  still carry `AndOrStatus`. Renaming them is a GOLDEN CHANGE (the render echoes book comments), so I
  left them under the zero-golden mandate. ~SUSPECT the right time to sweep them is the next deliberate
  corpus re-bless; flagged so a future grepper knows the stale name in fixtures is intentional residue,
  not a missed reference.
- **tc-outclaim-symbol** (§2): `OutClaim(Symbol)` over an owned-string newtype, chosen to keep
  `Observable: Copy`. +SURE for THIS round (nothing produces values, so the interner-handle costs
  nothing). ~SUSPECT a future real stdout-producer may want richer claim shapes (a hash, a prefix, a
  length-bound) rather than full interned text — but that is a value-plane decision (when a probe
  actually captures stdout), deferred. The newtype boundary is exactly where that future refinement
  lands without touching consumers.
- **tc-gate5-disposition-format** (§3): I extended the `--debug-argv` line format
  (`argv <id> <disp> <words>`) rather than adding a separate readout line. Conservative: it is the
  minimal change, keeps one line per site, and the disposition is intrinsic to the gate's decision.
  The format is a cli-edge debug readout (not a golden, not consumed by anything but `argv_echo_check`),
  so widening it is safe. Flagged in case a reviewer prefers a separate `disp <id> <word>` lane.

## §7 Exclusion-check (the four-by-two discipline, AGENTS.md)

- **other phase**: the rename is phase-agnostic (it renames a `Channel`, an `inv-superposition`
  un-collapsed fact the phased caller reads — both Probe and Apply callers see the same renamed
  variants). The tuple-in-shape adds `stdout`/`stderr` to the PREDICTION the check yields (probe-side)
  and the SUBSTITUTION reproduces (apply-side) symmetrically; both default ⊤. gate-5 is apply-disposition
  driven (the carve-out reads the apply plan's `Disposition`), but the probe render is byte-unchanged.
- **other user**: the renamed channels read clearer to BOTH the admin (a `StatusRenderFloor` block in a
  FAIL says "the render can't substitute this guard" — a remedy, vs the opaque `AndOrStatus`) and the
  engineer (the four-vs-one source split is now legible at the enum). The gate-5 carve-out's FAIL message
  is unchanged for run sites; an omit/replace site simply isn't asserted (no new confusing output).
- **other reliability**: the carve-out's `run`-only filter is the unreliable-input-safe direction — an
  `omit` minted from a KNOWN controller rc is the only omit gate-5 now skips; an unreliable/⊤ controller
  never folds (the branch stays live ⇒ the site is `run` ⇒ still asserted). So the carve-out cannot hide
  a wrong resolution on a live site.
- **reverse propagation**: N/A — the rename/tuple are forward-fact shapes; gate-5 consumes the engine's
  forward `Disposition` + value-flow output as data.
- **the killer cross-cell**: the §4 PROBE_HEADER trap IS an exclusion-check catch — "document in the
  header" (item-2) read naively breaks "zero golden" (the gate), because the header is emitted bytes.
  Caught before blessing; resolved by documenting in the doc-comment, not the literal.

## §8 Confidence summary

- +SURE: the rename is behavior-preserving (cargo test green with zero logic edit); 50/50 e2e zero-xfail;
  fmt/clippy -D warnings/typos clean; ZERO golden bytes changed (git-verified + e2e content-diff green).
- +SURE: `OutClaim(Symbol)` keeps `Observable: Copy`; nothing produces a non-⊤ value (probe header
  byte-unchanged, `consumption_ok` value-blind on stdout/stderr).
- +SURE: the gate-5 carve-out fails on a wrong-resolution run site (direction 1) and passes a
  shimmed-guard fold the OLD gate false-failed (direction 2) — both run from temp copies, repo untouched.
- +SURE: `fold-oror-guard-omits` cannot drop its marker (gate-1 builtin-unshimmability, orthogonal to
  the gate-5 carve-out). Reported only, not touched.
- ~SUSPECT: the six stale fixture comments (tc-fixture-comment-rename) should sweep at the next corpus
  re-bless; left now to honor zero-golden.
- ~SUSPECT: no seeded `#[expect]` became unfulfilled (clippy -D warnings green with the existing set), so
  the rename did NOT free any expect to delete — the brief's "may let you DELETE" did not materialize here.
