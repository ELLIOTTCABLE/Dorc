> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, ADVERSARIAL stance (24Kb): verbatim extract from commit fd5fa82 on branch worktree-agent-abd7ff8be88067e1b. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Kb-03 — corpus observations (raw, line-cited; findings live in 24Kb-04)

Read order: README → TODO → DESIGN → USER_STORY → ORACLE_PROVIDES → spike/CLAUDE.md → KNOBS →
e2e/run.sh + fixtures (guard23-ternary-flagship, seam-two-providers-one-kind, inverted-vouch) →
15x-strawmen README + apt-get.straw.sh → 24G → 17O → 24C (find-return-vouches) → targeted greps +
plan/lib.rs + cli/main.rs excerpts. Stayed out of quarantine-DO-NOT-READ and corpus dirs throughout.
Empirical shell tests run locally (bash 5.3.9 msys, /bin/dash): see "shell experiments" below.

## Shell experiments (reproducible; /tmp/dorctest)

- `foo.bar() { ...; }` — bash: accepted (define+call), even `bash --posix`; dash: "Syntax error:
  Bad function name" rc=2, both `-n` and execution. +SURE.
- `apt-get_predict() { ...; }` (hyphen) — bash: accepted; dash: same rc=2 parse error. +SURE.
  So even a single-underscore-stripped name for a hyphenated command is dash-invalid; the engine's
  actual munge (`apt_get__is_converged`, plan/lib.rs:708) handles this — the LAW TEXT does not
  (spike/CLAUDE.md rul-ternary-verdict: "`name.predict()` → `name_predict()`").
- book-lines-then-dotted-def: dash executes preceding lines then dies rc=2 at the def; anything
  after never runs. bash runs whole file fine.
- `printf '%s\n' "/x" : service` prints THREE lines (`/x`, `:`, `service`) — un-stripped trailing
  marks are argv, not inert.
- `dest : fb.Certs = "/x"` under bash: `dest: command not found`, rc=127.
- Corpus-grade consequence (fixture apt.oracle.sh:13): un-stripped
  `dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed` passes `:` and the mark as TWO
  EXTRA PACKAGE ARGS to dpkg-query → rc 1 always → a raw-executed authored probe silently reports
  "absent". Raw execution of authored oracle bodies is not merely broken, it is *plausibly wrong*.

## Dialect surface inventory (as settled/authored)

- Role families (24G §2, settled): per-TOOL `predict()`/`is_converged()`/`is_diverged()`/
  `touches()` keyed by command; per-KIND `resolve()`/`reaches()` keyed by kind. All optional,
  monotonic floors (rul24-threefunc-monotonic).
- Authored names are dotted (`apt-get.is_converged()` — fixture apt.oracle.sh:20; USER_STORY:236);
  ship-forms are mangled `apt_get__is_converged` (goldens guard23-ternary-flagship/expected.out:33;
  plan/lib.rs:708,1226,1383,1500). Fixture files MIX eras: `apt_get__predict()` authored
  pre-mangled beside dotted `apt-get.is_converged()` in one file (guard23 package.oracle.sh:12,30).
