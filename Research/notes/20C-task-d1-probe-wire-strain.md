# 20C — task-D1 (the WIRE): probe becomes a real, runnable, self-reporting, site-keyed artifact

> Round-20 spike note, append-only. Records task-D1 (the WIRE, 205 §6 / 20B §3): the
> probe-projection is re-keyed from per-FACT to per-SITE (`inv-site-keyed-results`), the
> rendered probe is now SELF-REPORTING (emits a results-record per site when run), and
> the cli/hostsim probe-answer plumbing re-keys site→cell. Explicitly NOT in this slice:
> the `Query` effect-class + rule-query-validity (D2); new harness gates
> (probe-exec-under-mocks, argv-echo differential — D3); new e2e case authoring beyond
> migrating the existing 44. AI-authored, confidence-marked. Trust R/D/I/K + 19H/19I +
> the human rulings over this.

## §0 What landed

- `plan::compile_probe` re-keyed: signature `(ast, cfg, classes, probe_body)` (was
  `(classes, probe_body)`). Output `ProbePlan { checks: Vec<ProbeCheck>, unresolvable:
  Vec<LeafId> }`; `ProbeCheck` gained `site: LeafId`. Per RESOLVABLE site (a site whose
  class is `EstablishAmbient` AND whose kind has a declared `oracle_probe_*`) one check;
  every other site recorded `unresolvable`. **The per-fact dedup is gone** — two
  same-command sites stay distinct (`inv-site-keyed-results`).
- `ProbePlan::render_sh` emits a self-reporting artifact: per resolvable site, the
  kind's `oracle_probe_*` wrapper (defined once per kind), invoked per-site with the
  resolved entity F-QUOTE-bound, followed by a record-emitter that captures `$?`, maps it
  to the three-outcome word, and `printf`s `site <id> effect=<W> rc=<n>`. Un-resolvable
  sites render as `# site:<id> skip-unresolvable` comments (never invoked).
- `site_order(ast, cfg, classes)` factored out (new private fn): the canonical
  span-sorted `LeafId` assignment shared by `compile_probe` and `build_plan`, so the
  probe's site-ids == the apply plan's leaf-ids (one id space; a record `site N` keys
  back to plan leaf N). Pinned by `probe_site_id_equals_plan_leaf_id`.
- `cli`: `parse_results` consumes the site-keyed records into `SiteResults { verdict:
  BTreeMap<LeafId,Verdict>, declared_rc: BTreeMap<LeafId,Rc> }`; `facts_from_sites`
  re-keys site→fact (via `probe.checks`) into the `FactKey→Observable` map `build_plan`
  consumes. The fact-keyed fold/classify machinery is UNTOUCHED — only the probe-answer
  plumbing re-keys.
- `hostsim`: the apply-2 DST test demonstrates the site→cell bridge explicitly (map each
  probe site to its fact, ask the cell-keyed fact-store, re-key to the fact-map
  `build_plan` wants). The fact-store stays cell-keyed; the kFAIL-withhold monitor is
  untouched.
- corpus: all 44 `probe-results.txt` re-keyed (mechanical, off the live `# site N:
  <label>` map); all `expected.out` re-blessed. **Zero apply-disposition deltas** (§3).

## §1 The record grammar chosen (deliverable, documented in the artifact header)

```
site <leafid> effect=<holds|absent|cant-tell> rc=<n>      # the main per-site record
declared-rc <leafid> rc=<n>                               # TRANSITIONAL fold lane (legacy)
```

- `effect` is the fact-probe's three-outcome (`an-probe-shape`) derived FROM the probe
  command's rc by the oracle's existing convention: `0 ⇒ holds`, `1 ⇒ absent`, else
  `cant-tell`. The wrapper does this mapping in sh (`if [ $_rc -eq 0 ]…`). +SURE this is
  the existing convention — verified against the corpus probes (`package.oracle.sh`'s
  `oracle_probe_package` returns 0/1/2 with exactly these meanings; the doc-comment on
  `oracle::FactProbe` states it). No divergence found across the corpus probes.
