# What Dorc does

You have a shell script that sets up a machine. Maybe it installs some packages,
copies a config file into place, enables a service, opens a firewall port. You have
been running it by hand, or piping it over SSH, for a while. It mostly works. The
annoying part is that most of the time, most of it does nothing new: the packages are
already installed, the config is already correct, the service is already running - and
you sit there watching it re-do all of that, hoping nothing subtle got worse, so that
the one line you actually changed can run at the end.

Dorc exists for exactly that situation. In Dorc's vocabulary your script is called a
book (as in runbook). You hand Dorc the book and a target machine. Dorc reads the
book, quietly interrogates the machine to find out which parts are already true, and
then shows you a plan: your own script, in its original order, with the parts that
are provably unnecessary today commented out, and a one-line reason attached to
everything that remains. You approve the plan, and Dorc applies it - meaning it runs
exactly what the plan said it would run, in exactly the order you wrote.

That is the whole product. Everything else in this documentation is detail about how
Dorc earns the right to comment a line out, because commenting out someone's
firewall command on a hunch would be much worse than no tool at all.

## The promises

Dorc's behavior is governed by a short list of promises, in strict priority order.
When two of them could conflict, the higher one wins, always.

First: planning never changes the machine. The interrogation step (Dorc calls it
probing) is read-only. Dorc will not run something during planning unless a human
being has explicitly vouched, in writing, that the specific check being run does not
mutate anything. When Dorc is not sure a check is safe to run, it does not run it -
and simply knows less about your machine as a result.

Second: Dorc never skips work it cannot prove unnecessary. The cardinal sin, in
Dorc's design, is leaving out a command that was actually needed. Every ambiguity, at
every layer, resolves toward running your line. A confused check, a timed-out probe,
a tool nobody has described - all of these mean the line stays in the plan and runs.
You will sometimes watch Dorc run things that turn out to have been unnecessary;
that is the deliberate, safe direction to be wrong in.

Third: Dorc never reorders or parallelizes your book on a machine. The order you
wrote is sacred. Dorc's speed comes entirely from leaving things out and from doing
its read-only probing in parallel up front - never from rearranging your work.

Fourth: the plan is honest. Any line that might execute is shown to you; nothing that
could touch the machine is hidden, ever. When Dorc is very sure about a line it may
present it quietly, but a line that can run is never invisible. The plan you approve
is the thing that happens; if the world shifts mid-apply, Dorc proceeds safely and
flags what diverged in its report, rather than improvising or stopping to ask you
questions it should have asked up front.

## Where the knowledge comes from

Dorc understands shell - it parses your book as a real program, follows your
variables, and understands your control flow. But it knows nothing about what the
commands themselves do. Nothing in the text of `foobar sync-certs /etc/certs` says
whether that command is a harmless status query or a disk formatter.

That knowledge comes from oracles: small files of plain shell, written by people who
know a given tool, that answer questions like "is this invocation already satisfied
on this machine?" in a form Dorc can safely run during probing. Dorc ships with a
base library of oracles for the boring famous tools, and anyone can write one for
anything else - the second half of this documentation teaches how.

The consequence you will feel as a user: Dorc's value on your book is exactly
proportional to how much of your book has been described. Lines whose tools have
good oracles can vanish from your plan when they are already satisfied. Lines whose
tools are undescribed always run, and - because an undescribed command might change
anything - they also limit what Dorc can prove about the lines after them. The plan
includes gentle hints about which single description would help your book most. You
get out what you put in; the tool is designed so that the first small effort pays
the steepest reward.

## The floor, and the off-ramp

If you ignore every feature, Dorc degrades to exactly what you were doing before:
your script runs, top to bottom, once. It is designed to never be worse than that
floor.

And if you stop using Dorc entirely, your book is still just a shell script - it
never stopped being one. Nothing in your book is rewritten into a Dorc-specific
format, there is no state database holding you hostage, and anything an oracle
author wrote can be mechanically reduced to plain portable shell. The exit was
designed before the entrance.

<!-- quoted: DESIGN.md priorities + approach; IMPLEMENTATION.md to-execute-or-not;
     spike/CLAUDE.md rul-attention-honesty, no-reorder-ever, inv-kfail,
     rul-divergence-proceed; USER_STORY.md stage 0-1 -->
