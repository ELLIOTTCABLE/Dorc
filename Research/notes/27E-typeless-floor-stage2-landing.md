# 27E — block-rebuild stage 2 (typeless-floor) landing + residue

AI-authored (Opus builder, r27 stage-2 session). Records what landed for `270:block-rebuild`
stage 2 (the typeless floor + the two stage-1 handoffs B/C), the kernel seam's final shape, and
the deferred part-D cut. Companion to the conductor's `27D` ledger (do not edit 27D from here —
the conductor folds this in). Authority: root docs + `spike/CLAUDE.md` rulings + `271`/`24L`
outrank this.

## What landed (all green: 645 unit + 128 e2e, clippy `-D warnings` clean, 4 gates)

- **A — the typeless floor (`24L` §2–§7).** A markless verdict-only oracle (`foobar__is_converged()`
  in a plain file) now licenses guard+elide at its own tool's converged sites. Mechanism: an
  **auto-cell** (`dorc_core::auto_fact`) — a private per-provider singleton establish-cell keyed at
  the reserved unnameable kind `dorc-auto:<provider>` (`fence-unnameable`: an authored kind is
  DNS-labels-before-a-colon, so the embedded `:` can never be authored), entity `Singleton`
  (`fence-no-entity` — no operand promoted to a referent, §3), selector `converged`.
  - **The kernel seam (report item 5 — load-bearing for the next stages).** `classify` stays
    VERDICT-UNAWARE (`inv-determinism`); the cli edge lifts the set of providers bearing an
    `is_converged` (`dorc_oracle::verdict::verdict_providers`) and threads a
    `&BTreeSet<ProviderId>` INTO `classify`/`classify_with_why_diags`/`command_effect` as an
    explicit new PARAMETER (not a smuggled lift). `command_effect`'s two concrete-argv Opaque
    points (`no-check-resolved`, `empty-effect-map`) consult it: verdict-bearing ⇒ mint the
    auto-cell (`auto_or_opaque`), else Opaque. The set is keyed the same way `command_effect`
    keys a book word (forward-munge). member/inline-body sites are DENIED auto-cells (empty set)
    — the in-loop floor runs them anyway.
  - **Vouch plumbing rides unchanged** — `build_vouches` keys off the establish-bearing class +
    the provider's verdict funcdef; the auto-cell classifies `EstablishAmbient`, so no new license
    type. The elide-weld/guard mint consume the existing `ByVouch<VerdictVouch>`.
  - **fence-no-disjoint** — the survival tier reads an auto-coordinate as may-touch, never a
    distinct canonical (`survival::disjoint` short-circuits to `MayAlias` when either side's kind
    is registered auto). The plan is interner-free, so the cli deposits the auto-kind set into
    `Resolutions` (already threaded down the survival chain). Closes the §4 near-miss
    (distinctness-as-license). Unit-pinned (`fence_no_disjoint_auto_backing_never_survives` shows
    both the naive survival AND the fenced demote).
  - **probe emission (the fourth touch-point)** — the shipped probe for an auto-cell IS the
    stripped VERDICT body (`<provider>__is_converged <argv>`, `24L` §2); its rc maps to the Effect
    verdict through the record scaffold's existing rc-partition (0=holds/1=absent/else=cant-tell).
    Wired via a `ship_auto` closure into `compile_probe` + a `verdict: bool` on `ProbePredict`
    steering `render_sh`'s funcname. **GATED on the vouch** (`is_vouched(node)`): a DECLINED
    verdict (a refuse path — `return 2`, the R2-MULTIOP arity gate) has nothing to measure and
    ships no probe — this preserved `guard23-refusepath-rc0-never-passes` at zero golden churn
    (the fix for the one regression the first e2e caught).
  - Riders: **quoted-`"$@"`** was already licensing at HEAD (the lexer routes `$@` to
    `Word::Literal("$@")`, which resolves; the DESIGN founding one-liner vouches and ships).
    **resid-return-arity** — `run_return` now declines `return 0 junk` (`words.len()>2`) + test.
  - e2e exemplars: `typeless-floor-converged-elides` (markerless oracle: ambient elide + past-wall
    guard) and `typeless-floor-oneliner` (the DESIGN `mycmd --dry-run "$@"` founding example).

- **B — marker-gating enforcement (`marker-gates-syntax-only`).** A dialect construct (bind/mark)
  in an UNMARKED oracle/book is a loud `error[missing-dialect-marker]`; bare `__role` floor bodies
  lift markerless (that IS the floor). Built as a DEDICATED cli-edge pass (`oracle::marker`), NOT
  inside `lift_predicts` — see the tc-flag below.