- `rc` is the RAW probe-command status, carried on the wire for provenance. The cli
  PARSES it (grammar-validity) but feeds it to NOTHING (the wrong-concrete firewall, §2).
- Records go to the probe's stdout (the round-trip's return channel). No exit-code
  semantics for Dorc verdicts (rc opaque — standing human ruling); the record IS the
  out-of-band lane.
- Why a separate `declared-rc` line rather than reusing the `site` record's `rc`: see §2
  — they are DIFFERENT observables, and conflating them is the disaster class.

The grammar is minimal and line-oriented (disposable, per 202 §3 "the exact line-grammar
is disposable"). `parse_results` is liberal: unrecognized leading token, non-numeric
site-id, missing `effect=`, non-int rc — all dropped (⇒ that site folds to Unknown ⇒
run, the kFAIL-perform floor). `garbage-stdin` now pins exactly these new-grammar failure
modes.

## §2 The wrong-concrete firewall — the load-bearing correctness boundary

The prompt's CRITICAL WRONG-CONCRETE TRAP, made concrete in code: a `site` record's `rc`
is the CHECK/PROBE command's rc (`dpkg-query`'s), NOT the book command's (`apt-get`'s).
For an establish-class site these are DIFFERENT observables — feeding the probe-rc into
the fold as the site's status would be a confidently-wrong concrete (the disaster class).

In D1 the parsed `site`-record rc is consumed by NOTHING:
- `parse_results`'s `site` arm reads only `effect=` into `verdict`; it does not read the
  trailing `rc=` at all.
- `facts_from_sites` sets `Observable.status` ONLY from the `declared_rc` lane; the
  Effect channel (`Observable.effect`) comes from the site's verdict. So the
  self-reported probe rc never reaches the fold.
- The ONE legacy exception (`fold-oror-guard-omits`) folds a probe-sourced rc through the
  TRANSITIONAL `declared-rc <site> rc=N` line, consumed exactly as today's `AndOrStatus`
  relaxation. **Proven load-bearing** (traced, not assumed): with `declared-rc 0 rc=0`
  the guard's known rc folds the `|| install` branch dead ⇒ the install elides to `true`;
  WITHOUT the line, the undeclared rc is ⊤ ⇒ no fold ⇒ the guard runs and the line stays
  verbatim. The `site 0 effect=holds` alone does NOT fold it. Not widened.

D2 (the Query class) is what will legitimately equate a guard's probe-rc with its site
status (`command -v` is a read-only Query; its check IS the guard, and rule-query-validity
gates when the probe-time rc is fold-valid). Until then, the firewall keeps the establish
sites' probe-rc out of the fold entirely.

## §3 GOLDEN DISCIPLINE — every disposition delta (expected zero; got zero)

Before blessing, I diffed the APPLY section (the 2nd `#!/bin/sh` block) of the current
output against every committed `expected.out` (43 non-xfail cases). **Zero apply-section
differences.** The re-key touched ONLY the probe section; no elision/run/omit disposition
changed, and no site-id leaks into an apply section (the apply renders the book
verbatim/elided, never referencing a site-id). The blessed diffs are exactly: probe
section replaced (per-fact → per-site self-reporting), apply section byte-identical.

This is the 19I §3 trap's guard: a case must not pass because the re-key happened to
re-derive the same answer for the wrong reason. It didn't — the dispositions are computed
by the UNCHANGED fact-keyed fold/classify; only the probe-answer transport changed, and
the site→fact re-key reproduces the same fact-keyed observations the old fact-keyed lane
produced.

## §4 The emitted-function shape chosen (and why)

**The SIMPLER sanctioned shape (205 §1 fallback / st-2 ruling, 20B §3): one
`<kind>__check()` wrapper per kind from the kind's real `oracle_probe_*` body, invoked
per-site with the resolved entity bound.** NOT the check's argparse skeleton.

