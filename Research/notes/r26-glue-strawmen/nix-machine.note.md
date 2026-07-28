# Companion note — `nix-machine.sh` + `nix-hm-splice.nix`

Round: ops-glue-residue (r26), writing phase, builder B2 (os-install + nix).
Written 2026-07-28. Tier: AI-authored, imagination-tier strawman commentary.
Every Dorc spelling in the books is invented; the tool facts are cited.

---

## §1 — What is real and what is invented

**Real** (fetched 2026-07-28, first-party unless graded otherwise):

| claim | source |
|---|---|
| `nixos-rebuild switch` = "Build and activate the new configuration, and make it the boot default"; `boot` = boot-default-without-activating; `test` = activate-without-boot-default | `nixos-rebuild.8` — `https://www.mankier.com/8/nixos-rebuild` (rendered man page), cross-read against `pkgs/by-name/ni/nixos-rebuild-ng/nixos-rebuild.8.scd` in nixpkgs `master` |
| `nix.set_profile()` (which advances `/nix/var/nix/profiles/system`) runs for `SWITCH`/`BOOT` only, never `TEST` | nixpkgs `master`, `pkgs/by-name/ni/nixos-rebuild-ng/src/nixos_rebuild/services.py`, `_activate_system()` |
| `/run/current-system` = "A symlink to the currently active system configuration in the Nix store"; `/nix/var/nix/profiles/system` = "The Nix profile that contains the current and previous system configurations" | `nixos-rebuild.8` FILES section |
| `dry-activate` — "**The list of changes is not guaranteed to be complete.**" | `nixos-rebuild.8`, verbatim |
| `dry-activate` output regression, filed and closed without a fix ("No changes AFAIK") | `https://github.com/NixOS/nixpkgs/issues/501386` (opened 2026-03-19, closed 2026-04-29) |
| `system.build.toplevel` — "This option contains the store path that typically represents a NixOS system. **You can read this path in a custom deployment tool for example.**" | nixpkgs `nixos/modules/system/activation/top-level.nix` |
| `nixosConfigurations.<name>.config.system.build.toplevel` is the canonical attr path | Nix manual, `nix flake check` |
| `--target-host` / `--build-host` run over ssh (with `ControlMaster`/`ControlPersist=60`) | `nixos-rebuild.8`; `…/src/nixos_rebuild/process.py` |
| `--use-remote-sudo` is deprecated in favour of `--elevate=sudo`; `--sudo` is an alias | `…/src/nixos_rebuild/__init__.py` argparse help strings |
| `--flake` with no `#name` defaults to the current hostname | `nixos-rebuild.8`; `models.py` `Flake._get_hostname` |
| Nix is **input-addressed by default**; content-addressed store objects are FODs and the experimental `ca-derivations` | Nix manual glossary (`input-addressed store object`, `content-addressed store object`) + `development/experimental-features` |
| flakes and `nix-command` are both still listed under "Currently available experimental features" | Nix manual, `development/experimental-features` |
| the installer's own PATH wart: "Nix won't work in active shell sessions until you restart them." | `NixOS/nix` `master`, `scripts/install-multi-user.sh`, `configure_shell_profile()` |
| multi-user profile script is `/nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh`, sourced from `/etc/bashrc`, `/etc/profile.d/nix.sh`, `/etc/zshrc`, `/etc/bash.bashrc`, `/etc/zsh/zshrc` | same file, `PROFILE_NIX_FILE` + the `PROFILE_TARGETS` fallback list (neither the Darwin nor the systemd installer overrides it) |
| upstream has no automated uninstall — the `uninstall)` branch is commented out and `remove_nix_artifacts()` is `failure "Not implemented yet"`; it prints a runbook instead | same file |
| Determinate's installer has `/nix/nix-installer uninstall` and enables `nix-command`+`flakes` by default | `https://github.com/DeterminateSystems/nix-installer` README — **[B] vendor**, commercially motivated |
| NixOS eval time: "Evaluating a NixOS configuration … can take several, even dozens of seconds … NixOS is getting slower faster than Nix is getting faster" | `https://discourse.nixos.org/t/…/65114` — **[C] community**, high-signal, dated 2025-06-02 |
| Home Manager: "Any entry here should be idempotent, meaning running twice or more times produces the same result as running it once." | `nix-community/home-manager` `master`, `modules/home-environment.nix`, `home.activation` option description |
| all activation blocks concatenate into ONE `pkgs.writeShellScript "activation-script"` (bash, `set -eu` + `set -o pipefail`); no per-block file | same file, `home.activationPackage` (`activationCmds = lib.concatStringsSep "\n" (map mkCmd sortedCommands.result)`) |
| `home.emptyActivationPath` defaults **true** at stateVersion ≥ 22.11: "It is recommended to keep this at `true` to avoid uncontrolled use of tools found in PATH." | same file |
| `run` / `run --quiet` / `run --silence`; `DRY_RUN`, `VERBOSE`, `VERBOSE_ARG`, `verboseEcho`; `$DRY_RUN_CMD` deprecated in favour of `run` | same file's option description + `lib/bash/home-manager.sh` + `docs/release-notes/rl-2405.md` |
| HM generation profile = `$XDG_STATE_HOME/nix/profiles/home-manager` (fallback `/nix/var/nix/profiles/per-user/$USER`), distinct from the package profile `~/.nix-profile` | `modules/lib-bash/activation-init.sh`, `setupVars()` |
| `home-manager test` and bare `rollback` parse but are rejected at dispatch (implementation commented out) | `home-manager/home-manager` CLI script |
| tailscale on NixOS: `useRoutingFeatures` doc says "you will still need to call `sudo tailscale up` with the relevant flags"; without `authKeyFile` there is no automatic `up` at all | nixpkgs `nixos/modules/services/networking/tailscale.nix` |
| "The store is readable to all users on the system … Organize your derivations so that secrets are read from the filesystem (with appropriate access controls) at run time." | Nix manual, `store/secrets` |

