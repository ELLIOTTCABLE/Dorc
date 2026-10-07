# 312f — Identity and effects: the design sittings ledger

> AI-authored (Astra). Living notes for the sittings after the mechanization recorded in
> `notes/312e`. The human is present. This ledger records their decisions, the conductor's
> reasoning, and measured evidence separately. Nothing is ruled unless marked **[TYPED]** or
> **[ACKED]** with the scope of that acknowledgment. Grades: +SURE, ~SUSPECT, -GUESS, --WONDER.
> Successors append sittings and corrections. They do not rewrite an earlier sitting into a
> decision the human did not make. The specification and its explicit departures remain
> `specs/311-identity.assay.md`; this ledger is not a replacement specification.

## § 1-remit-and-reading-boundary

**[TYPED]** The work is abstract design, not product implementation. The conductor loaded the
`conductor`, `architect`, `using-alloy`, and `asd-ste100` skills. The human initially withheld
311 itself while the conductor read the roots and recent ledgers. They then authorized a full
read of 311 and supplied two ephemeral reference files:
`_tmp-311-held-work-reference.md` and `_tmp-311-item-first-pass.md`.

**[TYPED]** Use the human to explore one foundational question at a time. Explanations should
be conversational in pace, technical and precise in content, and written in STE100-style
English. Present small pieces so the human can interrogate them. Do not turn the inventory of
held work into a schedule, or treat a predecessor's proposed repair as a decision.

The conductor read 311 in full, both shared halves, the core roots except the explicitly
postponed `AID-NEEDS.md` and `ANALYZER-NEEDS.md`, and the recent design and mechanization ledgers.
The principal current records are `notes/312cg`, `notes/312ch`, `notes/312d`, and `notes/312e`.
Earlier meanings in `notes/311q`, `notes/311t`, and `notes/312b` are read under their successors.
No product code was consulted to adjudicate the design.

## § 2-local-effects-and-the-conservative-bound

The first question concerned a write through two nested filesystems. The concrete example had
`/srv/lab/outer.img` mounted at `/mnt/outer`, an inner image at `/mnt/outer/inner.img` mounted
at `/mnt/inner`, and a book copying into `/mnt/inner/app.conf`. A sibling file,
`/mnt/inner/other.conf`, supplied the contrasting case. The setup was illustrative and was not
executed.

The conductor separated three descriptions: the command author names its destination; the
inner filesystem's describer names its backing image; the outer filesystem's describer names
its own backing image. In a real library one reusable description could serve both filesystem
instances. Distinct authors exposed composition without assuming coordination.

The epistemic question was whether a finished declaration names direct effects or all eventual
consequences. The conductor provisionally favored local descriptions composed by the engine.
This was a proposal, not a decision. The source of the question is the difference between the
one-step closure truths and the transitive soundness conclusion in
`311:2.6-may-write-the-writeset`, with its held third-thing hole.

**[TYPED]** The human reaffirmed the purpose of “and nothing else”: it intentionally admits
residue that is epistemically unknowable across other actors, machines, and future behavior.
At least one oracle engineer explicitly authors that risk. Every consuming admin explicitly
accepts it through the risk flag. Neither consent reduces the engineer's obligation to pursue
as much justified confidence as possible. Closing a list does not require proof of an
unknowable universal.

**[TYPED]** The standing conservative reading remains: a write described only as a write to an
image can affect the whole image. A locally known, specifically described write is different.
Dorc knows no image semantics. The human requested referentially agnostic reasoning and
permitted hypothetical tools where useful to expose who can supply knowledge.

Conductor correction: “follow the chain” is insufficient by itself. A particular update to a
file changes backing bytes. An arbitrary write to those backing bytes can affect other files.
Composing possibilities gives a conservative bound; it does not establish that the particular
update changed those other files. The human has not chosen a replacement effect algebra.

## § 3-virtual-descriptions-before-new-model-structure

**[TYPED]** The human challenged a recurring move: proposing new substructure where ordinary
nested or virtual MSorts and MSchemes already express more. They requested an example under
arbitrary stdlib effort, including an outer configuration file whose opaque string embeds an
inner configuration document known to another author.

The conductor's chat example was JSON containing an INI document. A hypothetical `jsonslot`
adapter exposes the decoded string to `inictl`. One line changes `worker.enabled` from `0` to
`1`; another reads `worker.trace`. The selected operation preserves layout and inode identity.
Those are authored tool properties, not engine assumptions.

The proposed description chain maps the inner value's location through a decoded JSON-string
position to a canonical file-byte key. Distinct names alone do not separate anything. The
comparison must reach a shared key space whose owner warrants the distinction. The INI author
knows INI interpretation, the JSON author knows encoding and carrier edits, and the file
stdlib knows substrate identity. Several parties could investigate the whole composition, but
that is not the cheapest assignment of reusable expertise.

**[ACKED, problem statement only]** The human accepted the statement that this representation
may use existing objects while the current invalidation rule can still prevent its useful
consumption. They did not approve a repair, a new warrant, a new species, or the claim that the
chat strawman is a complete model of real JSON/INI tooling.

Conductor's narrow finding (+SURE of the formula, not yet measured in a new book):
`tokenInvalidatedBy` compares each writeset entry against every MKey ancestor of the read key.
A key against its own container is UNKNOWN. Because UNKNOWN counts as a touch, even a precise
write to one member can invalidate a sibling's token. Closing the traversal and the lookup's
read set does not remove this separate token rule. Existing evidence is the list-file book in
`311w:2.6.3-a-book-stage-five-the-list-file-named`; the new example must isolate this rule rather
than repeat that book's additional open-traversal cause.

The concrete exercise must distinguish a location alias from a dependency. A logical INI
setting is not automatically identical to a carrier byte. A location MScheme may yield into
another location MScheme only when they name the same MReferent. A derived setting instead
needs an ordinary read dependency on the carrier locations. Escaping, variable-length values,
and parser-wide validation can defeat a naive one-byte story.

## § 4-durable-work-authorized

