# spike/crates/oracle — CLAUDE.md

Role: lifts an oracle's sh **statically** (never sources or runs it) into the
engine's index; home of the authored-surface contract and the stdlib-oracle
quality bar. Read `spike/CLAUDE.md` first — its license & trust cluster IS this
crate's law; specs: `notes/277` (coordinate/grammar) · `notes/273` (wrappers) ·
`notes/274` (eval'ers) · `notes/278` (the one-page dialect reference). Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law — the lift

- **static-lift-only** — never execute oracle code; never model a tool by
  dry-running the mutator (`an-fact-centric`: the command-centric
  `--dry-run`-as-probe strawman is dead, permanently).
- **declarations-only-files** — a top-level mutator or unmodeled construct in an
  oracle file is a loud ⊤-reject; a malformed lift is a `Diagnostic`, never a
  panic; an absent effect-entry is ⊤ ⇒ run, never a silent wrong-elision.
- **argparse-is-the-vouch-typechecker** — the oracle author's own argparse is the
  sole entity-resolver (identity-declared-never-inferred) and the type-checker of
  the vouch: book argv acquires a probe-execution license only by passing THROUGH
  the author's arms, declines included (`271:rul-only-oracle-bytes-ship` ·
  rul-argv-flows-bytes-do-not). Engine shape-matching over book bytes is an
  unchecked cast — it never mints.
- **rc-partition-here** — 0 = named sense; 1 = complement; ≥2 = flat sink, runs,
  never inverted, never licenses. Never collapse statuses out of a
  verdict-function; a tool with a non-test exit vocabulary needs an explicit
  `case $? in` remap arm in the delegation body.
- **bind-is-identity-never-authority** (`28O` tbl-ambient-annotation-sites; the principle,
  promoted at the r28 resume fold) — an inline bind is an entity-identity channel: it
  resolves entity REFERENCES and kind-tags them for the book-site back-map; it is never a
  kind authority and never a cell authority. A mark's own coordinate is authoritative for
  its cell in full; ambient state may fill ONLY what a coordinate genuinely lacks. Known
  residue, named not chased: a shared bind's unresolvable value still ⊤s a nullary-verb
  check whose cell needed nothing from it (`28O:fnd-unresolved-bind-value-tops-the-whole-check`;
  the one-rule sweep is specified there, unstarted).
- **effect-check-falsification-first** — proven-mutation ⇒ fails fast and lifts
  NOWHERE (not probe, not guard — `271:rul-no-mutating-guards`); the unprovable
  region rides the authored vouch; the check is never a completeness gate.
  vouch-scope-is-the-body-never-the-tool: a body-vouch mints no command-family
  fact.
- **withdrawing-drops-detected-too** (`28K` §1) — `PredictSet::withdrawing` (and the
  `VerdictSet`/`TouchesSet` forwarders) removes a contested family from `checks` AND
  `detected`. Dropping only `checks` would leave the header behind, so the marks-lost
  backstop (`crate::validate`) would report a WITHDRAWN funcdef as a lift failure and
  point the author at the wrong repair (`271:rul-sin-ordering`).
