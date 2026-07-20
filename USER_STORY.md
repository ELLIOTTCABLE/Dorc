> Document tier: AI-written, heavily user-audited (same class as KNOBS.md — every word
> human-reviewed before it counts; trumps the Research/ planning-ocean; changes rarely.)
>
> All terminal output below is ILLUSTRATIVE — the render format is not settled design. What IS
> settled, and what the renders are drawn to obey: the plan is the whole book, in original
> order, as plain sh; elided lines are present-but-commented-out; anything that will execute is
> never hidden; every surviving line carries its reason. ("rul-attention-honesty",
> `spike/CLAUDE.md`.)

The gradual-enhancement walkthrough
===================================

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


### Stage 0 — a thought-experiment: Dorc with its standard library disabled

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
plan: 7 to run (0 skipped)
```

Nothing was probed (nothing could be: probing requires an oracle to vouch that a check is
safe to run in the read-only probe phase, and no amount of cleverness substitutes for that
vouch). The plan is just the book, annotated. Spent: nothing. Gained: nothing yet, minus a
few seconds of analysis. That asymmetry — you get out what you put in — is the whole deal.

- Gained: a plan surface, and a place for hints to accrue.
- Lost: nothing. The off-ramp is intact: the book is still just a script.


### Stage 1 — the base library: the famous half

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
plan: 4 to run (3 skipped)
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


### Stage 2 — more coverage, below a wall: guards appear

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
plan: 2 to run, 2 to verify (3 skipped)
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


### Stage 3 — two minutes of engineering: the minimal foobar oracle

The hint has sharpened (it knows the topology now):

```
hint: 'foobar' (line 8) is unmodeled: it is the first wall - an oracle vouching its
      convergence would elide it when converged, and un-wall 1 downstream site
```

Annoyed, our admin puts on the engineer hat for exactly the length of a coffee. `foobar`
already has a status query (most tools do). They append to the book's own file — oracles and
runbooks can share a file, once it carries the dialect marker (`# dorc-lang/v0.2`, near the
top; an unmarked file always stays plain sh, and features requiring non-sh constructs
require the marker):

```sh
foobar__is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs)
      dest : org.foob.Certs = "$1"
      [ "$2" = "" ] || return 2
      foobar status --certs-current -- "$dest"    : org.foob.Certs:"$dest"@synced
      ;;
   *) return 2 ;;
   esac
}
```

11 lines, one verb, one probe, one gate — and the function's *name* is the license. In order:

- `foobar__is_converged()` declares "this body answers, for `foobar` invocations, the
  question in its name." The `__role` name is the opt-in semaphore — and it is already a
  plain POSIX function-name any shell can run (strip erases binds and marks only; it never
  rewrites a name). And the name is a *contract*: by writing a
  function that answers "is it converged," the author licenses Dorc to act on its yes — so
  its yes must mean "re-running this is noise I accept," not merely "some state holds." (A
  dpkg-'installed' package with an upgrade pending is exactly the gap: whether that counts
  as a yes is the author's judgment, and only theirs.) The plan attributes every elision
  and guard to the function that answered, by name; when the answer is wrong, there is a
  person to be wrong.
- `dest : org.foob.Certs = "$1"` binds the operand as the entity, in a kind this author just
  minted. Nobody approves kind names; there is no registry. It only has to agree with
  itself. (At the call-site that operand was `"$CERTS"` — the analyzer resolves plain
  variable-flow to the constant before binding; ordinary shell habits don't defeat it.)
- `[ "$2" = "" ] || return 2` is the arity gate: a two-operand invocation (`foobar
  sync-certs A B`) declines instead of quietly probing only the first operand — without
  it, a half-converged host would under-execute the second one.
- The trailing `: org.foob.Certs:"$dest"@synced` says: this probe's exit code *establishes*
  that cell (`#` introduces the selector — which aspect of the entity this line measures).
  The engine never interprets what "synced" means — it is an opaque token bound to the
  author's probe.
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
 8  # foobar sync-certs "$CERTS"                       # converged: org.foob.Certs:/etc/nginx/certs@synced
 9  # systemctl enable --now nginx                     # converged: service enabled+active
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 1 to run, 1 to verify (5 skipped)
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
plan: 2 to run, 2 to verify (3 skipped)
```

`foobar` comes back as a run, and `systemctl` falls back to its guard *for that apply* —
the wall is real again, because `foobar` will really act. The plan is per-world-state; the
promise ("plan mirrors what apply will do; divergence discovered mid-apply is
proceed-and-flag, never a mid-apply question") is not.

And a plan-time can't-tell — probe timeout, weird rc — is not quietly rounded to converged:
no verdict, no guard, no elision; the site runs. Everything fails toward run.

- Spent: ~15 minutes skimming docs, 11 lines of sh, zero new languages, zero config formats.
- Gained, steady state: seven tool-sites of attention down to two; foobar's re-sync mutation
  avoided.
- Gained, structurally: the certs state is now *described* — future books that touch it
  inherit the coverage for free.
- Not gained: anything about `hork`. Walls fall one tool at a time, each by its own author.


### Stage 4 — the battle-ready oracle: breadth, honesty, publication

Weeks later, with time to spare, the engineer hat comes back on — not to improve this book's
steady-state plan (it is already two lines; there is nothing left to buy here), but to make
the oracle *worth publishing*: correct for colleagues' books, other verbs, other days, other
argv shapes. The enriched oracle:

```sh
foobar__is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew)
      dest : org.foob.Certs = "$1"
      [ "$2" = "" ] || { printf 'UNK multi-operand foobar\n' >>"$DORC_REPORT"; return 2; }
      foobar status --certs-current -- "$dest"   : org.foob.Certs:"$dest"@synced
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
  belongs in the describing sibling, `foobar__predict()`: the lane that states what is true
  and predicts what would happen, and never licenses skipping a mutation.)
