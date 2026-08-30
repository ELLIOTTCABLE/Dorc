# 28K — Oracle loading and resolution: the function-environment pass

> Tier: LLM-authored plan, conductor-synthesized from a human design dialogue (2026-07-28,
> session `r28-name-conflict-design`); subordinate to the root docs and `spike/CLAUDE.md`.
> Ruling grades used throughout: **[TYPED]** = human typed it (in the dialogue, or in the
> CLI-inputs round note it descends from); **[ACKED]** = human confirmed the substance in
> dialogue, exact wording unratified; **[PROPOSED]** = conductor-derived, consistent with
> everything typed, awaiting ratification (§9 collects these).
>
> Companion round: the CLI-inputs sitting (branch `ai/r28-cli-inputs`; its round-note is
> being renumbered and is deliberately not cited by ID here). Its §5 handoff — "two
> definitions of one role function" — is the question this plan answers; and since that
> note is being re-sited with its loading material removed, every loading-relevant ruling
> it carried is reproduced durably in §2a. Its phases 1–2 remain that round's build work,
> not re-planned here.

## §0. The problem, and the shape of the answer

The engine today permits multiple definitions of one role function (e.g.
`apt_get__predict()`) across the input unit and *combines* them: resolution is
first-match-in-load-order (flagged `tc-` in place at `analysis/src/effect.rs:395-398`,
descending from the round-20 expedient recorded in `notes/208` strain-W4), and the emitted
artifact **redefines the function immediately before each call site**. That emission is
correct only under strictly-linear adjacent execution, and its named breakage routes
(probe parallelism; guard interleaving in the apply artifact; sourced-oracle shadowing;
any future hoisting) are real. It also permits by one route what
`oracle/src/reserved.rs:10-14` refuses by another (an artifact carrying two same-named
funcdefs).

The intent it accidentally served, though, is genuine and in-scope: a messy repo of
half-maintained oracle-shaped sh is a real input, and both hard-single-definition
fail-fast and whole-definition precedence-discard destroy value there (a bundle's one
junk oracle bricking its dozen good ones; a battle-tested oracle's presence killing a
janky one's coverage of shapes the good one never modeled).

**The answer, in one sentence: there is no Dorc resolution mechanism.** Oracles enter the
unit by ordinary sh loading (`.`-sourcing, definitions, `unset -f`, subshells), the
analyzer's abstract interpretation computes the resulting function environment exactly as
a shell would, licensure reads that environment — and exactly one thing is layered on
top: Dorc *complains, and withholds licenses*, where sh would silently let one unit's
definition shadow another's, with `unset -f` blessed as the native spelling that retires
the complaint. Runtime behavior is never altered; the emitted apply artifact pins every
guard to the analysis-resolved definition bytes so that no licensure decision is ever
re-derived by runtime name-resolution.

Governing prior rulings this rests on: `rul-vouch-is-verdict-authoring` ·
`rul-ternary-verdict` (never engine-synthesized semantic sh) ·
`identity-declared-never-inferred` (no engine argparse, ever) · `silence-licenses-nothing`
· `inv-top-reject` · `271:rul-sin-ordering` (mis-attribution outranks all other sins).

## §1. The head-model rules (the whole user-facing semantics; ≤4 by design)

- **rul-sh-loads-dorc-reads** [ACKED] — oracles enter by ordinary sourcing; the oracle
  answering any site is exactly the definition a shell would have live (last definition
  wins; `unset -f` removes; subshells scope; a define-if-absent guard lands only in a
  free slot). Dorc adds no loading construct, no manifest format, no registry: the
  admin's existing mental model of sh loading IS the resolution model.
- **rul-silent-shadowing-refuses** [ACKED] — when one unit's definition overrides a role
  family that a *different* unit defined, that family's licenses switch off (complaint +
  run; never a plan-abort, never an execution change) until an explicit `unset -f` of the
  name, textually between the two definitions, spells the intent. Guarded incoming
  definitions are exempt *as a consequence, not a blessing*: under plain sh they cannot
  override at all.
