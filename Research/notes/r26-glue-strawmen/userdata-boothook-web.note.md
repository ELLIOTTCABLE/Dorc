# Companion note — `userdata-boothook-web.sh`

> Round r26 (ops-glue-residue), writing phase, builder B1 (cloud lifecycle).
> The book is IMAGINATION-TIER frozen evidence and must never be executed.
> This note is where the honest accounting lives.

---

## 1. What is real, and what is invented

### Real (primary docs, all fetched 2026-07-28)

| Claim in the book | Source |
|---|---|
| `#cloud-boothook` is a plain-sh user-data format that "is run very early in boot, during the network stage, before any cloud-init modules are run" and "runs every boot" | <https://docs.cloud-init.io/en/latest/explanation/format/boothook.html> |
| `INSTANCE_ID` is exported to boothooks | same page |
| The prefix line is stripped, the remainder written mode `0o700` under `/var/lib/cloud/instances/<iid>/boothooks/`, then **exec'd directly** (so line 2's shebang is honoured and is effectively mandatory) | `cloudinit/handlers/boot_hook.py`, `_write_part` + `subp.subp([filepath], …)` — <https://github.com/canonical/cloud-init/blob/main/cloudinit/handlers/boot_hook.py> |
| A boothook that fails is **caught, logged and swallowed** | same file: `except subp.ProcessExecutionError: util.logexc(LOG, "Boothooks script %s execution error", filepath)` |
| "log errors, but proceed" is the stated paradigm, so half-applied firstboot is an ordinary outcome | <https://docs.cloud-init.io/en/latest/explanation/return_codes.html> — NB the page frames the old "reports `done` despite an error" behaviour in the past tense; modern cloud-init reports `degraded done` and exits 2. Reporting is not repairing: nothing re-applies the half that did not land |
| `cloud-init-per <freq> <name> <cmd> [args…]`, freqs `once`/`instance`/`always`; "run only once for a given instance-id and re-run for new instance-id" | <https://manpages.debian.org/testing/cloud-init/cloud-init-per.1.en.html> |
| Sem paths: `once`/`always` → `/var/lib/cloud/sem/bootper.$name.$freq`; `instance` → `/var/lib/cloud/instance/sem/bootper.$name.$freq`; `-` is rewritten to `_`; `must be root`; `always` writes a sem it never reads | verbatim source — <https://raw.githubusercontent.com/canonical/cloud-init/main/tools/cloud-init-per> |
| The `-`→`_` rewrite exists because cloud-init's `migrator` module renamed sem files out from under it | <https://github.com/canonical/cloud-init/issues/3314> |
| Upstream's own once-per-instance boothook example is a hand-written guard (`cloud-init-per instance do-hosts /bin/false && exit 0`) | boothook format page, above |
| EC2: "User data is an instance attribute", retrievable "using instance metadata" from inside the box | <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html> (same page carries the 16 KB raw cap) |
| Azure narrows it — custom-data is NOT in IMDS, Azure *user-data* is, and neither should hold secrets: "We advise *not* to store sensitive data in custom data" | <https://learn.microsoft.com/en-us/azure/virtual-machines/custom-data> — corrects an over-broad claim in this note's first draft |
| Size caps: DO 64 KiB, Azure 64 KB, GCP 256 KB, EC2 16 KB raw | DO API reference "may not exceed 64 KiB"; r26 turn-01 channel ledger for the rest |
| The dpkg-lock race is real, long-lived, and the community's answer is a `fuser` wait loop; `fuser` is absent on minimal Ubuntu images | <https://bugs.launchpad.net/cloud-init/+bug/1693361> (apt-daily racing cloud-init) and Sinjakli's write-up, both via r26 turn-01; the tool-availability caveat is Sinjakli's, verbatim |
| `apt-get -o DPkg::Lock::Timeout=<n>` exists | apt ≥ 2.0 (`apt-get(8)`); the modern first-party answer to the same race |
| The image-capture trap: an image taken from a `manual_cache_clean: true` instance never applies per-instance config again, **including ssh host key rotation** | <https://docs.cloud-init.io/en/latest/explanation/first_boot.html> |

