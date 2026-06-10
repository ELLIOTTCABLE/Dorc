# 20G — task-D3b (corpus + render): the parity re-grounding, the realistic book, the leaf-exact render fix, and the gate-1/gate-5 tension that reshaped the value-story

> Round-20 spike note, append-only. Records task-D3b (the four work-items handed by the
> orchestrator, building on 20F's harness gates): probe shims that flip the gate-1 opt-outs
> (item-1), the group-J realistic guarded book (item-2), three SAFE-degrade boundary strawmen
> (item-3), and the T14 leaf-exact case-arm render fix (item-4). AI-authored, confidence-marked.
> Trust R/D/I/K + 19H/19I + the human rulings over this. Builds on 20F (the five gates this
> consumes), 20B §2/§3 (the charter — the headline-vacuity finding + the floor strawmen), 20E
> (the Query class semantics), the plan/CLAUDE.md render section. The headline finding is
> **strain-D3b-fold-vs-gate5** (§5): a SHIMMED Query-guard fold is structurally incompatible
> with gate-5, which reshaped item-2 away from the prompt's literal "Query folds above".

## §0 What landed (all green: cargo fmt/clippy -D warnings/test 64+pass-0-fail-1-ignore;
## `sh e2e/run.sh` 50/50 with ZERO xfail remaining; `typos spike` clean)

- **item-1**: 15 inert probe shims (`dpkg-query` ×13 + `dpkg-query`-variant + `getent`) authored
  into the relevant cases' `mocks/`; 17 `PROBE_RESULTS=authored` markers DELETED (parity now
  enforces); 9 markers KEPT with rewritten reasons. The honest split (§1).