- **rul-scope-by-subshell-resource** [ACKED] — to prefer a different oracle for a region,
  re-source its file inside `( … )` around that region; sh's own scoping semantics do the
  rest, in analysis and at runtime alike. Per-line preference is a subshell around one
  line; sh has no finer scoping quantum and Dorc invents none.
- **rul-unloadable-is-unlicensed** [ACKED] — dynamic source paths, load loops/cycles, and
  host-dependent conditional loading leave the affected names ⊤ (walls). This is the
  ambient `inv-top-reject` posture wearing loader clothes, not a new rule; and the
  richness cut goes the other way where the analysis genuinely understands: a source path
  through resolvable variable-flow (`LIB=./oracles; . "$LIB/yum.sh"`) loads fine, exactly
  as `"$CERTS"` has always resolved.

**The one-exception check-in** [TYPED, as sharpened]: nothing here exceeds correct,
standards-compliant abstract analysis of ordinary POSIX sh, with exactly one exception
*category* — complain-where-sh-allows on definition conflicts (two members: the
cross-unit shadow above, and the pre-existing within-file redefinition refusal, `216`
e-1) — plus `unset -f` blessed with communicative semantics to manage the complaint. The
blessing is safe to layer because its runtime meaning is *also* modeled faithfully: in
blessing position it is typically behaviorally inert, and where it isn't (a call in the
unset-to-redefinition window) the positional interpretation tracks that too, so the
Dorc-reading and the sh-reading cannot be brought into contradiction.

## §2. The analyzer pass (this is an analyzer upgrade, not a separate loader) [TYPED]

