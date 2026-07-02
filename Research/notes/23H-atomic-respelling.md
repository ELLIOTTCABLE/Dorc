# 23H — the atomic re-spelling session (R2/R3/R4): what landed, the differential, the recipe

AI-authored, 2026-07-02. Append-only (append-only-spike-notes). **Never-vouch applies:** everything
here is process-evidence, not proof — the gate/harness tallies are machine-run, but "the reconciliation
is correct" is a human-battle-testing claim I do not make. Confidence marks throughout
(+SURE / ~SUSPECT / -GUESS / --WONDER).

Task frame: complete R2 (retire the marker spellings; derive the engine structures from the inline
dialect), R3 (the probe emitter ships `strip_check(<provider>.check)` per-site with the site's argv),
R4 (convert all live fixtures + big-bang golden re-bless) — as ONE atomic workspace change, under the
three baked-in rulings (jc-dpkg-i, jc-fblessed, jc-polarity-vs-rc FINAL). Design of record: `23E`.

## §0 Outcome in one line (honest scope)

**R2's derivation CORE landed, proven, committed** (the linchpin — the riskiest novel logic, built
ADDITIVE with the 23E-mandated differential proof). A required R1 parser-gap fix landed with it. The
remaining bulk — the cross-crate wiring swap, the R3 probe-emitter reshape, the 25-shape / 152-file
fixture conversion, and the ~120-golden big-bang re-bless — is DESIGNED-AND-RECIPE'd here but NOT yet
landed: it is a large, correctness-critical (wrong-elision is THE sin), attention-intensive change whose
BLESS-discipline re-bless I could not complete AND responsibly verify in one sibling-free session
without risking the exact failure the safety mandate forbids. Tree is GREEN at every checkpoint; nothing
is half-wired or broken. This note makes the remainder mechanical.

## §1 LANDED — R2 derivation core (commit `fe50437 (AI new ana) R2: derive effect-map …`)

`spike/crates/oracle/src/check/derive.rs` (new) walks the inline-dialect check body and reproduces the
effect-map the retired `oracle_effect`/`oracle_kind` markers used to lift (23E §3). +SURE, tested:

- **The derivation** (`derive_check(&Check) -> (Vec<DerivedEffect>, Vec<DerivedVouch>)`): a structural,
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

- **The differential proof** (the 23E §3 safety net, `check::derive::tests`): 5 tests assert
  `derive(inline-dialect) == lift(markers).effects` — package (apt-get install/reinstall establish,
  purge/remove inverted), service (systemctl enable/start establish, disable inverted), tool
  (verbless `command -v` Observe), the converged-vouch mark, and the wildcard-arm-keys-no-verb pin. The
  claim maps to the old `Polarity` label ONLY for the comparison (the end-state carries no polarity).
  oracle lib tests: 42 → 47.

**Nothing is wired.** The old `lift`/`Polarity`/`KindIndex` are untouched; `derive_check` is dead code
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
- **jc-fixture-funcname** (~SUSPECT, for P3): fixtures should keep the legacy `apt_get__check` form,
  NOT the period `apt-get.check` semaphore. Rationale: (i) dash-valid (period+hyphen names are
  dash-invalid) though run.sh only `dash -n`s the RENDERED ARTIFACT (not oracle files, verified
  e2e/run.sh:213 vs :918), so this is belt-and-braces; (ii) the OLD lift (`lib.rs::lift_one`) suppresses
  parse-diags only for `…__check`-suffixed funcdefs — a period-named funcdef would surface spurious
  book-parse diagnostics from the old lift while markers coexist (the additive intermediate, P3). The
  strip is idempotent on the legacy form, and `derive_check`/R3 both key off `check.provider` regardless
  of spelling, so behaviour is identical. The period form is an optional lint-semaphore (23D §1); the
  spike does not need it. Flag, don't settle.
- **jc-dpkg-i** (RULED — author `dpkg.check()`): the unit fixture `fixtures/package.oracle.sh` (and its
  pinned test `lifts_the_package_fixture_cleanly`, oracle lib.rs) declares `oracle_effect dpkg -i
  establish installed` with NO `dpkg__check`. Under R2 (effects derive from check bodies) there is
  nothing to derive it from. Ruling: author a minimal `dpkg__check` (e.g. `case $1 in -i) shift ;;
  esac; pkg : package = "$1"; dpkg-query -W "$pkg" … : package:"$pkg".installed`). Small, honest;
  preserves the pin's intent (dpkg -i establishes `package:X#installed`). NB no e2e book uses `dpkg -i`
  (only the unit test pins it), so this is a unit-fixture-only edit.
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

