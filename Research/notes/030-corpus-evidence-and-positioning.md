# Real-world corpus evidence + positioning vs ShellCheck

Source: "Bash in the Wild" (TOSEM 2022) [A-bash-in-the-wild-tosem-2022] — 1.35M GitHub bash scripts (general) + 14k from top-1k repos. Broader/messier population than CoLiS's Debian maintainer scripts, so it stress-tests the linearity bet against the user's actual target (note: RQ3 sample projects include **RetroPie-Setup** and **LinuxGSM** — literally game-server/console setup scripts, the planning-log's canonical use-case).

## Feature usage — the linearity bet holds across the broad population
SURE: real bash is shallow and mostly-linear.
- `if` 70% (general) / 58% (top-1k); `for` <40%; `while` <30%; `case`/`until`/`select` low. → "scripts follow a more linear fashion with only conditional branches." Matches CoLiS's Debian finding *and* the planning-log "flat forest" intuition — now on 1.35M scripts.
- Arrays ~10–15% (rare). Recursion effectively absent (CoLiS found zero).
- **`eval` 9.2% general / 4.2% top-1k.** This is the hard `unsafe`/⊤ boundary, and it's a minority — ~90%+ of scripts are eval-free, i.e. inside the analyzable fragment. GUESS: the user's "homelab + provisioning" target is cleaner still.
- Most-used features (both datasets, ~same order): variable, command-substitution, `if`, parameter-expansion, redirection, pipeline, special-parameter, grouping `{}`, function-definition, filename-expansion.

## The bootstrap-oracle priority list (answers "ship which ~40-50 oracles first")
SURE — this is the empirically-ranked corpus for the built-in oracle set (planning-log's bootstrapping mitigation), no guessing required:
- **Top builtins:** `echo`(78%), `[`/`test`(56%), `exit`, `cd`, `set`, `pwd`, `export`, `source`/`.`, `shift`, `read`, `local`, `return`, `printf`, `eval`, `break`, `exec`, `unset`, `trap`, `declare`, `getopts`.
- **Top GNU coreutils:** `rm`(33%), `mkdir`(30%), `cat`(29%), `dirname`(23%), `cp`(23%), `date`, `sleep`, `mv`, `basename`, `ls`, `cut`, `chmod`, `tr`, `true`, `touch`, `head`, `wc`, `ln`, `uname`, `tee`, `sort`, `tail`, `readlink`, `mktemp`, `chown`, `id`, `seq`.
- File/path/directory utilities dominate → the file-as-defer decision (planning log) is validated as the highest-frequency domain; the convergence-predicate-around-deferred-file-tools is where most oracle value concentrates.
- Total scope to cover the bulk: ~57 builtins + ~102 coreutils ≈ **159 commands** (Dong et al.'s study scope); the *top ~30 of each* covers the overwhelming majority of files.

## Positioning vs ShellCheck (the de-facto baseline) — the niche is open at our altitude
SURE and important for the "what is this" framing:
- ShellCheck (Haskell, 22k★, since 2012) is the *state-of-practice* and the *only* good shell static-analyzer ("no other good static analysis tool for Bash"). It is a **linter**: finds quoting/word-splitting/error-handling smells via patterns during parse.
- RQ3's #2 real-world bug theme — **"lack of existence checking of resources; developers assume static resources exist"** — is explicitly **NOT covered by ShellCheck** ("ShellCheck does not check the existence of resources and is not able to detect such bugs"). *That existence/state check is exactly Dorc's altitude* (check-before-mutate, derived/composed). So Dorc is not a better linter — it operates one level up (state/effect/composition), on the bug class the incumbent structurally cannot reach.
- Command-option bugs (invalid flags) are flagged as statically catchable via "mandb or a custom command-option database" — this *is* the Dorc oracle's "mirror the CLI, fail-fast on unparsable flags" contract (the user's latest decision). The literature independently arrives at our oracle design.
- Consequence for Phase 2: **ShellCheck is a candidate front-end reuse** (smell-filter / sanity layer), not a competitor. And "static analysis tools are insufficient at analyzing real-world bash" confirms the analysis-level niche is unoccupied (matches the planning-log's final landscape verdict).

## RQ3 bug taxonomy (200 bug-fixing commits) — what Dorc must model or cede
57 Bash-semantic bugs, themes: Quoting/word-splitting 15.7% (ShellCheck mostly catches), **File/Path/Dir existence 14.0% (ShellCheck can't — Dorc's core)**, Command-options 10.5% (DB-catchable → oracle contract), Permission 10.5% (cede to transport/`become`, but "permission" is a Tier-B fact), Error-handling 10.5% (`|| true` / `&&`-chaining — our exit-status/CFG modeling). Application-semantic bugs (122) are domain-specific/generic — out of scope (not derivable; this is the planning-log's "can't discover latent deps" wall).

## Net implication for MH1 feasibility (the de-risking spike)
Two independent corpora (Debian 28k; GitHub 1.35M) agree: provisioning/management shell is short, constant-heavy, shallow-control-flow, eval-rare. The planning-log spike ("measure clean-parse % and ⊤-bound %") is *very likely to come back favorable* — but should still be run on the **user's own homelab corpus**, because both studies' populations differ from "one admin's machines" and the ⊤-bound rate (external/non-deterministic reads) was not directly measured by either study. SUSPECT the real risk isn't parse-rate (99.9% in CoLiS) but the *oracle-coverage bootstrap* and the *quoting/word-splitting fragility* (80% of scripts have ≥1 smell) — i.e. the analyzer must treat unquoted expansion as a first-class hazard, not an edge case.