### Invented (imagination-tier; all on the round's sanctioned palette)

- `dorc compile` as an offline guard-artifact face, with the no-semantic-fork rule.
- The `dorc-run` runner (for the day-N face; design prose only).
- The `cloud-init-per` wrapper oracle, and `: lends user|fsview|netns` + `env "$@"`
  as its dimension enumeration.
- Wall-transparent pure-delay loops (chafe-3 below).
- `--risk-faultless-skips` renders that presuppose a derived redirect footprint
  (finding-1 below) — flagged inline where used.

---

## 2. Why this shape

### The three deliveries are one meaning, and that is the load-bearing claim

The chef-solo grave is a two-code-path split: `if Chef::Config[:solo]` forks in
recipes, and local-mode's stated win was "one less code path". The rule this book
obeys is that **offline may NARROW, never FORK**. Concretely, across the three
deliveries:

- the *bytes the machine runs* differ (raw source / compiled artifact / probed plan);
- the *set of lines with a chance of executing* is identical in all three;
- what changes is only how much is known before execution, and therefore how much
  can be removed. Nothing changes what a line means, and no line exists in one
  delivery and not another.

That is why the raw delivery is in the book's header rather than being a curiosity.
If `dorc compile` were ever the *only* way to make this file work, the file would
have become a Dorc artifact rather than a shell script, and the off-ramp would be
gone. The test is simple and permanent: **paste the source into
`--user-data-file` and the machine still comes up.**

### Being your own payload constrains where annotations may live

Delivery #1 runs the source un-stripped. A bare `pkg : sm.dorc.Package = "$1"` in
the book body would be `sh: pkg: not found` on a real boot. So every mark in this
file sits inside a function body that the raw delivery never calls: the oracle is
annotated, the book is not.

That is a livable discipline — books mostly do not need marks — but it is a real
constraint, and it is **the strongest argument I have seen for the `#:` comment
carrier existing at all** (`KNOBS:kTYANNOT-eol-comment`). A `#:`-carried mark is
inert on every route including an un-stripped raw boot, which is exactly the
property this delivery class needs. The inline colon form's cost, elsewhere a
strip-pass away, is here a hard wall: there is no strip pass in the cloud's boot
sequence.

### The re-runnable guarded artifact is a repair story, not a convenience

Cloud-init's error paradigm is "log errors, but proceed", and the boothook handler
implements it literally — a boothook that dies is caught and logged, and cloud-init
still reports `done`. Upstream separately disclaims re-running its own payload
("may be destructive and must never be done on a production system"). Put those
together and the channel's normal failure mode is: **a half-applied machine that
reports success and has no supported way to finish applying itself.**

An every-boot payload where every mutation is guarded turns "half-applied" into "it
will finish on the next boot, and each boot is cheaper than the last". Nobody ships
general convergence machinery into this cell — turn A's finding was "empty of
convergence machinery, not of channels" — and this is why the cell is worth
occupying.

### `cloud-init-per` is described, not replaced

