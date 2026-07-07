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
set -eu
CERTS=/etc/nginx/certs
apt-get update
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx
cp ./nginx.conf /etc/nginx/nginx.conf
foobar sync-certs "$CERTS"
systemctl enable --now nginx
hork tune --profile web >>/var/log/hork.log 2>&1
ufw allow 443/tcp
```

A year of accreted habit shows: strict-mode at the top; the classic hand-written
idempotence-guard on the nginx install (they got burned once); the noisy vendor tool's
output banished to a logfile. Seven tool-commands, plus two lines of plain housekeeping.
Five of the tools are boring and famous. `foobar` is their own little cert-distribution
tool — sane, scriptable, just theirs; nobody has ever written an oracle for it. `hork` is a
proprietary vendor tuning daemon-poker that nobody understands, including the vendor.

The story below re-runs this same book at each stage, on the same host, in two world-states:
the *steady state* (host already converged — the overwhelmingly common case in real ops) and
the occasional *drifted* day. Watch three currencies: what the run MUTATES, what it COSTS
(wall-clock), and what it demands of the user's ATTENTION.


Stage 0 — a thought-experiment: Dorc with its standard library disabled
------------------------------------------------------------------------

```
$ apt-get install dorc
$ dorc apply webhost.sh web1.example.net
```

The floor promise: this is *no worse than what they already did*. The book runs, top to
bottom, in order, once. Dorc will never reorder or parallelize within a host; the book's
order is sacred.

This stage as a whole, though, is an illustrative impossibility: the base oracle library
ships *with* the tool, so real day-one behaviour is stage 1 below. Pretend for a moment it
didn't — because the bare mechanism is worth seeing once:

```
$ dorc plan --verbose webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  apt-get update                                     # runs
 6  dpkg -s nginx >/dev/null 2>&1 \
 6     || apt-get install -y nginx                     # runs
 7  cp ./nginx.conf /etc/nginx/nginx.conf              # runs
 8  foobar sync-certs "$CERTS"                         # runs: unmodeled ('foobar')
 9  systemctl enable --now nginx                       # runs
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ufw allow 443/tcp                                  # runs
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
`sm.dorc.*` bootstrap vocabulary (FIXME: intentionally-invalid TLD, so strawman names cannot
leak into reality): dpkg/apt, coreutils, and friends. With those loaded, the probe phase has
something to do. `dorc plan` now ships each covered oracle's own *check* — its read-only
convergence probe, stripped of annotations, byte-for-byte the author's sh — to the host, in
parallel, and folds the results into the plan.

Steady state (nginx installed, config already correct, index fresh):

```
$ dorc plan --verbose webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  # apt-get update                                   # converged: package index fresh
 6  # dpkg -s nginx >/dev/null 2>&1 \
 6  #    || apt-get install -y nginx                   # converged: your guard holds (dpkg -s rc 0)
 7  # cp ./nginx.conf /etc/nginx/nginx.conf            # converged: content match
 8  foobar sync-certs "$CERTS"                         # runs: unmodeled ('foobar')
 9  systemctl enable --now nginx                       # runs: unmodeled ('systemctl')
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ufw allow 443/tcp                                  # runs: unmodeled ('ufw')
plan: 4 run, 0 verify, 3 elided
```

The top three sites are *elided*: commented out, literally removed from what will execute,
because the probe proved them already-true on this host and their oracles explicitly vouch
that proven-converged means safe-to-not-run. (That vouch is an authored judgment, not an
inference; more on it at stage 3.)

Line 6 deserves a pause: that's the admin's own, hand-written, pre-Dorc idempotence guard —
and it is *exactly the shape the analyzer lifts*. The condition is a read the base library
vouches safe to probe; its measured exit code proves the fallback branch dead on this host;
the whole line goes. Years of defensive habit turn out to have been hand-written oracle
material all along — that continuity is the entire bet of the tool.