**[TYPED]** Create this running ledger as `312f`, record the discussion, and write a thorough
example in the existing `Research/notes/312b-exercises/` family. Include strawman oracles.
If tractable and valuable, add failing Alloy in Assay's book form, with granular claim sigs
that match the example. Work in a new conductor worktree and commit granularly with the
`commit` skill.

The worktree is `.tmp/trees/r31-identity-effects-conductor`, branch
`ai/r31-identity-effects-conductor`. No change to 311's inference rules is authorized by this
work order. An expected failing desired outcome is evidence for a sitting, not permission to
change the model or weaken the test.

The validation target is a fixed world with two distinct members in one described carrier:
truthful closed descriptions, a write to one member, and an independent read of the other.
The current-behavior witness must be inhabited. A desired-survival assertion should fail for
the token-invalidation reason even after the other conservative causes are removed. A coarse
carrier write must still defeat the read. These are proposed evidence obligations, not new
product rulings.

## § 5-example-authorship-and-the-experimental-check

**[TYPED]** The human clarified the Alloy remit during authorship. The assay is primarily a
precision and attention exercise, so that a later repair can be tested mechanically. It need
not be committed if imperfect or if integration would require unrelated specification changes.
The ledger and concrete exercise must be completed and committed. Eventual integration as a
regression example is intended, not a requirement to force integration now.

The concrete exercise is `312b-exercises/precise-writes-inside-an-opaque-carrier.md`. It uses a
JSON carrier containing an INI string, one layout-preserving patch, and one already-converged
sibling bit setter. The tools are explicitly hypothetical and narrowly contracted. The oracles
separate INI location knowledge, JSON encoding and writeback, and canonical file-part identity.
Jules explicitly sources Inez's plain-data helper and emits one complete invocation claim;
there is no new protocol for merging completion records.

A precision correction to the chat: the yielding MSchemes name the same storage byte at each
step, not a logical boolean falsely equated with its encoding. Other encodings need different
ordinary descriptions. Parser dependencies, inode replacement, timestamps, and consumed guest
stdout are explicit. The exercise does not claim the two toy commands are production-ready.

Static fixture check, using Node's JSON parser and byte arrays only: the carrier has 63 bytes
including LF. The enabled and trace digits occupy zero-based carrier positions 48 and 57,
and decoded positions 17 and 25. Replacing byte 48 with `1` changes exactly that byte, preserves
JSON and decoded length, and leaves `trace=0`. No shell fixture or hypothetical tool was run.
The Windows `python` command was unavailable; Node supplied this check without installation.

A second precision correction arose while constructing the evidence. A precise setter is not
sufficient if the verdict uses a general parser that validates every other value. That parser
can genuinely depend on the supposedly independent slot. The exercise now explicitly selects
fixed-profile reader and setter tools. They validate structural bytes and the selected slot,
not the other slot's value, and promise no whole-document validation. The valid JSON/INI
fixture is one supported instance. A generic parser remains a separate, potentially coarser
case. This narrows the example, not Dorc's admitted world or a specification law.

Assay has no cross-document import or book-append command. The experiment therefore assembles
an ignored temporary document from unchanged 311 plus four book fragments, with the unchanged
shared halves beside it. No specification file or committed result lock changes. The assembly
script is `.tmp/312f-study/build-study.cjs` in this worktree, beside its output.
The canonical-key slice compresses the upstream identity into a supplied rooted carrier and
uses explicitly closed empty primary-key traversals. It does not mechanize JSON, INI, helper
custody, or the actual layout probes. The metadata write is retained as a separate part.

The first parse refused a new map name for literal `1`, which 311 already names `pid_1`.
Reusing the existing word atom repaired that join-key conflict. The assembled document then
passed every parse lint. This was a spelling repair, not a model change. The untouched 311
source hash is recorded in `.tmp/312f-study/baseline.json`.

## § 6-measured-token-invalidation-without-a-routing-wall

The experimental books ran against unchanged 311. The conductor read the JSON reports and the
wanted book's counterexample, then strengthened the current-behavior book to check every other
sparing prerequisite. That strengthened book ran again on both platforms. No inference rule,
truth predicate, hole exclusion, scope, or solver option was changed to obtain these results.

Each book has two lines. Its fixed world has four MKeys: carrier, enabled byte, trace byte,
and timestamp aggregate. The carrier is rooted; the three parts have it as their MParent.
The primary part MScheme carries unique-name. Both sorts have closed may-read sets and finished
entailments. All four primary-key traversals are explicitly closed-empty. The flag is set.
The fact reads only the trace byte. The precise line writes the enabled byte and timestamps.

The reported scope is `6 but 4 int, 7 seq, exactly 71 Shword, exactly 0 Class, exactly 2 Line`.
There are exactly 20 claims, except in the whole-carrier case, which has 19. Each fresh command
had a 120-second CPU budget. All results were definite; there was no timeout or construction
disagreement.

| book | line outcomes / conjunction | book run | platforms |
| --- | --- | --- | --- |
| `precise_sibling_current` | no counterexample at the stated scope | SAT | Windows and WSL |
| `precise_sibling_wanted` | first line has no counterexample; second line and conjunction have counterexamples | UNSAT | Windows and WSL |
| `whole_carrier_current` | no counterexample at the stated scope | SAT | Windows |
| `same_member_current` | no counterexample at the stated scope | SAT | Windows |

The two sibling books have byte-identical declarations and world facts; this was checked on
the generated modules after excluding their module headers and commands. They also have the
same first outcome. The current book's SAT run therefore witnesses the wanted book's world
independently of its impossible wanted outcome. Assay reports the wanted book's own premise as
UNSAT because that run conjoins all requested outcomes, including survival. That UNSAT is not
an inconsistent world and is not a vacuous proof. The counterexample itself supplies another
witness of the first outcome with the second false.

The strengthened current book confirms these facts together (+SURE at the stated scope):

- The written payload key and the read key compare DISJOINT.
- The written payload key and the carrier compare UNKNOWN.
- Neither the readset nor the relevant writeset is top.
- Every writeset member compares DISJOINT with the read key.
- Routing invalidation is false, and lifecycle invalidation is false.
- The actual writes affect no dependency of the measured fact.
- Token invalidation is true, and sparing is false.

