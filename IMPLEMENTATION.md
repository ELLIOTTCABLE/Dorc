Implementation
==============

These are less-user-facing/high-level-goals *details*. They are, for the most
part, subservient to [./DESIGN.md].


Moving parts
------------

Mostly a rehash of DESIGN, but as a refresher, we've got, effectively, two
high-level components:

1. an orchestrator; and
2. a compiler.

The latter breaks down further into a few inter-dependant parts:

1. parser,
2. analysis engine, and
3. a probe/guard-compiler.

... as well as a few more-boring components like the CLI, a few shared types in
`core`, and a host-simulator for determinstic testing.


Agentic editing
---------------

I'm making a concerted effort to use LLMs on this project (it's partly a
proving-ground to myself, to experiment with whether these supposedly-amazing
tools can even be *used* any real engineering work, beyond CRUD web-UIs); and
pursuant to that, there's some directional details required:

 - LLMs are dumb and lose context. Deterministic tooling (agentic-hooks,
   linters, tests, and most importantly, a strong static-typing discipline) are
   critical to my approach.

 - A significant amount of design-work is going *through* LLMs; the one
   source-of-truth remains these root-level markdown documents, but I'm
   attempting to stay hands-off on everything else, and provide all direction
   through agents. (vomit-emoji)

 - Finally, although it cannot increase objective-truth directionality (you
   can't figure out if you're right with it), adversarial prompting is used
   heavily to *explore the cardinalities* of problems; pushing models into
   different corners of their state-space often leads them to find novel
   approaches or surface different bugs.

The first point might lead to this code feeling *especially* noisy with
unnecessary, over-the-top tight typings; that's very much intentional, strong
types, and a strong prompting discipline, helps ground subagents in the local
invariants. (They *cannot* remotely be relied on to synthesize and apply as many
simultaneous invariants - and worst, softer pareto-frontier cost/benefit
tradeoffs and global goals - as a human could, and the only way I have found to
handle the herding-cats nature of this is to *extremely *localize* those
invariants. To *enforce* them wherever possible, or *spotlight* them where
nondeterministic or undecidable.)


Correctness
-----------

Besides *over*-typing as described above, we're of course pursuing general
statically-typed best practices (just with the 'carefulness' knob turned up to
11):

 - most importantly: make bad states *unrepresentable*;
 - and try to tweak the ergonomics to *guide* authors towards good practices,
   when we *need* a full state-space occasionally representable, or when a
   design-constraint is against an undecidable value.

