# 312b-exercises/01 — Placement parametric in an observer

> Exercise record (Fable, 2026-09-08). Not durable; takeaways live in `notes/312b`. Method: hold
> `notes/311` as the final types, push one concrete habit onto it, card only where it strains.
> Plain book-sh grounds the case; the model's response is prose in 311 vocabulary; no oracle
> spellings. Grades: +SURE / ~SUSPECT / -GUESS / --WONDER.

## The habit

```sh
pipx install httpie
sudo pipx install poddle
sudo git config --global user.email ops@example.net
. "$HOME/.nvm/nvm.sh"; nvm use 20; npm install -g pm2
```

Three tools whose state lives under the caller's home. pipx keeps apps under `~/.local/pipx`
and shims under `~/.local/bin`, overridable by `PIPX_HOME` (GOTCHAS
`an-omitted-store-breaks-invariance`). git's global config is `~/.gitconfig` or
`$XDG_CONFIG_HOME/git/config`, overridable by `GIT_CONFIG_GLOBAL`. nvm rewrites `PATH` from a
sourced shell function so `node` and `npm` reach a per-user tree.

## First glance

The pipx author describes a mKind for installed apps whose mNaturalKey is the app name.
:named-in: the pipx store, a directory. mPlacement: that store and the shim directory. Both are
File mEntities, so the author names them. `/root/.local/pipx`? `/home/alice/.local/pipx`? Every
user's? The first glance lands the whole burden on the pipx author, including which users exist
on the box and what sudo does to `HOME`. That is the wrong seat, and it is impossible.

## Where the model strains

`311` § 2.1 supplies a mNamespaceInstance three ways: FIXED (the owner names it), SITE (the
tool author's bind fills it from argv), AMBIENT (the mEntryChain's lent instance for that
mNamespaceKind). The pipx store is none of these. It is computed: `${PIPX_HOME:-$HOME/.local/pipx}`,
an sh expansion over the environment, falling back to the observer's passwd home when `HOME` is
unset. § 2.3's mPlacements have the same shape. § 2.8's mTraversal decomposes a mKey into the
mNamespaceKind's own mEntities; here the first step of the path is not a directory entry but an
environment or User resolution. Three strains, one cause: a mNamespaceInstance can be a
function of ρ and of another mKind's `resolve()`, and the model has no supply mode for that.

The nvm line is the same strain from the command-word side: `node` is a mNaturalKey in the
engine's own PATH-search mNamespaceInstance (§ 1.9's transit-free local mRoute), and a sourced file
rebinds `PATH`. The engine's mRoute claim must perish on a ρ write, not only on a mount-table
write. Today the `.` line is a blind act and everything below it is havoc, which is safe and
useless.

## The card

Unknowability class: vacant. Every piece is known to someone.

Can know at all: the tool author (the store rule, from docs or source); the wrapper author
(what sudo does to `HOME` and the environment, by default); the admin (`PIPX_HOME` if they set
it; their sudoers `env_keep` and `always_set_home`); the host (the actual value, by a read in
the denoted context); the engine (the meaning of `${X:-default}`, which is language, not a
world fact).

Can reasonably know: the tool author for the rule and nothing beyond it. The wrapper author for
the default lend. The admin for the overrides. Nobody enumerates users.

Danger, by seat. A wrong store rule mis-routes the probe, reads the wrong store, and mints a
wrong verdict at the tool's own lines in every book naming the tool: cardinal, attributed, high
leverage. An omitted mPlacement is the silent channel under the flag. A wrong `HOME` lend from
the sudo author mis-routes every `$HOME`-keyed store under every wrapped mSite everywhere: the
highest leverage in this exercise, and policy-dependent. A wrong admin override bites one
deployment.

Verification means. The rule is differentially testable on a host (pipx prints its
environment; git prints its config paths). The lend is testable in one line. Both are
mechanical, which makes both seats safer than their leverage suggests.

Transformation menu. Declare the rule as an sh expression the engine evaluates over ρ at the
mSite, unknown where ρ is unknown. Or ask the tool where its store is, in the denoted context,
as a `resolve()` owned by the tool author returning a File coordinate: costs a probe, needs the tool
to expose it, cannot be reasoned statically, but is the cheap rung and robust to overrides. Or,
for the wrapper half, ask sudo's policy and decline on surprise (the same shape as asking
apt-config for hooks).

Residue if nobody speaks. No tool oracle: wall. A rule with a `HOME` hole under a wrapper whose
author claims nothing about the environment: the instance is unknown, the mKey is unknown, guard
or run. Monotone: every added half buys its mSites and none removes a prior one.

## Leads and wants

- `lead-computed-namespace-instance`: a fourth supply mode, COMPUTED, an sh expression over ρ,
  argv, and a `resolve()` of ambient instances; SITE and AMBIENT may be its special cases. Unknown
  inputs make the instance unknown.
- `lead-ask-the-tool-for-its-store`: measure-in-context as a rung for mNamespaceInstances and
  mPlacements, owned by the tool author, returning a coordinate in another mKind. The newbie
  rung; the declared rule is the experienced rung.
- `lead-env-is-a-routing-namespace`: ρ routes command words (`PATH`), stores (`HOME`, `XDG_*`,
  `PIPX_HOME`, `GIT_CONFIG_GLOBAL`), and referents (`AWS_PROFILE`). `30S` covers pins and
  severs for verdict bodies; the routing role is wider and belongs in 311's mNamespaceKind inventory
  (`311:open-observer-namespace-inventory`).
- `lead-local-route-perishes-on-rho-writes`: the engine's own mRoute claim for command words is
  keyed by `PATH` and cwd as well as the mount table; a `PATH` write is a routing mutation.
- `lead-policy-dependent-lends-split-two-ways`: the wrapper author declares the default lend;
  the admin's seat overrides per deployment; the wrapper's oracle may read the policy and
  decline. Same split as exercise 02's sudoers case.
- `want-user-home-read-in-stdlib`: the passwd home of a User is a `resolve()` the User mKind owner
  supplies, so a tool author's rule can name "the observer's home" without owning it.

## Open

- Whether COMPUTED instances make SITE and AMBIENT redundant, or whether keeping them named
  preserves a teachable ladder.
- Whether the engine may evaluate `${X:-default}` over ρ under a wrapper without the wrapper
  author having spoken, or whether silence must make the whole expression unknown (~SUSPECT the
  latter, by `rho-claim-ladder`).
