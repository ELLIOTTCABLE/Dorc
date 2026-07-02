USER_STORY: the gradual-enhancement walkthrough
===============================================

> Document tier: AI-written, heavily user-audited (same class as KNOBS.md — every word
> human-reviewed before it counts; trumps the Research/ planning-ocean; changes rarely.)
>
> All terminal output below is ILLUSTRATIVE — the render format is not settled design. What IS
> settled, and what the renders are drawn to obey: the plan is the whole book, in original
> order, as plain sh; elided lines are present-but-commented-out; anything that will execute is
> never hidden; every surviving line carries its reason. ("rul-attention-honesty",
> `spike/CLAUDE.md`.)

The cast: one person, wearing both hats — the *admin* (writing the runbook, wants the server
fixed) and, eventually, the *engineer* (writing an oracle, wants their tool described well).
They have a small Debian VPS, and one runbook they've been SSH-piping at it for a year:

```sh
#!/bin/sh
# webhost.sh - bring up the static site
apt-get update
apt-get install -y nginx
cp ./nginx.conf /etc/nginx/nginx.conf
foobar sync-certs /etc/nginx/certs
systemctl enable --now nginx
hork tune --profile web
ufw allow 443/tcp
```

Seven commands. Five are boring, famous tools. `foobar` is their own little cert-distribution
tool — sane, scriptable, just theirs; nobody has ever written an oracle for it. `hork` is a
proprietary vendor tuning daemon-poker that nobody understands, including the vendor.

The story below re-runs this same book at each stage, on the same host, in two world-states:
the *steady state* (host already converged — the overwhelmingly common case in real ops) and
the occasional *drifted* day. Watch three currencies: what the run MUTATES, what it COSTS
(wall-clock), and what it demands of the user's ATTENTION.


Stage 0 — day zero: install, first contact
------------------------------------------

```
$ apt-get install dorc
$ dorc apply webhost.sh web1.example.net
```

The floor promise: this is *no worse than what they already did*. The book runs, top to
bottom, in order, once. Dorc will never reorder or parallelize within a host; the book's
order is sacred.

But even un-enriched, `dorc plan` already exists:

```
$ dorc plan webhost.sh web1.example.net
apt-get update                                   # runs
apt-get install -y nginx                         # runs
cp ./nginx.conf /etc/nginx/nginx.conf            # runs
foobar sync-certs /etc/nginx/certs               # runs: unmodeled ('foobar')
systemctl enable --now nginx                     # runs
hork tune --profile web                          # runs: unmodeled ('hork')
ufw allow 443/tcp                                # runs
plan: 7 run, 0 verify, 0 elided
```

Nothing was probed (nothing could be: probing requires an oracle to vouch that a check is
safe to run in the read-only probe phase, and no amount of cleverness substitutes for that
vouch). The plan is just the book, annotated. Spent: nothing. Gained: nothing yet, minus a
few seconds of analysis. That asymmetry — you get out what you put in — is the whole deal.

- Gained: a plan surface, and a place for hints to accrue.
- Lost: nothing. The off-ramp is intact: the book is still just a script.


Stage 1 — the base library: the famous half
-------------------------------------------

Dorc ships with a small base library of oracles for the boring famous things — the
`org.dorc.*` bootstrap vocabulary: dpkg/apt, coreutils, and friends. With those loaded, the
probe phase has something to do. `dorc plan` now ships each covered oracle's own *check* —
its read-only convergence probe, stripped of annotations, byte-for-byte the author's sh — to
the host, in parallel, and folds the results into the plan.

Steady state (nginx installed, config already correct, index fresh):

```
$ dorc plan webhost.sh web1.example.net
# apt-get update                                 # converged: apt.Cache:.fresh
# apt-get install -y nginx                       # converged: apt.Package:nginx.installed
# cp ./nginx.conf /etc/nginx/nginx.conf          # converged: content match
foobar sync-certs /etc/nginx/certs               # runs: unmodeled ('foobar')
systemctl enable --now nginx                     # runs: unmodeled ('systemctl')
hork tune --profile web                          # runs: unmodeled ('hork')
ufw allow 443/tcp                                # runs: unmodeled ('ufw')
plan: 4 run, 0 verify, 3 elided
```

