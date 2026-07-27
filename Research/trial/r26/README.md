# r26 live-smoke kit

A small Debian-12 runbook plus hand-written v0.2 oracles — the material this project's plan/apply
pipe first met a real machine with. **It has now been run**, twice over: against a throwaway
container (`mise run livetest`, repeatable) and against the `26E` VPS (once, 2026-07-27; the raw
output is in `live-evidence/`). The old "no part of this has been run anywhere" caution is retired;
what replaces it is narrower and still binding — see §5.

## §1. What is in here

| path | what it is |
|---|---|
| `smoke-book.sh` | the VPS book: 11 tool-lines, `#!/bin/sh` + `set -eu`, POSIX, LF-only, root, no sudo |
| `container-book.sh` | the same book minus what an unprivileged container cannot do (§4); what `livetest` drives |
| `r26-smoke.conf`, `r26-motd` | the two config drops (port 8088, so the vhost cannot collide with the default site) |
| `oracles/*.oracle.sh` | five narrow oracles: `dpkg`, `apt-get`, `cp`, `systemctl`, `logger` |
| `records/*.txt` | hand-authored probe records per world, plus their framed forms |
| `renders/*.txt` | the five hermetic `dorc plan` baselines |
| `predictions.md` | the pre-registered ledger, its two disclosed re-registrations, and where it was wrong |
| `frame-records.sh` | turns an authored records file into the stream `dorc plan` admits (§3) |
| `render-baselines.sh` | regenerates every file in `renders/` from the current engine |
| `live-evidence/` | the raw output of the one live VPS run, and what it found |

## §2. Running it — the real spellings

Every `FIXME-cli-spelling` this file used to carry is resolved. `dorc plan --host` ships its own
probe and builds the plan from what comes back; `dorc apply --host` ships a plan you have already
read. There is no separate ship-it-yourself step any more.

### The repeatable way

```sh
mise run livetest          # provision a container, drive the whole loop, tear down (~35 s)
mise run livetest:target   # provision one and LEAVE it up, printing how to reach it
mise run livetest:clean    # remove whatever livetest left behind
```

`livetest` is the acceptance loop: plan → assert against `renders/` → apply for real → re-plan →
assert convergence → re-apply → assert inertness. It is never part of `gate`, `bless` or
pre-commit, and must not become part of them.

### Against a host you already have

```sh
cd Research/trial/r26

# The two `cp` sources are relative, so they must exist in the remote login directory.
scp -F ../apply/ssh_config -i ~/.ssh/dorc-r25 r26-smoke.conf r26-motd root@<ip>:/root/

# Plan: this SHIPS A PROBE, runs it there, and builds the plan from the real records.
mise run dorc -- plan --book=smoke-book.sh --oracle-dir oracles \
   --host root@<ip> --ssh-config <config-with-an-IdentityFile> > plan.sh

# Read plan.sh. Then apply it.
mise run dorc -- apply --host root@<ip> --ssh-config <same> --plan plan.sh

# THE POINT: repeat. The second pass is the converged-world measurement.
```

Flags that exist and work but are **absent from `dorc --help`**: `--host`, `--plan`,
`--accept-new`, `--ssh-config`, `--connect-timeout`, `--probe-timeout`, `--apply-timeout`.
`--host` is accepted on `plan` and `apply` only, and is mutually exclusive with `--results`.
A destination may carry a port — `root@localhost:2222` — which reaches ssh as `-p 2222`.

Dorc has no `-i`/identity flag on purpose: the user's ssh config is the credential plane. That
makes `../apply/ssh_config` unusable as-is with `--ssh-config`, since it expects its key from
`ssh -i` on the command line; add an `IdentityFile` line to a copy.

`Research/trial/apply/apply-run.sh` (rc/timeout capture, run directories) predates the transport
and is no longer the way in — it wrapped the manual ship-and-run steps the CLI now does itself.

## §3. Reproducing the hermetic baselines

```sh
Research/trial/r26/render-baselines.sh      # rewrites every renders/*.txt
```

That wraps `frame-records.sh` (which turns `records/<world>.txt` into the framed stream `dorc plan`
admits) and the five `dorc plan` invocations, and builds the uncommitted ceiling book itself. Site
ids are positional and shift whenever a book gains or loses a line, so re-derive them from
`dorc probe` (the `# site N: <coord>` comments) before editing any records file.

