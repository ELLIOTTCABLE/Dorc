# r26 live blood — the first time Dorc touched a real machine

2026-07-27. Target: the `26E` box, `root@140.82.10.231` (Debian 12.15, kernel 6.1.0-50-amd64,
systemd running), reached over real internet ssh from the Windows/git-bash controller. Engine:
`ai/r26-livetest` at the tip that re-derived `../predictions.md` §6 — those baselines were
committed BEFORE any of this ran, and the git order is the evidence.

Every artifact in this directory is raw captured output. Nothing was re-run to look better, and
nothing that disagreed with a prediction was reconciled.

| file | what it is |
|---|---|
| `00-recon-pristine.txt` | the box's world before any dorc mutation |
| `01-plan-1.sh` / `.stderr` | the plan built from a real probe of the pristine world |
| `02-apply-1.stdout` / `.stderr` | the first live apply — real `apt-get`, real nginx |
| `03-plan-2.sh` / `.stderr` | the re-plan against the now-converged world |
| `04-plan-ceiling.sh` / `.stderr` | the ceiling variant against the converged world |
| `05-apply-2.stdout` / `.stderr` | the re-apply, for no-op-ness |

## §1. What happened, in order

| step | rc | wall | summary |
|---|---|---|---|
| plan 1 (pristine) | 0 | ~1.0 s | `sites=16 elide=0 omit=0 guard=0 run=16` |
| apply 1 | 15 | 10.4 s | nginx installed and started; failed at the book's `curl` check |
| plan 2 (converged) | 0 | 1.0 s | `sites=16 elide=0 omit=0 guard=0 run=16` |
| plan 3 (ceiling, converged) | 0 | 1.4 s | `sites=15 elide=4 omit=4 guard=0 run=7` |
| apply 2 | 15 | 5.7 s | materially a no-op; failed at the same `curl` check |

Both plan summaries match their hermetic baselines exactly, and so does the ceiling. The transport
half of r26 works: a probe was rendered, shipped over ssh, executed on a machine that was not this
one, and its real records came back through admission and built a plan — with no authored fixture
anywhere in that chain. The whole round trip is about a second.

## §2. Findings

**fnd-recon-note-overstated-absence.** `26E` §3 records the box as having `curl`, `jq` and `nginx`
absent. At first contact only `nginx` was absent; `ca-certificates`, `curl` and `jq` were all
installed (`00-recon-pristine.txt`). Whether the original recon was wrong or something installed
them in the eleven hours between provisioning and this run is not determinable from here. It did
not affect the plan summaries, because the book's leading `apt-get update` walls every site below
it regardless of world — which is the one case where being wrong about the world is free.

**fnd-smoke-book-never-reloads-nginx.** Both applies failed, at the same line, for a reason that is
the book's and not Dorc's. `smoke-book.sh` drops its vhost with `cp`, then runs `systemctl enable
nginx` and `systemctl start nginx`, then curls port 8088. But nginx's own postinst has already
STARTED the daemon by the time the `cp` lands, so `systemctl start` is a no-op on a running unit,
nothing ever reloads the config, and port 8088 is never listened on. Verified directly on the box:
the conf file is present and correct, nginx is `active`, and `ss -ltn` shows 22 and 80 only.

This is an ordering defect that only a live run could surface — every hermetic render of this book
is unaffected, because nothing hermetic executes it. The book needs a reload (`systemctl reload
nginx`, or `restart`) after the config drop. **The book is deliberately NOT fixed here**:
`predictions.md` is pre-registered against these exact bytes, and quietly editing the artifact
after measuring it is the one thing a pre-registration forbids. It is a defect to fix in a
commit of its own, with the ledger updated alongside.

Dorc's own behaviour through this was correct and is worth stating plainly: it ran the book
faithfully, did not mask the failure, reproduced the remote exit status through the sentinel, and
exited 15 (`transport-apply-failed`) rather than claiming success.

**fnd-converged-plan-is-byte-identical.** Plan 2 is byte-for-byte plan 1. This is exactly what
`predictions.md` §6 said would happen and is the round's least satisfying true result: the book's
bare `apt-get update` caps every site below it, and the guard tier that used to soften that is
currently returning nothing (`del-authored-coordinate-voids-guard`). So the book as written has NO
observable convergence signal — pristine and converged are indistinguishable from the summary, the
artifact, and the decision digest alike.

**fnd-ceiling-cascade-holds-live.** The convergence signal that does exist showed up exactly where
the re-derived baseline put it. With the one `apt-get update` line removed, the same converged box
yields `elide=4`: all four `dpkg -s x || apt-get install -y x` lines fold, against a real probe of
a real machine. The hermetic number and the live number are the same number. That is the strongest
single result in this directory — the ladder cascade is not an artifact of hand-authored records.

**fnd-apply-two-is-materially-inert.** The second apply installed nothing. Its entire stdout is
five `Hit:` lines and `Reading package lists` — no `Unpacking`, no `Setting up`. The `cp`s re-ran
idempotently, `systemctl enable` no-opped, and only the broken reachability check failed. Re-apply
safety holds.

## §3. Rough edges met on the way

- Every transport diagnostic surfaced as an unwritten loom placeholder:
  `apply: error[transport-apply-failed]: [unwritten: transport-apply-failed]`. The exit code is
  informative and the echoed host stderr above it is what actually told the story. A first-time
  user meeting this would learn nothing from the sentence itself.
- `dorc --help` documents none of the transport surface. `--host`, `--plan`, `--accept-new`,
  `--ssh-config`, `--connect-timeout`, `--probe-timeout` and `--apply-timeout` all parse and all
  work; none appear in the help text, so the only way to discover them is to read `cli/src/lib.rs`.
- Dorc has no `-i`/identity flag, by design (the user's ssh config is the credential plane). The
  `Research/trial/apply/ssh_config` in-repo passes its key via `ssh -i` on the command line, so it
  cannot be handed to `--ssh-config` as-is; this run used a copy with an `IdentityFile` line added.
- The `26E` usekeychain scar does not bite Dorc: the ssh driver already passes
  `-o IgnoreUnknown=UseKeychain` in its default posture. It still bites raw `ssh` for provisioning.

**fnd-controlpath-defeats-the-transport.** A `ControlPath` in the ssh config `dorc --host` composes
with kills the session on Windows, at any `ControlMaster` setting, and the failure is the
information-free `[unwritten: transport-session-lost]` (exit 14). Isolated against a live container:
identical invocations pass with a config carrying no `ControlPath` and fail with one that does, with
the container proven alive by a control run between each pair. The mechanism is visible in raw ssh's
own words — `unix_listener: cannot bind to path .../root@localhost:22398.sock...` — the `%p`
expansion puts a colon in the socket filename, and Windows filesystems cannot hold one.

This is worth a product decision rather than a doc note. The driver already pins its
non-negotiables (`BatchMode=yes`, `ClearAllForwardings=yes`, `ForwardAgent=no`) as `-o` flags that
layer over the user's config precisely so their config cannot break the run; connection
multiplexing looks like it belongs in that list. Not changed here — the transport is another lane's
code, and this lane's job was to find out.
