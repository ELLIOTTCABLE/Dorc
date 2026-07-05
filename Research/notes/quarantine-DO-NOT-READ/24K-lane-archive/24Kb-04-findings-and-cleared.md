> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, ADVERSARIAL stance (24Kb): verbatim extract from commit fd5fa82 on branch worktree-agent-abd7ff8be88067e1b. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Kb-04 — findings (kill-attempted) + cleared ledger

Method: every finding was attacked before acceptance — against the strawman shield (USER_STORY's
own illustrative/strawman flags; ORACLE_PROVIDES per-entry spelling status), against "already
known and priced" (17O carve-outs; KNOBS registry), and against the safe-direction defense.
Findings below live in the SETTLED layer (welded rulings, settled stages 0-4, law text, fixtures)
or in the compat-permanent class, unless marked otherwise. Lesson numbers = 24Kb-02.

## finding-authored-form (severity HIGH · confidence HIGH · lessons 11, 18, 06, 01)

The settled dialect forfeits sh-nativeness of the AUTHORED artifact — dotted fnames are a dash/ash
parse error (verified; dash rc=2 "Bad function name", also under -n), binder lines are rc=127 under
bash, trailing marks execute as extra argv (a raw-run authored apt probe silently answers "absent"
because `:` and the mark become dpkg-query package args — apt.oracle.sh:13) — while the settled
surface story still sells sh-nativeness: README:6 "pure, idiomatic POSIX-sh"; DESIGN:75-81 trivial
off-ramp; USER_STORY:379 "Still just sh: stripped, it runs on any POSIX box"; USER_STORY:459
"documentation that executes". The reconciliation on record (KNOBS kTYANNOT:62 containment —
"annotations live only in oracle bodies — books stay verbatim-runnable") is CONTRADICTED by settled
USER_STORY:233-234 ("oracles and runbooks can share a file") — one of the two settled artifacts is
false. The whole position rests on a strip transpiler that their own 17O calls "correctness-critical
source-to-source", which today has: no user-facing tool (F-ESCAPENAME deferred contingent on the
pole that lost), no versioning story, and an inconsistently-specified transform across settled
artifacts (law text `name_predict` single-underscore, spike/CLAUDE.md:180; code+goldens
`apt_get__is_converged` double + hyphen-munge, plan/lib.rs:708, guard23 expected.out:31-33; prose
`foobar_is_converged` USER_STORY:253; renders `foobar_check` USER_STORY:170).
KILL-ATTEMPTS: (a) "known since 17O, priced into kTYANNOT" — the pricing predates both the
share-a-file promise and the de-facto stamp; the containment premise that "cheapened the trade" is
contradicted in-corpus; the price was never re-summed cumulatively (each non-sh feature priced
against the previous one's sunk cost: the dot justified the rename; inline marks then "cheap because
the rename is already required" — KNOBS:62). (b) strawman shield — the dot + strip discipline are
settled law (rul-role-split; rul-ternary-verdict; USER_STORY:258 "Spelling settled 2026-07-03");
the annotation SYLLABLE is open (dq-kOOB residual) but the two-surface structure is stamped.
(c) "authors only meet stripped forms" — authors WRITE the authored form: they cannot `sh -n`,
shellcheck, source-at-a-REPL, or unit-test their own oracle file with any standard tool; the
project's own e2e gate dash-checks only rendered artifacts (run.sh:124-132), so even in-house the
authored dialect has no syntax gate. SURVIVES.
Constructive edge: either kill share-a-file (restore containment) or ship + version a first-class
`dorc strip`/fmt round-trip and rewrite the outward claims to "compiles to plain sh" honesty
(TypeScript's posture, [typescript-design-goals]) before the first library author onboards.

## finding-return-decline (severity MED-HIGH · confidence HIGH mechanics / MED impact · lessons 05, 22, 14, 03)

Two runtime-identical spellings carry OPPOSITE static meanings: a verdict arm ending in a probe
command = vouch; the same arm ending `if probe; then return 0; else return 1; fi` (or any
`return N`/`true`/`false`/`:` terminal) = DECLINE, undifferentiated (24C:222; cli/main.rs:1808-13).
Under every shell these are indistinguishable at runtime — and the shipped guard DOES read the live
rc — so the dialect's static reader and its own runtime artifact disagree about the author's text.
USER_STORY:267-270 actively teaches the misreading: "the exit-status partition is fixed and
blessed: 0 = the named sense holds" invites `return 0` as the yes-spelling; the blessed decline
`*) return 2` reinforces that the function's rc is what's read. An author who writes the
mainstream-idiomatic explicit-return style ships a silently-inert oracle (every path declines; no
elision, no guard, no error).
KILL-ATTEMPTS: (a) "declines fail safe" — correctness yes; adoption no: silent-value-zero is the
Dialyzer-trust lesson inverted, and the doc/behavior mismatch is a standing bug-report generator.
(b) "the lint will come" — none exists; declines are sanctioned non-diagnostics (a legitimate
decline and a misread yes are indistinguishable to any future lint without intent). (c) "24C fixed
the real bug" — it fixed wrong-VOUCH (soundness); it created undifferentiated-decline (silent
ergonomics) and left the welded partition text implying the opposite. SURVIVES. Cheap fix now:
differentiate `return 0/1` (model as the answer, or loud-refuse them) and say "the answer is the
last command's status, never an explicit return" in USER_STORY stage 3 — one sentence.

## finding-resolved-by-construction (severity HIGH (process) · confidence HIGH · lessons 06, 15)

The design's own gate for the most user-facing language decision was bypassed by implementation
momentum, and the registry records it: KNOBS kTYANNOT:62 "de-facto kTYANNOT-inline; the formal weld
is human-reserved. The upstream gate (dq-kOOB...) resolved by construction: the inline dialect...is
stamped and implemented." DESIGN.md:336-337 (human) rates the language axis "most critical when
calibrated against how locked-in any decision becomes"; DESIGN:592 leaves the typed-sh section
"(UNFINISHED)". The reserved decision's losing pole carried the off-ramp; it lost without the human
ever typing the weld — at the exact pre-first-user moment where mistakes acquire tenure
([feldman-make-tabs-email]).
KILL-ATTEMPTS: (a) "the registry disclosing it IS the discipline working" — disclosure without
re-decision is a post-hoc label; the knob's own text now argues the fait accompli ("two containments
cheapen the trade", one of which finding-authored-form shows is contradicted in-corpus). (b) "spike
code is disposable, so nothing is really settled" — the rulings layer (spike/CLAUDE.md, USER_STORY
settled flags, ORACLE_PROVIDES statuses) is explicitly design-truth, not spike-internal; the dot is
in that layer. SURVIVES as the process finding management should hear: the accretion mechanism is
self-documented.