Sadly (:P) typing cannot catch *everything*; and for this project (with its
simultaneous constraints of "correct" and "built mostly by agents"), I'm trying
to keep all my other correctness-tooling similarly overbuilt:

 - a new toy for me, here, but I'm leaning heavily on [Deterministic-Simulation
   Testing][video] or "DST" (see also the [FoundationDB paper][fdb-paper]);
   attempting to build a distsys-adjacent tool with an Idiots Cloud is
   potentially the strangest thing I have ever done, and I'm hoping tight-loop,
   deterministic, reproducible regression-tests *across distributed states* will
   help me keep it under control;

 - with, of course, the standard components of thorough (... emdash-splattered,
   sigh) documentation and judicious integration-tests. (In the 'agentic age',
   I'm not sure where to land on unit-tests; if we're trying to treat code
   itself as a little more disposable and less precious, then I guess I'm going
   to lean slightly away from the extremely-granular tests, to try and be
   judicious about agent-context-window attention-rot; but that will push more
   correctness-obligation up into *rich* integration-tests, which I trust agents
   less to write ... we'll see.)

   [video]: <https://www.youtube.com/watch?v=4fFDFbi3toc> "'Testing Distributed Systems w/ Deterministic Simulation' by Will Wilson, Strange Loop 2014, R.I.P."
   [fdb-paper]: <https://www.foundationdb.org/files/fdb-paper.pdf> "FoundationDB: A Distributed Unbundled Transactional Key Value Store; Zhou, Miller, et al. SIGMOD 2021"


### Correctness vs. best-effort: a band

Something worth diving into directly, because it's one of the most critical
parts of the project, is what correctness means here. It's elaborated on in
DESIGN, but I feel I can't stress it enough: *we're* correct, only so we can do
our very best in a very, very incorrect environment.

And *implementing* that, is hard.

Throughout the implementation, it's important to keep track of two angles on
"provenance":

1. "where something came from" (through transformations, across machines), and
2. "how much we trust it" (across the two axes of
   competence/security-privilege.)

"Facts" established by-contract-with-the-user are subject to user-error, and
must only be relied upon as a last-resort, in ways strongly bounded by our
explicit design edges. We never *implicitly* allow-in tainted
"decide-based-on-imperfect-user-assumptions", except at the platform-edges where
we've explicitly chosen that as a design constraint. Comparatively, "facts"
established by static analysis are provable and trustworthy, therefore 'clean.'

Our "correctness", therefor, exists in a very narrow band *between the admin &
engineer*, between ops and devops. There's only a *relatively small range of
things* we can actually prove about what's going on, from an opaque reading of
shell-script syntax. A much larger majority of what we do, is buried behind the
phrase 'best effort': it's a subtle and biased set of framings to ensure that,
as either admin-user-behaviour or oracle-author-behaviour degrades, *our*
behaviour degrades *only as much as necessary*, in the precise ways forced by
the user error/omission, and no further.

This dimension also intereacts in a complicated fashion with the probe/apply
inequality. Two of the many things we ask an oracle-engineer to be accurate about
may *sound* similar ...

1. "does your oracle-implementation ever cause mutation" and,
2. "what aspects of the runtime behaviour of the command have you modeled, and
   how completely?"

Unfortunately, they have very different constraints in practice. There's *no*
fallback for mutation. We can't meaningfully describe ourselves as 'best-effort'
there - what 'best-effort' means, in functional implementation terms, is
establishing a *failure gradient*. "We only fail you as much as you've already
failed yourself." But there's no *gradient* to accidental mutation: we've told
you to keep your oracle mutation-free; and we've told you we won't cause
mutation-on-probe. If that contract is broken, it *fully* collapses; there's no
"partially mutated" state for us to aspire to, nor a "partially mutating" state
to cause us to reach for it.

The opposite is true for the apply-time semantic, though: *partial benefit
exists*. Dorc could, potentially, elide *many* runbook commands; but it could
also, potentially, elide *less*, while still providing value in the few it does
manage to elide. Similarly, 'elision' can collapse to 'guarding' and still
provide *some* benefit, just not *most* of the benefit. Therefore an
*under-modeled* command - a poorly-written, low-resolution oracle -
can/should/hopefully reach toward that half-beneficial
outcome.


### To execute, or not to execute?

For every mutative command in a playbook from a user, there's three possible
outcomes:

1. "under-execute": to mistakenly elide a command that *was* necessary to
   converge system-state;
2. "correctly-execute": to run, exactly once, a command that *was* necessary to
   converge;
3. "unnecessarily-execute": to run, exactly once, a command that *was not*
   necessary to converge;
4. "over-execute": to run, *more* than once, any mutative command.

These four are necessarily in tension and cannot be perfectly reconciled, given
imperfect user-behaviour; thus, we've an established priority amongst them:

1. (highest) *never* under-execute: do not risk skipping the execution of a
   command that is desired/required (except by explicit user-dictum, i.e. `dorc
   bump`)
2. avoid over-execution: don't repeatedly-execute commands to achieve
   overestimated convergence (i.e. protect users from non-idempotent commands as
   much as possible)
3. (lowest) avoid unnecessary-execution: save the user time by eliding commands
   that are *genuinely* safe to elide (basically, the value-prop.)

Note the inherent directionality (and imbalance) of user-trust imposed on us by
that ordering (or, depending on how you look at it, the imbalance-of-user-trust
that *caused* that ordering):

 - we wish to guard the user from being too anal about "idempotence of
   mutative/apply-stage commands"; but if we genuinely never assume the user can
   competently achieve idempotence, *we can never safely exist*. There's a
   natural floor to our user-disturst here, avoiding-depending-on-idempotence is
   very much best-effort.

 - in contrast, we try *very hard* to ensure there's no mutation before a `plan`
   is presented (not enumerated in the above list, because it's an explicit
   failure-mode, period - it's about *probe*-stage commands, which we construct
   from oracles - not, from the admin-user's perspective, "their problem.") And,
   because under-estimation of probe safety *leads to relying on idempotence
   anyway*, there's an asymmetric safety-story here.

Note that basically all of the above can be summarized as "no worse than just
running the script, blind, which is what you would have done without Dorc":

1. the `dorc plan`, probing-stage *would not exist* without Dorc. By offering it
   at all, we're promising a user that they're not doing anything to their
   machine - we must do our best not to violate that.

2. then, during `dorc apply`, the user would normally run the entire script
   exactly once:
    - this will *probably* run unnecessary commands, and *also*
      may-but-ideally-wouldn't involve idempotency errors that unhelpfully
      mutate the machine - both failure-modes we're therefore allowed to
      replicate, if hopefully minimize

    - but it *would not* result in blind, unknowing *multiple-execution* within
      a single script-execution (thus, a failure-mode we're *less* allowed to
      make, because it is surprising.)


### Guarding, full elision, and gradual-enhancement

For our rather-draconian correctness requirements, 'full elision' (the original
goal of the project) is substantially difficult in the requirements it places on
the user (or rather, the 'collective user' - the user and the community of
oracles their runbook depends upon.)

In particular, as mentioned in DESIGN, some commands function as a 'poison
wall': if the admin uses some little-known command, and writes no oracle for it,
then Dorc can know nothing about it (the frame problem.) In particular, if
*other* commands' oracles declare that they depend on particular shared state
(and everything in ops depends on shared state), then *we have no way of knowing
if those commands can be safely elided anymore*, after the unmodeled, opaque
command runs.

As a motivating example (see-also the USER_STORY.md):

```sh
apt-get install -y nginx      # well-known tool w/ a battle-tested oracle
hork tune-packages            # opaque, Dorc knows nothing about this
systemctl enable --now nginx  # well-known tool w/ a battle-tested oracle
```

Dorc's general purpose is to 'lift' questions about that last `systemctl` to a
"probing phase", along with many other questions, so that it can be removed if
it's unnecessary. However, the `systemctl` *depends* on state established
earlier in the control-flow - the installation of `nginx`. In ideal conditions,
all these facts can be probed together, and elided together; but in cases like
above, *we can no longer trust the results of our own probing.* (That is,
perhaps nginx was indeed installed at probe-time, but `hork` is a little-known
package-management tool that *specifically uninstalls `nginx`* in some cases.)

So, when Dorc's 'knowability-model' of the world 'degrades' past a certain point
in the CFG (the "poison wall"), we're left in a state where *probing* is
relatively useless; and Dorc's *value proposition* changes: we can no longer
'fully elide' commands (i.e. that `systemctl` line cannot be removed safely from
the planning-result "apply-script".) In this state, we still have plenty of
information about the script, though, and we attempt to degrade into a
secondary, still-useful mode, by *runtime-guarding* that command: wrapping it in
a test that will skip it if, indeed, the convergence-state holds at runtime
*after `hork` has run.* (Effectively "automatically coding defensively" against
the unmodeled, unknowable behaviour of `hork`.)

It's critical to understand that this is a *different product*, though: the
*primary* value-proposition of Dorc is human-attention; performance is
secondary. *Even if* the `systemctl` line never actually runs, we have to *show
it to the user* in the apply-plan; it takes up mindshare and attention, and
those are much more precious resources than 30 seconds of wallclock.

Our only recourse is to push hard on gradual enhancement: ensure the user has
high-quality reporting about *why* the last 50% of their script is 'still there'
(fails full-elision), *what they can do* to improve Dorc's value to them
(attribute and suggest repairs.) In an ideal world, the first step should
devolve to "write a ~three-line convergence-focused oracle so `hork` itself can
elide" (since elision casts no poisoned shadow.) Further enhancement providing
reporting about `hork`'s actual first-order footprint will further improve
behaviour to the point where it can avoid poisoning *even when unconverged*.


### Inter-oracle collaboration, global state, and the golden hill

The above is somewhat mollified if one writes a basic oracle for `hork`. A
simple truth: if `hork` never runs, `hork` *cannot* poison something unexpected
between `apt-get` and `systemctl`. Thus, the trivially-true easiest route around
the danger is to *make `hork` not run*.

The simplest route to that is to write the most-minimal oracle that helps Dorc
fully-elide `hork` itself, in isolation: a convergence-test thereof, plus the
author's blessing to act on it (FIXME: spelling/details unsettled):

```sh
hork.is_converged() { hork --check "$@" ;}
```

This *doesn't* buy you all of Dorc's functionality, but it buys the most of it,
with the least effort; now (again, speaking in a vaccuum, because all of this is
modulo *other* state-actors and CFG participants), as long as `hork --dry-run`
passes, Dorc can safely make assumptions about `apt-get` speaking to
`systemctl`. Abstract-interpretation is unpoisoned, and the richer machinery can
run for those other commands; the poison-wall is lifted.

However, for *better* behaviour, to *fully* lift the poison-wall in all cases
(i.e. enable Dorc to elide *later* commands, even when probing surfaces that
hork is diverged), you must ....

FILLME


By-contract and by-dictate
--------------------------

Besides ~computer science and hard engineering~, there's really only a small
spectrum of ways we can *make* something true. We can "contract" it
(we'll-do-if-you-do), or we can "dictate" it ("we-*stop*-if-you-don't".)

We generally want to steer hard towards *contract* over dictate; that's a more
precise meaning of 'best-effort.' We play defensively against exactly the errors
we ask you not to make.

However, there's some cases where we're either *forced*, or very very rarely
choose, to *dictate* things - that is, explicitly exclude handling them, and
*fail-fast*, in your face, abandoning our best-effort stance. For example:

 - clear, immediate errors in *Dorc-created* language features and idioms (i.e.
   static typing errors when there *are* declared types, and they *disagree
   irreconcilably*)

(UNFINISHED, FILLME)


Spelling, language-design, and the flavour we want
--------------------------------------------------

[DESIGN.md][] goes into some detail about a core Dorc tenet: staying
"spelled-as-sh." It's worth elaborating on *why*, though. (You'll further note
that, despite mentioning that several times, we've evolved a *very*
not-spelled-as-sh typesystem.)

A portion of spelled-as-sh is flavour; this project was borne out of my general
annoyance at Ansible-YAML, and the observation that "nothing you do is going to
stop ops'ers from writing a bunch of sh; it's as inevitable as JavaScript on the
web." To me, it was *always*, observably, going to be "sh-plus-<something>", and
it follows that the *simplest* thing is to make the plus-something ... nothing.
Just make it all sh.

(If you can't beat 'em, join 'em.)

However, it runs deeper than that: there's a principaled approach here that
draws *towards* sh, other than just "we're stuck with it."

At the end of the day, most ops-tasks involve *doing things on servers*. An
orchestrator, meanwhile, sits *between* the human and doing-things-on-servers.
Worst of all, though, an orchestrator *like dorc* can, and follow this slowly
... decide to *not* do-things-on-servers.

(See the above: that's "under-execute". By my design, the Cardinal Sin.)

To some extent, with the level of chaos and underspecified-unknowns involved in
ops, we'll (we-as-in-Dorc) *never* be able to fully guarantee safety. (See
'correctness' above.) We *tame* it, we bound it and corral it.

In practice, what "corral" it *really* boils down to is two things:

1. rearranging the danger (often *concentrating* the dangerous unknowns into
   focused locations, where users have more leverage to deal with them), and
2. *attributing* the danger (ensuring users know when, where, why, and
   *because-of-whom* something went wrong.)

Don't mistake the second one as CYA blame-game playing: the quickest resolution
to a real-world problem *is* through attribution.

You shift leverage into a place where someone *can* effectively fix it, and you
ensure fixes to that class of problems get *routed to that empowered person. In
Dorc, this usually looks like a deep provenance-web, and language-design that
concentrates and surfaces uncertainty, putting all the uncertainty we possibly
can *into some particular person's hands*.

And here's where spelling-as-sh shines:

**authorship.**

If we never generate code, if we never transpile or collate or restrict, then
there's always *a particular bit of actual-sh, written by a particular actual
human*, that made a particular thing happen on a server. Someone is
*answerable*; and with Dorc's help, we can try to ensure their *answer* is a
quick "oops, I can fix this."

(In many cases, that person is you, wearing your engineer-hat instead of your
admin-hat; which is aligned-incentives with Dorc-as-gradual-enhancement-engine:
issues and problems' warnings can *say* "this worked, but poorly; when you have
time, <over here> is precisely where value-add is most leveraged to make your
admin-life less painful.")


### Be sh, or be *very*-not-sh, don't half-ass it

That said, there's boundaries. The value of spelling-as-sh drops to near-zero
when two things are *both* true:

1. a thing cannot be idiomatic, cannot contribute to the off-ramp: it has *no*
   value (not less; *none*) outside of the Dorc ecosystem;
2. **and** that thing doesn't directly produce a single, concrete thing on a
   server - it doesn't *run commands itself*. (It doesn't need point-to-point
   "here's where's that shell-command came from" provenance.)

In such cases, we try and stay eyes-open to the *downsides* of spelled-as-sh.

Because here's the thing: sh *sucks*; it's a *terrible* programming-language.
It's turing-complete, but pathological for real software-engineering.
Stringly-typed, decades upon decades of accreated backwards-compatibility and
cross-platform-*in*compatability, missing several decades' PLT- and
industry-insight into programming-language design and ergonomics.

So, trying to shoehorn-in "spelled-as-sh" when it has no benefit to us, or when
sh *has* no spelling of a concept, is a fool's errand. Hence the other pole of
our approach:

Either spell it idiomatically, or don't spell it as sh *at all*.

When we break with sh, we break with sh *hard*, and try to follow actual modern,
quality language-design principals.