- The arity gate grows a breadcrumb: a two-operand invocation now hits the loud `UNK`
  refusal instead of stage 3's silent decline — and a refusal is just an answer-2 with a
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


### Stage 5 — the footprint: facts surviving a wall that stays

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
plan: 3 to run, 3 to verify (0 skipped)
```

One stale index cost the entire book its shape. And the frustrating part: *everyone
watching knows* `apt-get update` doesn't touch nginx's config, or foobar's certs, or the
service state. Everyone except Dorc — because knowing that is a claim about what a black-box
binary touches, and silence licenses nothing. Somebody who knows the tool has to say it.

The spelling being mocked here: a third role-sibling, next to `predict()` and
`is_converged()` — `disturbs()` — that *answers the disturbance question as runnable sh*.
The base library's apt oracle grows one; so does foobar's, one line in the author's
stage-4 file:

```sh
apt_get__disturbs() {                        # base library (STRAWMAN body)
   while [ "${1#-}" != "$1" ]; do shift; done
   verb="$1"; shift
   case "$verb" in
   update) : disturbs sm.dorc.PkgIndex ;;    # whole-kind claim: the kind rides the mark
   esac
}

foobar__disturbs() {                         # appended by foobar's author (STRAWMAN body)
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew) printf '%s\n' "$1" : disturbs org.foob.Certs ;;
   esac
}
```

Read it the way the analyzer does. Invoked with a site's argv (same contract as its
siblings), the body *emits the entities this verb disturbs, one per line*, the kind (and
someday a `#selector`) riding the trailing mark — and by emitting anything at all for a
matched verb, the author claims **at most these** ("whatever else this disturbs is residue
I answer for"). An unmatched verb emits nothing: no claim, no license, the wall stands —
silence stays safe. Stripped, it is a plain function any shell can run;
`foobar__disturbs sync-certs /srv/certs` printing `/srv/certs` is documentation that
executes.

The engine's move is then mechanical, and old as compilers: every probed fact already knows
*where its own truth lives* — nothing new to author, a fact's backing simply *is* what its
probe reads (`dpkg -s` reads the dpkg database; `systemctl is-enabled` reads unit state; it
cannot claim more than that by construction). A running wall's footprint gets intersected
against each downstream fact's backing. Empty intersection ⇒ the fact provably survives the
wall ⇒ its elision stands *even though the wall runs*. Non-empty, or no footprint ⇒ exactly
the stage-2 world: guard or run. (This is the separation-logic frame rule wearing work
clothes; the emitted-at-probe-time variant — stage-4 tools like `apt-get install`, whose
real file-payload only the host knows, answer by *asking the tool* inside `disturbs()` — is
what the literature calls a dynamic frame.)

One more thing before the payoff, because it is deliberate and permanent: none of this is
on by default. Surviving a wall means trusting authors' at-most claims with no runtime net,
so the whole tier sits behind an explicit flag — `--risk-faultless-skips`. Without it, the
plan above is what you get: honest walls, guards, runs.
And an honesty note that must outlive every future edit of this document: this opt-in is
marketing at best (you chose the danger; it isn't a Dorc bug when an author's claim is
wrong) and theatre at worst (it is desirable enough that nearly everyone will turn it on and
forget it). It exists anyway — the choice should be typed by the person who owns the
consequences, even when everyone types it.

The same stale-index morning, with footprints shipped and the trust typed:

```
$ dorc plan --verbose --risk-faultless-skips webhost.sh web1.example.net
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
plan: 2 to run, 1 to verify (5 skipped)
```

Now's a good time to start showing what the *default* output will look like, to the average
user:

```
$ dorc plan --risk-faultless-skips webhost.sh web1.example.net
 3  set -eu
 4  CERTS=/etc/nginx/certs
 5  apt-get update                                     # runs: diverged (index stale)
10  hork tune --profile web >>/var/log/hork.log 2>&1   # runs: unmodeled ('hork')
11  ( ufw_check allow 443/tcp ) \
11     || ufw allow 443/tcp                            # verify: converged, but past 'hork' (line 10)
plan: 2 to run, 1 to verify (5 skipped)
```

(This is the "attention product." The core goal, realized: remove all the lines 'you don't
need to worry about'.)

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

- Spent: one `disturbs()` arm per verb an author is willing to answer for; one typed flag
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
> high community effect.)


### Stage 6 — the kind-owner: two names, one thing

The footprint machinery compares coordinates by *name*, and names lie. On a real Debian
host, `nginx` and `nginx-full` can be one package under two names (provides); two paths can
be one file through a symlink. Watch it fail:

```sh
apt-get install -y nginx        # runs — a wall; its footprint says sm.dorc.Package:nginx
...
dpkg -s nginx-full >/dev/null 2>&1 || apt-get install -y nginx-full
```

The second line's fact is backed by `sm.dorc.Package:nginx-full`; the wall's footprint says
`sm.dorc.Package:nginx`; different strings ⇒ "disjoint" ⇒ the line elides *past a wall that really
touched its referent*. Every other gap in this machinery fails toward running too much;
this one under-executes, silently. It is the one place a name must be more than a string.

The fix is one function, written by the kind's owner — the party who holds what "the same
entity" means for that vocabulary:

```sh
sm_dorc_Package__resolve() {                 # the package kind's owner (STRAWMAN body)
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


### Stage 7 — reach: what touching an entity drags with it

> (FIXME, updated 2026-07-16: mechanism LANDED in the round-24 build; the member name and
> mark grammar are RULED (`271:rul-at-most-family-names` — `only` in a role name =
> complete-by-contract, totalistic-survey-before-authoring; `277` §4d); the body below
> stays strawman-tier in fine detail.)

One gap is left, and it is not the owner's — it is everyone else's. A colleague's oracle
for some package-fiddling tool honestly declares:

```sh
hork__disturbs() { ... tune) printf '%s\n' "$1" : disturbs sm.dorc.Package ;; ... }
```

They mean "I touch the nginx package" — the whole thing, files included. But coordinates
compare within kinds: `sm.dorc.Package:nginx` does not cover `sm.dorc.File:/etc/nginx/nginx.conf`, so a
downstream file-fact happily survives hork's wall. And the colleague *cannot* fix it —
which files a package owns is apt's knowledge, not theirs. So the owner says it once, for
everyone:

```sh
sm_dorc_Package__disturbance_reaches_only() {     # the package kind's owner (STRAWMAN body)
   printf '%s\n' "$1" : disturbs sm.dorc.Service  # a package may enable its same-named unit
   dpkg -L "$1" : disturbs sm.dorc.File           # and reaches exactly the files it installed
}
```

- Declared once by the owner; applied by the engine to EVERY footprint coordinate of that
  kind, whoever emitted it. The colleague's `sm.dorc.Package:nginx` now covers nginx's files
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


### The residue, and the honest product statement

`hork` never gets an oracle. The vendor won't document it; nobody sane will vouch for it.
So line 10 runs on every apply until the end of time, and the `ufw` line behind it verifies
rather than elides, forever. Past the last wall the product statement is exactly this: *Dorc
narrows your attention only where the world is described; elsewhere it makes your book fast
and safe, but not shorter.* The plan's honesty about that — nothing hidden, every surviving
line carrying its reason, every reason naming its wall — is itself the feature. A tool that
commented out `ufw allow 443/tcp` on vibes would be worse than no tool.


## The bought unsoundness: one corner, fully fenced

Everything in this story is honest in one of two cheap ways. Either Dorc *declines* to
promise — a wall, a guard, a line that stays in the plan — or a claim bites the person who
made it: stage 3's vouch, when wrong, endangers its own author's own tool's line, and that
price was stated where it was bought. Cheap honesty runs out in exactly one place, and it is
kept open on purpose: the survival tier (stages 5–7), where Dorc removes a line from the plan
on the strength of *other people's* at-most claims, with no runtime net underneath. This
section is that corner painted in full — a tool that buys unsoundness owes you the receipt.

Count what must all be true, together, before it can bite:

1. The admin typed `--risk-faultless-skips`. Otherwise this tier does not exist — the claims
   are never even lifted.
2. The line's own author vouched it (`is_converged()`, reached, answering yes). No vouch, no
   elision — the line runs.
3. The probe genuinely measured the line's fact converged, minutes ago. Diverged or
   can't-tell, the line runs.
4. Something mutative really ran upstream, between that measurement and this line. (No
   running wall ⇒ an ordinary stage-3 elision; nothing here is being trusted at all.)
5. The running wall was *described* — its author made a clean at-most claim. An opaque wall,
   a confused trace, a half-resolvable argparse: all collapse to a total wall, and everything
   behind it runs or guards. Structural partiality cannot reach this corner; only a *clean*
   claim can be wrong here. (This is the oracle's side of
   `--risk-faultless-skips` - *both* players must *explicitly* buy-in to
   unsoundness.)
6. That clean claim was wrong in the one way no machine can see: complete-looking but
   semantically incomplete. A `disturbs()` omitting a cell the tool really disturbs; a
   `disturbance_reaches_only()` missing an edge; a `resolve()` splitting one referent into
   two names. (The frame problem — permanent, not an implementation gap.)
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

Said once, plainly, to the admin: "past `--risk-faultless-skips`, you are trusting named
authors' at-most claims." Everywhere else in Dorc, you were only ever trusting their
*measurements*, and your own eyes.

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
5        1     1          5        4                 a disturbs() arm per verb (pays out on drifted days, not here)
6        1     1          5        4                 a resolve() per owned kind (pays out where names collide)
7        1     1          5        4                 a disturbance_reaches_only() per owned kind (pays out in other people's books)
```