Critically to the value-proposition, without the illustrative `--verbose`, Dorc
knows enough to *not even show those first lines to the user most of the time.*
User-attention is conserved, safety is preserved.

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
- Still not gained: the bottom four sites run blind, every single time, exactly as before
  Dorc.


Stage 2 — more coverage, below a wall: guards appear
----------------------------------------------------

The plan has been nagging, politely, with attribution and counts:

```
hint: 'foobar' (line 8) is unmodeled: it degrades 3 downstream sites; an oracle for it
      would recover them whenever its state is converged
hint: 'systemctl', 'ufw' are covered by library 'debian-service-essentials'
```

So they install the library. Now `systemctl` and `ufw` have quality oracles — probes,
convergence vouches, the works, every bit as good as the base library's. Steady state:

```
$ dorc plan --verbose webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  # apt-get update                                   # converged: package index fresh
 6  # dpkg -s nginx >/dev/null 2>&1 \
 6  #    || apt-get install -y nginx                   # converged: your guard holds (dpkg -s rc 0)
 7  # cp ./nginx.conf /etc/nginx/nginx.conf            # converged: content match
 8  foobar sync-certs "$CERTS"                         # runs: unmodeled ('foobar')
 9  ( systemctl_check enable --now nginx ) \
 9     || systemctl enable --now nginx                 # verify: converged, but past 'foobar' (line 8)
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 2 run, 2 verify, 3 elided
```

Two new things on screen, and they are the crux of the whole design. Note first what they
are *not*: the new sites arrived as guards because of their *position* — modeled-but-below
an unmodeled command — not because library oracles are somehow lesser. Had `foobar` sat
last, both would have elided like the top three.

