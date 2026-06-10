# 20F — task-D3a (harness gates only): the five gates, their demonstrated-failure evidence, and the convergence-axis residual

> Round-20 spike note, append-only. Records task-D3a (205 §1 / 20B §2+§3): the five
> acceptance-harness gates the 20B crosscheck owed — probe-exec-under-mocks + record-parity
> (gate-1, load-bearing), redirection sandbox (gate-2), stderr-severity floor (gate-3),
> order-sensitive run-set (gate-4), argv-echo differential (gate-5). HARNESS ONLY: the sole
> engine touch is a cli-edge `--debug-argv` readout + a one-line `report()` severity-format
> change (both I/O-edge, kernel untouched). NO corpus authoring beyond opt-out markers + the
> reorder re-bless (the corpus slice + render work is D3b, separate). AI-authored,
> confidence-marked. Trust R/D/I/K + 19H/19I + the human rulings over this. Builds on 20B §2/§3
> (the charter), 20C (the wire + record grammar gate-1 consumes), 20E §9 (what D3 inherits).

## §0 What landed (all green: cargo fmt/clippy -D warnings/test 252-pass-0-fail-1-ignore;
## `sh e2e/run.sh` 46/46 incl. the standing T14 render xfail; `typos spike` clean)

- `spike/e2e/run.sh`: four new gate functions + the wiring — `scan_redirect_safety` (gate-2),
  `probe_exec_check` (gate-1), `scan_diagnostics` (gate-3), `argv_echo_check` (gate-5); plus
  `exec_check` refactored for the gate-2 sandbox cwd + gate-4 unsorted compare.
- `spike/e2e/scan_redirects.awk`: NEW — the gate-2 redirect scanner (a conservative lexical
  pass over the rendered artifact; POSIX awk, the harness's existing dependency).
- `spike/crates/cli/src/main.rs`: `--debug-argv` flag + `emit_debug_argv` (gate-5 readout);
  `report()` now leads each diagnostic with its severity word (`<stage>: error[<code>]: …`)
  so gate-3 can key on the `error[` shape. Both are the sanctioned I/O edge; `inv-determinism`
  still exempts only `cli`, the kernel stays print-free.
- 26 `PROBE_RESULTS=authored` marker files (the gate-1 opt-out, §2) + 4 `expected-diagnostics`
  files (the gate-3 declarations, §4) + 8 `expected.ran` re-blessed to ordered form (gate-4, §5).

## §1 The gate mechanisms + their demonstrated failures (run from temp copies, never the repo)

Every gate was proven to FAIL on a planted perturbation (the crash-stub discipline 20B §2
established): a gate that cannot fail is not a gate.

- **gate-1 — probe-exec-under-mocks + record-parity** (`rule-probe-exec-gate`, the load-bearing
  one). For each `mocks/` case, EXECUTE the rendered probe under `PATH=<case>/mocks` + sandbox
  cwd + `DORC_LOG`, then assert:
  - **(a) site-completeness + grammar (ALWAYS)**: the resolvable-site emitter set (the
    `printf 'site N …` lines) must equal the emitted-record set, each record matching
    `site <int> effect=<holds|absent|cant-tell> rc=<int>`.
  - **(b) parity + (c) vouch-closure (unless opt-out)**: produced effect-words must match the
    authored `probe-results.txt`; and NO record may carry `rc=127` (an un-shimmed probe command
    — the executable half of vouch-closure failing loud).
  - DEMONSTRATED (a): a `dorc` wrapper rewriting one emitter to `effect=BOGUS-NOT-A-WORD` ⇒
    emitters `0 1` but valid records `0` ⇒ loud FAIL printing the malformed record. DEMONSTRATED
    (b): corrupting `exec-distinct-selectors`'s fixture to `site 1 effect=absent` (mock produces
    `holds`) ⇒ FAIL with "do NOT silently re-bless". (c) fires on 25 corpus cases by construction
    today (§2) — that IS the residual, captured as opt-outs rather than hidden.
  - **The wrong-concrete firewall is respected**: gate-1 compares on the `effect=` projection
    (drops `rc=`), because the establish-site rc is the probe-command's, never fold-usable (20C
    §2 / 20E §2). It validates the convergence signal (Effect), not the firewalled rc.

