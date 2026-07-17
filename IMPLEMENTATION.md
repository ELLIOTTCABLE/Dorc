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


Collaboration
-------------

The above is somewhat mollified if one writes a basic oracle for `hork`. A
simple truth: if `hork` never runs, `hork` *cannot* poison something unexpected
between `apt-get` and `systemctl`. Thus, the trivially-true easiest route around
the danger is to *make `hork` not run*.

The simplest route to that is to write the most-minimal oracle that helps Dorc
fully-elide `hork` itself, in isolation: a convergence-test thereof, plus the
author's blessing to act on it:

```sh
hork__is_converged() { hork --check "$@" ;}
```

This *doesn't* buy you all of Dorc's functionality, but it buys the most of it,
with the least effort; now (again, speaking in a vaccuum, because all of this is
modulo *other* state-actors and CFG participants), as long as `hork --check`
passes, Dorc can safely make assumptions about `apt-get` speaking to
`systemctl`. Abstract-interpretation is unpoisoned, and the richer machinery can
run for those other commands; the poison-wall is lifted.

> Of course, not all tools are so easy to describe. Luckily, this is all subject
> to control-flow analysis. A much more *realistic* minimal oracle, for a
> command that doesn't provide a magical, ops-friendly "give me any invocation
> whatsoever that my command possibly has and non-mutatively tell me whether it
> would do anything" flag, would involve some argparse machinery. Any flags or
> subcommands that you haven't modeled, should simply be *refused* (strawman:
> `return 2` or higher.)
>
> ```sh
> hork__is_converged() {
>    if [ "${1-}" = "flib" ]; then
>       hork --check "$@"
>    else return 2
>    fi
> }
> ```

However, for *better* behaviour, to *fully* lift the poison-wall in all cases
(i.e. enable Dorc to elide *later* commands, even when probing surfaces that
hork is diverged), a lot more buy-in, from a lot more parties, becomes
necessary. (The bet here is that that's *relatively few* parties - the
gradual-enhancement curve might near a cliff at the very far end, but
correspondingly-few tools need cliff-sized oracles; and we can try to ship
high-quality examples that cover many important bases in the stdlib.)

This is where we start to get into the questions around the frame-problem raised
in [DESIGN.md]. How can multiple, mutually-unaware parties, collaborate and
communicate about global state on *someone else's* computer, in the future, *in
a way that most-completely protects that future user from our undesired
failure-mode, under-execution?*


### The refined ladder of sins

That immediately brings us to a refinement of that ladder from above: there's
something *worse* than simple under-execution, than a wrong-skip - and that's
under-execution due to a transited claim *that we can't attribute*. Dorc
operates in a flawed world, and ops will always be full of chaos and mess;
worse, Dorc inherently operates on top of claims made by third-party-non-authors
of tools. But at the very least, Dorc can *keep track of* how it knows what it
knows, and *help you with specific instructions to remedy it* when something
goes wrong.

Hence, our elaborated priority-order for failure-modes:

1. (the worst possible) *mis*-attributed, incorrect elision: if your runbook
   skips a critical command, `dorc why` makes a claim about the cause thereof,
   and *that claim leads you down the wrong path towards remedying that.*
2. *un*-attributed, incorrect elision: when a critical command was skipped, and
   it shouldn't have been, but Dorc cannot tell you specifically why (and
   therefore how to fix it.)
3. incorrect elision, attributed correctly: a command was missed, but Dorc can
   tell you precisely what to do about it.


### Survival: Travelling claims, faultless elision, and user-consent

In particular, it turns out that soundly avoiding #2 in 100% of cases is
effectively impossible, if Dorc wants to effect any traveled-claim elision *at
all*.

What if the above runbook instead looked like this:

```sh
apt-get install -y nginx      # well-known tool w/ a battle-tested oracle
hork tune-packages            # now Dorc has a simple, convergence-only oracle for this
# ... 97 more lines ...
systemctl enable --now nginx  # well-known tool w/ a battle-tested oracle
```

Under our default constraints, argued-for throughout these documents, *98
commnds cannot be skipped*, if `hork` reports unconverged.

For churn-heavy commands, that show up early-on in runbooks, this is
catastrophic to Dorc's value-proposition. (We have a fallback 'guard' behaviour
that may yield performance benefits; but critically, it cannot yield *attention*
benefits: once we've showed the user a plan with the guarded commands included,
that scarce resource of user-attention is *spent*, and cannot be recovered. From
a certain point of view, the full-elision, user-attention-preserving behaviour
*is* the product.)

