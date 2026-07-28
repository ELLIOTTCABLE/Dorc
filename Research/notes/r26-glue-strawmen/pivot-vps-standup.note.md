# Companion note — `pivot-vps-standup.sh`

> Round r26 (ops-glue-residue), writing phase, builder B1 (cloud lifecycle).
> The book is IMAGINATION-TIER frozen evidence and must never be executed.
> This note is where the honest accounting lives.

---

## 1. What is real, and what is invented

### Real (primary docs, all fetched 2026-07-28)

| Claim in the book | Source |
|---|---|
| `doctl compute droplet create <name> --region --size --image --ssh-keys --user-data-file --tag-names --wait`; `--size` and `--image` are `(required)`; `--wait` defaults `false` | <https://docs.digitalocean.com/reference/doctl/reference/compute/droplet/create/> (generated 23 Jul 2026 from doctl v1.164.0) |
| `--wait` = "Instructs the terminal to wait for the **action** to complete"; droplet status is only `new`/`active`/`off`/`archive` | same page + <https://docs.digitalocean.com/reference/doctl/reference/compute/droplet/get/> |
| `--wait` does not cover cloud-init; users have asked for it and it is not there | <https://github.com/digitalocean/doctl/issues/773> ("Wait for initialization, with cloud-init") |
| `doctl compute droplet get <id\|name> --format PublicIPv4 --no-header` | droplet/get reference (worked example: `--format Name,ID,PublicIPv4`) |
| `doctl compute domain records create <domain> --record-type A --record-name … --record-data … --record-ttl` (TTL default 1800) | <https://docs.digitalocean.com/reference/doctl/reference/compute/domain/records/create/> |
| **Rebuild keeps the IP and changes the SSH host key** | <https://docs.digitalocean.com/products/droplets/how-to/rebuild/> — "when you rebuild a Droplet, the IP address is retained", plus the known-hosts mismatch banner printed verbatim in the doc |
| Droplet names are **not** unique; `delete <name>` with ≠1 match is ambiguous | <https://www.digitalocean.com/community/questions/doctl-compute-droplet-delete-xyz-when-there-are-1-droplets-with-the-name-xyz> |
| doctl exit code on not-found is undocumented (community-consistent at 1) | absence in the reference pages; corroborated by digitalocean/doctl#1635 and CI logs in the wild |
| User-data cap 64 KiB, plain text | <https://docs.digitalocean.com/reference/api/reference/droplets/> — "may not exceed 64 KiB in size" |
| `cloud-init status --wait`; eight status values incl. `degraded done` | <https://docs.cloud-init.io/en/latest/howto/status.html> |
| `--wait` can block forever: a failing `bootcmd` leaves status `running`, not `error` | <https://github.com/canonical/cloud-init/issues/4505> |
| Everyone punts host-key verification on new machines | Terraform: "Because the SSH connection type is most often used with newly-created remote resources, SSH host key validation is disabled by default" — <https://developer.hashicorp.com/terraform/language/resources/provisioners/syntax>; nixos-anywhere hardcodes `UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no` (r26 turn-03 evidence) |
| Port-open ≠ login-works | an author in the wild wrote two wait loops with two predicates and commented the difference (r26 turn-03, `[B-kthw-vm-ssh-setup-pivot-2026]`) |

### Invented (imagination-tier; every one is on the round's sanctioned palette)

- `dorc-run` shebang execution. Design prose only; no binary exists.
- The `ssh` oracle's **connection-dance arm** minting `sm.dorc.SshEndpoint:…@reachable`
  — sanctioned by `ack-connection-dance-oracles-core`, including its rider.
- `: transits epoch` as a mark verb classing `droplet create` as an epoch transit,
  under the transit-relative epoch law (a *converged* transit elides and casts no
  boundary; only a *firing* one does).
- `: lends scope` on the `ssh` wrapper. **This one is my closest approach to the
  palette's edge** — see chafe-1.
- `--verbose` plan renders below, and the `omitted` count in the plan tally.
- Kind names `sm.doctl.Droplet`, `sm.dorc.SshEndpoint`: minted here, on the
  deliberately-invalid `sm.` TLD, so nothing leaks into a real vocabulary.

---

## 2. Why this shape