The counterexample's comparison table shows the same pattern for the timestamp key: it is
DISJOINT from the trace key and UNKNOWN against their carrier. The carrier is the only MKey
ancestor of the trace key. Thus the token rule is the remaining blocker in this fixed world.
The whole-carrier and same-member controls retain the required collision behavior.

This establishes a value gap in an admitted canonical-key world. It does not establish that the
concrete oracles discharge all their contracts, or that a proposed repair is sound. In
particular, the fixture does not decide whether aggregate parent state must include every child
state, how actual changes compose, or what additional speech should license token stability.
The world uses 311's existing `holds` abstraction and has no chronology of changed identities.
These limits are recorded in the experiment's README, not hidden by a new model axiom.

### § 6.1-replay-material-and-conduct

The uncommitted experiment remains in this conductor worktree under `.tmp/312f-study/`:
`build-study.cjs`, `books.assay.md`, `combined.assay.md`, the shared halves, `baseline.json`,
`README.md`, and six JSON reports. `precise-current-report.json` and
`precise-current-wsl-report.json` contain the strengthened diagnostic. The other reports retain
their original measurements. The source 311 prefix in the assembled input was checked
byte-for-byte against the untouched specification.

Counterexample instances are under
`spike/target/alloy/combined/instances/book_precise_sibling_wanted/`, notably `line_2.xml` and
`every_line.xml`. The source fragments and assembly script are the replay inputs; generated
modules and instances are disposable outputs. No result lock was written. Every check command
returned exit 1 for new rows against a missing experimental lock, with the desired red reported
separately as a counterexample. No result is described as an acceptance-gate pass.

The first commit hook needed mise trust for the new worktree. The human explicitly authorized
that trust on Windows and WSL, without installation. A later hook briefly stashed the untracked
assembly script under hk's default policy and restored it. The script now lives in the ignored
experiment directory. Subsequent hooks use `HK_STASH=none` and `HK_FIX=0`, the intended agent
mode. No user stash was altered or removed.

## § 7-state-for-the-next-sitting

The durable changes are this ledger, the concrete exercise, and their pointer in
`Research/README.md`. All are committed on `ai/r31-identity-effects-conductor`.
`mise run both gate:full-quiet` passed on Windows and WSL with the three applicable document
checks selected. The worktree remains available because it holds the running sitting and the
uncommitted experimental sources and reports. No branch was merged, no remote was changed,
and `specs/311-identity.assay.md` and its lock are untouched.

The design remains open. The experiment isolates the token rule's value cost even with
canonical keys and ideal closed routing. It does not authorize deleting that rule: writes to
a carrier and genuinely identity-changing member operations still require conservative
handling. The next discussion can therefore examine what the current statements actually
promise about a member token's lifetime, without first inventing a new region species.

## § 8-provisional-contract-analysis-after-the-example

This is a thematic synthesis, not a sequence of turns. The analysis remains soft. No proposed
contract, table placement, or repair below is ratified. No further Alloy experiment was run
for these interpretations; § 6 remains the measured evidence and has the narrower scope it
states.

### § 8.1-the-fixed-boundaries

**[TYPED, reinforcing ack]** The conservative behavior is the floor. This sitting introduces
neither incorrectness nor ambiguity. The option space is explicit speech that permits correct
additional elision; the question is who speaks and where. The flag does not supply absent
knowledge, although explicit closures still admit the epistemically unknowable residue under
the double consent recorded in § 2.

**[HUMAN, probable nack]** An implicit extension of the command writer's obligation damages
gradual enhancement. Making it optional requires a distinguishable statement or entrypoint.
That has an authorship cost comparable to giving the speech to someone better placed to know.
This is not a ruling against every optional statement a command author could make.

**[TYPED reminder]** Every warrant and claim is subject to full abstract interpretation. An
author may condition speech, decline, or retain the coarse floor through arbitrary legal sh.
For example, a probe can establish that a database is in a stable mode before a path emits a
claim. A single warrant is therefore not necessarily uniform across a scheme. The conductor
withdraws the expressiveness objection that presumed such uniformity. The meaning of a claim
and the validity of its measured conditions remain separate questions.

**[HUMAN]** The two-ended table is non-normative. Gaps in it have often corresponded to design
gaps, so it is a useful search pattern, not authority. Whether a two-ended relation requires
both ends, either end, or only coherence when both speak is explicitly set aside here. Nothing
in this synthesis chooses that licensing policy.

### § 8.2-the-validity-condition-that-stood-out

The conductor's formulation, singled out by the human as important while they still described
the analysis as soft:

> The troublesome sentence is that a warrant holds while its token stands. It gives the answer
> under a condition, but does not itself establish that condition after a write.

The textual anchor is `311:1.5.1-the-token-over-time-and-the-per-level-closure`: “A warrant
holds while that MKey's MResolution or MToken stands.” The invalidation rules decide when that
condition ceases to hold. The quoted observation is not a ruling that a new warrant is needed.
Another existing statement may already justify preservation, with a missing or overly coarse
consumer. Alternatively, the present contract may lack that statement. Those readings have
not been distinguished conclusively.

A useful provisional separation (~SUSPECT): identity asks what equality or inequality licenses
while interpretations are valid; validity asks which changes can end that authority; effects
ask what a running operation changes. The experiment shows a current consumer's behavior. It
does not establish which of these existing promises should justify the desired alternative.

### § 8.3-the-alternatives-reduced-to-their-knowledge

Three candidate forms remain in view, without choosing a spelling or requiring a new species:

- A direct, conditional stability promise by a naming-system owner. It might state that a
  supported class of member writes preserves interpretations in that owner's key space.
- A closed description of an interpretation's dependencies, supplied by its lookup or
  naming-system owner. Existing routing and lookup-read machinery may carry much of it.
- A description from the mutation side, supplied by the owner of the store or naming
  machinery. It might identify which assignments can change when the described state changes.
  This is distinct from imposing that knowledge on every command writer.

**[HUMAN correction, accepted by the conductor]** “Use a stronger identity” is not an
independent escape from authored speech. Detecting a strong identifier or a stable store gives
someone grounds for the corresponding promise. Someone must still state what can vary.

