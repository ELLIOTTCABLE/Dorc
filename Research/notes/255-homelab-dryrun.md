# 255 — homelab field-trial: the paper dry-run (predicted plan-shape + ceiling)

> AI-authored (Opus, single-pass), 2026-07-04. The one non-trivial addition from the round-25
> protocol review (`notes/254` F5; `plans/252` §7): before the human's first real-machine trial,
> do for the homelab book what `USER_STORY.md` did for the webhost — the predicted plan-shape at
> each oracle-coverage stage, plus the perfect-oracle **ceiling** (the missing denominator, so a
> low real-day count reads as "hit the ceiling," not "tool weak").
>
> **THIS IS THEORY.** Every disposition below is *me reading the design*, not the tool running.
> Predictions are confidence-marked (`+SURE`/`~SUSPECT`/`-GUESS`/`--WONDER`); the book is
> **untested pre-VPS**. Prediction-vs-observation on the day IS the anti-woo instrument (`plans/250`
> `human-woo-cool-adversary`); a wrong prediction caught on the day is a **finding**, not a failure.
> Trust `USER_STORY.md` / `DESIGN.md` / the `spike/` fixtures over this where they conflict.
>
> Book: **`notes/255-homelab.book.sh`** (companion; realism-flags inline). This note references it
> and quotes only the sections whose verdicts move between stages.

## §0. The scenario and the single lens

**Target (human-LOCKED 2026-07-04):** one fresh Debian-12 box — an **nginx reverse-proxy** over
**self-signed TLS** fronting **Windmill** (native binary on a system **postgres**) and **Home
Assistant** (Container). The human plays the lazy admin who wants the homelab working, not "fancy
Dorc crap," and grudgingly hand-writes **one** minimal oracle (`plans/250` grounding scenario).

