Implementation
==============

These are less-user-facing/high-level-goals *details*. They are, for the most part, subservient to [./DESIGN.md].


Moving parts
------------

Mostly a rehash of DESIGN, but as a refresher, we've got, effectively, two high-level components:

1. an orchestrator; and
2. a compiler.

The latter breaks down further into a few inter-dependant parts:

1. parser,
2. analysis engine, and
3. a probe-compiler.

... as well as a few more-boring components like the CLI, a few shared types in `core`, and a host-simulator for determinstic testing.


Agentic editing
---------------

I'm making a concerted effort to use LLMs on this project (it's partly a proving-ground to myself, to experiment with whether these supposedly-amazing tools can even be *used* any real engineering work, beyond CRUD web-UIs); and pursuant to that, there's some directional details required:

- LLMs are dumb and lose context. Deterministic tooling (agentic-hooks, linters, tests, and most importantly, a strong static-typing discipline) are critical to my approach.
- A significant amount of design-work is going *through* LLMs; the one source-of-truth remains these root-level markdown documents, but I'm attempting to stay hands-off on everything else, and provide all direction through agents. (vomit-emoji)
- Finally, although it cannot increase objective-truth directionality (you can't figure out if you're right with it), adversarial prompting is used heavily to *explore the cardinalities* of problems; pushing models into different corners of their state-space often leads them to find novel approaches or surface different bugs.

The first point might lead to this code feeling *especially* noisy with unnecessary, over-the-top tight typings; that's very much intentional, strong types, and a strong prompting discipline, helps ground subagents in the local invariants. (They *cannot* remotely be relied on to synthesize and apply as many simultaneous invariants - and worst, softer pareto-frontier cost/benefit tradeoffs and global goals - as a human could, and the only way I have found to handle the herding-cats nature of this is to *extremely *localize* those invariants. To *enforce* them wherever possible, or *spotlight* them where nondeterministic or undecidable.)


Correctness
-----------

Besides *over*-typing as described above, we're of course pursuing general statically-typed best practices (just with the 'carefulness' knob turned up to 11):

- most importantly: make bad states *unrepresentable*;
- and try to tweak the ergonomics to *guide* authors towards good practices, when we *need* a full state-space occasionally representable, or when a design-constraint is against an undecidable value.

Sadly (:P) typing cannot catch *everything*; and for this project (with its simultaneous constraints of "correct" and "built mostly by agents"), I'm trying to keep all my other correctness-tooling similarly overbuilt:

- a new toy for me, here, but I'm leaning heavily on [Deterministic-Simulation Testing][video] or "DST" (see also the [FoundationDB paper][fdb-paper]); attempting to build a distsys-adjacent tool with an Idiots Cloud is potentially the strangest thing I have ever done, and I'm hoping tight-loop, deterministic, reproducible regression-tests *across distributed states* will help me keep it under control;
- with, of course, the standard components of thorough (... emdash-splattered, sigh) documentation and judicious integration-tests. (In the 'agentic age', I'm not sure where to land on unit-tests; if we're trying to treat code itself as a little more disposable and less precious, then I guess I'm going to lean slightly away from the extremely-granular tests, to try and be judicious about agent-context-window attention-rot; but that will push more correctness-obligation up into *rich* integration-tests, which I trust agents less to write ... we'll see.)

[video]: <https://www.youtube.com/watch?v=4fFDFbi3toc> "'Testing Distributed Systems w/ Deterministic Simulation' by Will Wilson, Strange Loop 2014, R.I.P."
[fdb-paper]: <https://www.foundationdb.org/files/fdb-paper.pdf> "FoundationDB: A Distributed Unbundled Transactional Key Value Store; Zhou, Miller, et al. SIGMOD 2021"

### Correctness vs. best-effort: a band

Something worth diving into directly, because it's one of the most critical parts of the project, is what correctness means here. It's elaborated on in DESIGN, but I feel I can't stress it enough: *we're* correct, only so we can do our very best in a very, very incorrect environment.

And *implementing* that, is hard.

Throughout the implementation, it's important to keep track of two angles on "provenance":

1. "where something came from" (through transformations, across machines), and
2. "how much we trust it" (across the two axes of competence/security-privilege.)

"Facts" established by-contract-with-the-user are subject to user-error, and must only be relied upon as a last-resort, in ways strongly bounded by our explicit design edges. We never *implicitly* allow-in tainted "decide-based-on-imperfect-user-assumptions", except at the platform-edges where we've explicitly chosen that as a design constraint. Comparatively, "facts" established by static analysis are provable and trustworthy, therefore 'clean.'

Our "correctness", therefor, exists in a very narrow band *between the admin & engineer*, between ops and devops. There's only a *relatively small range of things* we can actually prove about what's going on, from an opaque reading of shell-script syntax. A much larger majority of what we do, is buried behind the phrase 'best effort': it's a subtle and biased set of framings to ensure that, as either admin-user-behaviour or oracle-author-behaviour degrades, *our* behaviour degrades *only as much as necessary*, in the precise ways forced by the user error/omission, and no further.

### To execute, or not to execute?

For every mutative command in a playbook from a user, there's three possible outcomes:

1. "under-execute": to mistakenly elide a command that *was* necessary to converge system-state;
2. "correctly-execute": to run, exactly once, a command that *was* necessary to converge;
3. "unnecessarily-execute": to run, exactly once, a command that *was not* necessary to converge;
4. "over-execute": to run, *more* than once, any mutative command.

These four are necessarily in tension and cannot be perfectly reconciled, given imperfect user-behaviour; thus, we've an established priority amongst them:

1. (highest) *never* under-execute: do not risk skipping the execution of a command that is desired/required (except by explicit user-dictum, i.e. `dorc bump`)
2. avoid over-execution: don't repeatedly-execute commands to achieve overestimated convergence (i.e. protect users from non-idempotent commands as much as possible)
3. (lowest) avoid unnecessary-execution: save the user time by eliding commands that are *genuinely* safe to elide (basically, the value-prop.)

Note the inherent directionality (and imbalance) of user-trust imposed on us by that ordering (or, depending on how you look at it, the imbalance-of-user-trust that *caused* that ordering):

- we wish to guard the user from being too anal about "idempotence of mutative/apply-stage commands"; but if we genuinely never assume the user can competently achieve idempotence, *we can never safely exist*. There's a natural floor to our user-disturst here, avoiding-depending-on-idempotence is very much best-effort.
- in contrast, we try *very hard* to ensure there's no mutation before a `plan` is presented (not enumerated in the above list, because it's an explicit failure-mode, period - it's about *probe*-stage commands, which we construct from oracles - not, from the admin-user's perspective, "their problem.") And, because under-estimation of probe safety *leads to relying on idempotence anyway*, there's an asymmetric safety-story here.

Note that basically all of the above can be summarized as "no worse than just running the script, blind, which is what you would have done without Dorc":

1. the `dorc plan`, probing-stage *would not exist* without Dorc. By offering it at all, we're promising a user that they're not doing anything to their machine - we must do our best not to violate that.
2. then, during `dorc apply`, the user would normally run the entire script exactly once:
    - this will *probably* run unnecessary commands, and *also* may-but-ideally-wouldn't involve idempotency errors that unhelpfully mutate the machine - both failure-modes we're therefore allowed to replicate, if hopefully minimize
    - but it *would not* result in blind, unknowing *multiple-execution* within a single script-execution (thus, a failure-mode we're *less* allowed to make, because it is surprising.)
