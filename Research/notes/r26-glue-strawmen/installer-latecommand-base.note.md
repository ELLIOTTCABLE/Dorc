# Companion note — `installer-latecommand-base.sh`

Round: ops-glue-residue (r26), writing phase, builder B2 (os-install + nix).
Written 2026-07-28. Tier: AI-authored, imagination-tier strawman commentary.
Every Dorc spelling in the book is invented; the installer facts are cited.

---

## §1 — The pick: Ubuntu autoinstall (subiquity), not Debian preseed

The brief said pick the better-documented one and say why. It is not close.

| | Ubuntu autoinstall / subiquity | Debian preseed / d-i |
|---|---|---|
| spec form | versioned (`version: 1`) + a published JSON Schema page; "Unrecognized keys are ignored in version 1, but they will cause a fatal validation error in future versions" | prose appendix + one commented example file |
| how you learn the full key set | read the schema | run an install, then `debconf-get-selections --installer > file; debconf-get-selections >> file` — the doc's own prescribed method |
| is the key set closed? | yes, per version | **no** — it is whatever debconf questions the udebs and packages you installed happen to ask |
| command-list semantics | documented: "Each command can be a string (in which case it is executed via `sh -c`) or a list… The commands are run as root. Any command exiting with a non-zero return code is considered an error and aborts the installation (except for `error-commands`, where it is ignored)." | not stated anywhere I could reach |
| chroot mechanics | readable in curtin's source with exact bind-mounts, policy-rc.d, resolv.conf handling | one paragraph in the internals manual which itself says the real spec "lives in the `README` in the source for debian-installer-utils" |
| validation ordering | documented as a five-step sequence | not documented |

The decisive line is the third row. Debian's preseed surface is *open-ended and package-defined*: there is no closed spec because there cannot be one. Ubuntu's is a versioned schema with a stated forward-compatibility policy. For a strawman that has to handle warts precisely, one of those is a research target and the other is a séance.

Debian preseed and kickstart both keep a SIBLINGS row; kickstart in particular supplies the sharpest first-party statement about the chroot regime (§4).

Sources: `https://www.debian.org/releases/stable/amd64/apbs05.en.html` · `https://www.debian.org/releases/trixie/example-preseed.txt` · `https://d-i.debian.org/doc/internals/ch02.html` · `https://canonical-subiquity.readthedocs-hosted.com/en/latest/reference/autoinstall-reference.html` and `…/reference/autoinstall-schema.html` · `https://raw.githubusercontent.com/canonical/subiquity/main/doc/reference/autoinstall-reference.rst`. All accessed 2026-07-28.

## §2 — A correction the round should carry: subiquity is **not** busybox

`turn01` records: "Both execute in a **busybox** installer environment [A-debian-installer-internals-2024]", covering subiquity and preseed together. The citation is `d-i.debian.org/doc/internals` — a Debian-installer document. It does not describe subiquity, and subiquity is not d-i.

- **Debian d-i: busybox, first-party.** `/etc/inittab` runs `busybox init`; VT2/VT3 are "(busybox shell)". Confirmed at `d-i.debian.org/doc/internals/ch02.html` §2.2.
- **Ubuntu subiquity: a full Ubuntu live-server userspace.** Subiquity ships as a *snap* inside the live-server squashfs; the ISO uses **casper** (the standard Ubuntu live mechanism); the docs describe `/etc/sudoers.d` entries and an sshd in "the installation system". Sources: `canonical/subiquity` README, `autoinstall-reference.rst` (`casper/install-sources.yaml`), `…/explanation/security-overview.html`.

So the busybox-floor question is load-bearing for **Debian preseed and kickstart-adjacent** installer hooks, and NOT for subiquity late-commands, which get a real Ubuntu with dash-as-`/bin/sh` and bash present.

This does not change the book. The artifact stays floor-safe (dash ∩ posh, `printf`-only, never `echo` with flags — busybox `echo` has escapes off by default and takes `-e` where dash and posh do the inverse, and it fails silently both ways) because the *same* artifact should be droppable into a `preseed/late_command` too. But the round's notes should not carry "subiquity is busybox"; it is wrong and it would mislead a later sizing or capability argument.

## §3 — The design problem the book exists to answer

One file, two execution regimes:

| | day zero | day N |
|---|---|---|
| where | inside `curtin in-target`, chrooted to `/target` | the booted machine, over ssh |
| who | root, always | an admin, `sudo`-ing |
| init | **no manager**; curtin injects `SYSTEMD_OFFLINE=1`; policy-rc.d refuses daemon starts with rc 101 | systemd, live |
| Dorc | a compiled artifact; no controller, no probe, nothing elides — everything is a guard | plan/apply; most of the book folds |
| network | up, but the resolver is the *installer's*, copied in for the duration of the call | the machine's own |

