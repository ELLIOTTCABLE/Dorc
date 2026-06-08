# 17x — round-17 strawmen: extract real-script guards, correlate by kind (2026-06-07)

Companion to `notes/171`–`174` + the `plans/170` charter (the K1 kind-identity round). Follows the `15x-strawmen/` convention
(`books/` + `oracles/` + `.straw.sh`). Where 15x were *hand-written* strawmen, **these `books/` are real,
third-party, commit-pinned downloads**; the `oracles/` are my extractions from them. Goal: stop theorising
and test, on real provisioning code, whether the **getent-pattern executable guard** (round 174 — a blessed
kind named/probed by a command: `getent passwd`, `getent group`, `systemctl is-active`, `command -v`) can
(a) be *extracted* from how people actually write ops scripts, and (b) *correlate independently-authored
scripts by kind* to enable an elision that token-co-reference cannot.

## Method — and a deliberate constraint
The mutative bodies were **not read** (human directive: don't let the implementation bias the extraction).
Each oracle was built purely from `grep` over the book for (i) guard/probe idioms and (ii) the kind-naming
head-lines (`useradd`/`groupadd`/`usermod`/`systemctl enable`/`ufw allow`) — never the logic between them.
The `books/*.book.sh` are byte-for-byte as downloaded and **unmodified**; the extraction lives only in
`oracles/`.

## Provenance (commit-pinned; verify with `sha256sum books/*`)
| book | repo @ commit (file) | role / distro | lines | sha256[:16] | license |
|---|---|---|---|---|---|
| `plik-postinstall.book.sh` | root-gg/plik @4fbe4fe (`releaser/scripts/server-postinstall.sh`) | self-hosted app postinstall / Debian | 35 | `74c8fee640dcc578` | MIT |
| `consul-provision.book.sh` | scottslowe/learning-tools @7fc9e09 (`consul/consul/consul.sh`) | Consul service / Debian | 70 | `6373e7e8bde12fd3` | MIT |
| `enginescript-redis.book.sh` | EngineScript/EngineScript @3efaf87 (`scripts/install/redis/redis-install.sh`) | redis cache / Ubuntu (WP stack) | 94 | `286ae6b54701848d` | GPL-3.0 |
| `gh-runner.book.sh` | fedorenkoivan/devops @b27bfac (`lab3/scripts/setup-runner.sh`) | GH Actions runner / Ubuntu | 115 | `c67d243b2ba5779f` | MIT |
| `onservice-harden.book.sh` | onServiceTeam/onservice-onsite-app @5d1f2c6 (`scripts/server/01-harden.sh`) | server hardening / Ubuntu | 117 | `55806282b3a2fb31` | none in-file |
| `termidar-setup.book.sh` | N-Erickson/termidar @040f0d6 (`server/setup.sh`) | SSH app-server / RHEL/Oracle | 251 | `91462b2241a941b6` | none in-file |

Honest-sampling note (priority 1): subagents were told **not** to cherry-pick guard-friendly scripts —
the set spans clean (consul, plik) to genuinely messy (enginescript sentinel-idempotency; onservice
unconditional hardening) and 3 distro families. No real secrets present (scanner flagged only
`VAR="${VAR:-default}"` param-defaults — false positives).

## Kind-surface coverage (the test result)
`G` = author-guarded · `U` = mutated **unguarded** · `S` = supplied probe (Dorc must add) · `–` = n/a.

| kind | plik | consul | redis | runner | onservice | termidar |
|---|---|---|---|---|---|---|
| **user** | G | G | S `www-data` | G | G `deploy` | G |
| **group** | G | – | G `redis` | S `docker` | – | – |
| **membership** | – | – | U→S `www-data@redis` | U→S `runner@docker` | U→S `deploy@sudo` | – |
| **service** | S | G | U→S | U→S `docker` | U→S `f2b`,`unatt` | G |
| **tool** | – | – | – | G `docker` | – | G `go` |
| **file/dir** | G* | G | – | G | G `swapfile` | U `sshd_config` |
| **port** | – | – | – | – | U→S(fragile ufw) | U→S(clean firewalld) |
| **package** | U | U | U | U | U | U |
| **sentinel** | – | – | U(whole-script) | – | – | – |

`*` plik's config guard is **inverse-polarity** (`grep -q DEFAULT_PATH` = *not yet* configured).

### What this says (viability)
- **The getent-pattern core (`user`/`group`/`service`/`tool`/`file`) is real and uniform.** Every book's
  idempotent spine is exactly these kinds, probed by exactly the round-174 commands. A *careful* author
  (`consul`) writes the entire oracle for free; the kinds are identical across all six, so they are
  cross-script-correlatable with one blessed probe each.
- **But a real corpus leaves a long tail un-liftable** (priority-1 honesty): config **content**, whole-script
  **sentinels**, **packages**, and **group-membership** + **ufw ports** mutated unguarded. Those need
  *oracle-supplied* probes (S) or fall to the ⊤-run floor. A hardening book (`onservice`) is the worst case:
  ~2 of ~9 state-changes are author-guarded.
- **Port-probe-ability is provider-dependent** — `firewall-cmd --query-port` is a clean getent-pattern probe
  (termidar); `ufw status | grep` is fragile and re-introduces the 15x `.`-as-regex wrong-skip. Same kind,
  two providers, two answers.

## The solid working example (priority 2) → `oracles/_correlate.straw.sh`
`enginescript-redis.book.sh:77` runs `usermod -aG redis www-data` **unguarded** — an undeclared dependency
on `user:www-data`, which a *different* script (the web stack) creates. There is **no shared token** linking
the two, so co-reference (094 g1) is blind. Because `user:www-data` and `group:redis` are **blessed,
uniformly-probe-able kinds**, Dorc can `getent passwd www-data` to discharge the precondition across the
script boundary, and `id -nG www-data | grep -qw redis` to **elide the `usermod` itself** — an elision
reachable *only* by kind-correlation, on real downloaded code. The same shape recurs for `group:docker`
(gh-runner consumes a docker group a separate docker-host script establishes).

## Caveats
- `oracles/*` are strawmen for discussion, not a contract; several probes are **SUPPLIED** (marked `(B)`),
  i.e. *not* present in the book — they show what Dorc would need, and so double as the coverage-gap tally.
- Commits #2/#3 were pinned to the serving SHA (re-pin from `git log -- <path>` for archival certainty).
- Per AGENTS, the books are inline specimens; do not "fix" them — they are frozen evidence.

## `adversarial/` — round-17 crosscheck strawmen (a different category)
Added during the `plans/17N` adversarial crosscheck (`notes/17O`). Unlike the `books/`+`oracles/` above (real
extractions), these are **illustrative regression-test seeds** authored to ground a finding:
- `r17-crosscheck-runnable.straw.sh` — verified shell-hazard demos (runs clean under `dash`): `command -v`
  function-shadowing, `|| true` rc-masking, mutating-`trap`-fires-during-probe, the off-ramp annotation.
- `r17-crosscheck-dorc-inputs.straw.sh` — Dorc elision-decision inputs (valid sh + the correct-vs-wrong Dorc
  verdict): R2-CHANGEDELTA (run-delta), R2-PROBEGATE, R2-CONTEXT, F-ALGEBRA (≥enum structured key), the
  oracle-quality class.
- `compiled-probe.straw.sh` — what a *lifted probe* looks like (oracle interceptors + OOB verdict lane +
  CFG-preserved variant); the sanitized human sketch. Inlined into `plans/17N` §3.
