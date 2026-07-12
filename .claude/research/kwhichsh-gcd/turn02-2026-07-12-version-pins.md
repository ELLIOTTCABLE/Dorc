# kWHICHSH GCD — floor version pins (posh/dash battery empirics)

> Turn 02, 2026-07-12. Empirical pinning of the two-binary dialect floor.
> Floor spec being pinned: *"a valid dorc-lang v0.1 base-dialect text parses and
> runs identically under `posh <vP>` and `dash <vD>`; disagreement = outside the
> dialect."* This turn picks `(vP, vD)` and verifies the pick against a 12-item
> battery with real, built/extracted binaries.
> Certainty markers per global-prompt rule: +SURE / ~SUSPECT / -GUESS.
> Empirics tier: **real-binary** (WSL2 Ubuntu 24.04.4; dash built from official
> source tarballs, posh extracted from official Debian `.deb`s — no installs).

---

## TL;DR — recommended pair

| Pole | Pick | One-line rationale |
|---|---|---|
| **vP (posh)** | **0.14.1** | The version in *current Debian stable* (Trixie). Battle-tested across bullseye→bookworm→trixie (3 releases); battery-identical to 0.14.5. +SURE it is current-stable; +SURE it is the principle-faithful pick. |
| **vD (dash)** | **0.5.12** | The **newest** official release that still *lacks* `set -o pipefail` (pipefail is introduced in **0.5.13**, not 0.5.12 as believed). Battery-identical to 0.5.11.5. ~SUSPECT on the exact notch (see `dash-pin-tension`). |

Recommended pair: **`(posh 0.14.1, dash 0.5.12)`**. Both battery-verified against
the floor; every one of the 12 checks is concordant across all four candidate
binaries (two posh × two dash) — **zero deviations from the expected column**.