Current baselines, all re-derived on the post-kernel-arc engine (`predictions.md` §6):

| render | summary |
|---|---|
| `plan-pristine.txt` | `sites=16 elide=0 omit=0 guard=0 run=16` |
| `plan-converged.txt` | `sites=16 elide=0 omit=0 guard=0 run=16` |
| `plan-ceiling-converged.txt` | `sites=15 elide=4 omit=4 guard=0 run=7` |
| `plan-container-pristine.txt` | `sites=12 elide=1 omit=0 guard=0 run=11` |
| `plan-container-converged.txt` | `sites=12 elide=4 omit=4 guard=0 run=4` |

The live VPS run reproduced the first, second and third of those exactly, and `livetest` reproduces
the last two on every run.

## §4. Sharp edges — read before editing anything here

**Four engine behaviours around marks.** The first three are *silent*: the file still parses,
`dorc lint` still reports `0 errors`, and the annotations are simply inert.

1. **Three constructs void every mark in a marked file.** A bracket test in statement position
   (`[ x ] || return 2`), a glob pattern in a `case` arm (`-*)`, `?*)` — a bare `*)` is fine), and a
   backslash continuation whose next line starts with a redirection. The tell is `dorc strip`
   leaving the marks in place. **Check any edit with `dorc strip <file>` and confirm no
   `r26.smoke.` survives outside comments.**
2. **Any unmodeled statement before the marked line makes the site unresolvable**, so no probe
   ships and nothing can elide. `case`, assignment and `shift` are modeled; `test`, `command -v`
   and `if <command>; then` are not. Defensive gates must be spelled
   `if [ "${n-}" ... ]; then return 2; fi` with string comparisons only.
3. **A verdict body may not inspect `$?`.** `case $? in …` makes the site unresolvable. This is why
   `systemctl enable --now` is declined outright rather than modeled.
4. **`dorc plan --results` does not accept the grammar `dorc --help` documents.** Bare
   `site N effect=… rc=…` lines are refused (`host-evidence-admission-refused`, exit 12). Records
   must carry the run nonce and terminal token under a matching `book=` digest header;
   `frame-records.sh` is that transform. A trailing `#` comment on a record line parses as a record
   field and refuses the whole stream — comments must stand on their own line.

**An authored coordinate on an `is_converged` verdict currently costs you the guard.** Since the
verdict-keying change, such a site keys the coordinate its author wrote instead of a synthesized
`dorc-auto:<cmd>@converged`. Sibling cells now split correctly — but nothing on the licensing side
asks for the authored cell, so a vouch that used to buy a guard buys nothing. `predictions.md` §6
(`del-authored-coordinate-voids-guard`) has the demonstration. This is why every render here shows
`guard=0`.

**An unprivileged container cannot run the systemd half of this book.** No runtime reachable from
this project grants what systemd needs as PID 1 — `wslc run` exposes no `--privileged` and no
`--cap-add` — so `systemctl` answers "System has not been booted with systemd as init system" and
`set -eu` takes the book down there. That, plus the fact that the bare `apt-get update` is designed
to cost the book every elision, is why `container-book.sh` exists.

**The book never reloads nginx, so its own `curl` check cannot pass.** Found live, on the VPS: the
package's postinst starts nginx before the `cp` drops the vhost, and `systemctl start` no-ops on a
running unit, so port 8088 is never listened on. `smoke-book.sh` is left unfixed on purpose —
`predictions.md` is pre-registered against these bytes. `live-evidence/README.md`
(`fnd-smoke-book-never-reloads-nginx`) is the write-up.

## §5. Standing cautions

- The `cp` sources are relative; the run's cwd must hold them, or both drops resolve to nothing.
- The kit installs `nginx`, `curl`, `jq`, `ca-certificates` — light enough for the 2 GB box.
- The oracles mint throwaway `r26.smoke.*` kinds. They are not stdlib and must never be treated as
  a naming precedent; `sm.dorc.*` is reserved.
- These books are still real-command material. They are executed **only** by `livetest`, or by a
  human deliberately pointing `dorc --host` at a machine they own. The hermetic corpus's
  never-execute law is untouched, and nothing here may be run against the controller or WSL.
- Teardown of the `26E` box remains human-acked only (`26E` §5).
