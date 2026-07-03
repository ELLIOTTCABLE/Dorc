# 23H — the atomic re-spelling session (R2/R3/R4): what landed, the differential, the recipe

AI-authored, 2026-07-02. Append-only (append-only-spike-notes). **Never-vouch applies:** everything
here is process-evidence, not proof — the gate/harness tallies are machine-run, but "the reconciliation
is correct" is a human-battle-testing claim I do not make. Confidence marks throughout
(+SURE / ~SUSPECT / -GUESS / --WONDER).

Task frame: complete R2 (retire the marker spellings; derive the engine structures from the inline
dialect), R3 (the probe emitter ships `strip_predict(<provider>.check)` per-site with the site's argv),
R4 (convert all live fixtures + big-bang golden re-bless) — as ONE atomic workspace change, under the
three baked-in rulings (jc-dpkg-i, jc-fblessed, jc-polarity-vs-rc FINAL). Design of record: `23E`.

## §0 Outcome in one line (honest scope)

**R2's derivation CORE landed+proven, and R4a (the fixture conversion) landed for 146/151 oracle
fixtures — all golden-stable, committed.** The linchpin (the derivation + the polarity-free `ValueClaim`)
was built ADDITIVE with the 23E-mandated differential proof; a required R1 parser-gap fix landed with it;
then the inline dialect was rolled across every derivable corpus shape additively (case-$verb arms +
marks, markers retained ⇒ always-green). **STOPPED at `jc-singleton-mark` (UNRULED, §4):** the 5
`pkgindex` oracles are true nullary Singletons (`apt-get update`, no operand) whose effect-mark selector
cannot be spelled in the `kind:entity.prop` grammar (no entity), and 23E's recipe does not cover it —
so those 5 fixtures are NOT converted, and the WIRING (P4, analysis→derivation) cannot land without
either a Singleton-mark ruling or a marker-fallback (which contradicts "retire entirely"). The remaining
bulk — the wiring swap, the R3 probe-emitter reshape, and the ~120-golden big-bang re-bless — is
DESIGNED-AND-RECIPE'd here but NOT landed (blocked on jc-singleton-mark for the wiring; the re-bless is a
large, correctness-critical, attention-intensive step I did not rush). Tree is GREEN at every checkpoint
(e2e 123/9/0/0); nothing is half-wired or broken. This note makes the remainder mechanical.

Commits this session (all on `ai/spike3-r23`, gates green each): `fe50437` (R2 derivation core + lexer
fix), `d59efa3` (this note), `e743d7c` (R4a package shape ×61), `9a3faef` (R4a package-family ×47),
`0785fa6` (R4a non-package shapes ×38).

## §1 LANDED — R2 derivation core (commit `fe50437 (AI new ana) R2: derive effect-map …`)

`spike/crates/oracle/src/predict/derive.rs` (new) walks the inline-dialect predict body and reproduces the
effect-map the retired `oracle_effect`/`oracle_kind` markers used to lift (23E §3). +SURE, tested:

- **The derivation** (`derive_predict(&Check) -> (Vec<DerivedEffect>, Vec<DerivedVouch>)`): a structural,
  source-order walk accumulating (a) the current annotation-kind and (b) the current verb (bound on a
  `case $verb` arm — recognized by scrutinee-symbol == the check's verb-binding sym, never by decoding
  text, `inv-referent-agnostic`). A command with a trailing ESTABLISH/OBSERVE mark emits one
  `DerivedEffect { verb, kind, selector, claim }`; a bare CONVERGED-VOUCH mark emits a `DerivedVouch`.
  Context is passed BY VALUE into every recursion, so a per-arm annotation/verb never leaks to a sibling
  path (shared-before-the-case AND per-arm annotations both fall out of source order for free).
  - kind ← the inline `pkg : package = "$1"` annotation reached on the path.
  - provider ← the check funcname (already lifted by R1).
  - verb ← the `case $verb` arm literal pattern; a `*` catch-all keys NO verb (it is entity-resolution /
    fall-through — pinned by `wildcard_arm_keys_no_verb`). A verbless check → the ε-verb (`verb: None`).
  - selector ← the trailing mark's `.prop`.

- **The polarity-free representation** (`ValueClaim`, jc-polarity-vs-rc FINAL — §3 below): replaces
  `Polarity{Establish, Kill, Query}` with `{Establish, EstablishInverted, Observe}` — NO create/destroy
  axis. `Establish` = rc-direct write-claim; `EstablishInverted` = the `!` mark (rc-inversion plumbing,
  the FORMER Kill, carrying no "kill" concept); `Observe` = the `:?` read-only guard-class (former
  Query).

- **The differential proof** (the 23E §3 safety net, `predict::derive::tests`): 5 tests assert
  `derive(inline-dialect) == lift(markers).effects` — package (apt-get install/reinstall establish,
  purge/remove inverted), service (systemctl enable/start establish, disable inverted), tool
  (verbless `command -v` Observe), the converged-vouch mark, and the wildcard-arm-keys-no-verb pin. The
  claim maps to the old `Polarity` label ONLY for the comparison (the end-state carries no polarity).
  oracle lib tests: 42 → 47.

**Nothing is wired.** The old `lift`/`Polarity`/`KindIndex` are untouched; `derive_predict` is dead code
outside its tests. This is deliberate: it proves the re-spelling reproduces the old effect-map BEFORE
any marker is deleted (the differential discipline), so the wiring swap can later be mechanical.

## §2 LANDED — R1 parser-gap fix (same commit) — REQUIRED by R4's fixture shapes

+SURE, found by the differential tests failing on exactly the redirect-bearing shapes. R1's dialect
lexer (`check/lexer.rs::redirect`) mis-lexed an **fd-dup redirect followed by a mark**: for the R2
fixture idiom `dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed`, after consuming the
`2>&1` fd-dup the lexer's "optional space-separated target" (POSIX `> /dev/null`) greedily swallowed the
following `:` marker token (`:` is a word-byte), so the redirect became `2>&1 :` and the mark was LOST
(`mark: None`, the target became a trailing literal word). R1 never tested a mark after a redirect, so
the gap was latent. Fix (surgical): a space-separated redirect target is never a dialect marker — a
leading `:` disqualifies it. `2>&1 : k:e.p` now lexes as `2>&1` + the `:` marker. Whole baseline stayed
green (123/9/0/0); the `> /dev/null` form is unaffected (its target does not start with `:`).

## §3 The polarity dissolution — jc-polarity-vs-rc (FINAL, human 2026-07-02) — RECORDED

Verbatim-in-spirit transitional-freeze note, recorded here per the ruling's requirement:

> **TRANSITIONAL freeze: no polarity doctrine here — dissolves into the uniform no-vouch-no-elide
> license when the guard/vouch tier lands.**

The ruling: property values are OPAQUE booleans; the engine knows no creation/destruction; NO polarity
class survives the lifted representation; the `!` mark is plain rc-inversion plumbing on a value-claim;
verb asymmetries are the oracle-author's domain (which arms they vouch), the engine has one default
(no vouch ⟹ run). INTERIM/required: a site whose reached arm carries an rc-INVERTED claim classifies
`MustRun` — a behaviour-FREEZE so HEAD's pre-vouch-law elision machinery cannot begin eliding a
formerly-kill-classified site as a side effect of this re-spelling.