The oracle in §0 exists so that the idiom the channel documents keeps working when
somebody inherits a payload that uses it. But look at what the book below it does
with that power: nothing, until §6. State-keyed rationing retires name-keyed
rationing everywhere it can — this is the chezmoi `run_onchange_` story replayed in
a different room — and §6 is the honest exception, where the intent ("seed once,
then let the operator own it") is genuinely not a state check anybody could write.

---

## 3. Rendered artefacts

> Illustrative. Render format is not settled.

### Day zero — the compiled artifact

```
$ dorc compile userdata-boothook-web.sh >build/ud-web1.txt
compiled: 5 guards inserted, 3 oracle bodies inlined, 0 lines elided (offline: no probe)
          6.1 KiB   (target channel cap is yours to know: DO 64 KiB · EC2 16 KiB raw)
note:  6 sites were already guarded by the book's own author and were left alone
warn:  'cat' (line 161) writes through a redirect: unmodeled, runs unguarded
warn:  'tar' (line 191) is unmodeled: runs unguarded
warn:  'curl' (line 190) is unmodeled: the book's own guard is what rations it
```

Excerpt of the emitted bytes:

```sh
#cloud-boothook
#!/bin/sh
# dorc compile — offline guard artifact from userdata-boothook-web.sh
# No probe phase exists, so nothing is elided; every modeled site carries its
# oracle's own check in front of the author's untouched bytes.
set -eu
: "${INSTANCE_ID:=unknown}"
DREP_V1=/var/log/dorc/report.txt
mkdir -m 700 -p /var/log/dorc 2>/dev/null || DREP_V1=/dev/null
export DREP_V1

SITE=web1.example.net
DOCROOT=/srv/www
CONF=/etc/nginx/sites-available/web1
SEED=https://dist.example.net/web1-docroot.tar.gz

apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift 2; done
   case $1 in
   update)  [ -n "$(find /var/lib/apt/lists -maxdepth 1 -name '*_Packages' \
                      -newermt '-1 hour' -print -quit 2>/dev/null)" ] ;;
   install) shift; while [ "${1#-}" != "$1" ]; do shift; done
            [ "$2" = "" ] || return 2
            dpkg-query -W -f='${Status}' -- "$1" 2>/dev/null \
               | grep -q '^install ok installed$' ;;
   *) return 2 ;;
   esac
}
ufw__is_converged()      { … }
systemctl__is_converged() { … }

until getent hosts deb.debian.org >/dev/null 2>&1; do sleep 1; done
if command -v fuser >/dev/null 2>&1; then
   while fuser /var/lib/dpkg/lock-frontend >/dev/null 2>&1; do sleep 1; done
fi

APT="apt-get -o DPkg::Lock::Timeout=120"

( apt_get__is_converged -o DPkg::Lock::Timeout=120 update ) \
   || $APT update
dpkg -s nginx >/dev/null 2>&1 || $APT install -y nginx
dpkg -s unattended-upgrades >/dev/null 2>&1 || $APT install -y unattended-upgrades

( ufw__is_converged allow 80/tcp )  || ufw allow 80/tcp
( ufw__is_converged allow 443/tcp ) || ufw allow 443/tcp
( ufw__is_converged --force enable ) || ufw --force enable
…
```

Three things to read off that.

1. **The admin's own guards are untouched.** Lines 138, 139, 159, 171 and 190 already
   check before they act; the compiler recognises them and inserts nothing. Their
   guard *is* the mechanism. Five inserted guards, six left alone.
2. **The inserted checks are somebody else's tested sh**, resolved argv and all
   (`-o DPkg::Lock::Timeout=120 update` flows into the check as arguments; the
   author's bytes on the right of `||` are byte-identical to the source).
3. **The offline face's honest value is small, real, and unexciting**: it converts the
   five lines the admin could not be bothered to guard, plus it opens the report
   lane. It buys no attention — there is nothing to remove without a measured world —
   and the note should not pretend otherwise. What it *does* buy is that this file
   can be handed to a machine that has never heard of Dorc.

### Day N — steady state, over ssh

```
$ dorc plan --verbose userdata-boothook-web.sh web1.example.net
 53  set -eu
 57  : "${INSTANCE_ID:=unknown}"
 59  SITE=web1.example.net
 60  DOCROOT=/srv/www
122  until getent hosts deb.debian.org …; do sleep 1; done  # runs: wait, casts no wall
129  # if command -v fuser >/dev/null 2>&1; then            # omitted: branch dead
130  #    while fuser /var/lib/dpkg/lock-frontend …         #   ('fuser' not on PATH)
131  # fi
135  APT="apt-get -o DPkg::Lock::Timeout=120"
137  # $APT update                                          # converged: package index fresh
138  # dpkg -s nginx >/dev/null 2>&1 \
138  #    || $APT install -y nginx                          # converged: your guard holds (rc 0)
139  # dpkg -s unattended-upgrades … || …                   # converged: your guard holds (rc 0)
147  # ufw allow 80/tcp                                     # converged: rule present
148  # ufw allow 443/tcp                                     # converged: rule present
149  # ufw --force enable                                   # converged: active
159  # [ -d "$DOCROOT" ] || install -d -m 755 "$DOCROOT"    # converged: your guard holds (rc 0)
161  cat >/run/web1.conf.new <<EOF                          # runs: unmodeled (writes via redirect)
171  cmp -s /run/web1.conf.new "$CONF" || { … }             # runs: your own guard re-checks live
                                                            #   (past line 161)
180  ( systemctl_check enable --now nginx ) \
180     || systemctl enable --now nginx                     # verify: converged, but past 'cat' (line 161)
190  [ -f /opt/web1-docroot.tar.gz ] || curl … "$SEED"      # runs: your own guard re-checks live
                                                            #   (past line 161)
191  cloud-init-per instance seed-docroot tar -xzf …        # runs: unmodeled ('tar')
plan: 5 to run, 1 to verify (8 skipped)
```

Line 161 costs the bottom third of the book its shape, and it is worth being angry
about — see finding-1. With the derived redirect footprint that finding proposes,
and the admin's typed consent:

```
$ dorc plan --verbose --risk-faultless-skips userdata-boothook-web.sh web1.example.net
…
161  cat >/run/web1.conf.new <<EOF                          # runs: writes sm.dorc.File:/run/web1.conf.new
171  cmp -s /run/web1.conf.new "$CONF" || { … }             # runs: your own guard reads line 161's file
180  # systemctl enable --now nginx                         # converged: enabled+active;
                                                            #   survives line 161 (footprint disjoint)
190  # [ -f /opt/web1-docroot.tar.gz ] || curl … "$SEED"    # converged: your guard holds;
                                                            #   survives line 161 (footprint disjoint)
191  cloud-init-per instance seed-docroot tar -xzf …        # runs: unmodeled ('tar')
plan: 3 to run (11 skipped)
```

Note that line 171 correctly does *not* survive: its guard reads exactly the file
line 161 writes. The footprint machinery gets the interesting case right, which is
the whole reason it is worth the trust it costs.

Default render, steady state, with consent typed:

```
$ dorc plan --risk-faultless-skips userdata-boothook-web.sh web1.example.net
122  until getent hosts deb.debian.org …; do sleep 1; done  # runs: wait, casts no wall
161  cat >/run/web1.conf.new <<EOF                          # runs: writes sm.dorc.File:/run/web1.conf.new
171  cmp -s /run/web1.conf.new "$CONF" || { … }             # runs: your own guard reads line 161's file
191  cloud-init-per instance seed-docroot tar -xzf …        # runs: unmodeled ('tar')
plan: 3 to run (11 skipped)
```

### Day N — drifted

Somebody flushed the firewall during an incident and never put it back, and the
nginx config was hand-edited on the box.

```
$ dorc plan --verbose userdata-boothook-web.sh web1.example.net
…
137  # $APT update                                          # converged: package index fresh
138  # dpkg -s nginx … || $APT install -y nginx             # converged: your guard holds (rc 0)
147  ufw allow 80/tcp                                       # runs: diverged (rule absent)
148  ufw allow 443/tcp                                      # runs: diverged (rule absent)
149  ufw --force enable                                     # runs: diverged (inactive)
159  # [ -d "$DOCROOT" ] || install -d …                    # converged: your guard holds (rc 0)
161  cat >/run/web1.conf.new <<EOF                          # runs: unmodeled (writes via redirect)
171  cmp -s /run/web1.conf.new "$CONF" || { … }             # runs: your own guard re-checks live
180  ( systemctl_check enable --now nginx ) || …            # verify: converged, but past 'cat' (line 161)
190  [ -f /opt/web1-docroot.tar.gz ] || curl … "$SEED"      # runs: your own guard re-checks live
191  cloud-init-per instance seed-docroot tar -xzf …        # runs: unmodeled ('tar')
plan: 8 to run, 1 to verify (5 skipped)
```

The firewall drift is caught and named *at plan time*, before anything is touched.
The config drift is not, and cannot be: the comparison that detects it is the
admin's own runtime `cmp`, and its input does not exist until line 161 runs. So the
plan says "these three lines will restore your firewall" and stays silent about the
config until the apply. That is exactly what bare sh does — no worse — but it is a
real asymmetry, and it is caused by the same thing finding-1 is about.

---

## 4. Findings and chafe points

### finding-redirect-writes-live-outside-argv  ← the big one

The oracle contract is argv-keyed by design and by law: the engine parses no tool
argv, and the oracle's own argparse is the sole entity-resolver
(`identity-declared-never-inferred`). A redirect target **never reaches argv**.
`cat >f <<EOF`, `printf … >f`, `cmd >>f`, `tee f` — the single most common way shell
mutates a file is structurally outside the mechanism that describes mutations.

Nothing in the current design can fix that from the oracle side. An `fs` stdlib
author cannot write a `cat__disturbs` that names `/run/web1.conf.new`, because
`cat`'s argv on that line is *empty*.

The proposal, and I think it is both cheap and squarely in-doctrine: **the engine
derives a `sm.dorc.File` disturbance from every output redirect it parses.** Three
reasons it is not a doctrine breach:

- It is a *widening*, and widening is always the safe direction — the same
  reasoning that lets `disturbance_reaches_only` widen a footprint. It can never
  license an elision; it can only make an existing total wall into a narrow one.
- It is referent-agnostic: it reads sh structure, not tool semantics. POSIX, not an
  author, guarantees that `>f` writes `f`.
- It needs no vouch, because it grants no license. Today an unmodeled command is ⊤
  and walls everything; this makes an unmodeled command *with a redirect* wall only
  what its redirect touches, and leaves the rest of its ⊤-ness intact.

What it does not do: license eliding line 161. Content-matching a heredoc against a
file is a value-prediction requiring a real read, and the admin already wrote that
read — it is line 171.

Without this, the write-if-changed idiom **self-walls**: you write a candidate file,
and the act of writing it poisons every fact below. That idiom is not exotic. It is
how careful people write config into shell scripts, and it appears in this book
because I could not find a more honest way to write the same thing.

### chafe-loops-must-be-wall-transparent

Same chafe as `pivot-vps-standup.note.md` names, and it bites harder here: line 122
is the book's **first executable line**. A `StatusIterated` condition blocks
unconditionally today, so if a wait loop also *walls*, this book degrades from line
122 onward and the whole exercise is pointless. The narrowing needed is the same:
a loop whose body is pure-delay and whose condition is an oracle-modeled read never
elides and casts no wall.

Note that line 122's condition is `getent hosts`, which is the canonical
*non-hermetic* read — live DNS, warm caches. That is fine and, I think,
clarifying: the loop condition must NOT license anything (it cannot, and
`hermeticity-precondition` says so), it only needs to be recognised as read-only so
the loop is known not to mutate. Wall-transparency and licensing are separable here,
which is what makes the narrowing cheap.

### chafe-cloud-init-per-is-a-gating-wrapper

`wrapper-law` describes a peeling wrapper as one that "execs the REMAINDER verbatim,
**once**, locally". `cloud-init-per` conditionally does not exec at all: if the sem
file exists and freq ≠ `always`, it exits 0 having run nothing. It is a *gating*
wrapper, and the law has no cell for one.

The book takes the safe reading — the oracle peels, the inner command's own
convergence governs, and the gate is invisible to analysis. That is sound but lossy
in one direction and behaviour-changing in another:

- **Lossy:** on a day the sem exists, the line is a guaranteed no-op and Dorc cannot
  know it. Worst case it guards or runs a line that was never going to do anything.
  Over-execution, sin #2, acceptable.
- **Behaviour-changing:** if Dorc *elides* a `cloud-init-per` line because the inner
  command is converged, the sem file is never written, so the tool's own rationing
  silently downgrades to Dorc's. Since Dorc's is state-keyed and strictly better,
  this is arguably a feature — but it is a change to a third-party tool's bookkeeping
  made by us, on a line whose author was asking for something else, and it should be
  a conscious ruling rather than an emergent one.

The tempting fix — `cloud_init_per__is_converged()` returning 0 when the sem exists —
is *sound* (the tool's own source guarantees the no-op) but it makes a wrapper oracle
into a state-claiming oracle, which cuts across "no wrapper oracle ever mentions a
kind". Flagging, not resolving.

### chafe-privilege-asymmetry-across-deliveries

Day zero the payload runs as root, by construction. Day N it runs as whoever ssh'd
in. `cloud-init-per` hard-fails `must be root`; `ufw`, `systemctl enable`, `apt-get
install` and every write under `/etc` do too. This book sidesteps it by assuming
`root@` on day N, which is what both DigitalOcean and Hetzner hand you by default —
but that is a dodge, not an answer, and the offline-artifact story owes a real one.

The `cloud_init_per__predict` body models it honestly (`[ "$(id -u)" = 0 ] || return
2`) — declining rather than guessing — which is at least the right *shape*: privilege
is an early-bound host fact, computed once, threaded through. Three independent tools
in turn C's evidence do exactly that with a `$SUDO` prefix variable. Whatever the
answer is, it is that shape and not a per-line decision.

### chafe-compile-has-no-size-budget-lane

The book carries the channel caps as a comment because there is nowhere better to
put them. A `dorc compile` that silently emits 17 KiB for an EC2 target has produced
a file that will be rejected at create time, on the cloud's side, with a message
that will not mention Dorc. Under `KNOBS:kWARN-rich` this is exactly the class of
detection worth building while the analysis is in hand.

But there is nothing to *probe* — the cap is a fact about a channel Dorc never
touches, and `rul-capability-probing-per-feature` says capability-probing is Dorc's
job per-feature, per-host, which does not reach a cap belonging to an API the book
merely mentions. So it has to be told. The minimum honest version is an admin-typed
number (`dorc compile --max-bytes=16384`), not a cloud database; the maximum is a
warning that names the byte count and shuts up. Recommending the minimum.

---

## 5. The ledger

Counting the 14 tool-sites; `attention-lines` counts everything facing the user in
the default render.

| world | ran | verified | elided/omitted | attention-lines | spent |
|---|---|---|---|---|---|
| day zero, raw (payload, no Dorc) | 14 | 0 | 0 | n/a — nobody is watching | nothing |
| day zero, compiled | 9 | 5 | 0 | n/a — nobody is watching | one `dorc compile`; +6.1 KiB of channel budget |
| day N steady, no footprints | 5 | 1 | 8 | 6 | one probe pass |
| day N steady, with footprints + flag | 3 | 0 | 11 | 4 | the same, plus typed consent |
| day N drifted (firewall flushed) | 8 | 1 | 5 | 9 | one probe pass |

**Gained, day zero:** safety, not attention. Five unguarded mutations stop firing
blind on every one of this machine's boots, using tested checks instead of
hand-rolled ones, and the report lane records what declined and why. There is no
attention to save because there is no human in the loop at boot — which is precisely
why the *safety* half is the half that matters in this cell.

**Gained, day N:** the payload becomes a book. The same file that built the machine
now reports drift on it and repairs only what drifted. Nothing was migrated, no
second artifact exists, and the firewall drift above was named before a single byte
was written.

**Gained, structurally:** the channel's own failure mode gets an answer. "Log errors
but proceed" plus "never re-run this" is a hole with a documented shape, and a
re-runnable guarded artifact fills it exactly.

**Not gained, and worth repeating:** the day-zero compiled face buys no attention and
never will — there is no measured world to justify a deletion. Any pitch for the
offline artifact that leans on the attention product is selling the wrong half.

**Not gained:** anything about `cat`-through-a-redirect (finding-1), `tar`, or
`curl`. And the boothook cannot own the hostname — cloud-init's own
`cc_set_hostname` runs *after* boothooks, in the same stage, and would overwrite it.
That is not a Dorc limitation; it is a fact about the cell, and a book in this cell
that tries to set a hostname is simply wrong.

**Spent:** one wrapper oracle (~25 lines, publishable — every boothook in the world
will meet `cloud-init-per`), the discipline of keeping marks out of the book body,
and 6.1 KiB of a 64 KiB channel budget.
