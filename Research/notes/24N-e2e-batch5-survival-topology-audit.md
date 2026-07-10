# 24N — e2e batch-5 (survival tier) per-topology assertion-depth audit (read-only)

AI-authored (Opus audit agent), 2026-07-09, round 24. Executes the READ-ONLY half of
LIVING_STATUS r24 queue item 1d, which 24I deferred as its migration ORDER batch 5 ("LAST +
most care — survival tier vs sweep; all ~SUSPECT until per-topology assertion-depth verified").
Changes NO product code. The deliverable is the migration-readiness table (§4) that unblocks the
eventual move of these e2e cases to cheaper tiers.

The question per case (24I batch-5): confirm the sweep's `TopologyClass` scenario asserts the
SAME end-state, so the e2e is either duplicating sweep coverage (delete/degraduate) or adding
assertion-depth the sweep structurally lacks (keep). No case is deleted here; this is the spec.

## 0. The two artifacts being compared

**The sweep** (`spike/crates/sweep/`, flavour-C DST): per `u64` seed mints a `[wall,(extra-wall)?,
victim]` book from ONE fixed package oracle (`ORACLE_SH`), runs the real kernel in-process
flag-ON and flag-OFF, evolves two `hostsim::Host` copies, and asserts (`tests/sweep.rs`):
- HONEST end-state equality (`s_bare == s_apply_on`) — a wrong elision is RED;
- flag-OFF never under-executes (`!off_diverged`, every seed) — the conservative baseline;
- attribution-under-lies (a LYING divergence is a victim survival whose `crossed_walls` names the
  recorded liar leaf; for `AliasWall`, also names `resolver = "package"`);
- determinism (bit-identical replay);
- sometimes-asserts: all NINE `TopologyClass`es + honest-elision/lying-divergence/derived-/alias-/
  reach-lying-divergence/reach-poison-attribution/flag-distinguishes counters are non-vacuous.

Structural facts it exposes per trial: `Disposition` (Run/Omit/Guard/Replace), `Survival{
elided_leaf, elided_label, crossed_walls:[{wall_leaf, provider, resolver}]}`, `reach_poisonings:
[(leaf, kind)]`, and end-state fact-label sets.

**A survival-tier e2e case** (`spike/e2e/cases/<name>/`, driven by `run.sh`) asserts a
SUPERSET of render/emission properties the sweep never touches (verified against `run.sh`):
- byte-golden `expected.out` (full probe+apply artifact text; content-diff, unless XFAIL);
- `expected.ran` run-set under inert mocks (gate-4/exec_check — ordered);
- gate-1 probe-parity (mock records == authored `probe-results.txt`) unless `PROBE_RESULTS=authored`;
- gate-5 argv-echo (engine argv ⊆ dash's bare-run argv) and gate-6 dual-rail license judge;
- gate-3 stderr floor; gate-7 `expected-why` substring emission (opt-in);
- `dash -n` runnability on both rendered artifacts.

## 1. The load-bearing structural constraint (the exclusion-check finding — 24N-fd1)

+SURE: the sweep is **package-KIND-ONLY** (`scenario.rs:173 const KIND = "package"`; every cell
mints through `cell()` with that kind) and **oracle-fixed** (one `ORACLE_SH`, verbs
install/config/purge/refresh/place). Three whole classes of survival-tier e2e case are therefore
**sweep-inexpressible by construction**, not merely un-generated:
- **cross-kind** disjointness/reach (package↔service, package↔fs) — no second kind exists in the
  generator;
- **opaque / un-oracled** commands (`hork wombat`) — the generator emits only oracled apt verbs,
  so a `⊤`/Opaque poison wall never arises;
- **declared-incoherent footprints** (a `touches()` that omits its own establish coord) — the
  sweep's dishonesty axis is GROUND-TRUTH `CellDelta` divergence (`Honesty::Lying`), and its
  declared footprints are always coherent; a declared at-least⊄at-most contradiction is a
  different failure mode it never mints.

Consequence for the migration: the cases resting on any of these are KEEP-e2e (or need a NEW
authored twin), NOT sweep-subsumed. Making the sweep subsume them is a GENERATOR EXTENSION
(multi-kind + opaque + incoherent-footprint axes), a much larger piece of work than an e2e move —
out of scope here, flagged for whoever revisits sweep breadth.

## 2. What an in-memory twin preserves vs what is render-incidental (question iii, general)

+SURE, uniform across the batch:
- **Preservable structurally** (a plan-tier / observable_matrix-style twin, or the sweep itself):
  the per-leaf `Disposition`, the survival witness (`crossed_walls` leaves/provider/resolver), the
  `reach_poisonings`. The `expected.ran` run-set is a PROJECTION of the disposition set (Run sites
  = ran lines), so it too is structurally preserved — it is NOT a render property.
- **Render-incidental** (a twin may drop): the probe/apply artifact BYTES, comment wording,
  shebang framing.
- **Neither — the "real-emit" half** (question iii's real subtlety): gate-1 probe-parity, gate-5
  argv-echo, gate-6 dual-rail, gate-7 why-lens STRING emission, and `dash -n` runnability. These
  are the property the 24I doctrine says "stays anchored by the STAY yardstick+tie-downs+derived-
  survive." A plan-tier twin CANNOT reproduce probe-parity/argv/dual-rail (no real shell); it MUST
  add the one-shot `dash -n` per rendered artifact (the 24I ap-2 flag — `observable_matrix.rs`
  today asserts render strings via `.contains()` with NO `dash -n`, the latent text-diff
  blindness). So a why-lens-bearing case degraduates in TWO pieces: the structural survival →
  sweep/plan twin, and the why-lens STRING → a cheap cli/analysis EMISSION twin (gate-7's job),
  which is what "TWIN-then-degraduate" means below.

+SURE: `observable_matrix.rs` today covers door-3/converged-fold/status-consumption cases only —
`grep` finds NO survival/alias/reach/resolver/disjoint/crossed_wall mentions in
`plan/tests/`. So every TWIN verdict below implies a twin that must be AUTHORED (the sweep already
covers the seeded-structural half; the plan-tier gap is the named-scenario + emission half).

## 3. Per-case topology mapping + discriminating power (questions i, ii, iv)

Cite by case dir under `spike/e2e/cases/`. "Sweep class" = the `TopologyClass` the scenario
instantiates; "asserts today" = the e2e's live assertions; "sweep gap" = what the e2e adds.

- **strawman24-survive-simple** — MissConverged (flag-ON). Asserts: ran={oldpkg only}, byte
  golden, `expected-why` (survives+elides · wall site 0 · apt-get touches {package:oldpkg} ·
  backing package:nginx disjoint), gate-1/5/6. Sweep gap: NONE structural — `MissConverged` honest
  is the golden-hill path (`honest_elisions>0`), survival witness + disjoint-elide asserted across
  seeds. Adds only why-lens EMISSION + render. +SURE structural-duplicate.

- **strawman24-survive-simple-unflagged** — flag-OFF mode-gate. Asserts: ran={both}, byte golden
  (must equal the Stage-1 baseline). Sweep gap: the sweep runs flag-OFF on EVERY seed
  (`off_diverged` never; `flag_distinguishes>0`), so the "flag off ⇒ both run" mode-gate is
  duplicated; the only residue is the BYTE-IDENTICAL-to-Stage-1 render LAW (the flag must not
  perturb baseline bytes) — a render_corpus concern, not a sweep one. ~SUSPECT.

- **strawman24-modeled-wall** — flag-OFF Stage-1 (fd10 pin). Asserts: ran={both} (a modeled
  running-but-partial wall totalises downstream converged). Sweep gap: NONE — this IS the sweep's
  flag-OFF rail (the plan_off that never under-executes). Historical fd10 (HEAD wrongly elided
  pre-fix). +SURE duplicate; ~SUSPECT keep a 1-line plan regression pin.

- **strawman24-opaque-wall** — Stage-1, `PROBE_RESULTS=authored`. site1 `hork wombat` un-oracled
  ⇒ Opaque ⇒ verbatim poison wall; site2 curl past it RUNS; site0 nginx pre-wall elides. Asserts:
  ran={hork, curl}, byte golden. Sweep gap: TOTAL — the generator never emits an Opaque command
  (24N-fd1). +SURE not-subsumed.

- **strawman24-survive-killwall** — KillWall (flag-ON). Asserts: ran={purge only}, byte golden,
  `expected-why` (survives+elides · wall site 0 purge · disjoint). Sweep gap: NONE structural —
  `KillWall` is a `want` class (asserted reached); kill-uniformity survival is the same walk.
  Adds why-lens + render. +SURE duplicate.

- **strawman24-survive-multiwall** — MultiWall + a downstream footprint-less SilentWall demote, 5
  sites, 2 outcomes. Asserts: ran={oldpkg,badpkg,purge-gonepkg,curl}, byte golden, `expected-why`
  (2-crossing witness). Sweep gap: the 2-crossing witness aggregation is `MultiWall` (asserted
  reached), BUT the COMBINED book-shape — survive-2-walls THEN a silent total-wall demoting a
  LATER converged site in one book — EXCEEDS the sweep's fixed `[wall,(extra)?,victim]` 3-command
  single-victim template. ~SUSPECT: the aggregation half is sweep-covered; the mixed multi-victim
  shape is not.

- **strawman24-nonsurvive-bare** — SilentWall (flag-ON). Asserts: ran={both} (footprint-less
  mutator demotes even flagged). Sweep gap: NONE — `SilentWall` is a `want` class asserting exactly
  "footprint-less ⇒ demote even flag-ON." No why-lens, no special wiring. +SURE duplicate.

- **strawman24-nonsurvive-hit** — HitConverged (flag-ON). site0 `touches()` over-claims
  package:oldpkg AND package:nginx; site1 nginx HITS ⇒ not disjoint ⇒ demote even flagged. Asserts:
  ran={both}. Sweep gap: NONE structural — `HitConverged` (asserted reached) is the same
  footprint-hits-victim-entity ⇒ demote kernel path; the sweep reaches the hit via a `config`
  different-selector spelling, the e2e via an over-claiming footprint — same demotion. ~SUSPECT
  (spelling differs, kernel identical).

- **strawman24-crosskind-residue** — cross-KIND MissConverged (flag-ON), honesty/disclosure pin.
  site1 `systemctl enable nginx` (service:nginx) survives site0's package:nginx wall because
  service ≠ package kind ⇒ disjoint. Asserts: ran={install only}, byte golden, `expected-why`
  (backing service:nginx disjoint). Sweep gap: TOTAL — package-kind-only sweep cannot express a
  service kind (24N-fd1); AND this is a deliberate residue-DISCLOSURE pin (adequacy-seed sibling;
  24I: never migrate the honesty pins). +SURE not-subsumed.

- **strawman24-incoherent-refused** — declared footprint-incoherence (flag-ON). site0 `touches()`
  emits package:wrongpkg but establishes package:oldpkg ⇒ coherence fails ⇒ `footprint-incoherent`
  warning + footprint REFUSED ⇒ total wall ⇒ nginx demotes. Asserts: ran={both} (+ the warning is
  the deliverable). Sweep gap: TOTAL — declared incoherence is not the sweep's ground-truth-lying
  axis (24N-fd1). ~SUSPECT: the demote is subsumable but the DIAGNOSTIC is the point.

- **strawman24-alias-provides** — AliasWall honest (flag-ON). `package.resolve()` canonicalizes
  nginx-full→nginx ⇒ canonical HIT ⇒ demote (both run). Asserts: ran={both}, byte golden, gate-1
  parity over `resolv …` records. Sweep gap: the closure demote IS `AliasWall` honest (asserted
  reached; lying half asserts resolver attribution); the residue is that the e2e drives
  `resolve()` through the ORACLE-SH + probe `resolv` record WIRING, whereas the sweep injects
  `Host::with_resolution` directly — the sh-oracle→resolve() LIFT is unpinned by the sweep.
  ~SUSPECT (structural duplicate; the lift-plumbing needs a twin unless pinned elsewhere).

- **strawman24-alias-symlink** — AliasWall on the `fs` kind (flag-ON), `fs.resolve()` ~realpath.
  Kernel-identical to alias-provides and to sweep `AliasWall`, different kind. Asserts: ran={both},
  byte golden. Sweep gap: the fs KIND (24N-fd1) — a per-kind generality demo the package-only sweep
  cannot express; largely a near-duplicate of alias-provides at the kernel. ~SUSPECT.

- **strawman24-reach-crossauthor** — ReachWall, cross-KIND package→fs (flag-ON,
  `PROBE_RESULTS=authored`). `package.reaches()` drags the fs victim /etc/nginx/nginx.conf ⇒ poison
  demote. Asserts: ran={both}, byte golden, `expected-why` (poisoned via package.reaches()). Sweep
  gap: the reach-poison-demote NAMING is `ReachWall` honest (`reach_poison_attributions>0`, names
  "package"), BUT the reach here CROSSES KINDS (package→fs) and rides the `reach` probe-record
  wiring — both sweep-inexpressible (24N-fd1). ~SUSPECT (attribution half sweep-covered; cross-kind
  half not).

- **strawman24-reach-static-service** — STATIC cross-KIND package→service reach (flag-ON,
  `PROBE_RESULTS=authored`). `package.reaches()` STATICALLY reaches service:nginx (NO probe `reach`
  record — resolved via the `reaches()` arm, not host-derived). Asserts: ran={both}, byte golden,
  `expected-why` (poisoned via package.reaches()). Sweep gap: the sweep's `ReachWall` is
  host-DERIVED and SAME-KIND; static + cross-kind reach is doubly sweep-inexpressible (24N-fd1).
  ~SUSPECT.

## 4. Migration-readiness table (the deliverable)

Verdicts: **SUBSUMED-by-sweep** = the sweep already asserts the discriminating structural claim
across seeds; delete the e2e (the sweep IS the twin). **TWIN-then-degraduate** = structural half is
sweep-covered but the case adds an EMISSION/render/shape property that must move to an authored
plan/cli twin first. **DEGRADE-to-diag** = the case's real deliverable is a diagnostic; move to an
analysis-tier diag test. **KEEP-e2e** = the discriminating power is sweep-inexpressible (24N-fd1)
and render/real-shell-bound.

| case (spike/e2e/cases/) | sweep class | verdict | one-line reason | cert |
|---|---|---|---|---|
| strawman24-survive-simple | MissConverged | TWIN-then-degraduate | structural survival = sweep golden-hill; why-lens string → cli emission twin; real-sh anchored by STAY yardstick | +SURE struct / ~SUSPECT twin-value |
| strawman24-survive-simple-unflagged | (flag-OFF) | TWIN-then-degraduate | mode-gate = sweep flag-OFF rail + flag_distinguishes; residue is the byte-identical-baseline render law → render_corpus twin | ~SUSPECT |
| strawman24-modeled-wall | (flag-OFF) | SUBSUMED-by-sweep | modeled partial wall totalises = sweep off-never-under-executes; keep a 1-line fd10 plan pin | +SURE / ~SUSPECT pin |
| strawman24-opaque-wall | (none) | KEEP-e2e | fully un-oracled Opaque poison wall — sweep emits only oracled verbs (24N-fd1) | +SURE |
| strawman24-survive-killwall | KillWall | TWIN-then-degraduate | kill-uniformity survival = sweep KillWall; why-lens → cli twin | +SURE struct |
| strawman24-survive-multiwall | MultiWall | TWIN-then-degraduate | 2-crossing witness = sweep MultiWall, but the 5-site survive+silent-demote shape exceeds the sweep's 3-cmd template — twin must preserve it | ~SUSPECT |
| strawman24-nonsurvive-bare | SilentWall | SUBSUMED-by-sweep | footprint-less mutator demotes even flagged = sweep SilentWall exactly | +SURE |
| strawman24-nonsurvive-hit | HitConverged | SUBSUMED-by-sweep | footprint hits victim entity ⇒ demote even flagged = sweep HitConverged (over-claim vs config-selector, same kernel) | ~SUSPECT |
| strawman24-crosskind-residue | (none) | KEEP-e2e | cross-KIND disjoint-survival — package-only sweep can't express; disclosure/honesty pin (adequacy sibling) | +SURE |
| strawman24-incoherent-refused | (none) | DEGRADE-to-diag | declared footprint-incoherence + `footprint-incoherent` warning — not the sweep's ground-truth-lying axis; the diag is the point | ~SUSPECT |
| strawman24-alias-provides | AliasWall | TWIN-then-degraduate | closure-demote = sweep AliasWall honest; the oracle-sh→resolve() `resolv`-record lift the sweep bypasses needs a twin | ~SUSPECT |
| strawman24-alias-symlink | AliasWall | TWIN-then-degraduate | fs-kind resolve() generality — kernel-dup of alias-provides + sweep AliasWall; needs a kind-parameterized plan twin | ~SUSPECT |
| strawman24-reach-crossauthor | ReachWall | KEEP-e2e | reach-poison NAMING = sweep ReachWall, but cross-KIND (package→fs) + reach-record wiring are sweep-inexpressible | ~SUSPECT |
| strawman24-reach-static-service | ReachWall | KEEP-e2e | STATIC cross-KIND (package→service) reach via reaches() arm — sweep's ReachWall is dynamic + same-kind | ~SUSPECT |

Split: SUBSUMED 3 (modeled-wall, nonsurvive-bare, nonsurvive-hit) · TWIN 6 (survive-simple,
survive-simple-unflagged, survive-killwall, survive-multiwall, alias-provides, alias-symlink) ·
DEGRADE-to-diag 1 (incoherent-refused) · KEEP-e2e 4 (opaque-wall, crosskind-residue,
reach-crossauthor, reach-static-service).

## 5. Handoff notes for the migrator

- The 6 TWINs all need AUTHORED plan-tier twins — `observable_matrix.rs` has no survival/alias/
  reach coverage today. The sweep is the twin for the STRUCTURAL survival half; the plan/cli twin
  is for the named-scenario shape + why-lens EMISSION. Add the one-shot `dash -n` per rendered
  artifact (the 24I ap-2 flag) to any render-asserting twin.
- The 4 KEEP-e2e cases share ONE root cause (24N-fd1): they exercise cross-kind / opaque /
  incoherent dimensions the package-only, oracle-fixed sweep cannot generate. They stay e2e until
  (and only if) someone extends the sweep GENERATOR with those axes — a separate, larger task.
- crosskind-residue is additionally a deliberate honesty/disclosure pin (24I: the adequacy pins
  are never migrated — their POINT is documenting what the differential canNOT see).
- The 3 SUBSUMED deletions each rest on a `want`-class sometimes-assert already firing in
  `tests/sweep.rs` (SilentWall, HitConverged) or the flag-OFF invariant (modeled-wall) — the
  migrator should eyeball that the sweep counter is non-vacuous before deleting.
