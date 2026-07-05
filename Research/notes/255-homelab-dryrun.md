# 255 — homelab field-trial: the paper dry-run (predicted plan-shape + ceiling)

> AI-authored (Opus, single-pass), 2026-07-04. The one non-trivial addition from the round-25
> protocol review (`notes/254` F5; `plans/252` §7): before the human's first real-machine trial,
> do for the homelab book what `USER_STORY.md` did for the webhost — the predicted plan-shape at
> each oracle-coverage stage, plus the perfect-oracle **ceiling** (the missing denominator, so a
> low real-day count reads as "hit the ceiling," not "tool weak").
>
> **RE-POINTED 2026-07-04:** the service tier swapped Windmill → a **lean OpenTelemetry stack**
> (otel-collector + prometheus + grafana), same method + structure, three vendor walls instead of
> one (why: `LIVING_STATUS`; §0/§4/§8 below). The elision *mechanics* are unchanged; the *numbers*
> and the value-curve are re-derived for 30 tool-sites.
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
**self-signed TLS** fronting a **lean OTel monitoring stack** — **otel-collector**
(`otelcol-contrib`), **prometheus**, and **grafana** (native binary on a system **postgres**) — plus
**Home Assistant** (Container). Each of the three OTel services is a **separate native systemd unit**
brought up by a **version-guarded binary download**. The human plays the lazy admin who wants the
homelab working, not "fancy Dorc crap," and grudgingly hand-writes minimal oracle(s) (`plans/250`
grounding scenario).

**Why the OTel stack replaced Windmill (2026-07-04):** windmill's *native* multi-unit install is
admin-invented (upstream documents only docker/compose); docker-compose would hide the multi-service
behind one opaque `docker compose up` Dorc can't exercise, and is redundant with HA's docker wall.
The OTel stack gives genuine multi-service as **separate native units Dorc can see**, **documented**
installs (§8: all three have first-party native paths, unlike windmill), and it is the human's
familiar ground. Net effect on this dry-run: **three tractable vendor walls, not one** — so the
Stage-C hand-oracle value-curve is walked three times (§2C).

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
  the next wall. This is the whole value-curve. **With three vendor walls in a row (§2), oracling
  them un-walls three blocks in sequence — the curve, walked thrice.**

**Adequacy / converged≠no-op** (the primary trial target, `plans/250` `target-adequacy`): a probe
can say "converged" while running the command would still mutate (the `strawman24-adequacy-seed`
fixture: `dpkg -s nginx` reports installed, but a pending upgrade means `apt-get install` would still
act). This is the naked risk under *every* elision, calibrated-never-proven. The book seeds two live
cases (the `[ -f cert ]` guard that never checks expiry; the version-guarded binaries a `command -v`
would wave through — now **three** of them) — §2/§3.

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

Full book: `notes/255-homelab.book.sh` (**~30 mutation-capable tool-sites** + housekeeping — up from
21, because three services carry three downloads + three config/unit blocks). The cast, by how Dorc
sees each command **at the base-stdlib stage** — the assumed ~40-oracle bootstrap stdlib (`plans/252`
P5): apt/dpkg, `pkgindex` (apt-get update), systemctl (`service`), ufw (`firewall`), cp/`file`,
coreutils (install/ln/rm), nginx (`nginx -t` is read-only):

| kind | commands | Dorc's handle |
|---|---|---|
| **base-oracle-able** (elide/guard) | `apt-get update`, `dpkg -s x \|\| apt-get install x` (×4), `cp` (×5 configs+units), `install -d` (×3), `ln -sf`, `systemctl enable --now` (×3), `ufw allow` (×2), `nginx -t` | famous; stdlib probes + vouches |
| **the THREE tractable vendor walls** (hand-oracle targets) | `otelcol-contrib --version \| grep -q … \|\| {curl…tar}`, `prometheus --version \| grep -q … \|\| {…}`, `grafana --version \| grep -q … \|\| {…}` | unmodeled at base; a 6-line `<svc>.is_converged()` oracles each (§2 stage C) |
| **opaque poison-walls** (the honest residue / "horks") | `su - postgres -c "…"` (×2), `docker run … home-assistant`, `systemctl daemon-reload`, `systemctl reload nginx` | unmodeled command, or run-delta verb — never elide at the built stages |
| **conditional / edge** | `[ -f cert ] \|\| openssl req …` (hand-guarded, adequacy trap), the `cat > vhost <<EOF` heredoc (heredoc-refusal edge), `rm -f default` (a KILL) | §2 notes each |

Site IDs `hl-1..hl-30` (used in the ledgers) map top-to-bottom over the book's tool-sites:

```
hl-1  apt-get update                        hl-16 systemctl daemon-reload            [WALL]
hl-2  dpkg nginx      || install            hl-17 su - postgres  (role grafana)      [WALL]
hl-3  dpkg postgresql || install            hl-18 su - postgres  (db grafana)        [WALL]
hl-4  dpkg docker.io  || install            hl-19 systemctl enable --now otelcol
hl-5  dpkg openssl    || install            hl-20 systemctl enable --now prometheus
hl-6  otel  download  (ver-guard)  [WALL 1] hl-21 systemctl enable --now grafana
hl-7  install -d /etc/otelcol-contrib       hl-22 docker run homeassistant           [WALL]
hl-8  cp otelcol-config.yaml                hl-23 install -d /etc/nginx/certs
hl-9  cp otelcol-contrib.service            hl-24 [ -f cert ] || openssl req         (adequacy)
hl-10 prom  download  (ver-guard)  [WALL 2] hl-25 if [ ! -f ]; cat > vhost <<EOF     (heredoc)
hl-11 install -d /etc/prometheus …          hl-26 ln -sf … sites-enabled
hl-12 cp prometheus.yml                      hl-27 rm -f … default                    (a KILL)
hl-13 cp prometheus.service                  hl-28 nginx -t && systemctl reload nginx [WALL]
hl-14 grafana download (ver-guard) [WALL 3]  hl-29 ufw allow 22/tcp
hl-15 cp grafana.service                     hl-30 ufw allow 443/tcp
```