- **the-frame-lookup-is-the-only-resolution-seat** (`28Q` §1.3, né
  live-source-is-the-only-resolution-seat; `28M:fnd-verdict-resolution-duplicates-live-source`) —
  which definition answers at a site is ONE question asked ONE way:
  `funcenv::LiveDefinitions::definition_before` names the definition the frame holds, and
  `dorc_core::answering_file` selects the row that definition produced. Every role-lane seat
  routes through it — the effect-map lift, `VerdictIndex::from_sets`, `analysis::effect`'s predict
  and verdict lanes, the three cli ship closures, `plan::build_vouches` and
  `build_wrapped_vouches`, and (since the crosscheck burndown, `308` §1) the wrapper lane: each of
  `__predict`/`__lend_map`/`__enter` resolves its OWN frame answer at the consuming site (sh binds
  each member independently), and the ONE resolved inner verdict feeds the shipped body, the entry
  consent, and the carry closure-proof together — `try_carry` takes the resolved body by
  parameter and cannot reach a second definition by construction. Stage-0 added the
  ordinary-seat twin: `analysis::effect`'s primacy test and cell mint read ONE resolved
  verdict (`live_verdict`); resolve once and pass the body — a third lookup is the
  `28M:fnd-verdict-resolution-duplicates-live-source` failure class returning. Derived rows are keyed by their
  producing file, so identity and cells
  are read from ONE definition and the chimera is UNREPRESENTABLE rather than gated: the whole-unit
  `live_source` scan and the positional agreement veto that narrowed it are BOTH retired, because
  they were two readings of one environment and could disagree (`28P:fnd-build-vouches-relifted-the-verdict-sets`
  is what that cost the last time). TWO named non-resolution folds survive, both consuming
  withdrawn inputs: `dialect_minting_source`,
  vocabulary-AGGREGATION only — it preserves the sparing dialect's minting SET and answers nothing
  about which definition speaks (`28Q` §9 `pin-two-position-sparing`) — and
  `escalation_policy_diagnostics`, a frameless whole-unit POLICY disclosure over the
  loaded-and-withdrawn set (aid-plane; licenses nothing; a frame would invent a site the question
  does not have).
  A seat's predicate asks only "does this file DEFINE the role", never "does its body answer this
  argv" — the second question is the retired decline-fallthrough cascade (`28K` §6). A decline by
  the resolved definition is a decline. Seats read the driver's WITHDRAWN per-file sets, never raw
  source: `lift_from_sets` and `dorc_plan::build_vouches_from_sets` exist because both re-lifted
  from text and so resolved over a population every other seat had already narrowed. A seat that
  re-lifts is a seat that will disagree.
  **Winner-shifting, permanently** (`28Q` §1): with no agreement veto behind it, every
  function-environment precision bug now SELECTS WHOSE JUDGMENT governs a site. The frame solver is
  license-review-tier forever; precision work on it is never ordinary value-add.
- **closure-travels-with-the-definition** (`28K` §4 `rul-pin-by-definition-bytes`) — a role funcdef's
  span does not carry the helpers it calls or the file-level constants it reads, so `crate::closure`
  is what every emitting seat asks before shipping a body: `HelperIndex::build` over the ordered
  sources ONCE per unit, `closure_for` per definition. Measured before it existed: a verdict body
  calling a helper lifted clean and shipped alone, which is usually a safe rc-127 decline and is NOT
  reliably one (a body that ignores the helper's status and answers 0 from a later test reports
  converged off a helper that never ran — priority-1 under-execute). Three sub-rules: helpers resolve
  across the WHOLE loaded set (`28M` §7 `tune-explicit-composition-is-sanctioned` — the helpers-file +
  thin-entrypoints package shape is the community-critical one); RESOLUTION is sh's own
  last-declaration-wins (`28R:rul-resolution-matches-shell-loading`, built at the emission stage —
  sound because the index admits only whole-top-level-load-inert sources, so over the indexable
  population the last indexed declaration IS what a shell binds), while the LICENSE is what custody
  gates: `307:rul-emission-custody-composite` — the vouch SUSPENDS iff the resolved custody differs
  from the voucher's AND (the book defines the name, or the declarations differ in bytes); the
  singular cross-file reach stays licensed (the package shape), byte-identical plurality counts as
  singular, and load order never silently adjudicates whose body serves whose vouch;
  and constants ride per CONTRIBUTING FILE rather than per reference, because `ParamComplex` discards
  the name so a reference-driven capture could not prove itself complete.