- **item-2**: ONE new e2e case `headline-guarded-realistic` (a scrappy `set -e` book, 5 oracle
  kinds incl. a NEW `dpkg -s` `query` oracle) — the group-J restoration. gate-1 parity ENFORCES
  across an 11-site, 5-kind probe mix (the corpus's largest). Convergence-sensitivity demonstrated
  (§3). The Query value-story it CAN tell under enforced parity + gate-5 is narrower than the
  prompt envisioned — see §5.
- **item-3**: three SAFE-degrade strawmen — `loop-degrades-safely` (⊤-reject ⇒ poison+havoc),
  `sourcing-degrades-safely` (Opaque source ⇒ downstream unresolvable), `partial-top-argv-runs`
  (⊤ operand ⇒ no resolvable site). Each with mocks + ordered expected.ran (§4).
- **item-4**: `render_apply` leaf-exact case-arm fix (T14) — the standing xfail
  `render-case-arm-oneliner-wrong` converted to a PASSing `render-case-arm-oneliner` with authored
  goldens. Zero golden churn elsewhere (§6). The if-guard render floor STAYS untouched (§6, flagged).

## §1 The enforced/opted-out parity split — the convergence-axis re-grounding number

**38 mocks cases · 29 enforce gate-1(b) parity + (c) vouch-closure · 9 opt out.** (20F left it at
33 mocks · 7 enforce · 26 opt out; D3b's shims flipped 17 opt-outs to enforced, and the 5 new
cases added 5 enforced — net 29 enforced.) The 9 surviving opt-outs, by the ONLY three reasons
parity is genuinely impossible:

- **builtin `command -v` (4)**: `fold-oror-guard-omits`, `guard-status-blocks-elision`,
  `exec-query-guard-composition`, `exec-query-after-mutator-runs`. `command` is a shell BUILTIN —
  PATH=mocks-only cannot shim it, so under mocks it resolves the operand absent-from-PATH (rc 1),
  never the authored real-host `holds`. +SURE this is permanent, not a deferral. (D3b DID ship a
  `dpkg-query` shim for these cases' install establish-probe, but the load-bearing site is the
  builtin Query.)
- **timing-dependent `find` (4)**: `exec-singleton-update`, `exec-poison-wall-dead`,
  `headline-partial`, `headline-pi-webhost`. The pkgindex probe is
  `test -n "$(find … -newermt '-1 hour')"` — a genuine FILESYSTEM-TIMING observation. A `find`
  PATH-shim forcing a non-empty line WOULD make parity enforce, but that fabricates "there is a
  fresh apt list" to match a fixture authored for a REAL host — the all-exit-0 masking
  `inv-probe-sourced-values` forbids. So they stay opted-out under anti-masking, not under "shim
  not yet written". (tc-find-shim, §7.)
- **deliberately-empty fixture (1)**: `probe-operand-quoting` — a QUOTING test (F-QUOTE) whose
  probe-results authors ZERO convergence records on purpose (both installs run regardless). The
  probe emits 2 records, so any shim diverges from the intentionally-empty fixture ⇒ parity can't
  enforce without corrupting the test's purpose. A third opt-out reason beyond builtin/find.

Each kept marker now carries a one-line reason naming WHICH of these three applies (the
admin/engineer reading a FAIL sees the remedy or the permanence). gate-1(a) site-completeness
still runs on ALL 38.

**Anti-masking discipline held**: shims were authored to match the EXISTING fixture answers (the
fixture is the spec); NO probe-results were re-blessed to match shim output. Demonstrated by
construction — e.g. `exec-diverged`'s `dpkg-query` shim exits 1 (the fixture says `absent`), not 0.
Verified the flip ENFORCES (not vacuous): corrupting `exec-converged`'s fixture `holds`→`absent` in
a temp copy ⇒ gate-1 parity FAILs loudly ("do NOT silently re-bless"); repo fixture restored intact.

## §2 item-2 — the realistic guarded book (`headline-guarded-realistic`)

A scrappy `set -e` pi-webhost book (12 command lines): a bare pre-flight Query (`dpkg -s
ca-certificates`), a `dpkg -s nginx || apt-get install -y nginx` idempotency guard, `apt-get
update` (Singleton), converged `curl` + diverged `htop` installs, `systemctl enable`+`start`
(distinct `#enabled`/`#active` selectors), a `dpkg -s vim || …` guard BELOW the mutators, and `ufw
allow 80/tcp`. Five oracle kinds (package, service, firewall, pkgindex, + a NEW `pkgstate` `query`
oracle on the EXTERNAL `dpkg -s` command — chosen precisely because it IS shimmable, unlike the
builtin `command -v`, so its probe enforces gate-1 parity).

What the run-set diff PROVES (hand-derived before bless, verified under mocks):
- **The bare valid Query (`dpkg -s ca-certificates`, holds) IS value-substituted to `true`** — the
  one VISIBLE Query elision in the apply render. ABSENT from the apply run-set; PRESENT in the bare
  book's (gate-5 satisfied — §5).
- **C-3 reality, demonstrated across 5 mutator kinds**: `apt-get update` (converged pkgindex),
  `apt-get install curl` (converged), `systemctl enable/start` (converged service), `ufw allow`
  (converged firewall) ALL RUN under `set -e` — their rc is ⊤ (`fork-mutator-rc`), errexit consumes
  status, the `AndOrStatus` floor blocks the license. The run-set has every one. This is the 206 §2
  headline cost at realistic scale.
- **Diverged install runs**: `htop` (absent) runs.
- **The two `|| install` guards run VERBATIM** (their `dpkg -s` calls ARE in the run-set), gating
  their installs at RUNTIME — the SAFE kFAIL-perform deferral (dorc could not statically fold them;
  the real guard decides at apply-time).

5 oracle kinds × 11 probe sites all reproduce the fixture under shims (gate-1 ENFORCES): pkgstate
(dpkg -s, branched), package (dpkg-query, branched), pkgindex (find — see §7, a deliberate authored
fresh-host find-shim), service (systemctl is-active), firewall (ufw status piped to a minimal
literal-match grep shim). +SURE this is the corpus's broadest enforced-parity probe.

## §3 The convergence-sensitivity demonstration (item-2 requirement)

Flipping ONE fixture fact in a temp copy (`headline-guarded-realistic`'s `site 2`
pkgstate:nginx `absent`→`holds`, shim kept consistent) ⇒ the `dpkg -s nginx || apt-get install -y
nginx` line FOLDS: the valid holds-Query proves the `||` install dead, the whole line collapses to
`true`. **Run-set diff (baseline → flipped): loses BOTH `ran: dpkg -s nginx` AND `ran: apt-get
install -y nginx`.** The case is genuinely convergence-sensitive — its run-set is a function of the
host facts, not hard-coded. (+SURE, run from a temp copy; repo unperturbed.) NB the flipped temp
copy then trips gate-5 on the omitted nginx install — which IS strain-D3b-fold-vs-gate5 (§5)
manifesting; the run-set CHANGE is the demonstration and is unambiguous.

## §4 item-3 — the three SAFE-degrade boundary strawmen (the 19I-floor visibility, exec-gated)

Each pins a SAFE degrade the corpus was blind to, and — unlike the prior analysis-only
`toprejected`/`background-amp-runs` — ships mocks + an ORDERED expected.ran so the EXEC gate proves
everything-at-or-below actually runs (not just that a diagnostic fires):

- **`loop-degrades-safely`** — `for x in a b; do …; done` mid-stream ⊤-rejects (loop +
  cfg-top-node diagnostics, declared in expected-diagnostics). The install ABOVE has the ⊤ loop as
  its CFG successor ⇒ ⊤-containment refuses elision even though it probes `holds` (the bait); the
  install BELOW is poisoned to UNRESOLVABLE by the havoc. Run-set: nginx + the loop body (`a`,`b`) +
  curl ALL run. (A subtle find: the above-install IS a resolvable probe site — site 0 emits a
  record — but is un-elidable by ⊤-containment. Probe-resolvable ≠ apply-elidable.)
- **`sourcing-degrades-safely`** — `. helper.sh` (a LITERAL-target source, KEPT not ⊤-rejected, but
  UNMODELED ⇒ Opaque) poisons downstream ambient-ness: the nginx install below it is UNRESOLVABLE
  (the probe emits NO record — empty fixture, parity vacuous) and runs verbatim despite a would-be
  `holds`. Run-set: nginx runs. (Harness note: the helper ships in `mocks/` and is sourced no-slash
  `. helper.sh` so dash finds it via PATH at exec time — the gate-2 sandbox cwd is a throwaway
  mktemp, so a `./helper.sh` relative source would not resolve. tc-source-path, §7.)
- **`partial-top-argv-runs`** — `apt-get install -y "$UNSET_VAR"` has a ⊤ operand ⇒ it is not even
  a resolvable probe SITE (a `# site:1 skip-unresolvable` comment, no record), so NO convergence
  fact can ever license eliding it (the "bait" is structurally impossible to attach). It runs
  verbatim. The contrast: the FIRST install (`nginx`, fully resolved + converged) DOES elide to
  `true`. Run-set: the empty-operand install runs; nginx elided.

+SURE all three are exec-gated SAFE degrades; the run-set is the real assertion, the diagnostics
(where present) are gate-3-declared.

## §5 strain-D3b-fold-vs-gate5 (the headline finding — a real cross-component tension)

**A SHIMMED Query-guard FOLD is structurally incompatible with gate-5 (the argv-echo differential,
20F).** Traced end-to-end, +SURE:

- gate-5 asserts ONE-DIRECTIONALLY that every engine-resolved + shimmed site's argv appears in the
  BARE book's executed-argv log (engine-resolved-and-shimmed ⊆ bare-logged).
- A Query guard folds its `|| install` only when the guard reports **holds** (rc 0). For gate-1
  parity, the SHIMMED guard (`dpkg -s X`) must then exit 0. But the SAME shim runs in the bare book,
  so the bare `dpkg -s X` ALSO exits 0 ⇒ the `||` short-circuits ⇒ the install is NOT in the bare
  log ⇒ gate-5 FAILS on the (engine-resolved, omitted) install.
- The existing `fold-oror-guard-omits` escapes this ONLY because its guard is the BUILTIN
  `command -v` — un-shimmable, so it FAILS in the bare book (mocks-only PATH, no `nginx` executable)
  ⇒ the install runs in the bare book ⇒ gate-5 passes. That builtin-ness is EXACTLY why it
  gate-1-opts-out. So: **the Query fold/omit demonstration is the exclusive province of builtin
  guards (gate-1-opted-out); a gate-1-ENFORCING shimmed guard can never demonstrate the omit under
  gate-5.** Confirmed by direct experiment: a holds shimmed guard ⇒ gate-5 FAIL printing the omitted
  install's argv; flipping to absent ⇒ verbatim (no fold), gate-5 passes.
- Corollary observed: a valid Query guard reporting **absent** (rc 1) renders VERBATIM (not
  substituted to `false`) — even the bare unconsumed case substitutes only on a usable rc that the
  consumer reads; an absent `||`-guard's branch is live, so the engine keeps it verbatim. So under
  enforced parity the valid-vs-invalid SUBSTITUTION distinction is ALSO invisible in the `||` idiom
  (both verbatim when absent).

Consequence for item-2: I could not author the prompt's literal "Query folds above (install
omitted)" under the mandatory "all gates green" + "gate-1 ENFORCES" constraints. The realistic book
instead demonstrates (a) a BARE valid Query substituted to `true` (the one gate-5-clean Query
elision — the bare guard runs in the bare book, so gate-5's ⊆ holds while the apply substitutes
it), (b) C-3 mutator-runs across 5 kinds, (c) diverged-runs, (d) `||`-guards deferred-to-runtime.
The omit/fold value-story stays demonstrated by `fold-oror-guard-omits` (builtin, opted-out) + the
plan-unit `observable_matrix.rs` Query tests (20E §2). ~SUSPECT the clean long-run fix is a gate-5
refinement: skip an engine site whose disposition is `Omit`/`Replace` (it is intentionally not in
the apply run-set, and for a guarded omit the bare book legitimately may skip it too) — but gate-5
is 20F/D3a-owned and load-bearing, so I did NOT touch it (flagged, not done — §7 tc-gate5-omit).