This is what shipped pre-D1 already (`compile_probe` pulled `idx.probe_for(kind).body`),
so D1 preserved it and re-keyed the INVOCATIONS to per-site. Why this shape over the
argparse-skeleton-with-declared-probe shape the prompt described first:
- **st-2 compliance falls out for free.** The check bodies' PLACEHOLDER probe commands
  (`pkgindex`'s tautological `test -n fresh`; the simplified `dpkg-query -W "$pkg"` in the
  per-case apt checks) must NOT ship (20B §3). Because the shipped wrapper comes from
  `oracle_probe_*` — NOT from the `<provider>__check` argparse — the placeholders never
  reach the artifact. The `<provider>__check` functions remain the engine's entity-
  resolver only (`analysis::effect::command_effect` reads them; they never ship).
- It is the minimal diff over the working spike-2 render, and the prompt explicitly
  sanctioned it: "If reconciling argparse-skeleton + declared-probe-body is awkward, the
  SIMPLER sanctioned shape is: emit one wrapper function per (kind, selector) from
  `oracle_probe_*` … invoke it per-SITE …".

~SUSPECT cost of this shape (the per-selector limitation, strain-D1-perselector below):
the wrapper is per-KIND, not per-(kind, selector). So `service:nginx#enabled` (site 5)
and `service:nginx#active` (site 6) BOTH invoke `service__check 'nginx'` — the SAME probe
body (`systemctl is-active --quiet`), which only observes `#active`. The `#enabled` site
gets the wrong probe. This is a pre-existing oracle/probe-model limitation (the probe is
keyed by kind, the cell by selector), NOT introduced by D1, but D1's per-site invocation
makes it visible in the artifact (two identical `service__check 'nginx'` invocations
reporting against different sites). Recorded as strain-D1-perselector; the real fix is a
per-selector probe (`an-per-entity-selector` carried into the probe key), deferred.

## §5 The site↔cell bridge design

- **Site identity** = the stable `LeafId`, assigned by `site_order` (span-sort the
  classify leaves, enumerate). `build_plan` assigns the SAME ids by the same span-sort
  over the same `classes` slice — so probe site-id ≡ plan leaf-id by construction. I
  factored `site_order` out and added a doc-note tying `build_plan`'s assignment to it
  (they MUST stay byte-identical); `probe_site_id_equals_plan_leaf_id` pins the
  equivalence so a future edit that drifts one is caught. ~SUSPECT a cleaner long-run
  shape is `build_plan` literally calling `site_order` (today it duplicates the sort);
  left as-is to keep the D1 diff to the probe, flagged.
- **The bridge** (site → cell): the probe's `checks` each carry `(site, fact)` — that IS
  the bridge. The cli's `facts_from_sites` walks `probe.checks`, looks up each site's
  reported verdict + declared-rc, and keys the resulting `Observable` by the site's
  `fact`. The hostsim DST test does the same (site → fact → `host.observe(fact)`). The
  fact-store / fold / classify all stay cell-keyed; only this one map re-keys.
- **Two sites, one cell** (`inv-site-keyed-results`): two same-command sites on the same
  cell write the same `fact` into `by_fact`; last-in-site-order wins. +SURE this is sound
  for D1: they are the same cell on the same host, so they report the same verdict. (And
  in practice the second same-cell site is `EstablishWritten` ⇒ unresolvable ⇒ not in
  `checks` at all — see strain-D1-samecell.)

## §6 What strained (the primary deliverable)

