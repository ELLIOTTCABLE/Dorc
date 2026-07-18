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
  safety. (The one deliberate exception: a wrong at-most claim, section 5c.)
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
mean something different. Current per-command roles: `predict`, `is_converged`,
`disturbs`, `lend_map`, `enter` (the last name still provisional). Current
per-kind roles: `resolve`, `disturbance_reaches_only`, `state_stored_only_in`.

The `only` naming convention binds authorship posture: a role with `only` in its
name is complete-by-contract - consumers act on its negative space, so authoring
it requires a totalistic survey first. A role without `only` grows arm by arm,
each matched arm complete for its own shape only.

## 3. The answer channel: exit-status law

Verdict-bearing members answer through their final exit status, read against one
fixed, permanent table:

| status | meaning | consequence |
|---|---|---|
| 0 | the named sense holds | may license guard and elision |
| 1 | the complement holds | the line is needed; runs |
| 2 or higher | cannot say | the line runs, always |

Only 0 and 1 ever carry a verdict; everything at 2 and above is one flat
"confused" sink, semantically flat forever - no future version will assign
meanings to individual high statuses, and nothing at 2+ can ever license
anything. Author obligations on this channel:

- Never collapse or invert statuses: no `!`, no `|| true`, no arithmetic on
  `$?` to "normalize" it. Sense-inversion is expressed by member naming and by
  the `:!` mark sigil, never by hand.
- Mind pipeline tails: a body ending in a pipeline answers with the tail's
  status (or the pipeline's, under pipefail); make sure that status is the
  answer you mean, and prefer shapes where the tool under description produces
  the status directly.
- Translate foreign exit vocabularies explicitly (`case $? in`) whenever the
  delegate's dialect differs from the table - especially tools whose 2 means
  something definite.
- Route every surprise to 2+: missing binaries, unrecognized output, permission
  oddities. `command -v tool >/dev/null 2>&1 || return 2` is the standard gate.

## 4. The grammar: marker, coordinates, marks, binds

The marker. `# dorc-lang/v0.1`, exact, alone on its line, within roughly the
first ten lines. Gates the syntax below for this file (and only the syntax; role
names need no marker). An unmarked file is plain shell.

Coordinates. `KIND:ENTITY#SELECTOR`, with the tail parts optional:

    sm.dorc.Service:"$svc"#enabled      one cell of one entity
    sm.dorc.Package:"$pkg"              a whole entity
    sm.dorc.PkgIndex                    a whole (singleton) kind

Kinds are reverse-DNS, two dots minimum; mint only under a domain you plausibly
answer for, reuse others' kinds only as their owners document. Entities are
written bare when they contain only letters, digits, `.`, `_`, `-`, `/`, and
double-quoted (with normal `"$var"` interpolation) otherwise. Selector tokens
are identifiers (letter or underscore first, then letters, digits,
underscores); the `#` must directly touch the entity (or the `:` in the
entity-less transitional form). A selector-less coordinate means the whole
entity and, on the consuming side, interacts with every cell of it. Polarity
never appears in a coordinate; it rides the mark sigil.

Marks. A mark trails a statement, whitespace-separated, one assertion per line:

    cmd args   : COORD      verdict: exit 0 asserts the cell holds
    cmd args   :! COORD     verdict, complement sense: exit 0 asserts it does not
    cmd args   :? COORD     observe: this statement reads that cell

Verdict and observe marks mint selector tokens into the kind's vocabulary and
attach facts to the one line that measured them; they are single-cell only - a
statement establishing two cells is two lines. An observe inside a verdict body
widens that fact's staleness surface (its backing) to include the observed
cell: always safe, often obligatory for honesty. Emission lines in the disturbs
and reach members carry a third mark position - `: KIND` or `: KIND#SELECTOR`
typing the emitted entities - and only these emission marks may use brace
alternation (`#{enabled,active}`). Claim emissions never mint tokens. Two
further trailing-token vocabularies are role-scoped: dimension tokens in
`lend_map` bodies, and substrate plus `invariant:<dimension>` tokens in
`state_stored_only_in` bodies. All token vocabularies are engine-owned and
closed; authors never invent tokens.

Binds. `name : KIND = "$value"` assigns and declares the value an entity of the
kind. Binds name entities, never cells. Strip reduces a bind to the assignment.

The `dorc:` prefix. `dorc:sh -c '...'` is the one prefix-position spelling:
full-analysis invitation on an interpreter head. Bare `sh -c '...'` is the
permanent escape hatch (hints only, licenses nothing); `dorc-sh` typed directly
is the runtime object and is left alone by strip. No annotation syntax is ever
recognized inside opaque payload strings.

## 5. Per-member contracts

Each member below states: how it is invoked, what its output means, what it
licenses, what its author must hold true, and how it fails. The probe contract
(read-only, fast, reentrant, answer-from-durable-state, fail-toward-2) applies
to every body in this section without further mention.

### 5a. `cmd__is_converged` - the verdict member

Invoked with a site's arguments (everything after the command word, as the
book's values resolved). Answers per section 3, where the named sense is "the
state this invocation exists to establish already holds."

Licenses: at this tool's own sites only - insertion of this body as a runtime
guard (`( check args ) || original-bytes`), and, when a probe-proof applies,
full elision of the line. The guard always preserves the original bytes and
always falls through to them on any non-0 answer. The vouch is inadmissible
everywhere else: it never becomes a fact, never informs another site's
reasoning, never transfers to another tool.

Author holds true: that an answer of 0 means not-running this invocation is an
acceptable outcome - all of it, including effects beyond the checked state
(converged is not the same as no-op; the pending-upgrade case is yours to
judge). That the body declines every shape not deliberately modeled: unknown
verbs, unmodeled flags (especially state-addressing ones like `--root`),
operand counts beyond what is checked. That multi-operand shapes are either
fully checked (every operand) or declined - a partially-checked yes is a wrong
yes.

Failure modes: a wrong 0 causes this tool's line to be skipped or guarded-away
when it was needed - under-execution, at your own tool's site, attributed to
this function by name. A wrong 1 merely runs a converged line (safe, noisy). A
mutating body breaks the probe promise itself (see section 7, first entry).

### 5b. `cmd__predict` - the modeling member

Invoked with an invocation's arguments. Stands in for the command inside probe
constructs: its stdout, stderr, and exit status are consumed as the command's
predicted observables, per the claim vocabulary - delegation of the real
(read-only) tool claims all channels faithfully; `printf` claims stdout;
`return N` claims the status; redirecting a channel to `/dev/null` declines
that channel; `return 2` up front declines the shape.

Licenses: substitution of this body for the tool inside composed probes and
lifted hand-guards, but only where every channel the surrounding construct
consumes is covered by the body's claims. Recognition of the peel shape (a
body whose `"$@"` runs its own argument-slot) classifies the tool a wrapper.
A predict never licenses eliding anything by itself.