The top three lines are *elided*: commented out, literally removed from what will execute,
because the probe proved them already-true on this host and their oracles explicitly vouch
that proven-converged means safe-to-not-run. (That vouch is an authored judgment, not an
inference; more on it at stage 3.) Note `apt.Cache:.fresh` — the package index is a
*singleton* piece of state, one instance in the world, so its property hangs on an empty
entity slot.

What did NOT happen is as important. `foobar` is the first opaque command — the first
*poison wall*. Dorc knows nothing about what it touches, so no probe result from above the
wall can be trusted to still hold below it; if `systemctl` had an oracle, its site still
could not statically elide, because it sits downstream of a command that might invalidate
anything. And with no oracles at all for the bottom four, they simply run.

- Spent: `apt-get install dorc`, effectively.
- Gained, steady state: three mutation-capable commands provably not run (the `cp` and the
  `apt-get`s can no longer surprise); a few seconds of wall-clock; three lines of attention.
- Gained, drifted day: if the config file has been fiddled, the `cp` line comes back into the
  plan on its own — the plan is a function of the probed world, not of hope.
- Still lost: the bottom four lines run blind, every single time, exactly as before Dorc.


Stage 2 — the community library: guards appear
----------------------------------------------

The plan has been nagging, politely, with attribution and counts:

```
hint: 'foobar' (line 4) is unmodeled: it degrades 3 downstream sites; an oracle for it
      would recover them whenever its state is converged
hint: 'systemctl', 'ufw' are covered by library 'debian-service-essentials'
```

So they install the library. Now `systemctl` and `ufw` have quality oracles — probes,
convergence vouches, the works. Steady state:

```
$ dorc plan webhost.sh web1.example.net
# apt-get update                                 # converged: apt.Cache:.fresh
# apt-get install -y nginx                       # converged: apt.Package:nginx.installed
# cp ./nginx.conf /etc/nginx/nginx.conf          # converged: content match
foobar sync-certs /etc/nginx/certs               # runs: unmodeled ('foobar')
( systemctl_check enable --now nginx ) \
   || systemctl enable --now nginx               # verify: converged, but past 'foobar' (line 4)
hork tune --profile web                          # runs: unmodeled ('hork')
( ufw_check allow 443/tcp ) \
   || ufw allow 443/tcp                          # verify: converged, but past 'hork' (line 6)
plan: 2 run, 2 verify, 3 elided
```

Two new things on screen, and they are the crux of the whole design.

