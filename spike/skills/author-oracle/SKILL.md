---
name: author-oracle
description: Author, extend, or review a Dorc oracle - the plain-sh tool-description artifact whose claims Dorc's planner trusts. Load BEFORE writing or modifying any `__role` function (`*__is_converged`, `*__predict`, `*__disturbs`, `*__lend_map`, `kind__*`) or any file carrying a `# dorc-lang/v0.2` marker. Enforces the mandatory reading order, the tool-research protocol, the dialect floor, and the verification gates.
---

# author-oracle

You are about to write an oracle. This skill is imperative: follow its steps in
order, honor its NEVERs absolutely, and route every uncertainty to the mechanisms
it names (declining, flagging upward) rather than to guessing.

## The shape of this task - read this first, and hold it

You are not performing translation. The most likely way for you to fail here is
to treat oracle-writing as a mechanical mapping - "thing I observed about the
tool" to "line of oracle code", one to one - which produces something that LOOKS
like diligent coverage and is wrong in the only way that matters. Nothing about
this task is 1:1.

An oracle is a recording of JUDGMENTS: your considered positions on how this
tool participates in the ops-world. What should "already done" mean for this
verb, on a machine you will never see? Which invocation-shapes deserve an answer
at all? What does a yes tolerate skipping? Which looks-done states are lies? The
shell file is merely the notation those judgments are recorded in - executable
so machines can act on them, readable so humans can audit and second-guess them.
The judgments are the artifact.

Operative consequences, for the whole task: every arm you write is a decision,
and a decision requires a rationale you can state - if you cannot state the
rationale, you have not made the decision, and the arm must decline instead.
Expect soft edges and weighing, not derivation; expect the research below to
change your mind; expect several of your most valuable lines to be deliberate
refusals to answer.

## What you are building, from zero

Dorc is a static-analysis orchestrator for POSIX-ish shell. An admin hands it a
"book" (an ordinary imperative shell script that converges a machine); Dorc
probes the machine read-only, proves some lines already-satisfied, and produces a
plan: the same script with proven-unnecessary lines elided, uncertain-but-likely
lines guarded by a runtime re-check, and everything else left to run. Dorc's core
epistemics: it parses shell fluently (variables, control flow, quoting) but knows
NOTHING about what any external command does. All command-level knowledge enters
through oracles.

An oracle is a file of plain POSIX shell functions whose NAMES wire them up.
`foobar__is_converged()` declares: "for invocations of the command `foobar`, this
body answers 'is this invocation's work already done?'". The name is derived
mechanically (every non-[A-Za-z0-9_] character of the command word becomes `_`,
then `__` + the role; `apt-get` -> `apt_get__is_converged`). No registration, no
manifest. Other roles exist (`__predict`, `__disturbs`, `__lend_map`, per-kind
members); most oracles need only `is_converged`.

The stakes, which shape every rule below: (1) oracle bodies EXECUTE during the
read-only probe phase, on production machines, under Dorc's promise to the admin
that planning never mutates - your body is part of keeping that promise; (2) an
oracle's "yes" (exit 0) LICENSES Dorc to skip or guard a command - a wrong yes
silently un-runs work someone needed, which is the system's cardinal sin; (3) a
decline (exit >= 2) is always safe, always available, and costs only value. When
in doubt, at any decision, at any scale: decline.

## Step 0 - mandatory reading, before anything else (NO exceptions)

Read, in this order, IN FULL:

1. Root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md` - the human-written ground
   truth for what Dorc is and why.
2. `spike/docs/reference/oracle-contract.md` - THE contract. Every obligation,
   license, and failure mode of every member you might write. This is your
   working companion for the entire task; re-open it whenever you are about to
   author a member, and walk its section-8 checklist before you finish.
3. From `spike/docs/writing-oracles/`, at minimum: `02-your-first-oracle.md`,
   `03-the-probe-contract.md`, `05-covering-a-real-tool.md`,
   `11-authoring-with-the-engine.md` (classing your declines, linting as you
   write, reading `dorc why` back), and `10-the-shell-dialect.md`. Read the
   others (`spike/docs/README.md` lists the full path) when your task touches
   their topic: `04` for kinds/marks/binds, `06` for predict, `07` for
   footprints, `08` for wrappers/contexts, `09` for kind ownership, `12` for
   publishing.
4. If you are working inside the spike: `spike/CLAUDE.md` (binding invariants;
   its "authored surface" section is the densest correct summary of the member
   semantics) and the `CLAUDE.md` of any crate you touch.

Do not skim these and reconstruct from your priors. Dorc's contract is unusual
in specific, deliberate ways (silence-as-wall, decline-as-first-class,
attribution-over-prevention) and pattern-matching to "config management tool"
or "test framework" WILL mislead you.

## Step 1 - understand the tool, deeply, before writing anything

An oracle encodes tool expertise. Acquiring that expertise is most of the task;
budget accordingly. Work outward through these rings:

Ring 1 - the invoking site. You were asked to describe this tool for a reason:
some book invokes it. Read every usage site in the book(s) at hand: which verbs,
which flags, which operand shapes, what the surrounding script implies about
intent. This grounds which invocation-shapes matter first.

Ring 2 - the tool itself. Local first: `man` pages, `--help` output, shipped
docs. Then, if web tools are available to you (use them; if network research is
unauthorized or unavailable, say so in your report and mark the affected
judgments lower-confidence rather than silently guessing):

- Search how the tool is ACTUALLY used in the wild - hot-path verbs and flags,
  common idioms, known footguns. Corpus-happenstance research directs your
  coverage effort better than the manual's own emphasis does.
- Read the tool's source code when a Dorc-critical question is undocumented.
  The questions Dorc forces are precisely the ones a non-ops-focused tool
  author never documents: does the status verb write a lock file? does the
  query scaffold a config directory on first run? does `--check` refresh
  metadata anyway? Source (or an issue-tracker search) answers these; the
  manual usually does not.

Ring 3 - the Dorc questionnaire. Before authoring, be able to answer, per
invocation-shape you intend to cover:

- What state does this shape establish, and where does that state physically
  live (which files, kernel, per-user dirs, a remote)?
- What is the cheapest genuinely-inert read that answers "already done"? Inert
  means: no writes even as root (not merely "writes fail as my user"), no
  locks, no cache scaffolding, no log-appending query daemons, no network.
- What is the tool's exit vocabulary, and does it differ from Dorc's fixed
  {0 holds / 1 complement / >=2 cannot-say} table?
- Where does "converged" diverge from "re-running would do nothing"? (Pending
  upgrades, partial syncs, timestamp refreshes - the yes-judgment gaps.)
- Do entity names alias (provides, symlinks, case)? Multi-operand semantics?
  Flags that re-address state (`--root`, `--config`, `-C`)?
- Is the tool environment-sensitive (`$HOME`, locale, XDG)? Is it a wrapper
  (does it exec a guest command)?
- How does behavior drift across the versions actually deployed?

Every question you cannot answer maps to a shape you DECLINE, not a shape you
guess at. Research may include running the tool's read-only verbs yourself where
your environment sanctions it; NEVER run a mutating verb of the tool while
authoring, and never assume a `--dry-run` flag is honest without evidence.

## Step 1.5 - you are writing for the world, not for your invoker

Hold this against the gradient of your situation, which pushes the other way:
the user who invoked you has one book, one tool-site, one itch. An oracle is not
that site's helper - it is a published, shared artifact, consumed by strangers'
books on strangers' machines, held to a global standard. Concretely:

- Cover and decline by the TOOL's real shape (ring-2 hot-path research), not by
  the invoking book's usage. The invoker's verb is where you start, never where
  you stop.
- Never special-case the invoker's arguments, paths, or hostnames. If you
  cannot answer for a shape in general, decline it in general.
- Mint kinds under a defensible global namespace and document them; no
  book-local vocabulary hacks.
- Judgment calls (what your yes tolerates; which verbs you deliberately
  declined and why) go in the file's header comment for the next stranger.

If the invoking user pressures toward a site-specific shortcut, satisfy their
site through honest general coverage or an honest decline - and flag the
tension upward rather than resolving it by narrowing the artifact.

## Step 2 - shell-quality rails

Before writing sh, load your rails, in parallel with this skill:

- If the environment offers shell-authorship skills (anything shaped like
  "shell-scripting", "POSIX", "shellcheck"), load them now alongside this one.
- The canonical references - fetch/search them as needed while writing (URLs
  verified resolving 2026-07-18):
  - the ShellCheck wiki: https://www.shellcheck.net/wiki/ (per-code pages at
    `https://www.shellcheck.net/wiki/SC<code>`);
  - Rich Felker's sh tricks: https://www.etalabs.net/sh_tricks.html
  - David Wheeler's filenames-in-shell essay:
    https://dwheeler.com/essays/filenames-in-shell.html
  - the autoconf manual's Portable Shell chapter:
    https://www.gnu.org/software/autoconf/manual/autoconf-2.68/html_node/Portable-Shell.html
  - the pure-sh-bible: https://github.com/dylanaraps/pure-sh-bible
  - shellhaters.org (POSIX spec map): http://shellhaters.org/
  - Greg's wiki (bash-first - filter its advice through the dialect before
    importing it): https://mywiki.wooledge.org/