Solving this requires what we call a 'skip-survival': fully skipping a later
command, *even though an earlier, mutative command actually ran.* In the above
example, this is achieving a knowledge-of-the-world state where `hork` can
*actually run*, and yet `systemctl` can still be completely omitted from the
plan.

Unfortunately, this is impossible to achieve *soundly* - within our earlier
constraints of full, attributed-to-a-causative-source-code-line proof for every
elision.

Thus, we provide two modes, and ask user consent to reach for the greater value,
when it carries higher risk.

1. By default, we *risk no unattributed elision*: if a feature would require we
   leave a hole where some conjunction of circumstances could lead to
   unattributed, incorrect elision ("nobody did something locally incorrect; but
   the net effect of several people's choices was incorrect elision for the
   end-user"), then that feature ships disabled.

2. then, for risk-tolerant admins, we provide an opt-in `--risk-faultless-skips`
   flag that enables such features.

To be clear, we're still defensive against these - as of this writing, the
survival-licensing featureset only results in the described cardinal sin in
particular scenarios, and they're carefully boxed in; but I still consider it a
failure-mode that's owed explicit consent from the admin. People's risk-profiles
will vary.

Part of that boxing is to *double-end* the consent: specific authorship actions
*in the oracles* are also necessary to license an elision that *could*, under
*some combinations of circumstances, end up traveled and unattributable.


### Kinds, reach, disjointness, UNFINISHED - SEE USER_STORY

Thus a division: in our model, claims own what *a particular line* can say about
the world, and the flag ownes what *no* particular line has said (i.e. where
Dorc synthesizes knowledge from *multiple*, mutually-unaware claims that may in
fact synthesize an incorrect fact about the universe that no actual human meant
to claim) ...


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
2. *attributing* the danger to an opt-in (ensuring users know when, where, why,
   and *because-of-whom* something went wrong; and ensuring that's not the
   default, mindless path - attribution-to-fixer requires consent-to-fix.)

Don't mistake the second one as CYA blame-game playing: the quickest resolution
to a real-world problem *is* through attribution.

You shift leverage into a place where someone *can* effectively fix it, and you
ensure fixes to that class of problems get *routed to that empowered person*. In
Dorc, this usually looks like a deep provenance-web, and language-design that
concentrates and surfaces uncertainty, putting all the uncertainty we possibly
can *into some particular person's hands*.

And that's where spelling-as-sh shines:

**authorship.**

If we avoid generating code, if we hard-avoid transpilation, collation, or
restriction, then there's always *a particular bit of actual-sh, written by a
particular actual human*, that made a particular thing happen on a server.
Someone is *answerable*; and with Dorc's help, we can try to ensure their
*answer* is a quick "oops, I can fix this." Having the user write plain sh,
*means we *get* to just *move it into place*, and run it, and make decisions
*based on it - but there's always "an original command" that existed,
*as-human-written, before we took any action.

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
 - when we *can* protect you from yourself, and we know something *provably, for
   sure*

Many modern languages are aggressive about types and correctness; they
fail-fast. I personally consider that the *right path*, for the most part: it
leads to tight software-development loops, and produces better software with
less pain. However, it requires a totalistic knowledge of the world that Dorc
explicitly refuses to attempt. Further, it requires *buy-in* that Dorc refuses
to ask for. Dorc will live *alongside* other things it can't control; it might
not even be the *major* player in your ops story. You might move partially onto
it; you might use it for a small bounded task, it might need to coexist with a
different style or pattern, or correspondinly, it might need to accept a day-one
buy-in with an ocean of unreviewed, un-migrated 'legacy' scripting.

Finally, the kinds of correctness that *matter* in this problem-space are only
going to be ~10% amenable to anything Dorc can ever do, at all - the majority of
pain you'll be feeling during ops-work is inescapable, and cannot be
papered-over even by Dorc having *perfect* knowledge about the scripts you're
handing it. Our best possible value-add isn't to be that tier of perfect, when
the buy-in costs to you *attaining* that value are catastrophically high.


### Contracts, boundaries, horizons

Thus, as mentioned above, in many places, Dorc's core job is to "shuffle risk
around." We take your incomplete work and failures, we accept them, and we try
to concentrate and attribute the bite.

Often, that bite is felt as 'contract':

1. "If you, person-wearing-hat-A, does thing X; then Dorc will ensure Y."
2. meanwhile "Dorc promises you, person-wearing-hat-B, Y will hold true; *as
   long as person-A-did-X."

As mentioned above, the value-add here is heavily dependant on *attribution*:
knowing *why* Y did not hold, and critically, *what your next step is* to get
back to work.

"Contract" is, however, a documentation/marketing topic, at the end of the day:
it's DX. Our *engine design* is primarily about carefully deciding where that
contract lies, and which bits of risk it pushes to where; which corresponding
work it will ask of which player, and what the consequences will be.

These are generally the priorities when designing our contract, our edges, and
our horizon:

0. "do no harm"; this one's obvious, but Dorc not *introducing* risk that didn't
   exist *before* is, of course, literal-bug territory, not design-territory.
1. keep harms *attributable*: one of the worst mistakes we can make is to open
   the door to *risk that is nobody's fault*. Risk-that-is-nobody's-fault, is
   risk-that-nobody-can-*repair*. (This bites hard in "collaboration", above.)
2. keep harms *local*: pursuant to gradual-enhancement, it's important to
   maximally ensure that your mistakes cost *you*, as often as possible; and
   that blast-radius is low; infectious/spreading risk is worse than localized
   risk.

All of these stay live and in-play, though; everything in ops is a tradeoff, and
there are places where we judge that to extract *enough* value, it is worth it
to bend these rules. (Most notably, as described above, collaborative
full-elision *provably requires* nonlocality; and that particular 'hot corner'
is the source of much of these design-theorems.)


### The shared-horizon of fault

There's a natural asymmetry to fault in this space: 'engineer-hat' is almost
always carrying a larger blast-radius, from Dorc's perspective, than admin-hat.
We *want* to encourage you to design high-quality oracles and share them; it's
hard work, it's work that's often goes under-done and under-maintained and
under-shared in the ops space. But that is, simultaneously, brushing
perilously-close to harm-0 above (Dorc creating risk that didn't exist before)
*and* harm-2 (Dorc spreading risk to others.)

Further, introducing a collaborative space introduces *communication failures*:
oracle-author-A doesn't know oracle-author-B, and *certaintly* doesn't know all
of the admins X, Y, and Z who are going to be using their work. Inversely, admin
X is, realistically, not going to deeply read any documentation or communication
from the majority of oracles they use: they're task-focused, they need `foobar`
to stop blocking their deployes, they're going to install a `foobar` oracle and
move on with their firefighting.

For all these reasons combined, there's a truism to our by-contract,
attributed-failures story that we can't work around:

Errors that we *can't* attribute, are necessarily *our fault*.

This isn't in the "our responsibility to fix or prevent" meaning of fault, to be
clear. As elaborated above, it's *impossible* for us to even *know* about
most/many of the things that may, by the above logic, be our fault.

Hence, the 'horizon' - in a way, the inverse of the 'contract.' The contract is
the small, bounded surface between Dorc-and-engineer (and in particular, the
*exclaves* of the horizon; the places where we can *attributedly* transport risk
between engineer and admin.) Instead of "here's what we-Dorc need from
you-author, to keep the promises we make to yourself / to others", the horizon
is "here's what we *decline* to promise, ever, forever."

That is, the horizon is what we sell to admins: our product is inherently an
*attention* product; "we promise you you don't need to worry about <x>, within
bounds <y>."

Part of <y> is the oracle-contract; so the very first part of the
horizon is inherently "oracle-authors doing things wrong." (We promise
gradual-enhancement, not *repair* - if you do nothing, we can recover; our
defaults are safe. If you take action, but take it *wrong*, and break the
contract, then we break our promises to the admin.)

Another part is the general messiness of the ops-universe. (The canonical example
here is a pathological `apt` "maintainer-script" that uninstalls an unrelated
package: Dorc can't know about that, the admin fighting a fire in real-time
can't know that, the person writing the `apt` oracle can't possibly know that.
Most importantly of all, from this document's perspective, Dorc cannot hope to
*attribute* that, without live instrumentation running on every host, watching
every fact's backing-truths in real-time.)

And here's the key observation: *our* horizon has to *subsume the
oracle-author's horizon*. This flows from one of the very first paragraph in
this section: in a realistic world, the admin is *not reading* the deep,
detailed risk-assesment documentation for every oracle they use.

This means we, Dorc, not individual oracles, need to manage that horizon: we
need to decide, dictate, and advise oracle-authors on *where the horizon lies* -
the outer boundaries of their contract-to-the-admin are a part of their
contract-to-us.

That said, 'hoziron' is always a *minimum*: it's the
shared-basis-of-attention-discard; it's what we, Dorc and our
(trained-by-documentation-and-lint) oracle-writers *promise* the admin they can
stop worrying about. Oracle authors, and Dorc, can of course always go
above-and-beyond to produce *extra* safety.