## finding-stage3-arity (severity MED-HIGH · confidence MED (+SURE asymmetry, ~SUSPECT exact path) · lessons 23, 09, 02)

The celebrated two-minute stage-3 oracle omits the arity gate that the contract's own semantics
require ("for this invocation, taken whole" — ORACLE_PROVIDES provides-convergence): USER_STORY
:236-245 binds `dest = "$1"` and probes it, so a colleague's `foobar sync-certs /a /b` site is
answered "yes" having checked only /a. Stage 4 (:353, :374-375) retrofits `[ "$2" = "" ]` and names
the hazard ("a probe that quietly checked only the first operand"); fixtures carry the gate in
predict() (apt.oracle.sh:11) — where the witness conjunction (no probe ⇒ no witness ⇒ run) saves
multi-operand sites — but the blessed authoring order is verdict-function-FIRST (rul-role-split:
"expected authoring order is is_*verged() first"), and a verdict-only oracle's body doubles as the
probe source with no gate anywhere on the licensing path. Under-execution is the project's own
named cardinal sin (USER_STORY:681).
KILL-ATTEMPTS: (a) "the analyzer might refuse multi-operand argv statically" — nothing in the
stage-3 body distinguishes arities; the argparse IS the authored control-flow (provides-decoding),
and it accepts any arity. (b) "multi-operand foobar is contrived" — the project's own stage-4 text
considers it real enough to gate; apt-style multi-package invocations are the bread-and-butter
shape. (c) "teaching doc ≠ law" — USER_STORY is the settled walkthrough ("every word
human-reviewed"), explicitly the shape users will meet; the on-ramp exemplar IS the de-facto
template library authors copy. SURVIVES pending one empirical check I could not run (no built
binary in this worktree): whether the verdict-lift refuses ungated multi-operand argv by some path
I haven't seen. Cheap fix: put the arity gate (or an engine-side operand-count refusal) in the
stage-3 exemplar; it costs one line against a cardinal-sin class.

## finding-colon-variance (severity MED · confidence HIGH · lessons 22, 05, 01)

The `:`-mark is accreting per-context meanings: value-binder (`pkg : package = "$1"`), trailing
establish-mark (predict bodies), emission-typing (reaches bodies — 24G §4: "a trailing mark means
establish in predict(), emission-typing in reaches() — coherent, but a language decision to
document loudly"), bare ACK/POISON statement-marks, plus retired tilde grammar; all puns on sh's
`:` no-op builtin (load-bearing: strip-fidelity exists precisely because a leftover `:` would
clobber rc — spike/CLAUDE.md:181-186). One glyph, ≥4 role-dependent readings, meaning recoverable
only from the enclosing function's name — Wall's sigil-variance shape ("trying to do too much...
I was unduly influenced" [wall-apocalypse-2]) plus CD hidden-dependencies.
KILL-ATTEMPTS: (a) strawman shield — the final syllable is open (dq-kOOB residual), so the GLYPH
may change; but the STRUCTURE (one mark family, per-role semantics) is what 24G settled and what
survives any syllable swap. (b) "the machine disambiguates fine" — the cost is reader decode + the
author's model, not the parser. SURVIVES at MED — the cheap mitigation is a written rule ("one mark
form per role" or distinct syllables per meaning) before third-party authors mint habits.

## finding-mangled-namespace (severity MED · confidence HIGH absence / MED shadowing · lessons 21, 01)

The engine injects mangled function definitions into the artifact's flat namespace
(`apt_get__is_converged` preamble; `package__resolve`; `<provider>__touches`) with: no documented
reservation rule for `*__*` or `_rc`/`_e` scratch names; no collision lint against book-defined
names (a book's own later same-name def would shadow the preamble def); and a non-injective munge
(`apt-get` and a hypothetical `apt_get` command both key `apt_get__*` — unrecorded anywhere,
grep-verified). Mitigations that exist and are good: guard INVOCATIONS are subshell-wrapped
(book variable namespace protected); DORC_* env prefix is consistent; preamble-dedup keys exist
for oracle-vs-oracle. Oracle-body variables (`verb`, `pkg`, `dest` — every oracle uses them) share
one top-level shell across all oracles in a probe artifact — benign today, uncontracted for
authors.
KILL-ATTEMPTS: (a) "collisions are vanishingly rare" — individually yes; the class is what compat
makes permanent (lesson-21's reverse collision: a FUTURE dorc minting a new suffix can capture an
existing user function silently). Reservation docs + a lint are near-free pre-first-user; they stop
being free after. (b) "renders are illustrative" — the mangle scheme is code+golden law, not
render. SURVIVES at MED.

## finding-interpretation-versioning (severity MED · confidence HIGH · lessons 10, 07, 06)

No policy exists for "same book, new dorc, different plan": inv-top-reject's trigger-set explicitly
shrinks as modeling grows (spike/CLAUDE.md:277-286), so each release converts former walls into
elisions/guards — verdict drift on unchanged books is designed-in — yet nothing pins or even names
analyzer-version sensitivity of plans ([shellcheck-directives-versioning] "avoids any surprise
build breaks when a new version with new warnings is published"; [typescript-nonsemver]).
KILL-ATTEMPTS: (a) "plans are per-run and human-approved; drift is absorbed at the ack" — the ack
absorbs attention, not automation: the first CI job that diffs plans or greps `plan: N run` freezes
the current shape (Hyrum), and admin muscle-memory is real interface. (b) "pre-1.0, everything
drifts" — exactly why a one-paragraph policy ("plans are not stable across dorc versions; here is
the changelog discipline for verdict-affecting releases") is cheap now. SURVIVES at MED.

## finding-channel-stability (severity MED-LOW now, HIGH at first scraper · confidence HIGH · lessons 07, 20)

Machine-shaped emissions have no per-channel stability declarations: probe record grammar
(`site N effect=... rc=...`), `# dorc:` artifact trailer comments, `$DORC_REPORT` `UNK` lines, the
`plan: N run, M verify, K elided` tally. rec-1 (artifact vs render planes) is the right structure —
but the artifact plane is byte-floored BY LAW while its embedded `# dorc:` grammar was never
designed as a stable format: an undesigned grammar frozen by law is the porcelain lesson inverted.
KILL-ATTEMPT: renders are flagged illustrative (USER_STORY header) — correct instinct, wrong
altitude: the .sh artifact and the OOB lanes are not renders. SURVIVES as a declare-now item.

## finding-author-burden-residue (severity MED-LOW · confidence HIGH · lessons 17, 14 — mostly-known)

The verdict-function's call-context hazards are handled where the ENGINE emits (subshell +
`||`-left; sense-flip glue; strip-fidelity) but the AUTHOR side is prose-only: "never collapse
statuses out of a verdict-function — no `!`, no `|| true`, mind pipeline tails" is judgment-tier
"till linted" (rul-rc-partition), with R2-ORTRUE (17O, verified) showing the exact failure. Known
and roadmapped; the finding is TIMING: the library contract ships to a first author before the
lint exists, and masked-rc habits baked into a published oracle are permanent.

## Timing notes (lesson-06 corollaries, not standalone findings)

- sudo/become: human-ruled "almost certainly needs first-class, baked-into-the-language handling
  later" (17O:273-278) — a known future language feature deferred past first-user; when it lands it
  will reshape oracle argv contracts (context-qualified identity).
- Kind naming convention: no norm at the surface (ad-hoc `fb.` prefix in the exemplar; bare nouns
  in the base library; reverse-DNS mentioned only in 24G §4(d)); within-kind aliasing is owned
  (resolve(), refuse-both on duplicates), cross-kind co-reference is a named residual. The cheap
  pre-publication move is a written convention; after the first published oracle it's Hyrum-frozen.

## CLEARED — suspicions checked and withdrawn; places the accretion got it right

- cleared-dash-blindspot: "nobody noticed the dot breaks dash" — FALSE; verified live in 17O
  (F-OFFRAMP), known since 15x ("As authored they also fail dash -n"), priced into kTYANNOT. The
  surviving finding is the contradiction/pricing form, not ignorance.
- cleared-rc-partition: the 0/1/≥2 partition itself is well-chosen — congruent with grep's
  0/1/>1 convention [posix-grep-exit-status]; ≥2⇒run absorbs the shell-reserved 126/127/128+n
  range in the safe direction by construction; probe-side 127 is additionally surfaced by the
  vouch-closure gate (run.sh gate-1c). Ran lesson-16's DETECT: no licensing-side collision exists.
  The declared-dual sense-flip glue is a lossless inversion. Genuinely good design.
- cleared-emitted-code-craft: the emitted-artifact layer shows real substrate mastery — guard
  invocation subshell-wrapping (book vars protected), deliberate `||`-left errexit-exemption
  (USER_STORY:198-199), the strip-fidelity trailing-`:` rc-clobber law, the redirect-suppression
  bug caught by their own emitter gates (24C:186-188), and the three-way consumed-Status split
  (Relaxable/Invariant/Iterated, spike/CLAUDE.md inv-one-observable) which is beyond most shell
  tooling's substrate model. Lesson-17's DETECT largely PASSES on the engine side.
- cleared-error-posture: 24G §5 (human-ruled): hard errors = syntax + genuine contradictions only;
  everything else smell-or-silence — Dialyzer-shaped under-claiming [dialyzer-adoption]; plus the
  cargo-cult razor (rul24-boilerplate-cargocult) actively pruning input ceremony. Lesson-14's
  DETECT passes on posture (the residue is finding-return-decline's silent-inert corner).
- cleared-naming-process-now: 24G §6's name-as-contract reasoning (bias-aware naming on
  completeness claims: `reaches` chosen over `manifest` BECAUSE omission is the dangerous
  direction) is sophisticated, recorded practice; the check→predict corpus-wide rename happened
  pre-first-user — the cheap window used as intended.
- cleared-honest-pricing: USER_STORY's bought-unsoundness section (:642-703) — the 8-condition
  conjunction, the named cardinal sin, "marketing at best, theatre at worst" — is unusually honest
  self-pricing; stage-2's monotonicity claim is properly scoped by stage-5's admission (:512-517).
  My suspicion of unscoped monotonicity overclaim: WITHDRAWN.
- cleared-two-surface-structure: rec-1's artifact-vs-render plane split is the porcelain lesson
  half-learned in advance (structure right; stability declarations missing → finding-channel-
  stability is the residue, not the structure).
- cleared-verdict-oob-conflict: round-20 "no exit code can mean 'unknown'" vs round-23 ≥2=confused
  — textual drift, semantically resolved by the tool-rc/verdict naming discipline; doc-hygiene at
  most. WITHDRAWN.
- cleared-binder-syllable: attacking the `dest : Kind = ...` SYLLABLE while it is explicitly
  strawman-frozen pending dq-kOOB would die to the shield; only the structural consequences
  (finding-authored-form) are charged. Partial withdrawal, honestly noted.
- known-not-mine: value-inversion vs code quality (careful-engineer books drive the analyzer to ⊤
  — 15x deploy-widget; kSILO watch-item) and the oracle-quality probe hazards (R2-SHADOW/IDCACHE/
  GETENT/ORTRUE) are recorded, adjudicated project knowledge; reported here only as context.

## Meta-observation (lesson-02, constructive)

No evaluation of the authored surface with any outside shell-literate human is recorded anywhere in
the corpus; 24G §1's own meta-lesson concedes design errors surface only at walk-me-through-it
altitude, and every walkthrough participant so far is the designer or the AI. The cheapest
correctives from the ergonomics literature fit this project exactly: a Cognitive-Dimensions
walkthrough of the stage-3 exemplar (an afternoon, [green-petre-cogdims]), an Elm-style
error-message catalog for the lift diagnostics [elm-compiler-errors], and showing five target-
population admins the stage-3 oracle cold before the first real author writes one.
