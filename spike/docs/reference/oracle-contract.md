# The oracle contract

This is the complete reference for what an oracle author may write, what each
thing they write licenses Dorc to do, what they are obligated to in return, and
exactly how each obligation fails when neglected. It assumes the concepts taught
in `writing-oracles/` and does not re-teach them; it is meant to be open in a
second window while you write anything you intend to publish.

A stability note governs the whole page. Three tiers of this surface have three
different lifetimes: role-function names are permanent (a compatibility surface
that will never be re-read to mean something else); the annotation syntax is
versioned by the file marker (a marked file keeps parsing as its version said);
and the engine's verdicts - what actually elides on a given day - are explicitly
unstable and improving. Design your oracle against the first two; never against
the third.

## 1. The standing rules of the world

Every member-specific contract below operates inside these; they are never
suspended and never traded against performance.

- Probing is read-only. Anything that executes at plan time is oracle-authored,
  and its author has accepted the no-mutation contract. There is no fallback,
  no gradient, and no acceptable amount of probe-time mutation.
- Apply never skips needed work without a license. Ambiguity, confusion,
  timeout, and silence all resolve toward running the admin's line. Skipping
  needed work is the system's cardinal failure; every license that permits
  not-running is explicit, authored, and attributed.
- Silence licenses nothing. An unmodeled command, verb, flag, dimension, or kind
  is unknown - a wall - never a permissive default. Adding description only ever
  adds capability at your own tool's sites; it cannot weaken anyone else's
  safety. (The one deliberate exception: wrong at-most speech - a footprint, or
  a finished definition - sections 5c and 5h.)
- Only oracle bytes execute during probing. Book commands never ship; a site's
  arguments flow into your functions through your own argument parsing, but the
  admin's written command-position bytes are never executed by the probe lane.
- The book's order is sacred. Nothing you write can cause reordering; elision
  and probe-parallelism are the only speed mechanisms that exist.
- Everything is attributed. Every elision names the answering function; every
  survival names the claims it rested on; every entered context names the
  consents that licensed it. *Publishing an oracle is accepting that your name
  is in that chain.*

## 2. The family: names, roles, extension

A family is the set of `__role` functions describing one target, and membership
is by name construction alone - never by file, never by author. Two species:

- Command families: `<munged-command>__<role>` - `foobar__is_converged`,
  `apt_get__disturbs`, `sudo__lend_map`.
- Kind families: `<munged-kind>__<role>` - `sm_dorc_Package__resolve`,
  `sm_dorc_KernelParam__state_stored_only_in`.

The munge is total and mechanical: every character of the command word or kind
name that is not a letter, digit, or underscore becomes an underscore. Munged
names are plain POSIX function names, recognized even in unmarked plain-sh
files, and permanent once published.

The role vocabulary is owned by the engine, closed at each version, and extends
only by introducing new names - an existing role name will never be re-read to
mean something different. Current per-command roles:
- `cmd__predict()`,
- `cmd__is_converged()`,
- `cmd__disturbs()`,
- `cmd__lend_map()`,
- `cmd__enter()` (still provisional).

Current per-kind roles:
- `kind__resolve()`,
- `kind__disturbance_reaches()`,
- `kind__state_stored_only_in()`.

The `only` naming convention binds authorship posture: *a role with `only` in
its name is complete-by-contract* - consumers act on its negative space, so
authoring it requires a totalistic survey first. A role without `only` grows arm
by arm, each matched arm complete for its own shape only. The reach member lost
its `only` deliberately: its completeness contract is a spelled act inside the
body (section 5h), not a property of the name.

## 3. The answer channel: exit-status law

Verdict-bearing members answer through their final exit status, read against one
fixed, permanent table:

| status | meaning | consequence |
|---|---|---|
| 0 | the named sense holds | may license guard and elision |
| 1 | the complement holds | the line is needed; runs |
| 2 or higher | cannot say | the line runs, always |

Only 0 and 1 ever carry a verdict; 2 communicates "I cannot meaningfully speak
for this" (collapsing error-states, NYI, instability, and so on). Everything at
2 and above is currently one flat "confused" sink, semantically flat. The table
binds verdict-bearing members alone: a predict's exit status is a prediction of
its tool's status, never an answer read against this table (section 5b).

- Stray away from 'flattening' shell-vocab operators like `||` that will paper
  over exit-status-semantics; and *extremely avoid shell-flipping-and-flattening
  tools like `!`* - they will incorrectly flip tool "error" and other confused
  statuses into convergence!
- Instead, preferentially translate foreign exit vocabularies explicitly (`case
  $? in`) whenever the delegate's dialect differs from the table - especially
  tools whose 2 means something definite.