This is the kind of cross-cell breakage the AGENTS.md exclusion-check warns about: the "Query fold"
cell that 20B §2 + 20E green-lit at the PLAN layer breaks at the HARNESS layer once a probe is
actually shimmed and executed. The value-story re-grounding STALENESS-AUDIT flagged ("the VALUE
story moves to Query-guard folds") is real, but the folds it moves to are gate-1-OPTED-OUT folds.

## §6 item-4 — the leaf-exact case-arm render fix (T14)

The defect (the standing xfail): `render_apply` is line-granular — it neutralises a whole LINE by
commenting it + emitting the stand-in at the line indent. For a one-liner `case` arm
(`nginx) apt-get install -y nginx ;;`), commenting the line ALSO swallows the structural `nginx)`
pattern and `;;`, leaving `case nginx in` followed by a bare `true` — a `dash -n` syntax error.

The fix (surgical, `crates/plan/src/lib.rs`): a NEW AST-structural helper `case_arm_oneliner_leaves`
walks every `NodeKind::Case` arm and collects the body-`List` items whose line equals the arm's
first-pattern line (a same-line arm body). `render_apply` substitutes those leaves IN-SITU — keep
the `pat)` prefix + ` ;;` suffix, replace ONLY the command's byte-span with its stand-in, append a
trailing provenance comment: `  nginx) true ;;   # dorc: elided …`. Detection is AST-structural
(not text-scanning for `)`/`;;`, which a command's own `)` would defeat) and scoped to DIRECT arm
body items only.

- **Zero golden churn elsewhere** (+SURE, git-verified): the ONLY pre-existing case the fix touched
  is the converted xfail. The multi-leaf-line cases (`render-multileaf-line-all-elide`,
  `exec-multileaf-line-mixed`) keep the whole-line comment form — they are `;`-separated sequences,
  NOT case arms, so the AST detection never fires on them. A multi-LINE arm body (body on its own
  line) ALSO keeps the whole-line form (the pattern line ≠ the body line). Both pinned by a new
  unit test pair (`render_one_liner_case_arm_body_substitutes_in_situ_keeping_arm_structure` +
  `render_multi_line_case_arm_body_keeps_whole_line_comment_form`).
- The converted case `render-case-arm-oneliner` ships authored goldens (expected.out hand-derived)
  + mocks (`apt-get` + a holds `dpkg-query`) + an EMPTY expected.ran. The exec gate proves the arm
  executes correctly: the rendered `case nginx in nginx) true ;; *) : ;; esac` runs `true` (the
  matched arm), nothing external ⇒ empty run-set; the bare book runs `apt-get install -y nginx`
  (the arm body) ⇒ gate-5 satisfied. gate-1 parity enforces (dpkg-query holds).
- **The if-guard render floor (`Status` channel) STAYS OUT OF SCOPE** (+SURE I did not touch it):
  the `consumption_ok` `Status`-blocks-unconditionally rule (the line-granular render cannot
  substitute an `if`/`then`/`fi` guard in-situ) is a DIFFERENT mechanism — it blocks the LICENSE in
  `plan`, not the render. My fix touches only the case-arm-body render path. **FLAG (per the prompt's
  explicit instruction): the case-arm in-situ substitution does NOT generalise to the if-guard floor
  for free** — a `case` arm body has a clean single byte-span to replace with the `pat)`/`;;`
  bracketing intact; an `if cond; then BODY; fi` guard's *condition* substitution would need to
  reproduce the guard's Status channel in-situ AND keep `then`/`fi` balanced, which is the deferred
  leaf-exact/structural render (C-5/seam-prov), not this. I did not attempt it.

## §7 tc-* / judgment calls flagged (conservative defaults taken; flagged up, not settled)

- **tc-find-shim** (§1): a `find` PATH-shim CAN make the pkgindex (`test -n "$(find … -newermt)"`)
  probe deterministic. I shipped one for `headline-guarded-realistic` (item-2) — there the fixture
  AND shim are authored together for a from-scratch fresh-host mock, which is honest. I did NOT ship
  one for the EXISTING find-cases (`exec-singleton-update` etc.), whose fixtures were authored for a
  REAL host — retro-adding a force-holds find-shim there would fabricate freshness to match
  (`inv-probe-sourced-values` masking). The line I drew: a find-shim is honest when co-authored with
  its fixture for a mock host, masking when bolted onto a real-host fixture. ~SUSPECT the human may
  want a different line (e.g. ban inert find-shims entirely, or allow them everywhere); flagged.
- **tc-gate5-omit** (§5): the clean fix for strain-D3b-fold-vs-gate5 is plausibly a gate-5
  refinement — skip an engine site whose disposition is `Omit` (it is deliberately absent from the
  apply run-set; for a guarded omit the bare book may legitimately skip it too). I did NOT do it
  (gate-5 is 20F/D3a-owned + load-bearing; a refinement risks re-opening the wrong-concrete hole
  gate-5 closes). Flagged for the orchestrator: is the shimmed-Query-fold worth a gate-5 carve-out,
  or is "folds are builtin-guard-only under gate-1-opt-out" the accepted permanent shape?
- **tc-source-path** (§4): `sourcing-degrades-safely` sources `. helper.sh` (no slash, PATH-found
  via mocks/) rather than the prompt's literal `. ./helper.sh`, because the gate-2 exec sandbox cwd
  is a throwaway mktemp — a `./helper.sh` relative source resolves nothing there and dash aborts
  (`.` of a missing file is fatal). No-slash PATH lookup is the sandbox-safe faithful Opaque source.
  Flagged in case the orchestrator wants the literal relative form (which would need a sandbox-cwd
  change in run.sh, D3a territory).
- **tc-pkgstate-kind** (§2): the new `query` oracle keys `dpkg -s` to a DISTINCT `pkgstate` kind
  (not the `package` kind `apt-get install` establishes). Rationale: the fold reads the guard's
  Status (rc) to decide the `||` — it does NOT cell-match guard-to-install (cf. fold-oror, where
  `command -v` queries `tool:` while the install establishes `package:` — distinct kinds, still
  folds). A distinct kind avoids a two-providers-one-kind cross-file probe-resolution wrinkle
  (`seam-two-providers-one-kind`'s spurious missing-probe diagnostic). Sound, but ~SUSPECT a
  same-cell `dpkg -s nginx` (querying `package:nginx#installed` directly) is the MORE realistic
  idiom; deferred (it would need the cross-file probe-supply seam sorted).

## §8 Exclusion-check (the four-by-two discipline, AGENTS.md)

- **other phase**: item-1's shims make the PROBE phase mock-executable to PARITY (it was
  site-complete-only in 20F); the apply phase exec gate was already there. The render fix (item-4)
  is apply-phase-only (the probe render never had the case-arm bug — it emits flat records).
- **other user**: the kept opt-out markers' rewritten reasons name the exact remedy/permanence for
  the admin/engineer; the strawmen's book comments explain the SAFE degrade in-line (why their
  commands run). The realistic book reads as a scrappy admin's book (the intended "lazy admin"
  voice), not an engine artifact.