- **a-top-reject-is-not-a-definition-vector** (the emission stage, measured) —
  `UnsupportedReason::DynamicExecution` covers `eval`, a computed `.`, AND a command-position
  `"$@"` — the defining tautology of every peeling wrapper; the finer `SyntaxUnsupportedReason`
  is diagnostic-only and does not ride the node. Keying ANY rule on that AST reason puts every
  wrapper oracle in the world into its trigger (measured: it forced `context-entry-wrapped-guard`
  into defensive emission). Definition-vector detection is literal-word-only
  (`is_definition_vector`'s doc comment is the full finding); of the rule's named vectors only
  `alias` and `funcenv::unresolvable_loads()` are reachable — `eval` in a book is already an
  ERROR-tier whole-run refusal and is banned in oracles.
- **only-load-inert-sources-contribute** (`28K` §2a) — `HelperIndex` indexes a source only when its
  WHOLE top level is provably inert to load. That inertness IS the license to hoist a declaration
  above somebody's book, and it is also what keeps the BOOK out of the index without threading its
  id: a runbook has commands at top level, so its helpers stay where its author put them.
  **INERTNESS IS DYING IN LITERAL** [human-typed 2026-08-16]: blessed read-only top-level
  commands (`.`/`source`, `command`, more to come) are becoming legal oracle top level, so
  whole-file inertness is a constrained HYGIENE posture, never a premise the engine computes
  from. No resolution shortcut may assume a flat inert prefix. MEASURED CORRECTION
  (the pin battery, 2026-08-16): a host-conditional definition
  (`command -v jq || jq() { … }`) does NOT land can't-say through any conditional
  machinery — the dialect lift never SEES a role funcdef in a `||` operand
  (`parse_file` scans top-level items only; zero rows, zero `detected` headers),
  while `dorc_syntax::parse` and `definition_table` DO register it (a
  parsed-definition→no-lifted-row disagreement the join census does not cover),
  and the frame solver holds no opinion about an oracle file's own conditionals.
  Today's refusal = the load-inertness gate PLUS that silent blindness; the
  blessing must supply a real MAY-grade binding, never merely widen the
  allow-list (pinned both ways:
  `a_host_conditional_oracle_definition_licenses_nothing` /
  `a_described_host_conditional_definition_is_may_bound`;
  `FORFEITS:forfeit-whole-file-inertness-refusal`). Last-wins-over-the-index is
  an interim that dies with the funcenv table-widening
  (`rul-unsure-falls-toward-sh-parity`).
- **a-definitions-file-is-not-a-mark-fragment** — `validate` runs `lint_mark_subset` only for a
  source with no `__` AND no top-level funcdef. `__`-freedom alone read every helpers-only file — and
  every ordinary BOOK carrying a function — as a bare marked-statement fragment and errored
  `predict-out-of-dialect` at the funcdef header (eight corpus cases carried that false error). The
  fragment reading is for files that define nothing, which is structurally what the `mark-*` cases are.
- **safe-across-vouch** (né tolerates-vouch; `27C` §2, mark spelled `: safe-across
  <dim>` per `281` §5) — per-function, per-dimension; asserts "this
  body's effects are read-only BY DESIGN, not by privilege-starvation"; gates
  context-shifted execution under the escalation dial (both-sides consent).
  Wrapper entry forms are the ONE licensed seat for real context entry; predict
  closure bodies never escalate (real `sudo` is never a blessed closure idiom).

## The authored surface — worked minimum (the one syntax anchor; semantics live in `spike/CLAUDE.md`)

```sh
# dorc-lang/v0.2
systemctl__is_converged() {
   case "${1-}" in
   enable) systemctl is-enabled --quiet -- "${2-}"   : sm.dorc.Service:"$2"@enabled ;;
   start)  systemctl is-active  --quiet -- "${2-}"   : sm.dorc.Service:"$2"@active  ;;
   *) return 2 ;;
   esac
}
```

- **marker-and-names** — the `# dorc-lang/v0.2` marker gates syntax only; `__role`
  NAME-recognition is permanent and works in unmarked files; names are bare
  munged POSIX NAMEs (dots are dead); families are name-derived, never
  file/author-derived (`271:rul-family`).
- **minting-law** — verdict (`:`/`:!`) and observe (`:?`) marks on runnable lines
  MINT selector tokens; claim/disturbs emissions never mint; rc-arity caps a line
  at ONE verdict (`281` §7); brace-alternation `@{a,b}` / `verb {a,b}` expands to
  N cells and is refused ONLY on verdict payloads.

## The stdlib quality bar — regression classes, NOT engine holes

These pin "good, battle-tested sh" against ways it silently lies to a lifter. The
engine cannot check meaning (`inv-referent-agnostic`); the floor is authored
honesty + stdlib CI.

- **R2-SHADOW** — `command -v X` must confirm X resolves to an executable FILE:
  functions/aliases/builtins shadow it, and Dorc's own sourced-oracle idiom is
  exactly such a shadower. Fails unsafe (reports installed ⇒ elides the install ⇒
  priority-1 under-execute).
- **R2-IDCACHE** — group membership via `getent group`, never `id -nG` (`id`
  reads a stale resolver cache: member-removed-but-cache-warm ⇒ wrong elision).
  No same-session re-probe of a cell the run already mutated.
- **R2-ORTRUE** — refuse an errexit-masked rc as a probe verdict:
  `… || true` forces rc 0 and always reports holds. A lifted guard's rc is a
  verdict only if the analyzer can prove it unmasked.
- **F-GETENT-HOSTS** — per-database hermeticity: `getent passwd`/`group` are
  file-backed (fine); `hosts`/`ahosts` route through nsswitch → live DNS —
  read-only ≠ hermetic, disqualified from licensing (`KNOBS:kVOLATILES`).
- **R2-MULTIOP** — a body binding ONE operand must gate on there being no second
  (`[ "$2" = "" ] || return 2`): otherwise `install nginx curl` resolves to
  nginx alone, and a host with nginx-but-not-curl elides the whole install and
  never installs curl (priority-1 under-execute). The gate degrades multi-operand
  argv to ⊤ ⇒ run — the safe direction. The engine cannot supply this; it parses
  nothing.
- **F-BLESSED** — "blessing" is a stdlib oracle shipped day-1, not a separate
  mechanism. An honest `service` verdict is TWO probes (`is-enabled` AND
  `is-active` — discharging `enable --now` needs both), spelled as per-verb arms
  in the verdict body; below that floor ⇒ over-correlation ⇒ under-execute.
- **quality-bar-accretions** — quote-as-law · printf-doctrine (never `echo` with
  flags/escapes) · no bare globs in oracle bodies (zsh NOMATCH abort) · prefer
  full-read forms over early-exit `-q` where the producer minds SIGPIPE
  (sigpipe-flap-class) · before authoring `kind__state_stored_only_in()`: the
  store-survey audit ("have you audited every store this kind's tools reach,
  from every context?" — `only` = complete-by-contract) · emit-never spellings
  (`test -a`/`-o`; bare `set -o pipefail` — durable text carries the self-gating
  idiom).

## Direction

- **grammar-is-v0.2** (né respell-owns-the-churn) — the authored mark surface is
  dorc-lang v0.2 (`plans/281`): `@` selectors, word verbs, the `#:` carrier,
  single-mark-per-line production. The r28 cutover retired the
  `.prop`/`#`/`touches`/`reaches` spellings corpus-wide; nothing left to convert.
- **polarity-to-transitions** — the lifted effect is a binary bit at HEAD;
  becomes a typestate transition at the entity-algebra-rebuild (this crate owns
  the lift shape; `analysis` owns the transfer).
- **ktyannot-status** — inline annotation is de-facto and IMPLEMENTED
  (`KNOBS:kTYANNOT`; the formal weld is human-reserved). The spike IS the
  livability experiment: record friction in numbered notes; don't relitigate the
  spelling.