The forbidden answer is `if [ "$DORC_OFFLINE" ]`. That is chef-solo's grave — `if Chef::Config[:solo]` forking recipes — and the round already minted the rule: **the offline face may NARROW (compile-time refusals, verdicts unavailable) and may never FORK semantics.**

The answer the book uses is one line:

```sh
if [ -d /run/systemd/system ]; then INIT_LIVE=yes; else INIT_LIVE=no; fi
```

`/run/systemd/system` exists exactly when systemd is the init of the *current root*. It is the canonical test — Debian's own maintainer-script helpers use it — and it is false in the chroot and true on the booted machine, for the right reason rather than by coincidence.

This is the "metadata is spelled in sh" doctrine paying out at full strength, and it is worth stating as a general result rather than a trick: **the regime difference between an offline artifact and a live apply is, in every case I found, observable as an ordinary host fact.** Not a delivery flag, not a compile-mode variable — a question about the machine, asked in the idiom people already use. The book does not know how it was delivered and does not need to.

Concretely it lets three pairs of lines coexist honestly:

- `systemctl enable ssh` (symlinks; works offline) beside a guarded `systemctl restart ssh` (needs a manager);
- `timedatectl set-timezone` (D-Bus; absent offline) beside the `/etc/localtime` symlink it manages;
- `ufw --force enable` (kernel state) beside `systemctl enable ufw`.

The offline face narrows the *mechanism* under an observed constraint. The intent is identical and it is spelled once.

## §4 — What is actually present in the installer environment

All from first-party sources, 2026-07-28.

**Where late-commands run.** "The commands are run in the installer environment with the installed system mounted at `/target`. You can run `curtin in-target -- $shell_command` to run in the target system." Not chrooted unless you say so.

**How they run.** "Each command can be a string (in which case it is executed via `sh -c`) or a list, in which case it is executed directly. The commands are run as root. Any command exiting with a non-zero return code is considered an error and aborts the installation (except for `error-commands`, where it is ignored)."

Consequence for us, and it is a pleasant inversion of the usual posture: **here the channel rc IS the contract.** Everywhere else Dorc refuses to trust a channel's exit code and rides an in-band sentinel. In `late-commands` subiquity reads the rc and fails the install on non-zero, which is what you want — a base-machine book that failed should not report a provisioned machine. The artifact's `set -eu` composes with it directly. Worth noting because it is the one channel found this round where rc-trust is correct rather than merely tempting.

**What `curtin in-target` does** (curtin `master`, `curtin/commands/in_target.py` + `curtin/util.py` `ChrootableTarget`):

- bind-mounts `/dev`, `/proc`, `/run`, `/sys`, plus `/sys/firmware/efi/efivars` when UEFI;
- writes `/usr/sbin/policy-rc.d` in the target, exiting **101** for every initscript except `makedev` and `x11-common`, and removes it on exit — **unless the file already existed**, in which case curtin neither tracks nor removes it;
- bind-mounts `/usr/bin/true` over `/usr/bin/ischroot`, because curtin may run the command in a separate PID namespace and `ischroot` would otherwise misdetect;
- injects `SYSTEMD_OFFLINE=1` into the environment when the target is not `/`;
- and the resolv.conf dance, below.

**systemd in the chroot.** Not documented in curtin or subiquity prose. It *is* documented, first-party, on the kickstart side, and the statement is general enough to cite: "Because post-install script runs in a `chroot` environment, most `systemctl` commands will refuse to perform any action." (RHEL 9, *Kickstart script file format reference*.) Combined with `SYSTEMD_OFFLINE=1` and policy-rc.d/101, the `enable`-works/`start`-does-not split is well-grounded even though no single vendor writes that sentence.

## §5 — The DNS trap, and why it is a hermeticity story rather than a new problem

Kickstart documents the classic version, and the wording is 20+ years old and *still reproduced verbatim* upstream in pykickstart: "If you configured the network for DHCP, the `/etc/resolv.conf` file has not been completed when the installation executes the `%post` section. You can access the network, but you can not resolve IP addresses."

Anaconda's current docs give a different root cause for the modern era — the target's `/etc/resolv.conf` symlink is not created when `systemd-resolved` is absent — and pykickstart's own page bolts on a pointer to it without rewriting the stale prose. So the headline claim in the most-quoted document is obsolete and the correct explanation lives two hops away. That is worth flagging on its own: this trap is *famous, wrong in its famous form, and still shipping.*

Ubuntu's is subtler and, for our purposes, worse. `ChrootableTarget.__enter__` renames the target's existing `/etc/resolv.conf` aside into a temp dir inside `/etc`, then does `shutil.copy("/etc/resolv.conf", rconf)` — copying the *installer's* resolver config into the target as a plain file. `__exit__` renames the original back and removes the temp dir.