- **other reliability**: the strawmen ARE the unreliable-input cells (a loop, an opaque source, an
  unresolved operand) — each degrades to run (kFAIL-perform), never to a silent elision. item-2's
  invalid guard-below is the unreliable-guard cell (validity-blocked ⇒ runs).
- **reverse propagation**: N/A to corpus authoring; the render fix consumes the engine's forward
  `Disposition` output + the AST (back-map), no propagation.
- **the killer cross-cell**: strain-D3b-fold-vs-gate5 IS an exclusion-check catch — the "Query
  folds" cell verified green at the PLAN layer (20E) breaks at the HARNESS layer (gate-5) once the
  probe is shimmed-and-executed. Verified the "fix" (absent guards) does not break OTHER cells
  (C-3, diverged, strawmen all still green).

## §9 What the one-Observable pass (next slice) inherits

- **strain-D3b-fold-vs-gate5 wants a ruling** (tc-gate5-omit): either gate-5 skips `Omit`-disposed
  sites (re-enabling shimmed-Query-fold e2e demonstration), or the corpus accepts "folds are
  builtin-guard-only under gate-1-opt-out" as permanent. The one-Observable unification touches how
  the fold reads the Status channel, so it should decide this with the firewall semantics fresh.
- **The 29/38 enforced-parity ratio is the live convergence-axis number** — re-count after the
  one-Observable pass adds/cuts cases; the 9 opt-outs are now cleanly tri-categorised (builtin /
  find-timing / empty-fixture), so a future reader can see which are permanent (builtin) vs
  anti-masking (find) vs test-shape (quoting).