**How this maps to HEAD behaviour (the zero-behaviour-change anchor):** at HEAD `Polarity::Kill` →
`CommandEffect::Kills(fact)`, which (a) GENS the fact into `Reach` (poisons downstream ambient-ness,
e.g. `kill-then-install`, `exec-same-cell-kill`) and (b) classifies `MustRun` (`classify_site`'s
`_ => MustRun` arm — a `Kills` never becomes `EstablishAmbient`). So the FREEZE is exactly HEAD's Kill
behaviour: an `EstablishInverted` claim must gen-into-Reach AND classify `MustRun`. The wiring step
(§5, P4) must preserve both, and mark the classifier line with the freeze code-note (ru-26
churn-disclosure). +SURE this reproduces HEAD for every corpus oracle (the marker `kill` rows all had
`MustRun` sites); the differential test's `EstablishInverted↔kill` label mapping is the evidence.

`Observe` (former Query) is NOT a polarity (read-vs-write is orthogonal to create/destroy), so it
survives as a distinct claim → `CommandEffect::Queries` (gens nothing, `QueryResolvable`).

## §4 jc-/hazard register (this session)

- **jc-polarity-vs-rc** (§3) — human FINAL; implemented as `ValueClaim` (no create/destroy) +
  the `EstablishInverted → MustRun+gen` freeze (freeze itself lands in P4 wiring).
- **jc-lexer-redirect-mark** (§2, FIXED) — R1 lexer gap: a mark after an fd-dup redirect. Fixed
  surgically; flagged because it edits R1's landed code (necessary for R4's shapes).
- **jc-fixture-funcname** (~SUSPECT, for P3): fixtures should keep the legacy `apt_get__predict` form,
  NOT the period `apt-get.check` semaphore. Rationale: (i) dash-valid (period+hyphen names are
  dash-invalid) though run.sh only `dash -n`s the RENDERED ARTIFACT (not oracle files, verified
  e2e/run.sh:213 vs :918), so this is belt-and-braces; (ii) the OLD lift (`lib.rs::lift_one`) suppresses
  parse-diags only for `…__predict`-suffixed funcdefs — a period-named funcdef would surface spurious
  book-parse diagnostics from the old lift while markers coexist (the additive intermediate, P3). The
  strip is idempotent on the legacy form, and `derive_predict`/R3 both key off `check.provider` regardless
  of spelling, so behaviour is identical. The period form is an optional lint-semaphore (23D §1); the
  spike does not need it. Flag, don't settle.
- **jc-dpkg-i** (RULED — author `dpkg.predict()`): the unit fixture `fixtures/package.oracle.sh` (and its
  pinned test `lifts_the_package_fixture_cleanly`, oracle lib.rs) declares `oracle_effect dpkg -i
  establish installed` with NO `dpkg__predict`. Under R2 (effects derive from predict bodies) there is
  nothing to derive it from. Ruling: author a minimal `dpkg__predict` (e.g. `case $1 in -i) shift ;;
  esac; pkg : package = "$1"; dpkg-query -W "$pkg" … : package:"$pkg".installed`). Small, honest;
  preserves the pin's intent (dpkg -i establishes `package:X#installed`). NB no e2e book uses `dpkg -i`
  (only the unit test pins it), so this is a unit-fixture-only edit.