**Invented** (does not exist; may never):

`dorc plan` / `dorc-run` / `dorc compile --fragment` · the `#:` mark carrier applied to a book · the `org.nixos.*` and `org.tailscale.*` kinds · `__is_converged` / `__disturbs` / `__predict` bodies for any of these tools · the `${DREP_V1}` report sink's contents · the `config.dorc.activation` Home Manager option namespace · `--path=` / `--plan-only` / `--whylog=` flags on `dorc-run`.

---

## §2 — A correction the round should carry: **not** content-addressed

The charter (`Strawman-book candidates`, nix-machine bullet) says nix's convergence verb "is content-addressed (current-system store path vs expected closure)". The conclusion survives; the adjective does not.

Nix is **input-addressed by default**. An ordinary derivation's store path is a hash over the derivation — the recipe and its inputs — not over the bytes produced. Content-addressed store objects exist, but only for fixed-output derivations and for `__contentAddressed` under the still-experimental `ca-derivations` feature.

This is not pedantry with no consequence; it changes what the check *means*, in our favour:

- content-addressing would answer "do these two systems have the same bytes";
- input-addressing answers "was this system built from this recipe" —

and the second is the question a convergence check should be asking. Recommend the synthesis note say **input-addressed**, and say why that is the better property rather than a weaker one. Someone will check; nix's own glossary is explicit.

## §3 — Why this shape, and what the delegation oracle actually buys

The book is the glue layer, and the flake is untouched. That is the happy-parent posture, and it is the only honest one available: nix is a Big Boy, "go use nix" is advice we give, and a Dorc that had opinions about a flake would be a strictly worse nix.

What is left over is real, though, and it is not small:

1. **Bootstrap.** Nix's own bootstrap glue is README-shaped. The one-liner is documented in two upstream places in two different dialects (the manual shows `bash <(curl -L …)` — a bashism; nix.dev's install guide shows the POSIX-safe `curl -L … | sh -s -- --daemon`), and the thing everyone gets wrong is not the install, it is the line after it.
2. **The PATH cliff.** The installer's own final message is "Nix won't work in active shell sessions until you restart them." A bootstrap script cannot restart a shell. It has to source `/nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh` — a path nothing tells you, that you find by reading `install-multi-user.sh`. And `command -v nix` is the wrong guard for exactly the same reason it is the obvious one.
3. **The residue.** `tailscale up`, user services (`nixos-rebuild`'s own man page: "user services need to be started manually as they aren't detected by the activation script"), out-of-store secrets, and GC policy.

Against that, the delegation oracle is the exhibit. Two questions, both answered exactly by nix:

```sh
[ "$(readlink -f /run/current-system)" = "$(readlink -f /nix/var/nix/profiles/system)" ]   # switch, not test
[ "$(readlink -f /run/current-system)" = "$(nix eval --raw "$dir#nixosConfigurations.$attr.config.system.build.toplevel")" ]
```

`system.build.toplevel`'s own option documentation invites this: "You can read this path in a custom deployment tool for example."

**The switch/test split is the part I did not expect to be load-bearing.** `nixos-rebuild` advances `/nix/var/nix/profiles/system` for `switch` and `boot` and never for `test`. So a machine where someone ran `nixos-rebuild test` yesterday has `/run/current-system` pointing at the desired closure and a boot default pointing somewhere else. A one-symlink check would say converged, elide the `switch`, and leave a machine that silently reverts on its next reboot — a real under-execution, from a check that looked complete. Two readlinks fix it. That asymmetry is worth generalizing: **a tool whose verb fuses two acts needs a check per act, and the cheap check is usually the one for the act you forgot about.**

## §4 — Why `dry-activate` gets no vouch, and why that is the good news

`nixos-rebuild dry-activate` is the verb an incumbent-comparison would reach for: it builds the closure and prints what activation would change. And it is not a convergence verb, by its own documentation: "The list of changes is not guaranteed to be complete." Its exit code is nowhere documented as a drift signal, and its output has an open-then-closed-without-fix corruption report against the current implementation.

This is the `ansible-playbook --check` decline from USER_STORY rung 3, replayed against a far better tool, with the same answer — but the interesting difference is *why* it declines. Ansible's check-mode is untrustworthy because its least check-aware task is a shell task that does nothing. Nix's dry-activate is untrustworthy because activation is genuinely not fully predictable from the outside, and the maintainers say so in the man page rather than pretending.

And nix is the case where the decline costs nothing, because a *better* check exists beside it. That is the shape to want from an ecosystem: not "the vendor's dry-run is trustworthy" but "the vendor exposes state identity, so we do not need their dry-run."

## §5 — The hermeticity precondition is spelled `flake.lock`

`kVOLATILES-exclude` is welded: hermeticity is a precondition for any sound skip system, not an optimization. The oracle's local-path gate and lock-file check are that weld made concrete.

- A locked local flake evaluates reproducibly. Two evaluations minutes apart agree.
- `github:me/cfg#host` re-resolves its inputs on every evaluation. Two evaluations minutes apart may honestly disagree, and the disagreement is invisible.
- So do `--override-input`, `--impure`, `--recreate-lock-file`, `--upgrade`.

Each of those is an argv-keyed decline in the oracle. This is the cleanest instance I have seen of *the invocation itself carrying the hermeticity fact* — the admin's own command line says whether the check can be trusted, and the oracle's argparse reads it. No annotation, no config, no user declaration: the flags they typed anyway.

**Flagged for the knob entry / synthesis note:** this generalizes past nix. "Does this invocation pin its inputs" is a question many tools' argv answers (`--frozen-lockfile`, `--locked`, `pip install -r` with hashes, `terraform plan -lock-timeout`), and it is the same shape every time — a flag that makes a check hermetic or not. It may deserve a name.

## §6 — The eval cost, honestly

Rung two evaluates a NixOS closure. That is seconds and worsening; it is a tracked, acknowledged problem in the nix world. Under `kPROBING` this still lands well inside the VALUE band — the apply it might avoid is a full build-and-activate, minutes, plus service restarts — but it is not free, and it is the most expensive check in either book.

Rung one exists for that reason and is nearly free: compare `nixos-version --configuration-revision` against `git rev-parse HEAD` on a clean tree. It works only for admins who set `system.configurationRevision = self.rev`, which nix's own templates ship and `nix-darwin`'s generated config includes. When the tree is dirty `self.rev` is null and the rung declines to rung two.

The gradual-enhancement story writes itself and is worth noting because it comes from *the admin's* side rather than the oracle author's: **an admin who adds one line to their flake makes our check an order of magnitude cheaper.** Most of the gradual-enhancement curve in USER_STORY is engineer-side (write an oracle, write a footprint). This is a rung the *admin* buys, in the tool's own idiom, and the hint machinery should be able to say so.

## §7 — Where I was WRONG-shaped and had to back off

Three things I wrote and then deleted, recorded because the deletions are the content:

**wrong-probe-may-write-store.** My first oracle used `nix build --no-link --print-out-paths`. That *realises* the closure: it downloads from caches and writes store outputs. A probe may not do that — it is minutes of work and unambiguously a mutation. `nix eval --raw` computes the same path by evaluation. It still writes `.drv` files into the store as a side effect of instantiation, which is a genuine, small, store-append mutation, and I am recording it rather than hiding it: **the oracle author is vouching that store instantiation is not a mutation of any kind anyone models.** That is precisely the authored judgment `rul-vouch-is-verdict-authoring` exists to carry, and it is exactly the `converged ≠ no-op` adequacy gap wearing a different hat. It should not be laundered into "the probe is read-only".

**wrong-splice-straddles-writeboundary.** Home Manager's DAG has a native check-phase/write-phase split that reads exactly like probe/apply. Splitting ourselves across it is wrong: `writeBoundary` is where Home Manager rewrites $HOME, so a pre-boundary probe is separated from a post-boundary apply by an unmodeled command that may invalidate anything — a poison wall we would have inserted *ourselves*, by choosing to straddle. Every fact would need a guard; the split buys zero elisions and costs a phase. One post-boundary entry has no staleness by construction. (The pre-boundary slot is still useful, for a different job: HM's own words are "verifies, but does not modify … and exits if an unexpected state is found" — a refuse-early seat.)

