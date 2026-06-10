Dorc: Design notes
==================
(README.md describes what; this describes why and how.)

The value of this tool hinges, potentially *moreso* than most software, on UX.
There's a sound niche in the design-space, but the true differentiator isn't so
much that "Dorc is surprisingly good" as it is that "Ansible is surprisingly
bad." (Due, of course, to limitations imposed by early design decisions; not
incompetence! I'm not an Ansible hater; there's a reason I'm trying to build an
Ansible-inspired tool, lol.)

This tool is rather especially subject to the network-effect; like Ansible or
any other orchestrator, it will live or die on being able to bootstrap a library
of contributed components, corresponding to the gargantuan, heterogeneous ocean
of Ops Things in the World™. Pursuant to that, it's important to make it
easy/fun to use *outside* of large industry; to some extent, although I want
this work to be purposeful outside of hobbyists, I strongly suspect
ops-hobbyists (homelabbers) and individual developers ("I have a personal
website" or "I want to manage my dotfiles and config across multiple
heterogeneous development machines, without faff") are my realistic
target-market at the moment, outside of, well, myself.


Why this design?
----------------
The primary (only?) downside of some of the most popular non-Ansible tooling out
there, besides the much-discussed agent/agentless division, is *upfront cost.*
To get much (any?) benefit out of many of these tools, you must:

1. Learn an immense pile of new things, including a new configuration-language;
2. learn, and avoid, the *footguns* of those things;
3. 're-write' (at the very least conceptually, if not literally) the specific
   deployment of <thing you want deployed> into that language; and
4. most importantly, for many of them, recurse to #1 on *basically the rest of
   your infrastructure*.

That's, tbh, usually the *correct* approach, imho. Ansible has been largely
superseded for a reason. However, if we focus tightly down to the usecases that
*aren't* well-served by that approach, we can extract a few values:

 - just like any software engineering, devops can occasionally benefit from an
   exploratory, experimental approach: try a thing or approach out before you
   invest too much effort into it.
 - YAGNI; 80% of the probably-good-for-you-tbh features of Ansible or Terraform
   or K8s *might be irrelevant* to the thing you're currently exploring.
 - time-to-first-successful-user-goal: it's *more rewarding* to reach grounded
   success in your goals earlier on
 - and potentially most importantly, *rewrites kill projects.*
   Modernization/refactoring is expensive and rarely easy to justify if it must
   be done in a wholesale, stop-the-world fashion.
 - when the tool fails you, you're gonna have the urge to stop trying to work
   within its framework and limitations; you're going to be pushed back towards
   the old, lingua-franca standbys to get the problem *solved*.

Against all of these goals, the combination of 1. pure-shell and 2. gradual
enhancement, holds up particularly well.

 - shell's *already* the language-of-exploration for devops. "SSH in and just
   run the command" is the *default* that everything else has to pull you *away*
   from, in some justified direction.
 - there's no 'cliff' where YAGNI must be swallowed. Any value you want to add,
   you add through the same language you already know, mostly in the same idioms
   you already understand.
 - the first "I used Dorc successfully" is five minutes in: `apt-get install
   dorc; printf 'apt-get install docker' >docker-host.dorc.sh; dorc apply
   docker-host.dorc.sh some-host.my-domain`. The first "I benefitted from Dorc"
   is ten minutes in: when you add a `if dpkg -s ca-certificates; then ...`
   guard that Dorc can analyze, lift, and probe.
 - if you have a large existing legacy infrastructure, I'd ideally like it to be
   *minimally* invasive to start using Dorc for parts that you're currently
   managing with shell-scripts; although this is a common flavour of claim that
   I'm generally suspicious of, personally. This will differ for each org; but I
   hope for it to always be a strictly *easier* sell (although not necessarily
   the *correct* one) towards recalcitrant management.
 - and if Dorc fails you in any way, the off-ramp is absolutely trivial, both in
   the immediate term (`ssh some-host.tld 'dash -s' <myscript.dorc.sh`) and in
   the longer-term/organizational sense (it's all just executable shell-script,
   scattered with good-idea-to-guard-anyway checks and guards; *maybe* with some
   slight dorc-isms that you'd omit idiomatically. Use as-is or clean up or
   'promote' to a newer, better language, do whatever you like, it's
   bog-standard.)

There are, of course, massive downsides:

 - everything becomes much less guaranteed, and much more best-effort. (There's
   a *reason* the popular projects chose to develop new languages, deploy
   complicated and strict configuration requirements, make assumptions about how
   well you've learned them and how much time you'll dedicate to using them
   correctly.)
 - every bit of effort you invest, although hopefully
   gradually-and-fairly-repaid with direct benefits in that 'gradual fashion',
   is *also* locking you into this approach - just like with any tool. There's
   some tipping-point that will be unique to every project or infrastructure
   where "fuckit, we spent so long doing this that we would have been better off
   stop-the-world re-writing it in a strict environment from day one."
 - there's entire classes of sanity-check, or optimization, that will *never* be
   possible on top of a sh substrate; we're abandoning any hope of providing
   those from day one.
 - I mean, you have to write ... POSIX-shell. Yikes.

I, personally, think many of these are covered by simply saying "go use the
better tool", though. My perspective is that this niche exists and can be better
served; not that it should be eliminated or ignored.


Dorc's approach
---------------
The absolute dumbest, most-straightforward way to use Dorc:

1. write an arbitrary (but pure-POSIX-sh; not bash/zsh/fish/what-have-you)
   shell-script
2. hit 'run', and that script runs on all of your hosts, serially.

Everything else from here on should fall back to that, because *that is what you
will fall back to if any of this works poorly or gets annoying.*

Now, let's enrich it.

The very first thing you'll reach for in an ops setting, and one of critical
importance to "the Ansible approach", is usually called "idempotence." (And no,
Mark Burgess, I do actually mean idempotence, [not convergence][].) When you
have some imperative set of instructions that has, at least, upon occasion,
resulted in a desired state; you often can improve how often that 'on occasion'
applies by simply ensuring it doesn't run when something's already done - "if it
ain't broke, don't fix it", because computers are chaos-gremlins and *will* find
a way to fuck themselves up if you touch them.

However, the *approach* to idempotence is very often a 'check-then-execute' one:

    FILLME: CODE EXAMPLE

When you have an entire Ops Ocean of state to interrogate and apply,
imperatively, it can be quite silly to actually *execute* a deep tree of these
check-then-execute blocks (think Ansible roles), when many of the following will
be true:

 - most of the 'roles' will already be applied on many of the machines;
 - some of the 'roles' will *fail* to apply for predictable reasons, that can't
   currently be fixed, on some of the machines;
 - some transitively-dependant subtree of state will be entirely invalidated by
   one of those first two.
 - and many *parts* of those three will be irrelevant to the <thing you just
   changed and want to experiment with> or <thing that's subtly breaking and you
   want to debug>, which is *very* frustrating in a tight develop/debug loop in
   real-world practice.

But that above 'flavour' of code, holds. Although shell-script has become the
defacto language of ops, there's some useful truths about [the *kinds* of
shell-script that's common][corpus] in these settings; it's usually
'embarrassingly shallow', with simple, global-system-state guards: "does this
configuration file exist", "is this package installed", "is our hostname in
<set>", "does the config-server respond to ping from us", so-on and so-forth.
Such guards and interrogations necessary for idempotence are, however, *written
locally*; and *especially* in the case of collaboration amongst
teams/communities, when automation is constructed for local/small components and
parts of what will eventually be the large, complex system of a machine's state,
those become slow to interrogate serially and individually, over and over, every
time you change any little thing about the system in question.

Pursuant to that, well, it's 2026. We're adults, and many kinds of static
analysis are borderline trivial, nowadays.

### given oracles, probe; then converge.

Our approach is double-ended.

1. "From the host, backwards", we'll attempt to establish information about
   relevant *current state* on those machines:
    - when-possible - often only-when 'it's state that can be statically,
      globally checked'; and
    - when-performant - i.e. ~mostly 'when it's somehow-faster to check than to
      apply'.

2. Additionally, "from the working-dir, forwards", we'll read (if asked)
   information about the relevant *changes* to the system-state:
    - when relevant/possible - i.e. the user has requested a
      partial-apply/update/git-sync; as opposed to a genuinely-ensure-the-world
      convergence/reconciliation; and
    - when up-to-date - we're not going to be long-term arbiters of
      centrally-claimed-remote-state; but we *may* short-term-persist probe- and
      partial-application-results to reduce immediate work on subsequent
      re-runs.

Importantly, all of this must be optimized for that 'embarrassingly shallow'
case in an 'embarrassingly parallel' fashion:

1. First, the "probing phase": For the state-establishment, we'll use one
   static-analysis; we *lift* (on Dorc's part) & *author* (on the
   user/community's part) any relevant state-oracles that can
   vouched-safe-to-run; and ship *sanitised, non-mutative,
   massively-parallelized* versions of the ops-automation to all the hosts in a
   "probing phase". This allows us to learn correctness-affecting facts about
   each machine, that we can use to tune the deployment/application phase.

   For these purposes, the probe-analysis must *under*-approximate - that is,
   it's better to not ship a probe at all (losing opportunity to elide
   evaluation later), than it is to ship something that may be mutative.
   (Another way to look at this: an optimizing compiler with an unusual idea of
   'correct'; or a query-planner.)

2. Then, the "application phase": a second, inverse view of the analysis,
   potentially (if running 'unsound') with *elision* of portions of the
   control-flow-graph that is fully irrelevant to the user's goals - either
   already correct on the target system(s) (by-probing, not
   by-stale-central-state), or having no control-flow interdependency with the
   modified portion of the ops-scripts, in partial-deploy-mode. ("Stuff that is
   already correct; *or* stuff that, while potentially incorrect,
   we-don't-care-about-right-this-second.")

   This apply is the *over*-approximation phase. Same analysis, different
   fail-safe posture, different performance profile.

   (Both #1 and #2 are *heavily* dependant upon oracle coverage, though: an
   operation with no 'oracle' to reassure us that a particular check is
   non-mutative, is something we can *never* run in non-mutative probing-mode,
   no matter how obvious it may seem. Similarly, an operation with no 'oracle'
   to declare its (hopefully-lack-of) global-data-dependencies to us, is again
   an operation we can never elide or rearrange during application.)

   Luckily, both are designed around gradual enhancement: you make the most
   minimal claims to us that reassure *your* risk-profile, and Dorc will do as
   much as it can for you, within the bounds of correctness. Put another way -
   the more you put in the effort to tell Dorc, the more Dorc can skip.

3. Finally, we can present all of this in Terraform's plan/apply UX: take our
   constructed plan for the 'apply' phase (hopefully dynamically updated in
   real-time as the probe-phase asynchronously proceeds over-the-network, and
   uncovers elision-relevant state on various targets) and present it, *still as
   a simple shell-script*, to the user for approval/edition. In ideal cases, the
   entire repo-full-of-shell-script-equivalent of "running an entire Ansible
   playbook" can hopefully be reduced to one or two shell commands, directly
   narrowed to the state-mutators relevant to the user's current
   goals/changes/garbage-fire-with-business-consequences, which they can proceed
   to interactively execute or modify as appropriate.

Presenting a *plan* to the user, though, means we don't have the apply-phase
'convergence floor' for the probe-phase; it's relatively performance-forward, as
the user is sitting there, waiting on a coherent plan (i.e. for all the probes
to finish and return), before they can hit 'submit' and go off and do something
else in the worst/slowest case. We can't be depending on multiple runs for
truth. Further, we don't have a guarantee that the user will *accept* the plan -
by the very act of presenting a plan and giving them an option, we're implicitly
promising to *not* pre-apply parts of the presented plan. (i.e. providing
plan/apply limits the "safety fallback" of probe-causes-mutation; even
probe-causes-maybe-eventually-*desirable*-mutation is, sadly, verboten.)

   [not convergence]: <https://web.archive.org/web/20121109085116/http://chester.id.au/2012/06/27/a-not-sobrief-aside-on-reigning-in-chaos/>
   [corpus]: <FILLME>


Priorities
----------
Given all the above, then, we care about, in approximately this order:

1. **correctness**, but only within the bounds of our contracts w/ users: our
   promises ("plan stage doesn't mutate", as long as your handwritten oracles
   don't; "all state will be applied", as long as your oracles don't lie) *must*
   be kept;
2. **user effort**, with respect to the value we then provide: again,
   NixOS/K8s/Terraform exist; if we 'ask too much' of our user, they might as
   well just use those, which will be strictly better, as they don't have to
   work off of *inference*. We're worthless if we aren't scrappy/best-effort
   based on very little user-input.
3. **performance**, at the "cross-network wallclock total": this is the primary
   pain-point we're trying to address, subject to the above two limitations,
   Ansible, while annoying, would be relatively *fine* if it didn't force you to
   choose between taking 45 minutes to re-execute an entire play after one minor
   change vs. trying to narrow down some overcomplicated manual set of
   tags-to-re-apply.
4. and finally **invisibility**, which is basically an aesthetic reframing of
   #2: we want to *feel*, to a user, like they're just writing bog-standard,
   ops-y shell-scripts. In most domains, "magic" is bad - in a PLT regime, I'd
   be pushing for precision and correctness and overspecification ... but NixOS
   exists. We don't *need* to be that, here. So, let's be magic.


Project components
------------------
The goals here break down cleanly into a few moving parts:

1. A *language*: For a variety of reasons, I want to *closely*, but not
   *perfectly*, follow POSIX sh. For end-users, it should be basically
   indistinguishable, but I do *not* want to make that promise - or rely on
   existing shell-parsers, for example. 99% of scripts should parse as-is, but
   I'm comfortable with very-weird edge-cases (or, y'know, any and all
   bashisms/zshisms) failing to parse. There are angles to the future shape of
   this that I *do not* want constrained by 'oh, this conflicts with the
   edge-case parsing of a particular dash implementation-detail.' (Someday, this
   language may involve further features and elaborations that make it a strict
   superset of POSIX-sh; but not yet.)

   (This is the second-most-critical part; rising to "most critical" when
   calibrated against how locked-in any decision becomes.)

