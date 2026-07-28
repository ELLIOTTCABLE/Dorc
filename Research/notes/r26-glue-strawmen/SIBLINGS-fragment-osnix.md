# SIBLINGS fragment — nix/NixOS + Home Manager, and the OS-installer channel

Builder B2 (os-install + nix), r26 ops-glue-residue writing phase, 2026-07-28.
For the conductor to merge into root `SIBLINGS.md` at step 3.

Rows are ARCHITECTURE-tier only: fundamental decisions and lock-in/lock-out.
Nothing here is an NYI item either side could close with mild implementation
work — where I was tempted, I dropped the row. Grounding is my own research
(citations live in the two companion notes in this directory).

---

## A posture note the conductor should rule on before merging

The charter's two-posture cut is sibling glue-tools (a legitimate user
CHOICE) versus the Big Boys (NOT a choice). **nix/NixOS is squarely a Big
Boy** and the framing fits without strain: no sane person should persistently
choose Dorc over NixOS for a machine they can describe; "go use nix" is
advice we should give out loud; Dorc exists for nix's bootstrap glue and its
residue.

**Home Manager is a Big Boy with an escape hatch**, and the escape hatch is a
real seat rather than a competitive surface. It belongs in the same column
with the seat noted, not in a column of its own.

**The installer ecosystem is neither**, and I think it wants a third posture.
`autoinstall`/`preseed`/`kickstart` are not products a user chooses instead
of Dorc — they are *delivery channels* that every one of these tools has to
arrive through, ourselves included. Comparing "Dorc vs autoinstall" is a
category error; the useful comparison is "what does this channel do and not
do for whoever lands in it." I have kept the rows but flagged the column, and
would rather the conductor decide the shape than have me invent a taxonomy
the human has not typed.

---

## Column: nix / NixOS

Posture: **Big Boy — not a choice.** Dorc's position is the glue before it
and the residue around it.

| Capability | nix | Dorc | Why it is architecture, not a gap |
|---|---|---|---|
| Whole-system state has a single identity you can compare | **Y** — a store path, input-addressed: a complete hash over the derivation and its inputs | **N** | We have per-cell measurements and no global identity, because we never require a total description. `kDEPS` is welded to accept-partial; a global identity is exactly what accepting partial knowledge forfeits. |
| Atomic activation, and rollback to a previous whole-system generation | **Y** — profile generations, boot-menu entries, `--rollback` | **N** | We mutate machines in place. There is no generation to return to and there never will be, because there is no artifact we built the machine from. |
| Cross-machine identical outcome, guaranteed | **Y**, given a committed lock | **N** | We are best-effort against heterogeneous, drifted, partly-unknown machines. This is the thesis, not a shortfall. |
| Build once, substitute many (binary caches) | **Y** | **N** | We ship shell, not artifacts. Nothing to cache. |
| Dependency knowledge is total | **Y** — the closure | **N** — `kDEPS-accept-partial`, welded | The anti-declarative thesis in one row. |
| Secrets can live in the managed description | **N** — "The store is readable to all users on the system"; the manual tells you to read secrets from the filesystem at runtime instead | **N** | A shared N, and worth keeping: secrets are the largest single member of nix's residue, and residue is our seat. Neither side should claim it. |
| Manages a machine you did not build, and does not own | **N** — NixOS is all-or-nothing; the value comes from owning the description | **Y** | Ours is the whole reason to exist. Not fixable on nix's side without stopping being nix. |
| Adoptable incrementally, without a rewrite | **N** — the upfront cost is the tool | **Y** | `DESIGN.md`'s opening argument, with nix named. |
| Imperative "fix this one thing on this one box, now" | **N** by axiom | **Y** | The push/imperative niche. |
| Off-ramp cost | **High** — leaving nix means rewriting the description | **~Zero** — strip and it is sh | Ours is a welded design constraint (`kLANG`); theirs is inherent to owning a language. |
| Plan/evaluation cost | Seconds to dozens of seconds for a NixOS closure, acknowledged and worsening upstream | Cheap; network round-trips dominate everything we do | Follows from the same root: they evaluate a whole-world description, we do not have one. |
| Ships its own push-over-ssh deployment | **Y** — `nixos-rebuild --target-host` / `--build-host`, with ssh `ControlMaster` multiplexing | **Y** | Included to correct an easy assumption: nix is not pull-only or local-only. It is a genuine sibling *deployer* as well as a Big Boy. |
| Offers a convergence verb worth delegating to | **Y**, and it is the best of any incumbent found this round | — | The row where they win and we *consume* the win: our oracle reads their state identity rather than modelling anything. |
| Offers a *dry-run* verb worth delegating to | **N** — `dry-activate`'s own man page: "The list of changes is not guaranteed to be complete" | — | Notable precisely because the tool is excellent. Activation is genuinely not fully predictable from outside, and they say so instead of pretending. A tool can have a sound convergence check and an unsound dry-run; the two are different questions. |

