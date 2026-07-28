# SIBLINGS audit — B2 lane (nix/NixOS column · Home Manager prose · OS-installer channel bullets)

Follow-up pass, 2026-07-28. Light tier. Conductor integrates; this file touches
nothing else. Every URL below was fetched this session and resolved; the two
grading exceptions are marked inline.

**Pinned refs used for source permalinks** (so the quotes stay findable after
`master` moves):
- nixpkgs `5074d8679cc7f41dd035bff04ed8549c9b5e9d10`
- home-manager `f469c79b955609d6a8fdd9e689be76a93b1621d7`

---

## §1 — Citations

### 1a. nix/NixOS column cells

| Row | Cell | Proposed footnote text | URL |
|---|---|---|---|
| Re-measures the live system every run | N | Compares *closures*, not the machine: `--diff` is documented as "show the diff between the system closure in /run/current-system and the newly built system closure" — description against description. Drift outside what activation rewrites is never interrogated. | https://github.com/NixOS/nixpkgs/blob/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/pkgs/by-name/ni/nixos-rebuild-ng/nixos-rebuild.8.scd |
| Preview before mutating | ~ | Split, and the split is the point: the *closure* diff is complete (`--diff`, `nix store diff-closures`), the *action* preview is self-documented as not — `dry-activate` "will print which systemd units would be restarted. The list of changes is not guaranteed to be complete." | same URL |
| Fleet: inventory, groups, per-host data | ~ | The data half is native — `nixosConfigurations.<host>` over shared modules is inventory, groups and per-host data in one expression. The orchestration half is one host per invocation (`--target-host`, ssh, `ControlMaster`); fleets are third-party (colmena, deploy-rs). | same URL (`--target-host`, `NIXOS_REBUILD_SSH_DEFAULT_OPTS`) |
| Privilege escalation built in | ~ | Real but scoped to its own invocation: `--elevate {none,sudo,run0}`, `--ask-elevate-password`, `NIX_SUDOOPTS`. It elevates *itself*; it is not a per-task escalation facility, because NixOS has no per-task layer. | same URL |
| Remembers what it built; delete by un-declaring | Y | Bounded by what activation reaches. `/nix/var/nix/profiles/system` "contains the current and previous system configurations"; but the same page concedes "user services need to be started manually as they aren't detected by the activation script at the moment", and runtime state (data dirs, `/var`) is outside the closure by design. | same URL (FILES + DESCRIPTION) |
| Whole-system rollback | Y | The system profile holds current *and previous* generations and generates the boot menu; `--rollback` selects the generation before the current one. | same URL |
| Secrets management story | N | First-party and explicit: "The store is readable to all users on the system… Organize your derivations so that secrets are read from the filesystem (with appropriate access controls) at run time." The answer is out-of-band schemes, not a story nix owns. | https://nix.dev/manual/nix/latest/store/secrets |
| Creates infrastructure (VMs, DNS, networks) | N | `build-vm` and `build-image --image-variant proxmox` produce *local artifacts*, not provisioned resources; cloud/disk provisioning is ecosystem (nixos-anywhere, disko, NixOps), not nix/NixOS. | https://github.com/NixOS/nixpkgs/blob/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/pkgs/by-name/ni/nixos-rebuild-ng/nixos-rebuild.8.scd |

Cells I judged obvious and left unfootnoted: "declare the end state Y",
"templating Y", "no resident software N", "existing scripts run unchanged N",
"authored in the target's own language N", "full value without an authoring
layer N", "off-ramp N".

### 1b. Prose-section quotes (Home Manager)

| Claim | Verbatim | URL |
|---|---|---|
| HM mandates idempotence and assists none of it | "Any entry here should be idempotent, meaning running twice or more times produces the same result as running it once." | https://github.com/nix-community/home-manager/blob/f469c79b955609d6a8fdd9e689be76a93b1621d7/modules/home-environment.nix |
| the check/write phase split | "If the script block produces any observable side effect… it *must* be placed after the special `writeBoundary` script block. Prior to the write boundary one can place script blocks that verifies, but does not modify, the state of the system and exits if an unexpected state is found." | same |
| `emptyActivationPath` default + rationale | "Whether the activation script should start with an empty `PATH` variable… It is recommended to keep this at `true` to avoid uncontrolled use of tools found in PATH." Default: `true` for stateVersion ≥ 22.11. | same |
| no per-block file | `activationScript = pkgs.writeShellScript "activation-script"`, whose body is `lib.concatStringsSep "\n" (map mkCmd sortedCommands.result)` under `set -eu` + `set -o pipefail` | same |
| reader-facing option page (for anyone who wants rendered docs rather than source) | — | https://nix-community.github.io/home-manager/options/home-manager/home.html#opt-home.activation |

Note for the conductor: `home.emptyActivationPath` is `internal = true`, so it
does **not** appear on the rendered options page. The source permalink is the
only citable home for that quote.