Author holds true: that every claimed channel is faithful for every matched
shape, on hosts unlike theirs; that no matched shape mutates; that
convenience channels not thought through are declined, not guessed.

Failure modes: a wrong channel claim corrupts a composed probe's result - which
can surface as a wrong verdict for some enclosing construct. Bounded by the
coverage rule (unclaimed channels block substitution) and attributed to the
predict that claimed.

### 5c. `cmd__disturbs` - the footprint member

Invoked with a site's arguments. For a matched shape, emits the disturbed
entities one per line on stdout, each typed by a trailing kind (or
kind#selector) mark; `:` serves as the emission line for whole-kind claims. A
matched shape's emission is a complete at-most claim: this invocation disturbs
at most these cells, and anything omitted is declared untouched. An unmatched
shape emits nothing and claims nothing.

Licenses: under the admin's explicit risk flag only - survival of downstream
proven facts past this command actually running, wherever fact-backing and
claimed footprint are provably disjoint. Never consumed outside the flag;
never able to manufacture separation the comparison machinery cannot prove.

Author holds true: the survey. Match a shape only after genuinely enumerating
that shape's effects; when unsure whether some cell is disturbed, include it
(over-claiming only walls; under-claiming under-executes); when unsure the
enumeration is complete, do not match the shape. Selector-precision is welcome
refinement of a complete survey, never a substitute.

Failure modes: the sharpest in the system. An omitted cell in a matched shape
can silently un-run someone else's line - a different tool, author, and file -
with no runtime net. The bite is flag-gated (the admin typed for it), fully
attributed (the why-machinery names this member), short (the next plan
re-probes reality and the line returns), and narrow (each other line held its
own license) - but it is real, and it is the one place your mistake spends
other people's safety. Author accordingly.

### 5d. `cmd__lend_map` - the wrapper dimension member

Invoked with the wrapper's own arguments; body parses the wrapper's prefix
exactly as the tool does and ends with `"$@"` at the guest position. Emits one
entry per dimension: a valued line (`printf '%s\n' "$target"   : user`) maps
that dimension; an empty entry (`:   : fs-view`) passes it through unchanged;
a dimension with no entry at all is unknown and walls.

Licenses: interpretation of wrapped sites - which context the guest denotes,
per dimension - feeding context keying, entry composition, and chain folding.

Author holds true: enumerate-every-dimension (say "unchanged" explicitly); that
the peel position agrees with the predict's (checked statically; disagreement
is refused at plan time); that emitted values are single-link truths (the
engine owns composition across chains - never pre-compose nesting yourself).

