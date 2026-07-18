> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, NEUTRAL stance (24Ka): verbatim extract from commit 19df800 on branch ai/24Ka-langreview. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Ka — Findings & cleared ledger (the language review)

Instrument: the 24 pre-registered lessons (24Ka-...-03). Every finding carries severity ·
confidence · lesson-tie · citations · the kill-attempt run against it and why it survived.
Certainty markers per house style. "Strawman-evaporation" = the expected defense that a
spelling is labeled strawman-tier; a finding survives it only per lesson 10's test (load-
bearing already, or of the class that freezes at first external author).

## FINDINGS

### finding-compiled-dialect — the oracle language is a compiled language wearing sh's clothes
Severity HIGH · confidence HIGH · lessons 2, 7, 1 (+ 19, 3 as sub-costs)

Statement: every "it's still just sh" claim about oracles is true only POST-STRIP; the
authored form (dotted/hyphenated fnames; `dest : fb.Certs = "$1"` binds; trailing `: kind`
marks; `:?` observe-marks; bare ACK/POISON mark-lines) is not executable sh — dash/ash
parse-die on the fnames (my empirics, ledger main-6; team's own 17O F-OFFRAMP), bash
silently corrupts (binds hit rc-127; trailing marks append LIVE ARGV to real commands;
`printf ... : service` corrupts the emitted data stream via format-reuse). The strip is a
correctness-critical source-to-source compiler (17N's own words), implemented engine-side
(plan/tests/erasability.rs), with name-mangling (`apt-get.is_converged` ->
`apt_get__is_converged`, per goldens).

The self-contradiction chain, citable:
- DESIGN.md:75-76 — off-ramp = `ssh host 'dash -s' <myscript.dorc.sh` ("absolutely trivial").
- DESIGN.md:490-493 — oracle and runbook "*must* be intermixable in the same file".
- USER_STORY.md:233-235 — stage 3 blesses appending the oracle to the book's own file. Doing
  so makes THE BOOK dash-unrunnable (parse death at the dotted fname — before any book line
  if prepended; rc=2 tail-death if appended).
- IMPLEMENTATION.md:364 — "we hard-avoid transpilation" (the authorship principle).
- KNOBS kTYANNOT — "not inert under stock shells (aborts or silently corrupts; verified,
  17O F-OFFRAMP) ... demands a correctness-critical strip pass".
- 17N top-paragraph — "17N itself treats 'breaks the off-ramp' as a welded kill (kill-8) —
  so this spelling fails 17N's own bar."
- 17N candidate-spellings note — the pivot moment: the dash -n objection declared "moot once
  Dorc owns its parser" — conflating "Dorc accepts it" with "it is still sh." The off-ramp
  was never Dorc's parser; it is everyone else's.

Kill-attempts run:
1. "Strawman-tier, will respell": FAILS to kill. kTYANNOT is "de-facto inline; formal weld
   human-reserved"; TODO-ADDTL:21 "stamped-in-practice with the strip pass paying the
   off-ramp"; the census corpus uses it at volume. This is lesson-10's exact freeze pattern —
   and the brief says stdlib + first real user are imminent.
2. "The containments price it": PARTIAL. (a) "books stay verbatim-runnable" is defeated by
   the blessed share-a-file flow (above); (b) "oracles already require the rename for their
   dotted names" is true but circular — the dotted name IS the first non-sh choice; it
   cannot justify itself.
3. "The emitted artifacts are dash-gated, so the product is safe": TRUE and credited
   (cleared-harness) — but it addresses the artifact lane, not the authored-file lane, the
   publication lane ("Publishing it is pushing a file to a repo" — the repo file is the
   non-sh form), or the ecosystem-tool lane (shellcheck/shfmt/highlighters see authored
   files; could not test locally, no shellcheck on box — ~SUSPECT parse-error under sh mode,
   accepts-with-warnings under bash mode).
4. "kLANG welds sh-is-the-product, out of scope": NO — kLANG welds the SUBSTRATE choice;
   this finding is about the dialect quietly ceasing to BE that substrate for one of its two
   audiences. It is the in-scope half.
Census corroboration (2026-07-05, -06 note): oracle files with ZERO dialect constructs = 0
of 187 — no sh-only authoring path exists for oracles in practice; 189 binds + 307
establish-marks + 140 negations + 30 observe-marks at volume. Books: 153/154 pure POSIX
(the one exception is itself a STRIPPED re-ingested artifact), so book-side erasability
holds at volume exactly as long as share-a-file stays un-exercised.

The unforced-trade observation (the sharpest lever, offered into the open dq-kOOB /
kTYANNOT weld): the kOOB redline's own text (KNOBS:55) bans "sidecar *configuration* — no
YAML, no frontmatter, no pragma, no comment-parsing" while explicitly allowing "out-of-band
*metadata*". A type/kind annotation is arguably METADATA (it configures nothing; it
describes an operand; it is erased without behavior change — that is the definition of the
strip). Under that reading, eol-comment annotations (`dest="$1" # : fb.Certs` — same line,
same locality, inert under every shell ever shipped) are redline-CLEAN, and the entire
off-ramp cost of the inline pole is an unforced trade. Every successful
annotate-an-entrenched-substrate system in the phase-1 ledger chose the inert channel
(ShellCheck directives [main-5], Sorbet sigils [grad-8], PEP 484 type-comments [grad-3],
TC39 types-as-comments [grad-20]); the one system that chose non-inert syntax (TypeScript)
accepted being a compiled language and paid Microsoft-scale tooling to make that cheap
[main-1, grad-18]. Dorc is currently choosing the TS architecture while writing the
ShellCheck marketing.
Fallback if inline stays (their call to make): stop claiming identity — say "compiled
superset, trivial one-command off-ramp", ship `dorc strip` as a stable first-class tool,
un-bless share-a-file for dash-targeted books (or auto-strip on read), and add the
ecosystem-tool story (a shellcheck plugin/pre-strip shim) to the DX budget.