### P3 — fixture conversion (25 shapes, 152 files; 23E §5 recipe)
Convert each shape's check body to carry `case $verb` arms + trailing marks, KEEPING the markers
(additive ⇒ always-green; the differential test then proves derive==old across the REAL corpus before
any retire). Per-shape recipes (23E §5), keeping the legacy `…__check` funcname (jc-fixture-funcname):
- **package (61×, apt-get):** keep `pkg : package = "$1"`; inside the `[ "$2" = "" ]` guard add
  `case $verb in install|reinstall) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
  purge|remove) … : package:"$pkg".installed! ;; esac`. (Split install/purge into distinct arms — the
  `*`-arm CORPUS shape cannot key the effect distinction.)
- **service (systemctl):** per-verb arms carry the selector: `enable) systemctl is-enabled -- "$svc"
  : service:"$svc".enabled ;; start) systemctl is-active -- "$svc" : service:"$svc".active ;;
  disable) systemctl is-enabled -- "$svc" : service:"$svc".enabled! ;;`.
- **tool (command -v):** verbless OBSERVE `command -v -- "$tool" >/dev/null :? tool:"$tool".present`.
- **user (useradd):** verbless ESTABLISH `getent passwd "$user" : user:"$user".present`.
- **seam (apt + yum), pkgindex, pkgstate, firewall, confline:** each provider's own `.check()` with its
  own probe + selector arm, same kind (the cross-oracle anchor).
- **guard23-vouch (flagship + pair-b):** replace `oracle_vouch_converged='apt-get install'` with a bare
  `: apt-get:install~` on the install arm's path, under the `# STRAWMAN … dq-kOOB` comment block.
- **CORPUS_CHECK_SRC** (analysis `effect.rs` tests) + oracle `tests/check.rs` fixtures convert too.
- **coverage inline fixtures** (`lib.rs` ~1134, `main.rs` ~527) convert with the rest.
CORPUS-WIDE DIFFERENTIAL (do this): a test iterating every e2e oracle fixture, asserting
`derive(check-body) == lift(markers).effects`. Golden-stability check: adding case-arms must not change
evaluate's (verb,entity) resolution — every book verb needs an arm, else evaluate → `NoProbeReached` →
Opaque ⇒ a site flips resolvable→Opaque ⇒ golden churns. The exec gates catch this; verify e2e green.

### P4 — wire analysis to the derivation + the polarity freeze
- Build the interned index from `derive_check` (a `KindIndex`-equivalent keyed by (provider, verb)); the
  vouch datum carried to wherever `oracle_vouch_converged` went. Swap `analysis::effect` to consume it.
- `cell_effect`: `ValueClaim::Establish → Establishes`; `Observe → Queries`; `EstablishInverted →` the
  freeze (gen-into-Reach + `MustRun`; a neutral `CommandEffect` that poisons + never
  `EstablishAmbient`). MARK the classifier line with the §3 freeze code-note.
- Retire `Polarity` from the lifted representation (blast radius: `oracle`, `analysis::effect`, `plan`,
  `hostsim`, `coverage`, `core::diag` — grep `Polarity`). Verify e2e exec gates stay 123/9/0/0.

### P4b — R3 probe-emitter reshape
- `compile_probe`'s closure `Fn(KindId, SelectorId) -> Option<String>` becomes a source of the
  **stripped check body per provider** (`strip_check(oracle_src, check)`); the invocation becomes
  `<provider>__check <site-argv…>` (needs the site's resolved argv threaded in — `compile_probe`
  currently lacks it; thread the ValueFlow or a per-site argv map). `ProbeCheck`/`ProbePlan::render_sh`/
  `render::probe::*` reshape: dedup the wrapper per PROVIDER (one `apt_get__check`, invoked per verb-site
  with full argv), not per `(kind,selector)`. +SURE the probe rc is preserved (`apt_get__check install
  -y nginx` → `dpkg-query -W nginx`, identical to `package_installed__check 'nginx'`).
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
- Baseline before edits AND after the R2 core commit — behaviour IDENTICAL: e2e `123 round-trips / 9
  xfail / 0 XPASS / 0 red`; `cargo fmt --check` · `clippy -D warnings` · `cargo deny check licenses bans
  sources` · `typos` all clean; full workspace tests green (oracle lib 42 → 47, +5 differential).
- No golden re-blessed (the R2 core touches no emitted surface). No behaviour changed (additive).