A runtime re-check remains a different product outcome: it can protect execution but normally
returns the later operation to the guarded plan. Neither that observation nor the possibility
of a stronger identifier settles the missing contract.

### § 8.4-the-two-competent-ends-and-the-table

The candidate pair is the naming-scheme or lookup owner, who knows the dependencies of their
interpretation, and the owner of the affected store or naming machinery, who knows what its
mutations can disturb. These can be different people. They can also be one author serving two
roles. No argument established a uniquely possible speaker; the concern is placing reusable
expertise and responsibility with the least duplicated effort.

The conductor first pointed at the empty container end of ROUTE/CAT. That was too quick.
Static membership in a catalog does not itself specify stability across mutations. Nor is this
the missing ADDR/STORE thing-side “no other home” closure, which concerns alternative identity
placements rather than the lifetime of one interpretation.

The later working reading (~SUSPECT) spans existing rows: the ROUTE thing-side supplies
resolution dependencies, WRITE supplies effects that can reach those dependencies, and ADDR
supplies the identity warrants consumed while validity holds. Ordinary virtual MSorts could
name assignment state, so a store owner's existing effect descriptions could identify changes
to it. This might need a more complete interpretation-side contract and a consumer connection,
rather than two new warrants or a new row. It is a candidate construction, not a demonstrated
reduction of the full problem. In particular, the blank ROUTE/CAT cell has not thereby been
proved necessary or unnecessary.

### § 8.5-executed-reads-versus-semantic-dependencies

The remaining suspected distinction (~SUSPECT) is between all reads a lookup body performs and
all changes that can invalidate the meaning of its result. A primary lookup can return its
argument without consulting a mutable assignment table on which that argument's interpretation
still depends. This motivates examining whether current resolution-dependency speech covers
semantic validity, including primary-token validity, rather than assuming that a closed empty
executed-read set certifies it.

That example is not yet a counterexample to the complete existing contract. Its force depends
on the exact obligations already carried by the uniqueness warrant, supplied parent, lookup,
and closure. The conductor has not established whether the missing piece is new optional
speech, clarification of existing speech, or its consumption by invalidation. No existing
warrant gains a stronger obligation merely because that would make the desired book pass.

## § 9-investigating-existing-speech-before-adding-any

**[HUMAN objection]** A database-derived answer requires knowledge of the database. Whether an
author obtains that knowledge through a command or through their own investigation, omitting
its required read attribution is a contract failure. The conductor's return-the-argument
example did not establish a new category of unreportable semantic dependence.

The conductor withdrew that use of the example. Returning unchanged bytes is distinct from
establishing unchanged denotation; a supplied name may retain its bytes while its referent
changes. But its declared MParent and other foundations already report more than the function
body's direct reads. No honest counterexample to the full existing contract was demonstrated.

**[ACKED as investigation questions, not answers]** Can existing resolution-dependency speech
cover the semantic basis of a primary token's interpretation? Can fully reported existing
foundations distinguish a sibling-value update from a change to the naming structure? If so,
the problem could be consumption or composition rather than absent speech. The human asked the
conductor to investigate both.

### § 9.1-source-findings-and-their-limits

+SURE of the text: the original prose 311 defines a MResolution as the fact that N, of S,
resolved in P at program point p, reaches MReferent R. It is not defined merely as producing
the next token's bytes. Its backing includes the closed emitted traversal and the lookup
body's reads. An unclosed traversal retains its catalog given whole. An open lookup read set
invalidates under any write. The current specification retains these statements across
`311:1.7-resolution-and-its-traversal` and its prose residue.

+SURE of the text: the independent parent-store token-invalidation clause already existed in
the prose baseline, before mechanization. The clause follows the observation that a first
write can change an MKey-Primary. It is not a mistranscription introduced by the Alloy builder.
Changing its scope would be a design clarification or revision, not an automatic fidelity fix.

~SUSPECT: the semantic scope of MResolution supplies a stronger existing basis for the desired
contract than the conductor's earlier account of executed reads alone. It does not establish
that every existing closure already guarantees all the required validity conditions. Inputs,
parent identity, warrant conditions, traversed state, and read provenance still need a coherent
composition rule. The primary lookup's identity result is not a license to manufacture a
closed empty basis for dynamic knowledge.

The focused older reads were `plans/27C` § 4 and `plans/30T` § 3.1, plus the relevant
`ANALYZER-NEEDS` rows. They distinguish tracked inputs and marked external reads, and require
marks for a verdict's visible reads. They do not override 311's newer meanings or establish
that its literal-return case has no other foundation. `311q` § 10 records the conditional
warrant lifetime as an earlier ack; its scope must not be silently widened.

### § 9.2-both-existing-warrants-still-hit-the-same-rule

A fifth temporary book, `precise_sibling_with_both_warrants`, adds the existing
`:guarantees-unique-referent` statement to § 6's precise sibling world. Unique-name, closed
reads, closed traversals, complete writes, and the other facts remain. No definition changes.
The strengthened diagnostic again requires that token invalidation alone prevents sparing.

Windows and WSL both returned an inhabited world and no counterexample to that diagnostic.
The scope is six, with four-bit integers, sequence bound seven, 71 words, zero classes, two
lines, and 21 claims. The conjunction was solved fresh on each platform; its line checks were
entailed. All parse lints were empty. The reports are `both-warrants-report.json` and
`both-warrants-wsl-report.json` in `.tmp/312f-study/`. Exit 1 still denotes new rows against a
missing experimental lock. No lock or specification was edited.

This confirms only that adding this warrant does not overcome the current consumer in the
fixed world. It does not show that the warrant is redundant, or decide its intended temporal
meaning. The broader question remains a contract question, not something this static world
can settle by itself.

## § 10-semantic-links-and-an-existing-rooted-alternative

**[TYPED]** Assay experiments run on Windows only from this point. The human sees no useful
platform difference for this model-finding work. This does not change the separate project
completion-gate convention.

