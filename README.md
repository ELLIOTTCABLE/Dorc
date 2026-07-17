Dorc, my "dash orchestrator"
============================

[Lazy code motion][] (PRE/DCE) for ops scripts.



What dis
--------

A specification-mining (think TypeScript type-narrowing guards
static-analysis-based orchestrator/system-automation-tool, where instructions
and config can be spelled in idiomatic, POSIX sh. Designed for
gradual-enhancement and best-effort defensiveness; the tool you use when you
*want to be lazy*.

Describe a system in shell; apply that description by running the shell.
(Dotfiles, networks, developer-environments; whatever description you want. If
it's idempotent, but you're lazy, you probably want Dorc for it.) If that sounds
like what shell-script already is, well, that's the point: Do what shell already
does well, without perturbing your expectations.

Given:

 - that script ('runbook'), alongside
 - a library of also-POSIX-sh, read-only tool-descriptors - "oracles" that Dorc
   can abstract-interpret (locally) and ship/execute to interrogate state
   (remotely),

... then Dorc can intelligently *probe* a machine, and *elide* (fully) or
*guard* (partially/performantly) portions of that runbook according to the
actual, live system-state - saving you attention, the risks of non-idempotence,
and time.

> (This project is my exploration of the feasibility of replacing Ansible with,
> well, shell-script (because fuck YAML); while trying to best-effort retain
> some of the soundness and performance gains of the Big Boy orchestrators.)


Rationale
---------

My core thesis:
 - Terraform, K8s, Nix, etc exist, and are good and pure and you should probably
    be using them ...

 - ... however, Ansible-alikes still have a (much-reduced) place. Specifically,
   by 'Ansible-alikes', I mean:

   1. **imperative, *not* declarative.** (the "stuff I want to fix here" is
      *defined* by 'what terraform or k8s suck at.' I much prefer declarative;
      but there are corner-cases all over the universe where it's just not
      feasible to perfectly align and totalistically describe every lil' bitsie
      of a given network of systems.)
   2. **push, *not* pull.** (Puppet et al. are great, but sometimes it's just
      nice to have one less thing to manage. further, one-less-thing-to-manage
      often translates to *direct* correcness and security-surface-area
      benefits.)
   3. **gradually-enriched.** (sure, I could write a 500-line YAML nightmare
      fully specifying the precise idempotency representation for a little toy
      tool I want to deploy onto a machine; but ... sometimes it's most
      efficient to *start* with `apt-get install the-thingie && the-thingie
      --start` and work your way up to all the Heavy, Correctness metadata and
      structuring. Again, NixOS exists; if you're gonna write the 500 lines of
      YAML upfront, just ... use the better tool, now?)

 - Given that the're useful in theory, though, we can probably (definitely) do
   better than Ansible's UX *for that niche*, using modern tools.

In pursuit of that, I want to follow to its logical conclusion this particular
observation: **When Ansible gets annoying, one tends to fall back to "just write
a goddamn shell-script and ship it to the host."**

Thus, Dorc: an attempt to build an Ansible-alike (by the above constraints;
explicitly *not* trying to supplant/replace the Things That Are Already Good),
but one whose *UX revolves around shell-scripts*, staying tightly-bound to "what
you would do if this tool didn't exist", and simply enriching/supercharging that
precise approach.


### What this very-isn't

There's many things we're trying very hard *not* to be:
 - **the best version of an 'orchestrator' tool** ... because the *best version
   needs to describes the whole world*. (Again, if you haven't yet, and you can,
   please, just go use the *actually good* tools. Never choose a
   gradually-enhanced tool for a greenfield project!) That niche is well-filled,
   we're not headed there.
 - **provably sound (at the whole-system level.)** We draw strict boundaries
   around things which will *make* us unsound, and try to make that distinction
   as clear as possible; and further, we will never *stop* you from doing what
   you want to get done, in pursuit of soundness (again, go use the better
   tools!); but we will instead promise soundness *within* only those strict
   pre-conditions. (Some have called this "soundiness" - [Livshits et al.,
   2015][soundiness].) This tool aspires to best-effort, best-effect in a *very*
   chaotic and uneven problem-domain; and I won't have perfection holding it
   back from that.
 - **battle-grade,** and probably never will be; but, see-num-1; why aren't you
   using the *correct approach* for that? This is here to help *your* stuff be
   *more* battle-grade; but that by-definition means we're always going to be
   operating on relatively-toy-stuff

(Further elaborated in [./DESIGN.md](DESIGN.md).)

   [Lazy code motion]: <https://www.cs.cornell.edu/courses/cs6120/2019fa/blog/lazy-code-motion/> "Ryan Doenges. CS 6120: Lazy Code Motion"
   [soundiness]: <http://soundiness.org/documents/InDefenseOfUnsoundness.pdf> "Livshits et al. (2015). 'In Defense of Soundiness: A Manifesto'. Communications of the ACM. 58. 44-46. 10.1145/2644805."