(Stages 6–7 deliberately do not move this book's numbers: their value shows where names
alias and where different authors' oracles meet — the fleet, not the single book.)


## Recovery: how you find out, and what you do about it

Everything above is about what Dorc *does*. This section is about how you *learn* — which
is not a side-topic, because an orchestrator like Dorc is a tool that sometimes decides to
*not* do things. A tool with that power owes you an account of its decisions; and it owes
you that account most of all on the morning something is wrong, which is precisely when
you are least patient with it.

Three principles govern every way Dorc talks to you.

First: knowledge arrives at the moments you were already looking. There is one plan and
one moment of consent; Dorc never adds interaction moments beyond it. What Dorc
*volunteers* — the plan's annotations, a hint that an oracle would recover three sites —
is rationed ruthlessly, because your attention is the product being conserved. What you
*ask for* is the opposite: unbounded. `dorc why` answers a question you chose to ask, so
it holds nothing back; and it works *after* everything is apparently done, with nothing
you had to set up beforehand. The receipt it reads is small and boring — what was
measured, what was decided, what actually ran — and the full explanation is recomputed
from it on demand, which is why there is no log-ocean to rotate and no history database
to maintain. Ask tomorrow; ask next week.

Second: every explanation is a chain, and every link in the chain wears a label saying
what kind of thing it is. *Measured* — a probe read the world, at a stated time.
*Vouched* — a named author accepted a judgment, at a stated file and line. *Claimed* — a
named author asserted something no machine verified. *Derived* — Dorc computed a
consequence. *Consented* — you typed a flag. These labels are not writing style; the
machinery cannot render a claim in measurement's clothing, and a person's name is
attached wherever a person is load-bearing. When an explanation is wrong, the labels are
what keep it honestly wrong: Dorc ranks its own possible failures, and telling you a
*wrong* cause is worse than admitting it cannot tell — so where certainty runs out, the
chain says so rather than rounding up.

Third: deciding and explaining fail in opposite directions, on purpose. When Dorc
*decides*, doubt makes it act less — an unsure line runs, an unsure probe never ships.
When Dorc *explains*, doubt makes it say more, with the doubt attached — the explanation
may draw on knowledge the deciding machinery is forbidden to touch, provided every such
statement carries its label. A narrower rule would make the safe tool useless at exactly
the moment it owes you the most.

Here is what that feels like, on the worst morning this document describes. Recall the
bought-unsoundness corner: `--risk-faultless-skips` typed, foobar's author's at-most
claim trusted, and — the unlucky day — the claim was incomplete: `sync-certs` also
rewrites a systemd drop-in, which its author forgot. The certs drifted; foobar ran; the
`systemctl` line's elision survived on the strength of the claim; the service came up
wrong overnight. You know none of this yet. You know the site is down, and that
yesterday Dorc said everything was fine.

```
$ dorc why 9
book.sh:9  systemctl enable --now nginx
  removed from the plan (elided); did not run in the apply of 2026-07-17 06:12.

  it was removed because all of the following held together:
  1. measured:   nginx was enabled+active on web1 at plan time (06:11:52)
  2. vouched:    the service oracle's author accepts already-enabled+active as
                 reason enough not to re-run this (systemctl.oracle.sh:12)
  3. ran above:  book.sh:8 `foobar sync-certs /etc/nginx/certs` really ran --
                 ordinarily that would send line 9 back into the plan as a
                 re-check --
  4. claimed:    but foobar's oracle claims sync-certs disturbs at most its own
                 certs (foobar.oracle.sh:31 -- an author's claim; nothing
                 verifies it)
  5. derived:    that claim does not overlap what link 1 measured
  6. consented:  --risk-faultless-skips was set, which is what lets 4+5 keep a
                 line out of the plan past a running mutation.

  if line 9 SHOULD have run: dorc cannot see which link is wrong, but the links
  are not equally trustworthy -- 4 is the one unverified human claim in this
  chain. if `foobar sync-certs` also touches service state, that claim is what
  wrongly kept line 9 out.
  to check:  `dorc plan book.sh web1` re-measures the world as it is now.
  to fix:    foobar.oracle.sh:31 is the line to widen; every book using that
             oracle inherits the repair.
```

Notice what the answer does not do. It does not guess. The epilogue's finger points at
link 4 not because Dorc detected the wrongness — it cannot, that is what the frame
problem means — but because the *design* knows which link in this chain was unverified by
construction: it is the one place this document said a naked human promise ships. Stating
that is not an accusation; it is the receipt for the trust you spent when you typed the
flag.

And notice the shape of recovery, because it is always the same two moves. *Re-measure*:
the next plan reads reality, not memory — Dorc kept no state that can stay wrong, so the
broken fact comes back diverged and the line returns on its own. *Fix at the leverage
point*: one file, one line, named — and because that claim was the shared artifact, the
repair reaches every book on every host that trusts it, including people who will never
know the outage happened. Gradual enhancement runs on exactly this loop: the same channel
that nags you toward the first two-minute oracle is the one that, on the bad morning,
tells you which two minutes to spend next.

What recovery is *not*: there is no drift daemon, no fleet history, no stored suspicion.
The receipt is not a cache and never feeds a decision; it exists so that one question —
"why?" — always has an answer, at the moment you are angriest, with names on it. The plan
is the promise; the why is the receipt.

----

STATUS (refreshed 2026-07-16): stages 5–7's mechanisms LANDED in the round-24 build
(evidence ledger `Research/notes/24C`); the spellings above now show the block-settle RULED
layer — bare munged `__role` names (`24M`), the `disturbs` family
(`271:rul-at-most-family-names`), the `#selector` mark grammar (`277` §4), the flag's ruled
name (`271:rul-flag-named-risk-faultless-skips`) — with fine detail still strawman-tier;
the corpus-wide respell lands at `270:block-rebuild`. Nothing in stages 0–4 depends on
5–7's outcome. Design-round records: `Research/notes/24G` (the kind-owner family);
`plans/271` + `notes/277` (the ruled layer). Wrapped/contexted sites (sudo, chroot, netns)
are deliberately absent from this walkthrough — their settled design is context-entry
probing, `plans/27C`.


Other usage-patterns
====================

These are secondary to the primary (read: hard, and therefore worth-focusing-on)
orchestration goal. That said, they're positions where Dorc can cheaply add
value with the same infrastructure we're building, and thus worth keeping a
sideeye on.

> IMPORTANT: This section is less-settled-design than the above; consider it
> prospective, not prescriptive. A "stretch goal" if you will.


## Dotfiles / local-system management

Same machinery, different room: the "fleet" is one laptop, the admin and the host share a
chair, and — per DESIGN's not-the-only-tool principle — a beloved tool probably already owns
the dotfiles themselves. Both stories below are deliberately *cooperation* stories (the
happy-child and happy-parent postures, respectively); neither asks anyone to leave the tool
they love.

### Idempotence for `chezmoi` scripts

chezmoi owns this user's dotfiles, and owns them well; Dorc's ambition here is one script.
chezmoi's escape hatch for everything-that-isn't-file-contents is its run-scripts, and its
own docs draw the boundary plainly: scripts break the declarative model, should be used
sparingly, and *should be idempotent* — with no machinery offered toward either. So the
canonical package script rations itself by content-hash instead:

```sh
#!/bin/sh
# .chezmoiscripts/run_onchange_install-packages.sh
brew install ripgrep fd jq shellcheck
brew install --cask kitty
defaults write com.apple.dock autohide -bool true
killall Dock
```

`run_onchange_` means: re-run when this *text* changes. Two structural smells. Drift is
invisible — uninstall `ripgrep` and it stays gone forever, because the text didn't change.
And when the text does change (one cask added), *every* line re-runs: five brew no-op
crawls and a Dock restart, to install one package.

The move is one line — the shebang — and zero chezmoi configuration, because chezmoi execs
scripts directly and the shebang is honored:

```sh
#!/usr/bin/env dorc-run
# .chezmoiscripts/run_install-packages.sh    (renamed to plain run_: every apply)
brew install ripgrep fd jq shellcheck
...
```

`dorc-run` (the *analyzed* sibling of the strip-and-exec `dorc-sh` —
deliberately a different token, so `dorc-sh` can stay dumb forever): probe,
elide the converged lines, guard what cannot elide, run the rest — headlessly.
Being a good guest is most of the design:

- stdout/stderr and the exit code pass through byte-for-byte — chezmoi sees exactly the
  script it ran, so its fail-fast, keep-going, and run-state bookkeeping all keep working;
- nothing is printed at it (no TTY, nobody watching): the plan lands in the why-log, and
  `dorc why --last` answers "what did that apply actually do, and why";
- no second state database appears. Dorc remembers nothing; it re-measures.

And the rationing retires. Renamed to plain `run_`, the script becomes the reconcile loop
chezmoi couldn't offer: drift heals (`ripgrep` comes back), additions install alone (every
other line elides), and the mandated-but-unassisted idempotence is machinery now instead of
author-discipline.