**[ACKED]** Argument-to-output dataflow grants no semantic identity. Even equal bytes can mean
different things unless authored speech establishes otherwise. The conductor withdraws
“forwarding input retains its foundations” as a general semantic rule over values. Mechanical
dependency tracking and the support of an identity derivation are different things. A lookup
makes its own local yield claim; the engine composes that claim with other supplied claims.
The lookup author need not inspect an unknown caller's warrant. No warrant transfers to a new
MScheme merely because a binary preserved or transformed some bytes.

The ergonomic attack found no demonstrated need for one lookup author to certify the whole
chain. The important restrictions were local yields, inherited input knowledge rather than
repeat investigation, negative lookup dependencies, and no requirement that every result name
one existing physical object. The human's provisional acceptance of sufficient speech remains
qualified by that local reading; it is not a proof that every future naming system fits.

**[TYPED]** Reassess the alleged hole before adding design: can existing semantically
appropriate claims make the assay pass? If the gap survives, describe only the constrained
residue for the human's attention.

### § 10.1-the-scoped-derivation-experiment-is-unmeasured

An additional temporary book added an explicit `:yields` from a decorated input value to a
different canonical value, with layout as an ordinary MReferent. Both the producing lookup
and the primary interpretation declared and closed their traversal through the layout. The
verdict included layout in its marked reads. Both identity warrants remained present.

This is `.tmp/312f-study/explicit-yield-book.assay.md`, assembled against unchanged 311 as
`composed.assay.md` by `extend-derivation.cjs`. At scope eight, 28 claims, four-bit integers,
and the 120-second hot budget, both the diagnostic check and its inhabitation run timed out
while translating. A second encoding of the scope stated the exact signature counts already
required by the fixture's equalities: five referents, two sorts, three schemes, three shapes,
one vantage. No world fact or outcome changed. Both commands still timed out in translation.
The two reports are `explicit-yield-report.json` and `explicit-yield-exact-report.json`.
Neither run provides a verdict or an inhabitation witness.

The structural argument remains separate from those unmeasurements: on the ordinary chain,
a written member compares UNKNOWN with its own ancestor. The token rule consults that pair
without a closure-sensitive exception. Adding a producer's yield does not by itself remove
the ancestor from the final primary key's chain. This is not a claim that every alternative
identity representation must fail.

### § 10.2-root-speech-greens-a-different-description

The original fixture already supplies a rooted carrier. The next experiment describes its
parts with globally qualified identifiers under a `:root` part shape, rather than relative
identifiers `:identified-in` the carrier. The world still says the carrier holds the same
parts. The root carrier and root parts have different root shapes. No inference rule changed.

This is a stronger proposed description, not automatic propagation of a root warrant through
string formatting. Its intended concrete reading is a nominal position identified by the
carrier's non-reassigned identity and the position's own identity. The owner must answer for
that global naming claim. An unqualified offset, a pathname, or an identifier that can be
reassigned does not acquire that guarantee merely by receiving a prefix. Whether the concrete
library provides such identities is not measured by this fixture.

Windows results, from the hot tier, all with satisfiable book runs:

| book | asserted outcome | result at the stated bounds |
| --- | --- | --- |
| `rooted_precise_sibling` | sibling sparing, no token/routing invalidation, outside all five held sparing holes | no counterexample |
| `rooted_whole_carrier` | no sparing, despite no token/routing invalidation | no counterexample |
| `rooted_same_member` | no sparing, despite no token/routing invalidation | no counterexample |

The scope is seven, with four-bit integers, sequence bound seven, 71 words, zero classes,
and two lines. The claim counts are respectively 18, 16, and 17. The strengthened precise
check was solved fresh; its witness replayed successfully. The two control checks and their
witnesses were solved fresh. The precise book demands that it is outside the held holes as
part of its outcome, not as a premise exclusion.

Why it works in the fixed world: a rooted part's chain has no MKey ancestor, so the token
rule has no parent-store entry to test. Part-against-part comparisons use the common part
MScheme's warrants. Whole-carrier against part stays UNKNOWN across their two root shapes,
so the coarse write still collides. This also means the construction is not evidence that
unrelated whole carriers would spare each other's parts; that precision was not recovered.

Replay inputs: `rooted-alternative.cjs`, `rooted-books.assay.md`, and `rooted.assay.md` under
`.tmp/312f-study/`. Reports: `rooted-precise-outside-holes-report.json`,
`rooted-whole-report.json`, and `rooted-same-report.json`. The earlier, weaker precise check is
`rooted-precise-report.json`. All use unchanged 311 and no result lock.

### § 10.3-what-this-does-and-does-not-close

+SURE of the bounded result: existing `:root` speech can produce the desired outcome in this
stronger fixed description while retaining the two tested collision controls. The earlier
specimen therefore does not establish a general lack of expressive power.

~SUSPECT: globally named nominal positions are a legitimate library construction for the
fixed-profile carrier when its own global identity is available. This is not established for
arbitrary physical byte incarnations or arbitrary stores. It is not a proof that root-level
global comparability alone gives the required temporal guarantee.

The narrower unresolved question concerns primary keys that remain parent-scoped. Complete
resolution information still meets the unconditional parent-store veto there. The experiments
do not decide whether this is an intended price of weaker speech, a missing connection to
existing validity information, or an insufficiently stated lifetime contract. The difference
between global comparability and temporal stability must not be erased by the successful root
fixture. No new warrant or invalidation rule is adopted.

One limit of root promotion is already visible without a new experiment: uniqueness within a
parent does not imply unique naming across parents. The existing nested-pid-namespace example
has one process with different keys in two namespaces. Qualifying each pid with its namespace
can make the qualified names globally unambiguous, but does not make them globally unique names
for referents. The proposed root-part scheme's unique-name warrant would then be false. Finding
one common canonical identifier may need additional knowledge or access. Thus the root fixture
is not a general reduction of locally scoped comparisons to rooted ones. This argument concerns
identity expressiveness; it does not itself prove a temporal stability contract for pids.

## § 11-store-side-responsibility-for-collateral-naming-effects

### § 11.1-the-human-sub-ruling

**[TYPED, sub-ruling]** `__resolve` authors must not describe the effects of writes to the
parent store. The item-side warrants are not responsible for warranting properties of that
store, except potentially properties that apply only and locally to the resolved item.
Knowledge and responsibility for effects on other items' naming belong at the database/store
level. An implicit collateral-write obligation on the item resolver is excluded.