## Column: Home Manager

Posture: **Big Boy, with one real escape hatch.**

| Capability | Home Manager | Dorc | Why it is architecture |
|---|---|---|---|
| Declarative ownership of dotfiles and user packages | **Y** | **N** — we would be a worse chezmoi | Cleanly ceded. |
| Its activation escape hatch mandates idempotence and assists none of it | **mandate: Y / assist: N** — "Any entry here should be idempotent, meaning running twice or more times produces the same result as running it once" | assist: **Y** | The seat, stated in their own docs. Joins chezmoi, yadm, dotbot, and k8s init containers on the mandate-idempotence-assist-nothing tally — now first-party-sourced rather than inferred. |
| Activation blocks are individually addressable files | **N** — every block is a `types.str` concatenated into one generated bash script; there is no per-block file | — | This is what forecloses riding *inside* the slot and forces the store-path-invocation shape. A structural fact about their design, not a missing feature. |
| Activation runs with a curated, deliberately minimal PATH | **Y** — `emptyActivationPath` defaults true, "recommended … to avoid uncontrolled use of tools found in PATH" | — | Not a limitation: it is why anything we splice must be named by store path. Their hygiene rule and our correct spelling are the same fact. |
| Its dry-run answers "what would activation do" | **Y** (`switch -n`, via the `run` helper) | our plan answers "what does this book have left to do against the live host" | Different questions, both honest. Worth a row so nobody merges them. |
| A `test`-style verb that activates without persisting | **N** — parsed by the CLI, then rejected at dispatch; the implementation is commented out | — | Included because it is architecture-adjacent in an instructive way: a verb accepted by an argument parser and unimplemented at dispatch is exactly why capability-probing must probe *behaviour*, never names. |

## Column: the OS-installer channel (Ubuntu autoinstall/subiquity · Debian preseed/d-i · kickstart)

Posture: **channel, not competitor** — see the note above.

| Capability | The channel | Dorc-in-the-channel | Why it is architecture |
|---|---|---|---|
| Declarative machine description at install time | **Y** for autoinstall (versioned `version: 1` + a published JSON Schema; unknown keys become fatal in future versions) · **N** for preseed (open-ended debconf space; the doc's own way to enumerate the spec is to run an install and diff debconf state) | **N** | We are not an installer and should never grow one. |
| Fires again on day N | **N** — structurally: `late-commands`/`late_command`/`%post` run once, at install, and the machine then leaves the channel forever | **Y** | THE row. The channel's whole shape is one-shot; ours is the same file, re-run against a live machine. |
| Idempotence machinery in the hook | **N** — the hooks are raw strings handed to `sh -c` as root; nothing is mandated and nothing is assisted | **Y** | This is the offline-artifact's entire proposition, and it is a wholly empty cell rather than a contested one. |
| Drift detection, ever | **N** | **Y**, from day N onward | Follows from the row above. |
| Runs in the target's real environment | **N** — a chroot with no init manager, `SYSTEMD_OFFLINE=1`, daemons refused by a `policy-rc.d` returning 101, and a resolver borrowed from the installer for the duration of each call | inherits the same constraint, and **guards for it** | Not a defect of the channel; it is what "before first boot" means. Our contribution is that the same file survives both regimes by branching on an observable host fact rather than on a delivery flag. |
| Reports why something did not happen | **N** — a traceback on the console, plus whatever `error-commands` tars up | **Y** | The report lane survives into the artifact; it is one of the two things that do (the other being guards). |
| The channel's exit-code contract is meaningful | **Y** — "Any command exiting with a non-zero return code is considered an error and aborts the installation (except for `error-commands`, where it is ignored)" | matched, not beaten | Kept deliberately: this is the one channel found this round where trusting the rc is correct rather than merely tempting, and the row exists so nobody generalizes our never-trust-the-*channel*-rc floor into never-*produce*-a-meaningful-rc. |
| Attention conservation (a plan that shrinks) | **N** | **N in this channel** | Honest shared N. Offline there is no probed world, so there is no proof, so nothing may be removed. Everything is a guard. The attention product arrives on day N and not before. |

---

## Two rows I drafted and dropped, with reasons

- *"Supports arbitrary imperative escape hatches" for Ignition/Talos.* Out of my domain (B1 owns the cloud-lifecycle channels) and the Talos cell is already conceded in the round's turn-A adjudication. Left alone rather than duplicated.
- *"Has a package/oracle library"* comparing nixpkgs to a Dorc stdlib. Real, enormous, and **not architecture** — it is a maturity gap, and the row discipline excludes it. Worth a sentence in the synthesis note's limitations half instead; it does not belong in a table of fundamental decisions.