The `systemctl` site probed converged — but it sits below the `foobar` wall, so that fact
may be stale by the time the apply reaches it. Instead of eliding (forbidden) or running
blind (wasteful, and mutation-risky), the plan *inserts a guard*: the oracle's own check,
in-sequence, immediately in front of the untouched original command. At runtime —
*after* `foobar` has done whatever it does — the check re-verifies convergence live. If it
holds, the command short-circuits; if not (or if the check itself fails or can't tell), the
original bytes run, exactly as written. An in-sequence check has no staleness problem *by
construction*: everything above it has already happened.

Read the guard's anatomy once, closely, because everything rides on it:

- The predict body is the *oracle author's own sh*, shipped with only its annotations
  stripped — the same bytes the probe phase already ran. Dorc never synthesizes shell; there
  is always a human author to point at.
- The `check || command` shape means a broken or confused check *falls through to running
  the command* — failure lands on the safe side. And it composes with the book's own habits:
  the `set -eu` on line 3 is survived by design, because an `||`-left is errexit-exempt.
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
hint: 'foobar' (line 8) is unmodeled: it is the first wall - an oracle vouching its
      convergence would elide it when converged, and un-wall 1 downstream site
```

Annoyed, our admin puts on the engineer hat for exactly the length of a coffee. `foobar`
already has a status query (most tools do). They append to the book's own file — oracles and
runbooks can share a file:

```sh
foobar.is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs)
      dest : org.foob.Certs = "$1"
      foobar status --certs-current -- "$dest"   : org.foob.Certs:"$dest".synced
      ;;
   *) return 2 ;;
   esac
}
```

Nine lines, one verb, one probe — and the function's *name* is the license. In order:

- `foobar.is_converged()` declares "this body answers, for `foobar` invocations, the
  question in its name." The period-name is the opt-in semaphore; *stripped*, it is a plain
  `foobar_is_converged()` any shell can run. And the name is a *contract*: by writing a
  function that answers "is it converged," the author licenses Dorc to act on its yes — so
  its yes must mean "re-running this is noise I accept," not merely "some state holds." (A
  dpkg-'installed' package with an upgrade pending is exactly the gap: whether that counts
  as a yes is the author's judgment, and only theirs.) The plan attributes every elision
  and guard to the function that answered, by name; when the answer is wrong, there is a
  person to be wrong. (Spelling settled 2026-07-03 — authoring the verdict-function IS the
  vouching act; no separate vouch syllable exists.)
- `dest : org.foob.Certs = "$1"` binds the operand as the entity, in a kind this author just
  minted. Nobody approves kind names; there is no registry. It only has to agree with
  itself. (At the call-site that operand was `"$CERTS"` — the analyzer resolves plain
  variable-flow to the constant before binding; ordinary shell habits don't defeat it.)
- The trailing `: org.foob.Certs:"$dest".synced` says: this probe's exit code *establishes*
  that property. The engine never interprets what "synced" means — it is an opaque value
  bound to the author's probe.
- `*) return 2` is the native *decline*. The exit-status partition is fixed and blessed:
  0 = the named sense holds; 1 = its complement; anything ≥2 = "can't say," and can't-say
  always runs. Paths the author won't answer for simply answer 2 — declining is ordinary
  control-flow, not an annotation.

Steady state, after two minutes of work:

```
$ dorc plan --verbose webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  # apt-get update                                   # converged: package index fresh
 6  # dpkg -s nginx >/dev/null 2>&1 \
 6  #    || apt-get install -y nginx                   # converged: your guard holds (dpkg -s rc 0)
 7  # cp ./nginx.conf /etc/nginx/nginx.conf            # converged: content match
 8  # foobar sync-certs "$CERTS"                       # converged: org.foob.Certs:/etc/nginx/certs.synced
 9  # systemctl enable --now nginx                     # converged: service enabled+active
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 1 run, 1 verify, 5 elided
```

Two separate things just happened, and the second is the one people miss:

1. `foobar`'s own line elides when converged. That's the direct purchase.
2. The `systemctl` line — *untouched since stage 2* — upgraded from guard to full elision.
   Because **an elided command casts no wall**: a command that will not run cannot
   invalidate anything, so the wall at line 8 simply is not there on a converged day. The
   two-minute oracle didn't just buy its own line; it bought back every downstream fact it
   had been poisoning. This is why the minimal oracle is the steepest part of the
   value-curve, and why the hint machinery pushes it first.

On a drifted day — certs actually stale — the same plan honestly re-degrades:

```
$ dorc plan --verbose webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  # apt-get update                                   # converged: package index fresh
 6  # dpkg -s nginx >/dev/null 2>&1 \
 6  #    || apt-get install -y nginx                   # converged: your guard holds (dpkg -s rc 0)
 7  # cp ./nginx.conf /etc/nginx/nginx.conf            # converged: content match
 8  foobar sync-certs "$CERTS"                         # runs: diverged (org.foob.Certs not synced)
 9  ( systemctl_check enable --now nginx ) \
 9     || systemctl enable --now nginx                 # verify: converged, but past 'foobar' (line 8)
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 2 run, 2 verify, 3 elided
```

`foobar` comes back as a run, and `systemctl` falls back to its guard *for that apply* —
the wall is real again, because `foobar` will really act. The plan is per-world-state; the
promise ("plan mirrors what apply will do; divergence discovered mid-apply is
proceed-and-flag, never a mid-apply question") is not.

And a plan-time can't-tell — probe timeout, weird rc — is not quietly rounded to converged:
no verdict, no guard, no elision; the site runs. Everything fails toward run.

- Spent: ~15 minutes skimming docs, nine lines of sh, zero new languages, zero config formats.
- Gained, steady state: seven tool-sites of attention down to two; foobar's re-sync mutation
  avoided.
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
foobar.is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew)
      dest : org.foob.Certs = "$1"
      [ "$2" = "" ] || { printf 'UNK multi-operand foobar\n' >>"$DORC_REPORT"; return 2; }
      foobar status --certs-current -- "$dest"   : org.foob.Certs:"$dest".synced
      ;;
   purge-certs) return 2 ;;
   *) printf 'UNK unmodeled foobar verb: %s\n' "$verb" >>"$DORC_REPORT"; return 2 ;;
   esac
}
```