The function environment becomes a first-class abstractly-interpreted domain — positional
and scope-aware, exactly like variable-flow — over: `.`/`source` statements (inline the
file's items at the point, recursion = cycle-refusal), `unset -f` (env delete),
`command -v <literal-name>` (env lookup, feeding define-if-absent guards and conditional
sourcing), `[ -f <literal> ]` on local oracle paths (controller-side stat). A separate
loader-evaluator is explicitly rejected: it would be a second interpreter with permanent
coherence debt against the first.

- **rul-conflict-pass-is-semantic** [PROPOSED] — rul-silent-shadowing-refuses is
  evaluated over the abstract environment ("did a cross-unit override occur without an
  intervening unset of that name"), never as syntactic guard-pattern-recognition around
  load sites (`271:rul-net-quality-u-curve`: imperfect mechanical nets are the footgun).
- **rul-visibility-is-full-positional** [ACKED spike-tier, human-typed 2026-07-31 —
  `28M` §7; supersedes the two-regime split this bullet formerly carried] — every
  site-keyed consuming act (verdict, predict-at-site, probe-ship, vouch, guard
  eligibility) reads the function environment AS OF THE SITE'S POSITION in execution
  order: if a shell executing the book top-to-bottom would not have the definition
  live at that line, it answers nothing at that line, for *any* act. The naive mental
  model to preserve — Dorc as a "stupid guard-inserter" whose inserted text cannot see
  a definition loaded below it — now applies uniformly, not only to guards. The env
  constructs themselves (`.`, `unset -f`, `command -v`, subshell scopes) are
  positional as before.
  - The one ambient tier is VOCABULARY: kind-owner families load from the ambient
    prefix (CLI-named files, "before line 1"), single-occupancy; in-book vocabulary
    roles refuse-with-notice (`28M:obl-in-book-vocabulary-role-notice`).
  - Positionality bites only in books: marked oracle files are load-inert (§2a), so a
    definitions-only file's internal order stays unobservable and whole-file loading
    is order-free.
  - Named, accepted consequence (sharper than the old split's): a definition
    introduced late in a book licenses NOTHING above itself — no elision, no guard,
    no vouch. The stage-3 in-book oracle is spelled define-before-use
    (README/USER_STORY respelled so, 2026-07-31); a late definition with describable
    sites above it earns a move-it-up hint (aid-plane). Top-level conditional
    definitions and loads whose end-state is statically undecidable stay ⊤
    everywhere, as before.
- CLI-named files and `-I` include-dirs load as the ambient prefix ("before line 1", in
  command-line order); the book's own text executes after. The input surface's standing
  rulings are glossed in §2a.

## §2a. The input surface (imported)

The CLI-inputs round-note is being re-sited with its loading material removed; these
rulings are durable here now.

- **rul-named-files-are-positional** [TYPED, CLI-inputs round] — named input files are
  bare positionals (a shell glob is the many-file spelling); `--book=`/`--book`,
  `-o`/`--oracle` are deleted, not deprecated (`rul-strawman-formats-no-compat`).
  Positionals classify by *marker content*, never filename convention: marked ⇒
  oracle-input (dorc-lang), unmarked ⇒ book-input — an upstream junk-repo will not
  honor our naming, and a marked file is self-describing and provably inert to load.
- **rul-include-dir-is-flat-by-default** [TYPED, CLI-inputs round] — the bulk-load
  argument is `-I`/`--include-dir`, FLAT, one level; `--include-recursive` is a
  separate opt-in. Grounding (U-shape law: match a convention fully or break loudly,
  never a confusingly-similar middle): every surveyed include/library-path convention
  (GCC `-I`, `ld -L`, rustc, javac, GHC, protoc, shellcheck, ansible-lint, Puppet,
  Make) is a flat, name-driven, one-level lookup and none recurses; the audience's one
  ingrained drop-in-directory model is the `*.d`/run-parts idiom, unanimously
  single-level; and the shell world spells definition-directories as env vars
  (`FPATH`-family), never flags — so an env twin, if ever wanted, is `<NOUN>PATH`-
  shaped, and `-L`/`-p` spellings are rejected as category errors. Unmarked `.sh`
  under an include dir is walked past and named in the loaded-source inventory.
- **rul-marked-file-is-load-inert** [TYPED, CLI-inputs round] — a marker-carrying file
  must be provably no-op to load: top level holds function definitions and bare
  assignments only (file-global constants are a must-have), never commands — including
  command substitution or arithmetic inside an assignment value (`CERTS=$(hostname)`
  is a command in disguise); `export`/`readonly` stay rejected for now
  (`271:rul-posix-in-spirit-defaults`). Load-bearing in this plan twice: it keeps the
  abstract load total, and it is what makes the subshell re-source idiom safe to
  execute for real.
- **In-book lift** [TYPED, CLI-inputs round] — every input file, book included, feeds
  the role lifts (membership by name-construction, never file or author) — the stage-3
  rung made real. Books stay plain sh always (`rul-book-is-plain-sh-always`); dialect
  syntax lives only in marked files, so an in-book role function is bare-POSIX,
  recognized by name alone.
- **Provenance** [TYPED, CLI-inputs round] — one `SourceFileId` space over every
  input, book and oracle, per-file line numbers (`AID-NEEDS:law-lineno-identity`); the
  pinned-definition emission (§4) and the shadow-refusal diagnostics cite through it.
- **Open, carried with the filename surface** — `dorc why`'s optional address
  positional collides with bare-word input files (`dorc why webhost.sh cp.oracle.sh`
  reads the book as an address); candidate resolutions (first-bare-word stands, or
  `why` takes no file positionals and answers from the receipt, with a story owed for
  `why --results=FILE`) belong to the CLI round; carried here only so the question
  survives the note's re-siting.

## §3. Naming, defaults, and the admin's selection vocabulary

- **rul-library-naming-finds-defaults-only** [TYPED] — "naming a library/file is
  authorization to find a *default* oracle in there, when no specific oracle is named; it
  is not authorization to choose *which* oracle to use." Sharpens (does not reverse) the
  you-named-the-library widening of `24H:ack-6`: sole-providership keeps today's implicit
  consent; plurality never resolves by the mere act of loading.
- **rul-polyfill-guard-is-default-spelling** [ACKED, near-weld per human] — the native
  define-if-absent idiom is the author-side spelling of "default":

  ```sh
  if ! command -v yum__is_converged >/dev/null 2>&1; then
     yum__is_converged() { ... }
  fi
  ```

  Under plain sh, in either order, a guarded definition loses to an unconditional one,
  and guards among themselves resolve first-wins — the shell itself computes the
  specific-beats-default lattice; Dorc merely reads it. With the fallthrough-cascade
  rejected (§6), a guard's Dorc-meaning is *pure load-time sh* — nothing layered on text
  a shell would treat as dead. Etiquette (lint/hint lane): publishing an oracle for a
  tool outside your project's own scope ⇒ guard it; costs the author nothing standalone.

  *As-built: the lattice is READ, by the decidable-condition fold (`28M` §9). The guard's
  condition is evaluated against the solved function environment at its own position; the
  arm a decided condition proves dead has its edge masked and the environment re-solved, so
  a guard loaded after a real oracle leaves that oracle's binding intact instead of joining
  the family to ⊤. Two seats, not one: the same environment also subtracts every definition
  it proves binds at NO program point from the whole-unit resolution
  (`dorc_oracle::live_source`), because a guard that is textually last still won that answer
  and the site-keyed agreement gate then withheld — the same silent wall one seat further
  along. A condition outside the closed decidable set (`28M:dec-decidable-set-v0`) folds
  nothing and its family stays ⊤: conservative, sparing-inert, walls.*
- **The admin's toolkit, all native, none requiring the oracle author's cooperation:**
  global surgical removal (`. a-repo/tools.sh` then `unset -f yum__is_converged …` per
  family member); regional preference (rul-scope-by-subshell-resource — and re-sourcing a
  whole file in-scope is harmless: sibling definitions re-land identically, and the
  co-residence of junk with wanted definitions in one file stops mattering, because
  selection manipulates the *environment*, never files); conditional sourcing
  (`command -v f >/dev/null 2>&1 || . backup.sh`); and vendoring-with-custody (copying a
  definition into your own file under your own guard) as the deep fallback.
- **rul-conflict-vocab-lives-in-book** [TYPED] — CLI naming remains the zero-conflict
  convenience tier and refuses on any conflict with one message: resolve it in the book,
  where the durable resolution vocabulary lives with the semantics it governs (top-of-book
  sourcing block, versioned, greppable). Config stays spelled in sh; `KNOBS:kOOB` clean.

## §4. Emission: licensure never rides runtime name-resolution [TYPED]

- **rul-runtime-resolution-never-load-bearing** — a misalignment between our binding
  model and the landing shell's actual resolution must not be able to swap *whose
  judgment executes* (that is pope-sin tier, `271:rul-sin-ordering` — worse than any
  lost value). Therefore, wherever the runtime would re-derive an answer the analysis
  already decided, the artifact carries the answer instead. The probe lane already lives
  by this (`rul-only-oracle-bytes-ship`: the engine physically composes the chosen bytes;
  no runtime name-resolution exists there to disagree with). The apply lane's guards now
  do too; the brief no-preamble design (guard as bare call, bound by the book's own
  sourcing at runtime) is rejected — it was the only seam in the system where a
  licensure decision was re-derived at runtime by a mechanism we merely simulate.
- **rul-pin-by-definition-bytes** — the pinned material is the analysis-resolved
  *definition's* bytes (strip-applied, authored), plus its closure (helper functions and
  file-level constants it depends on, from its source file — machinery shared with probe
  composition). Pinning by re-inserting the *load-unit* (`. chosen-file.sh`) is
  explicitly rejected: it executes unrelated top-level material and re-imports the
  co-residence problem this whole design exists to kill.
- **rul-hash-munge-disambiguation** — when the unit's live history holds >1 distinct
  definition of a name (distinct bytes, post-dedup), the apply artifact emits each needed
  definition once, hoisted to the highest shared position, under a
  short-hash-disambiguated name; call sites stay readable:

  ```sh
  ( yum__is_converged_h4x2 groupinstall widget-deps ) \
     || yum groupinstall widget-deps
  ```

  In the single-definition common case the plain name is emitted, byte-identical to
  strip. Byte-identical definitions across files dedup by content first (vendored copies
  are the commonest real collision). The disambiguated name is engine scaffolding around
  authored bytes — the same sanctioned category as the guard glue and the declared-dual
  sense-flip — never a second source of convergence-truth; plan-render attribution names
  the authored function and its (file, line) via the SourceFileId provenance work.
  *Supersession:* this supersedes the `23A:P-reingest` derivation that a
  collision-dodging rename is unspellable (that derivation bound authored-text stripping;
  engine-assembled artifacts were out of its scope), and it retires today's
  per-site-redefinition emission entirely. It also dissolves the reserved.rs tension: the
  shipped artifact never again carries two same-named funcdefs by any route.
- **Artifact-surface asymmetry** [TYPED] — the apply artifact is a human-readable,
  human-reviewed, potentially durable *artifact*: definitions hoisted, call sites clean,
  provenance renderable. The probe artifacts are engine-internal — dynamically
  constructed, split, multiply-deployed, parallelized — and carry no such readability
  obligation; they were already pinned by composition and are unchanged by this plan.
- **Reingest property preserved** — a hash-munged name does not parse as a recognized
  `__role` (role vocabulary is closed), so a re-ingested artifact's guards read as opaque
  function calls ⇒ conservative run: "safe, merely unimproved," exactly the
  `23A:P-reingest` floor. Bonus closed hole: pinning ends plan/apply TOCTOU on oracle
  *files* — the vouched bytes and the license travel together in the approved artifact.

## §5. Consistency doctrine (the tier ordering, typed) [TYPED]

sh-modeling bugs that *confuse* users are explicitly a lower tier than bugs that
*mis-apply licensure*; wrong-but-consistent beats inconsistent. A binding-model bug under
this design misreads spelled intent (caught by calibration and by reading the plan) but
can never silently swap which body executes under a license — the artifact matches the
plan the human approved even when the model misread the book. Named for reuse,
**pattern-carry-the-answer**: "can we carry and inject the analysis's answer, without
degrading output quality, so analysis bugs bite consistently in the result?" — a
defense-in-depth question to re-ask wherever runtime would otherwise re-derive a decided
answer. Model-calibration rides `KNOBS:kVERIFY` as everywhere: differential manifests
with sentinel bodies (which-am-I emitters) against the pinned two-binary floor; one
battery case for `command -v`'s PATH-executable reach vs. our fn-definedness reading.

## §6. Rejected, permanently (the fences)

- **rej-argparse-fusion-forever** [ACKED] — engine-synthesized dispatch merging N
  authors' argparses: triple law-collision (engine-side argparse ban; engine-synthesized
  semantic sh in guard position; a merged judgment no human made or vouched — no
  attribution target). No payoff survives §4: everything fusion could buy, selection
  plus pinning buys without body analysis.
- **rej-decline-fallthrough-cascade** [PROPOSED as dead-on-principle, not
  deferred-pending-evidence] — "head declines ⇒ next definition answers, automatically"
  was the last scrap of engine-side trust adjudication and the one construct that put
  analyzer semantics on shell a plain reader sees as dead. Gap-filling is always an
  explicit admin act at an explicit scope (§3). The chain-combinator escape valve dies
  with it. (This also closes the re-armed-hazard-decline scenario outright: a deliberate
  `return 2` can no longer be silently routed around by a lower-quality author's body.)
- **rej-resurrection-of-dead-definitions** [ACKED] — sh-dead definitions (overridden
  unconditionals, unset names) are never licensure inputs; only sh's winner plus
  self-spelled (guarded) subordinates exist. The lifted model stays a strict superset of
  a bare-sh reader's prediction, never a divergence from it.
- **rej-load-order-as-trust-adjudicator** [TYPED, via rul-library-naming-finds-defaults-
  only] — appending a source line must never silently reassign a contested family; the
  shadow-refusal contains append-hijack, at the bounded cost of that family's coverage
  until one line of spelling.

## §7. The two-kind pattern: ruled a respell (answers the CLI-round §5 handoff)

Five fixture dirs deliberately define `apt_get__predict()` in two files
(`package.oracle.sh` install/purge arms; `pkgindex.oracle.sh` update arm), merged today
by first-match. Under sh semantics the later definition simply *wins* and the earlier
one's arms are dead — pure loading does not merge, and per §6 nothing may. **Ruling
[PROPOSED]: this pattern is a spelling error** — the CLI-round note's option
"one-function-per-provider-by-construction," grounded: per-verb combination of two
bodies' argparses IS cross-author merging, the thing every fence in §6 exists to refuse,
and the human's handoff framing (single literal function at runtime; generation ruled
out; merging gnarly) already suspected it a non-starter.

What the pattern actually wanted survives intact, relocated:

- One `apt_get__predict` body may mint cells of *many kinds* (marks are per-line; nothing
  ties one function to one kind). The two-kind fixture content merges into one body in
  one file — a mechanical fixture edit, inventoried below.
- Kind-*ownership* families (`sm_dorc_PkgIndex__resolve`, `…__disturbance_reaches`)
  remain separate, separately-publishable files — untouched by this plan. Independent
  publication is recovered at the kind level, where ownership actually lives.
- Two independent upstream repos both describing `apt-get` is genuine plurality:
  §1/§3 semantics govern (default-finding, shadow-refusal, admin selection). What the
  ecosystem loses is only *automatic* cross-author merge — which was never sound to offer.

Fixture/behavior consequences (golden churn is expected and re-blessed; these are the
behavior-bearing pins to keep): the five two-kind dirs merge their `apt_get__predict`
bodies; `guard23-reingest-collision-verbatim` keeps its refuse-and-run outcome, now
produced by the *general* shadow-refusal rather than a bespoke door-4 rule (its book's
inlined definition shadows the loaded oracle's, unblessed ⇒ family unlicensed ⇒ verbatim
run, no guard accretion; if ever blessed, the no-double-guard recognition governs the
guard-shaped line). The CLI-round's phase-two ordering rationale ("book sorts last so a
colliding book function stays inert under first-match") is superseded: a book definition
wins its family in the *ambient* regime (in-book is the most-local, admin-authored act),
the positional regime agrees wherever the definition precedes the guard-shaped line (as
the reingest fixture's inlined preamble does), and the shadow-refusal is the consent gate
when it collides with a loaded unit.

## §8. Interactions and honest residue

- **kSILO** — no new pull either direction: in-book role functions are first-class
  (stage-3 rung), oracle files gain no capability books lack in this pass. The guard
  etiquette mildly *rewards* publishing partials safely (a guarded arm keeps value after
  a stdlib exists), which is the pro-ecosystem direction.
- **Monotonicity, stated honestly** — "adding a library never changes existing sites"
  weakens to: adding can never *silently* change whose judgment governs (it refuses
  instead); blessed changes are the admin's typed act. Trust exposure for a family never
  exceeds the max of the single-provider worlds without an explicit spelling naming both.
- **res-book-ships-its-load-closure** — a book that sources oracle files needs them
  present at apply (Dorc: bundle the statically-known closure; off-ramp: ship the
  directory, as ops already does). Relative-path-vs-cwd robustness is a build concern.
- **res-host-conditional-loading** — `[ "$(uname)" = … ] && . mac-oracles.sh` forks the
  environment per host: real future need (`KNOBS:kTPLATFORMS` per-platform oracles),
  genuine probe-phase chicken-and-egg, v0-refused under rul-unloadable-is-unlicensed.
- **res-overlay-primacy-tax** — a library *designed* to replace stdlib families cannot
  self-authorize (guards subordinate; unconditional defs trigger the refusal), so its
  consumers pay one `unset -f` line per replaced family. Held as correct-but-taxing.
- **res-knobs-entry-owed** — the underlying tension (refuse-colliding-descriptions ↔
  resolve-by-spelled-selection over the tool-name commons) is a genuine KNOBS-tier axis;
  reported for human naming per KNOBS law, not added here.
- **res-terminology-retired** — the dialogue's interim vocabulary (seat, cascade,
  precedence order, candidate set) is retired; nothing in this design selects — the
  environment *is* the selection. Any doc reviving "seat/cascade" semantics is drifting
  toward §6's fences.

## §9. Ratifications owed (open until typed)

1. **rat-four-rules-wording** — §1's four rules as law, verbatim.

   > human: acked.

2. **rat-blessing-vocabulary-v0** — the shadow-refusal's blessing set is exactly
   {`unset -f` of the name, textually between the definitions}; guards exempt by
   consequence. Richer vocabulary only on field evidence.

   > human: examined, mostly holds. See 28M §8,
   >        `28M:rul-conflict-between-totals-is-falsification`,
   >        `28M:rul-composite-meets-toward-guard-run`, etc.

3. **rat-fallthrough-dead-on-principle** — §6's rejection is principled, not
   evidence-pending; nothing rebuilds toward it.

   > human: acked, I don't see any sane way to express and maintain multi-author
   >        engine-synthesis within our "attribution is paramount" paradigm

4. **rat-two-regime-wording** — §2's visibility rule: the direction is typed
   (hoisting only where no native-sh load-order applies; guards follow the
   stupid-guard-inserter model), but the wording, the consumer allocation between the
   regimes, and the accepted elide-only-above-a-late-definition cell deserve eyes.

   > human: acked and updated, USER_STORY and README are now consistent, I believe.

5. **rat-two-kind-respell** — §7's ruling and its fixture consequences.

   > human: soft ack; spirit not letter

## §10. Build shape — THE resume plan (rewritten 2026-07-31 at the ratification
sitting's close; a fresh implementation-conductor executes from here)

**Standing state:** lane `ai/r28-oracle-loading` (worktree `r28-oracle-loading`),
rebased onto post-loom `ai/main`, both legs green (Win 1826 / WSL 1822), locks at the
generator fixpoint. Stages A/B/G/D/E are LANDED — ledger `notes/28O` (read its rebase
section). §9 above is fully typed-closed; `28M` §7 is the ack-ledger and §8 the
license-plane ground truth — on conflict in the committee corner, `28M` governs. This
section is the ONLY live implementation plan for this lane; `28M` §8's build items are
incorporated below by name, and nothing else in r28 plans further work here. The
`bitem` slugs are ordered (build in integer order, parallelizing only where obviously
independent); they decompose the ruled stage letters — `bitem0`–`bitem7` are stage F,
`bitem8` is stage H, `bitem9` is stage C2 — so `28O`'s stage vocabulary still connects.

**bitem0-positional-regime-conversion** (the acked regime, §2 as rewritten): site-keyed
acts (verdict, predict-at-site, probe-ship, vouch, guard eligibility) move from the
ambient `live_source` answer to positional funcenv reads; vocabulary acts stay
ambient-prefix, single-occupancy, in-book refuse-with-notice. Mint the move-it-up hint
(late in-book definition with describable sites above; empty prose, defining case).
Pin the sharpened consequence cell (late definition licenses nothing above).

**bitem1-pin-by-definition-bytes** (§4, unchanged in substance): emission + closure +
hash-munge + content-dedup + provenance blocks. The closure captures from the LIVE
environment wherever sourced — cross-file helpers included (`28M` §8 overlay riders);
diamond-loading keys unit-identity to the DEFINING file's `SourceFileId` (pin:
version-skewed vendored copies refuse rather than dedup).