The human distinguished this from the read side, about which they remain concerned. This
sub-ruling does not settle the read contract, the exact store-side statement, or the policy
for combining two ends. It does not authorize a new API or a change to invalidation.

**[HUMAN concern]** An answer over the entire store may be expensive or impossible to obtain.
That may require coarse statements and limit precision. The human did not yet find the exact
relationship between that concern and the responsibility ruling clear.

### § 11.2-tentative-consequences-not-additional-rulings

~SUSPECT: the ruling narrows the investigation but does not remove the measured mechanical
obstruction. Item-local resolution information can identify dependencies without certifying
what writes elsewhere do to them. A store-side description can identify collateral changes
that reach those dependencies. The engine would compose the information rather than infer
store behavior from a resolver's closed read list. The exact sufficiency of either description
remains unsettled.

~SUSPECT: responsibility at the store level need not require a fine-grained survey of every
member on each invocation. The store describer could supply a conditional rule applied to one
described write and the relevant instance configuration. Examples discussed were stable
addressing under member-content changes, positional assignments affected by an insertion or
deletion, and a whole-store bound where finer knowledge is unavailable. These are candidate
uses of speech, not an adopted statement shape or invocation contract. Every claim remains
path-granular through the admitted shell analysis.

A constraint on that proposal: an input describing only “member A may be touched” does not
necessarily distinguish a content update from deletion. A store rule must cover the operations
its input description admits. Finer existing write descriptions might supply the distinction
without making command authors responsible for collateral naming behavior. Whether the current
interfaces carry sufficient input is not established.

~SUSPECT: the relevant store may be virtual. A third party's score-sorted view can have unstable
positional names over a physical database with stable native IDs. The view's author would
occupy the store-side seat for that addressing system. The physical database's statement must
not silently certify an unknown third-party lookup. This is a proposed application of ordinary
virtual MSorts, not a ruling that the two descriptions or their effects already compose
correctly under every relevant 311 rule.

The resulting candidate question is narrower: how can the store describer express the naming
effects of a described write, and how should token invalidation consume that information?
Existing store-owned effect entailments over ordinary naming-state objects may supply it.
That possibility is unverified. No conclusion is drawn yet about a new statement, entrypoint,
model row, or species. No new Alloy measurement accompanies this section; the earlier fixed
worlds do not establish the proposed contract or its ergonomics.

## § 12-the-root-acked-and-the-path

2026-10-01. A new conductor (Opus) over `55571543`, after a full read of 311, both shared
halves, this ledger, and its exercise. **[TYPED]** this ledger is reused, sittings are not
ledgered every turn, and commits are granular.

**[ACKED, the problem statement only, up to and excluding a proposed fix]** The conductor's
statement of what is wrong:

