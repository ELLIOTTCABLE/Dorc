# 20J — task-P (the find-1 repair + 20I §3 cleanups): per-selector probes, the resolution rule, and what strained

> Round-20 spike note, append-only. Records task-P (20I §2 find-1 + §3 dispositions): per-selector
> probe declarations (the live under-execute fix), the corpus follow-through, the shimmed-Query-fold
> corpus case, parity rc-tightening, the same-cell conflict floor, the pinned multi-operand hazard, and
> doc currency. AI-authored, confidence-marked. Trust R/D/I/K + 19H/19I + the human rulings over this.
> Builds on 20C (the WIRE + strain-D1-perselector), 20E (the Query class + tc-query-bare-elision),
> 20F (the gates + tc-probe-parity-projection), 20H (the gate-5 carve-out + tc-fixture-comment-rename),
> 20I (the charter-crosscheck that dispatched this task).

## §0 What landed (all gates green from `spike/`: `cargo fmt --check`, `cargo clippy --workspace
## --all-targets -D warnings` no-new-expects, `cargo test --workspace` 64-lib + suite-wide 0-fail-1-ignore;
## `sh e2e/run.sh` 52/52 ZERO xfail all six gates; `typos spike` clean from worktree root)

- **item-1 (per-selector probes — the find-1 fix).** `oracle::KindIndex` gains a
  `selector_probes: BTreeMap<(KindId, SelectorId), FactProbe>` alongside the kind-default `probes`. The
  lift recognizes `oracle_probe_<kind>_<selector>` funcdefs; resolution is `KindIndex::resolve_probe`.
  `plan::compile_probe`'s closure is now `Fn(KindId, SelectorId) -> Option<String>`; the emitted wrapper
  is keyed per `(kind, selector)` (`check_fn_name` ⇒ `<kind>_<selector>__check`).
- **item-2 (corpus).** The five service oracles ship per-selector probes (is-enabled / is-active); new
  boundary case `exec-enabled-not-active-host`; all probe-shipping goldens re-blessed (the wrapper
  rename — §3).
- **item-3 (shimmed-Query-fold).** New case `exec-shimmed-query-fold`: a `dpkg -s X || apt-get install X`
  fold under FULL enforced gate-1 parity + all gates.
- **item-4 (parity rc-tightening).** `e2e/norm_parity.awk` + the gate-1 (b) rewrite: a site whose
  authored record carries `rc=` is compared with it; rc-less sites stay effect-only.
- **item-5 (same-cell conflict floor).** `cli::facts_from_sites` merges same-cell records via
  `merge_observable` (⊤-on-disagreement), never last-write-wins.
- **item-6 (pinned hazard).** `oracle/tests/check.rs::naive_oracle_without_operand_guard_drops_trailing_operands_known_hazard`;
  `R2-MULTIOP` line in `oracle/CLAUDE.md`; the `[ "$2" = "" ]` sentence in 19H §2.1's defect annotation.
- **item-7 (doc currency).** `spike/CLAUDE.md` round-job st-2 note + vouch-closure 20I §3 disposition;
  the six `AndOrStatus` fixture comments swept to `StatusRelaxable`.

## §1 The resolution rule (deliverable) + the mangling scheme (deliverable)

**`KindIndex::resolve_probe(kind, selector) -> Option<&FactProbe>`** (the F-BLESSED sound floor):

1. the **per-`(kind, selector)`** probe if declared (`oracle_probe_<kind>_<selector>`) — always wins;
2. else the **kind-default** (`oracle_probe_<kind>`) **ONLY IF** `selectors_for_kind(kind).len() == 1`
   (the effect-map declares exactly one selector for that kind);
3. else `None` ⇒ the site is **un-probeable** ⇒ `compile_probe` records it `skip-unresolvable` ⇒ the apply
   RUNS it (`kFAIL-perform`).