The dialect, restated imperative (full rationale in `10-the-shell-dialect.md`):
POSIX plus `local`, nothing else. A stripped oracle must parse and run
identically under pinned `dash` (0.5.12) and `posh` (0.14.1) - that two-binary
run IS the conformance test; use it when the binaries are available, and
`shellcheck` + `checkbashisms` always. Quote every expansion. `printf`, never
`echo` with flags/escapes/variables. `f()` form only. BANNED: `[[ ]]`, `==`,
`${x/}`, `${x^^}`, `${x:off:len}`, `<<<`, `&>`, `|&`, `$'...'`, `function f{}`,
authored `eval`, writing `test -a`/`-o`, bare globs. `local x; x=$(cmd)` split
onto two statements where the status matters. `"${1-}"`-style defaults for
possibly-absent positionals. `--` before operands. `command -v`, never `which`.
Pipefail on durable surfaces only via the self-gating
`(set -o pipefail 2>/dev/null) && set -o pipefail`.

## Step 3 - author, in the contract's growth order

Write `cmd__is_converged()`, then STOP. Do not author further members
speculatively - not because they are forbidden, but because you lack the
engine's knowledge of what is actually missing. The workflow that replaces
speculation: land the verdict member; then, if a runnable Dorc is available
(build with `mise exec -- cargo build --workspace` from `spike/`; invocation
details in the cli crate's `CLAUDE.md` and `spike/crates/cli/tests/e2e.rs`), run a plan over
the motivating book and READ IT. The plan's reason strings and hints name what
is limiting it - which sites stayed unmodeled, which wall degrades the tail,
what one description would recover - with topology-knowledge you do not have.
Author a further member only against a named need: a plan reason, a hint, or an
explicit task instruction; and re-read the oracle-contract's per-member section
(5a-5i) immediately before authoring each. Two riders: before any plan run,
re-verify your bodies' inertness (step 4, item 3) - a plan run EXECUTES probe
bodies, and the read-only promise it keeps rests on the code you just wrote;
and if no runnable Dorc is available, stop at the verdict member and record
candidate next members in your report instead of authoring them.

The tripwire rules, restated dense - violating any of these is a broken
artifact, not a style issue:

- PROBE BODIES NEVER MUTATE. No gradient, no "only a little", no "only first
  run". Read-only by design, not by privilege-starvation. Applies to every
  member body, including `disturbs` emission bodies and `predict` delegations.
- DECLINE BY DEFAULT. `*) return 2 ;;` closes every case; unmodeled flags,
  unexpected arity, missing binaries, unrecognized output, and every surprise
  route to `return 2`. Open delegate-dependent bodies with
  `command -v tool >/dev/null 2>&1 || return 2`.
- CLASS THE DELIBERATE DECLINES. On a decline you can name (never the catch-all
  `*` arm), emit `printf '<verb> <class> <tail>\n' >>"${DREP_V1:-/dev/null}"` on
  the declining path, before `return 2`. v1 verb: `decline`; classes: `unsound`
  (no answer ever), `unmodeled` (a better oracle could), `interactive` (prompts),
  `hazard` (your editorial warning to the admin's book). Leave silent/ambiguous
  declines silent. Put the human-facing reason in a comment ON the arm - it is
  SHOWN to the admin by `dorc why`, never parsed. Keep the format string a
  LITERAL; a dynamically-built format cannot be read statically and demotes the
  class to runtime-only. Classing is aid-only: it never changes an rc or a
  license. Full treatment: `11-authoring-with-the-engine.md` and oracle-contract
  section 6a.
- THE rc TABLE IS FIXED. 0 = the named sense holds; 1 = its complement; >= 2 =
  cannot say, the line runs. NEVER collapse or invert statuses: no `!`, no
  `|| true`, no arithmetic on `$?`. Translate foreign exit vocabularies with an
  explicit `case $? in` remap. Mind pipeline tails; prefer shapes where the
  tool under description produces the status directly, and full-read pipe forms
  (`grep x >/dev/null`) over early-exit `-q`.
- PREFER THE MARKED-COMMAND FORM: the tool reads the state and its exit status
  IS your answer (`kp check "$1" "$2" : sm.dorc.KernelParam:"$1"`). Avoid
  capturing output into a variable and string-comparing - that shape is opaque
  to downstream machinery and loses value.
- A YES IS A JUDGMENT. Exit 0 means "not running this invocation is acceptable,
  including everything it would have done beyond the checked state". Where the
  tool's semantics make looks-done a bad reason to skip (purge verbs, remote
  syncs, upgrade-pending), decline permanently and comment why.
- MULTI-OPERAND: check every operand or decline the shape. A partially-checked
  yes is a wrong yes.
- MARKS AND BINDS (only in a file carrying `# dorc-lang/v0.2`, exact, own line,
  first ~10 lines): trailing `: KIND:ENTITY@selector` = verdict mark; `:!` =
  complement sense; `:?` = observe (disclose every extra cell your verdict
  reads). Meta facts spell their verb as a word: `: disturbs KIND` (footprint),
  `: safe-across DIM` (context vouch), `: lends DIM` (wrapper dimension),
  `: stored-in SUBSTRATE` / `: undivided-by-transit-across AXIS` (kind store).
  At most one verdict per line; one mark per physical line in production (an
  extra read is its own line). Binds (`x : KIND = "$1"`) name entities, never
  cells. Kinds are reverse-DNS >= 2 dots; reuse existing kinds as their owners
  document; never invent tokens outside the engine's closed vocabularies.
- FOOTPRINTS (`__disturbs`) are at-most claims: matching a shape asserts you
  enumerated EVERYTHING it disturbs. Include cells when unsure; leave the whole
  shape unmatched when the enumeration itself is unsure. A wrong footprint
  silently breaks OTHER people's lines - the sharpest knife in the system.
- WRAPPER MEMBERS (`__lend_map`, entry forms) and KIND MEMBERS (`__resolve`,
  `__disturbance_reaches`, `__state_stored_only_in`): do not author from
  this skill's summary - read oracle-contract 5d-5i and
  `08-wrappers-and-contexts.md` / `09-owning-a-kind.md` first, every time.
  `only`-named members demand a totalistic survey before authoring; the
  `disturbs nothing-else` tail record (reach bodies; dynamic `disturbs` bodies)
  is a completeness claim of the same weight - one line, the sharpest in the
  file.
- NAMES ARE PERMANENT. Function and kind names are a compatibility surface;
  choose once, evolve additively.

Style: the file must be excellent PLAIN SHELL first (it strips to a defensive
library; that off-ramp is a core product promise). Header comment: which tool,
which verbs covered, which deliberately declined and why. Then comment the
JUDGMENTS generously, in place: an arm's reason for answering, a decline's
reason for declining, a return-0 path's tolerance rationale - elaborated enough
that the human reviewing you, and the stranger deciding whether to install the
file, can second-guess the decision without redoing your research. This is not
comment-noise: an oracle is decisions falling out of rationale, and the
rationale is part of the artifact. What stays banned is mechanical narration -
comments restating what self-evident shell does.

## Step 4 - verify before you call it done

1. Walk `oracle-contract.md` section 8 (the battle-grade checklist), line by
   line, against your file. Actually walk it; do not assert it.
2. Run `dorc lint` over your oracle file(s) - it needs no book and contacts no
   host, and it is the single gate that runs the mechanical checks: `shellcheck`
   and `checkbashisms` (remapped to your original lines), dorc's own verdict-body
   and dialect checks, and your decline inventory. Walk the findings; read the
   decline inventory back and confirm every deliberate decline is classed the way
   you meant and reads statically (literal formats). If the pinned floor binaries
   are available, strip and run under both.
3. Re-verify inertness: for each delegate invocation in a probe-executed body,
   state (in your report, with evidence-grade) why it cannot write, as root,
   on a fresh host.
4. If the spike's harness is available and the task includes fixtures: e2e
   fixtures use INERT MOCKS only (`PATH=mocks-only` stubs) - never real
   mutators; the central e2e runner (`cargo test -p dorc-cli --test e2e`) is
   the only sanctioned executor of fixture material.
5. Report honestly: every judgment call with its rationale; every shape you
   declined and why; confidence marks (+SURE / ~SUSPECT / -GUESS) on the
   claims your research could not fully ground; every question that needs a
   human ruling. Your oracle is a set of signed claims - the report is where
   you show the signing was informed.

## Further-research seeds (open ONLY when confused, unsure, or facing a subtlety)

Everything below points into `Research/` - the project's internal planning
corpus. Know what you are getting into: it is LLM-generated, vast, dense with
project jargon and superseded layers; documents are only lightly annotated when
later work overrides them. The rule inside: `Research/README.md` is the map;
newest supersedes oldest; the root docs and `spike/CLAUDE.md` outrank all of it.
Enter for a specific question, extract the answer, and get out - do not
skill-up on the corpus wholesale from inside this task. NEVER enter
`Research/notes/quarantine-DO-NOT-READ/` or `Research/corpora/`.

- Dialect exactness (charsets, mark grammar, coordinate forms):
  `Research/notes/278-dorc-lang-v0-1-reference.md` (one page, assembles the
  typed rulings), then `Research/notes/277-entity-algebra-design.md` section 4
  for the grammar itself.
- Why the contract is shaped this way (verdicts, guards, silence-as-wall):
  `Research/notes/23O-round-23-closeout.md` - the settled law and its history.
- Dialect membership questions ("is this construct in?"):
  `Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md`.
- Wrappers, contexts, sudo-class questions:
  `Research/plans/27C-context-entry-probing-design.md` (THE current spec; its
  section 10 separates ruled from strawman).
- Stdlib-specific obligations (if your oracle targets the standard library):
  `Research/notes/27Q-stdlib-handoff-preconditions.md` section 2 - binding
  teachings, including the marked-command-form mandate and the quality bars.
- Named design tensions (when you sense two goals pulling apart): root
  `KNOBS.md` - reuse its slugs; never re-derive a tension under a new name.
- Living examples of the current as-built spelling: `spike/e2e/cases/*/`
  oracle fixtures (e.g. `context-entry-babby-elides/`, `carry-fsview-elides/`)
  - real, parsed, tested; but remember they are test fixtures with inert-mock
  bodies, not models of tool-research depth.

When a corpus dive surfaces a contradiction with the docs you read in step 0,
do not resolve it yourself: the corpus outranks the docs on design truth, but
your task is the oracle - flag the contradiction upward and proceed on the
corpus reading with a confidence mark.