What each addition buys — and refuses:

- Verb breadth: `renew` shares the arm, so the author's yes now covers it too; a colleague's
  `foobar renew /srv/certs` site guards-or-elides in *their* book.
- `purge-certs` deliberately answers 2 — the author's asymmetric judgment, expressed as
  ordinary control-flow: stale residue makes "looks absent" a bad reason to not-run a purge,
  and the author knows it. Can't-say, so purge sites always run. The engine did not decide
  that; the person who knows the tool did, by declining to answer. (If they ever want the
  purge-side *fact* measured anyway — for the plan's display, not as a license — that
  belongs in the describing sibling, `foobar.predict()`: the lane that states what is true
  and predicts what would happen, and never licenses skipping a mutation.)
- The arity gate: a two-operand invocation hits the loud `UNK` refusal instead of a probe
  that quietly checked only the first operand — and a refusal is just an answer-2 with a
  breadcrumb: the report goes out-of-band, the site runs, the plan carries the reason.
- The `*` arm: an unknown verb claims nothing, answers nothing, licenses nothing — a
  colleague's `foobar frobnicate` is exactly as safe as it was with no oracle at all, plus a
  breadcrumb in the report.
- Still just sh: stripped, it runs on any POSIX box with no Dorc in sight. Publishing it is
  pushing a file to a repo. Adopting it is downloading one.

- Spent: an hour or two, maybe, plus ownership — a published vouch is a standing judgment
  with their name on it, and the attribution machinery will cite it.
- Gained: every `foobar` site in every book on every host they (or anyone) run, forever;
  honest refusals at the edges instead of quiet wrongness.
- Explicitly not gained, and never will be by this route: `hork`.


Stage 5 — the footprint: facts surviving a wall that stays
----------------------------------------------------------

> (FIXME, updated 2026-07-04: the *mechanism* below is now BUILT — the round-24 spike
> carries authored footprints, probe-time derivation for payload-bound tools, and the
> differential nets behind them. The *spelling* remains strawman. One change already known
> to be owed: `touches()` bodies below emit stringly-typed `kind:entity` lines — the
> kind-half is due to migrate to stage 7's annotation-typed emission, and the in-band
> prefix (with its `| sed 's|^|kind:|'` dressing) should not be imitated.)

Everything so far elides around walls by *removing* them (stage 3: a converged wall is an
elided wall, and an elided command casts no wall) or verifies *behind* them (stage 2:
guards). One pain is left, and a drifted morning shows it. Say the package index has gone
stale overnight — nothing else; every other fact on the host still holds. Line 5 is now
*really going to run*, and an honest wall is a wall:

```
$ dorc plan --verbose webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  apt-get update                                     # runs: diverged (index stale)
 6  dpkg -s nginx >/dev/null 2>&1 \
 6     || apt-get install -y nginx                     # runs: your own guard re-checks live (past line 5)
 7  ( file_check /etc/nginx/nginx.conf ./nginx.conf ) \
 7     || cp ./nginx.conf /etc/nginx/nginx.conf        # verify: converged, but past 'apt-get update' (line 5)
 8  ( foobar_check sync-certs "$CERTS" ) \
 8     || foobar sync-certs "$CERTS"                   # verify: converged, but past 'apt-get update' (line 5)
 9  ( systemctl_check enable --now nginx ) \
 9     || systemctl enable --now nginx                 # verify: converged, but past 'apt-get update' (line 5)
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 3 run, 3 verify, 0 elided
```

One stale index cost the entire book its shape. And the frustrating part: *everyone
watching knows* `apt-get update` doesn't touch nginx's config, or foobar's certs, or the
service state. Everyone except Dorc — because knowing that is a claim about what a black-box
binary touches, and silence licenses nothing. Somebody who knows the tool has to say it.