- Mind pipeline tails: a body ending in a pipeline answers with the tail's
  status (or the pipeline's, under pipefail); make sure that status is the
  answer you mean, and prefer shapes where the tool under description produces
  the status directly.
- Route every surprise to 2+: missing binaries, unrecognized output, permission
  oddities. `command -v tool >/dev/null 2>&1 || return 2` is the standard gate.

## 4. The grammar: marker, coordinates, marks, binds

The marker. `# dorc-lang/v0.2`, exact, alone on its line, within roughly the
first ten lines. Gates the syntax below for this file (and only the syntax; role
names need no marker). An unmarked file is treated as plain shell; our
annotations become syntax-errors. A file whose marker names a version this Dorc
does not recognize is a loud diagnostic, never a silent downgrade to plain
shell. Add the marker at the moment a file first uses a bind, a mark, or the
`dorc:` prefix; a role-functions-only file needs none.

Two carriers, one grammar. Every mark rides one of two interchangeable carriers,
chosen per physical line; the grammar after the carrier is identical.

- The colon form (`:`) is the default. It lexes and highlights as ordinary shell
  in every editor checked, so a correctness-critical mark stays visible. Its cost
  is a real, accepted hazard: if an unstripped marked file reaches a shell by a
  route the intended paths do not cover (piped to `sh`, `source`d,
  stripped-then-forgotten), a colon mark corrupts that command's arguments, and a
  standalone colon mark forces the line's exit status to 0. The `dorc-sh` shebang
  catches a plain run; the hazard is what remains after that net.
- The hash-colon form (`#:`) is an opt-in comment carrier, for marks about
  genuinely dangerous state. It is a real comment - inert under any shell, on
  every route, even unstripped - so it never corrupts and never forces a status.
  Its cost is highlight demotion: the leading `#` greys the mark in some
  renderers. It is told apart from an ordinary comment, and from the version
  marker, by the colon touching the `#` with no space (`#:`, never `# :`).

The intro and its head sugar. A mark opens with the carrier, optionally followed
- with no intervening space - by one sugar character, then a space. The eight
legal intros:

    :    :!   :?   :=          the colon carrier: plain, and one per sugar
    #:   #:!  #:?  #:=          the same four on the comment carrier

The four sugars are head-only shortcuts for the core relations a line states
about a cell or a value - plain is `asserts`, `!` is `refutes`, `?` is `reads`,
`=` is `bind` - and apply only to the first mark on a line. Every other relation
is spelled as a word verb (below), and a core relation moved off the head is
spelled with its word too.

Coordinates. `KIND:ENTITY@SELECTOR`, with the tail parts optional:

    sm.dorc.Service:"$svc"@enabled      one cell of one entity
    sm.dorc.Package:"$pkg"              a whole entity
    sm.dorc.PkgIndex                    a whole (singleton) kind

- Kind: reverse-DNS, two dots minimum. Mint only under a domain you plausibly
  answer for; reuse others' kinds only as their owners document.
- Entity: written bare when it contains only letters, digits, `.`, `_`, `-`,
  `/`; double-quoted (with normal `"$var"` interpolation) otherwise.
- Selector: an identifier (letter or underscore first, then letters, digits,
  underscores). `@` introduces it, attached with no space - to the entity, or
  to the `:` in the entity-less transitional form `KIND:@SELECTOR`. (`@` lexes
  as an ordinary word character under the floor shells and highlights cleanly,
  which is why it, not `#`, carries the selector.)
- A selector-less coordinate means the whole entity - and, on the consuming
  side, it interacts with every cell of that entity. Reach for a selector
  whenever you can name the aspect you actually measured; reserve the bare
  form for claims that genuinely concern all of it.
- Polarity never appears in a coordinate; it rides the verb (`asserts` /
  `refutes`).

The keystone that keeps a verb apart from a coordinate: a verb never contains a
period, and a kind always contains at least two. That single rule decides the
head of a mark; everything after the head is read verb by verb, so a payload may
contain periods freely.

Marks. A mark trails a statement (binding to its exit status or value) or stands
alone on its own line (scope per its verb's member). Write one on any line whose
result deserves an address: marked facts are what plans report by name, what
disturbance-tracking keys on precisely, and what the survival machinery can
reason about. An unmarked check still works, but its fact is anonymous and
handled maximally conservatively.

    cmd args   : COORD      verdict: exit 0 asserts the cell holds
    cmd args   :! COORD     verdict, complement sense: exit 0 asserts it does not
    cmd args   :? COORD     observe: this statement reads that cell

Off the head those same three read as the words `asserts`, `refutes`, and
`reads`; the sugar is only a shortcut for the first mark. The full verb set is
engine-owned and closed, extended only by introducing new names:

- Core cell-and-value relations, sugar-eligible: `asserts` (plain), `refutes`
  (`!`), `reads` (`?`), `bind` (`=`).
- Meta relations, always word verbs: `safe-across` (a context vouch, section
  5f), `disturbs` (a footprint, sections 5c and 5h), `lends` (a wrapper
  dimension, 5d), `stored-in` and `undivided-by-transit-across` (a kind's store
  and its invariance, 5i).

The rules on marks:

- Verdict and observe marks mint selector tokens into the kind's vocabulary, and
  attach facts to the one line that measured them.
- At most one verdict (`asserts` or `refutes`) per line. A line has one exit
  status, and a verdict maps that one status onto one cell's truth; two cells
  would need two statuses, and which cell diverged is what decides what runs.
  This is the whole content of the old "one assertion per line" rule. Observes
  and every meta mark do not consume the status.
- An observe (`:?`) elsewhere in a verdict body widens that fact's staleness
  surface (its backing) to include the observed cell - always safe, often
  obligatory for honesty. Write one, as its own statement, whenever your verdict
  consults state beyond the cell it answers for.
- Emission members type what they emit with a verb-led mark. `cmd__disturbs()`
  and `kind__disturbance_reaches()` write `: disturbs KIND` or
  `: disturbs KIND@SELECTOR`; `cmd__lend_map()` writes `: lends DIMENSION`;
  `kind__state_stored_only_in()` writes `: stored-in SUBSTRATE` plus, per whole
  member, `: undivided-by-transit-across AXIS`. All these token vocabularies are
  engine-owned and closed; authors never invent tokens. (The reach member's
  finished-definition sentence is not a mark at all - it is a report-stream
  record, section 6a - because it must witness runtime completion, which no
  annotation can.)
- Brace alternation is a general shortcut for "several payloads, or one payload
  with a varying part": `@{enabled,active}` expands to one selector each,
  `safe-across {user,fs-view}` to one mark per dimension. It is legal wherever
  several cells are meaningful - selectors, observes, disturbs emissions - and
  refused only where it would forge a multi-cell verdict, since one status
  cannot witness two.

What this engine reads today. The grammar describes a mark-block: several marks
may share one physical line, and a block may spill onto continuation lines (each
re-opening with its own carrier). Today's engine reads one mark per physical
line, so an extra read or a second meta claim is disclosed as its own line. The
block form is specified in the grammar spec and will be adopted later without
changing any spelling written now.

Binds. `name : KIND = "$value"` assigns and declares the value an entity of the
kind. Binds name entities, never cells; strip reduces a bind to the plain
assignment. Write one when an operand you received is an entity you are about to
make claims about: the bind lets the analyzer carry the kind through your body's
value-flow and back out to the book's site, so the plan's reason can name the
concrete entity ("converged: org.foob.Certs:/etc/nginx/certs@synced") rather
than a positional argument. The usual rhythm is one bind per entity-shaped
operand, placed where the operand is first received; every later mark on that
value then inherits its identity. Skip binds for values that are not entities
(counts, modes, free text) and for entities you never mark - an unused bind is
noise, not safety.

The inline form above is the one this engine reads. The grammar also defines a
trailing bind that rides an assignment - `FOO="bar" := KIND` (sugar) or the word
`: bind KIND` - which is what would let the whole annotation surface sit on `#:`
comments; but that trailing form is not yet accepted in production (it is
diagnosed, not parsed), so write the inline bind for now.

The `dorc:` prefix. `dorc:sh -c '...'` is the one prefix-position spelling:
full-analysis invitation on an interpreter head. Bare `sh -c '...'` is the
permanent escape hatch (hints only, licenses nothing); `dorc-sh` typed directly
is the runtime object and is left alone by strip. No annotation syntax is ever
recognized inside opaque payload strings.

## 5. Per-member contracts

Each member below states: how it is invoked, when to write it, what its output
means, what it licenses, what its author must hold true, and how it fails. The
probe contract (read-only, fast, reentrant, answer-from-durable-state,
fail-toward-2) applies to every body in this section without further mention.

A typical oracle grows through the members in a stable order, and the order is
itself when-guidance:

1. `cmd__is_converged()` - first, always; most oracles rightly stop here.
2. `cmd__predict()` - when your tool starts appearing inside compound
   constructs and admins' hand-guards, and the hints say modeling would unlock
   them.
3. `safe-across` - when books wrap your tool's sites (sudo and friends) and you
   have re-audited the vouched body for shifted execution.
4. `cmd__disturbs()` - when your tool is the churn-heavy early wall that costs
   drifted-day books their shape, and you can survey its verbs completely.
5. `cmd__lend_map()` and `cmd__enter()` - only if your tool is itself a
   wrapper.
6. The `kind__*` members - only if you own a shared vocabulary.

### 5a. `cmd__is_converged()` - the verdict member

Invoked with a site's arguments (everything after the command word, as the
book's values resolved). Answers per section 3, where the named sense is "the
state this invocation exists to establish already holds."

Write it first, for any tool. Even for tools that are as cheap to run as they
are to test (think `mkdir -p`), Dorc benefits from a read-only test that it can
parallelize, and that can drive later elisions. ("If `mkdir -p` *might* change
the filesystem, I cannot safely skip later command `x`. If I can *check*, then I
can skip both.")

Licenses: at this tool's own sites only - insertion of this body as a runtime
guard (`( cmd__is_converged args ) || original-bytes`), and, when a probe-proof
applies, full elision of the line. The guard always preserves the original bytes
and always falls through to them on any non-0 answer. The vouch is inadmissible
everywhere else: it never becomes a fact, never informs another site's
reasoning, never transfers to another tool.

Author holds true:
- That an answer of 0 means not-running this invocation is an acceptable
  outcome; all of it, including effects beyond the checked state (converged is
  not the same as no-op; the pending-upgrade case is yours to judge).
- That the body declines every shape not deliberately modeled: unknown verbs,
  unmodeled flags (especially state-addressing ones like `--root`), operand
  counts beyond what is checked.
- That multi-operand shapes are either fully checked (every operand) or
  declined; a partially-checked yes is a wrong yes.

Failure modes:
- a wrong 0 causes this tool's line to be skipped or guarded-away when it was
  needed - under-execution, at your own tool's site, attributed to this function
  by name. A wrong 1 merely runs a converged line (safe, noisy).
- A mutating body breaks the probe promise itself (see section 7, first entry).

### 5b. `cmd__predict()` - the modeling member

Invoked with an invocation's arguments. Stands in for the command inside probe
constructs: its stdout, stderr, and exit status are consumed as the command's
predicted observables. The channel rules (this is the surface that most
recently shifted; the spellings below are current):

- Status is always predicted, and every value of it: the body's ordinary exit
  status simply is the prediction, whatever it is. There is no reserved
  decline status here; section 3's table belongs to verdict members alone.
- Stdout and stderr are declined by default. A body claims one positively by
  writing a `predicts` record to the report stream (section 6a) after the
  modeled output - `printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}"` -
  and the record deliberately trails the bytes it vouches for: its arrival is
  the completion witness, so a body that dies partway leaves an unclaimed,
  unusable channel rather than a half-true one.
- A shape the body does not model is refused out-of-band, with the whole-shape
  decline record - `printf 'predicts none unmodeled %s\n' "$1"
  >>"${DREP_V1:-/dev/null}"` - where `none` declines every channel at once
  (sugar over the per-channel vocabulary; the tail is free reason text). These
  are the first lines to write, not the last: fence every unexplored shape
  before modeling any. A predict never spells refusal through its status; a
  `return 2` here is nothing but a prediction that the tool exits 2.
- Delegation of the real (read-only) tool remains the natural body shape; the
  channel claims are still yours to speak, channel by channel.

Write it when your tool appears inside constructs rather than alone on lines:
pipelines and compounds cannot probe without a stand-in for every participant,
and an admin's hand-written guard invoking your tool lifts only through your
predict. The plan's hints point at exactly these sites; until they do,
`cmd__is_converged()` alone is usually the better spend.

Licenses: substitution of this body for the tool inside composed probes and
lifted hand-guards, but only where every channel the surrounding construct
consumes is covered by the body's claims. Recognition of the peel shape (a
body whose `"$@"` runs its own argument-slot) classifies the tool a wrapper.
A predict never licenses eliding anything by itself.

Author holds true:
- That every unexplored shape is fenced with a `predicts none` record before
  any shape is modeled; refusal never rides the status.
- That every claimed channel is faithful for every matched shape, on hosts
  unlike theirs.
- That no matched shape mutates.
- That convenience channels not thought through are declined, not guessed.

Failure modes:
- A wrong channel claim corrupts a composed probe's result, which can surface
  as a wrong verdict for some enclosing construct. Bounded by the coverage
  rule (unclaimed channels block substitution); attributed to the predict that
  claimed.

### 5c. `cmd__disturbs()` - the footprint member

Invoked with a site's arguments. For a matched shape, emits the disturbed
entities one per line on stdout, each typed by a trailing `: disturbs` mark
(`: disturbs KIND`, or `: disturbs KIND@SELECTOR` for a single cell; the
bare-kind form is the whole-kind claim). A matched shape's emission is a
complete at-most claim: this invocation disturbs
at most these cells, and anything omitted is declared untouched. An unmatched
shape emits nothing and claims nothing.

A body whose emissions come from host questions (a dynamic footprint - the
package manager asked for its real payload) carries one more obligation: every
completing matched path ends with the completion record, exactly once -

    printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"

The record trails the emissions it vouches for, so its arrival proves the
survey ran to completion; a body that dies or silently truncates leaves no
record, and the whole claim is refused - the wall stands total. (A purely
static body's emissions are read from source and need no witness.)

Write it when your tool is the wall that costs drifted-day books their shape:
churn-heavy, early-in-book commands (index refreshes, cache warms, log
rotations) whose effects you can genuinely enumerate. Do not write it
speculatively - an unsurveyed verb is better left unmatched, and a tool that
rarely runs mid-book earns little from a footprint.

Licenses: under the admin's explicit risk flag only - survival of downstream
proven facts past this command actually running, wherever fact-backing and
claimed footprint are provably disjoint. Disjointness within your claim's own
kind is the comparison machinery's to prove (names, resolvers); across kinds
it exists only where the claimed kind's owner has published a finished
definition (section 5h) - absent one, your claim walls, whatever it says.
Never consumed outside the flag; never able to manufacture separation the
comparison machinery cannot prove.

Author holds true:
- The survey: match a shape only after genuinely enumerating that shape's
  effects.
- When unsure whether some cell is disturbed, include it - over-claiming only
  walls; under-claiming under-executes.
- When unsure the enumeration is complete, do not match the shape at all.
- Selector-precision is welcome refinement of a complete survey, never a
  substitute for one.

Failure modes:
- The sharpest in the system: an omitted cell in a matched shape can silently
  un-run someone else's line - a different tool, author, and file - with no
  runtime net.
- The bite is flag-gated (the admin typed for it), fully attributed (the
  why-machinery names this member), short (the next plan re-probes reality
  and the line returns), and narrow (every other line held its own license) -
  but it is real, and it is the one place your mistake spends other people's
  safety. Author accordingly.

### 5d. `cmd__lend_map()` - the wrapper dimension member

Invoked with the wrapper's own arguments; body parses the wrapper's prefix
exactly as the tool does and ends with `"$@"` at the guest position. Emits one
entry per dimension: a valued line (`printf '%s\n' "$target"   : lends user`)
maps that dimension; a bare `: lends fs-view` passes it through unchanged; a
dimension with no entry at all is unknown and walls.

Write it whenever your tool is a wrapper at all: without it every wrapped site
is opaque, and the guest's own oracle never even gets consulted. It is the
highest-leverage member a wrapper family has - one small map un-walls every
book that uses the wrapper.

Licenses: interpretation of wrapped sites - which context the guest denotes,
per dimension - feeding context keying, entry composition, and chain folding.

Author holds true:
- Enumerate-every-dimension: say "unchanged" explicitly, per dimension.
- The peel position agrees with the predict's (checked statically;
  disagreement is refused at plan time).
- Emitted values are single-link truths; the engine owns composition across
  chains - never pre-compose nesting yourself.

Failure modes:
- A wrong lend value mis-keys facts to the wrong context - the
  measured-wrong-world class, capped by the entry-form's own siting duties.
- A missing dimension merely walls (safe, value-losing).

### 5e. `cmd__enter()` - the context entry form (name provisional)

Body is ordinary pre-entry shell ending in `"$@"` verbatim in command position
(`sudo -n "$@"`). Invoked at probe time with a composed check in guest
position, under the admin's escalation dial.

Write it when the contexts your wrapper denotes hold describable state worth
probing in place - and only when a non-interactive entry whose siting you can
verify (or decline) exists. A wrapper with no trustworthy entry story is
better off without one; wrapped sites still get verified by guards at apply.

Licenses: real context entry in the probe lane - the only licensed seat for
it. (Predict closure bodies never escalate; a wrapper with no entry form has
contexts that are simply never entered.)

Author holds true:
- Non-interactive by construction: fails rather than prompts, ever.
- Self-effects acceptable as probe residue, answered for by name (the
  auth-log class).
- Siting: entry through this form lands the guest in the same context the
  site's own bytes would reach - verified structurally, by interrogating the
  tool, or by a tripwire - and *declined* (2+) where unverifiable.
  Policy-routing wrappers make siting genuinely hard; a declined entry is
  always correct and costs only value.

Failure modes:
- A wrongly-sited entry measures the wrong world and can produce confident
  wrong verdicts - the worst object the probe lane can emit; hence the
  decline duty.
- A prompting entry hangs or fails probes (contained by the non-interactive
  construction).
- A mutating entry is a probe-contract break.

### 5f. The `safe-across` vouch (a mark, not a member)

A standalone mark inside a function body (`: safe-across user`;
brace-alternation `: safe-across {user,fs-view}` for several dimensions), scoped
like any statement to the paths that reach it.

Write it per function, when books genuinely wrap your tool's sites and you
have re-audited that body for shifted execution. There is deliberately no
blanket file-scope form: vouch the dimensions you have actually thought about,
function by function - and arm by arm, where verbs differ.

Licenses: executing that function in contexts shifted along the named
dimensions, under the admin's dial.

Author holds true:
- The body's effects are read-only by design, not by privilege-starvation:
  shifting it (notably to root) does not unlock writes it was silently
  attempting all along.
- The vouch claims nothing about answers (answers are supposed to vary per
  context) and nothing about other functions.

Failure modes:
- A false vouch is a probe-contract break executed in an entered context -
  bounded by the dial, attributed to the three consents involved.

### 5g. `kind__resolve()` - the canonicalizer

Invoked with an entity name of its kind; prints the canonical name, falling
through to the input for names it cannot answer. One resolver per kind in a
loaded world; duplicates are refused loudly.

Write it when your kind's names alias in the wild - package provides,
symlinked paths, case-folded names - and the substrate offers an authoritative
canonicalization query to delegate to. A kind whose names are canonical by
construction needs none, and loses nothing: plain name-comparison remains the
floor.

Licenses: canonicalization of both sides of every same-kind comparison -
aliased names collide correctly, distinct names stay distinct.

Author holds true:
- Conservative resolution: delegate to the substrate's own authoritative query
  where one exists; map unknown names to themselves.
- Uncertainty resolves toward echoing the input - never invent a merge, never
  invent a split.

Failure modes:
- A wrong merge over-verifies: needless collisions, guards where elisions were
  earned. Safe, value-losing.
- A wrong split re-opens the silent-skip hole the resolver exists to close -
  under-execution at some consumer's line, attributed to the resolver.

### 5h. `kind__disturbance_reaches()` - reach, and the finished definition

Invoked per footprint coordinate of its kind, whoever emitted it; emits the
implied coordinates in other kinds (footprint emission grammar; static lines
and read-only host-question lines both welcome). Two acts live in one body, on
two rungs:

- The emission lines are the entailment - part of what the kind's word means.
  They widen every footprint of the kind to cover what touching such an entity
  drags along, which only ever adds collisions: the safe direction, writable
  from partial knowledge, arm by arm. This is why the member's name carries no
  `only`.
- The finished-definition record, in tail position on a path whose survey is
  truly total, exactly once per completing path:

      printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"

  It says: disturbing an entity of this kind entails the emitted cells and
  nothing else, in any vocabulary, including ones this file has never heard
  of. Its arrival is also the execution witness - a dying host-question line
  means no record, no license, total wall.

Write the emissions when disturbing an entity of your kind implies effects in
other kinds that emitters cannot know from their seat - the
package-to-its-files case. The knowledge must be the kind's own, derivable
from its substrate; a guess about particular tools' behavior belongs in those
tools' `disturbs`, not here. Write the record only where you have genuinely
finished the survey for that shape; a body (or an arm) without it is the
informative rung - it contributes collisions and licenses nothing, and that is
a legal, useful resting state.

Licenses: the emissions widen footprints (footprints only; never what any fact
claims for its own backing). The record is what licenses cross-kind sparing at
all: only past it will the engine find one of this kind's claims disjoint from
another kind's fact and let an elision survive a running wall. Without it,
cross-kind pairs answer unrelated, and the facts behind the wall guard - safe,
blunter than the world.

Author holds true:
- Emissions: include when unsure - over-breadth only walls; partial knowledge
  is welcome, any time.
- The record: a totalistic survey first, per matched shape; when unsure the
  enumeration is complete, leave the record off that path and keep the
  emissions.

Failure modes:
- The record written early is this member's knife, and it is the sharp one: a
  fact in a kind you never heard of survives a wall that really touched its
  referent - someone else's line, silently un-run, behind the admin's flag.
  Attributed to the record's own line.
- An omitted emission edge without the record merely walls (safe,
  value-losing). Over-breadth merely walls. A dying survey merely walls - the
  missing record refuses the claim whole.

### 5i. `kind__state_stored_only_in()` - the store member

Emits the substrates where the kind's state lives (`: stored-in SUBSTRATE`
lines), plus zero or more whole-member invariance declarations
(`: undivided-by-transit-across AXIS` lines).

Write it when your kind's facts deserve to travel: state that is one store
machine-wide (kernel parameters, say) can answer across filesystem views, and
crisply-located state benefits from substrate keying either way. If you
cannot state where the state lives without hedging, the kind is not ready for
this member.

Licenses:
- Substrate keying of the kind's facts.
- Collision: a claimed footprint landing inside a declared store collides with
  this kind's facts - the safe direction, so an incomplete emission set here
  loses protection, never correctness.
- Per invariance line, cross-context carry of the kind's facts along that
  dimension: unflagged for substrate dimensions (the engine independently
  verifies each carried verdict body read nothing beyond its arguments and
  marked reads; bodies failing that structural check simply do not carry);
  flag-gated for identity dimensions.

Author holds true:
- The `only` contract on the emission set.
- Invariances true of the substrate itself, not of your test environment.
  Known-refused combinations are enforced (network-state kinds cannot claim
  netns-invariance); contradictions between a declared invariance and the
  member's own emissions are refused at plan time.

Failure modes:
- A false invariance line lets a fact measured in one world answer for
  another where the answer differs - under-execution of the consuming site,
  attributed to this line.
- The engine's structural check narrows but does not replace your duty: it
  verifies the measuring body's reads, not your claim about the substrate.

## 6. The probe execution environment

What a body may assume when it runs:

- It receives argv only. No environment contract exists beyond what your own
  file establishes; assume `set -u`-grade strictness in consumers.
- It may run many times, concurrently with other bodies, batched with
  strangers' bodies on the same host.
- In the apply lane the identical bytes run as guards; nothing may depend on
  "this only runs at plan time".
- Entered bodies run in shifted contexts exactly when their vouches and the
  dial align.
- Its exit status is the entire in-band answer - read against section 3's
  table for a verdict member, consumed as the predicted status for a predict.
  Dorc's own signalling travels out-of-band; no exit code means "unknown"
  except as section 3 defines.
- Its stdout and stderr are consumed only where a contract says so (predict
  channels, emission grammars); verdict bodies should run quiet.
- Refusal breadcrumbs and classed declines go to the report stream (subsection
  6a); its transport, grammar, and noise-tolerance are settled below, only the
  sink name still strawman.

### 6a. The report stream and authored declines

The report stream is a write-only side channel a body may append to. Its
records are verb-led: a short, closed-grammar structured head, then a free
tail. Heads are real speech - several record kinds below carry claims the
engine acts on - while free tails and unrecognized lines are annotation only,
never read by anything that decides. The verb vocabulary is engine-owned and
append-only; three verbs exist today:

- `decline <class> <tail>` - the classed refusal accompanying a verdict body's
  in-band `return 2`, naming which of the four kinds of no this is.
- `predicts <channel-set>` and `predicts none <reason>` - a predict body's
  channel speech (section 5b). Load-bearing both ways: a positive claim
  licenses substitution, and `predicts none` is the whole-shape refusal - a
  predict's only refusal, its status being reserved wholly for prediction.
  Fencing the unexplored with these is among the first and most primary things
  a predict author writes.
- `disturbs nothing-else` - the completion witness of dynamic at-most bodies
  and the finished-definition act of reach bodies (sections 5c and 5h).
  Load-bearing: it is the license for cross-kind sparing.

For every load-bearing record, position is part of the meaning: it trails what
it vouches for, so its arrival proves the body reached it.

- Sink. A body writes with `>>"${DREP_V1:-/dev/null}"`. The probe lane sets the
  variable to a file inside a scratch directory it created exclusively for this
  run, and drains that file afterwards. Where it cannot establish such a
  directory it sets the variable to `/dev/null` instead and captures nothing —
  your writes stay total either way, exactly as they are off-Dorc, where the
  `:-/dev/null` default makes every write a `set -u`-safe no-op. Nothing you
  write ever needs to change: the variable's value is chosen by the engine, per
  run and per execution environment, and your side of the idiom is fixed. The sink's env NAME carries the
  stream's format version (strawman `DREP_V1`); a future format mints a new name,
  and a recognized name is permanent once published - the role-name posture.
  `strip` leaves these lines alone: they are working shell, not annotation.
- Framing. One record per line, kept short (a record must fit a single atomic
  write). Free-form lines are legal.
- Noise tolerance. Ingestion never errors on a malformed line and never silently
  drops one. Unrecognized or free-form content is retained - sanitized,
  size-capped, attributed to the emitting body - and prints in full under the
  most verbose pull; default surfaces stay ruthlessly selected. Verbosity is the
  admission gate, not existence. Nothing you emit is lost.

The classed-decline record, emitted on a verdict body's declining path before
its `return 2`:

    printf 'decline unsound %s is a write-only trigger key\n' "$key" \
       >>"${DREP_V1:-/dev/null}"

- The class vocabulary is engine-owned and append-only. Four classes ship:
  `unsound` (permanently unanswerable -
  write-only triggers, nondeterministic reads); `unmodeled` (a better oracle
  could answer; not yet built); `interactive` (prompts by construction;
  unprobeable headless); `hazard` (the author's editorial claim about the admin's
  usage - deprecated or discouraged - the one class aimed at the book rather than
  at Dorc's own reasoning, and the only one that can surface, capped and
  attributed, on a plan). An unknown verb or class degrades to a generic
  author-noted line, never an error.
- Static-first. With a literal format string the class is read from source
  without execution: a per-arm inventory always (no sites needed), a per-site
  class when a site's argv threads statically to the arm. A dynamically-built
  format defeats static reading and demotes the class to a runtime-only fact,
  recovered only when the arm actually executes during a probe. Prefer literal
  formats.
- The class routes attention only: whichever class explains a refusal, the
  refused shape walls the same. A wrong class misdirects attention (a wrong
  `unsound` silences deserved enhancement pressure), attributed to the arm's
  file:line; it can never under- or over-execute a line. Classing is
  enhancement; a silent `return 2` stays exactly as legal and as safe. (A
  predict body's refusals are a different record entirely - `predicts none`,
  section 5b - and those are load-bearing, not enhancement.)
- The comment on the arm is display material. When `dorc why` inlines a declined
  arm, the arm's adjacent comment is shown to the admin as authored text - never
  parsed, never load-bearing on a decision. Write it for the operator reading
  their plan.

When to reach for a class, and the modeling-crutch caution. Class a decline
whenever you can state which of the four it is; genuinely-ambiguous or
unremarkable verdict declines may stay silent (`return 2` alone), and a
predict's `predicts none` tails stay free reason text, unclassed. Before reaching for a warning about a shape you can
actually answer, ask the sharper question: should the model be richer instead?
The recurring example is a tool whose live value and persisted value can differ
(a `sysctl` key set for the running kernel but not written to the boot config).
The temptation is an advisory note; the honest answer is two cells - `@value` and
`@persisted` - measured separately, not one cell plus prose. An advisory that
wants to explain a gap in your model is usually a missing distinction in the
model. Warning on a covered (answered) arm has no verb in v1 by design; it is a
deliberately-held seam pending exactly this question.

## 7. The failure catalogue, ranked

Worst first, with the repair path that attribution buys:

1. Probe-time mutation (any body; false `safe-across`; mutating entry form).
   Breaks the promise the whole product rests on; no gradient, no net. Repair:
   none after the fact - prevention is the entire section-3-of-page-three
   discipline. The engine refuses provable mutation at plan time; everything
   unprovable rides your vouch.
2. Wrong-world measurement (mis-sited entry; wrong lend value). Confident
   verdicts about a context the site does not denote; can under- or
   over-execute. Repair: decline-on-unverifiable siting; the static
   peel-coherence check; attribution to the wrapper family.
3. Wrong at-most speech (a disturbs omission; a finished-definition record
   written before its survey was truly total; a false invariance; a resolver
   split). Silent under-execution of someone else's line, behind the admin's
   risk flag (or the carry machinery, for invariance). Repair: fully
   attributed to the claiming line or record; next plan re-measures and
   self-heals; fix the one file, every consumer heals. (A reach body's omitted
   edge without the record is not in this class: it merely walls.)
4. Wrong yes (verdict 0 that should not have been). Under-execution at your
   own tool's site; attributed to your function. Repair: fix the arm; consider
   whether the verb belonged in the decline column all along.
5. Wrong channel claim (predict). Corrupt composed-probe results, bounded by
   coverage rules. Repair: decline the channel; delegate instead of asserting.
6. Over-caution everywhere else - declines, walls, unclaimed channels, absent
   members. Not a failure. This is the system working; the only cost is value
   not yet earned.

## 8. The battle-grade checklist

Before publishing, walk the file once against each line:

- Marker present; strip output verified to parse and run identically under
  both pinned floor shells; `shellcheck` and `checkbashisms` clean.
- Every verdict member: unknown-shape fallthrough to 2; existence gates on
  every delegate; no `!`, no `|| true`, no status arithmetic; foreign exit
  vocabularies remapped; pipeline tails audited.
- Every verdict arm: the yes re-examined as "is re-running truly acceptable
  noise here?"; multi-operand shapes fully checked or declined; state-
  addressing flags modeled or declined; the deliberate-decline verbs present,
  commented with their reasons, and classed (section 6a) where the class is
  known - literal format strings, so the class reads statically.
- Every body: read-only by design (not by privilege), no scaffolding
  side-effects, no dry-run flags taken on faith, answers from durable state,
  reentrant, cheap enough to pay the check-tax forever.
- Marks: one assertion per line; observes disclosed for every extra cell a
  verdict reads; complement senses on `:!`, never hand-inverted.
- Wrapper families: every dimension enumerated in `lend_map`; peel positions
  coherent between members; entry form non-interactive, siting-verified or
  declining; `safe-across` only on bodies re-audited for shifted execution.
- Every predict: unexplored shapes fenced with `predicts none` records, first
  - never through the status; channels claimed positively (`predicts`
  records), every claim record trailing the bytes it vouches for; statuses
  predicted, never answered.
- Footprints and kind members: matched shapes surveyed to completion;
  unsure-cells included; unsurveyed shapes unmatched; dynamic emission bodies
  ending every completing matched path with their completion record, exactly
  once; the finished-definition record written only where the survey is truly
  total, arm by arm; `only` members authored from a total survey or not at
  all.
- The file as a shipped artifact: header comment stating coverage, declines,
  and judgment rationale; kinds documented; names treated as permanent.

<!-- quoted: spike/CLAUDE.md invariants (license-and-trust, separation,
     observables, authored-surface, language-law blocks), decline-class-emission,
     report-lane-versioned-entry, report-surface-massaging-carve; 271 rulings
     ledger; plans/281 mark grammar v0.2 (supersedes 277 section 4 / 278 section 6);
     277 sections 1-6; 278 whole; 276 dialect rulings; plans/27C sections
     1-7; 23O settled law; USER_STORY.md bought-unsoundness; 27Q quality bars;
     27W:rul-emission-grammar-v1, rul-versioned-entry, rul-report-noise-tolerant,
     rul-advise-verb-deferred (modeling-crutch); plans/30U (finished-definition
     gate; disturbs nothing-else; witness-iff-licensure); notes/30D
     (predict channel claims; predicts records; verdict-partition-stays-separate) -->