So inside `curtin in-target`, DNS **works**. That is the trap. A guard that asks "can this machine reach the package mirror" is answered by the installer's resolver, for the duration of one call, about a machine that is not the one that will boot. It returns 0, confidently, and means nothing.

The good news is that we do not need a new rule for this. `hermeticity-precondition` already says read-only ≠ hermetic and names the `getent hosts` class as unable to license. The installer environment is the sharpest real-world instance of *why* that rule exists rather than a new exception to it. The book therefore carries no reachability guard; it sets the timezone and lets the daemon sort itself out on boot.

**Rider for the note's limitations half:** any future stdlib oracle whose check routes through name resolution is not merely non-hermetic in the abstract — it is actively wrong in the installer regime, where a copied-in resolver makes it *succeed*. Failing-open would be survivable; succeeding-about-the-wrong-machine is the cardinal-sin shape.

## §6 — Capability probing moves to runtime, and that is a narrowing

`rul-capability-probing-per-feature` puts capability-matching on Dorc, per-feature, per-host, with no tier ladder. In the networked lane that happens at plan time on the controller.

Offline, the host does not exist when the artifact is compiled. So the same matching has to happen inside the artifact, at runtime, and the artifact carries the probes it would otherwise have shipped and consumed.

I want to name this because it looked like a fork and is not:

- the **questions** are identical (does this host have `sudo` or `doas`; is systemd the init; does this `timeout` take `-t`);
- the **licenses** are identical (a capability answer never licenses an elision in either lane);
- only the **timing** differs, and later answers are strictly weaker.

Same book, same meaning, worse information. That is the definition of a narrowing under the no-semantic-fork rule, and it means the offline lane needs no capability machinery of its own — it needs the *existing* probes emitted inline instead of shipped. Which is, conveniently, what they already are: sh.

## §7 — The wait loop, and why it compiles in

The canonical firstboot fix for the apt/dpkg lock race — unfixed at the apt layer for years, ~2,000 indexed files carrying it — is `while fuser /var/lib/dpkg/lock-frontend; do sleep 1; done`.

It is also the wait-placement doctrine's clean case. The awaited fact is a lock file on the host's own disk: observable from inside. So the loop compiles into the artifact and costs one connection. The controller-side alternative, `until ssh box 'fuser …'`, costs one TCP handshake and one auth per poll — the cost cdist names as its own stated regret, and the thing the standing perf law forbids ("never let a network boundary participate in iteration").

Contrast with the pivot's own waits (is the machine reachable yet), which are definitionally *not* observable from inside the host and are the sanctioned controller-side exception. The cut is clean and the book sits entirely on the compile-in side of it.

Today the loop is `StatusIterated` and blocks unconditionally. The book is honest about that: it blocks, everything after it guards or runs, and the artifact still works. If modeled until-loops land, this line is the one that pays.

## §8 — The siting, and the all-in-one thesis

The non-Dorc standup of this machine is two artifacts:

```yaml
# autoinstall.yaml — the tool file
autoinstall:
  version: 1
  late-commands:
    - curtin in-target -- apt-get update
    - curtin in-target -- apt-get install -y curl
    - wget -O /target/postinstall.sh http://192.168.0.2/postinstall.sh   # ... and a server to host it
    - curtin in-target -- bash /postinstall.sh
```

plus `postinstall.sh`, plus somewhere to serve it from. (The `wget`-then-`in-target`-bash pattern is Canonical's own, from `canonical/autoinstall-desktop`'s reference `autoinstall.yaml`, where it appears commented out as the suggested customization hook.)

Ours is the same tool file with the book inlined, and no server:

```yaml
autoinstall:
  version: 1
  late-commands:
    - |
      cat >/target/root/.dorc-base <<'DORC_ARTIFACT_EOF'
      … the output of `dorc compile installer-latecommand-base.sh` …
      DORC_ARTIFACT_EOF
    - curtin in-target -- /bin/sh /root/.dorc-base
  error-commands:
    - tar -czf /installer-logs.tar.gz /var/log/installer/ /target/root/.dorc-report
```

Three notes on the YAML:

- **the heredoc is legal and the block scalar is documented.** Canonical's own examples use the folded form `>-` for long single commands; a literal `|` block preserves newlines and is handed to `sh -c` as-is, which is a valid multi-line shell script by construction. No feature is being invented.
- **no size limit is documented anywhere.** I looked at the reference, the schema page, the delivery page, and cloud-init's NoCloud datasource doc. Autoinstall user-data has no stated cap. That is a real difference from the cloud channels turn A sized (EC2's 16 KB raw being the tight one) and it should not be assumed to inherit their caps.
- **the delivery document can carry both.** Top-level cloud-config keys affect the *ephemeral installer*; `autoinstall:` is subiquity's; `autoinstall.user-data` is cloud-config for the *installed system's first boot*. Three tiers in one file, first-party-documented, with a guard: the installer errors if it finds autoinstall-specific keys at the top level rather than ignoring them. So `write_files` at top level is a second viable delivery for the artifact, and a cleaner one than a heredoc if the YAML gets long.

