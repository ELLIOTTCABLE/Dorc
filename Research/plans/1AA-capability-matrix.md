# 1AA — H2SaLS capability matrix: engine-at-HEAD vs the corpus (D4)

> **DRAFT — pending the D4 hostile crosscheck (round task #6).**
>
> **Disclosure (read first):** LLM-generated (round-1A matrix agent). The corpus this
> scores (`Research/corpora/H2SaLS/harden.sh` + the `*.oracle.sh` seeds) is an
> intentionally quality-varied ARTIFICIAL N-of-1 testing corpus for a static-analysis
> project (Dorc); it is NOT real ops code, was never executed, and an artificial corpus
> cannot expose the truth of real-world ops-code. N-of-1: every frequency below is one
> book's count, not a population estimate. The **"generality" column especially is
> gut-feel, not evidence** — it is marked as such and must not be cited as data.
>
> Subject: THE ANALYZER (spike HEAD) — its CFG, value lattice, effect resolution,
> ⊤-triggers — profiled against the book as a workload. Engine claims are cited to
> file:line and confirmed in §4; difficulty grades are mine. Confidence marks per the
> house convention (+SURE / ~SUSPECT / -GUESS / --WONDER).

Inputs: census (`Research/corpora/H2SaLS/census/*`), the imp-* ledger (1A6 §3), the
um-* catalogues (1A8, 1A9), `ANALYZER-NEEDS.md` (`an-*` + `st`), and the spike source
(`spike/crates/{syntax,analysis}`). Line refs `L<n>` are `harden.sh` lines.

**Column definitions** (tc-M5 in 1AC flags the criticality definition as my call):
- *freq* — this book, census-cited (mechanical).
- *gen.* — **GUT-FEEL** guess at how common the shape is in real admin sh. Not evidence.
- *crit* — if the engine mis-models or must-⊤ this row, how much of the book's
  END-TO-END orchestrated execution breaks or floors (control-flow/dependency, not
  security); `+ord` = the row carries ordering that must survive (cross-section
  dependency, e.g. edits-before-restart, rules-before-enable).
- *sh-rw* — sh-rewrite difficulty: how awkward D1 was here (the imp-* ledger).
- *oracle* — D3 oracle difficulty (the um-* catalogues).
- *engine* — difficulty vs the modeled subset at HEAD, each grade citing its mechanism:
  a ⊤-trigger, an `an-*` row + status (B/S/D/O/W), or "modeled".

Grades: `none < low < mod < high < max`. "modeled" = HEAD handles the shape.

---

## §1 YIKES-LIST (ranked): high criticality × high engine difficulty

One sentence of cheapest-path each; the ranking is mine, re-derived from the seeds
(y-a..y-e verified; y-3 and y-7 added; y-c demoted below the added y-3).

- **y-1 — file-edit via redirect + generic sed/grep/printf** (rows A4/B4; seed y-a,
  confirmed rank 1). Crit max+ord, engine max: the mutating "verb" is shell *syntax*
  (`>>`/`>`), and at HEAD a `Redir` CFG node classifies **Pure** (effect.rs:532-539 —
  not even Opaque), while `printf` is a blessed-pure builtin (effect.rs:300), so the
  book's 11 append-edits are *invisible mutations* — they neither elide nor poison;
  `an-redirection-effect`/S names the fix, and `inv-referent-agnostic` forbids the
  sed-argstring parse that would be the shortcut (um-file-1). **Cheapest path:** resolve
  `RedirTarget::Word` through the already-built value plane and gen a per-path file cell
  into reaching-defs (weak update; ⊤-target ⇒ Opaque) — a pure *poison-correctness* fix
  with no probe/elision story, leaving read-side elision to the confline/confblock query
  spectrum (tc-F2/F3) and `an-guarded-establish`/D. +SURE on the mechanism; ~SUSPECT on
  cheapest.
- **y-2 — helper functions** (row B1; seed y-b, confirmed rank 2). Crit max, engine
  high-but-roadmapped: bodies are detached sub-CFGs (cfg.rs:844-868; brk-2 =
  `an-call-return-edges`/D), so all 24 calls are unknown-command Opaques — 21 of them
  THE file-edit mechanism — each a ⊤-poison bomb; additionally the value plane is
  *call-transparent* (value.rs:386-399 — calls havoc nothing), so a body-written flag
  read downstream is a latent wrong-CONCRETE, not just imprecision. **Cheapest path:**
  budget-bounded inlining as roadmapped — this corpus is maximally friendly (no
  recursion, tiny bodies, 23/24 calls pass fully-literal-resolvable args) — but land
  y-1's redirect cells first or simultaneously (see tc-M2, 1AC: inlining alone exposes
  the printf-invisibility to the query-validity fold). +SURE.