**No host argument.** The pivot's defining constraint is that the book's first
lines have no host to run on — the host is what they *make*. The only sh spelling
that survives that is the one everybody already writes: the file is controller-local
by default and `ssh` is how it reaches anywhere else. A `dorc plan book.sh <host>`
invocation cannot express the standup at all, because there is no idiom in sh for
"this line runs back home". The dotfiles story in `USER_STORY.md` already licensed
the no-host form ("the target is the machine you are sitting at"); the pivot needs
it.

**The guard is the admin's, and it is wide on purpose.** Line 192 folds nine lines
dead on the strength of one narrow fact. That fact — an sshd answered a
`BatchMode=yes` login — licenses precisely one cell. It does not say the droplet is
the *right* droplet, that cloud-init finished, that the DNS record points here, or
that a key was ever installed. The admin's guard asserts all of that anyway, and
that is *correct*, because the guard is their line, their judgment, and their name
on the attribution. The engine's contribution is the narrow measurement and the
plain value-flow that follows from it; the wide inference is authored, not derived.
Getting that split wrong in either direction is the failure mode: an engine that
minted "reachable ⇒ provisioned" would be lying, and an engine that refused to fold
a false `if` would be useless.

**Waits are placed by observability, not by taste.** Two of the three waits in this
pair of books live inside the payload (`userdata-boothook-web.sh` §1) because the
awaited facts — a resolver answering, a dpkg lock clearing — are observable from
inside the box, and an in-artifact wait costs one connection however long it spins.
The two here are on the controller because "is this host reachable" is definitionally
not answerable from inside that host. That is the whole rule, and it lands on the
right side of the perf doctrine (never let a network boundary participate in
iteration) without needing a new one.

**`--wait` is not readiness, and the book says so three times.** DigitalOcean's
`--wait` tracks the create *action*. `until ssh … true` tracks sshd. `cloud-init
status --wait` tracks cloud-init. They are three different questions with three
different answers and the ecosystem conflates them constantly; the book spends three
lines refusing to.

---

## 3. Rendered plans

> Illustrative. The render format is not settled; what the renders obey is:
> the plan is the whole book in original order, non-executing lines are
> present-but-commented, and every surviving line carries its reason.

### Day zero — nothing exists

```
$ dorc plan --verbose pivot-vps-standup.sh
 36  set -eu
 38  DROPLET=web1
 40  FQDN=web1.example.net
 48  SSH="ssh -o BatchMode=yes -o ConnectTimeout=5 -o StrictHostKeyChecking=accept-new"
192  if ! ssh … "root@$FQDN" true; then                # runs: not reachable (measured 08:14:02)
194     doctl compute droplet create "$DROPLET" \
194        --region … --user-data-file ./build/ud-web1.txt --wait
194                                                    # runs: diverged (0 droplets named 'web1')
206     IP=$(doctl compute droplet get "$DROPLET" --format PublicIPv4 --no-header)
206                                                    # runs
208     doctl compute domain records list "$DOMAIN" … \
208        || doctl compute domain records create …    # runs: unmodeled ('doctl' verb 'domain')
221     ssh-keygen -R "$FQDN" >/dev/null 2>&1 || true  # runs: unmodeled ('ssh-keygen')
222     ssh-keygen -R "$IP"   >/dev/null 2>&1 || true  # runs: unmodeled ('ssh-keygen')
235     until ssh … "root@$FQDN" true; do sleep 5; done
235                                                    # runs: wait (a loop never elides)
244     timeout 600 ssh … cloud-init status --wait || true
244                                                    # runs: unmodeled ('timeout')
245  fi
255  ssh … certbot certonly … -d "$FQDN"               # runs: new epoch (line 194)
257  ssh … systemctl enable --now certbot.timer        # runs: new epoch (line 194)
258  ssh … systemctl enable --now unattended-upgrades  # runs: new epoch (line 194)
267  ssh … install -m 600 -D /dev/stdin /etc/restic/repo.pass
267                                                    # runs: new epoch (line 194)
276  curl -fsS -o /dev/null "https://$FQDN/"           # runs: unmodeled ('curl')
plan: 12 to run (0 skipped)

hint: 'timeout' (line 244) is unmodeled: it is a peeling wrapper and it walls
      1 downstream site; a wrapper oracle for it would recover them
hint: 'ssh-keygen' (lines 221, 222) is unmodeled: it degrades 6 downstream sites
```