**bitem2-resolution-seat-unification**: `VerdictIndex::from_sets` consults `live_source`
instead of re-implementing last-wins (`28M:fnd-verdict-resolution-duplicates-live-source`);
re-audit all five seats after bitem0 (the "five sites must agree" comment becomes true
or dies).

**bitem3-custody-and-monologue-pins** (`28M` §8): thread `SourceFileId` custody into the
`ReplaceLicense` mint (re-entry becomes a type error); pin split-family
establish-elide consumes nothing predict-derived; trace the stdout parallel of the
rc firewall (pin only what the trace finds missing); pin vouch-covers-the-stand-in-rc-0.
The small meet-direction registry over properties (typed lean, machinery-high) rides
here; flag to the conductor if it snowballs past "small".

**bitem4-committee-fence-at-sparing-tier** — retired unbuilt: no fence; the sparing
dialect is per speaker closure, read at the backing's frame, and builds with `plans/30J`
(`30J` §12).

**bitem5-split-family-coherence-aid** (`28M` §7 lean-demotion-is-not-deletion — this is
strictly MORE machinery than the fail-fast form it replaces, never less): collate the
divergence evidence WITHOUT early exit, carry it forward, narrate attributed under the
kWARN-rich weld (chimera-incoherence narration mints at minimum; render may wait for
the arrangement-walker round). Totals-conflict
(`28M:rul-conflict-between-totals-is-falsification`) is a DISTINCT detected class from
judgment-tier divergence; the engine knows them apart.