- Spent: one shebang line per script. No config, no migration, no new format.
- Gained: the `run_once_`/`run_onchange_` hash-rationing retires; host drift heals on every
  apply; "what did that script just do" has an answer.
- Off-ramp: intact twice over — the file strips to plain sh, and on a Dorc-less box it still
  runs bare (`sh script.sh`), exactly as before.
- Not gained: `sudo` lines still wall (honestly); and nothing here helps with chezmoi's own
  half of the world — that is the next story's direction.

### Pre-git single-binary wrapper for mechanizing git dotfiles / Brewfiles / etc

The inverse posture: Dorc above, beloveds below. Every real setup is a stack — a dotfiles
repo, a Brewfile, some `defaults` — and the glue *between* those tools is nobody's product:
a README of steps, or a bootstrap script with ordering anxieties. That glue is a book:

```sh
#!/bin/sh
# machine.sh — the glue layer, spelled as what you'd type anyway
set -eu
xcode-select -p >/dev/null 2>&1 || xcode-select --install
command -v brew >/dev/null 2>&1 || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew bundle check --file="$HOME/Brewfile" || brew bundle install --file="$HOME/Brewfile"
chezmoi apply
defaults write com.apple.dock autohide -bool true
```

Day zero, on a naked laptop, this file needs nothing but `sh` — not git (line 4 is where
git comes from, via the CLT), not Homebrew, not Dorc itself. `curl … | sh` and walk away:
the off-ramp is the on-ramp, and the book predates its tool. Dorc arrives as one static
binary whenever it arrives, and every later morning:

```
$ dorc plan machine.sh
 1  #!/bin/sh
 2  # machine.sh — the glue layer, spelled as what you'd type anyway
 3  set -eu
 4  # xcode-select -p >/dev/null 2>&1 \
 4  #    || xcode-select --install                     # converged: your guard holds (rc 0)
 5  # command -v brew >/dev/null 2>&1 \
 5  #    || /bin/bash -c "$(curl -fsSL …)"             # converged: your guard holds (rc 0)
 6  # brew bundle check --file="$HOME/Brewfile" \
 6  #    || brew bundle install --file="$HOME/Brewfile" # converged: your guard holds (rc 0)
 7  # chezmoi apply                                    # converged: chezmoi verify (rc 0)
 8  defaults write com.apple.dock autohide -bool true  # runs: unmodeled ('defaults')
plan: 1 to run (4 skipped)
```

No host argument: the target is the machine you are sitting at. Almost nothing
here is new machinery. Lines 4–6 are the admin's own hand-written guards —
including Homebrew's *first-party documented* scripting idiom on line 6 — and
they lift exactly like the `dpkg -s` guard in stage 1, once the base library
vouches those reads probe-safe. The one new thing is line 7, a beloved-tool line
with no hand guard: the beloveds ship their own read-only convergence verbs, so
their stdlib oracles are near-pure delegation —