**`early-commands` deserves a flag of its own**, because it is stranger than anything else found this round: "The autoinstall configuration is available at `/autoinstall.yaml` … and the file is re-read after the `early-commands` have run to allow them to alter the configuration if necessary." Canonical's own example is `wget -O /autoinstall.yaml $TRUSTED_SERVER_URL`. An installer channel whose payload can rewrite its own configuration before it is consumed is a self-modifying-config seat. I did not build a book on it and would not without direction; it is noted because it exists and nothing in the round's channel inventory has this shape.

## §9 — Where the value actually lands, day zero vs day N

Honest ledger, USER_STORY-style.

**Day zero (compiled artifact, in `curtin in-target`).**
- Spent: one `dorc compile` at build time; the artifact inlined into one `late-commands` entry.
- Gained: every non-idempotent line in the book grows a guard, so a re-run — a re-image, a second late-command pass, a first boot that repeats — does not double-apply. The `useradd`, the `ufw reset`, the `apt-get install` all stop being one-shot.
- Gained: a report. `${DREP_V1}` lands in the target and `error-commands` can ship it with the installer logs, so a failed install has an explanation rather than a traceback.
- **Not gained: attention.** Nothing elides. There is no probed world, so there is no proof, so nothing may be removed. The plan-as-attention-product is entirely absent from this regime and pretending otherwise would be dishonest.
- Not gained: anything for `hork`.

**Day N (`dorc plan installer-latecommand-base.sh box.example.net`).**
- Spent: nothing further. Same file.
- Gained: the whole thing. The standup folds — most sites converged, `[ -d /run/systemd/system ]` true so the offline arms are *omitted* by value-flow rather than elided (a dead branch could not run; that is cheaper than an elision and consumes no per-line vouches), and the book collapses to the residue.
- Gained, structurally: the machine build itself is classed as a transit. On a firing day the transit runs and downstream honestly guards; on every other day the transit is converged, an elided command casts no wall, and the post-install epoch gets full elision. Transit verbs therefore belong at the top of any describability priority list — an unmodeled, unguarded transit walls every single day.
- Not gained: `hork`, ever, and nothing after it. Which is why it is the last line in the book.

**The through-line worth carrying to the synthesis note:** the artifact regime is where Dorc's *safety* proposition lives alone, and the day-N regime is where the *attention* proposition arrives. They are the same file, and the second is free once the first exists. That is a better story than either half alone, and it is the actual answer to the "idempotence-wrapper positioning" worry for this channel: the wrapper is rung zero and the machine keeps existing afterwards.

## §10 — Escalations and flagged questions (conductor)

No book stopped. Nothing outside the sanctioned palette was needed — which is itself a finding: the installer siting's hard parts (regime difference, DNS, chroot, capability timing) all resolved into *existing* mechanisms plus admin-authored sh, with no new Dorc feature required.

- **flag-rc-trust-is-correct-here** — §4. `late-commands` reads the artifact's exit code and aborts the install on non-zero. This is the one channel found this round where trusting the channel rc is right. The in-band sentinel is still what the *transport* needs; but the artifact's own rc is a real contract with the installer, and the book must not swallow failures to be polite. Worth a sentence in the knob entry so nobody generalizes "never trust rc" into "never produce a meaningful rc".
- **flag-early-commands-self-modifying-config** — §8. A payload slot whose script can rewrite the config that is about to be consumed. No book built; no direction requested; recorded so the channel inventory is complete.
- **flag-policy-rc-d-preexisting** — curtin will not remove a `policy-rc.d` it did not write. An artifact that writes one (a plausible thing for a base-machine book to do) would silently persist it into the booted machine, where every future package install is quietly refused a daemon start. Nasty, real, and exactly the class of thing an `apt`/`dpkg` oracle's footprint should eventually know about. Not this round's to build.
- **flag-installer-env-is-not-busybox** — §2. Correction to `turn01`'s note. The floor claim is unaffected; the factual attribution is wrong and would mislead later work.
- **ask-artifact-report-sink-siting** — the book writes its report to `${DREP_V1:-/dev/null}`, and in the installer regime the controller that would normally supply that value does not exist. Somebody has to choose the path, and it has to land in `/target` to survive the reboot. That is a controller-supplied-literal question (`rul-scratch-root-never-read-from-host` forbids deriving it from the host environment), and in the offline lane there is no controller at compile time either — the *admin* supplies it, or `dorc compile` bakes a default. I did not decide; the book leaves the default in place and the YAML in §8 assumes `/target/root/.dorc-report`.