**bitem6-commissioned-composition-suites** (`28M` §7/§8, human-typed): (1) the
split-family lane-separation fixture (the W-B pins as a steady-state configuration);
(2) the two-file helper-package shape — helpers file (bulk logic, non-role names) +
ONE thin opt-out-able entrypoints file — proving both halves: entrypoints lift with
cross-file helper closure intact, AND the entrypoint file can be discarded/swapped
(`unset -f` / not-sourced / replaced by the admin's own) over the same helpers. These
tests MEASURE whether the check-dialect/lift currently ⊤s non-role calls inside role
bodies; a gap is reported as named lane work, never built around silently.

**bitem7-renames-and-small-riders**: rename `WhyReport.oracle_paths`/`oracle_srcs`
(source-wide now); optional lint candidate
entrypoint-only-constants-under-deep-require (~SUSPECT tier, skip if it drags).

**bitem8-differential-load-order-battery** (ruled stage H, unchanged): sentinel-body
load-order manifests under the pinned two-binary floor; the `command -v` PATH-reach vs
fn-definedness case; the `||`-operand funcdef parse question for the terse guard form
(the `if` form is canonical regardless).

**bitem9-value-flow-source-targets** (ruled stage C2), budget-permitting, as
originally ruled.

**fold-checklist-at-lane-close** (conductor's, not a bitem): eyeball the load-inert
refusal on the three loom-era lint cases; promote `28O`'s routed-to-fold items into
the registries; place the remaining supersession markers (`28O` collide-on-plural; the
E→F checkpoint asks); LIVING_STATUS re-measure; dispose of `ai/r28-cli-inputs`.

**bitemF-decidable-condition-fold** (`28M` §9; LANDED, sited into this lane by the
implementation conductor): pessimistic conditional-constant-propagation over the
function-environment domain, plus the whole-unit resolution's never-live subtraction that
carries the cure past the agreement gate. §3's as-built paragraph is the semantics; the
ledger is `28P`. Widening `dec-decidable-set-v0` is NOT part of it and stays closed.

**Explicitly NOT this lane's** (deferred by name, do not scope-creep): the per-speaker
dialect build (`30J`) · kind-level token registration · MH2 target-identity · richer blessing
vocabulary (field-evidence-gated) · `res-host-conditional-loading` ·
the dialect-reach widening for unary file-tests (gates stdlib revival, not this lane).