```sh
chezmoi__is_converged() {
   case "$1" in
   apply)  chezmoi verify ;;
   update) return 2 ;;
   *)      return 2 ;;
   esac
}
```

— with the judgment still the author's, exactly as at stage 4: `verify` answers for `apply`
(local truth), but the author *declines* `update`, whose convergence lives partly at the
remote (a pull elided on stale local knowledge would under-execute). Delegation is a shape,
not a free lunch. And the happy-sibling posture falls out of statelessness: chezmoi
rewriting half of `$HOME` between runs costs Dorc nothing, because there is nothing to go
stale — the next plan re-measures the world as it actually is.

- Spent: the bootstrap script they already had, minus its ordering anxieties; a stdlib of
  delegation one-liners somebody publishes once.
- Gained, day zero: naked-laptop bootstrap with `sh` as the only dependency.
- Gained, every later morning: the stack's glue layer — formerly apply-and-hope in every
  direction — gets a drift report and a minimal re-apply.
- Not gained: insight *inside* `chezmoi apply` or `brew bundle` — the happy parent hands the
  machine over at those lines and trusts their own verbs. Dorc adds value between the
  beloveds, not within them. (The two postures compose: the previous story sits inside the
  scripts chezmoi runs, this book sits above chezmoi itself.)


## Cooperation with existing ops ecosystems

