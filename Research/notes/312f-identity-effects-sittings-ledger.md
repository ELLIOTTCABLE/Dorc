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
nested or virtual mSorts and mSchemes already express more. They requested an example under
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
`tokenInvalidatedBy` compares each writeset entry against every mKey ancestor of the read key.
A key against its own container is UNKNOWN. Because UNKNOWN counts as a touch, even a precise
write to one member can invalidate a sibling's token. Closing the traversal and the lookup's
read set does not remove this separate token rule. Existing evidence is the list-file book in
`311:2.6.3-a-book-stage-five-the-list-file-named`; the new example must isolate this rule rather
than repeat that book's additional open-traversal cause.

The concrete exercise must distinguish a location alias from a dependency. A logical INI
setting is not automatically identical to a carrier byte. A location mScheme may yield into
another location mScheme only when they name the same mReferent. A derived setting instead
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

A precision correction to the chat: the yielding mSchemes name the same storage byte at each
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

Each book has two lines. Its fixed world has four mKeys: carrier, enabled byte, trace byte,
and timestamp aggregate. The carrier is rooted; the three parts have it as their mParent.
The primary part mScheme carries unique-name. Both sorts have closed may-read sets and finished
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
DISJOINT from the trace key and UNKNOWN against their carrier. The carrier is the only mKey
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