The **composition** is deliberately rich — richer than windmill's single-consumer graph (the
exercise-Dorc lens, `plans/252` §7 "more services = more of the analyzer's composition machinery"):
grafana reads the postgres DB the `su`/psql block created (hl-17,18 → hl-21 via `GF_DATABASE_*`);
grafana queries prometheus as a datasource (:9090); prometheus receives remote-write from
otel-collector; nginx proxies the ports grafana (:3000) and prometheus (:9090) bind; each
`systemctl enable` depends on the `apt` installs and the `cp`'d unit for its service; the firewall
opens the port nginx listens on. These are exactly the cross-command shared-state edges the
ambient-gate and walls are built to test — now four services deep.

## §2. The per-stage predicted-plan ledger

Counting the 30 tool-sites (housekeeping — `set -eu`, the `case` host-guard, the three `*_VER=`
assignments, `echo` — always shows and is excluded from the tallies, as in `USER_STORY`'s ledger).

### Stage A — bare (stdlib disabled): the floor

Illustrative only (the stdlib ships with the tool; real day-one is Stage B). Nothing can be probed
(probing requires an oracle's vouch that a check is read-only). The plan is the book, annotated.

**`plan: 30 run, 0 guard, 0 elided`** — +SURE. The floor promise: *no worse than running the script
blind* (`DESIGN` "no worse than just running the script"). Gained: a plan surface. Lost: nothing.

### Stage B — base stdlib: elision fires above the first wall

The stdlib gives the probe phase something to do. **Steady state** (converged re-run: packages
installed, index fresh, configs in place, certs present, all three binaries current):

The elidable cluster is the ambient top — everything above the first wall (`hl-6`, the otel
download, which is unmodeled at this stage):

```sh
apt-get update                                     # hl-1  elide: pkgindex fresh
dpkg -s nginx      … || apt-get install -y nginx   # hl-2  elide: guard holds, install dead
dpkg -s postgresql … || apt-get install …          # hl-3  elide
dpkg -s docker.io  … || apt-get install …          # hl-4  elide
dpkg -s openssl    … || apt-get install …          # hl-5  elide
otelcol-contrib --version | grep -q "$OTEL_VER" || {curl…tar}   # hl-6  RUN — unmodeled ⇒ FIRST WALL
```

Below `hl-6` every base-oracle-able site can at best **guard**; every unmodeled command **runs** and
re-walls. Crucially, the two *later* vendor downloads (`hl-10` prometheus, `hl-14` grafana) are
*also* unmodeled at base ⇒ each is its own wall, so their config/unit blocks (`hl-11..13`,`hl-15`)
guard rather than elide:

| site | command | verdict | why | conf |
|---|---|---|---|---|
| hl-1 | `apt-get update` | **elide** | pkgindex converged, ambient | +SURE |
| hl-2..5 | `dpkg -s x \|\| apt-get install x` | **elide** | hand-guard holds, install branch dead, ambient | +SURE |
| hl-6 | `otelcol-contrib --version… \|\| {curl…}` | **run** | otel unmodeled ⇒ **first poison-wall** | +SURE |
| hl-7 | `install -d /etc/otelcol-contrib` | guard | coreutils dir, converged, past hl-6 | ~SUSPECT (stdlib covers install-d? u1) |
| hl-8,9 | `cp otelcol-config.yaml / .service` | guard | file-content converged, past wall | ~SUSPECT |
| hl-10 | `prometheus --version… \|\| {…}` | **run** | prometheus unmodeled ⇒ **2nd wall** | +SURE |
| hl-11 | `install -d /etc/prometheus …` | guard | coreutils, past wall | ~SUSPECT (u1) |
| hl-12,13 | `cp prometheus.yml / .service` | guard | file-content, past wall | ~SUSPECT |
| hl-14 | `grafana --version… \|\| {…}` | **run** | grafana unmodeled ⇒ **3rd wall** | +SURE |
| hl-15 | `cp grafana.service` | guard | file-content, past wall | ~SUSPECT |
| hl-16 | `systemctl daemon-reload` | **run** | run-delta / unmodeled verb ⇒ wall | ~SUSPECT |
| hl-17,18 | `su - postgres -c "…"` | **run** | `su` unmodeled ⇒ wall (payload opaque) | +SURE |
| hl-19,20,21 | `systemctl enable --now <svc>` | guard | service enabled+active, past wall | +SURE |
| hl-22 | `docker run … home-assistant` | **run** | docker unmodeled ⇒ wall | +SURE |
| hl-23 | `install -d -m 0700 …/certs` | guard | coreutils dir+mode, past wall | ~SUSPECT (u1) |
| hl-24 | `[ -f cert ] \|\| openssl req…` | **run** | past a wall, the hand-guard re-checks live (`USER_STORY` st.5) | ~SUSPECT |
| hl-25 | `if [ ! -f ]; cat > vhost <<EOF` | **run** | past wall **and** heredoc-refusal (span can't edit) | ~SUSPECT |
| hl-26 | `ln -sf … sites-enabled` | guard | coreutils symlink, past wall | ~SUSPECT |
| hl-27 | `rm -f …/default` | **run** | a KILL (declines vouch, like `purge`); ⇒ wall | -GUESS |
| hl-28 | `nginx -t && systemctl reload nginx` | **run** | `reload` run-delta ⇒ wall (`nginx -t` is a read) | ~SUSPECT |
| hl-29,30 | `ufw allow …/tcp` | guard | firewall converged, past wall | +SURE |

**`plan: ~11 run, ~14 guard, 5 elided`** (of 30). **The elision is 5/30 ≈ 17%** — concentrated in the
ambient top; the book collapses to guards/runs after the first wall at `hl-6`. Note the elision
*count* is the **same 5** as windmill's ambient cluster, but the *percentage drops* (24%→17%) because
the three-service vendor tier adds ~10 sites, all below walls at base — so they **guard** (the guard
count nearly doubles, 7→14), they do not elide. This *sharpens* the "walls cap elision" lesson, and
it is exactly why §3's ceiling and §2C's three oracles exist. Confidence: the **5 elides are +SURE**;
the guard/run split is ~SUSPECT and rides two unknowns: **(u1)** whether the LLM stdlib carries
`is_converged` vouches for `install -d`/`ln`/`cp` (if not, hl-7/8/9/11/12/13/15/23/26 flip
guard→run); **(u2)** whether `rm`/`daemon-reload`/`reload` wall as predicted.

- Gained (steady): the whole ambient package cluster (five mutation-capable sites) provably not run;
  the classic hand-written `dpkg -s` guards lift exactly as `USER_STORY` promises.
- Not gained: **attention** — 25 of 30 sites still face the user. Past `hl-6` there is no proof to be
  had, and the vendor tier makes the past-the-wall region *larger* than windmill's.

**Drifted day** (representative: package index stale overnight — nothing else changed). `hl-1`
`apt-get update` now **runs** (diverged) ⇒ it becomes an *even earlier* wall, above the `dpkg`
cluster. `hl-2..5` degrade elide→**run-live** (their `dpkg -s` guards re-check past the `hl-1` wall,
per `USER_STORY` stage 5). **`plan: ~16 run, ~14 guard, 0 elided`.** One stale index costs the book
its entire shape — the `USER_STORY`-stage-5 lesson, and the exact pain the (unbuilt) footprint tier
exists to buy back. +SURE on the mechanism.

### Stage C — the two-minute oracle(s): three tractable walls, walked in sequence

The hint machinery points at the first wall (`USER_STORY` stage 3: an oracle for the first wall
un-walls downstream). But there are now **three** version-guarded downloads in a row, each the *same*
oracle-shape (r24 dialect, mirroring `strawman24-*/package.oracle.sh`'s `apt-get.is_converged`). The
minimal vouch, written once per service:

```sh
# minimal otel oracle (grafana/prometheus identical shape): vouch that the pinned binary being
# present-and-current is convergence, so dorc can lift my version-guard, elide the re-download, and
# stop walling the config/unit block below it.
otelcol-contrib.is_converged() {
   case "$1" in
   --version) otelcol-contrib --version 2>/dev/null | grep -q "$OTEL_VER" ;;   # 0 = current ⇒ converged
   *)         return 2 ;;                                                       # decline everything else ⇒ run
   esac
}
```

Because each vendor download is the **first wall of its own block**, oracling it elides its own line
**and** un-walls the config/unit sites between it and the next wall. The three oracles compound:

```
oracle written        elides its own line   un-walls              running elide count
── (Stage B)          —                     —                     5
+ otelcol             hl-6                  hl-7,8,9  (guard→elide)  9   (+4)
+ prometheus          hl-10                 hl-11,12,13              13  (+4)
+ grafana             hl-14                 hl-15                    15  (+2)
```

**Steady state, all three oracled (`plan: ~8 run, ~7 guard, 15 elided` of 30):**

```sh
otelcol-contrib --version… || {curl…}   # hl-6  elide: otel current (was RUN)
install -d /etc/otelcol-contrib          # hl-7  elide: un-walled (was guard)
cp otelcol-config.yaml …                 # hl-8  elide
cp otelcol-contrib.service …             # hl-9  elide
prometheus --version… || {…}             # hl-10 elide: prometheus current (was RUN)
install -d /etc/prometheus …             # hl-11 elide
cp prometheus.yml / .service …           # hl-12,13 elide
grafana --version… || {…}                # hl-14 elide: grafana current (was RUN)
cp grafana.service …                     # hl-15 elide: un-walled
systemctl daemon-reload                  # hl-16 run: run-delta ⇒ next wall (caps the un-wall)
```

The three two-minute oracles bought **+10 elisions** (5→15) — their own three lines plus the seven
downstream config/unit facts they had been poisoning. This is the `USER_STORY`-stage-3 "steepest part
of the value-curve" moment **scaled to a three-service homelab**: where windmill's single oracle
bought +3 (5→8 of 21), the OTel stack's three oracles buy +10 (5→15 of 30). Confidence: **the +3
own-lines are ~SUSPECT-toward-SURE**; **the +7 un-wall is ~SUSPECT** and rides the sharpest day-of
unknown — now **3× as load-bearing** because it gates all three:

> **⚠ key prediction to verify (u3, ~SUSPECT — now the pivotal day-of test):** each vendor guard is
> `<svc> --version | grep -q …` — a **stdout-consuming pipe guard**, not a bare rc-guard like
> `dpkg -s x`. Whether the built spike lifts a `cmd | grep -q X || fallback` (reproducing the
> version stdout as a probe-sourced value — `inv-probe-sourced-values`, the `consumed-output`
> fixture family) vs only rc-gated guards is genuinely uncertain from the design read. The three
> downloads use the **same** LHS shape, so u3 gates all three uniformly: **if it lifts, +10; if it
> does not, all three stay walls and Stage C == Stage B (5 elide, zero gain)** — and *that* is a
> first-class gap-log finding (the admin then rewrites the guards rc-form, or the oracles target the
> `curl` line directly; the friction itself is `target-admin-loop` signal). The RHS being a
> brace-group (`{ curl…; tar…; }`) rather than a bare `curl` does not change the LHS-lift question.

**Drifted day (stale MIDDLE binary — the multi-wall cascade windmill couldn't show):** all three
oracled, but the **prometheus** binary is present-yet-**old**. `prometheus.is_converged --version`
runs `prometheus --version`, sees the mismatch ⇒ **diverged** ⇒ `hl-10` `curl` **runs**
(re-downloads) ⇒ **re-walls everything below it**. Consequences: `hl-11,12,13` fall back to guard —
*and so do `hl-14,15`*: grafana is individually current, but it now sits **past the re-awoken
`hl-10` wall**, so its oracle can only **guard** it, never elide. **`plan: ~9 run, ~12 guard, 9
elided`** (down from 15). One stale binary in the *middle* of the vendor chain re-walls the *later*
vendor by position alone — the ordering/wall-cascade lesson a single-vendor book (windmill) could
not demonstrate. **The value a bare `command -v <svc>` guard could not give:** presence ≠ currency —
a `command -v` would have **wrongly elided the stale binary** (converged≠no-op); the version oracle
catches it. This is `target-adequacy` firing on a hand-authored oracle, now demonstrable on **any of
three** services — a co-primary trial win. +SURE on the mechanism.

## §3. The perfect-oracle ceiling and the un-oracleable residue (the denominator)

**If every command had a perfect oracle** (convergence vouch + a Stage-4/5 `touches()` footprint) and
`--trust-footprints` were on and built — the maximum the *design* can reach on this book, steady
state. **This is the denominator F5 demanded**: without it, a real-day 5–15 misreads as "tool weak"
instead of "hit the built ceiling."

**What the footprint tier buys** (`USER_STORY` stage 5; `notes/238`): a running wall that declares its
footprint no longer poisons downstream facts whose backing is **disjoint**. `apt-get update`'s
footprint (`pkgindex`) is disjoint from the dpkg-db, config files, certs, the three binaries → the
whole drifted-index collapse (Stage B drifted) is bought back. `daemon-reload`'s footprint (systemd
unit-cache) is disjoint from everything downstream → it runs but stops walling. An idealized `su`
footprint (the postgres DB) is disjoint from unit-state → the three `enable` sites survive it.
Entity-granular package footprints (`strawman24-survive-simple`) let a diverged binary re-download
not poison a converged sibling service.

**What NEVER elides — the residue map** (the honest horks; the plan must show these with reasons, per
`rul-attention-honesty`):

| site | why it is permanent residue | class |
|---|---|---|
| `hl-17,18` `su - postgres -c "…"` | the mutation lives **inside `su`'s `-c` string** — opaque to the analyzer; no general oracle can see the `psql`/`createdb` within. Elidable only by a bespoke per-invocation wrapper-oracle (impractical, naked). | opaque-wrapper |
| `hl-22` HA-internal state | `docker run` *can* vouch "container up at image X" (elides its own line at the ceiling) — but "container running" ≠ "HA configured/working": HA's real state is **inside the container**, un-probe-able from the host. A converged≠no-op that no host oracle closes. | un-probeable-substrate |
| `hl-28` `systemctl reload nginx` | run-delta with **no host-observable convergence** (nginx exposes no "loaded-config == on-disk" hash) — runs every apply, or takes a weak vouch that is itself an adequacy risk. | run-delta |
| `hl-22` docker's network footprint | even oracled, docker's footprint touches iptables/bridges → **intersects `ufw`'s backing** → keeps `hl-29,30` guarded (a real, correct non-elision even at the ceiling). | footprint-collision |
| `hl-25` heredoc vhost | the leaf's span covers `<<EOF`, not the body → **guards even at the ceiling** (a *render* limit no proof dissolves, `render21-heredoc-refusal`). | render-refusal |

**Ceiling tally (steady, perfect oracles + footprints):** `hl-1..15` elide (base cluster + all three
vendor blocks), `hl-19,20,21` elide (enables survive the disjoint daemon-reload/su footprints),
`hl-23,24,26,27` elide (certs dir, cert present w/ an idealized expiry-checking oracle, symlink, the
`rm` KILL), `hl-22` elides-own-line (adequacy-caveated), `hl-25` **guards even here** (heredoc-refusal),
`hl-29,30` guard (docker-iptables collision), `hl-16,17,18,28` run. **≈ 23 elide / ~3 guard / ~4 run
(of 30) — a ~77% ceiling.** -GUESS on the exact split (footprint collisions and the heredoc are the
fuzzy part); the *shape* is +SURE: a low-20s ceiling with a hard floor of ~4 opaque-wrapper /
run-delta / un-probeable sites (the *same* residue classes as windmill — the swap grew the elidable
top, not the floor).

**The denominators, side by side (of 30):**

| tier | elide | reachable on the day? |
|---|---|---|
| built spike, base stdlib (Stage B) | ~5 | **yes** |
| built spike, + 1 hand-oracle (otel only) | ~9 | **yes** (modulo u3) |
| built spike, + 3 hand-oracles (all vendors) | ~15 | **yes** (modulo u3) |
| perfect-oracle **ceiling** (Stages 4–5) | ~23 | **NO — footprint tier unbuilt** |
| permanent **floor** (never elides, any tier) | — (~4 sites always run) | — the honest residue |

So: **a real-day count near 5–15 is the built-tier ceiling, not tool-weakness.** The 15→23 gap is
precisely the unbuilt footprint tier + oracles for su/docker/cert/reload. The ~4-site floor is the
honest product statement (`USER_STORY` "the residue"): past the last wall, Dorc makes the book fast
and safe, not shorter — and the human LOCKED docker/HA + the `su` block + nginx-reload precisely to
exercise that floor.

## §4. Decisions-log (each ops-choice · alternatives · exercise-Dorc rationale)

- **dec-1 · HA via Container (docker), not Supervised/Core.** *[human-locked]* docker is the opaque
  poison-wall / the permanent residue — it exercises the honest-residue floor (§3) and is repeatable
  (pinned image), and it is the human's genuine dogfood want (`plans/252` §7). *Alt:* Supervised is
  more apt/systemd-oracle-able but HA unsupports it on generic Debian and it is a heavier,
  less-repeatable install. Container is the deliberate hork. **Unchanged by the OTel swap** — HA
  remains the sole opaque service-hork.
- **dec-2 · the three OTel services as native binaries + systemd, NOT their deb/apt or docker
  paths.** Keeps each service **out from behind the docker wall** so its version-guarded download is a
  *tractable* hand-oracle target (§2C) and its unit is service-oracle-able. Three downloads ⇒ **three
  tractable walls**, not one — the whole point of the swap. *Alt (the smoother paths):* docker-compose
  would put everything behind one opaque wall (windmill's problem); the **deb/apt** path would make
  each service apt-oracle-able (it would *elide like a base package* and stop being a hand-oracle
  wall). **FIRMED (255-firming, §8):** unlike windmill, all three native paths ARE documented — so the
  tarball form is a *deliberate exercise-Dorc choice*, not a realism compromise; the deb/apt path is
  the blessed fallback (and here it is a genuine first-class systemd path — see §8 — not compose-only
  as windmill was). Per `notes/256`, on-box-native is itself a deliberate divergence from the human's
  real (containerised) observability stack.
- **dec-3 · PostgreSQL as a system service (apt), NOT a container; grafana as its consumer.** (a)
  apt/systemctl-oracle-able ⇒ elision fires on install + service; (b) a **real drift-able DB** — the
  live `target-adequacy` substrate the trial is built to probe; (c) shared by grafana (via
  `GF_DATABASE_*`) ⇒ a rich cross-service edge. *Alt:* grafana defaults to an embedded sqlite (no
  cross-service edge, nothing to probe) or postgres-in-docker (opaque behind the docker wall).
  Grafana-on-external-postgres is a documented, supported config (§8). **Consumer swapped windmill →
  grafana; the substrate + its exercise-Dorc role are unchanged.**
- **dec-4 · postgres role/db via `su - postgres -c "…"`.** *Forced* by Debian peer-auth. **This is a
  genuine finding, not just a choice:** the idiomatic postgres-provisioning spelling is a poison-wall
  (`su` wraps an opaque `-c` payload the analyzer cannot see into) — honest residue *and* a realistic
  friction the day will surface. *Alt (pg_hba edits / running as a pg-authed user):* more setup, still
  opaque. Kept to exercise the opaque-wrapper residue class (§3). **Unchanged (role `grafana`).**
- **dec-5 · Self-signed certs via `openssl`, not Let's Encrypt.** *[human-locked, repeatability]* LE
  needs live ACME (network chaos breaks the differential, `plans/252` §7). openssl is unmodeled (a
  wall) but hand-guarded by `[ -f ]` — and that guard **never checks expiry**, making the cert the
  cleanest **converged≠no-op** demo (a present-but-expired cert wrongly "converges"). A strong
  secondary hand-oracle candidate (`openssl x509 -checkend`) if the day wants a second adequacy case.
- **dec-6 · Version-checked downloads (`<svc> --version | grep || {curl…}`), not bare `curl` or
  `command -v`.** Realistic (gh-runner uses a `[ -f config.sh ]` download guard; version-checking is
  the more-correct idiom) **and** it is the hand-oracle target: a `command -v` presence-guard is an
  un-shimmable builtin (non-mock-reproducible — the reason the headline fixtures switched to `dpkg -s`)
  *and* misses version (an adequacy gap). **Now ×3.** *New note:* the fallback is a brace-group
  (curl + tar-extract) because prometheus/grafana/otel ship tarballs, not a bare binary like windmill;
  this does not change the guard's LHS or the u3 test. **FLAG (u3):** whether Dorc lifts the
  stdout-consuming form is now the sharpest day-of unknown, gating all three (§2C).
- **dec-7 · Book order: packages → otel → prometheus → grafana → daemon-reload → postgres →
  bring-up → HA → proxy → firewall.** Positions the three vendor downloads as **consecutive tractable
  walls**, each above its own config/unit block, so each hand-oracle un-walls its block (the
  value-curve, walked thrice). *Alt (backends-first, postgres before the vendors):* puts the `su`
  wall first ⇒ collapses the whole book below it at base stage and kills the un-walling demo — noted
  as a **contrast variant the day could also run** (even more wall-dominated; a useful second
  data-point on how order shapes the plan). Placing all three unit-`cp`s before a *single*
  `daemon-reload` (hl-16) is both idiomatic and keeps the reload from splitting the vendor blocks.
- **dec-8 · `docker.io` (Debian-native), not docker-ce (official repo).** Single apt install ⇒
  apt-oracle-able (elides). *Alt:* the official-repo route adds `curl | gpg --dearmor` + a repo file —
  more opaque walls, orthogonal to what we're exercising. FLAG: `docker.io` is older (20.10); fine for
  `docker run`.
- **dec-9 · `set -eu` + scrappy hand-guards, flat sections (the lazy-admin shape).** This is the input
  Dorc infers best from — the hand-guards *are* the oracle material it lifts (`USER_STORY`: "years of
  defensive habit turn out to have been oracle material") — and what the human will actually write.
  `set -eu` exercises the errexit-honesty path (a guard's `||`-left is errexit-exempt by design).
- **dec-10 · A heredoc vhost write (`cat > … <<EOF`) + FIVE `cp`'d sidecar files.** The heredoc
  exercises the **heredoc-refusal edge** (`render21-heredoc-refusal`); the five `cp`s (otel config +
  otel unit + prometheus config + prometheus unit + grafana unit) exercise the clean file-content
  oracle at scale — five downstream facts that a vendor oracle un-walls. Both realistic; they hit
  different analyzer surfaces on purpose.
- **dec-11 · Three separate services, not one app (the swap's core).** Multi-service = more
  composition machinery (four services deep: grafana→postgres, grafana→prometheus, prometheus←otel,
  nginx→grafana+prometheus+HA) **and** three tractable walls instead of one. Where windmill exercised
  the value-curve once, the OTel stack exercises it three times and adds the **multi-wall cascade**
  case (§2C drifted): a stale middle-vendor re-walls the later vendor by position. *Alt (one bigger
  app, e.g. windmill):* fewer walls, no cascade, and — for windmill specifically — an admin-invented
  native path (§8).

## §5. Pre-registered predictions and caveats (the anti-woo instrument)

**Pre-registered (set BEFORE the day, per `plans/252` §7 F1 — vibe-words are post-hoc-gradeable):**

- **pred-1:** Stage B steady elides **exactly the ambient package cluster hl-1..5 (5 sites)**;
  everything from the first wall (hl-6, otel) down is guard/run. +SURE.
- **pred-2:** each vendor oracle moves the count by its own downstream block — **otel +4** (hl-6 own
  + hl-7,8,9), **prometheus +4** (hl-10 + hl-11,12,13), **grafana +2** (hl-14 + hl-15) — for **+10
  total (to 15)** with all three written — *iff* the stdout-consuming guard lifts (u3). If it lifts,
  ✓; if not, all three stay walls and Stage C == Stage B (5) and **that is the finding**, not a null
  result. ~SUSPECT.
- **pred-3:** the day's built-tier elision lands in **[5, 15] of 30 (~17–50%)** depending how many
  vendor oracles the admin writes (0→5, one→9, three→15); anything materially higher than 15 means my
  wall-map is wrong (a finding — likely the stdlib vouches more coreutils than I assumed, u1);
  materially lower than the oracle-count predicts means a vouch/lift I predicted didn't fire (a
  finding — check `dorc why` per `plans/252` B2 confound-isolation).
- **pred-4:** `su` (hl-17,18), `docker run` (hl-22), `systemctl reload` (hl-28), and
  `daemon-reload` (hl-16) **run at every built stage** and appear on every plan with a reason. If any
  *elides* on the built tier, my residue map is wrong (a surprising, valuable finding). +SURE they run.
- **pred-5:** the **ceiling is unreachable on the day** (footprint tier unbuilt); the drifted-index
  collapse (Stage B drifted → 0 elide) will reproduce and is **expected**, not a regression (`254` F4).
- **pred-6 (new, multi-wall):** with all three vendors oracled, a stale binary in the **middle**
  service (prometheus, hl-10) drops elision **15→~9** and forces the **later** service (grafana,
  hl-14,15) elide→guard *purely by position* (past the re-awoken wall). If grafana still elides with
  prometheus diverged, my wall-cascade model is wrong (a finding). ~SUSPECT.

**Caveats — where reality may diverge from this read:**
- **c-1** These dispositions are my reading of `USER_STORY` + the fixtures, not the tool. The whole
  point is prediction-vs-observation; expect misses, bank them as findings (`plans/250`).
- **c-2** The book is **untested pre-VPS**. The `# FLAG:` lines are the realism-risks; the OTel
  services' asset names / versions / ports / config-paths are now FIRMED (`255-firming`, §8). Still-open
  realism-risks: grafana's exact version-subcommand + tarball top-dir name, prometheus/grafana behind
  an nginx sub-path, the HA tag, the `docker run` non-idempotence, the `su` quoting, and
  `docker.io`-vs-docker-ce.
- **c-3** The base-stdlib coverage of coreutils (install-d/ln/cp) and the exact verbs the LLM stdlib
  vouches are assumptions (u1); they swing the guard/run split (larger here than in windmill because
  the vendor tier adds many cp/install-d sites) but **not** the elide count (pred-1 holds regardless).
- **c-4** u3 (stdout-consuming guard lift) is the single most load-bearing uncertainty for the Stage-C
  value story — **now 3× as consequential** (it gates all three vendor oracles at once). Put it first
  on the day's `dorc why` checklist.
- **c-5** "attention-lines saved" (the real value-prop, `DESIGN` priority 3) is moved *more* on this
  book than on windmill at Stage C (25→15 of 30 with all three oracled, vs windmill's 16→13 of 21) —
  because there are three un-wallable blocks, not one. But past the last vendor wall (hl-16 down) the
  residue is real and early; read the day through §3.

## §6. Summary ledger (`USER_STORY`-style)

```
stage                                elide  guard  run   attention   reachable on the day?
A  bare (stdlib disabled)              0      0     30     30         illustrative only
B  base stdlib — steady               5     14     11     25         YES
B  base stdlib — drifted (stale idx)  0     14     16     30         YES (expected collapse)
C  + otel oracle only — steady        9     11     10     21         YES (modulo u3)
C  + all 3 vendor oracles — steady   15      7      8     15         YES (modulo u3)
C  + all 3 — drifted (stale middle)   9     12      9     21         YES (adequacy + cascade)
—  perfect-oracle CEILING           ~23     ~3     ~4     ~7         NO (footprint tier unbuilt)
—  permanent FLOOR (never elides)   —      —      ~4     —          the honest residue
```

The shape to carry to the day: **the built tier tops out near Stage C-all-three (~15/30); the ceiling
(~23/30) needs Stages 4–5; and ~4 sites (`su`×2, `docker`/HA-internal, `systemctl reload`, with
`daemon-reload` also always-running) are the honest residue that never elides at any tier.** A number
in the 5–15 band on the day is the design working as predicted at the built frontier — not the tool
failing. The swap's headline vs windmill: **the same 5-site ambient floor, but a value-curve walked
three times (+10 not +3) and a new multi-wall-cascade case** — more of Dorc exercised for the human's
first real trial.

## §7. Appendix — the sidecar configs the book `cp`s

The book inlines the nginx vhost as a heredoc (hl-25) but `cp`s five external files. Their content,
sitting beside the book on the day (all FIRMED against first-party docs per `255-firming`, §8):

**`otelcol-config.yaml`** (hl-8) — OTLP in, remote-write to prometheus out, health_check extension:
```yaml
# FIRMED: OTLP receiver ports 4317 (gRPC) / 4318 (HTTP), health_check :13133 — opentelemetry.io.
receivers:
  otlp:
    protocols:
      grpc: { endpoint: 0.0.0.0:4317 }
      http: { endpoint: 0.0.0.0:4318 }
exporters:
  prometheusremotewrite:
    endpoint: http://127.0.0.1:9090/api/v1/write   # → prometheus (hl-10's service)
extensions:
  health_check: { endpoint: 0.0.0.0:13133 }
service:
  extensions: [health_check]
  pipelines:
    metrics: { receivers: [otlp], exporters: [prometheusremotewrite] }
```

**`otelcol-contrib.service`** (hl-9):
```ini
# FIRMED: config path /etc/otelcol-contrib/config.yaml + `--config` flag — opentelemetry.io.
# The .deb would ship this unit first-class; we hand-write it for the tarball form (dec-2).
[Unit]
Description=OpenTelemetry Collector (contrib)
After=network-online.target
Wants=network-online.target
[Service]
ExecStart=/usr/local/bin/otelcol-contrib --config=/etc/otelcol-contrib/config.yaml
Restart=on-failure
[Install]
WantedBy=multi-user.target
```

**`prometheus.yml`** (hl-12) — scrape self + the collector's own telemetry, accept remote-write:
```yaml
global: { scrape_interval: 15s }
scrape_configs:
  - job_name: prometheus
    static_configs: [ { targets: ['127.0.0.1:9090'] } ]
  - job_name: otel-collector
    static_configs: [ { targets: ['127.0.0.1:8888'] } ]   # collector self-metrics (FIRMED :8888)
```

**`prometheus.service`** (hl-13):
```ini
# FIRMED: web port :9090, `--config.file`, `--storage.tsdb.path` — prometheus.io. No upstream unit
# ships (admin-invented). --web.enable-remote-write-receiver accepts the collector's push.
[Unit]
Description=Prometheus
After=network-online.target
Wants=network-online.target
[Service]
ExecStart=/usr/local/bin/prometheus \
   --config.file=/etc/prometheus/prometheus.yml \
   --storage.tsdb.path=/var/lib/prometheus \
   --web.enable-remote-write-receiver \
   --web.external-url=https://homelab.lan/prometheus/   # FLAG: behind-nginx-subpath routing (known friction)
Restart=on-failure
[Install]
WantedBy=multi-user.target
```

**`grafana.service`** (hl-15) — on postgres, behind the /grafana/ sub-path:
```ini
# FIRMED: `grafana server` subcommand + --config + --homepath (grafana.com standalone-binary docs);
# GF_<SECTION>_<KEY> env overrides + http_port default 3000 + serve_from_sub_path (configure-grafana).
# FLAG: exact version subcommand; running as root (docs prefer a dedicated `grafana` user).
[Unit]
Description=Grafana
After=network-online.target postgresql.service
Wants=network-online.target postgresql.service
[Service]
Environment=GF_DATABASE_TYPE=postgres
Environment=GF_DATABASE_HOST=127.0.0.1:5432
Environment=GF_DATABASE_NAME=grafana
Environment=GF_DATABASE_USER=grafana
Environment=GF_DATABASE_PASSWORD=changeme
Environment=GF_SERVER_ROOT_URL=https://homelab.lan/grafana/
Environment=GF_SERVER_SERVE_FROM_SUB_PATH=true
ExecStart=/usr/local/grafana/bin/grafana server \
   --config=/usr/local/grafana/conf/grafana.ini --homepath=/usr/local/grafana
Restart=on-failure
[Install]
WantedBy=multi-user.target
```

Port/edge consistency check: grafana serves :3000 (proxied at `/grafana/`); prometheus serves :9090
(proxied at `/prometheus/`, and receives the collector's remote-write); the collector receives OTLP
on 4317/4318 (internal — NOT proxied); HA (host-net) binds :8123 (proxied at `/`). Grafana's
`GF_DATABASE_*` points at the role/db the `su`/psql block (hl-17,18) creates, and grafana queries
prometheus as a datasource. These are the cross-service edges §1 relies on.

## §8. Owed — reconciliation status

**All three OTel services — FIRMED (2026-07-04, `255-firming` pass; first-party docs + GitHub release
API):**

- **otel-collector** — asset `otelcol-contrib_0.155.0_linux_amd64.tar.gz` real (contrib distro, GitHub
  release `v0.155.0`); config `/etc/otelcol-contrib/config.yaml` + `--config` flag; OTLP ports
  4317/4318, health_check 13133, self-metrics 8888. **native+systemd = FIRST-CLASS:** the `.deb` ships
  `otelcol-contrib.service` + the config first-class; the tarball is also offered (we use it by choice).
- **prometheus** — asset `prometheus-3.13.0.linux-amd64.tar.gz` real (GitHub release `v3.13.0`), holds
  `prometheus`+`promtool`; web port :9090; `--config.file` + `--storage.tsdb.path` conventions
  (`/etc/prometheus/prometheus.yml`, `/var/lib/prometheus`). **native = FIRST-PARTY-DOCUMENTED**
  (pre-compiled binaries are the *headline* install method) **but NO upstream systemd unit** — the
  unit is admin-invented (like windmill's, but the *binary* path here is blessed, unlike windmill's).
- **grafana** — OSS standalone tarball `grafana-13.1.0.linux-amd64.tar.gz` from **dl.grafana.com** (the
  GitHub release ships only build-numbered `.deb`s); binary `grafana server` under a homepath
  (`/usr/local/grafana`); config `/etc/grafana/grafana.ini` (deb) or homepath `conf/grafana.ini`;
  `GF_<SECTION>_<KEY>` env overrides (so `GF_DATABASE_TYPE=postgres`, etc.); http_port default 3000;
  sub-path needs `root_url` + `serve_from_sub_path=true`. Grafana-on-external-postgres is a documented,
  supported config. **native+systemd = FIRST-CLASS:** apt.grafana.com / the deb ship
  `grafana-server.service` + `/etc/grafana`; the standalone-binary + hand-written unit is *also* fully
  documented (grafana.com gives the exact unit file we mirror).

**The recorded finding (native+systemd, first-class vs admin-invented — the whole realism case for the
swap):** windmill documented its native path **nowhere** (README/self_host = compose/helm/cloud only;
first-party Q&A "run from binary… not recommended"), so it was admin-invented. **The OTel stack
upgrades the realism across the board:** all three have documented native install paths; two
(otel-collector, grafana) even ship **first-class systemd units** via their packages; prometheus's
pre-compiled-binary path is the *headline* first-party install. Using the tarball + version-guard form
for all three is therefore a **deliberate exercise-Dorc choice** (to keep them tractable hand-oracle
walls, dec-2), *not* a realism compromise — and the deb/apt path is a genuine first-class fallback (not
compose-only, as windmill's was). Per `notes/256`, on-box-native remains a deliberate divergence from
the human's real (containerised, managed-postgres) observability practice.

**RAM / VPS-size note (per the human's "note if it needs the ~2 GB tier"):** grafana's own documented
**Small** tier is 2 cores / **2–4 GB** (grafana process only, excludes data sources; min-rec 512 MB).
Stack it with prometheus (TSDB, ~0.5–1 GB even at small scrape volume), the otel-collector (~100–200 MB),
system postgres (~200–400 MB), the **HA container** (~0.5–1 GB), and nginx (negligible): the box wants
**≥2 GB as a realistic floor, 4 GB comfortable** — HA + grafana + prometheus together are what push it.
The "lean OTel stack" is leaner than a full observability platform, but it is **not a 512 MB box**; the
~2 GB tier is the minimum and 4 GB is the recommended target for the trial VPS. ~SUSPECT on the exact
figures (workload-dependent); +SURE on the "≥2 GB, not 512 MB" shape.

**Still owed (day-one runnability only — NONE move the §2/§3 predictions, which turn on the *shape*:
base-oracle-able cluster, three tractable vendor walls, opaque-wrapper residue):** grafana's exact
version-subcommand (`grafana --version` vs `grafana server --version`) + tarball top-dir name; the
prometheus/grafana behind-nginx-subpath routing (a known reverse-proxy friction); the HA image tag;
`docker.io`-vs-docker-ce (dec-8); and the precise otel→prometheus wiring (remote-write receiver vs a
scrape job — the appendix picks remote-write).