The cast is professional now, and the frame from the dotfiles stories inverts: these teams
already have tools they trust and doctrines they chose. Dorc's positions here are the seams
those doctrines *concede* — and both stories below deliberately say out loud what Dorc
refuses to promise in them.

> (The design-round record behind these is `Research/plans/24R`; its §0
> impossibility-ledger governs every claim made here.)

### Mutable residue on a principaled, declarative ops team

This team did everything right. Terraform plans gate PRs; images are baked, not patched;
the Kubernetes estate reconciles itself from git. Doctrine: cattle, not pets; no SSH. And
yet — as the immutable canon itself concedes — baking the cattle *reduces* what config
management must own; it never reaches zero. The residue here: one bastion, two on-prem
edge boxes at a customer site, and the node-bootstrap script baked into the AMI. None of
them gets rebuilt on Tuesdays. Today they are managed by guilt: a `bastion.sh` somebody
SSHes over quarterly, with terminal history as the audit trail — because the moment you
need to change something *now* on a running host, the doctrine's tools are, by design,
not there.

The move: Dorc adopts the residue, and *only* the residue. Same script, now a book:

```
$ dorc plan bastion.sh bastion.prod
 1  #!/bin/sh
 2  set -eu
 3  # apt-get update                                   # converged: package index fresh
 4  # dpkg -s wireguard >/dev/null 2>&1 \
 4  #    || apt-get install -y wireguard               # converged: your guard holds (rc 0)
 5  corp-agent enroll --renew                          # runs: unmodeled ('corp-agent')
 6  ( ufw_check limit 22/tcp ) \
 6     || ufw limit 22/tcp                             # verify: converged, but past 'corp-agent' (line 5)
plan: 1 to run, 1 to verify (2 skipped)
```

Three things transfer from the culture they already have, and one question gets an honest
answer.

- The review instinct transfers whole: the plan *is* a script, so the reviewed artifact is
  what applies — their Terraform-plan-in-the-PR habit, satisfied with no new machinery.
  (Stated plainly: probed facts can drift between review and apply; the guards are the
  mitigation, and a long-stale plan deserves a fresh one.)
- The vendor agent stays honest residue: `corp-agent` is unmodeled, so it runs, walls, and
  the `ufw` line behind it verifies instead of eliding — forever, until someone who knows
  that tool vouches for it. Nothing is hidden to make the pets look tidy.