Day zero is the honest floor and it should look like this: nothing measurable,
nothing elided, one extra plan-time round-trip spent proving the machine is absent.
The value delivered is not elision — it is that the same file, unchanged, is also
what runs tomorrow.

### Day N — steady state

```
$ dorc plan --verbose pivot-vps-standup.sh
 36  set -eu
 38  DROPLET=web1
 40  FQDN=web1.example.net
 48  SSH="ssh -o BatchMode=yes -o ConnectTimeout=5 -o StrictHostKeyChecking=accept-new"
192  # if ! ssh … "root@$FQDN" true; then             # omitted: branch dead
194  #    doctl compute droplet create …              #   (sm.dorc.SshEndpoint:
206  #    IP=$(doctl compute droplet get …)           #    root@web1.example.net@reachable)
208  #    doctl compute domain records list … || …
221  #    ssh-keygen -R "$FQDN" …
222  #    ssh-keygen -R "$IP" …
235  #    until ssh … true; do sleep 5; done
244  #    timeout 600 ssh … cloud-init status --wait
245  # fi
255  # ssh … certbot certonly … -d "$FQDN"            # converged: lineage present
                                                      #   (certbot__is_converged, book:165)
257  # ssh … systemctl enable --now certbot.timer     # converged: enabled+active
258  # ssh … systemctl enable --now unattended-upgrades  # converged: enabled+active
267  ssh … install -m 600 -D /dev/stdin /etc/restic/repo.pass
267                                                   # runs: unmodeled ('install' reading stdin)
276  curl -fsS -o /dev/null "https://$FQDN/"          # runs: unmodeled ('curl')
plan: 2 to run (8 omitted, 3 skipped)
```

And the default, non-verbose render — the attention product:

```
$ dorc plan pivot-vps-standup.sh
267  ssh … install -m 600 -D /dev/stdin /etc/restic/repo.pass
267                                                   # runs: unmodeled ('install' reading stdin)
276  curl -fsS -o /dev/null "https://$FQDN/"          # runs: unmodeled ('curl')
plan: 2 to run (8 omitted, 3 skipped)
```

Two lines, both of which genuinely should run every day. Note *how* the eight-line
region left: not by eight vouches, but by one measured fact and one dead branch —
`omit`, on a value-flow license, with no per-line oracle coverage required inside
the region at all. That is the cheapest verdict in the vocabulary and boot regions
are where it pays most, because boot sections sit at the *top* of books, which is
the worst possible wall real-estate.

The trade, stated plainly: on a day the machine answers, **nothing inside the region
is checked**. If somebody hand-deleted the DNS A record, the book will not notice
until `curl` fails at line 276. That is not a regression — it is exactly what the
admin's bare-sh version did — but it is not drift-healing either, and calling it
drift-healing would be a lie. Finer oracles inside the region do not fix it; the
region is not *entered*. Only a narrower guard would, and writing one is the admin's
call, not ours.

### Rebuild day — the machine is there, its identity is not

The admin rebuilt the droplet by hand (`doctl compute droplet-action rebuild <id>
--image debian-13-x64`). DigitalOcean keeps the IP and changes the host key.

```
$ dorc plan --verbose pivot-vps-standup.sh
 36  set -eu
 48  SSH="ssh -o … -o StrictHostKeyChecking=accept-new"
192  if ! ssh … "root@$FQDN" true; then               # runs: not reachable
                                                      #   (host key verification failed —
                                                      #    accept-new does not re-pin a CHANGED key)
194  #    doctl compute droplet create "$DROPLET" …   # converged: sm.doctl.Droplet:web1@exists
                                                      #   → transit does not fire; no epoch boundary
206     IP=$(doctl compute droplet get … )            # runs
208     doctl compute domain records list … || …      # runs: unmodeled ('doctl' verb 'domain')
221     ssh-keygen -R "$FQDN" … || true               # runs: unmodeled ('ssh-keygen')
222     ssh-keygen -R "$IP" … || true                 # runs: unmodeled ('ssh-keygen')
235     until ssh … "root@$FQDN" true; do sleep 5; done   # runs: wait
244     timeout 600 ssh … cloud-init status --wait    # runs: unmodeled ('timeout')
245  fi
255  ssh … certbot certonly … -d "$FQDN"              # runs: scope unmeasurable at plan time
257  ssh … systemctl enable --now certbot.timer       # runs: scope unmeasurable at plan time
258  ssh … systemctl enable --now unattended-upgrades # runs: scope unmeasurable at plan time
267  ssh … install -m 600 -D /dev/stdin …             # runs: scope unmeasurable at plan time
276  curl -fsS -o /dev/null "https://$FQDN/"          # runs: unmodeled ('curl')
plan: 11 to run, 1 skipped
```

