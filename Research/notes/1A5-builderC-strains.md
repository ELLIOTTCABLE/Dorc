# 1A5 — builder-C strain note (clamav / rkhunter / auditd / lynis)

> **Disclosure, read first.** This note *and* the shell sections it describes are
> LLM-generated. They are part of an intentionally quality-varied **artificial
> testing corpus** for a static-analysis tool — not real security code, and not a
> faithful window onto real-world ops-code. Nothing here was executed; the only
> tool run against the fragment was `dash -n` (parse-only). The "admin persona,"
> the warts, and the confidence-marks below are all synthetic. Do not mine this for
> security advice or treat any finding as evidence about how real admins write sh.

Persona governs spelling (data-driven `while read` loops, guards-as-found, 4-space,
no bashisms); the play governs semantics. Line refs are into `section-C.sh`.

---

## s-clam-1 — cron module → cron.d drop-in (PRIMARY mapping awkwardness)
- **Play:** `cronvar{name=MAILTO,value=mail_to,user=root}` then
  `cron{name="clamav daily run", minute=0, hour=3, job=clamscan…, user=root}`.
- **Chosen sh** (fragment lines ~12-16): a single `/etc/cron.d/clamav-daily` file
  carrying both the `MAILTO=` line and a 6-field root cron row.
- **Why this, alternatives rejected:**
  - rejected `crontab -l | … | crontab -` (edit-and-reinstall root's user crontab):
    it is the *literal* analogue of what cronvar/cron mutate, but in sh it needs a
    read-modify-write with de-dup logic to stay idempotent, and that hand-rolled
    merge is exactly the kind of fragile cleverness the persona avoids. ~SUSPECT a
    real mid-defensiveness admin reaches for cron.d here too.
  - rejected appending to `/etc/crontab`: same merge problem, plus it edits a
    package-owned file.
- **The awkwardness that is the actual deliverable** (+SURE): Ansible's modules are
  **idempotent by identity** — `cronvar` keys on the variable *name*, `cron` keys on
  the `name:` comment marker it injects (`#Ansible: clamav daily run`). Re-running
  converges. My cron.d file is idempotent only by *whole-file overwrite* (`cat >`):
  re-running clobbers and rewrites, which lands in the same place but for a different
  reason. A static analyzer that models "named cron job, managed" from the Ansible
  side will see, on the sh side, only "a file gets truncated and rewritten" — the
  *job's name/identity* is gone.
- **Second, sharper mismatch** (+SURE): scope. `cronvar MAILTO` sets MAILTO in
  **root's user crontab**, applying to *every* job there. A `MAILTO=` inside a
  cron.d file applies **only to jobs in that same file**. So the semantics genuinely
  diverge: the play's MAILTO is system-wider than my spelling's. I preserved the
  *intent* (mail goes to `$MAIL_TO` for this job) but not the *blast radius*.
  Flagged rather than "fixed" because fixing it (writing root's crontab) reintroduces
  s-clam-1's merge problem.
- **Tainting note for the analyzer** (~SUSPECT): `$MAIL_TO` flows into a heredoc that
  becomes a crontab line; the value is interpolated at write-time, not run-time, so
  there is no `$MAIL_TO` token left in the on-disk artifact — a purely-runtime taint
  tracer would lose the edge here.

## s-clam-2 — clamscan-as-root preserved
- The guide itself warns (README ~2691) that `clamscan` as root is dangerous. The
  play runs it as root anyway. Preserved verbatim in the cron row (-GUESS this
  tension is exactly the sort of thing the corpus wants visible: rationale-doc says
  X, automation does not-X).

## s-rkh-1 — the unguarded `cp -p` (preserved wart)
- **Play:** `shell: cp -p /etc/rkhunter.conf /etc/rkhunter.conf.local`, no `creates:`.
- **Chosen sh** (line ~30): identical, unguarded.
- **Why preserved** (+SURE): the asymmetry is upstream's and deliberate to keep. The
  ssh role's moduli step *is* `creates:`-guarded; this one is not. So every run
  overwrites `.local` with pristine defaults, **discarding any prior edits**, and
  only the two lineinfile loops that follow restore them. Net effect is stable *only
  because the loops always re-run right after*. If anyone ever reorders the section so
  the cp runs without the loop, the admin's customizations vanish silently. Flagged,
  not guarded. ~SUSPECT this fragility is precisely what a "config drift" analyzer
  should catch and is the reason the asymmetry is interesting test material.

## s-rkh-2 — lineinfile loop → `while read` + grep/sed/append
- **Play:** two `lineinfile` loops (8 items on `/etc/rkhunter.conf.local`, 7 on
  `/etc/default/rkhunter`), each item a `{regexp, line}` replace-or-append.
- **Chosen sh** (lines ~34-66): per-item, `grep -Eq` the regexp → if present
  `sed -i -E "s|PATTERN.*|LINE|"`, else `printf >>`. Pairs fed from a
  `regexp<TAB>line` heredoc, matching the persona's data-driven preference over
  eight unrolled `if/sed` blocks or a helper function.
- **Alternatives rejected:**
  - unrolled repetition (one block per key): rejected — 15 near-identical blocks is
    the anti-pattern the persona explicitly avoids.
  - a `set_kv()` helper function: rejected — persona leans loop-over-data, not
    extract-a-function; also keeps the data (pattern→line table) legible in one place.
- **lineinfile semantics NOT cleanly mapped** (+SURE): real `lineinfile` replaces
  **only the last** matching line and is anchored by the full regexp; my `sed -i`
  replaces **every** matching line. For these files (one occurrence per key) the
  behavior coincides, but it is not the same operation. A `lineinfile`-aware analyzer
  that knows "exactly one line ends up matching" cannot infer that from `sed … g`-less
  but still all-lines `sed -i`.
- **Preserved loose-anchor wart** (~SUSPECT): the source regexps are prefix-only and
  `=`-less (`^NICE`, not `^NICE=`). So `^NICE` also matches a hypothetical
  `^NICEXXX=` line. This looseness is upstream's (Ansible's regexp is identically
  loose); I reproduced it rather than tightening to `^NICE=`. Note `^MAIL-ON-WARNING`
  contains a literal `-`, harmless in ERE but a thing the analyzer may want to see is
  *not* a range.
- **Delimiter fragility I introduced** (--WONDER): I used `s|PATTERN.*|LINE|`. If any
  replacement `LINE` contained a `|` the substitution would break. None of the 15 do,
  so it is safe here, but it is a spelling-choice fragility (not present in the play,
  which has no delimiter concept) — recording honestly as a place my rendering is
  *less* robust than the source even though semantics match for this data.

## s-rkh-3 — `--update` then `--propupd`, both unguarded
- **Play:** one shell task, two lines: `rkhunter --update` / `rkhunter --propupd`.
- **Chosen sh** (lines ~70-71): two bare commands, no guard. Both touch the network
  (`--update` pulls mirror data) and mutate state (`--propupd` rewrites the file-
  properties DB). Preserved unguarded and un-`&&`-chained (the play does not chain;
  under the preamble's `set -e` a failure of `--update` already aborts before
  `--propupd`, so the observable ordering is preserved either way). +SURE.

## s-aud-1 — `rm` without `-f`, paired with a refilling `wget` (preserved wart)
- **Play:** one shell task — `rm /etc/audit/rules.d/audit.rules`;
  `wget -P /etc/audit/rules.d/ …/Neo23x0/…/audit.rules`; `service auditd restart`.
- **Chosen sh** (lines ~84-86): identical, inline, unconditional. No handler — the
  play restarts inline, so I do too, and I own no change-flag.
- **Reasoning through the missing `-f`** (the requested analysis, +SURE):
  - *First run:* the `auditd` package ships `/etc/audit/rules.d/audit.rules`, so the
    file exists; `rm` succeeds.
  - *Repeat runs:* the `wget` from the prior run downloaded into the **same path and
    filename** (`-P <dir>` keeps the remote basename `audit.rules`), so the file is
    present again; `rm` succeeds again. The rm/wget pair is self-perpetuating.
  - *The latent break:* if the `wget` ever **fails** (network down, URL moved,
    raw.githubusercontent rate-limit), the file is left **absent**. The next run's
    `rm` then hits a missing file, exits non-zero, and — under the preamble's
    `set -e` — **aborts the whole runbook** at this line. The missing `-f` converts a
    transient upstream-fetch failure into a hard stop on the *next* run. Preserved
    exactly (no `-f`, no `rm -f`, no `|| true`) because the asymmetry/foot-gun is the
    point; "fixing" it would erase the very behavior the corpus is meant to exhibit.
  - *Second-order:* `wget` without `-O` and without `-N` will, if the file somehow
    survived (it should not, post-rm), write `audit.rules.1`. Post-rm that branch is
    unreachable here; noting it only so the analyzer's "wget may create .N suffix"
    rule is seen to be *suppressed* by the preceding rm. ~SUSPECT.
- **`service auditd restart`** (+SURE): inline and unconditional per the play (NOT a
  handler/notify). Restarts even when nothing changed. Preserved.

## s-lyn-1 — get_url(mode) → wget + chmod helper
- **Play:** `get_url{url, dest, mode=0644}` for the cisofy key (`.asc`).
- **Chosen sh** (lines ~99-104): a tiny `get_url() { wget -O "$2" "$1"; chmod "$3"
  "$2"; }` then one call. The persona "keeps guards as found"; get_url has no
  `creates:` here, so the fetch is unguarded (re-downloads every run). +SURE.
- **Mapping note** (~SUSPECT): Ansible `get_url` is *itself* conditionally idempotent
  (checksum/size short-circuit) even without `creates:`; my wget is not — it refetches
  unconditionally. So the sh is strictly more network-touching than the module. This
  is the same class of gap as s-clam-1 (module hides idempotency the sh cannot express
  cheaply) and I let it stand rather than bolt on a checksum guard the play does not
  have.

## s-lyn-2 — `creates:`-guarded dearmor (guard PRESERVED)
- **Play:** `command: gpg --dearmor … -o …keyring.gpg …keyring.asc` with
  `args.creates: …keyring.gpg`.
- **Chosen sh** (lines ~106-110): wrapped in `if [ ! -e …keyring.gpg ]; then … fi`.
  This is the one place in my four roles the source *is* guarded, and I reproduced
  the guard faithfully (contrast s-rkh-1's *un*guarded cp). +SURE that `creates:`
  ↔ `[ ! -e ]` is the correct, idiomatic mapping.

## s-lyn-3 — apt_repository → file-write + `apt-get update` (and the doubled upgrade)
- **Play:** `apt_repository{repo=…cisofy…, filename=cisofy-lynis}`; then a *second*
  `apt{update_cache,upgrade}` block; then `apt{install lynis}`.
- **Chosen sh** (lines ~113-122):
  - write `/etc/apt/sources.list.d/cisofy-lynis.list` via `cat >`, then **one**
    `apt-get update` immediately after — because `apt_repository` refreshes the cache
    as a side effect of adding a repo, and that refresh is semantically part of the
    module. Spelled explicitly so the analyzer sees the cache-update that the Ansible
    module would have done implicitly. ~SUSPECT this implicit→explicit unrolling is a
    mapping wart worth its own mention: one Ansible task becomes *two* sh statements
    (write + update), and a naive 1-task↔1-statement aligner will mis-pair them.
  - **then the play's own second `update_cache + upgrade`** (lines ~119-120):
    preserved as a *separate, redundant* `apt-get update; apt-get upgrade -y`. So the
    cache is updated twice in a row (once for the repo-add side effect, once for the
    explicit task). The play genuinely double-upgrades across the role (once at top,
    once here); both are kept. +SURE this redundancy is intentional-to-preserve.
- **Alternative rejected:** folding the repo-add's implicit update into the explicit
  one (single `apt-get update`): rejected — it would hide the module side effect and
  collapse a distinction the corpus exists to test.

## s-lyn-4 — ansi2html piped but never installed (preserved wart)
- **Play (final shell task):** `lynis audit system | ansi2html -l > /tmp/
  lynis-report.html`. No task anywhere installs `ansi2html`.
- **Chosen sh** (line ~128): reproduced verbatim. +SURE this is a latent runtime
  failure on a clean host (command-not-found → broken pipe; under `set -e` /
  `pipefail`-less dash the rc of the *pipeline* is `mail`'s, so it may even slip by).
  Preserved because the missing-dependency is exactly the kind of thing a static
  analyzer should flag and the corpus should contain.
- **mail -A semantics** (-GUESS): `mail -A <file>` attaches on bsd-mailx/mailutils but
  means something different on heirloom-mailx (`-A` = account). The play assumes the
  attachment meaning; I preserved `-A`. A portability-aware analyzer may want to note
  the `mail` variant is unpinned.

---

## tc-flags — cross-cutting judgment calls I deliberately did NOT settle
- **tc-1 (cron identity):** whether the corpus "wants" the cron.d spelling (what I
  chose) or the root-crontab edit-and-reinstall spelling is a genuine fork. I picked
  cron.d for persona-fit, but the merge-idiom spelling would stress a *different*
  analyzer capability (crontab read-modify-write taint). Left unsettled — a sibling
  builder rendering the same role the other way would be *more* valuable than me
  picking "the right one." ~SUSPECT.
- **tc-2 (`set -e` interaction with preserved un-`-f` rm and un-`&&` rkhunter pair):**
  I assumed the orchestrator's `set -eu` preamble is in force and reasoned the warts
  *under* it (s-aud-1's abort-on-next-run, s-rkh-3's implicit short-circuit). If the
  assembled book ends up *without* `set -e` in some test variant, several of my
  "latent abort" findings invert into "silently continues past failure." I did not
  hedge the fragment for both worlds — it is written for `set -e` as specified.
- **tc-3 (sed `-i` portability):** `sed -i` is GNU-flavored; BSD/macOS `sed -i` needs
  an arg. The corpus is "Debian admin" so GNU sed is the right call, but `dash -n`
  cannot catch this and I did not add a portability shim. --WONDER whether the test
  harness ever runs these on a BSD sed; if so, s-rkh-2's in-place edits break in a way
  orthogonal to the semantics.
- **tc-4 (implicit module side effects, generally):** I unrolled exactly one implicit
  side effect (apt_repository's cache refresh, s-lyn-3). I did *not* unroll others
  (e.g. apt `upgrade: yes` implies an update-then-upgrade; get_url's checksum
  short-circuit). Inconsistent on purpose-ish, but flagging the inconsistency: a
  reviewer may want all-or-nothing on "spell out what the module does implicitly."
  -GUESS the right policy is "spell only the side effects that change *files/repos*,
  not those that change *in-memory apt state*," which is the line I drew, but it is a
  judgment call, not a settled rule.
- **tc-5 (heredoc quoting of `$MAIL_TO`):** both the clamav cron.d heredoc and the
  rkhunter/lynis heredocs are *unquoted* (`<<EOF`), so `$MAIL_TO` expands at
  write-time. That is what I want for the cron.d/source-list content. But it means an
  unquoted `$` or backtick that ever entered those static blocks would also expand;
  the blocks are currently all-literal-except-the-intended-vars, so it is safe, but I
  did not defensively switch the *fully-static* lynis source-list heredoc to `<<'EOF'`
  (it has no vars). Left as-is for uniformity; a stricter author would quote the
  variable-free heredocs. --WONDER.