- **The leaf-exact render now covers `case` arms** — if the one-Observable pass makes the if-guard
  `Status` substitution expressible (retiring the render floor, C-5), the case-arm in-situ
  machinery (`case_arm_oneliner_leaves` + the byte-span replacement in `render_apply`) is the
  pattern to extend, but the if-guard needs Status-channel reproduction + `then`/`fi` balancing that
  the case-arm did not (§6 flag).
- **`headline-guarded-realistic` is the broadest enforced-parity probe** (11 sites, 5 kinds) — a
  good regression anchor for the one-Observable wiring; if a channel-prediction changes, this case's
  run-set is the most sensitive corpus witness.
- **The `pkgstate` `query` oracle** (the first EXTERNAL-command Query, vs the builtin `command -v`)
  is the shimmable Query the gate-1 parity story needed; tc-pkgstate-kind (same-cell vs distinct-kind
  query) is the deferred realism refinement.

## §10 Confidence summary

- +SURE: all 50 e2e cases green, 0 xfail; the 4 prior gates (fmt/clippy/test/typos) green; the
  render fix has zero golden churn outside the converted xfail (git-verified).
- +SURE: the 29/38 enforced/opt-out split is honest — shims match the EXISTING fixtures (anti-mask),
  verified the flip ENFORCES (temp-copy corruption ⇒ loud gate-1 FAIL).
- +SURE: strain-D3b-fold-vs-gate5 is a real structural tension (traced end-to-end + reproduced); the
  shimmed-Query-fold cannot demonstrate the omit under gate-5; the builtin-guard fold-oror escapes
  only via un-shimmable-ness.
- +SURE: the case-arm render fix is leaf-exact + AST-structural + scoped (the if-guard floor is a
  separate mechanism, untouched).
- ~SUSPECT: tc-find-shim's honesty line (co-authored-fixture OK, retro-bolt masking) is the right
  call but is a judgment the human should ratify.
- ~SUSPECT: tc-gate5-omit (skip Omit-sites in gate-5) is the clean fix for the headline strain, but
  it is a load-bearing-gate change I deliberately deferred to the human/orchestrator.
- -GUESS: a same-cell `dpkg -s` query (tc-pkgstate-kind) is more realistic than the distinct-kind
  `pkgstate`, but exercising it needs the cross-file probe-supply seam, deferred.