Every ops-choice below was made against **one rubric: does it maximally exercise Dorc?** (the
composition machinery, the ambient-gate, the poison-walls, the guard tier, the converged-vouch, and
the honest residue). The decisions-log (§4) records each choice, its alternatives, and the
exercise-Dorc rationale. Realism was the co-constraint: the commands are what a competent Debian
admin actually runs (grounded in the non-quarantine `pi-webhost.book.sh` and
`17x-strawmen/books/gh-runner.book.sh` idioms), version-pinned for the differential's repeatability
(`plans/252` §7: pin versions, self-signed not Let's-Encrypt, HA-Container not Supervised).

## §0.1 Verdict vocabulary (the mechanics this note applies)

From `USER_STORY.md` + the round-23/24 rulings (`spike/CLAUDE.md`; `notes/23M`, `24A`–`24D`), a
site is one of:

- **elide** — proven already-converged **AND ambient** (no un-elided command upstream disturbed its
  state) **AND** licensed by a reached **converged-vouch** (a `<cmd>.is_converged()` verdict-function
  — the elide-weld: *no vouch ⇒ run*, `24C`). Rendered commented-out (`true # dorc: elided […]`).
  A hand-written idempotence guard whose condition is a liftable read (`dpkg -s x || install x`)
  elides the same way when the guard proves the fallback dead — *and only when ambient*.
- **guard** — converged but sitting **past a poison-wall**: gets a re-verify `( cmd.is_converged args ) || cmd`.
  The original bytes always survive; on a converged apply the check short-circuits, else the command
  runs. Present on screen — costs a probe forever, saves no attention.
- **run** — un-oracled / ⊤ / diverged / vouchless. Runs verbatim.

Two structural facts drive everything:
- **A poison-wall** = any command whose downstream effect Dorc can't bound: an **unmodeled** command
  (silence = wall, `notes/23M`), or a modeled command running an **unmodeled verb**, or any command
  that **runs** (a converged-but-vouched command past a wall *guards*, and a guarded command *might*
  run, so it too walls downstream). Everything below a wall can at best **guard**, never elide.
- **An elided command casts no wall** (`USER_STORY` stage 3; claim-2 of `notes/238`). So the first
  wall's position caps elision; oracling that wall so it *elides* un-walls everything between it and
  the next wall. This is the whole value-curve.

**Adequacy / converged≠no-op** (the primary trial target, `plans/250` `target-adequacy`): a probe
can say "converged" while running the command would still mutate (the `strawman24-adequacy-seed`
fixture: `dpkg -s nginx` reports installed, but a pending upgrade means `apt-get install` would still
act). This is the naked risk under *every* elision, calibrated-never-proven. The book seeds two live
cases (the `[ -f cert ]` guard that never checks expiry; the version-guarded binary a `command -v`
would wave through) — §2/§3.

## §0.2 What is actually BUILT (bounds what the day can reach)

Per `LIVING_STATUS` (2026-07-04): **Stages 1–3 are built + green** — the frame-rule elide machine,
the guard tier (`( check ) || cmd` fires past a wall), and the elide-weld (a full skip demands a
`ByVouch`). **Stages 4–5 (derived footprints, `touches()`, survive-a-running-wall, `--trust-footprints`)
are NOT built** (`254` F4: unbuilt at `2e1fdc0`; NEXT on the ladder). Consequence, load-bearing for
reading the day: the **ceiling (§3) is unreachable on the day** — it needs the footprint tier. The
day can reach only the Stage-B/Stage-C numbers (§2). **A low day-count is an under-built-tier
artifact, not a weak tool** (`254` F4/F5). Do not let the day yeet full-elision on a number the
built tier structurally caps.

## §1. The book and its cast

Full book: `notes/255-homelab.book.sh` (~21 mutation-capable tool-sites + housekeeping). The cast,
by how Dorc sees each command **at the base-stdlib stage** — the assumed ~40-oracle bootstrap stdlib
(`plans/252` P5): apt/dpkg, `pkgindex` (apt-get update), systemctl (`service`), ufw (`firewall`),
cp/`file`, coreutils (chmod/ln/install/rm), nginx (`nginx -t` is read-only):

| kind | commands | Dorc's handle |
|---|---|---|
| **base-oracle-able** (elide/guard) | `apt-get update`, `dpkg -s x \|\| apt-get install x` (×4), `cp`, `chmod`, `install -d`, `ln -sf`, `systemctl enable --now`, `ufw allow`, `nginx -t` | famous; stdlib probes + vouches |
| **the tractable vendor wall** (hand-oracle target) | `windmill --version \| grep -q … \|\| curl … -o …windmill` | unmodeled at base; a 6-line `windmill.is_converged()` oracles it (§2 stage C) |
| **opaque poison-walls** (the honest residue / "horks") | `su - postgres -c "…"` (×2), `docker run … home-assistant`, `systemctl daemon-reload`, `systemctl reload nginx` | unmodeled command, or run-delta verb — never elide at the built stages |
| **conditional / edge** | `[ -f cert ] \|\| openssl req …` (hand-guarded, adequacy trap), the `cat > vhost <<EOF` heredoc (heredoc-refusal edge), `rm -f default` (a KILL) | §2 notes each |

Site IDs `hl-1..hl-21` (used in the ledgers) map top-to-bottom over the book's tool-sites.

The **composition** is deliberately rich (the exercise-Dorc lens, `plans/252` §7 "more services =
more of the analyzer's composition machinery"): windmill reads the postgres DB that the `su`/psql
block created; nginx proxies to the ports windmill (:8000) and HA (:8123) bind; `systemctl enable`
depends on the `apt` installs and the `cp`'d unit; the firewall opens the port nginx listens on.
These are exactly the cross-command shared-state edges the ambient-gate and walls are built to test.

## §2. The per-stage predicted-plan ledger

Counting the 21 tool-sites (housekeeping — `set -eu`, the `case` host-guard, `WM_VER=`,
`echo` — always shows and is excluded from the tallies, as in `USER_STORY`'s ledger).

### Stage A — bare (stdlib disabled): the floor

Illustrative only (the stdlib ships with the tool; real day-one is Stage B). Nothing can be probed
(probing requires an oracle's vouch that a check is read-only). The plan is the book, annotated.

**`plan: 21 run, 0 guard, 0 elided`** — +SURE. The floor promise: *no worse than running the script
blind* (`DESIGN` "no worse than just running the script"). Gained: a plan surface. Lost: nothing.

### Stage B — base stdlib: elision fires above the first wall

The stdlib gives the probe phase something to do. **Steady state** (converged re-run: packages
installed, index fresh, config in place, certs present, windmill binary current):

The elidable cluster is the ambient top — everything above the first wall (`hl-6`, windmill, which
is unmodeled at this stage):

```sh
apt-get update                                     # hl-1  elide: pkgindex fresh
dpkg -s nginx      >/dev/null 2>&1 || apt-get install -y nginx        # hl-2  elide: guard holds, install dead
dpkg -s postgresql >/dev/null 2>&1 || apt-get install -y postgresql   # hl-3  elide
dpkg -s docker.io  >/dev/null 2>&1 || apt-get install -y docker.io    # hl-4  elide
dpkg -s openssl    >/dev/null 2>&1 || apt-get install -y openssl      # hl-5  elide
windmill --version … | grep -q "$WM_VER" || curl … -o …/windmill      # hl-6  RUN — unmodeled ⇒ FIRST WALL
```

Below `hl-6` every base-oracle-able site can at best **guard**; every unmodeled command **runs** and
re-walls:

| site | command | verdict | why | conf |
|---|---|---|---|---|
| hl-1 | `apt-get update` | **elide** | pkgindex converged, ambient | +SURE |
| hl-2..5 | `dpkg -s x \|\| apt-get install x` | **elide** | hand-guard holds, install branch dead, ambient | +SURE |
| hl-6 | `windmill --version… \|\| curl…` | **run** | windmill unmodeled ⇒ **first poison-wall** | +SURE |
| hl-7 | `chmod 755 …/windmill` | guard | coreutils, converged, past hl-6 | ~SUSPECT (stdlib covers chmod?) |
| hl-8 | `cp windmill.service …` | guard | file-content converged, past wall | ~SUSPECT |
| hl-9 | `systemctl daemon-reload` | **run** | run-delta / unmodeled verb ⇒ wall | ~SUSPECT |
| hl-10,11 | `su - postgres -c "…"` | **run** | `su` unmodeled ⇒ wall (payload opaque) | +SURE |
| hl-12 | `systemctl enable --now windmill` | guard | service enabled+active, past wall | +SURE |
| hl-13 | `docker run … home-assistant` | **run** | docker unmodeled ⇒ wall | +SURE |
| hl-14 | `install -d -m 0700 …/certs` | guard | coreutils dir+mode, past wall | ~SUSPECT |
| hl-15 | `[ -f cert ] \|\| openssl req…` | **run** | past a wall, the hand-guard re-checks live (`USER_STORY` st.5) | ~SUSPECT |
| hl-16 | `if [ ! -f ]; cat > vhost <<EOF` | **run** | past wall **and** heredoc-refusal (span can't edit) | ~SUSPECT |
| hl-17 | `ln -sf … sites-enabled` | guard | coreutils symlink, past wall | ~SUSPECT |
| hl-18 | `rm -f …/default` | **run** | a KILL (declines vouch, like `purge`); ⇒ wall | -GUESS |
| hl-19 | `nginx -t && systemctl reload nginx` | **run** | `reload` run-delta ⇒ wall (`nginx -t` is a read) | ~SUSPECT |
| hl-20,21 | `ufw allow …/tcp` | guard | firewall converged, past wall | +SURE |

**`plan: ~9 run, ~7 guard, 5 elided`** (of 21). **The elision is 5/21 ≈ 24%** — concentrated in the
ambient top; the book collapses to guards/runs after the first wall at `hl-6`. This *looks* weak —
which is exactly why §3's ceiling exists. Confidence: the **5 elides are +SURE**; the guard/run split
is ~SUSPECT and rides two unknowns: **(u1)** whether the LLM stdlib carries `is_converged` vouches for
`chmod`/`install -d`/`ln` (if not, hl-7/14/17 flip guard→run, giving ~4 guard / ~12 run); **(u2)**
whether `rm`/`daemon-reload`/`reload` wall as predicted (all ~SUSPECT/-GUESS).

- Gained (steady): three mutation-capable commands (the two-plus `apt`/`dpkg` sites) provably not
  run; the classic hand-written `dpkg -s` guard lifts exactly as `USER_STORY` promises.
- Not gained: **attention** — 16 of 21 sites still face the user. Past `hl-6` there is no proof to be had.

**Drifted day** (representative: package index stale overnight — nothing else changed). `hl-1`
`apt-get update` now **runs** (diverged) ⇒ it becomes an *even earlier* wall, above the `dpkg`
cluster. `hl-2..5` degrade elide→**run-live** (their `dpkg -s` guards re-check past the `hl-1` wall,
per `USER_STORY` stage 5). **`plan: ~14 run, ~7 guard, 0 elided`.** One stale index costs the book
its entire shape — the `USER_STORY`-stage-5 lesson, and the exact pain the (unbuilt) footprint tier
exists to buy back. +SURE on the mechanism.

### Stage C — the two-minute oracle: `windmill.is_converged()`

The hint machinery points at the first wall (`USER_STORY` stage 3: an oracle for the first wall
un-walls downstream). The admin writes the minimal vouch (r24 dialect, mirroring
`strawman24-*/package.oracle.sh`'s `apt-get.is_converged`):

```sh
# minimal windmill oracle: vouch that the pinned binary being present-and-current is convergence,
# so dorc can lift my version-guard, elide the re-download, and stop walling my systemd block.
windmill.is_converged() {
   case "$1" in
   --version) windmill --version 2>/dev/null | grep -q "$WM_VER" ;;   # 0 = current ⇒ converged
   *)         return 2 ;;                                             # decline everything else ⇒ run
   esac
}
```

**Steady state** (binary current): `hl-6`'s version-guard now lifts; the `curl` folds dead; `hl-6`
**elides**. Because `hl-6` was the first wall and its ambient (hl-1..5) all elide, `hl-6` elides
fully (not merely guards) ⇒ **casts no wall** ⇒ un-walls the block between it and the next wall
(`hl-9 daemon-reload`):

```sh
windmill --version … || curl … -o …/windmill      # hl-6  elide: fb wm.Binary current (was RUN)
chmod 755 /usr/local/bin/windmill                  # hl-7  elide: un-walled (was guard)
cp ./windmill.service /etc/systemd/system/…        # hl-8  elide: un-walled (was guard)
systemctl daemon-reload                            # hl-9  run: run-delta ⇒ next wall (caps the un-wall)
```

**`plan: ~8 run, ~5 guard, 8 elided`** (of 21). The two-minute oracle bought **+3 elisions**: its own
line (`hl-6`) **and** the two downstream facts it had been poisoning (`hl-7`, `hl-8`) — the
`USER_STORY`-stage-3 "steepest part of the value-curve" moment, scaled to a homelab. Confidence: **the
+1 (own line) is ~SUSPECT-toward-SURE**; **the +2 un-wall is ~SUSPECT** and rides the sharpest
day-of unknown:

> **⚠ key prediction to verify (u3, ~SUSPECT):** `hl-6`'s guard is `windmill --version | grep -q …`
> — a **stdout-consuming pipe guard**, not a bare rc-guard like `dpkg -s x`. Whether the built spike
> lifts a `cmd | grep -q X || fallback` (reproducing windmill's stdout as a probe-sourced value —
> `inv-probe-sourced-values`, the `consumed-output` fixture family) vs only rc-gated guards is
> genuinely uncertain from the design read. **If it does not lift, Stage C == Stage B (zero gain)**
> — and *that* is a first-class gap-log finding (the admin then rewrites the guard rc-form, or the
> oracle targets the `curl` line directly; the friction itself is `target-admin-loop` signal).

**Drifted day (stale binary — the adequacy demo):** the binary is present but an **old version**.
`windmill.is_converged --version` runs `windmill --version`, sees the mismatch ⇒ **diverged** ⇒
`hl-6` `curl` **runs** (re-downloads) ⇒ re-walls ⇒ `hl-7`,`hl-8` fall back to guard. The plan honestly
re-degrades (`plan: ~8 run, ~7 guard, 5 elided`). **The value that a bare `command -v windmill`
guard could not give:** presence ≠ currency — a `command -v` would have **wrongly elided the stale
binary** (converged≠no-op); the version oracle catches it. This is `target-adequacy` firing on a
hand-authored oracle — a co-primary trial win. +SURE on the mechanism.

## §3. The perfect-oracle ceiling and the un-oracleable residue (the denominator)

**If every command had a perfect oracle** (convergence vouch + a Stage-4/5 `touches()` footprint) and
`--trust-footprints` were on and built — the maximum the *design* can reach on this book, steady
state. **This is the denominator F5 demanded**: without it, a real-day 5–8 misreads as "tool weak"
instead of "hit the built ceiling."

**What the footprint tier buys** (`USER_STORY` stage 5; `notes/238`): a running wall that declares its
footprint no longer poisons downstream facts whose backing is **disjoint**. `apt-get update`'s
footprint (`pkgindex`) is disjoint from the dpkg-db, config files, certs, unit state → the whole
drifted-index collapse (Stage B drifted) is bought back. `daemon-reload`'s footprint (systemd
unit-cache) is disjoint from everything downstream → it runs but stops walling. Entity-granular
package footprints (`strawman24-survive-simple`) let a diverged `apt install oldpkg` not poison a
converged `nginx`.

**What NEVER elides — the residue map** (the honest horks; the plan must show these with reasons, per
`rul-attention-honesty`):

| site | why it is permanent residue | class |
|---|---|---|
| `hl-10,11` `su - postgres -c "…"` | the mutation lives **inside `su`'s `-c` string** — opaque to the analyzer; no general oracle can see the `psql`/`createdb` within. Elidable only by a bespoke per-invocation wrapper-oracle (impractical, naked). | opaque-wrapper |
| `hl-13` HA-internal state | `docker run` *can* vouch "container up at image X" (elides its own line at the ceiling) — but "container running" ≠ "HA configured/working": HA's real state is **inside the container**, un-probe-able from the host. A converged≠no-op that no host oracle closes. | un-probeable-substrate |
| `hl-19` `systemctl reload nginx` | run-delta with **no host-observable convergence** (nginx exposes no "loaded-config == on-disk" hash) — runs every apply, or takes a weak vouch that is itself an adequacy risk. | run-delta |
| `hl-13` docker's network footprint | even oracled, docker's footprint touches iptables/bridges → **intersects `ufw`'s backing** → keeps `hl-20,21` guarded (a real, correct non-elision even at the ceiling). | footprint-collision |

**Ceiling tally (steady, perfect oracles + footprints):** `hl-1..8, 12, 14, 15, 17, 18` elide, `hl-13`
elides-own-line (adequacy-caveated), `hl-16` **guards even here** (heredoc-refusal is a *render* limit
no proof dissolves), `hl-20,21` guard (docker-iptables collision), `hl-9,10,11,19` run. **≈ 14 elide /
~3 guard / ~4 run (of 21) — a ~65–70% ceiling.** -GUESS on the exact split (footprint collisions and
the heredoc are the fuzzy part); the *shape* is +SURE: a mid-teens ceiling with a hard floor of ~4
opaque-wrapper / run-delta / un-probeable sites.

**The three denominators, side by side:**

| tier | elide (of 21) | reachable on the day? |
|---|---|---|
| built spike, base stdlib (Stage B) | ~5 | **yes** |
| built spike, + 1 hand-oracle (Stage C) | ~8 | **yes** (modulo u3) |
| perfect-oracle **ceiling** (Stages 4–5) | ~14 | **NO — footprint tier unbuilt** |
| permanent **floor** (never elides, any tier) | — (~4 sites always run) | — the honest residue |

So: **a real-day count near 5–8 is the built-tier ceiling, not tool-weakness.** The 8→15 gap is
precisely the unbuilt footprint tier + oracles for su/docker/cert. The ~4-site floor is the honest
product statement (`USER_STORY` "the residue"): past the last wall, Dorc makes the book fast and safe,
not shorter — and the human LOCKED docker/HA precisely to exercise that floor.

## §4. Decisions-log (each ops-choice · alternatives · exercise-Dorc rationale)

- **dec-1 · HA via Container (docker), not Supervised/Core.** *[human-locked]* docker is the opaque
  poison-wall / the permanent residue — it exercises the honest-residue floor (§3) and is repeatable
  (pinned image), and it is the human's genuine dogfood want (`plans/252` §7). *Alt:* Supervised is
  more apt/systemd-oracle-able but HA unsupports it on generic Debian and it is a heavier,
  less-repeatable install. Container is the deliberate hork.
- **dec-2 · Windmill as a native binary + systemd, NOT docker-compose.** Keeps windmill **out from
  behind the docker wall** so it is the *tractable* hand-oracle target (the version-guarded download →
  the human's one oracle, §2C) and its unit is service-oracle-able. *Alt (the blessed path):*
  docker-compose would put windmill behind a second docker wall — the whole windmill tier becomes
  opaque residue and the tractable hand-oracle vanishes. **FLAG (realism, ~SUSPECT):** the bare-binary
  path is less-documented than compose; if it doesn't run on the day, fall back to compose and the
  trial adapts (windmill joins the residue; postgres+nginx still exercise elision). Pending research
  confirmation of the release-asset name + `DATABASE_URL` env + default port (:8000).
- **dec-3 · PostgreSQL as a system service (apt), NOT a container.** (a) apt/systemctl-oracle-able ⇒
  elision fires on install + service; (b) a **real drift-able DB** — the live `target-adequacy`
  substrate the trial is built to probe; (c) shared by windmill ⇒ a rich cross-service edge. *Alt:*
  postgres-in-docker (windmill's compose default) is opaque behind the docker wall with no host-probe
  of the DB.
- **dec-4 · postgres role/db via `su - postgres -c "…"`.** *Forced* by Debian peer-auth. **This is a
  genuine finding, not just a choice:** the idiomatic postgres-provisioning spelling is a poison-wall
  (`su` wraps an opaque `-c` payload the analyzer cannot see into) — honest residue *and* a realistic
  friction the day will surface. *Alt (pg_hba edits / running as a pg-authed user):* more setup, still
  opaque. Kept to exercise the opaque-wrapper residue class (§3).
- **dec-5 · Self-signed certs via `openssl`, not Let's Encrypt.** *[human-locked, repeatability]* LE
  needs live ACME (network chaos breaks the differential, `plans/252` §7). openssl is unmodeled (a
  wall) but hand-guarded by `[ -f ]` — and that guard **never checks expiry**, making the cert the
  cleanest **converged≠no-op** demo (a present-but-expired cert wrongly "converges"). A strong
  secondary hand-oracle candidate (`openssl x509 -checkend`) if the day wants a second adequacy case.
- **dec-6 · Version-checked download (`windmill --version | grep || curl`), not bare `curl` or
  `command -v`.** Realistic (gh-runner uses a `[ -f config.sh ]` download guard; version-checking is
  the more-correct idiom) **and** it is the hand-oracle target: a `command -v` presence-guard is an
  un-shimmable builtin (non-mock-reproducible — the reason the headline fixtures switched to `dpkg -s`)
  *and* misses version (an adequacy gap). **FLAG (u3):** whether Dorc lifts the stdout-consuming form
  is the sharpest day-of unknown (§2C).
- **dec-7 · Book order: packages → windmill-binary staged → postgres → windmill-up → HA → proxy →
  firewall.** Positions the windmill download as the **first tractable wall** (above its own systemd
  block) so the hand-oracle un-walls downstream (the value-curve demo). *Alt (strict backends-first,
  postgres before windmill):* puts the `su` wall first ⇒ collapses the whole book below it at base
  stage and kills the un-walling demo — noted as a **contrast variant the day could also run** (it is
  even more wall-dominated; a useful second data-point on how order shapes the plan).
- **dec-8 · `docker.io` (Debian-native), not docker-ce (official repo).** Single apt install ⇒
  apt-oracle-able (elides). *Alt:* the official-repo route adds `curl | gpg --dearmor` + a repo file —
  more opaque walls, orthogonal to what we're exercising. FLAG: `docker.io` is older (20.10); fine for
  `docker run`.
- **dec-9 · `set -eu` + scrappy hand-guards, flat sections (the lazy-admin shape).** This is the input
  Dorc infers best from — the hand-guards *are* the oracle material it lifts (`USER_STORY`: "years of
  defensive habit turn out to have been oracle material") — and what the human will actually write.
  `set -eu` exercises the errexit-honesty path (a guard's `||`-left is errexit-exempt by design).
- **dec-10 · A heredoc vhost write (`cat > … <<EOF`) + `cp`'d unit file.** The heredoc exercises the
  **heredoc-refusal edge** (`render21-heredoc-refusal`: the leaf's span covers `<<EOF`, not the body,
  so it refuses render-edit even when converged); the `cp` unit exercises the clean file-content
  oracle. Both are realistic and hit different analyzer surfaces on purpose.

## §5. Pre-registered predictions and caveats (the anti-woo instrument)

**Pre-registered (set BEFORE the day, per `plans/252` §7 F1 — vibe-words are post-hoc-gradeable):**

- **pred-1:** Stage B steady elides **exactly the ambient package cluster hl-1..5 (5 sites)**;
  everything from the first wall (hl-6) down is guard/run. +SURE.
- **pred-2:** Stage C's one oracle moves the count **+3 (to 8)** — its own line plus hl-7,hl-8 —
  *iff* the stdout-consuming guard lifts (u3). If it lifts, ✓; if not, Stage C == Stage B and **that
  is the finding**, not a null result. ~SUSPECT.
- **pred-3:** the day's built-tier elision lands in **[5, 9] of 21 (~24–43%)**; anything materially
  higher means my wall-map is wrong (a finding — likely the stdlib vouches more coreutils than I
  assumed, u1); materially lower means a vouch/lift I predicted didn't fire (a finding — check
  `dorc why` per `plans/252` B2 confound-isolation).
- **pred-4:** `su` (hl-10,11), `docker run` (hl-13), and `systemctl reload` (hl-19) **run at every
  stage** and appear on every plan with a reason. If any *elides* on the built tier, my residue map is
  wrong (a surprising, valuable finding). +SURE they run.
- **pred-5:** the **ceiling is unreachable on the day** (footprint tier unbuilt); the drifted-index
  collapse (Stage B drifted → 0 elide) will reproduce and is **expected**, not a regression (`254` F4).

**Caveats — where reality may diverge from this read:**
- **c-1** These dispositions are my reading of `USER_STORY` + the fixtures, not the tool. The whole
  point is prediction-vs-observation; expect misses, bank them as findings (`plans/250`).
- **c-2** The book is **untested pre-VPS**. The `# FLAG:` lines (windmill binary path/port, HA tag,
  the `docker run` non-idempotence, the `su` quoting) are the realism-risks; some may not run
  first-try. Pending web-doc research reconciliation (versions/asset-names/env) — a follow-up pass on
  this note and the book.
- **c-3** The base-stdlib coverage of coreutils (chmod/ln/install-d/rm) and the exact verbs the LLM
  stdlib vouches are assumptions (u1); they swing the guard/run split but **not** the elide count
  (pred-1 holds regardless).
- **c-4** u3 (stdout-consuming guard lift) is the single most load-bearing uncertainty for the Stage-C
  value story; put it first on the day's `dorc why` checklist.
- **c-5** "attention-lines saved" (the real value-prop, `DESIGN` priority 3) is barely moved on this
  book at the built tier (16→13 of 21 face the user) — because the walls are real and early. That is
  honest, and it is the case *for* the footprint tier, not against the tool. Read the day through §3.

## §6. Summary ledger (`USER_STORY`-style)

```
stage                              elide  guard  run   attention   reachable on the day?
A  bare (stdlib disabled)            0      0     21     21         illustrative only
B  base stdlib — steady              5      7      9     16         YES
B  base stdlib — drifted (stale idx) 0      7     14     21         YES (expected collapse)
C  + windmill oracle — steady        8      5      8     13         YES (modulo u3)
C  + windmill oracle — drifted-bin   5      7      9     16         YES (adequacy catch)
—  perfect-oracle CEILING           ~14    ~3     ~4     ~7         NO (footprint tier unbuilt)
—  permanent FLOOR (never elides)   —      —      ~4     —          the honest residue
```

The shape to carry to the day: **the built tier tops out near Stage C (~8/21); the ceiling (~14/21)
needs Stages 4–5; and ~4 sites (`su`×2, `docker`/HA-internal, `systemctl reload`) are the honest
residue that never elides at any tier.** A number in the 5–8 band on the day is the design working as
predicted at the built frontier — not the tool failing.

## §7. Appendix — the sidecar config the book `cp`s

The book inlines the nginx vhost as a heredoc (hl-16) but `cp`s one external file
(`./windmill.service`, hl-8). Its content, sitting beside the book on the day:

```ini
# windmill.service  — FLAG: env-var names (MODE/PORT/DATABASE_URL), the run-user, and whether a
# single binary serves both server+worker are pending first-party-doc confirmation (dec-2 realism).
[Unit]
Description=Windmill (server+worker)
After=network-online.target postgresql.service
Wants=network-online.target postgresql.service

[Service]
Environment=DATABASE_URL=postgres://windmill:changeme@127.0.0.1:5432/windmill
Environment=MODE=standalone
Environment=PORT=8000
ExecStart=/usr/local/bin/windmill
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Port consistency check: the unit sets `PORT=8000`; the vhost's `location /windmill/` proxies to
`127.0.0.1:8000`; HA (host-net) binds `:8123`, proxied by `location /`. `DATABASE_URL` points at the
role/db the `su`/psql block (hl-10,11) creates. These are the cross-service edges §1 relies on.

## §8. Owed — reconciliation pending

A parallel web-doc research pass (first-party nginx/postgres/windmill/HA/openssl docs) is owed to
firm the `# FLAG:` lines in the book and this appendix: the Windmill release-asset name + whether the
bare-binary path is real vs compose-only (dec-2), the default port + env-var names, the HA image tag,
and the `docker.io`-vs-docker-ce call (dec-8). None of these move the **predictions** (§2/§3 are
robust to the exact commands — they turn on the *shape*: base-oracle-able cluster, first wall,
opaque-wrapper residue); they only firm the book's day-one runnability (c-2). If the bare-binary path
proves unreal, dec-2's compose fallback applies and the windmill tier joins the residue — a
pre-registered contingency, not a surprise.