- **C — totalistic forward-munge (`24C:rul24-totalistic-munge`).** Command-keyed roles now key by
  the funcdef SEGMENT via the forward munge (`to_funcname_segment`), not the backward `_`→`-`
  un-munge (deleted). The parser stores the RAW funcdef base for BOTH species (the collision lint
  needs the source distinction); `map_provider_name` now delegates to `to_funcname_segment`. A
  literal-`_` book word (`my_tool`) finds `my_tool__is_converged`; a dotted `my.tool` finds
  `my_tool__role`; the `munge-name-collision` refusal is the disambiguator. Regression pinned.

## tc-* judgment calls flagged UP (never settled locally)

- **tc-marker-gate-home** — I placed the marker gate as a dedicated cli-edge pass, NOT inside
  `lift_predicts`. The brief implied a lift-level gate + stamping ~100 unit tests. Rationale for
  the deviation: keeps `lift_predicts` a pure syntax mechanic (the ~100 lift-mechanic tests carry
  unmarked dialect deliberately and keep lifting), while the gate still fires at the user boundary
  (every oracle/book the cli/coverage load). NOT weaker for users; avoids churning 100 tests.
  **Conductor: confirm this placement or direct the lift-level+stamp approach.**
- **tc-set-e-blocks-auto-elide** — under `set -e`, an ambient auto-cell's book-command rc is
  consumed by errexit and is ⊤ (the firewall: the verdict measured `foobar status`'s rc, never
  `foobar sync-certs`'s), so the ELIDE is status-blocked and only the GUARD tier is reachable. A
  MARKED verdict declares a converged-rc and elides under `set -e`; the markless floor cannot. The
  exemplar drops `set -e` to show elide+guard. Safe (over-conservative), but the floor's elide
  tier is largely `set -e`-unavailable for bare commands — a real, correct limitation to surface.
- **state_stored_only_in interaction** — verified inert: `command_effect`'s auto-cell only mints
  at the two Opaque points for a BOOK command word; the `state_stored_only_in` colon-line marks
  live on a KIND funcdef and never reach `command_effect`. No pin added (no live-Establish path
  materialized); re-audit if the entity-algebra-rebuild gives those marks live semantics.
- **invited-rooms compile-pin (`279f` §5)** — NO hint-lane vs license-lane type split materialized:
  the auto-cell rides the existing `ByVouch<VerdictVouch>` machinery unchanged. Obligation passed
  forward (no compile-failure test landed this stage).

## Deferred / punted (with reasons)

- **effect-check rider (`271:rul-effect-check-home-typeless-floor`)** — PUNTED (punt-empowered,
  zero-guilt per the brief's typed sizing rider). It is a genuinely new static falsification pass
  over verdict-function bodies, NOT nearly-free at this seam. The seam it would attach to (the
  verdict-provider thread into classify) is now built, so it is cheap to slot later.
- **A6 (retire the pipe-guard otelcol improvised auto-cell) + D (the `24J` raw-ship repair,
  `271:rul-only-oracle-bytes-ship`)** — DEFERRED as a clean, unstarted commit series (nothing
  half-landed). A6 and D are COUPLED: retiring otelcol's improvised `v : io.opentelemetry.Collector`
  predict removes the `:?`-observe read-only vouch the CONNECTED-probe recognition relies on, so it
  cannot be done without D's reshape (compose `otelcol__predict '--version' | grep__predict '-q'
  '0.155.0'` instead of raw-shipping book bytes). D's build surface: `connected_check_pipes` +
  `compile_probe`'s connected path + `render_sh`, plus the three ratified build riders (per-channel
  coverage · stream-fidelity · capture-ships-real-bytes). Given the reshape's real ballooning risk
  (the brief's own "stop at a clean boundary" clause) and budget, I stopped BEFORE starting it
  rather than half-land. The landed `24J` connected shape is UNTOUCHED (still standing-law debt,
  as it was at my base).

## Predicted golden-delta classes (blessed)

Only the two NEW e2e cases' goldens were minted (128 = 126 + 2). Zero existing-golden churn: the
forward-munge display fix (`ship_touches_body` shows the book word `apt-get`, not `apt_get`) and
the vouch-gated auto probe both held the corpus byte-identical. The `strawman24-pipe-guard-floor`
`RAN_ORDER=lax` reflap (tar/curl reorder, identical multiset) was reverted, not committed.