- **strain-D1-samecell — two same-command sites are NOT two resolvable sites; the second
  is `EstablishWritten`.** My first `two_same_command_sites_stay_distinct_sites` test
  asserted two IDENTICAL `apt-get install -y nginx` lines yield TWO checks. It yielded
  ONE: the second install sees the first establish `package:nginx#installed` upstream ⇒
  the ambient gate classifies it `EstablishWritten` ⇒ unresolvable. This is CORRECT (the
  resting probe is stale after the first install), and it is the *right* demonstration of
  `inv-site-keyed-results`: the sites are NOT collapsed (site 0 resolvable + a DISTINCT
  site 1 recorded unresolvable), they just have different classes. +SURE after tracing;
  the test premise was wrong, not the engine. (The clean two-distinct-CELL case — nginx +
  curl — does yield two resolvable sites, pinned separately.) Datum: "two same-command
  sites stay distinct" is true at the SITE layer, but same-cell re-establish is Written,
  so only one is ever probe-resolvable. The split the prompt worried about
  (`compile_probe`'s old per-fact dedup collapsing them) was real but is now moot — the
  ambient gate already separates them by class before dedup could even apply.

- **strain-D1-perselector — the probe wrapper is per-KIND, not per-(kind, selector); a
  multi-selector kind ships ONE probe body for all its selectors.** (§4.) The headline's
  `service:nginx#enabled` and `#active` both invoke `service__check 'nginx'` →
  `systemctl is-active`, so the `#enabled` site is probed by the wrong (active-only) check.
  Harmless in the corpus (the headline host has both holding, and the e2e doesn't execute
  the probe against a real systemd), but it is a genuine probe-model gap the per-site
  rendering surfaces. The `oracle_probe_*` declaration is keyed by kind (`oracle::lift`'s
  `probe_bodies` map is `kind → body`), and the per-selector probe (e.g. `is-enabled` vs
  `is-active`) has nowhere to live. Deferred-not-irrelevant: it returns the moment a kind
  has selectors with genuinely different read-only checks (`service` is exactly that, per
  `F-BLESSED`'s "an honest service probe is TWO commands"). +SURE this is the same gap
  `oracle/CLAUDE.md` F-BLESSED already names; D1 didn't create it, it made it legible.

- **strain-D1-recordgrammar-rc-omitted — the corpus stand-in omits the `rc=` the real
  probe emits.** The artifact header documents `site <id> effect=W rc=<n>`, and the
  rendered probe DOES emit the rc (verified under mocks: `site 4 effect=absent rc=1`). But
  the migrated `probe-results.txt` stand-ins carry only `site <id> effect=W` (no rc) —
  because the cli discards a `site` record's rc (the firewall, §2), so adding it would be
  redundant data with zero behavioral effect, ×~30 files. The parser tolerates both forms.
  ~SUSPECT this is the right D1/D3 boundary: D3's probe-exec-under-mocks gate will run the
  REAL probe and produce records WITH rc, and the parser already accepts them. But a
  reviewer may find the header-vs-stand-in mismatch jarring; flagged as a tc-shaped call I
  resolved conservatively (omit the redundant field, document the tolerance). If D3 wants
  the stand-ins byte-identical to the emission, it adds the rc then.

- **strain-D1-siteorder-duplication — `build_plan` and `site_order` independently span-sort
  the same classes.** They MUST produce identical `LeafId`s or the wire breaks
  (`inv-site-keyed-results`). Today that holds because the sort key + enumerate are
  byte-identical in both, and a test pins it — but it is a latent footgun (an edit to one
  sort that misses the other silently wrong-keys every record). ~SUSPECT the clean fix is
  `build_plan` calling `site_order` for its leaf-id assignment; I left them separate to
  keep the D1 diff scoped to the probe path and the apply-render untouched. Flagged for a
  follow-up (cheap, but it touches `build_plan`'s hot path — out of D1's "the WIRE" remit).

- **strain-D1-quoting-win (a WIRE improvement, not a strain — recorded so it isn't
  re-derived as novel).** The site-keyed grammar SOLVES the "stdin re-key gotcha"
  (`cli/CLAUDE.md`): the OLD fact-keyed stdin used whitespace-split, so a spaced operand's
  `fact_label` (`package:web (proxy)#installed`) could not be keyed — `probe-operand-
  quoting`'s comment documented exactly this, forcing both installs to Unknown ⇒ run.
  Site-ids are integers, so the new grammar keys ANY operand (spaced, metachar) trivially.
  The case still asserts both installs RUN (I left the stand-in with no records — a
  deliberate choice to preserve the case's render-under-quoting purpose; updated its
  comment to note the grammar now COULD key them but deliberately doesn't). +SURE this is
  a real ergonomic win the re-key buys; the operand still F-QUOTE-binds correctly in the
  INVOCATION (verified: `package__check 'web (proxy)'`, `package__check 'x; touch …'`),
  which was already the spike-2 behavior — only the RESULTS lane improved.

- **strain-D1-unresolvable-leak (considered, ruled benign).** Un-resolvable sites render
  as `# site:<id> skip-unresolvable` comments carrying the site-id. Exclusion-checked
  against the apply: site-ids appear ONLY in the probe section (comments + record printfs),
  never in the apply section (verified: zero apply-section site-id leaks across 43 cases).
  So a site-id is never executed and never round-trips into a Dorc verdict. The
  skip-unresolvable comment is pure transparency (the human reading the artifact, and the
  D3 argv-echo differential). +SURE benign.

## §7 What D2 (the Query class) needs to know

- **The transitional `declared-rc <site> rc=N` lane is the seam D2 replaces.** D2's Query
  effect-class makes `command -v`/`dpkg -s`/`getent` first-class read-only guards whose
  check() IS the probe, and whose probed rc legitimately becomes the site's Status channel
  (subject to rule-query-validity's pristine-prefix gate, 205 §2). At that point the
  `declared-rc` line dies: a Query site's `site <id> effect=W rc=N` record's rc BECOMES
  fold-valid (it is the guard's own rc, not a mutator's check-rc), so the firewall (§2)
  relaxes FOR QUERY SITES ONLY. D2 must NOT relax it for establish sites — that is still
  the wrong-concrete (an establish site's record-rc is the dpkg-query rc, not apt-get's).
  The cli's `facts_from_sites` is where the relax lands: today `status` comes only from
  `declared_rc`; D2 will additionally take a Query site's record-rc into `status`,
  discriminating Query-vs-establish from the site's class/effect-class.
- **`fold-oror-guard-omits` is the case that flips.** Its guard `command -v nginx` is
  modeled today as an `EstablishAmbient` (`tool.oracle.sh`: `oracle_effect command ''
  establish present`), so it rides the establish path + the `declared-rc` lane. When D2
  reclassifies it as a Query, its `site 0 effect=holds rc=0` record carries everything the
  fold needs and the separate `declared-rc 0 rc=0` line becomes redundant ⇒ removable. D2
  should re-bless this case's `probe-results.txt` to drop the `declared-rc` line and prove
  the fold still works off the Query record's rc.
- **rule-query-validity (205 §2, the pristine-prefix rule) gates the rc, not the record.**
  The record always emits (the probe always reports what it observed); whether the rc is
  fold-VALID depends on whether an effect-bearing command reaches the Query site from
  entry. That gate lives in the engine (a reaching-defs bit), not the wire — the wire
  faithfully carries the observed rc regardless. D2 adds the gate; D1's wire is already
  shaped to carry the rc it will need.
- **Site identity is stable and shared.** D2 keys nothing new — it reads the same
  site-keyed records, on the same `LeafId` space, through the same `facts_from_sites`
  re-key. The only change is WHICH channel a Query site's record-rc feeds.

## §8 Gate status

All green (from `spike/`): `cargo fmt --check` clean; `cargo clippy --workspace
--all-targets -D warnings` clean (no new expects); `cargo test --workspace` 233 tests pass
(1 pre-existing ignore — the HOLE#1 subst-in-redir spec); `sh e2e/run.sh` 44/44 incl. the
standing render xfail and the crash-guard; `mise x -- typos spike` clean. Blessed under
the crash-guard intact (BLESS only after `cargo test` green, per the gate discipline).

## §9 tc-* / judgment calls flagged (not settled in-component)

- **tc-recordgrammar-rc-in-standin** (strain-D1-recordgrammar-rc-omitted): omit the
  redundant `rc=` from the corpus stand-ins (chosen) vs. carry it to match the emission.
  Conservative: omit + document parser tolerance; D3 decides if its gate wants parity.
- **tc-siteorder-share** (strain-D1-siteorder-duplication): `build_plan` should call
  `site_order` vs. keep the duplicated-but-pinned sort. Conservative: keep duplicated +
  pin equivalence; defer the `build_plan` refactor (touches the apply hot path, out of the
  WIRE remit).
- **tc-perselector-probe** (strain-D1-perselector): the per-kind probe wrapper under-serves
  multi-selector kinds. Not resolved (it is the deferred `an-per-entity-selector`-into-probe
  work); flagged so D2/later doesn't mistake the single shared `service__check` for a bug
  in the WIRE.