Read the two independent reasons carefully, because they are the point of this
render:

1. The **create elides** — one droplet named `web1` exists, `doctl__is_converged`
   answers 0, and because that transit does not fire, it casts no epoch boundary.
   The transit-relative law works: this is not a machine-build day, so nothing
   downstream is degraded *by the transit*.
2. Everything downstream runs anyway — because the controller could not open a
   session to the box at plan time, so no in-host fact could be measured at all.

The plan repaired one of those and could not repair the other. It could not, and
should not, have repaired the second: `ssh-keygen -R` is a mutation of the
controller's own trust store, and the probe phase does not mutate. Even a
controller-local one. Especially a controller-local one — a probe that silently
rewrites `known_hosts` is a probe that launders a security decision into a
performance optimisation.

Consequence worth naming: **a host-key mismatch costs the whole tail of a pivot book
its elisions, once.** The next plan, after the apply repaired the entry, is back to
the steady-state render.

---

## 4. Chafe points

### chafe-ssh-lends-scope-not-dimensions

`27C`'s dimension vocabulary is (user · fs-view · netns · ρ) — four axes *within one
host*. `ssh` does not lend any of them; it replaces the entire substrate. I spelled
that as a mapped lend of a fifth dimension, `scope`, on the reasoning that a scope
map *subsumes* the four (over there they are whatever they are, and the measure-in-
denoted-context mechanism resolves them by measuring there). The enumerate-every-
dimension law is satisfied in spirit — nothing is silently defaulted — but not in
letter, since four dimensions go unmentioned.

**This is the single most consequential spelling decision in the book, and it is a
conductor call, not mine.** Three candidate readings:

- *scope subsumes* (what the book does): one mark, and the four axes are answered by
  measurement rather than by claim. Cheapest; the enumerate law becomes "enumerate
  the axes that survive the entry", which needs stating.
- *scope is a fifth sibling*: `ssh` must then also mark the four, presumably as
  ⊤-relative-to-here. Verbose and, I think, meaningless — "the remote user is
  unknown to me" is not a fact about anything.
- *scope is not a dimension at all* but the fact-coordinate's context slot, and
  `ssh` is a scope-entry construct rather than a wrapper. Cleanest conceptually,
  largest machinery ask, and it collides with the attribution-scope re-entry trigger
  (`rul-attribution-is-controller-minted`) that a two-scope book fires anyway.

I did not escalate and stop, because the book is writable under all three readings
and only the mark spelling differs. But the third reading is the one I would bet on,
and it is not free.

### chafe-book-lines-consume-the-artifact-channel

Line 267 pushes a secret by redirecting a controller-local file into an `ssh` site's
stdin. Under the shipped invocation shape (`ssh host 'sh' -s < artifact`) a book line
that itself reads stdin eats artifact bytes. Turn B flagged this as an
engine-relevant escalation with a live specimen; here is a second one, and it is not
an exotic construct — "pipe a secret in over stdin so it never touches disk on the
controller *or* in an argv" is the correct way to do this.

The book cannot work around it (any workaround puts the secret somewhere worse), and
the escalation is already banked and human-owned, so this note only adds: the
copy-then-exec fix (`cat - >tmp && exec sh tmp`) is not optional for pivot-shaped
books, it is load-bearing. Any book that pushes material to a machine it just built
will hit this on its first useful line.

### chafe-loops-never-elide-but-should-not-wall