The spelling being mocked here: a third role-sibling, next to `predict()` and
`is_converged()`, that *answers the touch question as runnable sh*. The base library's apt
oracle grows one; so does foobar's, one line in the author's stage-4 file:

```sh
# FIXME: the `kind:entity` line-format here is stringly-typed; migrating to the stage-7
# annotation-typed emission (kind as a trailing mark, lines as raw entities).
apt-get.touches() {                          # base library (STRAWMAN spelling)
   while [ "${1#-}" != "$1" ]; do shift; done
   verb="$1"; shift
   case "$verb" in
   update) printf 'pkgindex:\n' ;;
   esac
}

foobar.touches() {                           # appended by foobar's author (STRAWMAN spelling)
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew) printf 'org.foob.Certs:%s\n' "$1" ;;
   esac
}
```

Read it the way the analyzer does. Invoked with a site's argv (same contract as its
siblings), the body *emits the entity-coordinates this verb mutates, one per line* — and by
emitting anything at all for a matched verb, the author claims **at most these** ("whatever
else this touches is residue I answer for"). An unmatched verb emits nothing: no claim, no
license, the wall stands — silence stays safe. Stripped, it is a plain function any shell
can run; `foobar_touches sync-certs /srv/certs` printing `org.foob.Certs:/srv/certs` is
documentation that executes.

The engine's move is then mechanical, and old as compilers: every probed fact already knows
*where its own truth lives* — nothing new to author, a fact's backing simply *is* what its
probe reads (`dpkg -s` reads the dpkg database; `systemctl is-enabled` reads unit state; it
cannot claim more than that by construction). A running wall's footprint gets intersected
against each downstream fact's backing. Empty intersection ⇒ the fact provably survives the
wall ⇒ its elision stands *even though the wall runs*. Non-empty, or no footprint ⇒ exactly
the stage-2 world: guard or run. (This is the separation-logic frame rule wearing work
clothes; the emitted-at-probe-time variant — stage-4 tools like `apt-get install`, whose
real file-payload only the host knows, answer by *asking the tool* inside `touches()` — is
what the literature calls a dynamic frame.)

One more thing before the payoff, because it is deliberate and permanent: none of this is
on by default. Surviving a wall means trusting authors' at-most claims with no runtime net,
so the whole tier sits behind an explicit flag — spelled out here as `--trust-footprints`
(STRAWMAN name). Without it, the plan above is what you get: honest walls, guards, runs.
And an honesty note that must outlive every future edit of this document: this opt-in is
marketing at best (you chose the danger; it isn't a Dorc bug when an author's claim is
wrong) and theatre at worst (it is desirable enough that nearly everyone will turn it on and
forget it). It exists anyway — the choice should be typed by the person who owns the
consequences, even when everyone types it.

The same stale-index morning, with footprints shipped and the trust typed:

```
$ dorc plan --verbose --trust-footprints webhost.sh web1.example.net
 1  #!/bin/sh
 2  # webhost.sh - bring up the static site
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  apt-get update                                     # runs: diverged (index stale)
 6  # dpkg -s nginx >/dev/null 2>&1 \
 6  #    || apt-get install -y nginx                   # converged: guard holds; survives line 5 (footprint disjoint)
 7  # cp ./nginx.conf /etc/nginx/nginx.conf            # converged: content match; survives line 5 (footprint disjoint)
 8  # foobar sync-certs "$CERTS"                       # converged: certs synced; survives line 5 (footprint disjoint)
 9  # systemctl enable --now nginx                     # converged: enabled+active; survives line 5 (footprint disjoint)
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 2 run, 1 verify, 5 elided
```

The book keeps its steady-state shape on a drifted day. `update`'s footprint is the package
index; the install's guard reads the dpkg database, the `cp`'s fact lives in a config file's
content, foobar's in its certs, the service's in unit state — all disjoint, all survive. And
the honest cells stay honest: on a *foobar*-drifted day, `systemctl` now stays elided too
(`org.foob.Certs` doesn't intersect service state) — but a hypothetical line below foobar whose
fact *lives in those same certs* would correctly stay guarded, footprint or no. `hork` has
no author, so it has no footprint, and no amount of machinery changes line 10 or 11 —
silence is a wall, forever.

Now the price, and it is the sharpest one in the whole design. A vouch (stage 3) that is
wrong endangers *its own tool's line*. A footprint that is wrong — an author who forgot
that `sync-certs` also rewrites a systemd unit — silently under-executes *someone else's
line*: the elision it wrongly licensed belonged to a different tool, a different author, a
different file. There is no runtime net under a survived elision; that is what "survives"
means. This is the one place Dorc ships a naked human promise, and the design treats it
accordingly: the footprint is opt-in (no oracle is required to grow one), scoped to the
author's own attended substrate, attributed by name in every elision it licenses (the
`why`-lens will say whose footprint you trusted), and priced at the professed horizon —
"past here, you are trusting authors' at-most claims." Attention saved on drifted days is
bought with exactly that trust, and with nothing else.

- Spent: one `touches()` arm per verb an author is willing to answer for; one typed flag
  per invocation from the admin who owns the consequences.
- Gained, drifted days: the book stops collapsing below the first thing that really runs;
  early-book churn (index refreshes, log rotations, cache warms) stops taxing every line
  after it.
- Gained, steady state: nothing. (Worth saying twice: on a fully-converged host the walls
  were already elided away. The footprint tier buys back the *drifted* days.)
- Not gained: `hork`, ever; and nothing below it.


> (Stages 6 and 7 are the rarefied end of the curve: kind-OWNER features. Fewer than one
> author in ten will ever write one — they exist for the handful of people who own a
> vocabulary (the apt oracle's maintainer; whoever ships the fs stdlib) — but each one
> written pays out across every book and every oracle that names that kind. Narrow base,
> high community effect. Mechanisms landing in the round-24/25 builds; spellings strawman.)


Stage 6 — the kind-owner: two names, one thing
----------------------------------------------

The footprint machinery compares coordinates by *name*, and names lie. On a real Debian
host, `nginx` and `nginx-full` can be one package under two names (provides); two paths can
be one file through a symlink. Watch it fail:

```sh
apt-get install -y nginx        # runs — a wall; its footprint says package:nginx
...
dpkg -s nginx-full >/dev/null 2>&1 || apt-get install -y nginx-full
```

The second line's fact is backed by `package:nginx-full`; the wall's footprint says
`package:nginx`; different strings ⇒ "disjoint" ⇒ the line elides *past a wall that really
touched its referent*. Every other gap in this machinery fails toward running too much;
this one under-executes, silently. It is the one place a name must be more than a string.

The fix is one function, written by the kind's owner — the party who holds what "the same
entity" means for that vocabulary:

```sh
package.resolve() {                          # the package kind's owner (STRAWMAN spelling)
   dpkg-query -W -f '${Package}\n' -- "$1" 2>/dev/null || printf '%s\n' "$1"
}
```

- Keyed by the KIND, not by a command — identity belongs to the noun-space. One resolver
  per kind; a second declaration is refused, loudly.
- The engine canonicalizes BOTH sides of every intersection through it — footprints and
  backings — so two names for one referent now collide (the line above correctly runs),
  while genuinely-different entities stay disjoint (the value survives).
- A name the resolver can't answer for is treated as may-collide ⇒ that site runs. A kind
  with no resolver keeps plain name-comparison — today's floor, nothing revoked.
- The sharp edge, honestly: a resolver that wrongly MERGES two entities only over-verifies;
  one that wrongly SPLITS one referent re-opens the silent skip. Same knife-tier as the
  footprint, and every survival it licensed cites it by name.

- Spent: a handful of lines, once, by the one author who owns the kind.
- Gained: every book, every oracle, every footprint naming that kind stops being fooled by
  aliases — including ones written by people who never heard of the resolver.


Stage 7 — reach: what touching an entity drags with it
------------------------------------------------------

> (FIXME: the freshest design — the round-24 part-B build, mechanism settled in dialogue;
> the spelling below is the strawman under construction.)

One gap is left, and it is not the owner's — it is everyone else's. A colleague's oracle
for some package-fiddling tool honestly declares:

```sh
hork.touches() { ... tune) printf 'package:%s\n' "$1" ;; ... }
```

They mean "I touch the nginx package" — the whole thing, files included. But coordinates
compare within kinds: `package:nginx` does not cover `file:/etc/nginx/nginx.conf`, so a
downstream file-fact happily survives hork's wall. And the colleague *cannot* fix it —
which files a package owns is apt's knowledge, not theirs. So the owner says it once, for
everyone:

```sh
package.reaches() {                          # the package kind's owner (STRAWMAN spelling)
   printf '%s\n' "$1"       : service        # a package may enable its same-named unit
   dpkg -L "$1"             : file           # and reaches exactly the files it installed
}
```

- Declared once by the owner; applied by the engine to EVERY footprint coordinate of that
  kind, whoever emitted it. The colleague's `package:nginx` now covers nginx's files
  without the colleague learning anything.
- Footprints only. A fact's backing stays the one cell its probe checks; reach only ever
  *widens* a claim — the safe direction. Claiming too much walls too much; it never skips.
- One body serves both maturities: the `service` line is static (read at plan time, ships
  nothing); the `dpkg -L` line is a host question (runs read-only at probe time). The day
  a static line needs to become a question, it changes in place — same function, same file.
- The KIND rides the trailing annotation; the output lines are raw entities. The vocabulary
  is fixed when the oracle is read — a host can never mint a new kind at runtime — and raw
  tool output needs no `| sed` dressing.
- An emitting line with no annotation contributes nothing — a nudge in the plan's hints,
  never an error. The hard failures in this whole family remain what they have always
  been: syntax, and declarations that genuinely contradict each other.

- Spent: a line or two per kind, once, by its owner.
- Gained: composition — the moment two authors' work meets in one book, their claims cover
  what they *meant*, not just what they typed.


The residue, and the honest product statement
---------------------------------------------

`hork` never gets an oracle. The vendor won't document it; nobody sane will vouch for it.
So line 10 runs on every apply until the end of time, and the `ufw` line behind it verifies
rather than elides, forever. Past the last wall the product statement is exactly this: *Dorc
narrows your attention only where the world is described; elsewhere it makes your book fast
and safe, but not shorter.* The plan's honesty about that — nothing hidden, every surviving
line carrying its reason, every reason naming its wall — is itself the feature. A tool that
commented out `ufw allow 443/tcp` on vibes would be worse than no tool.


The bought unsoundness: one corner, fully fenced
------------------------------------------------

Everything in this story is honest in one of two cheap ways. Either Dorc *declines* to
promise — a wall, a guard, a line that stays in the plan — or a claim bites the person who
made it: stage 3's vouch, when wrong, endangers its own author's own tool's line, and that
price was stated where it was bought. Cheap honesty runs out in exactly one place, and it is
kept open on purpose: the survival tier (stages 5–7), where Dorc removes a line from the plan
on the strength of *other people's* at-most claims, with no runtime net underneath. This
section is that corner painted in full — a tool that buys unsoundness owes you the receipt.

Count what must all be true, together, before it can bite:

1. The admin typed `--trust-footprints`. Otherwise this tier does not exist — the claims are
   never even lifted.
2. The line's own author vouched it (`is_converged()`, reached, answering yes). No vouch, no
   elision — the line runs.
3. The probe genuinely measured the line's fact converged, minutes ago. Diverged or
   can't-tell, the line runs.
4. Something mutative really ran upstream, between that measurement and this line. (No
   running wall ⇒ an ordinary stage-3 elision; nothing here is being trusted at all.)
5. The running wall was *described* — its author made a clean at-most claim. An opaque wall,
   a confused trace, a half-resolvable argparse: all collapse to a total wall, and everything
   behind it runs or guards. Structural partiality cannot reach this corner; only a *clean*
   claim can be wrong here. (This is the oracle's version of
   `--trust-footprints` - *both* players must *explicitly* buy-in to
   unsoundness.)
6. That clean claim was wrong in the one way no machine can see: complete-looking but
   semantically incomplete. A `touches()` omitting a cell the tool really disturbs; a
   `reaches()` missing an edge; a `resolve()` splitting one referent into two names. (The
   frame problem — permanent, not an implementation gap.)
7. The canaries missed it. The coherence cross-check catches claims that contradict their own
   oracle's other statements, and the engine supplies the site's own coordinate outright —
   what remains is precisely the undetectable class.
8. And the unlucky overlap: this line's fact had to live exactly in the omitted spot — the
   wall really touched the cell the claim said it didn't.

Then, and only then: a line that needed to run, didn't, silently.

**We've committed under-execution. Our cardinal sin.**

Even then, the bite has edges. It is *attributed* — the plan recorded whose
claims licensed the survival, the `why`-lens names them, and the fix repairs
every book downstream of that oracle at once. It is *short* — the next plan's
probe measures reality, not memory; the broken fact reads diverged and the line
comes back. And it is *narrow* — every other line in the book ran, guarded, or
elided under its own separate license.

What the corner buys, and why it stays open rather than closed: attention on drifted days —
the entire stage-5-through-7 product. Without it, any line that really runs is an honest
wall, and one stale package index costs the whole tail of the book its shape, every drifted
morning, forever (stage 5's opening render). And every mechanism that could "close" the
corner re-spends the currency it exists to save: a runtime re-check under a survived elision
is just a guard — the line returns to the plan, the attention returns to you; a disclosed
contingency costs more attention than the plain line it wraps. The nakedness is not a gap
awaiting a cleverer release. It is the purchase price of the only thing that was ever for
sale in these stages: *removal by proof, or honesty about the lack of one* — and the proof
here is only ever as good as named, attributable people's claims about their own tools.

Said once, plainly, to the admin: past `--trust-footprints`, you are trusting named authors'
at-most claims. Everywhere else in Dorc, you were only ever trusting measurements, and your
own eyes.

The final ledger, steady state on the same book (counting the seven tool-sites; the two
housekeeping lines always show, and attention-lines counts everything still facing the
user):

```
stage    ran   verified   elided   attention-lines   spent
0        7     0          0        9                 nothing
1        4     0          3        6                 nothing (bundled)
2        2     2          3        6                 a library install
3        1     1          5        4                 2 minutes of sh
4        1     1          5        4                 an hour, for everyone else's benefit
5        1     1          5        4                 a touches() arm per verb (pays out on drifted days, not here)
6        1     1          5        4                 a resolve() per owned kind (pays out where names collide)
7        1     1          5        4                 a reaches() per owned kind (pays out in other people's books)
```

(Stages 6–7 deliberately do not move this book's numbers: their value shows where names
alias and where different authors' oracles meet — the fleet, not the single book.)

----

STATUS: stages 5–7 are the round-24/25 build frontier — the mechanisms (footprint × backing
× disjointness; probe-time derivation; owner canonicalization) are built or building, the
spellings are strawman-tier, and nothing in stages 0–4 depends on their outcome. Stage 7's
annotation-typed emission is the settled direction stage 5's stringly `kind:entity` format
migrates TO (the FIXME at stage 5). Expect these stages' renders and spellings to churn
before anything else here does. The design-round record behind stages 6–7 is
`Research/notes/24G`.
