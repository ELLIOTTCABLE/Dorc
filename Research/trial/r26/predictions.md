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

## §0. One re-registration, disclosed

The ledger below is the **second** registration. The first (commit `d8831bf7`) predicted a
10-line book whose L8 was `systemctl enable --now nginx`. Between then and the first `dorc plan`
run, that line was split into `systemctl enable nginx` + `systemctl start nginx`, because the engine
cannot model a body that inspects a command's exit status, and `enable --now` establishes two cells
that one exit status cannot witness (`README.md` §4). The oracle now declines `--now` outright.

This re-registration still precedes any look at plan output. The original L8 prediction —
guard/guard — is preserved here rather than deleted; it now applies to L8 and L9 jointly.

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
| L8 | `systemctl enable nginx` | guard | guard | `systemctl__is_converged` vouches @enabled; below-wall ⇒ guard |
| L9 | `systemctl start nginx` | guard | guard | vouches @active; below-wall ⇒ guard |
| L10 | `curl -fsS -o /dev/null http://127.0.0.1:8088/r26` | run | run | honestly unmodeled — no oracle at all; an anonymous wall |
| L11 | `logger -t dorc-r26 "…"` | run | run | modeled, and deliberately declined `unsound`: an append-only log has no state to compare; an *attributed* wall |

**Predicted tally, both worlds: 0 elided, 4 guarded, 7 run** (of 11 tool-lines).
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
| L8 | `systemctl enable nginx` | guard | **elide** |
| L9 | `systemctl start nginx` | guard | **elide** |
| L10 | curl check | run | run (wall) |
| L11 | logger | run | run (wall) |

**Predicted ceiling, converged: 8 elided, 0 guarded, 2 run** — i.e. one bare `apt-get update` at the
top of the book is predicted to cost this kit **all eight** of its elisions. That is the whole lesson
the line exists to teach, and it is why the live run should be done twice: once as written, once with
L1 commented out. Confidence **~SUSPECT** on the exact count, **+SURE** on the direction.

> Measured afterwards: the direction held, the count did not — the real ceiling is **1** elide, not
> 8, for a reason this prediction did not anticipate (§4 m-3). Left standing as written.

## §3. What `dorc plan` actually said (hermetic, hand-fed records)

Renders under `renders/`. Engine tallies count **sites**, not book lines — the four guarded-install
lines carry two sites each, so 11 tool-lines is 16 sites.

| # | predicted (pristine / converged) | actual (pristine / converged) | match? |
|---|---|---|---|
| L1 | run / run | run / run | ✓ |
| L2 | run / run | run / run | ✓ |
| L3 | run / run | run / run | ✓ |
| L4 | run / run | run / run | ✓ |
| L5 | run / run | run / run | ✓ |
| L6 | guard / guard | run / **run** | ✗ (m-2) |
| L7 | guard / guard | **run** / guard | ✗ pristine (m-1) |
| L8 | guard / guard | run / **run** | ✗ (m-2) |
| L9 | guard / guard | **run** / guard | ✗ pristine (m-1) |
| L10 | run / run | run / run | ✓ |
| L11 | run / run | run / run | ✓ |

Engine summaries, verbatim:

- pristine — `sites=16 elide=0 omit=0 guard=0 run=16`
- converged — `sites=16 elide=0 omit=0 guard=2 run=14`
- ceiling (L1 deleted, converged) — `sites=15 elide=1 omit=1 guard=2 run=11`

**The headline prediction held: `elide=0` in both worlds of the book as written.** The bare
`apt-get update` does cost the book every elision, exactly as §1 said and for the reason §1 gave.
Everything else about the shape was wrong in three distinct ways.

## §4. Mismatches and findings

**m-1 — a guard is a converged-world artifact, not a pessimistic wrapper.** Predicted L6–L9 would
guard in *both* worlds; they guard only where the probe actually vouched `holds`. In the pristine
world those cells do not hold, so the sites simply run. This was my misreading of the vocabulary,
not an engine surprise: the guard exists to let a *vouched* line re-verify cheaply at apply, so
there is nothing for it to do when the vouch already says diverged. Harmless, and the safe
direction.

**m-2 — two sites of one command collide onto one synthesized cell, and at most one is licensed.**
Predicted four guards in the converged world; got two. An `is_converged` site is keyed
`dorc-auto:<command>@converged` — the engine synthesizes that coordinate and does **not** read the
`: KIND:ENTITY@SELECTOR` mark in the verdict body (confirmed against the in-repo fixtures: the `kp`
oracle carries a full coordinate mark and still keys `dorc-auto:kp@converged`). So this book's two
`cp` sites share one cell, as do its two `systemctl` sites, and only the later of each pair is
attributed the guard. Directly demonstrated: flipping one `cp` record to `absent` while leaving the
other `holds` makes **neither** guard — the two records meet to ⊤ on the shared cell. No fixture in
the corpus has two `is_converged` sites of the same command, so this is untested territory rather
than a known limitation. It caps any real book hard: books drop many files with `cp`.

**m-3 — only guards above the first mutator site can fold, so the ceiling is ~1, not 8.** Predicted
the L1-less converged book would elide eight lines. It elides **one**: the `ca-certificates` guard,
the only one sitting above every mutator site in the book. The engine's own hint names the next
line's `apt-get install -y curl` as "the first wall". A guarded install's rc is usable for the fold
only while no mutator site precedes it — and each guarded-install line contributes a mutator site of
its own, whether or not that branch turns out dead. So in a run of N `dpkg -s x || apt-get install x`
lines, at most the **first** can elide, and this holds even though every guard reported `holds`.
This is the same firewall the `headline-guarded-realistic` fixture documents ("below mutators ⇒
INVALID ⇒ its rc is withheld"); what is new is how sharply it caps the idiom the `USER_STORY`
leans on. My §2 ceiling was wrong by a factor of eight, and in the direction that matters.

Net honest read: the kit demonstrates the *disposition spectrum* well (declines with classes and
attributed file:line, honest walls, guards, one real elision) and demonstrates *elision volume*
badly. The three mismatches are all engine-shape findings, and m-2 and m-3 are worth more to r26
than the elision count would have been.

## §5. Standing caveats

- The two `cp` sites are relative-path (`./r26-smoke.conf`); the book must be run with this
  directory as cwd or both sites' operands resolve to nothing on the box.
- `cp__is_converged` vouches on content only. A drifted *mode* on `/etc/motd` reads as converged —
  correctly, since plain `cp` would not have fixed the mode either.
- The oracles are written in a deliberately narrow subset of the v0.2 dialect for reasons that have
  nothing to do with taste: three ordinary shell constructs silently void every mark in a marked
  file. See `README.md` §4. Do not "tidy" these files back toward idiom without re-running the
  strip check documented there.