**Two headline corrections to the errand's stated expectations** (details below):
- `pipefail-in-0.5.13`: `set -o pipefail` enters dash at **0.5.13**, *not* 0.5.12.
  The Debian/Ubuntu-patched system `dash 0.5.12-6ubuntu5` *also* rejects it — there
  is **no** "Debian pipefail backport" in the 0.5.12 line (this falsifies the
  turn01 note at its care-set table, and is the likely seed of the "added in
  0.5.12" belief). ⇒ newest-lacking-pipefail = **0.5.12**, not 0.5.11.5.
- `posh-stable-is-0.14.1`: current Debian **stable** (Trixie) ships posh **0.14.1**;
  0.14.5 is **testing/unstable** (forky/sid), uploaded 2026-04, long after Trixie's
  2025-08 release. The pool carrying both is not evidence 0.14.5 is stable.

---

## 1. Method / environment (hermeticity note)

- Host: Windows 11; Linux ELF via WSL2 `Ubuntu 24.04.4 LTS`. `wsl -e sh -c 'echo ok'` → ok.
- Toolchain already present (nothing installed): gcc, make, dpkg-deb, ar, tar, curl.
- All artifacts under WSL-side `/tmp/dorc-pins/` (never the SyncThing-synced repo tree).
- **dash**: built from official source tarballs — `./configure && make`, ran `src/dash`.
- **posh**: extracted amd64 `.deb` via `dpkg-deb -x` (no install), ran `usr/bin/posh`.
- Every candidate executed **only** as `<binary> -c '<exact-check-string>'` plus the
  permitted version probes. A POSIX driver (run under WSL's own `/bin/sh`, *not* under
  the candidates) delivered each exact check string from per-check files; no script was
  ever fed *to* a candidate binary.
- `git.kernel.org` was **not** fetched: its robots.txt `Disallow: /`s Claude/AI agents.
  Respected. pipefail-introduction evidence instead comes from the official release
  **tarball source + built binaries** (stronger than a web changelog anyway).

---

## 2. posh pin — why 0.14.1 (`posh-stable-is-0.14.1`)

Authoritative suite→version map from packages.debian.org (`exact=1`, all suites):

| Suite | posh version |
|---|---|
| bullseye (oldoldstable) | 0.14.1 |
| bookworm (oldstable) | 0.14.1 |
| **trixie (stable)** | **0.14.1** (`+b1` binNMU on arm64/riscv64) |
| forky (testing) | 0.14.5 |
| sid (unstable) | 0.14.5 |

Changelog dates (from each `.deb`'s `changelog.gz`): `0.14.1` = 2020-05-18;
`0.14.2` = 2025-08-23 (**two weeks after** Trixie's 2025-08-09 release); `0.14.3`
= 2025-12-04; `0.14.4`/`0.14.5` = 2026-04-18/19, all targeting `unstable`. So the
0.14.2→0.14.5 line never entered a stable release. +SURE.

Decision principle applied: *"posh pin = the version in current Debian stable …
current-stable is the battle-tested artifact."* Current stable = Trixie = **0.14.1**.
It has also been the `/bin/sh`-policy shell across three consecutive Debian releases,
which is as battle-tested as posh gets. The errand's loose "expected 0.14.x" holds;
the only correction is *which* 0.14.x (0.14.1, not the pool's newest 0.14.5). +SURE.

Fallback to 0.13.2 was gated on "0.14.x shows anomalies in the battery" — it showed
**none** (§4). No fallback. For completeness the battery was also run against 0.14.5;
it is **byte-identical in behavior** to 0.14.1 on all 12 checks, so the choice between
them is purely the current-stable principle, not a behavioral one. +SURE.

`POSH_VERSION` probe (permitted non-`-c` execution): `posh-0.14.1` → `0.14.1`;
`posh-0.14.5` → `0.14.5`.

---

## 3. dash pin — why 0.5.12, and the tension (`pipefail-in-0.5.13`, `dash-pin-tension`)

### The pipefail-introduction fact (verified three ways)

The errand's stated belief — "pipefail is believed added in 0.5.12" — is **false**.
pipefail enters dash at **0.5.13**. Evidence, all from official tarballs on Herbert
Xu's release host:

1. **Source `options.h` diff (0.5.12 → 0.5.13)** — one new option slot appears:
   ```
   63c63,64
   < #define	debug optlist[16]          # 0.5.12: no pipefail; NOPTS 17
   ---
   > #define	pipefail optlist[16]       # 0.5.13: pipefail is NEW slot [16]
   > #define	debug optlist[17]          #         debug pushed to [17]
   65c66
   < #define NOPTS	17
   > #define NOPTS	18                    # option count grew 17 → 18
   ```
2. **Whole-tree grep**: `pipefail` absent from `dash-0.5.11.5/` and `dash-0.5.12/`
   entirely; present in `dash-0.5.13/src/{options.c,options.h,jobs.c,dash.1}`.
   The 0.5.13 manpage documents it fully (`.It Em pipefail`, lines 1854-1858).
3. **Built-binary behavior**: `set -o pipefail && echo YES` →
   - `dash 0.5.11.5` (built): rc 2, `set: Illegal option -o pipefail`
   - `dash 0.5.12`  (built): rc 2, `set: Illegal option -o pipefail`
   - `dash 0.5.12-6ubuntu5` (system, Debian/Ubuntu-patched): rc 2, `Illegal option` — **no backport**
   - `dash 0.5.13`  (built): rc 0, prints `YES`

+SURE that pipefail-introduction is 0.5.13. +SURE there is no 0.5.12.x point release
(official listing goes `…0.5.11.5, 0.5.12, 0.5.13, 0.5.13.1…`), so the newest release
lacking pipefail is unambiguously **0.5.12**.

### `dash-pin-tension` — 0.5.12 vs 0.5.11.5 (a ratification point)

The errand gives a **primary rule** and a **soft preference** that now pull apart:
- Primary rule: *"NEWEST official release that still LACKS pipefail."* Under the
  corrected fact ⇒ **0.5.12**.
- Soft preference in the rationale: *"older is gently better for a floor"* ⇒ 0.5.11.5.

I read "older is gently better" as *reinforcing* that a pre-pipefail floor is wanted
(both candidates satisfy that), not as an independent instruction to pick older than
the newest-lacking. So the principle-faithful pick is **0.5.12**, and I recommend it.
But because `dash 0.5.11.5` and `dash 0.5.12` are **byte-identical in behavior on all
12 checks** (§4), there is *zero dialect risk* in either choice — the conductor can
ratify down to 0.5.11.5 on the "gently older" preference at no cost. ~SUSPECT on which
notch the conductor ultimately wants; +SURE the two are behaviorally interchangeable
for the floor.

The pair-agreement goal ("both reject bare `set -o pipefail`, cleaner than leaning on
the disagreement clause") is met by **either** dash candidate paired with **either**
posh: all four reject bare pipefail (§4 check 02). +SURE.

dash version provenance (no `--version` flag exists): `0.5.11.5` and `0.5.12` from the
tarball names / `PACKAGE_VERSION` in `configure`; system reference `dpkg -s dash` →
`Version: 0.5.12-6ubuntu5` (read-only query).

---

## 4. The battery matrix (4 binaries × 12 checks)

`rc` / `OUT=[stdout]` / `ERR=[stderr]`, verbatim. Version probe = check 12 (§2/§3).

| # | slug | dash-0.5.11.5 | dash-0.5.12 | posh-0.14.1 | posh-0.14.5 | expected | verdict |
|---|---|---|---|---|---|---|---|
| 1 | local-assign | rc0 `deltaX` | rc0 `deltaX` | rc0 `deltaX` | rc0 `deltaX` | `deltaX` rc0 both | ✅ PASS (keystone holds) |
| 2 | pipefail-bare | rc2 `Illegal option -o pipefail` | rc2 (same) | rc1 `set: pipefail: bad option` | rc1 (same) | FAIL/no-YES both | ✅ both reject |
| 3 | pipefail-gate | rc0 `OK` | rc0 `OK` | rc0 `OK` | rc0 `OK` | `OK` rc0 both | ✅ PASS |
| 4 | pe-subst | rc2 `Bad substitution` | rc2 (same) | rc1 `${x/b/z}": bad substitution` | rc1 (same) | reject both | ✅ both reject |
| 5 | dbracket | rc127 `[[: not found` | rc127 (same) | rc127 `[[: not found` | rc127 (same) | reject both | ✅ both reject |
| 6 | dollarsq | rc0 `$a\tb` | rc0 `$a\tb` | rc0 `$a\tb` | rc0 `$a\tb` | literal `$`-form, no tab, both | ✅ PASS |
| 7 | test-ao | rc0 `Y` | rc0 `Y` | rc0 `Y` | rc0 `Y` | `Y` both (lint-tier) | ✅ both over-accept |
| 8 | func-kw | rc2 `Syntax error "}" unexpected` | rc2 (same) | rc1 `syntax error: \`}' unexpected` | rc1 (same) | reject both | ✅ both reject |
| 9 | pe-classics | rc0 `abca\|bc\|6\|def` | rc0 (same) | rc0 (same) | rc0 (same) | `abca\|bc\|6\|def` both | ✅ PASS |
| 10 | errexit-andor | rc0 `OK` | rc0 `OK` | rc0 `OK` | rc0 `OK` | `OK` rc0 both | ✅ PASS |
| 11 | echo-n | rc0 `Y` | rc0 `Y` | rc0 `Y` | rc0 `Y` | report (−n suppressed) | ✅ all suppress `-n` |
| 12 | version | tarball/`PACKAGE_VERSION` `0.5.11.5` | dpkg-ref `0.5.12` | `$POSH_VERSION`→`0.14.1` | →`0.14.5` | — | recorded |

### Per-check reading (vs the errand's expected column)

- **01 local-assign** (the keystone): posh's `local a b c=delta` — mixed bare-decl +
  assign — is **accepted** and yields `deltaX`. The headline-risk (posh rejecting the
  assign form) did **not** materialize. +SURE the keystone is safe under both posh.
- **02 pipefail-bare**: both dialects reject; the pair *agrees* on rejection, so
  pipefail stays cleanly out-of-dialect without invoking the disagreement clause.
  Note rc differs (dash 2 / posh 1) — see `rc-on-rejection` observation.
- **04 pe-subst**: posh does **not** accept bash pattern-substitution `${x/b/z}` — no
  headline finding; posh stays a strong floor enforcer. +SURE.
- **05 dbracket**: posh (pdksh-derived) does **not** retain `[[ ]]` — treated as
  command-not-found (rc127), identically to dash. No headline finding; posh's
  floor-enforcer role is intact. +SURE.
- **06 dollarsq**: confirms the `$'…'`-OUT ruling — `$'a\tb'` yields literal `$a\tb`
  (no C-escape/tab expansion) on all four. +SURE.
- **07 test-ao**: Debian-mandated `-a`/`-o` over-acceptance present in both — recorded
  lint-tier, not a floor construct. +SURE.
- **08 func-kw**: `function f { … }` ksh-style rejected by both. +SURE.
- **09 pe-classics**: classic POSIX expansions (`%`, `##`, `${#x}`, `:-`) identical
  across all four. +SURE.
- **10 errexit-andor**: the `set -e; false && true` non-last-member exemption holds. +SURE.
- **11 echo-n**: all four suppress the `-n` flag (XSI/Debian echo), printing bare `Y`. +SURE.

---

## 5. Deviations & observations

`dev1` (**headline, changes vD**) — pipefail enters dash at **0.5.13**, not 0.5.12;
newest-lacking is 0.5.12, not the expected 0.5.11.5. §3. +SURE.

`dev2` (**headline, pins vP**) — current Debian **stable** posh is **0.14.1** (Trixie),
not the pool's newest 0.14.5 (which is testing/unstable). §2. +SURE.

`dev3` (**falsifies a prior note**) — the turn01 care-set table's "0.5.12 (Debian
pipefail backport)" is empirically wrong: the patched system `dash 0.5.12-6ubuntu5`
rejects `set -o pipefail`. No pipefail exists anywhere in the 0.5.12 line. +SURE for
Ubuntu 24.04's build; ~SUSPECT that *no* Debian 0.5.12-`N` revision anywhere ever
carried a backport (I only tested the one system package + upstream source).

`rc-on-rejection` (observation, **not** a deviation) — for *out-of-dialect* constructs
(checks 02/04/08) dash exits **2** and posh exits **1**, with different error text; for
`[[` (05) both exit **127**. The floor's "runs identically" is fully satisfied for
*accepted* (in-dialect) constructs (checks 01/03/06/07/09/10/11 — identical rc *and*
stdout). For *rejected* constructs the two binaries disagree on exit-code/message,
which is immaterial to dialect membership (rejection is rejection; the "disagreement =
outside the dialect" clause absorbs it). Worth a note only if any future dialect rule
tries to depend on the *exit code* of a rejected construct — it must not. ~SUSPECT this
is a non-issue; flag for the conductor.

**Battery deviations from the expected column: NONE.** All 12 checks landed exactly as
the errand's expected column predicted, for all four candidate binaries.

---

## 6. Provenance — everything fetched (official hosts only)

| Artifact | URL | sha256 |
|---|---|---|
| dash-0.5.11.5.tar.gz | http://gondor.apana.org.au/~herbert/dash/files/dash-0.5.11.5.tar.gz | `db778110891f7937985f29bf23410fe1c5d669502760f584e54e0e7b29e123bd` |
| dash-0.5.12.tar.gz | http://gondor.apana.org.au/~herbert/dash/files/dash-0.5.12.tar.gz | (extracted; NOPTS 17, no pipefail) |
| dash-0.5.13(.1/.2).tar.gz | http://gondor.apana.org.au/~herbert/dash/files/ | (corroboration: pipefail present) |
| posh_0.14.1_amd64.deb | http://deb.debian.org/debian/pool/main/p/posh/posh_0.14.1_amd64.deb | `1b5d164d880aa57c6808fec6185b0d07e0c15bee7c0c518bdd5252d17a24ef90` |
| posh_0.14.5_amd64.deb | http://deb.debian.org/debian/pool/main/p/posh/posh_0.14.5_amd64.deb | `175c2eb3bbedbf63d9a9186c5df666a38560d290a2c67961c225fe148d700d54` |
| suite→version map | https://packages.debian.org/search?keywords=posh&searchon=names&exact=1&suite=all | (documentary; §2 table) |
| dash git log (pipefail commit) | https://git.kernel.org/pub/scm/utils/dash/dash.git/ | **NOT fetched** — robots.txt `Disallow: /` for AI agents; respected |

System reference shell: `dash 0.5.12-6ubuntu5` (`dpkg -s dash`, read-only), WSL2 Ubuntu 24.04.4.