- The common root: 311 answers effect questions ("can this write change what that fact depended
  on, or what that key now names?") with its identity instrument ("is this the same thing, or
  inside it?"). Two stand-ins carry the load. "Inside" stands in for "changed by", which is too
  coarse: the token rule, a contained write touching its store, the exclusion keyed on the
  container, cells of one parent, and region against region. "Distinct" stands in for
  "independent", which is too loose: one thing beneath two separated things, the third thing,
  a route off the catalog, and the region closure.
- Each hole is a world-shape assumption that the composition relies on with no assigned
  speaker. Each needs one disposition: an existing statement read so that its author owns the
  assumption, the conservative floor, or a stated horizon.
- The test that refutes this framing: if three or four rulings on what the world relations
  mean do not collapse most effect-side holes, the framing is wrong.
- The item of § 3 to § 11 at product altitude: within any parent-scoped store, a write to one
  member invalidates every other member's token, and no statement anyone can make recovers it,
  since `compare()` of a key against its own container is UNKNOWN by step 2 of the walk. So
  USER_STORY stage 5 (a list-file write against a status-file fact in one filesystem) cannot
  survive, however precise the describers are. Only keys minted directly in a root's world
  spare. The clause guards a real concern, a store handing an old number to a new thing, with
  the wrong test: "is the write inside the store?" in place of "does the write change how the
  store names things?".

**[ACKED, generally]** The path: park everything off the critical path; firm the identity
half; rule the effect world, time-boxed to about two sittings, with the token clause as its
first worked case; and, if that does not converge, take the floor on every value-only hole,
close the soundness holes, and freeze.

**[TYPED]** Recut the root `_tmp-311-held-work-reference.md` around the path without dropping
items; group by the ruling likely to close them; move subordinate items under a parent as
probable correlates. Done the same turn; the file is gitignored and not committed.

**[HUMAN]** On the proposed fix (two rules: a write naming the store itself; the store
describer's unfinished entailment): a suspected break of referential agnosticism, a
fail-helpful default that assumes ops-shaped constructs. The human asked for a careful walk:
whether the two rules cover every name, address, id, or index invalidation in the core with
safe defaults, and whether, without new speech, a store owner can intentionally leave the
matter incomplete and default safe.

**[HUMAN]** 311 is known to owe Alloy 6's small-world time (about ten steps). The human asked
whether the analysis says that time must come first for mechanically tracked progress on this
item.

## § 13-the-two-rules-walked-time-and-a-real-store

2026-10-01, the same conductor. **[TYPED]** ledger the findings; add a GOTCHA if the case is
not covered; write a detailed exercise record that implies no clear conclusion. Everything
below is the conductor's, unacked unless marked.

### § 13.1-the-two-rules-withdrawn

- `fnd-rule-a-needs-non-disjoint` (+SURE of the walk) — "a write that names the store itself"
  is unsafe when read as SAME: a write naming the store in another vocabulary, or naming
  something that contains it, must count (`dd of=/dev/sda1` against a filesystem; a snapshot
  revert of a VM disk above it). The repaired form: a write counts when it compares other than
  DISJOINT with the store, except when its own described chain places it strictly below the
  store. A ⊤ writeset keeps invalidating.
- `fnd-rule-b-reads-an-effects-closure-as-a-naming-closure` (+SURE of the text) — 311 keeps
  naming apart from state (`311:1.7.1`: an MKey can cease to reach an object without the object
  changing). The finished record is a closure over state, and its truth exempts the holder
  ("affects only it, what it holds, and the entailment"). The naming map lives in the holder.
  So a finished record can never be false about a renaming, and cannot license one. Rule (b)
  gave it that second meaning, and a store owner could not finish effects while leaving naming
  open. Withdrawn.
- `fnd-the-ops-shaped-assumption` — the smuggled default was "a store renames only what a write
  names". It holds where names change only by creation or destruction (inode numbers, pids,
  uids), which are writes someone already names.
- `fnd-positional-is-caught-by-the-honest-prefix` — positional renumbering (`ufw insert 1`) is
  not a counterexample: an honest positional lookup emits the prefix it walks, which is its own
  lookup and not a store effect, and routing catches the insertion. What rule (b) lets through
  is narrower: renaming as a store behaviour that no key's own lookup walks.
- `fnd-token-leg-duplicates-routing-default` (+SURE of the fences) — for a primary key, an
  unclosed traversal is the parent given whole, so routing already invalidates on any write
  inside the parent. The token rule's member-write leg differs only after a closing act on the
  primary key. It is the backstop against unspoken store behaviour.
- `claim-no-store-sited-statement-speaks-naming` (posed in chat) — the conductor walked the
  store-sited species (the finished record, `:aliases-nothing-else`,
  `:guarantees-unique-referent`, the store sort's closed may-read set, `:places` and
  `looked-up-in`) and found none that speaks naming stability, and filed the primary key's
  closing act as item-side. **[HUMAN]** challenge: the walk did not account for the primary
  MScheme's `__resolve`, which is the store owner's naming half and is probably written by them.

### § 13.2-time

- `fnd-reuse-is-inexpressible-without-time` (+SURE of the fences) — `reaches` is one MReferent
  or none for the whole book, and the uniqueness warrants are timeless, so no world with every
  statement true reuses or reassigns a key. Every narrowing of the token rule would check green
  vacuously. ~SUSPECT, unmeasured: deleting the member-write leg outright leaves every law
  green. The experiments of § 6 to § 10 measured blocking only.
- `fnd-time-is-necessary-not-sufficient` — once reach varies, someone's truth sentence must say
  when it may vary. Order proposed: settle the speaker by argument; add the slice; write that
  speaker's temporal truth; narrow the rule and expect a red exactly where that statement is
  false.
- `fnd-the-smallest-slice` (~SUSPECT) — only `reaches` needs to vary, and only by line;
  `holds`, `affects`, and `passes` stay timeless for now. `30Y` § 4 already keeps the hook
  (sited world facts; assay hoists them today). Steps are lines: two to four per book. Start on
  the walk laws and the token and routing books, not the sparing law, which is at its ceiling
  (-GUESS on cost). Only `_tmp` § 3.3's group needs the slice; it edits `specs/`, so it needs
  the human's word.

### § 13.3-a-real-store-that-is-not-counting

- The case: Linux block-device naming, one partition under six names with six authorities
  (kernel name, device number, by-path, by-id, filesystem UUID and label, LVM names). A byte
  copy (`dd`), a label write on another device, an LVM snapshot, or duplicate physical volumes
  move content-derived names that another device held; a delete and rescan move the kernel's
  names and leave the content-derived ones. Record:
  `312b-exercises/a-copy-takes-the-originals-name.md`. GOTCHA:
  `a-copy-takes-the-originals-name`.
- What it strains, as observations: a content write moves a name with no counting; the renamer
  is a third store (udev, `blkid`, LVM, btrfs) reacting asynchronously; one store holds schemes
  with opposite sensitivities; a scanned name's honest dependency is an absence over the whole
  population; ties are broken by policy; with duplicate physical volumes a later write lands on
  the copy; the "stable" name is the cloneable one.
- What it does not settle: whether a narrowed token rule is safe for device numbers. It shows
  that a rule decided per store cannot treat device numbers and UUIDs differently.

### § 13.4-questions-the-human-put-for-the-next-turn

- **[HUMAN]** Speech about naming has been modelled as an MScheme, and the naming half of a store
  MSort's owner as the primary MScheme they probably also write. Does the primary MScheme's
  `__resolve` resolve `claim-no-store-sited-statement-speaks-naming`?
- **[HUMAN]** The single-item `__resolve` row of the two-ended table may have no effective far
  end; no store-level API endpoint for an MScheme was designed, unless forgotten. If so, and
  acked, consider placing this speech on the MScheme rather than on the MSort.
- **[HUMAN]** Is that speech meaningful only for primary MSchemes? Can a secondary MScheme
  yielding into the same MSort have different write and invalidation semantics?
- **[HUMAN]**, tabled but kept in scope: this may be the first time MSchemes get significant,
  dangerous descriptive power, which is hard to square with fail-safe defaults. Each must fail
  safe with no collaboration, with no default that lets someone else's speech make one's own
  non-dangerous speech dangerous, except the usual "and nothing else": a sort declared with no
  scheme; a stranger's scheme yielding into one's sort; a stranger's sort naming one's scheme
  as its primary.

## § 14-the-conductors-answers-the-humans-responses-and-the-open-asks

2026-10-01, the same conductor, near its context limit. **[TYPED]** the human rewinds before
seeking real answers; this section carries the threads for a successor. Nothing here is ruled.
The conductor's answers to § 13.4 were put in chat and are banked first; the human's responses
follow; the open asks are last.

### § 14.1-the-conductors-answers-to-section-13-4-unacked

- On the primary MScheme's `__resolve` (~SUSPECT) — the conductor's error was to file a primary
  key's closing act as item-side speech. The primary MScheme's owner is the MSort's owner (the
  fence fact under `312d:enc-primary-owner-is-sort-owner`), and for keys a store mints the
  primary MScheme is that store's naming, so the seat is the store-level one that § 11.1 asks
  for. The act exists: a lookup emits what it crossed and closes the list, the closing act's
  prose meaning (`311:1.7.1`) is "what can make this key stop reaching its object", and the
  fences already admit `EmitsCrossed` and `ClosesTraversal` on a primary key. Three things block
  it: 311 says a primary `resolve()` is the identity on the key, so it has no body to emit from;
  the token rule ignores any emission; the closing act's mechanized truth reads the static
  `passes`. Proposed reading: no new speech act; one existing act gains one new seat (primary
  MSchemes), and the token rule's member-write leg folds into routing. Caveat: the primary
  MScheme's owner is the member sort's owner, who can differ from the store's describer (a
  third party keying rows by rowid inside another's database); that owner chose the key space
  and can stay on the floor by emitting nothing.