Failure modes: a wrong lend value mis-keys facts to the wrong context - the
measured-wrong-world class, capped by the entry-form's own siting duties. A
missing dimension merely walls (safe, value-losing).

### 5e. `cmd__enter` - the context entry form (name provisional)

Body is ordinary pre-entry shell ending in `"$@"` verbatim in command position
(`sudo -n "$@"`). Invoked at probe time with a composed check in guest
position, under the admin's escalation dial.

Licenses: real context entry in the probe lane - the only licensed seat for
it. (Predict closure bodies never escalate; a wrapper with no entry form has
contexts that are simply never entered.)

Author holds true: non-interactive by construction (fails rather than
prompts, ever); self-effects acceptable as probe residue and answered for by
name (the auth-log class); siting - entry through this form lands the guest in
the same context the site's own bytes would reach, verified structurally, by
interrogating the tool, or by a tripwire, and *declined* (2+) where
unverifiable. Policy-routing wrappers make siting genuinely hard; a declined
entry is always correct and costs only value.

Failure modes: a wrongly-sited entry measures the wrong world and can produce
confident wrong verdicts - the worst object the probe lane can emit; hence the
decline duty. A prompting entry hangs or fails probes (contained by the
non-interactive construction). A mutating entry is a probe-contract break.

### 5f. The `tolerates:` vouch (a mark, not a member)

A bare colon-line mark inside a function body (`:   : tolerates:user`;
brace-alternation for several dimensions), scoped like any statement to the
paths that reach it.

Licenses: executing that function in contexts shifted along the named
dimensions, under the admin's dial.

Author holds true: the body's effects are read-only by design, not by
privilege-starvation - shifting it (notably to root) does not unlock writes it
was silently attempting all along. The vouch claims nothing about answers
(answers are supposed to vary per context) and nothing about other functions.

Failure modes: a false vouch is a probe-contract break executed in an entered
context - bounded by the dial, attributed to the three consents involved.

### 5g. `kind__resolve` - the canonicalizer

Invoked with an entity name of its kind; prints the canonical name, falling
through to the input for names it cannot answer. One resolver per kind in a
loaded world; duplicates are refused loudly.

Licenses: canonicalization of both sides of every same-kind comparison -
aliased names collide correctly, distinct names stay distinct.

Author holds true: conservative resolution - delegate to the substrate's own
authoritative query where one exists; map unknown names to themselves. A
wrong merge over-verifies (safe); a wrong split re-opens the silent-skip hole
the resolver exists to close (dangerous), so uncertainty always resolves
toward not-merging-but-also-not-inventing-splits: echo the input.

### 5h. `kind__disturbance_reaches_only` - reach

Invoked per footprint coordinate of its kind, whoever emitted it; emits the
implied coordinates in other kinds (footprint emission grammar; static lines
and read-only host-question lines both welcome).

Licenses: widening of every footprint of the kind to cover what touching such
an entity drags along. Footprints only; never widens what any fact claims for
its own backing.

Author holds true: the `only` contract - a totalistic survey of what
disturbance of this kind can reach. Over-breadth only walls; an omitted edge
re-opens exactly the cross-kind gap the member exists to close, so the survey
duty is real. Wrong-direction danger is asymmetric like the resolver's, and
authoring posture is the same: include when unsure.

### 5i. `kind__state_stored_only_in` - the store member