- **gate-2 — redirection sandbox** (20B §3, the human-surfaced hole). PATH-isolation governs
  which COMMANDS run, not where their `>`/`>>` write. Two real mitigations on existing machinery:
  - every `exec_check` (apply AND the new probe-exec) now runs in a subshell `cd`'d into a fresh
    `mktemp -d` — a bare relative redirect lands in disposable space, not the repo (today it
    lands in `run.sh`'s cwd);
  - BEFORE executing, `scan_redirects.awk` REFUSES any redirect whose target is absolute, dynamic
    (`$`/backtick), or `..`-escaping; allowlist `/dev/null` + fd-dups (`2>&1`). A conservative
    lexical pass (comments stripped, quote-state tracked); over-refusal prints the line.
  - DEMONSTRATED: a book perturbed to `apt-get install -y nginx > /etc/evil-marker` ⇒ REFUSED
    before exec, `/etc/evil-marker` never created. And a relative `> relative-marker.txt` ⇒
    allowed, lands in the sandbox, confirmed ABSENT from both the case dir and the harness cwd.
  - +SURE this is real mitigation, NOT the true answer: the Linux/VPS isolation tier stays the
    answer for arbitrary books (20B §3). The scan is over OUR renders, conservative by mandate.

- **gate-3 — stderr-severity floor** (20B §2 residual). dorc's stderr (previously `2>/dev/null`)
  is captured; a case FAILS if it carries an `error[`-shape diagnostic not covered by an
  `expected-diagnostics` file (fixed-string `grep -F` per line). Warnings/notes are free-form.
  - DEMONSTRATED: appending a `for` loop to a no-`expected-diagnostics` case ⇒ undeclared
    `error[syntax-unsupported]` + `error[cfg-top-node]` ⇒ loud FAIL. Negative direction verified:
    the `error[` floor pattern matches neither `warning[…]` nor `note[…]` lines.
  - tc-flag (`tc-report-severity`, §6): this needed `report()` to EMIT severity (it printed
    `<stage>: <code>: <msg>`, severity-blind). I judged surfacing severity a correct I/O-edge fix
    regardless of the gate (a diagnostic stream that hides severity is itself a defect) and took
    it; flagged because it is an engine-repo edit, narrowly outside "harness only".

- **gate-4 — order-sensitive run-set** (20B §2). `exec_check` compares `expected.ran` UNSORTED
  (sequential sh is deterministic; sorting discarded the welded "book order is sacred" assertion).
  - DEMONSTRATED: perturbing a golden to the SWAPPED order (sorted-EQUAL to execution, so the OLD
    gate was provably blind) ⇒ the unsorted gate FAILs with an order diff.
  - Re-bless summary (§5): 8 `expected.ran` re-blessed to ordered form, ALL pure reorders, ZERO
    `expected.out` touched.

- **gate-5 — argv-echo differential, msys tier** (cm-2). Cross-checks the engine's per-site
  resolved argv against dash's ground truth: `dorc --debug-argv` emits `argv <leafid> <word|TOP>`;
  the harness runs the BARE `book.sh` (all-shims by construction) under mocks + sandbox, collects
  the shims' logged `ran:` argvs, and asserts — ONE-DIRECTIONALLY, conservatively — that every
  FULLY-RESOLVED site whose argv[0] is a SHIMMED command appears in the log.
  - DELIVERED in full (not partial). ~60 argv resolutions asserted across the 33 mocks cases
    (1–8 per case; `headline-*` assert all 8 externals). DEMONSTRATED: a wrapper rewriting the
    engine's readout to claim `apt-get install -y apache` where the bare book runs `nginx` ⇒
    loud FAIL ("dash disagrees with value-flow") — exactly the prefix-env wrong-concrete class
    that died to crosscheck this round would now die to this gate by construction.
  - The conservative formulation (skip `TOP`, skip builtins/un-shimmed, ⊆ not =) produces ZERO
    false failures on the corpus, INCLUDING the branchy guard cases (§6 gate-5 obstacle analysis).

## §2 The PROBE_RESULTS=authored opt-out list — the honest residual of the convergence axis

This is the deliverable the prompt asked for by name. 33 mocks cases; **26 opt out**, **7 enforce**
gate-1 (b)+(c) fully. Of the 7 enforced, only ONE exercises non-trivial parity:

- **`exec-distinct-selectors`** — the SOLE genuine end-to-end parity case: 2 resolvable sites
  (`service:nginx#enabled`/`#active`), both `holds`, reproduced because `systemctl` IS shimmed
  (exit 0 ⇒ holds). +SURE this is the only case whose probe the apply-only mocks can faithfully
  reproduce today.
- **6 zero-resolvable-site cases** (`exec-detached-fn`, `exec-multi-entity`, `exec-opaque-neighbour`,
  `exec-opaque-var-runs`, `exec-same-cell-kill`, `top-eval`): empty probe ⇒ site-completeness +
  parity vacuously hold, vouch-closure has nothing to check. Enforced but trivial.

The 26 opt-outs, by reason (each marker file carries a case-specific one-line reason):
- **`dpkg-query` un-shimmed** (the `package` kind's probe; mocks carry only the apply
  `apt-get`) — 17 cases: `exec-consumed-stdout`, `exec-converged`, `exec-devnull-exempt`,
  `exec-diverged`, `exec-dollarq-blocks-elision`, `exec-enclosing-pipe-subshell`,
  `exec-errexit-top-status-runs`, `exec-literal-unset-pure`, `exec-multileaf-line-mixed`,
  `exec-pure-builtin`, `exec-resolved-var-elides`, `exec-subshell-establish`,
  `exec-subst-body-nonleaf`, `exec-top-arith-in-arg-ok`, `probe-operand-quoting`,
  `render-multileaf-line-all-elide`, `seam-two-providers-one-kind`.
- **`command -v` (tool, builtin — resolves against PATH=mocks-only where the operand is absent)
  + `dpkg-query`** — 4 guard-composition cases: `exec-query-after-mutator-runs`,
  `exec-query-guard-composition`, `fold-oror-guard-omits`, `guard-status-blocks-elision`. The
  fixtures assert real-host convergence, not all-exit-0/PATH-absent mock output.
- **`getent` un-shimmed** (the `user` kind) — `andor-rc-undeclared-runs` (mocks: mkdir/useradd).
- **`test -n "$(find … -newermt '-1 hour')"` (the `pkgindex` kind)** — filesystem-timing
  dependent, inherently non-mock-reproducible without a find/test shim:
  `exec-singleton-update` (this one fails PARITY, not 127 — `find` is real, returns no fresh
  lists ⇒ `absent` not the authored `holds`); `exec-poison-wall-dead` (dpkg-query + pkgindex).
- **4-kind mix** (`dpkg-query` + `ufw status | grep` with `grep` un-shimmed + `find`-timing +
  `systemctl`) — `headline-partial`, `headline-pi-webhost`.

+SURE this list is the convergence axis's honest state: the corpus's `mocks/` dirs were authored
for the APPLY gate (they carry only the book's mutators), so the PROBE commands have no shims.
gate-1's site-completeness still runs on all 33; parity+vouch await **D3b shipping probe-specific
shims** (e.g. an inert `dpkg-query` that exits 0/1 per a fixture). The opt-out is the anti-masking
guard: it refuses to silently re-bless fixtures to match all-exit-0 mock output (which would be the
19I §3 trap from the other side).

## §3 The gate-1 feasibility finding that shaped the design (+SURE, traced under mocks)

Running the rendered probe under the present apply-mocks reveals the un-shimmed probe commands
produce `rc=127 ⇒ cant-tell`, NOT a script abort: the `<kind>__check` wrappers swallow the
not-found via their own `>/dev/null 2>&1`, so the only signal is `rc=127` in the emitted record.
So "an un-shimmed invocation = command-not-found = loud fail" (the prompt's framing) could NOT be
detected via the probe's exit code — I detect `rc=127` in the records explicitly. This is why
gate-1(c) is a record-scan, not an exit-status check; recorded because a future reader will expect
the exit-code path and find it doesn't fire.

## §4 The expected-diagnostics declarations (gate-3)

4 cases legitimately emit error-severity diagnostics (their REASON for existing) and ship an
`expected-diagnostics` file (fixed-string patterns, substring-matched):
- `toprejected` (`error[syntax-unsupported]: loop constructs` + `error[cfg-top-node]`),
- `top-eval` (`error[syntax-unsupported]: \`eval\` …` + `error[cfg-top-node]`),
- `background-amp-runs` (`error[syntax-unsupported]: background/async \`&\`` + `error[cfg-top-node]`),
- `seam-two-providers-one-kind` (`error[oracle-missing-probe]: … \`package\` …` — a DELIBERATE
  missing probe; the case tests two providers, one without a probe).
All other cases must keep stderr free of `error[` lines — the floor.

## §5 The .ran reorder summary (gate-4) — mechanical, verified pure

8 `expected.ran` re-blessed via `BLESS=1`, run ONLY after gates 1–3 were green. Each verified a
PURE REORDER (sorted-equal to the pre-bless golden — same lines, execution order replaces sorted
order); the `BLESS` run touched ZERO `expected.out` (diffed: 0 changed) and exactly these 8 `.ran`:
`andor-rc-undeclared-runs`, `exec-errexit-top-status-runs`, `exec-opaque-neighbour`,
`exec-opaque-var-runs`, `exec-same-cell-kill`, `headline-partial`, `headline-pi-webhost`, `top-eval`.
The 25 single-command `.ran` were already order-invariant (sorted == ordered). Example
(`headline-partial`): the ordered golden now captures the book's real sequence (`apt-get update` →
`apt-get install` → `ufw allow 80` → `ufw allow 443` → `systemctl enable` → `start` → `nginx -t` →
`systemctl reload`), which the sorted golden discarded. No NON-reorder diff appeared (the
stop-and-flag condition); none was expected (execution is the same commands, just ordered).

## §6 tc-* / judgment calls flagged (conservative defaults taken; flagged up, not settled)

- **tc-report-severity** (§1 gate-3): I changed `report()` to emit the severity word — an
  engine-repo edit (the I/O edge, not the kernel), narrowly outside "harness only". Conservative
  read: it is the correct fix independent of the gate (severity-blind diagnostics are a defect),
  it is the minimal change that makes "error-severity shape" a real matchable thing, and it broke
  zero tests. If the orchestrator wants it reverted, gate-3 would instead have to floor on ALL
  diagnostic-shaped lines (failing warnings too — contradicting "warnings allowed"), which is
  worse. Flagged for ratification.
- **tc-gate5-always-on** (gate-5): the prompt called gate-5 "a new optional gate section". I run
  it by default for mocks cases (not behind an env flag) because the conservative formulation
  produces zero false failures and catches regressions automatically. It is skipped under `BLESS`
  (it asserts, never re-authors). If "optional" meant opt-in, gate it behind an env var — but
  always-on is the higher-value default and is not flaky on the corpus. Flagged.
- **tc-probe-parity-projection** (gate-1): parity compares the `effect=` projection, DROPPING
  `rc=`. Rationale: the fixtures historically omit `rc=` for establish sites (20C
  strain-D1-recordgrammar-rc-omitted), and the firewall forbids an establish rc from the fold
  anyway — so the convergence signal (Effect) is the honest parity target. A future D3b that ships
  probe shims emitting genuine rc could tighten parity to include rc FOR QUERY SITES (where the rc
  IS fold-valid, 20E §2). Conservative-for-now: effect-only parity. Flagged.
- **tc-scan-conservatism** (gate-2): the redirect scanner is lexical, not a sh parser; it
  over-refuses by design (e.g. it would refuse a `>` whose target is a legitimately-absolute
  `/dev/stdout` — only `/dev/null` is allowlisted). On the current corpus it over-refuses exactly
  the 2 non-mocks cases with absolute-path redirects (`redir-as-effect`, `enclosing-group-redir`)
  — which is HARMLESS because those cases lack `mocks/` and are never executed (the scan runs only
  inside `exec_check`). If a future case needs an absolute non-`/dev/null` target in an EXECUTED
  artifact, the allowlist widens consciously. Flagged.

## §7 Exclusion-check (the four-by-two discipline, AGENTS.md)

- **other phase**: gate-1 closes the probe-exec gap that the apply-exec gate (20B §2 / `ap-2`)
  already covered for apply; the two now mirror (both execute under inert shims + sandbox + scan).
  The probe is read-only by construction, so the sandbox/scan is belt-and-suspenders there, but
  the SAME machinery guards both phases (no asymmetry to sneak a redirect through the probe).
- **other user**: the opt-out markers + expected-diagnostics carry human-readable reasons (the
  admin/engineer reading a FAIL sees WHY and the remedy — "add a probe shim, or mark
  PROBE_RESULTS=authored", "declare it in expected-diagnostics"), not opaque gate-internals.
- **other reliability**: gate-5's one-directionality is exactly the unreliable-branch
  accommodation — a branch the bare run skips (or a site the engine ⊤s) does NOT over-assert
  (engine-resolved-and-shimmed ⊆ logged, never =). Verified no false-fail across all 33 cases.
- **reverse propagation**: N/A to the harness (no analyzer propagation here); the gates consume
  the engine's forward output (probe records, resolved argv, diagnostics) as data.

## §8 What D3b (the corpus + render slice) inherits

- **Probe shims are the unblock for 25 of the 26 opt-outs.** An inert `dpkg-query`/`getent`/`ufw`/
  `grep` shim (exit 0/1 per a fixture, logging like the existing apply shims) lets those cases'
  probes reproduce the authored fixture ⇒ flip `PROBE_RESULTS=authored` → enforced parity. The
  gate machinery is already in place and waiting; D3b just authors shims + removes markers. (The
  26th, `exec-singleton-update`, needs a `find`/`test` shim to defeat filesystem-timing.)
- **gate-1 site-completeness already runs on all 33** — D3b's new cases get it for free, and get
  parity+vouch the moment they ship probe shims. A new mocks case with NO probe shims must carry a
  `PROBE_RESULTS=authored` marker (with a reason) or fail gate-1(c).
- **The guarded realistic e2e book (20B §2's group-J restoration)** is D3b/task-D corpus work; when
  it lands, gate-5 will validate its value-flow argvs and gate-1 its probe records automatically.
- **Query-site rc parity** (tc-probe-parity-projection): when D3b ships a Query probe shim emitting
  a genuine rc, parity for Query sites could tighten to include the rc (the fold-valid one, 20E §2).
- **The T14 render xfail** (`render-case-arm-oneliner-wrong`) stays xfail — its fix is render-layer
  (leaf-exact `render_apply`), explicitly D3b/seam-prov, untouched here. gate-2's scanner is render-
  shape-aware (lexical over our renders); a leaf-exact render change should re-run the gate-2 corpus
  over-refusal check (§6 tc-scan-conservatism) in case new redirect shapes appear.

## §9 Confidence summary

- +SURE: all five gates are implemented, green, and each demonstrably FAILS on a planted
  perturbation (run from temp copies; the repo was verified unperturbed afterward).
- +SURE: gate-4's 8 re-blessed `.ran` are pure reorders, 0 `expected.out` changed (diffed).
- +SURE: the kernel is untouched — only `cli` (the sanctioned I/O edge) gained `--debug-argv` +
  the `report()` severity word; `cargo test` 252-pass-0-fail confirms no behavioral regression.
- +SURE: the 26-case opt-out list is the honest convergence-axis residual (the corpus's mocks were
  authored for apply, not probe), NOT a gate weakness — site-completeness still runs on all 33.
- ~SUSPECT: gate-5 always-on (tc-gate5-always-on) is the right default, but the prompt's "optional"
  framing wants a human nod. Conservative formulation has zero corpus false-fails.
- ~SUSPECT: tc-report-severity (the `report()` edit) is correct and minimal, but it is the one
  place I touched engine-repo code beyond the sanctioned debug flag; flagged for ratification.