The `systemctl` site probed converged — but it sits below the `foobar` wall, so that fact
may be stale by the time the apply reaches it. Instead of eliding (forbidden) or running
blind (wasteful, and mutation-risky), the plan *inserts a guard*: the oracle's own check,
in-sequence, immediately in front of the untouched original command. At apply time —
*after* `foobar` has done whatever it does — the check re-verifies convergence live. If it
holds, the command short-circuits; if not (or if the check itself fails or can't tell), the
original bytes run, exactly as written. An in-sequence check has no staleness problem *by
construction*: everything above it has already happened.

Read the guard's anatomy once, closely, because everything rides on it:

- The check body is the *oracle author's own sh*, shipped with only its annotations
  stripped — the same bytes the probe phase already ran. Dorc never synthesizes shell; there
  is always a human author to point at.
- The `check || command` shape means a broken or confused check *falls through to running
  the command* — failure lands on the safe side, and an errexit book survives a failing
  guard (a `||`-left is exempt).
- The subshell-wrapped invocation keeps the check's variables from leaking into the book's.
- It still reads as the check-then-execute idiom a diligent human writes by hand, and the
  rendered plan is still just a runnable script — the off-ramp holds.

The costs, stated plainly. First, the *check-tax*: those two sites now pay a read on every
apply, forever, instead of the elision they'd earn in a described world. Second — and this
is the deeper one — *attention is not saved*. Both lines are still on screen, still in the
approval, still in mindshare. A guard makes the book fast and safe; it does not make it
shorter. Only proof does that, and past an opaque command there is no proof to be had.

- Spent: one library install.
- Gained, steady state: the last two well-known tools stop blind-running (ufw's mutation, a
  service-manager poke — both now avoided when truly converged); drift below the walls is
  caught at the last moment instead of causing a wrong non-run.
- Gained, always: monotonicity. Note what installing this library could NOT do: hurt
  anything. An oracle's claims license behaviour at its own tool's sites only; adding one
  never endangers someone else's line. Silence licenses nothing; more description is only
  ever upgrade.
- Not gained: a single line of attention. Four lines still stare back, two of them
  wall-formers.


Stage 3 — two minutes of engineering: the minimal foobar oracle
---------------------------------------------------------------

The hint has sharpened (it knows the topology now):

```
hint: 'foobar' (line 4) is unmodeled: it is the first wall - an oracle vouching its
      convergence would elide it when converged, and un-wall 1 downstream site
```

Annoyed, our admin puts on the engineer hat for exactly the length of a coffee. `foobar`
already has a status query (most tools do). They append to the book's own file — oracles and
runbooks can share a file:

```sh
foobar.check() {
   verb="$1"; shift
   case "$verb" in
   sync-certs)
      dest : foobar.Certs = "$1"
      foobar status --certs-current -- "$dest"   : foobar.Certs:"$dest".synced
      : foobar:sync-certs~   # vouch: synced certs = nothing worth re-running (STRAWMAN spelling)
      ;;
   esac
}
```

Eight lines, one verb, one probe, one vouch. In order:

- `foobar.check()` declares "this body is the oracle for `foobar`". The period-name is the
  opt-in semaphore; *stripped*, it is a plain `foobar_check()` function any shell can run.
- `dest : foobar.Certs = "$1"` binds the operand as the entity, in a kind this author just
  minted. Nobody approves kind names; there is no registry. It only has to agree with
  itself.
- The trailing `: foobar.Certs:"$dest".synced` says: this probe's exit code *establishes*
  that property. The engine never interprets what "synced" means — it is an opaque value
  bound to the author's probe.
- The `~` vouch is the license, and it is a *judgment*, not a fact: "when this arm's probes
  hold, I judge re-running this to be noise." The plan attributes every elision and guard
  to the vouch that licensed it, by name; when a vouch is wrong, there is a person to be
  wrong. Its exact spelling is still strawman-tier design.

Steady state, after two minutes of work:

```
$ dorc plan webhost.sh web1.example.net
# apt-get update                                 # converged: apt.Cache:.fresh
# apt-get install -y nginx                       # converged: apt.Package:nginx.installed
# cp ./nginx.conf /etc/nginx/nginx.conf          # converged: content match
# foobar sync-certs /etc/nginx/certs             # converged: foobar.Certs:/etc/nginx/certs.synced
# systemctl enable --now nginx                   # converged: service enabled+active
hork tune --profile web                          # runs: unmodeled ('hork')
( ufw_check allow 443/tcp ) \
   || ufw allow 443/tcp                          # verify: converged, but past 'hork' (line 6)
plan: 1 run, 1 verify, 5 elided
```

Two separate things just happened, and the second is the one people miss:

1. `foobar`'s own line elides when converged. That's the direct purchase.
2. The `systemctl` line — *untouched since stage 2* — upgraded from guard to full elision.
   Because **an elided command casts no wall**: a command that will not run cannot
   invalidate anything, so the wall at line 4 simply is not there on a converged day. The
   two-minute oracle didn't just buy its own line; it bought back every downstream fact it
   had been poisoning. This is why the minimal oracle is the steepest part of the
   value-curve, and why the hint machinery pushes it first.

On a drifted day (certs actually stale), the same plan honestly re-degrades: `foobar` comes
back as a run, and `systemctl` falls back to its guard *for that apply* — the wall is real
again, because `foobar` will really act. The plan is per-world-state; the promise ("plan
mirrors what apply will do; divergence discovered mid-apply is proceed-and-flag, never a
mid-apply question") is not.

And a plan-time can't-tell — probe timeout, weird rc — is not quietly rounded to converged:
no verdict, no guard, no elision; the site runs. Everything fails toward run.

- Spent: ~2 minutes, 8 lines of sh, zero new languages, zero config formats.
- Gained, steady state: 7 lines of attention down to 2; foobar's re-sync mutation avoided.
- Gained, structurally: the certs state is now *described* — future books that touch it
  inherit the coverage for free.
- Not gained: anything about `hork`. Walls fall one tool at a time, each by its own author.


Stage 4 — the battle-ready oracle: breadth, honesty, publication
----------------------------------------------------------------

Weeks later, with time to spare, the engineer hat comes back on — not to improve this book's
steady-state plan (it is already two lines; there is nothing left to buy here), but to make
the oracle *worth publishing*: correct for colleagues' books, other verbs, other days, other
argv shapes. The enriched oracle:

```sh
foobar.check() {
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew)
      dest : foobar.Certs = "$1"
      [ "$2" = "" ] || { printf 'UNK multi-operand foobar\n' >>"$DORC_REPORT"; exit 254; }
      foobar status --certs-current -- "$dest"   : foobar.Certs:"$dest".synced
      : foobar:sync-certs~
      : foobar:renew~
      ;;
   purge-certs)
      dest : foobar.Certs = "$1"
      foobar status --certs-current -- "$dest"   : foobar.Certs:"$dest".synced!
      ;;
   *) printf 'UNK unmodeled foobar verb: %s\n' "$verb" >>"$DORC_REPORT"; exit 254 ;;
   esac
}
```

What each addition buys — and refuses:

- Verb breadth: `renew` shares the probe and earns its own vouch; a colleague's
  `foobar renew /srv/certs` site now guards-or-elides in *their* book.
- `purge-certs` reads the same probe *inverted* (`!`): exit-0 means certs present, which for
  a purge means not-converged. The `!` is pure exit-code plumbing — the engine has no notion
  of "removal", only opaque values. And note the author's own asymmetric judgment, expressed
  the only place it belongs — in what they vouch: sync-certs converged is vouched skippable;
  purge-certs deliberately carries *no* vouch (stale residue makes "looks absent" a bad
  reason to not-run a purge, and the author knows it). No vouch, no guard, no elision: purge
  sites always run. The engine did not decide that; the person who knows the tool did.
- The arity gate: a two-operand invocation hits the loud `UNK` refusal instead of a probe
  that quietly checked only the first operand. Refusal exits carry the report out-of-band;
  the site just runs, with a reason in the plan.
- The `*` arm: an unknown verb claims nothing, vouches nothing, licenses nothing — a
  colleague's `foobar frobnicate` is exactly as safe as it was with no oracle at all, plus a
  breadcrumb in the report.
- Still just sh: stripped, it runs on any POSIX box with no Dorc in sight. Publishing it is
  pushing a file to a repo. Adopting it is downloading one.

- Spent: an hour, maybe, plus ownership — a published vouch is a standing judgment with
  their name on it, and the attribution machinery will cite it.
- Gained: every `foobar` site in every book on every host they (or anyone) run, forever;
  honest refusals at the edges instead of quiet wrongness.
- Explicitly not gained, and never will be by this route: `hork`.


The residue, and the honest product statement
---------------------------------------------

`hork` never gets an oracle. The vendor won't document it; nobody sane will vouch for it.
So line 6 runs on every apply until the end of time, and the `ufw` line behind it verifies
rather than elides, forever. Past the last wall the product statement is exactly this: *Dorc
narrows your attention only where the world is described; elsewhere it makes your book fast
and safe, but not shorter.* The plan's honesty about that — nothing hidden, every surviving
line carrying its reason, every reason naming its wall — is itself the feature. A tool that
commented out `ufw allow 443/tcp` on vibes would be worse than no tool.

The final ledger, steady state on the same seven-line book:

```
stage    ran   verified   elided   attention-lines   spent
0        7     0          0        7                 nothing
1        4     0          3        4                 nothing (bundled)
2        2     2          3        4                 a library install
3        1     1          5        2                 2 minutes of sh
4        1     1          5        2                 an hour, for everyone else's benefit
```

TODO/UNFILLED: the propagation frontier. Everything above elides *around* walls by removing
them (stage 3) or verifies *behind* them (stage 2). Whether facts can ever statically
survive a wall that stays — "this running command provably cannot touch that state" — is the
footprint/disjointness tier: real design-in-progress, deliberately not described here, and
nothing in this document depends on its outcome.