`until ssh … true; do sleep 5; done` (line 235) is a `StatusIterated` condition,
which blocks unconditionally today — correctly, since no single rc reproduces a
per-iteration sequence. But the *consequence* the book needs is narrower than the
rule delivers: the loop must always run (never elide — a host reachable at probe
time may be gone at apply time), and it must cast **no wall**, because its body is
`sleep` and it mutates nothing. Today an unmodeled loop is a total wall, so a book
with three wait loops in it degrades three times over for no reason.

The minimal claim that fixes it, and I believe it is genuinely minimal: *a loop whose
body is pure-delay and whose condition is an oracle-modeled read is wall-transparent
and never elidable.* Not a new license — a narrowing of an existing pessimism. Both
books lean on it; without it, `userdata-boothook-web.sh` degrades from its first
executable line onward.

### chafe-controller-minted-identity-is-left-on-the-table

Every reference implementation punts host-key verification, and the reason is
structural: whoever verifies the key did not create the machine. Here, *the
controller created the machine*. It knows the create action's ID, the droplet ID,
and the moment of creation; cloud-init's `phone_home` module will POST
`pub_key_rsa`, `pub_key_ecdsa` and `pub_key_ed25519` alongside `instance_id`,
`hostname` and `fqdn` if asked. The pieces of a real bind-at-creation are all
present and nobody assembles them.

This book does not either — `phone_home` needs a listener the controller does not
have, and the POST is unauthenticated (relocating the trust cell rather than closing
it). What the book does instead is *narrow* the punt: `accept-new` rather than `no`,
and `ssh-keygen -R` only on the path where the controller itself just rebuilt the
machine. That is strictly better than the ecosystem's answer and strictly worse than
what is actually available. Flagging for the synthesis note as an unclaimed
differentiator, not as this round's work.

### chafe-timeout-and-ssh-keygen-are-stdlib-holes

Three tools appear in every pivot script on earth and none is describable today:
`timeout` (a peeling wrapper — trivially oracle-able, and it walls whatever it
wraps), `ssh-keygen -R` (an idempotent removal with a cheap check: `ssh-keygen -F`),
and `curl` (declines, correctly, but should decline *loudly* with a class). Cheap
stdlib wins with outsized effect on this book class, because these tools cluster
exactly where the walls hurt most.

---

## 5. The ledger

Counting the 14 tool-sites (the housekeeping lines and the oracle definitions always
show; `attention-lines` counts everything still facing the user in the default,
non-verbose render).

| world | ran | verified | elided/omitted | attention-lines | spent |
|---|---|---|---|---|---|
| day zero, bare sh (no Dorc) | 14 | 0 | 0 | 14 | nothing |
| day zero, Dorc | 12 | 0 | 0 | 12 | one plan-time round trip; two hints |
| day N steady, bare sh | 14 | 0 | 0 | 14 | nothing |
| day N steady, Dorc | 2 | 0 | 11 | 2 | one ssh probe + one certbot probe |
| rebuild day, Dorc | 11 | 0 | 1 | 11 | one refused ssh probe |

**Gained, day zero:** nothing measurable, and the note should say so. The book is one
file rather than a `.tf`/`.yaml` plus a prep script, which is the all-in-one thesis
paying out at authoring time rather than at run time.

**Gained, steady state:** twelve lines of attention down to two, and — the part that
matters more than the count — the two that remain are the two that *should* remain.
A book whose steady-state render is "push the secret, check the site answers" is a
book you can read in one second and believe.

**Gained, structurally:** the pivot works at all. `ack-pivot-must-support` is
satisfied by ordinary machinery — a wrapper oracle, a connection-dance arm, dead-branch
value-flow — with no pivot-specific engine feature. The one genuinely new thing is
the transit class, and it earns its keep on rebuild day rather than on steady days.

**Not gained:** anything inside the omitted region on a steady day. Anything at all
on a host-key-mismatch day. And `curl`, `timeout`, `ssh-keygen` and `doctl`'s
non-droplet verbs, until somebody describes them.

**Spent:** roughly 120 lines of oracle for three tools, of which the `doctl` and
`ssh` ones are stdlib-shaped (written once, reused by everyone) and only the
`certbot` judgment is genuinely local. Plus one plan-time ssh round trip per run,
which the bare-sh version also paid — it just called it line 192.