### 1c. Prose-section quotes (nix)

| Claim | Verbatim | URL |
|---|---|---|
| `dry-activate` incompleteness | "The list of changes is not guaranteed to be complete." | https://github.com/NixOS/nixpkgs/blob/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/pkgs/by-name/ni/nixos-rebuild-ng/nixos-rebuild.8.scd |
| world-readable store + read-at-runtime guidance | as quoted in §1a | https://nix.dev/manual/nix/latest/store/secrets |
| `switch` fuses activate + boot-default; `test` omits the second | "*switch* Build and activate the new configuration, and make it the boot default." / "*test* Build and activate the new configuration, but do not add it to the GRUB boot menu." | nixos-rebuild.8.scd permalink above |
| user services are residue | "user services need to be started manually as they aren't detected by the activation script at the moment" | same |
| flake attr defaulting | "The flake must contain an output named 'nixosConfigurations.name'. If name is omitted, it default to the current host name." | same |

### 1d. Channel bullets (OS installers)

| Claim | Verbatim | URL |
|---|---|---|
| installer rc contract | "Any command exiting with a non-zero return code is considered an error and aborts the installation (except for `error-commands`, where it is ignored)." | https://canonical-subiquity.readthedocs-hosted.com/en/latest/reference/autoinstall-reference.html#ai-command-lists |
| where late-commands run | "The commands are run in the installer environment with the installed system mounted at `/target`. You can run `curtin in-target -- $shell_command` to run in the target system (similar to how plain `in-target` can be used in `d-i preseed/late_command`)." | https://canonical-subiquity.readthedocs-hosted.com/en/latest/reference/autoinstall-reference.html#late-commands |
| versioned schema + forward-compat policy | "Unrecognized keys are ignored in version 1, but they will cause a fatal validation error in future versions." | https://canonical-subiquity.readthedocs-hosted.com/en/latest/reference/autoinstall-reference.html |
| `early-commands` can rewrite the config being consumed | "The autoinstall configuration is available at `/autoinstall.yaml`… and the file is re-read after the `early-commands` have run to allow them to alter the configuration if necessary." Canonical's own example: `wget -O /autoinstall.yaml $TRUSTED_SERVER_URL`. | same page, `#early-commands` |
| error-commands seat | "commands to run after the installation has failed… Logs will be available in `/var/log/installer`" | same page, `#error-commands` |

**GRADING EXCEPTION (one).** The rc-contract quote in the first row is the only
one I did not read in my own fetch window. The page resolves (fetched six times
this session) and the anchor is referenced in-page from both `early-commands`
and `late-commands` as `[command list](#ai-command-lists)`; the verbatim string
is corroborated by a Kagi index snippet **of that exact URL** and independently
by the doc-reader's direct read, plus two mirrors of the same text in the Ubuntu
Server Guide PDF. Confidence +SURE on the wording, but flagged so nobody records
it as first-hand-in-window.

**GRADING EXCEPTION (two).** The Determinate Systems installer claims in the nix
companion note are vendor material (commercially motivated) and were graded [B]
there; nothing in the SIBLINGS table depends on them.

---

## §2 — Row chafe

### CHAFE-WRONG — "The reviewed text is byte-for-byte what executes" · nix `Y`

**Why it chafes.** Under the row's plain wording the cell is false. What a nix
user reviews is `configuration.nix` / `flake.nix`. What executes is a *generated*
activation script plus systemd units materialised in the store — for Home
Manager I read the generator myself (`pkgs.writeShellScript "activation-script"`
concatenating every block); NixOS's is `switch-to-configuration`. The Nix
expression is evaluated into derivations; the text is not the thing that runs.

**Internal-consistency argument, which I find decisive:** Terraform gets `N` for
HCL→API-calls. `.nix`→derivation→activation-script is the same transformation
shape. Two cells cannot both be right.

**But there is a real property underneath, and it is stronger than Dorc's.**
`nixos-rebuild build` yields a store path you can inspect (`--diff`,
`nix store diff-closures`), and `switch` activates *exactly that path*, bit-identical,
by construction. "The artifact I reviewed is what ran" is a claim nix can make
more completely than we can.

**Proposed resolution — split the row:**
- *"The artifact you reviewed is exactly what runs"* → Dorc `Y`, nix `Y`, Terraform `~` (plan file), others `N`.
- *"You review it in the language you wrote"* → Dorc `Y`, everyone else `N`, k8s `~`.

If the conductor wants one row, keep the current wording and set nix to `N`
with a footnote naming the artifact property, rather than leaving a `Y` a nix
user would dispute in the other direction.
Evidence: home-environment.nix permalink (§1b) · nixos-rebuild.8.scd `--diff`.

### CHAFE-WORDING — "Converges continuously with nobody present" · nix `N`

