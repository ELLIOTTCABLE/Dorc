# Dorc documentation

Dorc is a tool that runs your shell scripts on machines, and works out which parts of
them it can safely not run. You describe a machine's desired state the way you already
do - a plain POSIX shell script - and Dorc probes the machine, shows you a plan of
what would actually happen, and then applies exactly that plan. The more the tools in
your script have been described to Dorc, the more of your script it can prove
unnecessary today, and the shorter your plan gets.

There are two kinds of people in Dorc's world, and this documentation is split
accordingly. You will probably be both, at different moments of the same afternoon.

The first is the person running scripts: you have a server to fix, a laptop to set
up, a fleet to converge. You want to point Dorc at a script and get value with as
close to zero learning as possible. Start here:

- `running-books/what-dorc-does.md` - what Dorc promises, what it never does, and
  why you can trust a plan.
- `running-books/reading-a-plan.md` - how to read what Dorc shows you, and what to
  do about the lines that will not go away.

The second is the person describing tools: you know how some command actually works -
maybe you wrote it, maybe you just operate it daily - and you are willing to spend a
few minutes teaching Dorc about it, so that every script that uses it (yours today,
other people's later) gets smarter. That teaching artifact is called an oracle, it is
written in plain shell, and writing good ones is a genuine skill. This is the main
body of the documentation:

- `writing-oracles/01-what-dorc-sees.md` - the mental model: what Dorc can figure
  out on its own, and where it is blind without you.
- `writing-oracles/02-your-first-oracle.md` - the two-minute oracle that makes a
  converged line disappear.
- `writing-oracles/03-the-probe-contract.md` - the one promise you must never
  break, and the habits that keep you from breaking it by accident.
- `writing-oracles/04-naming-state.md` - kinds, entities, and selectors: how your
  facts and a stranger's facts learn to talk about the same machine.
- `writing-oracles/05-covering-a-real-tool.md` - honest breadth: declining what
  you have not modeled, refusing loudly, and judging what "converged" should mean.
- `writing-oracles/06-predicting-behavior.md` - the modeling function: describing
  what a command would do, channel by channel.
- `writing-oracles/07-footprints-and-walls.md` - describing what a command
  disturbs, and the carefully-priced feature that builds on those claims.
- `writing-oracles/08-wrappers-and-contexts.md` - sudo, chroot, env, and friends:
  describing commands that run other commands somewhere else.
- `writing-oracles/09-owning-a-kind.md` - the responsibilities that come with
  publishing a vocabulary: aliasing, reach, and where state actually lives.
- `writing-oracles/10-the-shell-dialect.md` - exactly what shell to write, and the
  defensive habits that make an oracle survive strangers' machines.
- `writing-oracles/11-authoring-with-the-engine.md` - classing your declines,
  running the linter as you write, and reading back what an admin will see.
- `writing-oracles/12-publishing-and-the-off-ramp.md` - stripping, sharing,
  ownership, and what your users can rely on.

Finally, one reference document sits apart from the learning path:

- `reference/oracle-contract.md` - the oracle contract, complete. Every obligation,
  every license, every failure mode, in one place. Read the learning path first;
  this page assumes its concepts and does not re-teach them. Come back to it while
  writing anything you intend to publish.

A note on stability: Dorc's analysis gets better over time, on purpose, so the exact
set of lines it can prove away on a given day is not a stable interface - do not
build automation that depends on a plan's exact shape. What is stable is the
contract: the promises in these pages about what Dorc will and will not do, and the
meaning of what you write in an oracle.

<!-- quoted: 276:rul-verdicts-never-stable; DESIGN.md priorities; USER_STORY.md -->