- Annotation/mark family (`:`): binder `pkg : package = "$1"` (apt.oracle.sh:10); trailing
  establish-mark `cmd ... : kind:"$e".prop[!]` (apt.oracle.sh:13-14); `:?` observe mark + bare ACK/
  POISON statement-marks (ORACLE_PROVIDES provides-binding); per-ROLE semantics — same trailing mark
  = establish in predict(), emission-typing in reaches() (24G §4 "a language decision to document
  loudly"); two-level tilde vouch-mark = dead grammar (spike/CLAUDE.md rul-guard-license correction).
- rc partition (welded, rul-rc-partition): verdict-fn exit 0 = named sense; 1 = complement; ≥2 =
  confused ⇒ run. Declared-dual glue `( f args; [ $? -eq 1 ] ) || cmd`. Author negative contract
  (prose, "judgment-tier till linted"): no `!`, no `|| true`, mind pipeline tails.
- Static vouch semantics: reached path terminating in a real command = vouch; terminals
  `return N`/`true`/`false`/`:` = DECLINE, undifferentiated (24C:222 "`return N`/`false`/`:`/`true`
  now DECLINE"; cli/main.rs:1808-1813). Runtime guard, meanwhile, reads the function's live rc.
- Strip discipline (settled law): annotations removed; bare-mark statements deleted whole; the
  author's last substantive command must remain last status-affecting statement (trailing-`:`
  clobber hazard, caught + ruled); binder lines REWRITTEN to plain assignments (`pkg="$1"` in
  expected.out:13) — i.e., the strip is a transpiler, matching 17O F-OFFRAMP's own wording
  ("a correctness-critical source-to-source transpiler, not a regex strip").
- Env/reporting: `$DORC_REPORT` (USER_STORY:353,357), `$DORC_VERDICT` lane (spike/CLAUDE.md:136),
  `DORC_LOG`, `DORC_FLAGS`/`DORC_EXIT` (harness) — DORC_ prefix consistent. Probe artifacts use
  `_rc`/`_e` scratch vars; oracle bodies' own vars (`verb`, `pkg`, `dest`) share one top-level
  probe shell across all oracles (expected.out:9-25) — benign today (re-assigned at entry),
  uncontracted for authors. Guard INVOCATIONS are subshell-wrapped `( f args ) || cmd` — book
  variable namespace protected (good). Preamble function DEFS are top-level (shadowable).
- Harness reality: `dash -n` gates RENDERED artifacts only (run.sh:124-132); authored oracles are
  never dash-checked anywhere. Fixture books are plain sh; no fixture exercises oracle-in-book-file
  (the one dotted match in a book.sh is comment text — inverted-vouch book.sh:4).

## Load-bearing tensions/contradictions observed (worked into findings)

- KNOBS kTYANNOT (KNOBS.md:58-64): "de-facto kTYANNOT-inline; the formal weld is human-reserved...
  resolved by construction: the inline dialect... is stamped and implemented"; containment claim
  "annotations live only in *oracle bodies* — books stay verbatim-runnable". vs USER_STORY:233-234
  "They append to the book's own file — oracles and runbooks can share a file" (settled stage 3).
- spike/CLAUDE.md:179-181 strip law `name_predict` (single _) vs goldens/comments/code `__` +
  hyphen-munge (expected.out:31-33; plan/lib.rs:708). USER_STORY prose `foobar_is_converged`
  (:253) vs USER_STORY renders `foobar_check`/`systemctl_check`/`file_check`/`ufw_check`
  (:170,287,414-421 — renders illustrative per header, but four spellings circulate).
- USER_STORY:267-270 "The exit-status partition is fixed and blessed: 0 = the named sense holds..."
  + `*) return 2` presented as "the native decline" — vs return-any-N = static decline (24C).
  A shell-literate author will read the partition as licensing `return 0` = yes.
- USER_STORY stage-3 minimal oracle (:236-245): no arity gate; license semantics are per-invocation-
  taken-whole (ORACLE_PROVIDES provides-convergence); stage-4 (:353,374-375) retrofits the gate and
  names the hazard ("a probe that quietly checked only the first operand"). Fixture predict()s carry
  the gate (apt.oracle.sh:11 with comment); fixture verdict fns do NOT (apt.oracle.sh:20-28) —
  saved there by the witness conjunction (no probe ⇒ no witness), but a verdict-ONLY oracle (the
  blessed stage-3 authoring order, rul-role-split "is_*verged() first") has no gate anywhere and the
  verdict body doubles as the probe source ("default-assumed home for fact-establishes").
- 15x-strawmen/README.md:26-28: "deploy-widget.sh — careful engineer's script... drives Dorc to ⊤.
  The proof that good structure removes every cheap inference handle" — their own recorded
  value-inversion vs code quality; kSILO (KNOBS:189-195) registers the sociological half.
- 17O = prior adversarial crosscheck; F-OFFRAMP verified the raw-execution hazards live (dash abort;
  bash SILENT corruption for the local-binder form); human ruling created kTYANNOT. Carve-out list
  17O:144-148 says don't re-report those as new.
- Banned-word drift, trivial: probe artifact emits `# site:1 skip-unresolvable`
  (guard23 expected.out:26) though 'skip' is a banned design word (spike/CLAUDE.md:137).
- No user-facing strip/off-ramp tool exists or is planned post-inline-stamp (grep: only 17O
  F-ESCAPENAME, which deferred it contingent on the eol pole that then lost by construction).
- No interpretation-versioning/verdict-stability policy anywhere (grep across docs; inv-top-reject's
  trigger-set explicitly shrinks release-over-release — spike/CLAUDE.md:277-286).
- Kind naming: no convention normed at the surface (USER_STORY mints ad-hoc `fb.` prefix :240;
  base vocabulary is bare `package`/`service`/`pkgindex`/`file`); 24G §4(d) mentions "reverse-DNS
  kind names" in passing; duplicate-RESOLVER = loud refuse (24G §2); cross-kind co-reference =
  acknowledged residual (ORACLE_PROVIDES grounding-bridges; 24C strain-coreference-crosskind).
- R2-CONTEXT human disposition (17O:273-278): sudo/become "almost certainly needs first-class,
  baked-into-the-language handling later" — deferred pre-first-user.