**Why it chafes.** NixOS ships a first-party option whose documented behaviour
is exactly unattended convergence: `system.autoUpgrade.enable` — "If enabled, a
systemd timer will run `nixos-rebuild switch --upgrade` once a day." That is
shipped-in-the-box, not user-assembled, which distinguishes it from
Ansible-plus-cron (also `N`, but assembled by the user).

**Why `N` still survives:** "continuously" is doing the work. A daily timer is
periodic re-application, not a control loop that observes and corrects drift —
which is what makes Kubernetes the lone `Y`.

**Proposed rewording:** *"A control loop reconciles drift with nobody present"*
— then nix's `N` is legible rather than looking like an oversight, and the
distinction from a cron/timer is stated instead of implied. nix stays `N` under
that wording; `~` only if the criterion becomes "ships a first-party unattended
apply".
URL: https://github.com/NixOS/nixpkgs/blob/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/nixos/modules/tasks/auto-upgrade.nix

### CHAFE-WORDING (low) — "Creates infrastructure (VMs, DNS, networks)" · nix `N`

The parenthetical invites a reader who knows `nixos-rebuild build-vm` and
`build-image --image-variant proxmox` to think the cell is wrong. It is not —
those produce local artifacts, not provisioned resources. Proposed
parenthetical: *"(provisions cloud/provider resources)"*. nix stays `N`.

### Not chafe, footnote suffices — "Skips explained with queryable provenance" · nix `N/A`

`N/A` is right (the row is about skips). Worth one clause so a reader does not
infer nix has no provenance story at all: it ships `nix why-depends` and
`nix store diff-closures`, which are genuinely queryable — about the closure,
not about decisions. No cell change proposed.

### Not chafe — "Fleet" `~` and "Privilege escalation" `~`

Both are exactly right and for the reasons in §1a. Recording that I checked and
declined to pull, since a `~` with no footnote reads like a hedge.

---

## §3 — Out-of-lane flags (free; no research owed, one line each)

- **OUT-OF-LANE — Terraform, "Re-measures the live system every run" `~`:** plan
  refreshes every resource from provider APIs by default, which is a strong `Y`
  for cloud objects and a hard `N` for anything inside a host — the `~` may be
  right for the opposite reason a reader will assume, so it likely wants a
  footnote more than a cell change.
- **OUT-OF-LANE — cloud-init, "Off-ramp: stop using it, keep working artifacts" `~`:**
  an `#!/bin/sh` user-data payload *is* a plain script and survives cloud-init's
  removal intact; the `~` is presumably averaging that clean half against
  `#cloud-config`, which is worth saying in a footnote.
- **OUT-OF-LANE — cloud-init, "Check-then-converge inside its own units" `N`:**
  correct, but the interesting reason is that cloud-init *rations* instead of
  checking (`per-instance` / `per-boot` / `per-once` frequencies) — the same
  hash-rationing shape USER_STORY calls out for chezmoi's `run_onchange_`.
- **OUT-OF-LANE — Kubernetes, "Whole-system rollback" `~`:** `kubectl rollout undo`
  is per-workload, never whole-system, which is what the `~` presumably encodes;
  fine as-is, flagged only because a reader may read `~` as "partial support"
  rather than "different unit of rollback".

---

## §4 — Verification ledger

Fetched and resolving, 2026-07-28:

1. `https://nix.dev/manual/nix/latest/store/secrets` — 200, quote read in window.
2. `https://github.com/NixOS/nixpkgs/blob/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/pkgs/by-name/ni/nixos-rebuild-ng/nixos-rebuild.8.scd` (raw form fetched) — 200, all quotes read in window.
3. `https://raw.githubusercontent.com/NixOS/nixpkgs/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/nixos/modules/tasks/auto-upgrade.nix` — 200, quote read in window.
4. `https://github.com/nix-community/home-manager/blob/f469c79b955609d6a8fdd9e689be76a93b1621d7/modules/home-environment.nix` (raw form fetched) — 200, all quotes read in window.
5. `https://nix-community.github.io/home-manager/options/home-manager/home.html` — 200 (note: `options.xhtml` 302-redirects here; cite the target, not the old path).
6. `https://canonical-subiquity.readthedocs-hosted.com/en/latest/reference/autoinstall-reference.html` — 200, `late-commands` / `early-commands` / `error-commands` / version-key quotes read in window; `#ai-command-lists` quote per the grading exception in §1d.
7. `https://raw.githubusercontent.com/NixOS/nixpkgs/5074d8679cc7f41dd035bff04ed8549c9b5e9d10/nixos/modules/misc/version.nix` — 200, fetched while chasing a `stateVersion` citation I ended up not needing (the "Remembers Y" scope caveat is better served by the manpage's own user-services concession).

Dead/avoided: `https://nix-community.github.io/home-manager/options.xhtml` (redirect
stub only — do not cite). `https://www.mankier.com/8/nixos-rebuild` resolves and
was my original read, but it is a third-party manpage renderer; every quote has
been re-sourced to the pinned nixpkgs `.scd` and mankier should not appear in
`SIBLINGS.md`.
