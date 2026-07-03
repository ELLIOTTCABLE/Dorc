# 15x — round-15 strawmen (verbatim; preserved in-tree 2026-06-06)

Real-shaped strawman scripts from the round-15 adversarial premise-review (`notes/150`/`151`), copied
byte-for-byte (sha256-verified) from the now-prunable `01Xpbd` review worktree where they were authored as
untracked scratch. Preserved here because they are the only **non-quarantined** concrete "spell it in sh"
artifacts (round-16's are `quarantine-DO-NOT-READ`), and round-17 (`plans/170`, the K1 brief) depends on them.

> **Caveat — illustrative, not a model.** The oracles are **command-centric** (probe = dry-run the mutator),
> the form `16P` DP-1 *refuted* in favour of fact-centric. Read them for the `predict()`-shape **taxonomy** and
> the **hazards**, never as the recommended contract. As authored they also fail `dash -n` (dotted function
> names) — the off-ramp survives only as a mechanical rename (`notes/151` X4). Don't "fix" them; they are
> frozen evidence.

## oracles/ — one per `predict()`-spelling shape (the engineer side)
- `apt-get.straw.sh` — **dry-run-flag** (`apt-get --simulate | grep`); carries the `--option=` arg-guard leak.
- `systemctl.straw.sh` — **query-verb** (read-verb siblings `is-active`/`is-enabled`); the **structured-kind**
  case in the wild — `service{enabled,active}`, two selectors (the live `dq-entity-algebra` instance).
- `ufw.straw.sh` — **parse-status** (`grep ufw status`); the `.`-as-regex **silent wrong-skip** (`10.0.0.1`
  matches `10X0X0X1`) from a *defensive* author (X4, empirically run).
- `nginx.straw.sh` — **config-test** (`nginx -t`); validate ≠ convergence (the misfit finding — `-t` is a
  precondition, not a skip-check).

## books/ — the inference⟂quality pair (`notes/151` X4 THE-ONE)
- `pi-webhost.straw.sh` — scrappy admin book; richly inferable (`case $(hostname)`, `if ! command -v`, bare
  `ufw allow` / `systemctl enable --now`, `[ ! -f ]` guard, `nginx -t && reload`, sentinel `touch`).
- `deploy-widget.sh` — careful engineer's script; functions + `readonly` const-fold + heredoc desired-state +
  `mktemp`→`cp`→`mv` atomic-publish + `cmp -s` idempotency → drives Dorc to ⊤. The proof that *good structure
  removes every cheap inference handle* — the input the cheap optimizer fires on *least*.