- On the far end and the seat (+SURE of the text; ~SUSPECT of the placement) — `311` § 5.1's
  ROUTE row at catalog level has an empty store end, footnoted "a catalog makes no statement
  about its own entries"; no store-level naming endpoint was designed. Per-MScheme placement:
  the block-device case puts several MSchemes with opposite sensitivities on one MSort, which a
  per-sort statement cannot separate. It also matches 311's existing split (§ 5.2): identity
  statements sit on schemes (both lookup warrants, `:root`, `:identified-in`, `:yields`, the
  closing act, `alias nothing-else`); effect statements sit on sorts (may-read, the entailment,
  the finished record, observer-independence). The withdrawn rule (b) put a naming statement on
  the sort. For store-minted keys the two ends merge. The conductor also framed naming stability
  as "how long `:guarantees-unique-referent` stays true" (`311:1.5.1`: "a warrant holds while
  that MKey's MResolution or MToken stands"); see § 14.2 for the human's response.
- On secondary MSchemes — yes, secondary MSchemes into one MSort with different invalidation
  are everyday: a user by uid or by login name; a file by inode, path, or `/proc/self/fd/N`; a
  partition by device number, `UUID=`, by-path; a package by name, `name:arch`, `provides`.
  Secondary MSchemes already carry the speech (emission, closing act, the lookup body's read
  set); primary MSchemes are the ones without it. The example the conductor gave was
  `userdel alice && useradd alice`; see § 14.2.
- On failing safe with strangers (a first walk; ~SUSPECT each) — a sort declared with no scheme
  has only cells, whose naming is their parent's, so nothing can turn dangerous. A stranger's
  scheme yielding into one's sort: only a key's own scheme owner may emit for it (a fence fact),
  so the stranger can add a dependency for facts read through their keys and never remove one;
  one's own claims become load-bearing for their facts, as `:guarantees-unique-referent`
  already is. A stranger's sort naming one's scheme as its primary: refused today by the fence
  that the primary scheme's owner is the sort's owner, which is the unacked reading
  `312d:enc-primary-owner-is-sort-owner`, and by "at most one MSort per MScheme" when one's own
  `:primary-of` is in force; if both fall, the stranger's sort-level closures govern facts about
  one's keys, which are their "nothing else" claims, but one's silence then rests on that
  unacked reading. Defaults stay safe: no emission is the parent given whole; emission without
  a closing act keeps the parent; only the closing act narrows. Hand walk on the block-device
  case: the stdlib's device-number scheme emits the device itself and closes, so a copy onto
  `8:33` leaves `8:17`'s key standing, a rescan (another vocabulary) collides with every device
  number, and a UUID lookup that emits every device (or nothing) invalidates every fact read
  through a UUID.
- Asked in chat, unanswered: hold the per-scheme placement as the direction for `_tmp` § 3.3?
  Start path 2 now, and may the line-varying-reach slice run beside it (it edits `specs/`)?

### § 14.2-the-humans-responses

- **[HUMAN]** A gentle, probable nack on framing naming stability as "how long
  `:guarantees-unique-referent` stays true". Dorc's promises have so far been unbounded: Dorc
  models the disturbing act, never a duration; TOCTOU is the horizon; and a warrant is given
  when the act that would invalidate it, at the other end, is also modelled. Letting an author
  say "this holds for X time" would be new and does not follow from what has been built. Open to
  pushback. The human's understanding of Alloy time, to be confirmed: it models "A, then B, then
  C", showing an invalidation that follows an overwrite, not how long something stays valid.
- **[HUMAN]** On `userdel alice && useradd alice`: two commands, two models. Contract work
  describes items that are atomic from Dorc's perspective. A better example is an opaque, atomic
  `userrecreate alice`, whose own description covers it well. The two-command sequence is the
  interesting and dangerous case, which Dorc cannot handle without more speech from someone. The
  conductor's point stands; the example was imprecise.
- **[HUMAN]** The two ends of a row are more than possibly separate authors, and they can often
  be one human. A significant reason for them, slightly outside 311's usual altitude, is
  performance: where a warrant varies per key, calling a per-key `__resolve` across every key of
  a store is vastly expensive against one store-level call that dumps the whole map. The
  canonical example: dpkg lists the files of a package, and the package of a file, and neither
  substitutes performantly for the other.
- **[HUMAN]** The human still has no name for the hole and no clear fundamental understanding of
  the single hole under discussion, and reads that as a bad sign.

### § 14.3-the-open-asks-for-a-successor

- **[HUMAN]** In the block-device naming case, find a concrete store that offers both directions
  as separate commands for some subset of (store, names), each of which would be prohibitive for
  Dorc to compute piecemeal through a per-item entrypoint. Conductor's unposed, unverified lead:
  `findfs UUID=…` and `blkid -U` (name to device), `blkid /dev/sdb1` (device to names), and
  whole-map dumps (`blkid` with no argument, `lsblk -f`, `udevadm info --export-db`; for LVM,
  `pvs -o pv_name,pv_uuid`). Whether any pair is truly two directions that do not substitute is
  unchecked.
- **[HUMAN]** Set aside "we may not need a new member". Define the new member or members that
  would close this issue precisely and completely, so that the problem has a name. Only then
  test whether the member is unnecessary, another member in disguise, or derivable by engine
  logic. Do not fold too easily: "we have enough already, and the named thing is unnecessary"
  stays a legitimate outcome of the exercise.
- Carried from § 14.1 and still open: the per-scheme placement for `_tmp` § 3.3; path 2 and the
  time slice; confirming or correcting the human's reading of Alloy time.