- **y-3 — `apt-get update`/`upgrade` as the poison economy** (row A3; ADDED — not in
  the seed list, and deliberately ranked above y-c). Crit high, engine difficulty LOW —
  that is exactly why it ranks here: at HEAD-with-seeds, L51 `apt-get update` (no oracle
  arm, um-pkg-3) is Opaque ⇒ `Reach::Top` from line 51 onward (effect.rs:464), so
  **every** downstream win from y-1/y-2/y-4 is masked — nothing can read ambient, no
  Query can read valid, no Members site can self-reach. **Cheapest path:** ship the
  sibling `pkgindex.oracle.sh` (the spike's own keystone exemplar shape) declaring
  update ⇒ `pkgindex#fresh` as a declared-effect-no-probe cell — zero engine code, and
  it is the *first domino*: do it before measuring anything else. +SURE on the poison
  mechanics; ~SUSPECT "zero engine code" suffices for upgrade (um-pkg-2's set-entity may
  resist even a declared cell).
- **y-4 — command-substitution in argv/assignment** (row B2; seed y-c, demoted one
  slot). Crit high, engine high: `an-top-surface`/O — `collect_frags` collapses any
  cmdsub-bearing word to ⊤ (value.rs:995-1013) ⇒ consumer Opaque; the inner commands
  are additionally effect-bearing non-leaves (cfg find-cli-1), so 11 sites are both
  elision-blockers *and* poison sources. **Cheapest path:** not general folding —
  partition the 11: mktemp ×4 → `an-scratch-ownership`/S; `hostname`/`id -u` →
  `an-host-identity-fact`/S as stdout-predicting Queries; `getent|cut` → the
  tc-getent-stdout-provider Query shape (um-user-2); `openssl passwd` stays ⊤ (pure
  transform, salt-random — um-pure-1); the quoted-static `$(cat <<'EOF')` (L196) is
  constant-foldable in principle (unregistered — tc-M4). ~SUSPECT on partition coverage
  generalizing.
- **y-5 — service restart-on-change (run-delta)** (rows A8/B10; seed y-d, demoted to 5
  on *sequencing*, not importance — crit is max+ord). Engine high and structurally so:
  um-svc-1 (restart-convergence is not host-probeable) + um-file-restart-1 (run-delta
  lives only in the author's flag dataflow); the value plane already traces the flags
  but reads ⊤ at every guard here (branch-joins for A's real gating; the §9 ⊤-loop
  havoc — y-7 — wipes even B's welded-1 constants, value.rs:325-331). **Cheapest
  path:** keep restart effect-less (the seed already does — sound floor), and grow the
  flag-trace into a boolean changed-taint lattice (`an-early-cutoff`/S) only *after*
  y-1..y-3 land — restart elision is the LAST domino because its soundness condition is
  "every upstream edit converged", which needs the file-edit rows first. +SURE on
  un-probeability; ~SUSPECT on the taint-lattice shape.
- **y-6 — multi-operand install** (row A2; seed y-e, confirmed but sharpened). Crit
  high, engine mod: `an-partial-convergence`/D (per-entity verdict map) plus the
  explicitly-flagged single-fact `SkipClass` fold (effect.rs:608-612); the oracle's
  R2-MULTIOP refusal is sound but lands as Opaque, so the 20-package line (L130-150) is
  not merely "always runs" — it ⊤-poisons §§3-11's reaching-defs too (un-stated in
  1A8; the refusal is not free). **Cheapest path:** multi-fact `EstablishAmbient` +
  per-entity verdicts, keeping the oracle refusal until then. +SURE.
- **y-7 — the ⊤-loop havoc blast radius** (row B6; ADDED). Crit mod locally but the
  *damage is global*: both `while read` loops carry `continue` ⇒
  `UnsupportedReason::Loop` (ast.rs:303-310) ⇒ the whole loop is ONE ⊤ node whose
  transfer havocs EVERY tracked variable (value.rs:325-331) — so §9 erases all constants
  (MAIL_TO, all four change-flags, …) for §§10-11 and the handler guards. **Cheapest
  path:** narrow the havoc, don't model the loop — a ⊤-region's `salvaged` children are
  parsed (ast.rs:175-178), so havoc only the variables assigned inside the salvaged
  subtree (plus positionals); modeling `continue`-as-edge is the real fix but is CFG
  surgery. ~SUSPECT the salvage-scoped havoc is sound as stated (a ⊤ region could
  `eval`-assign — gate it on the region's reason).

Dropped from yikes consideration: heredoc *bodies* (parsed losslessly, data not code —
the difficulty is oracle-side un-knowable bytes, concl-5), `case` (lowers fine,
cfg.rs:793-817), param-expansion and `set -eu` (the two *good-news* rows — B8/B9).

---

## §2 The matrix

### (A) state-affecting command families

| row | freq (census) | gen. (GUT-FEEL) | crit | sh-rw | oracle | engine @HEAD |
|---|---|---|---|---|---|---|
| A1 install, single-op | 3 of apt-get 12 (L54,640,665) | very high | high (install spine gates §§3-11) | low (imp-module-defaults residue) | low (exemplar parity) | **modeled** — the showcase; in-corpus still floored by upstream Opaques (y-3) |
| A2 install, multi-op | 1 (L130-150, 20 pkgs) | high | high (deps for §§6-9) | low | refusal mandatory (R2-MULTIOP) | **mod**: an-partial-convergence/D + single-fact SkipClass fold (effect.rs:608-612); today Opaque ⇒ runs+poisons |
| A3 update / upgrade | 5 / 3 (L51,126,637,659,662 / 127,638,663) | very high | high as poison-source (+ord: update-before-install) | low | max (um-pkg-2 set-entity; um-pkg-3 volatile, an-volatile/S) | **low mechanically** (Opaque by design) — fix is oracle-side re-key; the first domino (y-3) |
| A4 line-edit cluster (grep-guard + sed-replace ∥ printf-append) | grep 14 · sed 11 · printf 15 · `>>` 11; 3 inline instances + 21 helper calls | max — THE admin idiom | max (+ord: feeds every restart flag) | high (imp-lineinfile-lastmatch; f-5 anchor asymmetry) | max (um-file-1: query-only, no establish to declare; um-file-2 3-outcome rc, an-probe-shape/S; tc-F1 path×line identity) | **max** (y-1): an-redirection-effect/S; Redir⇒Pure (effect.rs:532-539); printf blessed-pure (effect.rs:300) ⇒ invisible appends; sed generic + inv-referent-agnostic |
| A5 whole-file overwrite (`cat > f <<EOF`) | cat 8; 6 overwrite sites (L340,475,494,516,553,656) | very high | max-ish (§§5-8 payload) | high (imp-blockinfile-truncate — deliberate persona divergence) | high (concl-5 un-knowable bytes for 4/6 unquoted; um-cron-1 job≡file) | **high**: cat external ⇒ Opaque (runs+poisons — sound); content-prediction needs heredoc-body expansion (unregistered, tc-M4) + an-leaf-text/D |
| A6 managed-block dance (mozilla block L194-259; awk insert L449-467) | 1 + 2 calls | mod-high | high (sshd_config + before.rules) | **max** (imp-blockinfile-anchoring — the D1 strain champion) | high (confblock `cmp` honest only for static bytes; the book's own cmp IS an inline content-probe) | **high**: every leaf ⊤-or-Opaque (mktemp ⊤ paths, sed/awk generic, cp/mv Opaque); the cmp-gated branch is an-guarded-establish/D material |
| A7 ufw | 8 (L183-184,409-421) + loop | mod (tool-specific; nft/iptables siblings) | high +ord (pre-open before port flip L183; rules before enable L410) | mod (imp-module-defaults proto-less; fix-2) | **max** (um-ufw-1 tuple entity → an-entity-shape/O; um-ufw-2 2-outcome + pre-enable window → an-probe-shape/S) | **mod**: argv resolves fine (`"$SSH_PORT"`→literal); blocked at one-word annotation arity — engine-side missing piece is an-entity-shape/O; refusal ⇒ Opaque poison today |
| A8 service restart | 6 (L188 inline flush-point; L628; L685-695 ×4 guarded) | very high | **max +ord** (flush-point inline restart is load-bearing; handlers-at-end semantics) | high (imp-change-detection; err-handlers-endplay was D1's worst break) | **max** (um-svc-1: not host-probeable, structural) | **high**: needs flag-taint (an-early-cutoff/S, um-file-restart-1); flags read ⊤ at guards (branch-join + y-7 havoc); restart Opaque today = sound floor |
| A9 user / group / getent | groupadd 1 · useradd 1 · getent 3 (§1) | high | high (§1 gates all; lockout-adjacent — imp-module-converge) | mod (skip-vs-converge) | mod-max (um-user-1 password impossible — clean result; um-user-2 consumed stdout) | **split**: groupadd loop = the Members showcase, mechanically resolved (effect.rs:203-241) but never licensed (self-reach pristine unreachable — tc-M1); useradd Opaque via `$(openssl …)` (y-4); getent guards Opaque (no query provider — tc-F3) |
| A10 dpkg-statoverride | 1 (L84-89) | low | mod | mod (capture+case tolerance dance) | low-mod (cleanest check-then-act pair; um-stat-1 value-divergence) | **high as-written**: RHS cmdsub ⇒ `su_err` ⊤; the mutator is expansion-internal (correctly effect-bearing non-leaf, cfg find-cli-1) ⇒ never an elidable leaf in this spelling; Status+Stderr co-consumption beyond the fold (um-statoverride-1) |
| A11 perms/plumbing (chmod chown chgrp install cp mv rm mktemp) | 4·1·1·2·4·1·4·4 | very high | mod (support work) | low | mod (stat-probes feasible; unseeded by scope — tc-F5) | **low-mod**: literal-argv sites resolve; all Opaque (no oracle); mktemp ⊤ spreads through path vars — an-scratch-ownership/S would contain |
| A12 wget / gpg | 2 / 1 (L627,643,651) | high | mod (key gates lynis; rules gate auditd) | low-mod (imp-geturl-force) | high (um-fetch-1 presence≠currency) — EXCEPT um-validator-2: the `creates:` guard L650 IS the probe | **mod**: guard-shape recognition = an-guarded-establish/D + an-tier-a-forms/D (`[ ! -e ]`); wget Opaque; `get_url` call hits brk-2 |
| A13 non-idempotent tail (mail rkhunter lynis psad) | 2·2·2·1 | high (every runbook has a notify/audit tail) | low for others (tail position) | mod (err-shell-snippet-rc → `\|\| true` renders) | n/a-max (um-mail-1/um-audit-1: MustRun by nature; um-rc-rkhunter-1: no rc-semantics primitive) | **none needed beyond floor**: Opaque ⇒ runs = correct; `\|\| true` errexit-swallow modeled (cfg.rs:241-260). This row sets the elision *denominator* ceiling |

### (B) load-bearing sh constructs

| row | freq (census) | gen. (GUT-FEEL) | crit | sh-rw | oracle | engine @HEAD |
|---|---|---|---|---|---|---|
| B1 helper functions | 4 defs / 24 calls; 21 = the config-edit helpers | very high | max | n/a (natural sh) | n/a (book-side; oracles target their *bodies*) | **max-but-roadmapped** (y-2): brk-2 / an-call-return-edges/D — detached bodies (cfg.rs:844-868), calls Opaque, call-transparent value plane (value.rs:386-399) ⇒ latent wrong-concretes |
| B2 cmdsub in argv/assign | 11 (L38,65,84,95,109,196,239,240,432,444,452) | very high | high | n/a | partitioned (um-pure-1; tc-getent-stdout-provider) | **high** (y-4): an-top-surface/O; word-collapse value.rs:995-1013; inner commands effect-bearing non-leaves ⇒ also poison sources |
| B3 heredoc bodies | 10 (8 unquoted / 2 quoted; incl. `cmp - <<EOF` L250, `done <<EOF` L579/597) | very high | high (the payload bytes) | low (natural) | concl-5 (pre-write probe needs un-knowable bytes) | **low structurally** (body+quoted captured, ast.rs:269-275); content-USE unbuilt: no body expansion (tc-M4), render = an-leaf-text/D |
| B4 redirections | `>` 14 · `>>` 11 · null 3 · fd-dup 3 | max | max (+ord: THE write mechanism) | n/a | um-file-1 (no provider token) | **max** (y-1): Redir node exists (cfg.rs:75-78) but classifies Pure (effect.rs:532-539); an-redirection-effect/S; fd-dup beyond floor = an-fd-state/D; `>/dev/null`+`>&2` consumption itself modeled (an-output-consumed-enclosing/B) |
| B5 for loops | 2, both literal-list (L57, L420) | high | mod | n/a | (rows A7/A9) | **modeled** (task-L1/L2: ForLoop ast.rs:139-144; LoopHead+back-edge cfg.rs:889-899; Members value.rs:560-695; EstablishMembers effect.rs:396-419) — but never licensed here: in-loop floor (cfg.rs:198-206) + self-reach-pristine unreachable under any realistic preamble (effect.rs:478-483; tc-M1) |
| B6 while-read-over-heredoc | 2, both with `continue` (L572-588, L590-605) | mod-high | mod locally; **global value-damage** (y-7) | low (natural admin table idiom) | would be confline-shaped per pair | **high**: `continue` ⇒ UnsupportedReason::Loop (ast.rs:303-310) ⇒ one ⊤ node ⇒ havoc-all (value.rs:325-331); even sans continue: `read` lvalue-clobber (value.rs:426-441) + static-heredoc-table enumeration is unregistered machinery (tc-M4) |
| B7 case | 1 (L85-89) | high | low here | n/a | (A10) | **modeled** structurally (cfg.rs:793-817); scrutinee ⊤ here anyway; glob-arm value-matching unbuilt (moot) |
| B8 param expansion | 96 plain `$VAR` · 10 positional (all in detached fn bodies) · **0** braced/operator/special | the zeros do NOT generalize (gut-feel: real scripts use `${x:-d}` etc — ParamComplex ⇒ ⊤, ast.rs:218-220) | n/a | n/a | n/a | **none — good-news row**: the whole book sits inside the modeled `Param` subset; positionals only matter under inlining (the binder is y-2's work) |
| B9 `set -eu` + `\|\|`-tolerance | 1 (L24); 8 or-lists (5 = tolerance `\|\| true`) | very high | **max** (errexit decides reachability everywhere; tolerance inverts 3 upstream rc-semantics — 1A6 §1) | mod (err-shell-snippet-rc was MY mapping-error class) | n/a | **modeled-strong**: precise errexit edges incl. `\|\| true` swallow, cond/negation exemptions (cfg.rs:241-260; an-errexit-state/B; note-166); `-u` half = an-shell-options/S, benign (entry-⊤ seed compatible). The strongest engine row |
| B10 change-flag run-delta | 4 flags, 4 guards (L160-177, 404-406, 685-695) | very high (the notify idiom) | max +ord | high (imp-change-detection: A's honest cmp-gating vs B's welded-1) | **max** (um-file-restart-1: not host-probeable at all) | **high** (y-5): needs changed-taint (an-early-cutoff/S); value plane follows the flags but reads ⊤ at all four guards (A's branch-joins; B's welded-1s erased by y-7's havoc first); `if`-guard status is StatusRenderFloor (in-situ substitution impossible) |

### Cross-cutting reading (the three headline shapes of the gap)

- **head-1 — the gap is in the effect/identity planes, not the grammar.** +SURE. Of the
  whole ⊤-trigger set (ast.rs:295-313), the ONLY one this corpus fires is `Loop` (the
  two `continue`s). Zero eval / arith / backtick / env-prefix / braced-params /
  unquoted-glob (census zeros) — the parse-level subset fits this book almost exactly.
  What floors the book is: no-oracle Opaques, cmdsub-⊤, detached functions, invisible
  redirect-writes.
- **head-2 — the poison economy dominates the elision economy.** +SURE. One Opaque
  ⊤-poisons all downstream ambient-ness (effect.rs:464) and one ⊤-region havocs all
  downstream *values* (value.rs:325-331); this corpus triggers both early (L51; L572).
  Fix-sequencing therefore matters more than per-row difficulty: y-3 (cheap) unmasks
  everything; y-1 before y-2 (tc-M2); y-5 last.
- **head-3 — refusals are sound but not free.** +SURE, and unstated in the seed notes:
  every oracle refusal (R2-MULTIOP, ufw, restart) lands as `Opaque`, which both runs AND
  poisons. The honest-refusal posture is correct under kFAIL-perform, but each refusal
  costs all downstream rows their ambience — worth a per-refusal "declared-effect,
  no-probe" cell shape (the um-pkg-3 disposition) so a refusal can stop poisoning
  without ever licensing elision. ~SUSPECT that shape is already expressible in the
  dialect (the service seed does it for restart).

---

## §3 What this matrix does NOT say (scope guards)

- No security-domain claims anywhere above; "criticality" is execution/ordering only.
- Elision-RATE predictions are deliberately absent: at HEAD-with-seeds the sound
  elision count on this book is zero-or-near (head-2), and any % would be N-of-1
  noise. The round-21 consumer should re-derive rates after y-3/y-1 land.
- The oracle column grades the *difficulty of honest coverage*, not seed quality;
  tc-F2 (provider-collision: `crond` + `fetched` both key `test`) is now LIVE against
  effect.rs:139-151's first-resolves-wins comment ("no corpus case is ambiguous" — the
  seed-set makes one). Flagged in 1AC, not resolved here.

## §4 Engine-citations confirmed (appendix)

All verified by direct read this session, against the worktree at
`spike/crates/{syntax,analysis}` + `ANALYZER-NEEDS.md`:

- ast.rs:91-179 `NodeKind` set incl. ForLoop (139-144), WhileLoop (145-154),
  Unsupported+salvage (171-178); ast.rs:207-223 WordPart ladder (ParamComplex/
  Arithmetic ⊤-ward); ast.rs:264-276 `RedirTarget::HereDoc { body, quoted }`;
  ast.rs:295-313 `UnsupportedReason` = DynamicExecution / ArithmeticExpansion /
  DynamicLValue / Loop (no-`in` for, break/continue, cmdsub-in-list — 303-310) /
  Unmodeled.
- value.rs:1-37 module doc (authoritative): Flat height-2 domain, entry-⊤ seed
  (24-30, transfer at 298-303), ⊤⇒kFAIL-perform floor (34-37), non-convergence ⇒
  all-⊤ (36-37, 160-162, 513-515). value.rs:325-331 ⊤-region havoc-all.
  value.rs:386-399 `transfer_command` (function calls pass through — no havoc).
  value.rs:401-484 lvalue-builtin clobbers (read/unset/export/readonly/local/getopts;
  `cd`→PWD at 766). value.rs:560-695 Members pass (task-L2). value.rs:976-1021
  `recipe_of_word`/`collect_frags`: any cmdsub/arith/operator-expansion ⇒ word ⊤.
- effect.rs:92-201 `command_effect`: ⊤ command word ⇒ Opaque (106-107); ⊤ arg ⇒
  Opaque (128-134); no check resolves ⇒ Opaque (152-157); no effect-map cell ⇒ Opaque
  (166-173). effect.rs:285-304 `is_target_state_pure_builtin` = exactly {set cd export
  unset shift read readonly local : true false echo printf test [}. effect.rs:309-364
  `Reach` (+`is_pristine` 361-364). effect.rs:366-419 `SkipClass` incl.
  EstablishMembers+self_reached. effect.rs:421-443 entry-reachability (detached ⇒
  MustRun). effect.rs:451-469 reach transfer (Opaque ⇒ ⊤ join at 464). effect.rs:471-483
  `self_reach_holds` (suppressed-solve, pristine = empty). effect.rs:497-642 `classify`:
  Top-node ⇒ Opaque, **all other non-Command kinds incl. Redir ⇒ Pure** (532-539; grep
  over effect.rs finds zero `redir` mentions); single-fact-only fold flagged (608-612);
  Ambient/Written split (617-623); QueryResolvable validity (624-635).
- cfg.rs:75-78 `Redir` first-class node (sequenced before its command);
  cfg.rs:140-145+198-206 `in_loop` render-floor; cfg.rs:241-260 precise errexit
  materialisation (incl. `|| true`, negation, cond-region exemptions);
  cfg.rs:626-630 `exit`/`return` → program-exit, NO fall-through (so the L38
  root-check's `exit 1` poisons nothing downstream); cfg.rs:793-817 `lower_case`;
  cfg.rs:844-868 `lower_funcdef` (detached; def = pass-through Merge; find-7 TODO);
  cfg.rs:889-899 `lower_for` (LoopHead + back-edge); cfg.rs:923-939 `lower_while`
  (errexit-exempt cond, StatusRenderFloor).
- ANALYZER-NEEDS.md statuses cited: an-redirection-effect/S (§B), an-fd-state/D,
  an-word-expansion/S, an-exit-status/S, an-errexit-state/B; an-call-return-edges/D
  (§M); an-top-surface/O (§K); an-partial-convergence/D, an-entity-shape/O,
  an-scratch-ownership/S, an-host-identity-fact/S, an-volatile/S (§A/§C);
  an-guarded-establish/D, an-tier-a-forms/D, an-probe-shape/S (§D);
  an-early-cutoff/S (§G); an-leaf-text/D (§H).

NOT independently confirmed (taken from 1A8's ground-truth lift + effect.rs's own test
comments; the oracle crate was not read this session): the check-evaluator's modeled
`Word`/`TestOp` sub-dialect, `resolve_probe`'s multi-selector rule, and the exact
"annotation-reached-but-no-probe ⇒ Top" evaluate semantics. ~SUSPECT-confirmed only.
See 1AC §1.