### finding-nounset-idioms — flagship examples teach set -u-fatal spellings
Severity LOW-MED (downgraded from MED after the guard23 kill-attempt below) · confidence
HIGH · lessons 1, 22 (+2)

Statement: USER_STORY's battle-ready oracle (USER_STORY.md:353) teaches `[ "$2" = "" ]` as
the arity-gate; under `set -u` (dash) this kills the calling script precisely when the
invocation is well-formed (one operand) — verified empirically (ledger main-empirics,
2026-07-05): direct call under `set -u` = "2: parameter not set", script death; the
engine's subshell-guard shape safely falls through (rc 2 -> run). The same file teaches
`>>"$DORC_REPORT"` (USER_STORY.md:353,357) with no default — with DORC_REPORT unset (any
raw / non-Dorc context): set -u = fatal; otherwise redirect-to-"" fails the refusal
breadcrumb. Blessed spellings should be `[ "${2-}" = "" ]` (or `[ $# -le 1 ]`) and
`>>"${DORC_REPORT:-/dev/null}"`.
Kill-attempts: (1) "engine lanes are tested for nounset" — STRONGER than I first credited:
guard23-nounset-book-survives documents this exact hazard as 23C-fd2 / "the 218a set-u
hazard", names the fragile spelling "the corpus-standard check body", and human ruling h3
chose mechanism-neutral mitigation ("check-body `${2:-}` hygiene OR subshell-wrap, the
engine's choice") — the subshell won; the emitted guard survives. So the ENGINE lane is
closed by ruling. (2) "books own set -u, oracles run in harnesses" — mooted by (1) for the
in-Dorc lanes. RESIDUAL that survives: h3 fixed the insertion, not the authored corpus —
USER_STORY still teaches `[ "$2" = "" ]` (and unset `$DORC_REPORT`) as the blessed
spelling, and those files are also sold as directly-callable libraries ("documentation that
executes", USER_STORY:459) where a set -u caller dies on the WELL-FORMED path. `"${2-}"` /
`"${DORC_REPORT:-/dev/null}"` cost nothing and close the reuse+teaching lanes. Scoped
LOW-MED.
Census corroboration: the fragile gate is UNIVERSAL — `[ "$2" = "" ]` in 133 files; `${2-}`
and `$#` variants: ZERO in e2e. Sharper: the OLDER 15x strawmen used `[ "$#" -le 2 ]` (the
set-u-safe form) — the corpus REGRESSED from the safe variant to the fragile one as the
dialect firmed. And `DORC_REPORT` appears in NO fixture (USER_STORY-only protocol; the
UNK-breadcrumb idiom has never been exercised in the corpus) — meanwhile the 17x adversarial
strawman used the defensive `${DORC_SCRATCH:?}` form for ITS lanes, so the safe spelling was
known in-house.

### finding-mark-polysemy — one trailing-mark syntax, role-dependent meanings
Severity MED-HIGH · confidence HIGH · lesson 5 (+24)

Statement: the trailing `: ...` mark means ESTABLISH in predict() bodies, EMISSION-TYPING in
reaches() bodies (24G §4: "a trailing mark means establish in predict(), emission-typing in
reaches() — coherent, but a language decision to document loudly"); `:?` marks OBSERVE; `!`
suffix on properties inverts polarity (`package:"$pkg".installed!`); bare `: ...` lines are
ACK/POISON statements "equivalent to a comment, NOT a POSIX : command" (spike/CLAUDE.md
strip-fidelity) — i.e. the same leading token is a command in sh, a deletion-target in the
dialect. The newest fixtures add valueless binds (`v : otelcol`) and empty-entity
singletons (`otelcol:.v0155`, version-literal-as-property). One surface shape, at least
five semantic roles selected by body-role and by micro-grammar; Odersky's implicit is the
canonical recorded regret for exactly this (mechanism-over-intent, [main-8]).
Kill-attempts: (1) "role-scoped: the reader knows which function body they are in" —
PARTIAL: true for careful reading; fails for copy-paste across roles (a line moved from
predict() to reaches() silently changes meaning — the false-friend-within-the-dialect) and
for grep/tooling (no textual distinction). (2) "strawman-evaporation" — the mark grammar is
"settled-strawman" (ORACLE_PROVIDES provides-binding) and the role-polysemy is a design
DECISION (24G) not a placeholder; survives. Recommendation shape: distinct mark-heads per
intent (e.g. establish vs emit-type vs observe spelled differently) before the vocabulary
closes; the team already owns the precedent (killing the tilde for exactly this class of
reason).
Census corroboration: family sizes — establish 307 (negated 140), observe 30, value-binds
180, valueless binds 9, bare-kind emission-marks 2. Two additions: (a) ACK and POISON
bare-marks survive in LAW (spike/CLAUDE.md strip-fidelity) with ZERO corpus occurrences —
unexercised grammar, cheap to prune now or it ships untested; (b) the SAME command
(`command -v`) is marked establish (` : `) in one fixture family and observe (`:?`) in
others — polarity for identical probes is currently author-taste, one more
same-intent-two-spellings axis.

### finding-license-in-a-name — the liability act has no spelling and three tiers share a grammar
Severity MED · confidence MED-HIGH · lessons 6, 14 (+ boundary-note)

Statement: the most consequential act in the language — accepting skip-liability — is
currently spelled as a SIDE-EFFECT of naming a function `is_converged` (rul24-vouch-is-
verdict-authoring; USER_STORY:253-259 "the function's name is the license"). Three liability
tiers coexist (predict = never licenses; is_converged = licenses guard+elide; touches =
traveled at-most claim endangering OTHERS' lines) with no naming-grammar marking the tier
(is_/predicate, imperative, 3rd-person verb — inconsistent). 24G §6's own principle ("the
function name is nearly the entire contract surface") argues the grammar should carry it.
kCONTRACT-RUNGS is OPEN and this is evidence into it: the single-act weld is
checked-exceptions-shaped [reg-10] — coupling information-provision to liability forces
authors to withhold information (don't publish the verdict-function) or over-accept; Sorbet's
ladder [grad-8] and the wary-engineer pressure (ORACLE_PROVIDES provides-license) both point
at a separable, per-path rung-spelling. Boundary-note: WHAT a vouch means is settled
machinery (out of scope); HOW rung-selection is spelled, and whether names encode tier, is
squarely language.
Kill-attempts: (1) "five names is a small closed set; docs carry it" — partial: today yes;
the family is declared open-by-name (TODO-ADDTL:21 "extends by name, never by re-reading an
existing member" — good), which makes the grammar question COMPOUND with each addition;
survives as advice-now-cheap. (2) "names were individually well-chosen" — TRUE (reaches vs
manifest analysis is excellent, cleared-naming) — individual quality is not grammar.

### finding-kind-namespace — the composition anchor has no ownership/evolution story
Severity MED · confidence HIGH · lessons 21, 15

Statement: kinds are the cross-author composition anchor (footprint x backing intersection
compares kind:entity coordinates by string), yet: no namespacing rule (fixtures mix bare
stdlib kinds `package`/`file`/`service`/`pkgindex`, author-prefixed `fb.Certs`, plumbing
kinds `grepmatch`); 17N C2's reverse-DNS lean (`net.example.wombat`) appears nowhere in the
wild corpus; "Nobody approves kind names; there is no registry" (USER_STORY:260-262) is the
declared posture while stage-5-7 makes silent same-name collision the exact silent-
under-execute channel; kind collision + evolution semantics are acknowledged open
(TODO-ADDTL:21). DefinitelyTyped/typeshed drift [grad-4, grad-18] is the recorded outcome of
unversioned shared vocabularies.
Kill-attempts: (1) "duplicate-kind = refuse-both exists" (24G §2) — that handles duplicate
RESOLVERS within one analysis unit, not same-name-different-meaning across lineages (17N's
own Seam names this: "silent same-name-different-meaning never conflicts"); survives.
(2) "stdlib will de-facto own bare kinds" — plausible convention, unwritten; the cheap fix
is writing it (bare = stdlib-reserved; authors prefix) before external authors mint.
(3) strawman-evaporation — kinds are the ONE construct explicitly named "strawman-frozen
pending dq-kOOB" (spelling) but their SEMANTIC namespace outlives any respelling; survives.

### finding-no-epoch — zero version/epoch machinery at the artifact layer
Severity MED · confidence HIGH · lessons 9, 11

Statement: no artifact carries a dialect-version marker (grep: no dorc-version/edition/
dialect/epoch anywhere in spike/); the recognized-idiom set (what the analyzer rewards) is
enumerated (ORACLE_PROVIDES — credit) but unversioned; analyzer-verdict churn across
releases (TS's no-semver lesson, [grad-17]) has no stated policy. Ramey's one recorded wish
is compat-levels-earlier [shell-8]; Cox's is compat-machined-not-promised [reg-4]. The
timing is exactly lesson-9's cheap moment (pre-external-author).
Kill-attempts: (1) "books are bare sh, nothing to version" — TRUE for books (and right);
the oracle dialect + mark grammar + role vocabulary + kind namespace are the versionable
surfaces. (2) "the kOOB redline complicates a version pragma" — noted: a version marker is
metadata-not-config under the redline's own distinction, and several sh-native spellings
exist (a no-op function definition; a bare mark; a naming convention on the file). The
knot is designable; absence is the finding. (3) "e2e goldens are the compat gate" — they
gate the ENGINE against ITS corpus; nothing gates future engines against PUBLISHED oracle
artifacts (the direction that freezes). Survives MED.

### finding-migration-debt — respell-labels without codemods, already accruing
Severity MED · confidence MED-HIGH · lesson 10

Statement: the corpus already carries two naming/format strata under migration: predict
spelled `apt_get__predict` (pre-mangled legacy) beside dotted `apt-get.is_converged` in the
SAME fixture files (e.g. cases/converged/package.oracle.sh:6,20); touches()'s stringly
`kind:entity` + `| sed 's|^|file:|'` emission is flagged "should not be imitated"
(USER_STORY stage-5 FIXME) with typed emission "sequenced LAST" (24G §7) — no codemod story
for either. Lesson-10's test: a strawman label protects only what has a mechanical
migration path. Feldman's dozen users [reg-9] arrive with the stdlib.
Census numbers: ONE role, TWO spellings at 185 (`X__predict`) vs 4 (`X.predict`); 71 of 187
oracle files mix the schemes internally; 114 are pure-legacy. The migration is not
approaching — it has not begun, and the legacy spelling is the 97% majority.
Kill-attempts: (1) "they already respelled once cleanly (tilde death)" — TRUE (credit,
cleared-respell) but that was a mark DELETION during the same round, not a cross-corpus
rewrite; the two live strata are evidence the harder kind accrues. (2) "goldens churn freely
per the human's ruling" — that ruling is about ENGINE goldens, not published oracle files.
Survives MED, cheap to fix now (write the rename tool with the rename).

### finding-flagstrip-ceremony — the taught argparse idiom is ceremony that mis-parses
Severity LOW-MED · confidence HIGH · lessons 13, 22 (+ the team's own cargo-cult razor)

Statement: `while [ "${1#-}" != "$1" ]; do shift; done` appears 405 times across 135 files
(census: 4 per fully-guarded oracle — twice per role-function); it is (a) pure ceremony in most
bodies (the razor's 70%-danger-zone class, 24G §8: cargo-cults to 100%, no signal, full
cost) and (b) wrong for flag-with-separate-value invocations (`apt-get -o Opt val install
nginx` strips `-o`, then reads `Opt` as the VERB). The provides-decoding trust-shape
("resolves nothing => runs", ORACLE_PROVIDES) makes the mis-parse SELF-DEFEATING-SAFE
(wrong verb -> no case-arm -> no claim -> run), which caps severity — but at-volume
teaching of a known-wrong argparse idiom is lesson-22's demo-default trap, and 151-X4's
own lesson ("apt-get -o probe-mutation") is adjacent.
Kill-attempts: (1) "safe direction by construction" — TRUE, which is why LOW-MED not
HIGH; the residual is lost coverage + author bewilderment when a flagged invocation
mysteriously never matches, plus one contrived unsafe shape (a value that IS a verb name).
(2) "engine gate-5 cross-checks argv against dash ground truth" — engine-side only; the
authored idiom is user-side. Survives LOW-MED; candidate fixes: a blessed
`--`-aware/value-flag-aware decode idiom in the stdlib, or an engine-supplied decode
helper the oracle calls (keys into the same escalation machinery).

### finding-law-code-drift — the mangling law disagrees with the implementation
Severity LOW · confidence HIGH · lesson 11 (recognizer-is-API, the spec half)

Statement: spike/CLAUDE.md rul-ternary-verdict says strip = "`name.predict()` ->
`name_predict()`" (single underscore); the emitted goldens say "`name.is_converged()` ->
`name__is_converged()`" (double underscore; e.g. cases/guard23-fallthrough-canttell-runs/
expected.out:28-30) and silently mangle hyphens (`apt-get` -> `apt_get`). Also the mangle
is non-injective (`apt-get.x` and `apt_get.x` collide at `apt_get__x`; a user function
already named `apt_get__is_converged` collides with the emitted namespace — nothing
reserves `*__role` names, lesson 21-adjacent).
Kill-attempt: trivial-fix defense — yes, and that is the point of recording it now: the
mangling scheme is exactly the kind of accident (lesson 22) that freezes into "the"
scheme; one paragraph in the rulings doc + a reserved-namespace note closes it.

### finding-emitted-cryptic-collapse — the collapsed-pipe replacement artifact is opaque
Severity LOW · confidence MED · lesson 16

Statement: full-pipe collapse emits `: | true || : | :` (strawman24-pipe-guard-oracle-
converged book.sh header describes the render) — structure-preserving but unreadable
without the tool's explanation; the plan's per-line reason comments (credit) mitigate, but
the replacement grammar itself has no legend in any human doc.
Kill-attempt: "renders are illustrative/not settled" (USER_STORY header) — partially
evaporates; kept LOW as a watch-item for the render-format round: every machine-substituted
line should carry its reason inline (they mostly do today).

### finding-two-declines — one intent, two coexisting decline spellings
Severity LOW-MED · confidence HIGH · lesson 20

Statement (census-sourced): identical decline semantics are spelled two ways across the
corpus — explicit `*) return 2 ;;` (53 files, the 24D elide-weld family) versus implicit
unhandled-path/no-default-arm (the guard23 ternary family, whose own fixture comment
declares declines "spelled as unhandled paths"). rul24-vouch-is-verdict-authoring blesses
BOTH ("declining per-path = return 2 / unhandled path"), so the recognizer must forever
accept both, authors will argue about which is idiomatic, and diffs between the two carry
zero semantic content. The blessing of both is one sentence old; picking one (or naming the
implicit form as the beginner-visible one and the explicit as the lint-preferred) is cheap
now.
Kill-attempts: (1) "they are genuinely different acts (deliberate decline vs never-thought-
about-it)" — the STRONGEST counter and partially survives the finding: an explicit
return-2 arm CAN carry a breadcrumb and reads as authored intent; an unhandled path is
indistinguishable from an oversight. That distinction is exactly why one blessed form
should exist for "deliberate decline" (the explicit arm) with the implicit path defined as
identical-but-unattested; today the docs bless them as synonyms. (2) strawman-evaporation
— the partition itself is welded (rul-rc-partition), only the decline-arm spelling is at
issue; survives as LOW-MED advice.

## Boundary-notes (language flaws tracing to settled semantics; stated, not relitigated)

bn-1: The ternary rc-partition is welded and WELL-chosen (grep/diff/cmp family); but note
  the author-side negative contract it demands ("never collapse statuses out of a
  verdict-function — no !, no || true, mind pipeline tails", rul-rc-partition) is exactly
  the author-discipline class the team's own evidence says fails (151 X4, twice) — the lint
  is named in the backlog; until it exists the partition's edge is soft. (Lesson 4/18.)
bn-2: Silence-is-wall (silence licenses nothing) is the right Dialyzer-polarity and is held
  consistently — the cost (one stale index collapses the book's tail, USER_STORY stage 5
  opening) is the priced consequence of the semantics, not a language flaw.

## CLEARED — suspicions checked and withdrawn, and places the accretion landed right

cleared-rc-partition (L7, L8): the 0/1/>=2 partition mirrors the existing sh convention
  family (grep/diff/cmp/test); crash-statuses (126/127/130) land in the safe >=2 cell;
  consumed at a protocol boundary (engine + emitted glue), not ambient control flow. My
  pre-registered rc-context-zoo concern is answered BY DESIGN: the consuming-context
  taxonomy exists and is rigorous (inv-one-observable's StatusRelaxable / StatusInvariant /
  StatusIterated; while-condition blocks unconditionally; || true = consumed-in-form-dead-
  in-fact). This is lesson-8 done properly, better than I expected to find.
cleared-guard-shape (L8, L16): `( check ) || <original bytes>` — subshell isolation;
  errexit-exemption by construction; nounset-in-book tested (guard23-nounset-book-survives);
  bytes survive verbatim (no code path removes them); never engine-synthesized sh; the
  declared-dual glue (`( f_is_diverged args; [ $? -eq 1 ] ) || bytes`) is a lossless
  mechanical inversion. Robust and idiomatic — reads as the check-then-execute a human
  writes.
cleared-erasability-of-books (L2, L3): books are plain sh end-to-end — census: 153/154
  books contain zero dialect constructs, and the single exception is a re-ingested STRIPPED
  artifact (plain assignments, no marks), i.e. the exception PROVES the strip discipline;
  the admin's own hand-written guard is first-class lifted material (USER_STORY:120-124);
  paste-survival of book lines is perfect; the no-double-guard rule + kSILO mitigations
  directly answer my two-audience silo worry (L14) with named machinery.
cleared-never-lie (L4): silence-licenses-nothing, top-reject-loudly (inv-top-reject),
  means-nothing-not-refusal for unannotated emission (24G §5), hard-errors = syntax + true
  contradictions ONLY, can't-say always runs, no analysis-confidence threshold ever makes a
  probe safe. The Dialyzer trust-contract is honored across the whole surface, and 17N
  shows they understood WHY Dorc cannot copy Dialyzer's polarity blindly (no uniform
  runtime backstop; spine-1 makes the oracle declare the direction).
cleared-ladder (L14): the stage-0..7 ramp with per-rung pricing; two named function-families
  (per-TOOL / per-KIND) with monotone degradation ratified (rul24-threefunc-monotonic);
  <10%-of-authors honesty about kind-owner features. The two-audience architecture is
  genuinely good; my pre-registered falsifier ("name the dimension where audiences pull
  apart") is answered by kSILO's own text.
cleared-feature-ledger (L11): ORACLE_PROVIDES is the enumerated, statused,
  tombstone-disciplined semantic feature-list my lesson 11 demands (versioning excepted —
  see finding-no-epoch).
cleared-naming-theory (L6): 24G §6's name-as-contract + omission-bias principles
  (reaches-not-manifest; resolve's merge-bias = its safe direction) are sophisticated,
  falsifiable naming doctrine — better than most real languages document.
cleared-anti-freeze (L10 partial): sm.dorc.* invalid-TLD bootstrap names; the tilde
  vouch-mark killed cleanly pre-users; XFAIL/two-sided-pin promotion discipline in the
  harness. The respell muscle exists (see finding-migration-debt for its limit).
cleared-refusal-idiom (L12): `*) return 2` decline-as-ordinary-control-flow is visible,
  per-path, greppable; UNK breadcrumbs carry reasons out-of-band (modulo the DORC_REPORT
  wart in finding-nounset-idioms).
cleared-harness (L1): ap-2 dash -n gate on every rendered artifact; gate-5 uses dash as
  the SEMANTIC ORACLE for value-flow (executable spec-testing, exactly Oils' lesson
  [shell-2]); env -i determinism rail; mocks-only exec. The EMITTED side of the substrate
  envelope is machine-enforced today.
cleared-substrate-choices (L1): `local`-expected (the de-facto-universal extension class,
  my main-9 empirics) + dash-as-reference-implementation lean matches the evidence; the
  "two flavors of not-POSIX" distinction is implicitly understood (they test against dash,
  not against the spec text).
cleared-no-reorder (simple mental model): book order is sacred; apply-speed comes from
  elision only. Teachable in one sentence.
withdrawn-inline-env-binds: my pre-corpus guess that "inline binds" meant `VAR=x cmd`
  prefix-assignments (with their ksh93 persistence variance) was wrong; the dialect's binds
  are the typed `x : Kind = v` form. The prefix-assignment minefield is not implicated.
withdrawn-bang-collision: the `!` polarity suffix rides INSIDE property names
  (`.installed!`), positionally distinct from sh's prefix-`!` pipeline negation; kept as a
  note under finding-mark-polysemy, not its own finding.
withdrawn-verdict-ambient-consumption: I suspected the rc-partition would leak into ambient
  sh contexts (set -e zoo); it does not — engine-consumed + emitted-glue-consumed only;
  guard glue is errexit/nounset-robust (cleared-guard-shape).