`selectors_for_kind` is computed over the effect-map cells (`effects.values().flatten()`), so it is the
set of selectors any oracle declared a `(provider, verb)` cell for under that kind. +SURE this is the
right floor: a single probe body cannot soundly observe one of several distinct selectors (an `is-active`
verdict must not discharge an unmet `#enabled`), so a multi-selector kind without per-selector probes is
honestly un-probeable, not silently-wrong. The floor is **structural, not semantic** (critical): Dorc
cannot read whether `is-active` "means" `#enabled` (`inv-referent-agnostic`); it can only observe that a
multi-selector kind needs >1 probe body declared. So a single-selector kind whose one probe body is the
"wrong" command (the old `two-oracles` is-active-for-#enabled mismatch) is NOT caught by the rule — that
stays an oracle-quality (F-BLESSED) concern, not a structural one (§6 strain-P3).

**Mangling scheme** (reuses `check::map_provider_name` semantics, as the prompt required):
- the funcname kind-segment is the kind name in funcname form (`-`→`_`, `to_funcname_segment`); a probe
  suffix equal to it is the kind-default;
- a suffix `<kind-seg>_<rest>` is the per-selector probe for selector `map_provider_name(<rest>)` (`_`→`-`);
- the emitted wrapper is `<to_funcname_segment(kind)>_<to_funcname_segment(selector)>__check`.

`to_funcname_segment` (new, in `oracle`) is the `-`→`_` inverse of `map_provider_name`'s `_`→`-`; it is
the shared home of the convention on the emit/match side. `plan` depends on `oracle` for it now (moved
`dorc-oracle` from dev- to normal-dependency — no cycle: `analysis` already depends on `oracle`).

**The round-trip flag** (`tc-perselector-mangle`, `PROBE_SELECTOR_ROUNDTRIP`): a selector NAME carrying a
literal `_` is flagged WARNING (`underscore_selector_name_flags_roundtrip_warning`) — no
`oracle_probe_<kind>_<selector>` funcname can address it (`oracle_probe_kind_foo_bar` maps to selector
`foo-bar`, never `foo_bar`). +SURE the OTHER direction (funcname segment → selector) ALWAYS round-trips
(a funcname segment is `[A-Za-z0-9_]`, no hyphen, so `_`→`-`→`_` recovers it); the un-expressible case is
detected against the effect-map selectors, where the real names live, not at funcname parse. No corpus
selector has a `_`, so this is a latent-footgun guard, not a live path.

## §2 The emitted-name scheme + its golden cost (a deliberate, scoped churn)

The wrapper rename `<kind>__check` → `<kind>_<selector>__check` is UNIFORM (selector always in the name),
because that is the only **deterministic, local, collision-free** scheme keyed per `(kind, selector)` —
the prompt's `service_enabled__check` / `service_active__check` examples. Cost: every probe-shipping
golden's probe SECTION re-blessed (the `__check` wrapper lines). I verified the discipline three ways:
- **+SURE every `expected.out` delta across all cases is a `__check` wrapper line** (grep: zero
  non-wrapper, non-header changed lines except item-7's three `AndOrStatus`→`StatusRelaxable` comments).
- **+SURE every pre-existing case's APPLY section is byte-identical to HEAD** (a per-case apply-section
  diff over the whole corpus: the only apply-section change is the two NEW cases).
- **+SURE no pre-existing `expected.ran` is modified** (only the two new cases add `.ran`).

So item-1 is provably a probe-section-only change with zero disposition/run-set impact — the 20C §3
discipline upheld. The scheme is documented in `check_fn_name`'s doc + the `PROBE_HEADER` doc-comment
(NOT the emitted header bytes — same zero-extra-golden posture 20H took for the reserved keys; emitting a
header line would churn ALL ~52 goldens incl. zero-probe cases).

## §3 The corpus follow-through (item-2) — every golden delta, justified

- **The five service oracles** (`exec-distinct-selectors`, `headline-guarded-realistic`,
  `headline-partial`, `headline-pi-webhost`, `two-oracles`) now declare per-selector probes. The four
  multi-selector ones (`enabled`+`active`) ship BOTH `oracle_probe_service_enabled` (is-enabled) and
  `oracle_probe_service_active` (is-active) — "exec-distinct-selectors now ships TWO distinct probe
  bodies." `two-oracles` is single-selector (`enabled` only: enable + disable), so it ships only
  `oracle_probe_service_enabled` (is-enabled) — fixing the strain-2 F-BLESSED is-active-for-#enabled
  mismatch the fixture's own comment lamented.
- **No service-site apply disposition flipped.** +SURE, traced: in the three headline cases the service
  mutators ALREADY ran (the `set -e` C-3 status block forces converged mutators to run — 206 §2), so the
  per-selector change only re-bodies the probe SECTION (site 5/7 → is-enabled, site 6/8 → is-active),
  leaving every apply/`.ran` byte-identical. In `exec-distinct-selectors` (no `set -e`) both sites still
  elide (both holds via the now-distinct probes) — same disposition, same empty run-set. So the prompt's
  predicted "flip those sites to skip-unresolvable ⇒ run" did NOT materialize as an apply change for ANY
  existing case, because the multi-selector cases either already ran (errexit) or had both selectors
  holding. The flip's SOUND DIRECTION is instead demonstrated cleanly by the new boundary case (§4).
- **`exec-distinct-selectors` mid-flight datum** (recorded — the flip almost bit): BEFORE item-2 added
  the per-selector probes (i.e. with item-1 alone), `exec-distinct-selectors` flipped to RUNNING both
  service mutators (its old single is-active kind-default became un-probeable under the multi-selector
  floor) AND failed gate-1 parity (the probe emitted no records for the now-unprobeable sites). Adding
  the per-selector probes restored both sites to resolvable+eliding. This is the find-1 fix in miniature:
  item-1 (the floor) makes the under-probe un-probeable; item-2 (the honest probes) makes it probeable
  correctly. The two are a pair.

## §4 The boundary case `exec-enabled-not-active-host` — the run-set (deliverable, hand-derived)

Book (NO `set -e`): `systemctl enable nginx` then `systemctl start nginx`. Host: nginx enabled but
NOT active. Hand-derivation:
- site 0 = `service:nginx#enabled`, EstablishAmbient (enable/start gate DISTINCT selectors ⇒ neither
  poisons the other), resolved via `oracle_probe_service_enabled` (is-enabled) ⇒ `holds rc=0` ⇒
  `Converged`. No consumed status (plain sequence, no errexit). ⇒ **Replace** (elide to `true`).
- site 1 = `service:nginx#active`, EstablishAmbient, resolved via `oracle_probe_service_active`
  (is-active) ⇒ `absent rc=1` ⇒ `Diverged`. ⇒ **Run**.
- **Run-set: `systemctl start nginx`** (only). `enable` elides; `start` runs.

+SURE this host is ONLY expressible now: a single is-active kind-default would have reported BOTH sites
from is-active, so an enabled-but-stopped host could not have `#enabled` hold while `#active` is absent —
it would have wrongly elided the `start` too (the exact find-1 under-execute, made a positive corpus
demonstration). The mock `systemctl` answers `is-enabled` 0 / `is-active` 1 distinctly; gate-1 ENFORCES
parity (the mock reproduces it). Verified via the harness (BLESS captured exactly the hand-derived
run-set).

## §5 The shimmed-Query-fold case (item-3) + the parity-rc rule (item-4)

- **`exec-shimmed-query-fold`** (20I find-2, the 20H §3 temp-run moved into the corpus): `set -e` +
  `dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx`, host = nginx installed. The guard is the
  pkgstate `query` oracle on the EXTERNAL `dpkg` (shimmable, unlike `command -v`), so gate-1 ENFORCES
  parity (no `PROBE_RESULTS=authored` opt-out). Dispositions: site 1 (the `dpkg -s nginx` Query guard,
  VALID — only `set -e` upstream, pristine) reports `holds rc=0`; the fold reads its known rc 0 ⇒ the
  `||` install (site 2) is **Omit**ted (dead branch), and the guard is value-substituted to `true`. The
  whole line collapses to `true` under errexit; **run-set EMPTY**. This is the fold-under-all-gates the
  value-story was stranded from at the harness layer (20G §5 / 20H §3) — now a corpus case that passes
  gate-1 parity, the NEW gate-5 (omit/replace skipped), and the exec gate. The install site (site 2)
  still ships a `package` probe (it's EstablishAmbient with a probe) so it needs a `dpkg-query` shim;
  the host is consistent (nginx installed both via `dpkg -s` and `dpkg-query`).
- **parity rc rule (deliverable, `tc-probe-parity-projection` resolved-conservatively):** gate-1 (b) now
  compares a site WITH its `rc=` iff the AUTHORED fixture record carries one; rc-less records stay
  effect-only. The authored fixture is the source of truth for which sites are rc-bearing
  (`norm_parity.awk` strips rc from a record only when its site's authored record had none, applied to
  both sides). Rationale (documented in `run.sh`): a fold-valid Query/pkgstate rc IS a parity target (a
  wrong probe-emitted rc would be a wrong fold, 20E §2), but an establish site's rc is the
  probe-command's — firewalled from the fold — so it is not a target, and the fixtures historically omit
  it (no mass re-authoring). +SURE this is the right tightening: it catches a wrong Query rc without
  forcing every establish fixture to carry a firewalled rc. The corpus exercises it on
  `headline-guarded-realistic` (sites 1/2/9 rc-bearing pkgstate Query records) and `exec-shimmed-query-fold`
  (site 1 rc-bearing).

## §6 What strained (the primary deliverable)

- **strain-P1-uniform-rename-churn (the scope-vs-cleanliness tension).** "Keys per `(kind, selector)`"
  forces the wrapper name to carry the selector, which renames EVERY probe-shipping golden's wrapper
  (`package__check` → `package_installed__check`, ×~35 cases) — far beyond item-2's nominal
  service-oracle scope. I judged this IN scope for item-1 (the rename is item-1's direct, mechanical
  consequence, and the apply sections are provably untouched), not the "delta outside item-2/item-7 ⇒
  stop-and-flag" condition (which I read as a *disposition* change in an unrelated case — a bug signal).
  ~SUSPECT a reviewer could prefer a scheme that keeps `<kind>__check` for single-selector kinds and
  only disambiguates multi-selector ones — but that is stateful/non-local (the name would depend on how
  many distinct bodies the kind ships) and I rejected it as non-deterministic-per-site. Flagged for
  ratification: is the uniform `<kind>_<selector>__check` the right scheme, or should single-selector
  kinds keep the bare `<kind>__check`?

- **strain-P2-no-existing-apply-flip (the find-1 fix is invisible in existing dispositions).** The
  prompt anticipated existing multi-selector sites flipping to skip-unresolvable ⇒ run. They did NOT
  (§3): the headline cases already ran their service mutators under errexit, and exec-distinct-selectors
  had both selectors holding. So the find-1 UNDER-EXECUTE was real-and-reachable (20I §2 confirmed it at
  HEAD), but the CORPUS as authored never exhibited the dangerous host — exactly 20I find-1's "the corpus
  cannot even express the dangerous host" (gate-1 parity forced fixtures to agree with the blind
  probe). The fix's value is therefore carried by the NEW boundary case (§4), not by a flip in an old
  case. +SURE this is the honest read: the repair closes a hole the corpus structurally couldn't reach,
  and the new case is what proves the hole is closed. (This is itself a small datum about the
  parity-mediated corpus: it can hide a wrong-elision that only manifests on a host the fixtures don't
  encode — the cm-1 differential-gate's job, still owed; 20I find-6c.)

- **strain-P3-single-selector-mismatch-survives (the structural floor's blind spot).** The resolution
  rule is structural (selector COUNT), not semantic (does this body observe THIS selector). So a
  single-selector kind with the "wrong" probe body — e.g. a `service` oracle that only ever `enable`s but
  probes via `is-active` — passes the floor (one selector ⇒ kind-default permitted) and elides on the
  wrong verdict. I FIXED the one corpus instance (`two-oracles` → is-enabled) by hand, but the ENGINE
  cannot catch it (`inv-referent-agnostic`). +SURE this is correctly an oracle-quality (F-BLESSED)
  concern, not a structural one — recorded so a future reader doesn't expect `resolve_probe` to catch a
  semantic mismatch. The `R2-MULTIOP`/F-BLESSED quality-bar lines are where this lives.

- **strain-P4-roundtrip-guard-mostly-dead (recorded so it isn't re-derived as novel).** The
  `PROBE_SELECTOR_ROUNDTRIP` diagnostic can only fire from the effect-map-selector side (a selector NAME
  with a literal `_`); the funcname-parse side always round-trips. So the guard is reachable but
  exercises a case no corpus oracle hits. ~SUSPECT it is worth keeping (cheap, documents the
  inexpressibility) but it is belt-and-suspenders, not a live path. Flagged `tc-perselector-mangle`.

- **strain-P5-pipe-ran-order-flake (a latent gate-4 fragility my BLESS surfaced; NOT my change).**
  `exec-enclosing-pipe-subshell`'s book is `( apt-get install -y nginx ) | grep nginx` — a PIPE, whose
  two stages run CONCURRENTLY, so the `ran:` log order is race-dependent. gate-4 (20F §5) made `.ran`
  order-sensitive ("book order is sacred"), but a pipe has no deterministic stage order. A global `BLESS`
  re-ran the case and captured the rare race-flipped order (`grep` before `apt-get install`); the
  verifying run got the stable-majority order and FAILED on the diff. I RESTORED the original `.ran`
  (5 consecutive clean runs confirm `apt-get install` then `grep` is the stable order) and did NOT
  re-bless it. +SURE this is pre-existing (the pipe predates task-P) and orthogonal to the per-selector
  work; it is a real gate-4-vs-pipe-concurrency tension. ~SUSPECT the right long-run fix is to compare a
  PIPE leaf's `.ran` order-insensitively (a pipe is the one place "book order is sacred" cannot hold at
  runtime), but that is a harness-design call (gate-4's scope), flagged not fixed. **Discipline note for
  future re-blessers: never `BLESS` while the pipe case is in the suite without checking its `.ran`
  didn't flip.**

- **strain-P6-conflict-floor-is-defensive (item-5 has no live trigger).** The same-cell conflict floor
  (`merge_observable`) is sound and tested, but in practice only one site per cell is resolvable (a
  same-command re-establish is `EstablishWritten` ⇒ unresolvable ⇒ absent from `checks`,
  strain-D1-samecell). So `facts_from_sites` rarely sees two checks on one cell. +SURE the floor is still
  correct-to-have (a forged/flaky host, or a future classification that yields two resolvable same-cell
  sites, would hit it, and last-write-wins would be a silent wrong-elision); the unit tests construct the
  conflict directly (anti-masking). Recorded as defensive-not-live.

## §7 Exclusion-check (the four-by-two discipline, AGENTS.md)

- **other phase:** the per-selector resolution is phase-agnostic (it picks a probe BODY; the probe is the
  read-only `Phase::Probe` artifact, the apply consumes its records). `resolve_probe` returning `None`
  for a multi-selector-without-per-selector site is the `can't-probe ⇒ can't-elide` link — the apply
  RUNS (`kFAIL-perform`), the SOUND direction in BOTH phases. The new boundary case exercises the
  probe-emit AND the apply-elide halves.
- **other user:** the F-BLESSED floor reads clearer to BOTH users — the admin sees a service mutator RUN
  (not silently elided on the wrong probe), and the engineer authoring an oracle gets a structural nudge
  (`R2-MULTIOP` + the per-selector requirement) toward the honest two-probe shape. The
  `skip-unresolvable` comment names WHY (no per-selector probe for a multi-selector kind).
- **other reliability:** an unreliable host that forges a same-cell disagreement hits the conflict floor
  (⊤ ⇒ run); an unreliable per-selector probe (one selector flaky) degrades only ITS cell, not the
  kind's other selectors (the per-cell keying is the isolation).
- **reverse propagation:** N/A to the lift/probe-resolution (forward-fact shapes); the resolution rule
  consumes the effect-map's declared selectors as data.
- **the killer cross-cell:** the BLESS-flips-the-pipe-`.ran` trap (strain-P5) IS an exclusion-check catch
  — a global re-bless (the mechanism item-1/item-2 forces) silently regenerated a racy golden; caught by
  the verifying run and the "is this delta in scope" check (the `.ran` flip was outside item-1's
  probe-section scope ⇒ stop-and-investigate ⇒ restore).

## §8 tc-* / judgment calls flagged (conservative defaults; flagged up, not settled)

- **tc-perselector-wrapper-scheme (strain-P1):** uniform `<kind>_<selector>__check` (chosen) vs.
  bare-`<kind>__check`-for-single-selector. Conservative: uniform (deterministic, local); the golden
  churn is probe-section-only + apply-byte-identical. Flagged.
- **tc-perselector-mangle (strain-P4):** the round-trip guard for a literal-`_` selector name — kept as a
  reachable-but-corpus-dead WARNING. Flagged.
- **tc-probe-parity-projection (item-4, RESOLVED here):** the 20F-flagged effect-only parity is tightened
  to per-site rc (rc-bearing fixtures compared with rc). This was flagged for "a future D3b" in 20F §6;
  task-P is that follow-through. Recorded as resolved, not re-flagged — but a reviewer should confirm the
  authored-fixture-is-rc-truth design (vs. a per-site-class derivation) is the right seam.
- **tc-pipe-ran-order (strain-P5, NEW):** gate-4's order-sensitivity vs. pipe-stage concurrency. The
  pipe case's `.ran` is race-flippable; I restored the stable order and flag the harness-design question
  (compare pipe leaves order-insensitively?) for the human. NOT fixed (gate-4 scope).
- **tc-query-bare-elision (UNCHANGED, NOT touched — per the brief):** 20E's flagged behavior (a valid
  Query with entirely-unconsumed observables is still substituted) is left exactly as-is. task-P did not
  touch it. Still owed a human ruling on aggressiveness.

## §9 Confidence summary

- +SURE: the resolution rule is the sound F-BLESSED floor (per-selector-else-kind-default-iff-single);
  item-1 is provably probe-section-only (apply byte-identical across the corpus, three independent
  checks); the new boundary case's run-set is hand-derived and harness-confirmed; the shimmed-Query-fold
  folds under all gates incl. enforced parity; the conflict floor + multi-operand hazard are pinned.
- +SURE: all gates green (fmt/clippy-D-warnings-no-new-expects/test/e2e-52-zero-xfail/typos).
- ~SUSPECT: the uniform wrapper-name scheme (strain-P1) is the right call but wants ratification; the
  pipe-`.ran` flake (strain-P5) is a real latent gate-4 fragility I surfaced but did not fix.
- +SURE: the find-1 under-execute is closed structurally (multi-selector-without-per-selector ⇒
  un-probeable ⇒ run), and its value is carried by the new boundary case because the parity-mediated
  corpus could not express the dangerous host in an existing case (strain-P2 — itself a small datum for
  the still-owed cm-1 differential gate).
