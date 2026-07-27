# 26E — the r26 live build-target (operator card)

AI-authored (Opus ops-builder), provisioned 2026-07-27 at conductor direction under a
single-instance human authorization. This is the box `26D` §4/§6 anticipated: one throwaway
Debian-12 VPS, left RUNNING as the overnight build-target. Terse by intent — this note exists
so a later agent can find and reach the box without re-deriving anything.

## §1. The box

| field | value |
|---|---|
| instance id | `48f8dd1f-f62f-47c1-9a84-22151b48971f` |
| label / hostname | `dorc-r26-0727-073315-513d` |
| IPv4 | `140.82.10.231` |
| region | `ewr` (New Jersey) |
| plan | `vc2-1c-2gb` — 1 vCPU, 2048 MB, 55 GB NVMe |
| os | Debian 12 x64 (bookworm), `os_id 2136` |
| tag | `dorc-r26` |
| created (UTC) | 2026-07-27T07:33:16Z (active + SSH-reachable within 60 s) |
| cost | $10/mo ≈ **$0.0137/hr** (~$0.33/day) |
| pristine snapshot | `ff770de9-7544-4163-af3e-32be8fcb58d0` (`complete`, 55 GB, taken 07:34:47Z before any mutation) |

The 2 GB tier is a floor, not a preference: the acceptance book installs
grafana+prometheus+HA (`255` §8), which OOMs the 1 GB tier `252` originally specified.

## §2. Reaching it

```sh
ssh -F Research/trial/apply/ssh_config -i ~/.ssh/dorc-r25 -T root@140.82.10.231
```

Verified working from the Windows/git-bash controller, 2026-07-27. The `-F` is load-bearing,
not decoration — `26D` §4's usekeychain scar: bare `ssh` on this controller reads the human's
`~/.ssh/config`, hits macOS-only `UseKeychain`, and dies rc=255 before connecting. The path is
worktree-relative; absolutize it, or `-F` the same file from wherever you are.

`trial/apply/apply-run.sh` needs no arguments beyond the host — its `SSH_KEY` already defaults
to `~/.ssh/dorc-r25` and its `SSH_CONFIG` to the sibling `ssh_config`:

```sh
Research/trial/apply/apply-run.sh apply-run 140.82.10.231 <plan.sh>
```

Keys (controller-local, reused from r25 — the box carries the r25 pubkey, injected by
`vultr.sh` via cloud-init): private `~/.ssh/dorc-r25`, public `~/.ssh/dorc-r25.pub`
(ed25519, passphrase-less by construction). That private key is mode 0644, which git-bash's
OpenSSH tolerates but a real Linux controller will not — a WSL or container-side agent must
copy it into its own filesystem and `chmod 600` first, since `/mnt/c` permissions do not
respond to `chmod` under a default drvfs mount. The Vultr API key lives in `~/.temp/vultr.env` as
`VULTR_API_KEY` — sourced into env by `vultr.sh` only, never printed. It authenticated on the
r25-vintage credential; no rotation was needed.

## §3. Recon (pristine, pre-mutation)

Kernel `6.1.0-50-amd64` (Debian 6.1.176-1) · `/bin/sh` → `/usr/bin/dash` (the strict-POSIX
target `apply-run.sh` assumes) · apt 2.6.1 · dpkg 1.21.23 · OpenSSH 9.2 / OpenSSL 3.0.20 ·
1 vCPU · 1961 MB RAM + 5399 MB swap · 52 GB root, 41 GB free · python3 present · cloud-init
`status: done` (it exits non-zero while reporting done — cosmetic, ignore) · clock UTC.

The disk headroom means the `255` book's installs are nowhere near a constraint; RAM is the
only tight resource, hence §1's floor.

## §4. Blast radius

`vultr.sh` now tags and prefixes `dorc-r26` (retagged this session, along with the plan floor).
Every mutating path runs `_assert_tagged` and dies on anything lacking that tag, and
`_resolve_id` resolves an IPv4 only among tagged boxes — so a mistyped IP falls through as a
bogus id and no-ops rather than reaching a neighbour. The account does hold one unrelated,
untagged, long-lived instance that is NOT ours and must never be touched; the tag guard is what
keeps that true, so never widen `TAG`, and never point `destroy` at a bare IP you have not
confirmed is `dorc-r26`.

`restore` is available in the script but is NOT authorized for agents: the snapshot exists so
damage is recoverable-in-principle by a human, not so an agent can roll the world back
unilaterally. The whylog is default-ON and records raw host metadata unsanitized
(`AID-NEEDS:law-whylog-is-sensitive`) — acceptable here precisely because the box is throwaway.

## §5. Teardown — HUMAN-ACK REQUIRED, do not run unprompted

The box is deliberately left running. No agent may destroy it without a fresh, explicit,
single-case human instruction. When that comes:

```sh
Research/trial/vultr/vultr.sh destroy 48f8dd1f-f62f-47c1-9a84-22151b48971f
Research/trial/vultr/vultr.sh destroy-all   # instance + snapshot, everything dorc-r26
```

`destroy-all` also deletes the §1 snapshot, so it is the true end-of-round reap, not a retry
tool. `Research/trial/vultr/vultr.sh status` is the read-only view (live instances, snapshots,
rough accrued spend) and is always safe.