**wrong-kind-namespace-is-free.** I minted `org.nixos.SystemClosure` without thinking about it, and it is not obviously OK. `kind__resolve()` is keyed by kind and a second declaration is refused loudly, so two independent authors both minting `org.nixos.SystemClosure` produce a hard failure in any book that loads both. USER_STORY says "Nobody approves kind names; there is no registry. It only has to agree with itself" — which is right for a *private* kind and possibly wrong for a reverse-DNS namespace that names somebody else's project. See §9.

## §8 — The splice face, and what nix gives us for free

Charter shape B (splice a store-path invocation) is not merely the nix-idiomatic option here; it is the only correct one, and for a first-party reason. `home.emptyActivationPath` defaults to true and is documented as "recommended … to avoid uncontrolled use of tools found in PATH". A spliced `dorc-run` resolved through PATH would be exactly the uncontrolled use that option exists to prevent.

Three properties fall out, and all three are things every *other* splice target has to build:

- **hygiene** — a separate process cannot clobber the host script's `$oldGenPath`/`$newGenPath`/shell options. The subshell wrapper the fragment-render constraint set calls for is unnecessary here.
- **provenance** — the charter names embedding provenance / a source-map as the fragment render's "one real machinery ask". In the store, the fragment's identity *is* its path: globally unique, immutable, derived from its bytes. Nix hands us the anchor.
- **the no-fork rule has a natural enforcement point** — `dorc compile --fragment` runs at *build* time, inside a derivation, so a refusal is a build failure with a store-path-identified cause. An `exit` in a fragment would, spliced into HM's `set -eu` script, kill every activation block sorted after ours. Refusing at build time is a narrowing; rewriting `exit` to `return` would be a second dialect, which is what chef-solo died of.

**The one hard interface is PATH**, and it is worth stating as a general finding rather than a nix detail: *nix's entire value proposition is that nothing resolves by name; sh's entire idiom is that everything does.* A fragment that says `/nix/store/…-git-2.51.0/bin/git` is no longer a book — it does not run on the laptop with no nix, and the off-ramp is the product. So the dependency declaration has to move **up**, to the nix layer, where closure-completeness is expressible, and the fragment stays name-resolving sh. That is what `dorc.activation.<name>.path = [ pkgs.git ]` is for.

## §mark-carrier-choice — the day-zero book is the `#:` carrier's population

`kTYANNOT` is directional with both poles shipping: the salient colon form is
the default, and the `#:` comment carrier is "OFFERED for raw-exec-inertness".
This book is a live argument for who that offer is *for*, and I think it is
sharper than the knob entry currently states.

The book's headline property is that it runs bare — `curl … | sh` on a naked
laptop, before nix exists, before Dorc exists, before there is anything to
strip it with. The colon form is not inert under a stock shell: a mark
executes, and a colon-form bind on an assignment fails harder still (the
shell reads `:=` as an unknown command). An unstripped colon-form book is not
a runnable book.