- Nothing is contested: the Terraform and Kubernetes estates are untouched, and Dorc
  brings no state file to sit beside theirs — there is nothing of ours to lock, strand, or
  go stale. The position is the residue, permanently. That is the point.

The question: "can we cron this as drift monitoring?" The honest answer: yes, and it sees
*inside* the host where `terraform plan` and drift scanners see only the cloud API — and a
scheduled plan is a scheduled probe pass: real reads, on real, fragile pets, unattended.
That is the exact shape of the nightly dry-run cron that once broke production at a Chef
customer, and read-only ≠ non-blocking ≠ side-effect-free. So it ships with a
`-detailed-exitcode`-style contract (`dorc plan --exit-code`, STRAWMAN: 0 converged /
2 diverged / 1 error) *and* with timeouts, probe cost-classes, and an opt-out list for the
boxes too fragile to interrogate on a schedule. Anyone selling an unconditionally safe
scheduled dry-run is selling the claim Chef retracted.

- Spent: adopting one runbook. No agent, no state backend, no new estate.
- Gained: the pets get the same reviewed-change culture as the cattle; in-host drift
  visibility nothing else in the stack has; the emergency-mutation path becomes plan-gated
  instead of guilt-gated.
- Not gained, ever: the declarative estates. Happy siblinghood means the boundary holds
  from our side too.

### Gradual-enhancement transition from legacy Ansible

Six years of playbooks. The modules are fine — a good module already does, internally,
exactly what a Dorc guard does (check, then converge, fused at act-time), and nothing in
this story improves on one. But every real estate accretes shell tasks, and theirs itch
exactly where the linter says:

```yaml
- name: install node_exporter
  ansible.builtin.script: files/install-exporter.sh
  args: { creates: /usr/local/bin/node_exporter }

- name: retune ingest pipeline
  ansible.builtin.shell: sysctl -w net.core.rmem_max="$(cat /etc/ingest/rmem)" && systemctl restart ingest
  changed_when: false        # a standing lie, to keep the play quiet
```

Shell tasks report `changed` unless hand-annotated. Under `--check` they are either
skipped outright or judged by the annotation alone — the file test — never by observing
the task itself; and that `creates:` points at a path the script stopped creating two
refactors ago, so the apply *and* the check-run both trust a stale claim. The annotations
are hand-written convergence claims — maintained by hand, verified by nobody. The
transition is three rungs, and none of them is a rewrite:

**Rung 1 — zero migration.** `files/install-exporter.sh` gets a `dorc-run` shebang (the
runner from the dotfiles stories above). Ansible transfers the file and executes it
directly, so its flow is untouched: same task, same exit-code contract, same fail-fast.
The *interior* of the script now converges per-line and leaves a why-log; the `creates:`
stays, harmless and newly redundant.

**Rung 2 — the annotations map.** When a task does migrate into a book, nothing is learned
twice: `creates: /usr/local/bin/node_exporter` *is* `[ -e /usr/local/bin/node_exporter ] ||`
— the same guard, now lifted and probe-checked instead of trusted-and-stale. And
`changed_when` becomes nothing at all: the plan measures what changed rather than being
told. Six years of annotation discipline turns out to have been oracle-authoring practice
all along — the stage-1 continuity bet, replayed in ops.

**Rung 3 — what stays Ansible, stays.** The book's line for the module-heavy remainder is
`ansible-playbook site.yml --tags certs`, and its oracle teaches by *declining*:

```sh
ansible_playbook__is_converged() { return 2 ;}   # --check is not a convergence verb
```

A play's check-mode answer is only as honest as its least check-aware task — and shell
tasks are skipped outright ("report nothing and do nothing," per Ansible's own docs) — so
there is no trustworthy whole-play yes to delegate to; the honest answer is can't-say, and
the line runs, in the plan, on every apply, forever. Some incumbents simply do not offer a
check verb worth vouching for; a declined vouch is a line that runs, shown to your face,
rather than a simulation of confidence.

Off-ramp at every rung: playbooks stay playbooks, scripts stay scripts; strip Dorc and
every artifact is exactly the standard thing it was before.

- Spent: rung 1, a shebang per script file; rung 2, per-task and only when a task earns
  migration; rung 3, a one-line refusal.
- Gained: convergence, drift-healing, and a real answer to "what did that task actually
  do" in exactly the tasks Ansible's own check-mode is blind to.
- Not gained: module interiors; and no forecast for the `ansible-playbook` line, ever —
  which is the correct amount of forecast for a verb nobody trusts.