Emits the substrates where the kind's state lives (emission lines with
substrate tokens), plus zero or more whole-member invariance declarations
(`:   : invariant:<dimension>` colon-lines).

Licenses: substrate keying of the kind's facts; and, per invariance line,
cross-context carry of the kind's facts along that dimension - unflagged for
substrate dimensions (the engine independently verifies each carried verdict
body read nothing beyond its arguments and marked reads; bodies that fail
that structural check simply do not carry), flag-gated for identity
dimensions.

Author holds true: the `only` contract on the emission set; invariances that
are true of the substrate itself, not of your test environment. Known-refused
combinations are enforced (network-state kinds cannot claim netns-invariance);
contradictions between a declared invariance and the member's own emissions
are refused at plan time.

Failure modes: a false invariance line lets a fact measured in one world
answer for another where the answer differs - under-execution of the
consuming site, attributed to this line. The engine's structural check
narrows but does not replace your duty: it verifies the measuring body's
reads, not your claim about the substrate.

## 6. The probe execution environment

What a body may assume when it runs: it receives argv only (no environment
contract beyond what your own file establishes; assume `set -u`-grade
strictness in consumers); it may run many times, concurrently with other
bodies, batched with strangers' bodies on the same host; in the apply lane
the identical bytes run as guards, so nothing may depend on "this only runs
at plan time"; entered bodies run in shifted contexts exactly when their
vouches and the dial align. Its exit status is the entire in-band answer -
Dorc's own signalling travels out-of-band, and no exit code means "unknown"
except as section 3 defines. Its stdout/stderr are consumed only where a
contract says so (predict channels, emission grammars); verdict bodies should
run quiet, and refusal breadcrumbs go to the report stream, whose concrete
spelling is still settling.

## 7. The failure catalogue, ranked

Worst first, with the repair path that attribution buys:

1. Probe-time mutation (any body; false `tolerates:`; mutating entry form).
   Breaks the promise the whole product rests on; no gradient, no net. Repair:
   none after the fact - prevention is the entire section-3-of-page-three
   discipline. The engine refuses provable mutation at plan time; everything
   unprovable rides your vouch.
2. Wrong-world measurement (mis-sited entry; wrong lend value). Confident
   verdicts about a context the site does not denote; can under- or
   over-execute. Repair: decline-on-unverifiable siting; the static
   peel-coherence check; attribution to the wrapper family.
3. Wrong at-most claim (disturbs omission; reach omission; false invariance;
   resolver split). Silent under-execution of someone else's line, behind the
   admin's risk flag (or the carry machinery, for invariance). Repair: fully
   attributed to the claiming line; next plan re-measures and self-heals; fix
   the one file, every consumer heals.
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
- Every function: unknown-shape fallthrough to 2; existence gates on every
  delegate; no `!`, no `|| true`, no status arithmetic; foreign exit
  vocabularies remapped; pipeline tails audited.
- Every verdict arm: the yes re-examined as "is re-running truly acceptable
  noise here?"; multi-operand shapes fully checked or declined; state-
  addressing flags modeled or declined; the deliberate-decline verbs present
  and commented with their reasons.
- Every body: read-only by design (not by privilege), no scaffolding
  side-effects, no dry-run flags taken on faith, answers from durable state,
  reentrant, cheap enough to pay the check-tax forever.
- Marks: one assertion per line; observes disclosed for every extra cell a
  verdict reads; complement senses on `:!`, never hand-inverted.
- Wrapper families: every dimension enumerated in `lend_map`; peel positions
  coherent between members; entry form non-interactive, siting-verified or
  declining; `tolerates:` only on bodies re-audited for shifted execution.
- Footprints and kind members: matched shapes surveyed to completion;
  unsure-cells included; unsurveyed shapes unmatched; `only` members authored
  from a total survey or not at all.
- The file as a shipped artifact: header comment stating coverage, declines,
  and judgment rationale; kinds documented; names treated as permanent.

<!-- quoted: spike/CLAUDE.md invariants (license-and-trust, separation,
     observables, authored-surface, language-law blocks); 271 rulings ledger;
     277 sections 1-6; 278 whole; 276 dialect rulings; plans/27C sections 1-7;
     23O settled law; USER_STORY.md bought-unsoundness; 27Q quality bars -->