So the population is not "authors who prefer comments to salience". It is
**every book whose value proposition includes running before its tool
exists** — day-zero bootstrap books, offline artifacts pre-compilation, and
anything a person might `curl`. For those, the carrier is not a taste choice;
it is the only one that preserves the property the book was written for.

Two costs, met in practice while writing this:

- **Highlight demotion is real and it is worse in an oracle than a book.**
  A `#:` mark greys out in every renderer, and the marks in an oracle are the
  correctness-critical part. `rul-attention-honesty` is a source-side rule
  too, and greying the load-bearing line is the FlowType-annotations failure
  mode the knob explicitly names. In a *book* this barely matters (books
  carry few marks); in an *oracle* it is the whole surface.
- **rc-consuming marks cannot stand alone**, which constrains layout more
  than I expected. `asserts`/`refutes` bind to a statement's exit code, so
  the mark must trail on the same physical line. That is fine until the
  statement and the coordinate are both long — and coordinates with a kind,
  a quoted entity, a selector, and a `reads` clause are long. Twice while
  writing these oracles the honest fix was to **split the function** so the
  marked test was its last statement, rather than to fight the line. That is
  a good outcome and probably a lint hint worth having, but it is a real
  authoring pressure the knob entry does not currently mention.

**Suggested delta for whoever owns `kTYANNOT`:** the pole choice may not be
per-author taste at all. It looks like it wants to be **per-file, decided by
whether the file is ever executed unstripped** — books that bootstrap take
`#:`, oracles shipped for stripping take the salient form, and a file that is
both (as this one is) has a genuine tension that the current framing does not
name. This book resolves it by taking `#:` throughout and paying the greying
cost on its oracles, because runs-bare is load-bearing and salience is not.
I am not confident that is the right general answer.

## §9 — Escalations and flagged questions (conductor)

None of these stopped a book; all are things I declined to decide.

- **ask-kind-namespace-squatting-policy** — a third-party oracle author describing nix mints `org.nixos.*` kinds for a project they do not own. Because `kind__resolve()` refuses a second declaration loudly, two such authors collide hard in any book loading both. "No registry, only has to agree with itself" is right for a private kind and possibly wrong for a reverse-DNS namespace naming somebody else's project. Options I can see: (a) accept it, first-writer-wins, collisions are loud and rare; (b) a convention that unowned descriptions live under the *describer's* namespace (`com.myco.nixos.SystemClosure`), which fragments identity and defeats the point of shared kinds; (c) reserve a `sm.dorc.*`-style community namespace for describe-someone-else's-tool kinds. Not mine to pick.
- **ask-store-instantiation-mutation-class** — is `nix eval` writing `.drv` files into the store an acceptable probe-phase effect? I wrote it as an author-vouched residue (§7). If the answer is "no", the whole rung-two check dies and only rung one survives, which would make `system.configurationRevision` a hard requirement rather than an optimization. This is a real fork in the exhibit's value and I would rather it be ruled than assumed.
- **flag-pin-hermeticity-is-a-shape** — §5's observation that "does this invocation pin its inputs" is an argv-answerable question across many ecosystems. Possible knob-entry or synthesis material; possibly deserves a name.
- **flag-second-host-scope-inside-one-command** — `nixos-rebuild --target-host` puts a second host scope inside a single book line. That is `rul-attribution-is-controller-minted`'s named re-entry trigger ("any second scope becoming representable") arriving from a direction the round was not watching: not a pivot, not a fleet book, just a flag on an ordinary command. The oracle declines it, which is correct and cheap, but the existence of the shape is worth the synthesis note.
- **flag-admin-side-gradual-enhancement-rung** — §6's `system.configurationRevision` rung is bought by the *admin*, not the oracle author. USER_STORY's curve is engineer-side throughout. If admin-side rungs are a real category, the hint machinery has a second audience.
- **flag-mark-carrier-may-be-per-file-not-per-taste** — §mark-carrier-choice. `kTYANNOT`'s pole choice looks like it may be determined by whether a file is ever executed unstripped, rather than by author preference; and a file that is both a bootstrap book and an oracle host has a tension the knob does not name. Possible `KNOBS.md` delta, human-owned.