- **jc-singleton-mark** (UNRULED — STOP, blocks the wiring): a nullary/Singleton verb (`apt-get update`
  ⇒ `pkgindex#fresh`, no operand) cannot spell its effect-mark selector. The mark grammar is
  `kind:entity.prop`; `split_mark_target` needs an entity fragment between the `:` and the `.` to parse a
  `.prop` (a `pkgindex:.fresh` yields prop=None; a `pkgindex.fresh` reads the whole as a reverse-DNS
  kind). A Singleton has no entity, so there is no place to hang the selector. 23E §5 gives no Singleton
  recipe. Candidate spellings, each a design choice I did NOT make (task: flag, don't invent): (a) a
  placeholder entity token `: pkgindex:index.fresh` (derive reads selector=fresh; the entity fragment is
  opaque+unused — but it invents a meaningless token into the fixture); (b) a new `#selector` mark form
  `: pkgindex#fresh`; (c) extend the two-part `kind.prop` reading for a kind proven single-segment. The
  5 affected fixtures (`exec-poison-wall-dead`, `exec-singleton-update`, `headline-{guarded-realistic,
  partial,pi-webhost}` — each a `pkgindex.oracle.sh`) stay marker-only. **This blocks P4** (wiring
  analysis to the derivation): with these 5 un-derivable, the derived effect-map would drop the
  `apt-get update ⇒ pkgindex#fresh` cells ⇒ the poison-wall cases would change behaviour. Needs a human
  ruling before the wiring can proceed.
- **jc-fblessed** (RULED — accepted): the multi-selector F-BLESSED structural floor
  (`KindIndex::resolve_probe` → `None` for a multi-selector kind with only a kind-default probe)
  EVAPORATES under check-as-oracle — annotation-on-the-reached-arm IS the per-selector declaration; do
  NOT attempt to preserve the old per-`(kind,selector)` filing floor. For the corpus the per-verb arms
  carry the correct selectors, so behaviour is preserved; the floor's protection-for-malformed-oracles
  is consciously dropped (aligned with 23D §1.2, "constrained in what we ASSUME, not what they
  contain").

## §5 The recipe for the remainder (mechanical, per-phase — for the next session or resumption)

Discipline throughout: additive-first, commit at every green checkpoint, the exec gates (`sh
e2e/run.sh` apply/probe-under-mocks) are the load-bearing behaviour anchor (they catch a wrong-elision
even before goldens re-bless — golden text is NOT the safety net, the exec differential is). BLESS is
exclusive + case-by-case-inspected; lens-verify each of the 9 xfails still fails for its DESIGNED reason
(23A §1 / 23G §1 reasons).

### P3 — fixture conversion — LANDED for 146/151 (R4a, commits e743d7c/9a3faef/0785fa6)
DONE additively (case-$verb arms + marks, markers retained ⇒ always-green; e2e 123/9/0/0 after every
batch): all 108 package-family + 17 pkgstate + 10 service (marker-aware) + 3 firewall + 4 tool + 2
confline + 1 user + 1 yum. Empirically golden-stable (the old lift still reads markers; evaluate's
entity resolution is unchanged by the added marks; the shipped probe is still `oracle_probe_*` pre-R3).
The MARKER-AWARE subtlety (found in `two-oracles`): mark an arm ONLY if its verb is in that file's
markers — its predict body had a `start` arm with no `start` marker; marking it would make derive gain a
cell the old lift lacks (differential break). NOT DONE: the 5 pkgindex Singletons (jc-singleton-mark
STOP). Per-shape recipes as applied (23E §5), keeping the legacy `…__predict` funcname (jc-fixture-funcname):
- **package (61×, apt-get):** keep `pkg : package = "$1"`; inside the `[ "$2" = "" ]` guard add
  `case $verb in install|reinstall) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
  purge|remove) … : package:"$pkg".installed! ;; esac`. (Split install/purge into distinct arms — the
  `*`-arm CORPUS shape cannot key the effect distinction.)
- **service (systemctl):** per-verb arms carry the selector: `enable) systemctl is-enabled -- "$svc"
  : service:"$svc".enabled ;; start) systemctl is-active -- "$svc" : service:"$svc".active ;;
  disable) systemctl is-enabled -- "$svc" : service:"$svc".enabled! ;;`.
- **tool (command -v):** verbless OBSERVE `command -v -- "$tool" >/dev/null :? tool:"$tool".present`.
- **user (useradd):** verbless ESTABLISH `getent passwd "$user" : user:"$user".present`.
- **seam (apt + yum), pkgindex, pkgstate, firewall, confline:** each provider's own `.predict()` with its
  own probe + selector arm, same kind (the cross-oracle anchor).
- **guard23-vouch (flagship + pair-b):** replace `oracle_vouch_converged='apt-get install'` with a bare
  `: apt-get:install~` on the install arm's path, under the `# STRAWMAN … dq-kOOB` comment block.
- **CORPUS_PREDICT_SRC** (analysis `effect.rs` tests) + oracle `tests/predict.rs` fixtures convert too.
- **coverage inline fixtures** (`lib.rs` ~1134, `main.rs` ~527) convert with the rest.
CORPUS-WIDE DIFFERENTIAL (do this): a test iterating every e2e oracle fixture, asserting
`derive(predict-body) == lift(markers).effects`. Golden-stability check: adding case-arms must not change
evaluate's (verb,entity) resolution — every book verb needs an arm, else evaluate → `NoProbeReached` →
Opaque ⇒ a site flips resolvable→Opaque ⇒ golden churns. The exec gates catch this; verify e2e green.

### P4 — wire analysis to the derivation + the polarity freeze
- Build the interned index from `derive_predict` (a `KindIndex`-equivalent keyed by (provider, verb)); the
  vouch datum carried to wherever `oracle_vouch_converged` went. Swap `analysis::effect` to consume it.
- `cell_effect`: `ValueClaim::Establish → Establishes`; `Observe → Queries`; `EstablishInverted →` the
  freeze (gen-into-Reach + `MustRun`; a neutral `CommandEffect` that poisons + never
  `EstablishAmbient`). MARK the classifier line with the §3 freeze code-note.
- Retire `Polarity` from the lifted representation (blast radius: `oracle`, `analysis::effect`, `plan`,
  `hostsim`, `coverage`, `core::diag` — grep `Polarity`). Verify e2e exec gates stay 123/9/0/0.

### P4b — R3 probe-emitter reshape
- `compile_probe`'s closure `Fn(KindId, SelectorId) -> Option<String>` becomes a source of the
  **stripped predict body per provider** (`strip_predict(oracle_src, check)`); the invocation becomes
  `<provider>__predict <site-argv…>` (needs the site's resolved argv threaded in — `compile_probe`
  currently lacks it; thread the ValueFlow or a per-site argv map). `ProbePredict`/`ProbePlan::render_sh`/
  `render::probe::*` reshape: dedup the wrapper per PROVIDER (one `apt_get__predict`, invoked per verb-site
  with full argv), not per `(kind,selector)`. +SURE the probe rc is preserved (`apt_get__predict install
  -y nginx` → `dpkg-query -W nginx`, identical to `package_installed__predict 'nginx'`).
- Update ALL 7 `compile_probe` call sites: cli/main.rs:231, coverage/lib.rs:426 (byte-mirror of cli —
  apply the SAME edit, RESOLVED per spike/CLAUDE.md round-20), hostsim:434/531, plan tests
  (observable_matrix, erasability, lib.rs test helper), cli:1235.
- Big-bang re-bless the probe goldens (every probe artifact's shape churns; behaviour-preserving).

### P5 — retire markers
Delete `oracle_kind`/`oracle_probe_*`/`oracle_effect`/`oracle_vouch_converged` from ALL fixtures and the
old `lift`/`KindIndex`/`FactProbe`/`resolve_probe` code paths in oracle lib.rs (+ its marker-based
tests). Final gates + e2e 123/9/0/0 + lens-verify each xfail. Big-bang re-bless with case-by-case diff
inspection (BLESS exclusivity — orchestrator/sole-agent only, freshly-verified binary).

## §6 Validation ledger (this session)
- Baseline before edits AND after every commit — behaviour IDENTICAL: e2e `123 round-trips / 9 xfail /
  0 XPASS / 0 red` (re-run clean after each R4a batch); `cargo fmt --check` · `clippy -D warnings` ·
  `cargo deny check licenses bans sources` · `typos` all clean; full workspace tests green (oracle lib
  42 → 47, +5 differential). No golden re-blessed (nothing emitted-surface touched; R3 not done). No
  behaviour changed (the whole session is additive — derive is dead code outside its tests, the added
  fixture marks are inert under the old lift + removed by strip).
- R4a golden-stability is CORPUS-SCALE behaviour evidence (146 fixtures through the full apply/probe
  exec pipeline unchanged), stronger than the lift-only differential — but never-vouch still applies:
  this is machine-run process-evidence, not a human-battle-tested correctness claim.
- OPEN before the mission completes: (1) jc-singleton-mark ruling (unblocks pkgindex + P4 wiring);
  (2) P4 wiring + the EstablishInverted→MustRun freeze (with the §3 code-note); (3) P4b R3 probe reshape
  + all 7 compile_probe call sites (incl. coverage/lib.rs:426 byte-mirror) + probe-golden re-bless;
  (4) P5 marker retirement + old-lift deletion; (5) the big-bang re-bless with case-by-case inspection +
  lens-verify each of the 9 xfails. A corpus-wide `derive == old-lift` iterating test (over the 146
  converted fixtures) is worth adding before the wiring flips, as the final differential gate.

## §7 Finishing session (2026-07-02, jc-singleton-mark RULED) — P3-remainder + flip-gate + P4 LANDED; R3 STOPPED on jc-probe-command-divergence

AI-authored, append-only. Never-vouch applies (machine-run process-evidence, not proof). Confidence marks throughout.

### §7.1 LANDED (4 commits on `ai/spike3-r23`, each gates-green + e2e 123/9/0/0)
- **`be1f8da (AI new ana) R4a-remainder: 5 pkgindex Singletons via the empty-entity mark`** — the jc-singleton-mark
  ruling (empty entity slot `kind:.prop`) implemented in `split_mark_target` (`oracle/src/predict/parser.rs`):
  `pkgindex:.fresh` now parses DELIBERATELY as `entity=Some("")` (the-one, a real value ≠ `None`),
  `prop=Some("fresh")` — 23H flagged the prior parse as accidental (read `.fresh` as the entity, dropped the
  selector). The 5 `pkgindex.oracle.sh` (byte-identical) converted: `update) idx : pkgindex; test -n fresh : pkgindex:.fresh`.
  +SURE tested (3 new parser tests: the empty-entity parse, the same AFTER an fd-dup redirect per the ruling,
  and the two near-miss typos `pkgindex.fresh`/`pkgindex:fresh` that drop the selector — "fail loudly" at the gate).
  RESERVED wildcard (`kind:*.prop`) untouched, as ruled.
- **`af83ba0 (AI new ana test) R2: corpus-wide derive==markers flip-gate`** — the mandated flip-gate
  (`oracle/tests/corpus_differential.rs`): iterates ALL 151 e2e oracle fixtures asserting
  `derive_predict(inline) == lift(markers).effects` in BOTH directions (a transitional `KindIndex::effects_iter`
  accessor makes it total; deleted with the lift in P5). **The gate CAUGHT a real R4a bug** (+SURE):
  `guard-status-blocks-elision/tool.oracle.sh` models `command -v` as ESTABLISH (its marker + comment say so,
  to block an idempotency-guard's elision) but was converted with an OBSERVE `:?` mark — corrected to `:`.
  This is the flip-gate earning its keep; gate green 151/151.
- **`8ac750c (AI ana) R2 P4a: retire Polarity → ValueClaim`** — cov-q4/jc-polarity-vs-rc FINAL: `Polarity`
  DELETED workspace-wide; `EffectCell.claim: ValueClaim`; the marker `lift` maps `establish/kill/query` →
  `Establish/EstablishInverted/Observe` (so the gate stays valid, both sides speak ValueClaim). **The §3 FREEZE
  code-note lives at `analysis/src/effect.rs::cell_effect`** on the `ValueClaim::EstablishInverted => CommandEffect::Kills`
  arm (verbatim-in-spirit: "no polarity doctrine here — dissolves into the uniform no-vouch-no-elide license
  when the guard/vouch tier lands"; ru-26 spike-scoped-churn disclosure present). Behaviour-identical
  (`Kill`→`EstablishInverted`→`Kills`: gen-into-Reach + `_ => MustRun`). NB `hostsim/differential.rs` has its OWN
  local `Polarity{Establish,Query}` enum (the DST judge's model) — deliberately UNTOUCHED. Added `lift_derived`
  (derive-sourced effects + marker-sourced probes) as groundwork.
- **`0557f2e (AI ana) R2 P4b: flip analysis to derive-sourced effects`** — cli + coverage build the effect-map
  via `lift_derived` (predict bodies), not the markers. THE WIRING IS FLIPPED: analysis now reads the inline
  derivation. Converted the callers' inline oracle fixtures to carry marks (coverage `PACKAGE`/`PKGSTATE`,
  coverage/main `PKG_ORACLE`, hostsim's `oracle_text` generator). e2e 123/9/0/0 — the differential gate + stable
  e2e ARE the zero-behaviour-change evidence.

### §7.2 DIFFERENTIAL EVIDENCE (the zero-behaviour-change anchor)
- The corpus flip-gate (`derive == old-lift` over all 151 fixtures) is GREEN, and it caught the one real conversion
  bug (§7.1). Because analysis (P4b) now consumes the derivation, the gate's `derive==markers` equality is the proof
  that the marker→derive swap changed no effect cell.
- e2e stayed `123 round-trips / 9 xfail / 0 XPASS / 0 red` across every commit AND after the P4 flip (the exec
  gates — apply/probe-under-mocks + gate-1 parity + argv differential — are the behaviour anchor, golden-independent).
  gates: fmt/clippy(-D)/deny/typos all clean; full workspace tests green. No golden re-blessed (nothing emitted-surface
  touched — R3 not landed).

### §7.3 R3/P4b — ATTEMPTED, CODE-CORRECT-IN-CORE, then STOPPED (jc-probe-command-divergence, --WONDER→+SURE)
Implemented the full R3 reshape (ProbePredict: `provider`+`argv`+stripped-funcdef `sh`; `compile_probe` threads
`ValueFlow` + a `Fn(Symbol)->Option<String>` provider→`strip_predict` closure; `render_sh` dedups the funcdef per
provider + invokes `<provider>__predict <F-quoted argv…>`; `render::probe::{check_def,invocation(argv)}`; cli+coverage
build the provider→stripped-body map). **The core is CORRECT** (+SURE): the `converged` and `seam` probes emit exactly
the 23D §1 shape (`apt_get__predict 'install' '-y' 'nginx'` running the stripped body). 80/88 non-xfail cases were
exec-behaviour-CORRECT (only golden TEXT churned — expected re-bless).

**THE STOP (a design-shaped finding the recipe got wrong):** 8 cases FAILED gate-1 (mocked-probe records diverge),
because **the R4a predict-body probe commands DIVERGE from the retired `oracle_probe_*` bodies** the mocks + goldens were
authored against. §5-P4b's "+SURE the probe rc is preserved" is DISPROVEN for `pkgstate`/`tool`/`service`/`firewall`:
- `pkgstate`: old `dpkg -s "$1"` vs check `dpkg -s -- "$pkg"` — the `--` shifts the operand to `$3`; the positional
  mock (`case $2 in nginx)…`) reads `--`≠nginx ⇒ reports `absent rc=1` where authored is `holds rc=0`. VERIFIED by exec.
- `service`: old `systemctl is-enabled --quiet "$1"` vs check `systemctl is-enabled -- "$svc"` (drops `--quiet`, adds `--`).
- `tool`: old `command -v "$1"` vs check `command -v -- "$tool"` (adds `--`; and `command -v` is a shell BUILTIN, un-shimmable).
- `firewall`: old `ufw status 2>/dev/null | grep -q "$1"` (a PIPE) vs check `ufw status "$rule" >/dev/null 2>&1`. The old
  grep-pipe form is **NOT EXPRESSIBLE in the predict dialect** (no `|`), so the predict body CANNOT ship the old probe — the
  shipped probe MUST differ.
- `seam-two-providers-one-kind`: old kind-default `dpkg-query` for BOTH apt AND yum sites (an imprecision) vs NEW
  per-provider `apt_get__predict`(dpkg-query) + `yum__predict`(rpm -q). The per-provider probe is MORE correct but the case
  has no `rpm` mock.

**Why this is a STOP, not just churn (+SURE, the load-bearing reason):** most `--`/`--quiet` divergences are
behaviour-EQUIVALENT on a REAL host (dpkg/systemctl tolerate `--`/`--quiet`, same rc) — the mocks are merely brittle
(positional `$2`). BUT completing R3 requires re-authoring the mocks (+ some probe-results) to match the predict-body
commands AND re-blessing the goldens **in the same pass**. Co-authoring the mock (ground truth) and the golden
(prediction) together **defeats the exec-gate firewall**: a mock and a golden that agree on a WRONG behaviour both pass,
masking a regression — the exact anti-masking hazard `inv-probe-sourced-values` forbids ("no test may hand-inject an
observable the check itself should predict"). I will NOT autonomously co-re-author ground-truth + prediction and vouch
the result (never-vouch). The firewall case additionally forces a genuine probe-behaviour change (the dialect can't
express the old grep-pipe).

**jc-probe-command-divergence (FLAG, needs a human ruling before R3 lands):** for the divergent kinds, which shipped
probe is correct — (a) ALIGN each predict-body probe command to its old `oracle_probe_*` body (zero-behaviour-change under
the existing mocks; but IMPOSSIBLE for firewall's grep-pipe, and it makes the "authored predict body" bow to the retired
stand-in), or (b) SHIP the predict-body command as-authored and re-author every diverging mock + probe-results + golden
(design-pure — the predict() IS the oracle — but a behaviour change under mocks, and the mock+golden co-authoring must be
adversarially cross-checked to avoid masking)? A third option: (c) a per-kind case-by-case mix (align where dialect-
expressible + rc-identical, re-author where not). This is design-shaped and unruled; per the mission's "flag jc-, stop
that item" I stopped. The R3 CODE reshape above is re-implementable in ~1 focused pass once (b/c) is chosen — the
compile_probe/render/ProbePredict design is settled and validated on `converged`/`seam`.

### §7.4 The remaining recipe (unchanged from §5 except the jc-probe-command-divergence gate)
P4b (R3): reshape as in §7.3 (validated) → **resolve jc-probe-command-divergence FIRST** → then re-author the ~8
divergent cases' mocks/probe-results → re-bless probe goldens (case-by-case; the exec differential must stay the
independent ground truth — do NOT co-author a mock and its golden without an adversarial cross-check). P5: author the
minimal `dpkg.predict()` for jc-dpkg-i (the unit fixture `fixtures/package.oracle.sh` breaks when `oracle_effect` is
deleted — it has no check to derive from); delete markers from ALL fixtures + the old `lift`/`KindIndex`/`Polarity`(gone)
/`FactProbe`/`resolve_probe` + the `corpus_differential` gate + `KindIndex::effects_iter` + `lift`-not-`lift_derived`
(rename `lift_derived`→`lift`). Step 6: add `: provider:verb~` vouch-marks to the guard23 install arms, re-lens-verify
each xfail. Step 7: big-bang re-bless, all 9 xfails fail for their designed reason.

## §8 R3 RE-LANDED (2026-07-02, ask-probe-divergence RULED (b)) — LANDED + green; P5/vouch-marks REMAIN

AI-authored, append-only. Never-vouch applies (machine-run process-evidence, not proof). Confidence marks throughout.
Base: `b4d6dc0` (the ruling commit). Two commits landed on this worktree branch, gates+tests+e2e green at each:
`8652e23` (mocks-first) and `269ab5e` (R3 code + 88 probe goldens).

### §8.1 The R3 reshape (LANDED, `269ab5e`) — +SURE, compiles/tests/e2e green
- `plan::ProbePredict` now carries `provider: Symbol` + `argv: Vec<Symbol>` + `sh` = the whole stripped
  `<provider>__predict` funcdef ([`strip_predict`], already-existing). `compile_probe` threads `&ValueFlow` and a
  `Fn(Symbol,&[Symbol])->Option<String>` ship closure; `ship_for_argv` splits word0+operands, ⊤ command word or ⊤
  operand ⇒ un-shippable (`kFAIL-perform`). member (`member_argv`) + inline (`argv_values(body.node)`) paths threaded.
- `render_sh` dedups the funcdef per funcname BUT **re-emits when the body changes for a funcname** — the collision fix
  (see §8.3). `render::probe::invocation` = `<provider>__predict 'w0' 'w1' …` (F-quoted per word). `predict_fn_name` is now
  per-provider (`to_funcname_segment(map_provider_name(word0))`), matching what `strip_predict` mangles the funcdef to.
- 7 call sites: cli `ship_predict_body` (re-runs the analysis's own check resolution — first check, oracle-file order,
  whose provider matches + argparse resolves the argv — then strips it; so the shipped probe checks EXACTLY the fact the
  analysis decided), coverage byte-mirror `ship_predict_body`, hostsim ×2 + plan tests ×N (each threads a `ship_corpus`
  seam). `resolve_probe`/`FactProbe`/`add_probe` are now dead outside the retained marker path (P5's to delete).

### §8.2 Mock-vs-golden SEQUENCING evidence (the anti-masking spine — RULED (b))
- **Discovery**: landed R3 in the working tree → e2e revealed EXACTLY 8 enforced gate-1 breaks (matching §7.3's "8
  cases"), ALL mock-serviceable, and NO other exec-gate break; the other 50 (→88 after clippy/name churn shook out) are
  content-diff only = behaviour-correct, expected re-bless. The divergent kinds §7.3 under-counted (pkgindex `test -n
  fresh` tautology, confline `grep -q -- "$pat" "$file"`, service, tool, firewall) did NOT enforce-break: service mocks
  branch on `$1` (the subcommand, unchanged by `--`); tool's `command -v --` is rc-identical to `command -v` (verified
  dash+bash); firewall/pkgindex divergent cases are `PROBE_RESULTS=authored` (opted out). The enforced 8: 7 pkgstate
  `dpkg` operand-mocks (break on `dpkg -s -- <pkg>` ⇒ `$2=--`) + seam `rpm` (unshimmed, rc=127).
- **Mocks first, own commit** (`8652e23`): re-authored the 8 mocks `--`-tolerant (`pkg=$2; [ "$2"="--" ]&&pkg=$3`, +
  add seam `rpm`) — behaviourally IDENTICAL for the old command `dpkg -s nginx` ($2=nginx); rpm inert under the old
  probe. **Exec-verified with the OLD binary** (R3 stashed, rebuilt at b4d6dc0): `all 123 passed` — proves the mocks
  serve the FROZEN `probe-results.txt` independently of any new golden. No pinned intent flipped (gate-1=0 under R3
  confirms it the other direction). probe-results.txt NEVER moved — it is the immovable anchor both old+new mocks serve.
- **Goldens second, separate commit** (`269ab5e`): R3 code + re-bless. Never a mock and a golden for one case in one
  commit. The exec gates (gate-1 parity, ap-2 apply-exec) are the golden-independent behaviour anchor throughout.

### §8.3 jc-provider-check-collision — RESOLVED WITHIN THE RULING (not a STOP), +SURE
4 cases (`exec-poison-wall-dead`, 3× `headline-*`) have TWO files each defining `apt_get__predict` (`package` +
`pkgindex`). Per-provider dedup would collide (one funcname, two bodies). NO new naming scheme was invented: because the
render emits the funcdef INTERLEAVED with invocations, re-emitting the body before an invocation whose funcname's
currently-defined body differs makes sh's last-writer-wins + top-to-bottom exec give each invocation its own body. The
cli/plan ship closures pick the SAME check the analysis resolved (per argv), so `install …` ⇒ package body, `update` ⇒
pkgindex body. exec-poison-wall-dead + the headline cases pass (content-diff only). This is a genuine gap §7.3 didn't
reach (it STOPPED earlier, on the command divergence); flagged here because it was un-anticipated, but it needed no
ruling — the interleaved-render property carries it.

### §8.4 BLESS hazard hit + recovered (process note, the sharpest)
`BLESS=1 sh e2e/run.sh` regenerates EVERY `expected.out` AND `expected.ran` — INCLUDING the hand-authored XFAIL
goldens/run-sets (the desired-FUTURE guarded state, which the guard-tier-unbuilt engine cannot produce). It overwrote
the drift-trio's `expected.ran` (desired `dpkg-query -W curl` short-circuit → the no-guard `apt-get install -y curl`),
turning 3 xfails XPASS. RECOVERY: R3 changes only the PROBE, never the apply, so ALL `expected.ran` changes were
spurious/damaging → restored every one; restored the 9 XFAIL `expected.out` (hand-authored, content-diff-skipped);
kept only the 88 NON-xfail `expected.out` re-bless. Re-verified `all 123 passed` / 9 xfail / 0 XPASS / 0 red,
`head-expected.ran` untouched. LESSON for the P5 big-bang bless: BLESS the non-xfail set only; the XFAIL goldens are
hand-authored future state — restore them after any global BLESS (the promotion rule, run.sh:44-52, in action).

### §8.5 REMAINING (not started — P5 + vouch-marks + their bless)
Unchanged from §7.4 steps: P5 (jc-dpkg-i minimal `dpkg.predict()`; delete `oracle_kind`/`oracle_probe_*`/`oracle_effect`/
`oracle_vouch_converged=` from all fixtures + old `lift`/`KindIndex`/`FactProbe`/`resolve_probe`/`corpus_differential`
gate/`KindIndex::effects_iter`; `lift_derived`→`lift`) and Step 6 (guard23 `: apt-get:install~` vouch-marks + re-lens-
verify each xfail). The §8.4 lesson governs any re-bless they need. R3 is the unblock they were waiting on.

## §9 CLOSING SESSION (2026-07-02) — P5 + BLESS-guard LANDED; vouch-marks STOPPED on a strip-fidelity bug

AI-authored, append-only. Never-vouch applies (machine-run process-evidence, not proof — "the marker fiction
is fully retired" is a machine-gate claim, not a human-battle-tested one). Confidence marks throughout. The
`23K §2` rc/verdict naming discipline is in effect (never bare "rc": tool-rc / predicted-rc / apply-rc).
Base: `09f1f54` (the round-23 tip). Four commits landed on this worktree branch, gates + workspace tests +
e2e green at each (except the git-HEAD-relative diag retire-guard, which resolves the instant the diag.rs
deletion is committed — pre-commit gates are fmt/clippy/deny/typos, not `cargo test`).

### §9.1 LANDED — P5 marker retirement (2 commits)
- **`9aaf5c1 (AI re) P5: retire the marker lift`** — `lift_derived`→`lift` (the sole lift now; it builds the
  effect-map purely from `predict::derive_predict` over each `<provider>__predict`). DELETED: the marker `lift`/
  `lift_one`/`bind`/`RawEffect`/`scan_kind_assigns`/`lift_command` + `FactProbe` + `KindIndex.{probes,
  selector_probes}` + `add_probe`/`add_selector_probe`/`probe_for`/`selectors_for_kind`/`resolve_probe`/
  `effects_iter` + all marker-based oracle unit tests + `tests/corpus_differential.rs` (flip-gate; job done).
  Converted the 3 derive differential tests to assert the derived cell-set directly (dropped the marker-`lift`
  comparison half; the derivation coverage survives). **Diag-catalog consequence (+SURE, in-scope):** the 8
  now-dead oracle marker diag codes (`oracle-{non-literal-kind,missing-kind,missing-probe,bad-effect,top-level-
  mutator,non-declaration,duplicate-effect,probe-selector-roundtrip}`) were deleted from `core::diag` + the
  `diag_tidy` MIGRATED_PAYLOADS/MIGRATED_SLUGS/SPANLESS allow-lists (else `every_catalog_variant_is_constructed`
  reds — dead catalog). Callers `cli`/`coverage` swapped to `lift`; hostsim `oracle_text` generator + coverage
  inline fixtures stopped emitting markers; `plan/tests/erasability` ORACLE_SRC + two parse/check-test samples
  converted to the marked dialect. Behaviour-identical: the retired flip-gate had proven `derive == old-lift`,
  and e2e stayed 123/9/0/0 (the exec gates are the golden-independent anchor).
- **`5361238 (AI re) P5: delete the four markers from every fixture + jc-dpkg-i`** — stripped the four marker
  forms + the transitional additive-differential comment blocks from all 151 e2e oracle fixtures. **ZERO golden
  churn** (+SURE, verified): markers are top-level lines, never inside a `<provider>__predict` body, so
  `strip_predict` never shipped them and no `expected.out`/`expected.ran` contained one (grep-verified before the
  edit). **jc-dpkg-i RULED-and-done:** authored a minimal verbless `dpkg__predict` (strips `-i`, so `dpkg` is the
  ε-verb) + converted the unit fixture's `apt_get__predict` to the marked dialect; the pinned test
  `lifts_the_package_fixture_cleanly` now asserts the ε-verb establish cell (dpkg -i ⇒ package#installed),
  intent preserved. **Seam consequence (RULED, cov-q3 / jc-fblessed):** deleted `seam-two-providers-one-kind/
  expected-diagnostics` — its `oracle-missing-probe` was the per-file probe-completeness floor that "evaporates
  under check-as-oracle"; gate-3 only fails on UNDECLARED errors, so its disappearance is clean.

### §9.2 LANDED — BLESS hardening (1 commit, folded-in review item)
- **`9c5f78c (AI test) harden BLESS`** — `exec_check` blessed `expected.ran` unconditionally, so a global
  `BLESS=1` clobbered the hand-authored XFAIL run-sets (§8.4: flipped the drift-trio to XPASS, twice). Gated the
  ran-bless on `XFAIL_ACTIVE != 1`; the case then falls through to the ordinary compare and stays a red `xfail`,
  goldens untouched (the `expected.out` bless was already XFAIL-guarded structurally — the `elif BLESS` arm is
  unreachable for an xfail case). **BLESS-guard DEMONSTRATION (+SURE):** snapshotted the md5 of all 25
  `expected.{out,ran}`/`head-expected.ran` across the 9 XFAIL cases → ran `BLESS=1 sh e2e/run.sh` (blessed 123
  non-xfail cases) → re-snapshot: **byte-identical, zero XFAIL goldens touched.** (The bless additionally
  wanted to normalise 4 all-elided cases' empty `expected.ran` → a single `\n`; behaviourally identical, both
  compare-empty; reverted as unrelated to this session's work.)

### §9.3 LENS-VERIFY — all 9 xfails fail for their DESIGNED reason post-P5 (+SURE, one lens-lifted run)
Removed all 9 `XFAIL` files, ran e2e, restored via `git checkout` (robust vs the §7.5/23G mv-fragility). Each
case's designed gate-failure (23A §1 / 23G §1) reproduced exactly — none accidental, none for a new reason:

| case | designed failure (23A/23G) | observed (lens-lifted) |
|---|---|---|
| guard23-ternary-flagship | ap-2-exec ran-mismatch + gate-1 parity | ap-2-exec + gate-1 ✓ |
| guard23-fallthrough-drift-runs | ap-2-exec ran-mismatch | ap-2-exec ✓ |
| guard23-fallthrough-canttell-runs | ap-2-exec ran-mismatch | ap-2-exec ✓ |
| guard23-mutator-fails-book-continues | ap-2-exec ran-mismatch | ap-2-exec ✓ |
| guard23-why-attribution | gate-7 (no why-line) | gate-7 ✓ |
| guard23-heredoc-refuses-loudly | gate-7 (`refus`) + gate-1 parity | gate-1 + gate-7 ✓ |
| guard23-var-namespace-isolated | ap-2-exec ran-mismatch + gate-1 parity | ap-2-exec + gate-1 ✓ |
| guard23-nounset-book-survives | gate-1 parity ONLY | gate-1 ✓ |
| guard23-redirect-line-runs | gate-7 (`refus`) + gate-1 parity | gate-1 + gate-7 ✓ |

### §9.4 STOPPED — the guard23 vouch-marks + flagship golden re-derivation (jc-vouch-mark-strip-fidelity)
**The sharpest finding of the session (--WONDER→+SURE, empirically verified). The mission/§7.4/23H-§5 recipe —
"replace `oracle_vouch_converged=` with a bare `: apt-get:install~` mark on the install arm's path" — produces
a BROKEN shipped predict body under the current `strip_predict`.** The vouch is a bare `MarkKind::ConvergedVouch`;
`strip_predict` maps every bare mark → the `:` null command (`check.rs:147`). Placed per the recipe (trailing the
effect-marked probe command), the install arm strips to:

```
install) dpkg-query -W "$pkg" >/dev/null 2>&1; : ;;    # verified: dorc probe output on the flagship
```

The trailing `:` runs LAST, so the arm's exit status — the tool-rc the predict body mints into the guard's
apply-rc — is forced to 0 regardless of `dpkg-query`'s tool-rc. Consequences (both verified by exec):
1. **As a probe:** every install site reports `holds` (apply-rc 0) even when the package is absent — the
   convergence signal is destroyed.
2. **As a guard:** `apt_get__predict install -y curl || apt-get install -y curl` always short-circuits (apply-rc
   0 ⇒ "converged") ⇒ the mutator NEVER runs ⇒ **wrong-elision**, the exact sin the guard tier exists to prevent.

This is the `np-pathgrain` gap 23A explicitly left UNPINNED ("the vouch's own strip-fidelity — that the real
spelling strips out of shipped bodies … my assignment-spelling never enters a body, so there is nothing to
strip"). The mission MOVES the vouch INTO the body, so its strip-fidelity is now load-bearing and unruled. Three
candidate resolutions, each a design decision I did NOT make (per the mission's flag-jc-and-stop rule):
- **(A) strip deletes `ConvergedVouch` bare marks entirely** (not → `:`). Design-correct — the vouch "NEVER
  enters the fact-plane … engine-side" (rul-guard-license), so it should not ship to the host at all. Fixes the
  rc-clobber AND preserves the **P-pair byte-identity** (§9.5). Cost: a `strip_predict` contract change (the
  contract 23A hz-strip-scope already flagged as under-pinned). *My recommendation, ~SUSPECT.*
- **(B) place the vouch mark FIRST in the arm** (`install) : apt-get:install~; dpkg-query … ;;`). Verified to
  strip to `install) :; dpkg-query … ;;` — the `:` is first, `dpkg-query` last, so the arm's tool-rc survives
  (correct). No code change. Cost: ships a leading `:;` ⇒ **breaks the P-pair byte-identity** (cosmetically; the
  substantive "vouch changes no elide/poison/run" still holds, but the pin's enforcement weakens to
  substantive-equivalence, review-judged per jc-pair-mechanics).
- **(C) the mission's literal trailing placement** — BROKEN (the rc-clobber above). Not viable.
- A fourth consideration: 23K makes the vouch spelling a NEXT-ROUND redesign (`foo_is_converged`/dq-kOOB), so
  strip-fidelity work on the strawman `: provider:verb~` mark may be throwaway — an argument for deferring the
  whole item to the interface round rather than fixing strip now.

**Because the vouch-mark is blocked, the flagship golden re-derivation (mission step 3) is also blocked** — the
flagship's guard is licensed ONLY by the vouch (rul-guard-license: no vouch ⇒ no guard), so re-deriving its
guard preamble as `strip(oracle body)` presupposes the vouch mark is in the body in a settled form. The flagship
`expected.out` therefore stays as-is: hand-authored future-state (xfail, content-diff-skipped), now stale in
three compounding ways — pre-R3 probe half (`package_installed__predict 'nginx'`, frozen at §8.4's restore),
pre-R4a guard preamble (markless body), and shows-a-guard-without-a-vouch (post-P5 the oracle's vouch line is
deleted). All three reconcile together once (A/B) is ruled + the guard tier is built. FLAGGED, not touched.

### §9.5 Other flags (jc-*) — made-for-now, cheap to reverse, never silently
- **jc-vouch-bait-scope** (~SUSPECT): 24 guard23 fixtures carried `oracle_vouch_converged=` (added by 23A/23G
  AFTER 23H-§5's recipe, which names only flagship + pair-b). The acceptance grep forces deleting it from all
  24; the mission re-marks only flagship + pair-b. So the other 22 FLOORS lose their vouch-BAIT (the "converged +
  vouched, still runs" cases become "no vouch, runs" — weaker future-build discrimination). Kept mission-literal
  because (a) at HEAD the vouch is inert (guard tier unbuilt) so e2e stays green, (b) re-marking 22 floors would
  churn their probe goldens with the same `:` (§9.4) and hit the P-pair problem broadly, (c) 23K re-spells the
  vouch next round anyway. Their orphaned `# STRAWMAN VOUCH` comment blocks are LEFT (grep-safe — they say
  "vouch"/"converged-vouch", never `oracle_vouch`; and they still point at dq-kOOB/the next round). A reader
  should know the 22 floors' vouch-bait is temporarily absent until the interface round restores it.
- **jc-claude-md-history** (--WONDER): the acceptance grep (`grep -rI 'oracle_kind\|oracle_probe\|oracle_effect\
  |oracle_vouch' spike/`) is empty across all source/fixtures/tests EXCEPT two docs — `spike/CLAUDE.md` (6) and
  `spike/crates/oracle/CLAUDE.md` (4) — whose marker strings live ONLY inside human-directed `[CORRECTION — …
  human-directed]` blocks explicitly labelled "left standing as history" / "Original text stands as history".
  Editing human-directed content is out of an agent's remit, and the charter's own intent is to preserve that
  history; I read these as the acceptance's "historical notes" exemption in spirit. `spike/crates/syntax/CLAUDE.md`
  (3, plain stale examples, not history-tagged) WAS cleaned. If a strict grep-empty is wanted, the two history
  blocks are the human's to purge. (The `spike/target/` binary matches are gitignored build artifacts.)
- The seam `expected-diagnostics` deletion (§9.1) is a RULED consequence (cov-q3), recorded not flagged.

### §9.6 Validation ledger (this session)
- Gates green at each commit: `cargo fmt --check` · `clippy -D warnings` · `cargo deny check licenses bans
  sources` · `typos`. Workspace tests: 521 passed / 0 failed (the diag retire-guard is HEAD-relative — red only
  in the uncommitted window, green post-commit; pre-commit gates exclude `cargo test` by the spike/CLAUDE.md
  discipline). Fresh-build `sh e2e/run.sh` = **123 round-trips / 9 xfail / 0 XPASS / 0 red**.
- Marker retirement is CORPUS-COMPLETE in the source tree (grep-empty bar the two history-block docs above).
- Never-vouch holds: all of the above is machine-run process-evidence. The vouch-mark STOP is the honest
  boundary — I will not ship a probe/guard I verified is wrong-eliding, nor unilaterally change the strip
  contract on an unruled, next-round-redesigned strawman.

## §10 — the `check()`→`predict()` mechanical rename (2026-07-03, AI-authored, append-only)

Never-vouch applies (machine-run gate/e2e process-evidence, not a human-battle-tested correctness claim).
Executes the 23L addendum rename ruling (task #18): the oracle role-function formerly `check()` → **`predict()`**
corpus-wide; its aggregate status IS the predicted-rc, so the name now matches (inv-one-observable's own verb).
The verdict-functions `is_converged()`/`is_diverged()` were already correctly spelled and are UNTOUCHED (verified:
no `is_converged`/`is_diverged` occurrence changed). Historical-truth explicitly waived by the ruling ("global
find/replace … don't care about historical truth"); stamped-`233` + signed-`239` INCLUDED — `233` carries a
one-line disclosure appended to its existing `<!-- /* LATER-WORK ANNOTATION` block.

### §10.1 Landed — three granular commits on this worktree branch (gates green each)
- **`(AI re ana)` Rust engine** — module `oracle/src/check{.rs,/}` → `predict{.rs,/}` (git mv, rename-tracked);
  `oracle/tests/check.rs` → `tests/predict.rs`; types `Check`/`CheckSet`/`CheckHeader`/`ProbeCheck` →
  `Predict`/`PredictSet`/…; fns `strip_check`/`derive_check`/`lift_checks`/`ship_check_body`/`check_fn_name`/
  `parse_check_funcdef`/`at_check_funcdef` → predict-equivalents; consts `CHECK_SUFFIX`/`PERIOD_CHECK_SUFFIX`/
  `CORPUS_CHECK_SRC(_Q)` → predict-equivalents; the diag Check-family `CheckOutOfDialect`/`CheckUnterminated`
  (+ slugs `check-out-of-dialect`/`check-unterminated`) → `Predict…`/`predict-…` (no golden ripple — the slugs
  appear in no `expected-diagnostics`; the `diag_tidy` MIGRATED_PAYLOADS/SLUGS lists renamed in-sync); test-fn
  names carrying the role. +SURE compiler-checked: fmt/clippy(-D)/deny/typos clean, full workspace tests green.
- **`(AI re test)` fixtures + goldens** — 152 `*.oracle.sh` + all `book.sh` funcdefs `<provider>__check` →
  `__predict`; 94 non-XFAIL `expected.out` re-blessed (BLESS exclusive, sole-agent, freshly-built binary).
- **`(AI re dsn)` prose** — root docs, `spike/CLAUDE.md`, per-crate CLAUDE.md, `Research/plans/*`,
  `Research/notes/*` (this note incl.), strawmen; + fixtures-residual funcname refs (mocks/markers/`run.sh`
  selftest samples/`plan/Cargo.toml` comment).

### §10.2 The parser cut — HARD-CUT to `.predict` (~SUSPECT correct; decided in-session)
The R1 dialect parser now recognizes ONLY `<provider>.predict()` / `<provider>__predict` (`PERIOD_PREDICT_SUFFIX
= ".predict"`, `PREDICT_SUFFIX = "__predict"`); the old `.check(`/`__check` spellings are hard-cut, NOT kept in an
additive window. Rationale: every fixture, strawman, and test-string was renamed in the same pass and NOTHING
external depends on the `.check` spelling, so an additive `.check`-also window would strand nothing — it would only
carry a dead alias. Matches the human's lean ("global", historical-truth waived). Self-inflicted bug found+fixed:
the mechanical pass renamed the const NAME `PERIOD_CHECK_SUFFIX`→`PERIOD_PREDICT_SUFFIX` but its VALUE stayed
`".check"` (the `.check(`-with-paren pattern missed the bare `".check"` literal), which stranded 25 oracle tests
feeding the now-`.predict()` strawmen; fixed the value to `".predict"` (the actual hard-cut).

### §10.3 Scope discipline — what was deliberately NOT renamed
Word-boundary-anchored, named-identifier-only churn; bare-word `check`/`checks` left where the token is a
verb/verification sense, never the role. Confirmed OUT and untouched: `checked_sub`/`checked_rem` (Rust int
methods), `crosscheck`, `checkout`, `checksum`, `checklist`; the syntax-crate `check` closure + `check_parts` +
`check_simple_triggers` (parser ⊤-trigger, not the role); `checks_fact` (a probe predicate verb); `run.sh`'s
harness functions (`exec_check`, `guard_shape_check`, `probe_exec_check`), gate-language, "content-check";
`cargo fmt --check`/`deny check`. Bare-`check` local VARS (e.g. `strip_predict(src, check: &Predict)`) were left
as-is — they compile, and a blanket bare rename mangles verb-in-prose comments ("assertions check exactly…").

### §10.4 Deliberate leavings — the acceptance grep (`__check`/`\.check()`) residue, all enumerated
- **The 9 XFAIL `guard23-*/expected.out`** (hand-authored guard-tier FUTURE-state; guard tier unbuilt ⇒
  content-diff-skipped, BLESS-protected). Left BYTE-IDENTICAL (md5 before/after BLESS = unchanged, all 25
  `expected.{out,ran}`/`head-expected.ran`). They still carry `apt_get__check`, `package_installed__check`,
  `name.check()`, `CHECK-BODY` as the frozen desired-state; §9.4 already records they reconcile together (with a
  re-derivation) once the vouch-strip ruling lands + the guard tier builds. The mission's "XFAIL expected.*
  byte-identical" directive is why they were NOT hand-edited. (Their INPUT halves — oracle sources, and the 2
  guard23 books' comments — WERE renamed, so the frozen goldens now cosmetically diverge from their inputs; this
  is the pending-reconciliation state, not drift.)
- **`Research/notes/23L-interface-rulings.md` line 18** (`**foo.check()**` in rul-role-split). 23L is THE ruling
  that DEFINES check()→predict(); it names both spellings by design ("historical material centers on
  check()/predict()", its addendum-2). Renaming line 18's meta-reference while its addendum literally reads
  "`check()` → **`predict()`**, corpus-wide, RULED" would produce nonsense, so the whole file is left as the
  historical record. (spike/CLAUDE.md's own rul-role-split "né `check()`" meta-reference was likewise preserved —
  the mechanical pass HAD collapsed it to "né `predict()`"; caught + restored.)

Judgment-left (NOT acceptance-enforced — bare `check`, no `__`/no dot): ~52 fixture-comment bare-`check` (mix of
role + generic "convergence check"/"check-then-act"), concept terms (`check/probe split`, `check-tax`, `check-rc`),
and assorted historical-note bare-`check` prose.

### §10.5 Validation ledger (this rename)
- Baseline BEFORE any edit (at `d5d1178`): e2e `123 round-trips / 9 xfail / 0 XPASS / 0 red`, full tests green.
- After: `cargo fmt --check` · `clippy -D warnings` · `cargo deny check licenses bans sources` · `typos` all clean;
  full workspace tests green (the diag retire-guard is HEAD-relative — red only in the uncommitted window, green
  post-commit; verified green after the code commit). Fresh-build `sh e2e/run.sh` = **123 / 9 / 0 / 0** (verified
  twice: after the re-bless, and again after the mocks/run.sh rename — the `guard_shape` selftest's now-`__predict`
  sample still passes; mock-comment renames are inert).
- Golden re-bless inspected case-by-case: ZERO non-comment, non-funcname code change — the churn is exactly the
  `__check`→`__predict` funcdef-name + invocation, plus book-comment propagation into re-blessed goldens.
- `grep -rn '__check\|\.check()'` over `spike/ Research/ *.md` (excl quarantine/corpora/target) returns only:
  (a) the CORPUS stray-form leavings — 9 XFAIL goldens + 23L line 18 (enumerated in §10.4); and (b) META-DOCUMENTATION
  that intentionally NAMES the retired spelling to record the rename — this §10 itself, `233`'s one-line disclosure,
  and 23L. A rename-record that could not quote its own before-spelling would be useless; (b) is documentation OF the
  churn, never a subject of it.