2. An *analyzer*: the core value-prop, here, past what things like [FILLME][]
   and [FILLME][] and [Mitogen][] and the like can provide when used with plain
   shell-scripts, is *what we can do differently* in the devops-development,
   devops-debugging, and ops-deploy/apply stages based on information we can
   extract from plainly-written shell-script. A rich and pluggable analysis
   engine for sh, with abstract interpretation and projected system-state,
   should allow us to produce that 'better UX' - both gain minor perf over even
   Mitogen et al. in some edge-cases, as well as provide excellent *reporting
   and feedback* during those tasks. (Good error-messages; clear provenance of
   'why'; strong hints as to failure-modes/latent bugs ...)

   (This is, well, the thing. This is what I want, and this is what I am
   building; everything else is noise-but-necessary-to-make-it-not-suck, as far
   as I'm concerned.)

3. An *orchestrator*: I suspect, frankly, that mine will be shitty. This is not
   my strong-suit, I am not deeply familiar with the internals of the prior-art,
   and it's an absolute bottomless pit of corner-cases, hard problems, and
   encoded real-world experience that I just don't personally have. For this
   reason, I deeply hope to make this as minimal/ceded-to-tools-better-than-I,
   as possible.

   (Depending on what constraints doing so may introduce to the parts I care
   about, I may even try to make this *pluggable* - have separate 'deploy Dorc
   over an Ansible inventory' or 'integrate Dorc into your pyinfra scripts'
   deployers.)

4. A *library*: All of the above operating, as described, in practice, accounts
   only for a very narrow window of ops. It handles the stuff *between* your
   Kublet-managed docker container and your Terraform-spawned host; and to be
   valuable, it needs to function in that position for a *massive* array of
   substitutions for the words "kubelet" and "terraform" in that sentence. I
   absolutely do not intimately know all of those tools, I suspect nobody does;
   just like Ansible or anything else, then, the solution is clear boundaries, a
   clear API, and ensuring the existence of *some* answer to the question "what
   is this `docker` command I see and what does Dorc need to do about it to do
   its job well?"

   This is, obviously, the hardest actual part; I cannot magically
   summon-into-existence a bunch of knowledgeable code about domains I do not
   know or use. Nonetheless, in a way, it's a non-concern ... because the only
   control I have over it is:

5. DX *tooling*: This, while less *exciting* to me, feels like the thing I have
   the most personal leverage over. I have plenty of experience here, and I do
   think it's the zone that most shapes *this* niche of Ansible-alikes. A large
   part of this project won't be the analyzer-codebase *nor* the actual
   orchestrating-executor; but rather *things that absolutely minimize the
   effort* that users need to invest to make this tool Do Good Things for them.

   There's a cloud of potential components here, some bog-standard, some local
   to the problem-space; from containerized/sandboxed executors with eBPF/ptrace
   to surface missing dependencies-of-probing-oracles, to linters and LSPs. This
   is the "long tail" that makes this thing-I'm-building *actually good* (in a
   dislocated fashion, by encouraging more-and-higher-quality oracles), and
   where the bulk of the *work*-of-making-it-good lies.


