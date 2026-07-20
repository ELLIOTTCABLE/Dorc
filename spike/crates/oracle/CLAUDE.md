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
- **effect-check-falsification-first** — proven-mutation ⇒ fails fast and lifts
  NOWHERE (not probe, not guard — `271:rul-no-mutating-guards`); the unprovable
  region rides the authored vouch; the check is never a completeness gate.
  vouch-scope-is-the-body-never-the-tool: a body-vouch mints no command-family
  fact.
- **tolerates-vouch** (`27C` §2) — per-function, per-dimension; asserts "this
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

- **marker-and-names** — the `# dorc-lang/v0.1` marker gates syntax only; `__role`
  NAME-recognition is permanent and works in unmarked files; names are bare
  munged POSIX NAMEs (dots are dead); families are name-derived, never
  file/author-derived (`271:rul-family`).
- **minting-law** — verdict (`:`/`:!`) and observe (`:?`) marks on runnable lines
  MINT selector tokens; claim/disturbs emissions never mint; a mark asserts
  exactly ONE thing; brace-alternation `@{a,b}` is claim-emission marks only.

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

- **respell-owns-the-churn** — HEAD fixtures still carry `.prop` selectors and
  `touches`/`reaches` names; the corpus-respell sweep owns the conversion
  (`#prop` · `disturbs` · `disturbance_reaches_only`). Never fix piecemeal.
- **polarity-to-transitions** — the lifted effect is a binary bit at HEAD;
  becomes a typestate transition at the entity-algebra-rebuild (this crate owns
  the lift shape; `analysis` owns the transfer).
- **ktyannot-status** — inline annotation is de-facto and IMPLEMENTED
  (`KNOBS:kTYANNOT`; the formal weld is human-reserved). The spike IS the
  livability experiment: record friction in numbered notes; don't relitigate the
  spelling.
