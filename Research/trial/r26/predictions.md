# r26 live-smoke — prediction ledger

Pre-registered. Every "predicted" cell below was written and committed **before** `dorc plan` was
run even once against this kit (the git history is the evidence: this file's first commit precedes
the first `renders/` commit). Where the engine later disagreed, the disagreement is recorded in the
*actual* column and discussed under §4 — predictions are never edited to match reality.

Vocabulary, per the project's terms (never "skip"):

| term | meaning |
|---|---|
| **elide** | the site is provably converged and does not ship as a runnable command at all |
| **guard** | the site ships wrapped, `( oracle_check ) \|\| original-bytes` — the check re-runs live at apply |
| **run** | the site ships verbatim; no license was earned |
| **wall** | a site that runs *and* invalidates downstream facts, capping elision below it |

Two worlds:

- **pristine** — the box as `26E` describes it, immediately post-provision. Debian 12 cloud image:
  `ca-certificates` already installed; `curl`, `jq`, `nginx` absent; neither config file present;
  no nginx unit.
- **converged** — the same box after one successful run of `smoke-book.sh`: four packages present,
  both config files byte-identical to their sources, `nginx` enabled and active.

## §1. The predicted ledger

| # | book line | pristine | converged | reason |
|---|---|---|---|---|
| L1 | `apt-get update` | run | run | apt-get oracle declines `update` (classed `unmodeled`); no vouch exists, so it runs — **and, running, it walls everything below** |
| L2 | `dpkg -s ca-certificates \|\| apt-get install -y …` | run | run | guard would lift, but sits below the L1 wall ⇒ degrades to run-live |
| L3 | `dpkg -s curl \|\| apt-get install -y curl` | run | run | same; below-wall guard |
| L4 | `dpkg -s jq \|\| apt-get install -y jq` | run | run | same; below-wall guard |
| L5 | `dpkg -s nginx \|\| apt-get install -y nginx` | run | run | same; below-wall guard |
| L6 | `cp ./r26-smoke.conf /etc/nginx/conf.d/…` | guard | guard | `cp__is_converged` vouches by content, but below the wall a vouch buys a guard, not an elision |
| L7 | `cp ./r26-motd /etc/motd` | guard | guard | same |
| L8 | `systemctl enable --now nginx` | guard | guard | `systemctl__is_converged` vouches @active, disclosing the @enabled read; below-wall ⇒ guard |
| L9 | `curl -fsS -o /dev/null http://127.0.0.1:8088/r26` | run | run | honestly unmodeled — no oracle at all; an anonymous wall |
| L10 | `logger -t dorc-r26 "…"` | run | run | modeled, and deliberately declined `unsound`: an append-only log has no state to compare; an *attributed* wall |

**Predicted tally, both worlds: 0 elided, 3 guarded, 7 run** (of 10 tool-lines).
Confidence: the L1 decline and the L9/L10 walls are **+SURE**. The claim that a *running, unvouched*
L1 caps every site below it to guard-or-run is **~SUSPECT** — it is the `255` §2 stage-B/stage-5
mechanism applied to this book, not something re-measured here.

## §2. The counterfactual ceiling (L1 deleted)

The interesting number, and the reason L1 is worth keeping as a teaching artifact rather than
quietly dropping: what the same book is worth with its single bare index-refresh removed.

| # | book line | pristine | converged (the ceiling) |
|---|---|---|---|
| L2 | ca-certificates guard | **elide** (already installed on the image) | **elide** |
| L3 | curl guard | run (genuinely absent ⇒ installs ⇒ becomes the wall) | **elide** |
| L4 | jq guard | run (below L3's fresh wall) | **elide** |
| L5 | nginx guard | run (below L3's wall) | **elide** |
| L6 | conf drop | guard | **elide** |
| L7 | motd drop | guard | **elide** |
| L8 | `enable --now` | guard | **elide** |
| L9 | curl check | run | run (wall) |
| L10 | logger | run | run (wall) |

**Predicted ceiling, converged: 7 elided, 0 guarded, 2 run** — i.e. one bare `apt-get update` at the
top of the book is predicted to cost this kit **all seven** of its elisions. That is the whole lesson
the line exists to teach, and it is why the live run should be done twice: once as written, once with
L1 commented out. Confidence **~SUSPECT** on the exact count, **+SURE** on the direction.

## §3. What `dorc plan` actually said (hermetic, hand-fed records)

Filled in after the fact; renders under `renders/`.

| # | predicted (pristine / converged) | actual (pristine / converged) | match? |
|---|---|---|---|
| L1 | run / run | | |
| L2 | run / run | | |
| L3 | run / run | | |
| L4 | run / run | | |
| L5 | run / run | | |
| L6 | guard / guard | | |
| L7 | guard / guard | | |
| L8 | guard / guard | | |
| L9 | run / run | | |
| L10 | run / run | | |

## §4. Mismatches and findings

To be written against §3, honestly, including anything that makes the kit look worse than predicted.

## §5. Standing caveats

- The two `cp` sites are relative-path (`./r26-smoke.conf`); the book must be run with this
  directory as cwd or both sites' operands resolve to nothing on the box.
- `cp__is_converged` vouches on content only. A drifted *mode* on `/etc/motd` reads as converged —
  correctly, since plain `cp` would not have fixed the mode either.
- The oracles are written in a deliberately narrow subset of the v0.2 dialect for reasons that have
  nothing to do with taste: three ordinary shell constructs silently void every mark in a marked
  file. See `README.md` §4. Do not "tidy" these files back toward idiom without re-running the
  strip check documented there.