Sensitivities
-------------
The viability and value of Dorc depends heavily on a few things that are either
outside-our-control; are *inside* our control but need careful cost/benefit and
then Lots Of Work; or that we only 'control' in the sense that we throw up our
hands and directly make it the user's problem:

1. *Oracle-quality.* Although 'correctness' is discussed herein a lot (see
   'Priorities' above), it's *very bounded*. We can only be as correct as the
   user-supplied code, on *both* ends. If either end (the "admin" writing the
   ops-body, or the "devops" writing the oracle - usually the same person) fails
   to follow the contracts we present, we're immediately unsound. Our idea of
   correctness is *best-effort*, and the real-world user-experience will be
   greatly degraded by the reality of imperfect library-code, an imperfect
   understanding of the world. (That is, after all, the point of our tool:
   trying to be as good as we can be, while requiring less knowledge and
   perfection than, say, Terraform/K8s.)

   Design-wise, this will be a question of maximizing correctness and careful
   contract-design (what we're deciding to own); implementation-wise, it'll be a
   matter of A. *deep* defensiveness, B. *even deeper* user-friendly
   error-handling, debugging, and tracing/provenance, and finally C. rich
   tooling/support to minimize this class of causor. (oracle-author tooling;
   containers, test-harness, CI ...)

2. *Subtle traits of real-world ops habits.* This tool is billed on, and built
   for, laziness - keep doing the easy thing, don't learn Terraform or whatever
   (see 'Priorities' above.) You have some scripts, keep using them. However,
   "keep doing what I'm doing" doesn't magically yield value; I'm banking on
   being able to *add value* to someone's *existing habits*, without asking too
   much of them at once. That's *very* sensitive to, well,
   what-people-actually-do-when-they're-feeling-lazy; *some* lazy habits may
   just happen to be freely subject to static-analysis (and I'm betting this
   project on the intuition that they are); but just as easily, a bunch of users
   not showing up in CoLiS could be heavily `eval()` users, completely
   destroying any chance of control-flow-analysis, lifting, elision, or
   query-planning. (Or tons of ops-tasks might not be sensitive to the sorts of
   *improvements* my analysis can make - lifting/probing/skipping - and instead
   just be genuinely-best-served by a dumb, linear run-through, every time, with
   no smarts.)

   This will respond *somewhat* to a few design-`KNOBS.md` we can turn (in
   implementation - adjusting user efforts, rewarding certain ones; good
   error-messages, good hints and warnings; as well as in documentation/prose -
   extreme frontloading of "you get out what you put in", laziness is allowed,
   not rewarded, etc); but to some extent, if the reality bears out that *the
   majority* of habits people have defeat *the majority* of analysis that can be
   made sound, then using this tool becomes ~equiv to learning an
   objectively-better, describe-the-world alternative; and *this* tool's value
   drops from "you get to be lazy *and* it's got lots of ansible's traits for
   free!" to "you have to do as much work as for a strict tool, but to have an
   Ansible-tier unsound/gradually-typed one." That's only a value-prop to a very
   small set of people, and almost entirely based on taste ("I *really* like
   shell" or "I *really* like no-daemon".)


General design principals
-------------------------
A grab-bag of aesthetic (or-target-audience-narrowing) design-decisions with
throughlines in the various moving parts:

 - POSIX-sh as the sole-source-of-truth. We don't want to just avoid graduating
   to the *author* having to half-ass-re-implement Nix/Ansible-YAML; we extend
   that further, into our interfaces and APIs. (UI? Printing shell-script
   snippets and annotations to the CLI. API? Tracing/modules communicating by
   being fed sanitized shell-script and producing their own
   additions/replacements.) There may be some exceptions to this for
   practicality (a big one being error-handling/provenance, that will *have* to
   be OOB metadata); but for the most part, dogfood-first.

 - do-one-thing-well: even if we lose some corners of our design-space to it,
   it's much better to let someone else maintain (and perfect) a subcomponent
   than to NIH it ourselves. I want to depend heavily on external tooling where
   possible. More importantly, this makes your tool *easy for a user to reason
   about*. The less it *does*, the more it fits in with other tools, the better
   it is in practice.

 - provide best-effort based on implicit user declarations: be 'magic'. (Because
   the downside of 'magic' is hard-to-correctly-reason-about, but Dorc is
   strictly dominated in all of "correctness",
   "quality-when-significant-reasoning-is-applied", and *specifically*
   "correctness-when-reasoning-is-applied." The downsides of being 'magic'
   aren't as "down" for us.) For instance, if running in 'I just made a change,
   figure out what it was and re-apply only the minimal transitive-dependents'
   mode, e.g. `dorc bump x.sh`, pursue the implicit user-goal there (hot-loop
   perf, answers, feedback) over strict correctness ("I can't do that because
   the dependencies of <x> aren't fully clear to me.") Provide *different*
   user-goals as backstops against the unavoidable downsides thereof, e.g. `dorc
   reconcile x.sh` to say "pursue correctness, fully apply everything."


Contract & DX
-------------
For the two users, there are two 'flavours' of code. These *must* be
intermixable in the same file (and will usually start that way); the terms are
shorthand for the *style* of code, and two different ends of the UX-design-space
we try to simultaneously inhabit:

1. the 'oracle' - a body of sh, that we try to encourage the user to write
   *well*, with extra, defensive guards and certain behavioural expectations. (a
   region of code about which Dorc allows itself to make more requests and soft
   constraints - think Ansible "role" or Ansible "task", in flavour); and

2. the 'runbook' - the "rest" of the sh; allowed to be more chaotic, imperative,
   mutative, and generally written *to get something done*: to make a server
   behave a certain way, fix a bug, or so on. (think Ansible "playbook.")

For the properties we desire to provide to the ops-team/admin, we need to demand
certain things of the devops/engineer.

Per our overall goal, these can never be *guaranteed* (we must allow
gradual-enhancement, we must support/warn/error thoroughly, and we must
fail-fast-and-clearly, and finally, we must fail-safe, if any of these are
violated), but:

 - High-quality oracles should avoid mutation, in the correct, non-mutative
   sections (probing)

 - High-quality oracles should be fast, in the correct, fast sections (probing)

 - High-quality oracles should be totalistic in their guarding/sh-spelled
   'annotation' of the tools they're describing (i.e. a thorough
   exploration/rewriting-in-POSIX-sh of the state-*dependencies* of their
   target) ... for which we hope to eventually provide tooling (containerized
   TDD, eBPF, more static-analysis, runtime warns, etc)

 - but conversely, high-quality oracles should *not* change behaviour in the
   happy-case (*all* non-probing oracle-wrapping should consist of, effectively,
   hopefully, no-op protective guards.) Oracles surface existing behaviour, they
   don't change it (they express "docker will fail and exit if the file you pass
   it doesn't exist on-disk" in a static-analyzable, native-sh-spelled,
   fail-fast-and-cheap way; they don't *change* what `docker` will do if they
   are omitted/elided-removed. They should always strive to be a functional
   no-op to behaviour.)

Our contract with the oracles is what makes our product, more than anything
else. Encouraging and ensuring the maximal-quality of oracle; and gentle
handling of *less*-high-quality-oracles, is what differentiates.


Inference limitations
---------------------
Inference lies at the core, but cannot magically solve *all* problems. It's
constantly at tension with the opposite points on the triangle -
'ask-little-of-the-user' (sharing a vertex with 'fail-safe' and
'fail-helpfully'), and with *itself* at the other vertex (to infer something
about A from code containing B, I then need to know something about *B*. It's a
chicken-and-the-egg, also called the 'symbol grounding problem'.)

For one, no amount of analysis will tell you whether this probe is safe to run
during the non-mutative probing phase:

```sh
mycmd.check() { mycmd --dry-run "$@" ;}
```

For this reason, oracles effectively *have* to vouch for invocations of their
own command, in their own body. Simply by existence, an oracle vouches for
*itself*.

Relatedly, consider:

```sh
dpkg -s nginx || apt-get install -y nginx
nginx -c /etc/my_config
```

Without any metadata, without either special user-effort or a blessed
library/accounting (of every tool on earth) from *us*, that starts out, looking
to an inference engine like:

```sh
hork -s wombat || wibble -y wombat
wombat -c /etc/myconfig
```

Does the repetition of `wombat` in the first line, with control-flow, imply
`hork` is an idempotency-guard for `wibble`? Is the `wombat` on the first line
the same as the wombat on the second line?

What if this shows up in *another*, separate script in the analysis-unit:

```sh
command -v nginx || brew install nginx
```

What about now? Maybe we somehow have ascertained (user-direction?) that `dpkg`
and `apt-get` handle the same *thing*, the same *unit* of shared-state; but what
about this new, unknown `brew` command? How do we know that now? Concisely,

 - to *infer*, untold, that `wombat` is trackable-state, you'd need to know that
   `hork` is a *probe* of some kind; but
 - to *infer*, untold, that `hork` is a probe of kind K just from the CFG, you'd
   need to know that `wombat` is a K-entity.

So, cross-oracle compatibility will *have* to be anchored with some sort of
grounded anchor. A type-declaration, effectively. Without direction, even *with*
our relative 'unsoundness', about the best we can do is collect a
"company-it-keeps" record and provide *hints* to the user that better beavhiour
may be gained by categorizing/enriching `wombat` ("this looks like a guard,
maybe write an oracle for it.")

(UNSETTLED, CONTINUE)


"POSIX" sh
----------
I mildly-intentionally conflate the phrase 'POSIX sh' with 'Dash' and, well,
Dorc; but there's actually a narrow spectrum here.

There's two poles:
 - the *minimum* subset of the common shells users might be already using and
   familiar with (i.e., the *smallest* superset of the genuine POSIX standard), or
 - the *maximum* subset of the common shell features (i.e. the *largest*
   superset that still works in all of bash/zsh/dash/ash/etc.)

The target-language we're aping/imitating is subject to a few constraints:

1. We don't want to restrict ourselves (and our users) to the *literal*, classic
   standard (frustratingly, that is never what anybody means when they say
   "POSIX-sh"; it's almost universally used to mean "what dash and ash support
   on elderly remote hosts, no bashisms."); we want it to be easy and
   as-ergonomic-as-possible for our users to write *good, defensive* code,
   without learning new things; (esp. 'how to write defensive code in the
   absurdly-restrictive O.G. POSIX standard'; lack of `local`, for instance, is
   straight-up silly to target)

2. ... but it's *critical* that we maintain our on/off-ramp for a *variety of
   users*. People are used to bash; people use zsh everyday at their terminal.
   Their favourite distro has some pared-down shell they are *just barely* used
   to writing defensive scripts for. We don't want to lock users into a shell
   they *otherwise don't use* - asking zsh daily-drivers to write bashisms into
   their costly-to-author ops-scripts is a non-starter, as that completely
   destroys our offramp.

3. Finally, we're constrained on engineering-effort here: probably the *most*
   ideal for users, would be to manually target the second pole above: the
   *maximal* subset of the popular shells. ('Bring all your bashisms, as long as
   they're also zshisms!'); but that effectively means we have *no single target
   at all*. That turns us into an arbiter of "what *do* the two major shells
   sufficiently-similarly implement?", and turns fidelity/correctness into a
   moving target.

This, too, is unsettled; but my strong lean is towards a very mild superset of
POSIX; or maybe a slight subset of POSIX2024. If we can target a *specific
minimum version* of a *specific piece of shipped software* (i.e. `dash`
>=0.5.13), that has two massive benefits:

 - we can *ship* that version as an *executor* in some circumstances; and
 - even if we end up not doing that, 'testing that we do what we promised to'
   becomes 'testing that `dash` and our evaluator preform identically', a much
   easier testing/development target.


Prior art
---------
Conceptually, Dorc's problem-space shares a lot with:

1. Obviously, orchestrators and related tooling. That's thoroughly discussed
   above.
2. branch-prediction or a RDBMS query-planner; that's got *heavy* overlap with
   the provably solution-less, empirical, best-effort approach we're taking to
   "do whatever we can within imperfect-user-effort"
3. optimizing-compilers-but-unsound (from their perspective), in terms of the
   probe-compiler;
4. Maybe-surprisingly, build-systems? Their problem-space of "don't run things
   you don't need to" has been well-explored for decades.

We have many advantages that the above doesn't (or disadvantages, depending on
how you look at it), primarily around the "we're *not claiming* soundness to the
user under certain execution regimes" - our very core claim, posix-sh,
gradually-typed, is *inherently* lossy and unsound in a way that fully dominates
our engine's design.

Given that, we have lots of optimizations and aid that the above could never get
away with:

 - from moment one, we have to export a lot of correctness-claims onto the
   oracle-writer. If your oracle mutates, tough shit. If your oracle is slow,
   tough shit. "You get what you put in" is both our offer and our curse.
 - given that read-only is strictly a greater claim than idempotence, we can
   *always* reserve the right to fail back to multiple executions ('trivial
   convergence'); we don't need to pursue **minimality** as aggressively as a
   *real* build-system - it's a perf-cost, a value-loss, but not the main reason
   the user is here, so to speak.
