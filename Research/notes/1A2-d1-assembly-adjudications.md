# 1A2 — D1 assembly: orchestrator adjudications over the builder fragments

> LLM-generated (round-1A orchestrator). Part of the artificial-testing-corpus effort
> described in `1A1`; not real security code; cannot expose the truth of real-world
> ops-code. Records how `Research/corpora/H2SaLS/harden.sh` was assembled from the three
> builder fragments (notes `1A3`/`1A4`/`1A5`), what I changed, and why. Append-only.

## §1 Assembly facts

- Builders ran clean-context (no Dorc docs), Opus-class, personas/bands per 1A1 dec-5.
  Token spend: A 74k / B 70k / C 54k. All three self-gated `dash -n` clean.
- Assembled: my preamble (disclosure header per rul-1A-llm-disclosure; provenance; the
  two-plays-dissolved note; `set -eu`; 9 UPPER vars — `root_pw` omitted per dec-7; root
  check) + sections 1–4 (A) + 5–7 (B) + 8–11 (C). 669 lines. `dash -n`: PASS.
- The root check (`[ "$(id -u)" -eq 0 ] || exit`) is an orchestrator ADDITION with no
  play analogue (the play's remote_user/become covers it). Near-universal real-runbook
  reflex; +SURE it belongs; it is also deliberately a command-substitution-in-guard
  specimen on line 1 of the body.
- Builder fragments preserved untouched at `%TEMP%/dorc-1A-build/section-{A,B,C}.sh`;
  all my edits were applied to the assembled file only and are listed in §2.

## §2 Orchestrator edits to A's section (adjudicating A's tc-flags)

- **fix-1 (A-tc-4)**: `apt-get -y dist-upgrade` → `apt-get -y upgrade`. A claimed
  Ansible `apt: upgrade: yes` "historically means dist/full". ~SUSPECT that is wrong:
  `yes` ≡ `safe` (aptitude safe-upgrade / apt-get's safe path), `dist` is a distinct
  enum value the play did not choose. Plain `upgrade` is the conservative, closest
  spelling (C independently chose it for the same construct in §11 — consistency).
  Crosscheck directed to re-verify this mapping.
- **fix-2**: `ufw limit in 22/tcp`/`"$SSH_PORT/tcp"` → bare `22`/`"$SSH_PORT"`. The
  module emits proto-less rules (tcp+udp); B's §6 rules are bare; A's /tcp would have
  created a *second, distinct* ufw rule for the ssh port rather than converging with
  §6's duplicate (the play itself adds the ssh_port limit twice — once per role — and
  converges to ONE rule only because the spellings are identical). The admins-write-/tcp
  observation is real and recorded as **find-module-defaults** for the matrix: Ansible
  modules encode defaults (proto-less rules, last-match lineinfile, create-home) that
  no human admin would spell, and faithful sh must choose a side per-instance.
- **fix-3 (A-tc-1, B-tc-1)**: `ufw reload` → `service ufw restart` at the
  flush_handlers point. The flushed handler IS "restart ufw service"
  (`service: state=restarted`); B's section-end restarts use the same verb. The
  resulting double-restart across the book (inline at §3 + end of §6) is play-faithful
  (flush_handlers mid-play + end-of-play handler run = two restarts there too). KEEP.
- **fix-4 (A-tc-3 / s-ssh-4)**: Port branch rewritten to *literal lineinfile behavior*:
  exact `Port 22` present → still APPEND `Port $SSH_PORT` (host listens on BOTH during
  setup), any other `Port` → replace, none → append; plus convergence short-circuits so
  re-runs don't spuriously set `sshd_changed`. A implemented the play-comment's intent
  (leave 22, add nothing) — defensible, but the literal behavior is corroborated by the
  play pre-opening ufw for BOTH 22 and `$SSH_PORT` and by fail2ban jailing
  `port = $SSH_PORT`: the system-level reading wants the new port LIVE. A's reading
  would leave `$SSH_PORT` dead (ufw + jail pointing at a port sshd never listens on).
  +SURE literal is the better corpus rendering; the intent-vs-literal fork itself is
  preserved in A's s-ssh-4 and is matrix material (PCRE-in-module → sh branching).

## §3 tc-adjudications (no code change)

- **A-tc-2 / C-s-rkh-2** (sed rewrites every match vs lineinfile's last-match): accept;
  benign on these single-occurrence keys; recorded for the matrix (module-semantics
  fidelity class, same family as find-module-defaults).
- **A-tc-5** (skip-if-exists vs converge for user/groups): accept — admin-realistic;
  the narrowing-vs-declarative-converge delta is itself corpus value.
- **A-tc-6 ↔ B-tc-4** (change-detection spectrum): accept BOTH ends deliberately — A's
  real change-gating (cmp/grep -qxF) vs B's vacuous always-set flags is exactly the
  rul-1A-defensiveness-range texture, and gives the analyzer both a genuine run-delta
  specimen and its degenerate cousin.
- **B-tc-2** (set_conf defined §6, reused §7): accept; assembly preserves B's order;
  cross-section function reuse is a realistic multi-author artifact AND a brk-2
  (function inlining) specimen.
- **B-tc-5** (whitespace-wart cut-line: preserve structural warts, drop invisible
  trailing spaces): accept B's line.
- **C-tc-1** (cron.d vs crontab-merge): cron.d stands; the crontab read-modify-write
  spelling is noted as a possible future corpus-enrichment variant, not done now.
- **C-tc-4** (which implicit module side-effects to unroll): accept C's rule
  (spell side-effects that change files/repos; not in-memory apt state).
- **C-tc-2** (warts reasoned under `set -eu`): stands — the book IS set -eu (dec-2).

## §4 Verifications performed (verify-don't-relay)

- `gecoschec` typo: confirmed upstream in the GUIDE itself (L1275/L1288/L1294 at
  pin 5abb8c7), not just the play. A's claim verified.
- B's s-uu-2 (play comments out two origins-pattern lines the guide ships live):
  confirmed against `roles/unattended-upgrades/tasks/main.yml`.
- B's s-fw-3 trailing-space regexp `'^ENABLE_AUTO_IDS '` guard: confirmed in role yaml.
- All three fragments read line-by-line against the role yamls (in-context) before
  assembly; package list (20 names, order), the 13 sshd_config items, the 6 psad keys,
  the 8+7 rkhunter pairs, and all heredoc bodies matched the play modulo flagged warts.
- Correction to C's s-lyn-4 parenthetical: in `lynis audit system | ansi2html -l > f`,
  a missing ansi2html makes the PIPELINE rc 127 (last stage), so under `set -eu` the
  book DOES abort there (final section, post-everything-else); the `mail` line is a
  separate command, not the pipeline's tail. C's artifact is correct; only its note's
  aside was muddled. +SURE.

## §5 Handoff to the D1 crosscheck (task #2)

Directed targets (mapping-fidelity only; upstream bugs are preserved-by-design):
re-verify fix-1's upgrade:yes≡safe claim; the fix-4 literal-Port reading; every
blockinfile rendering (A's marked-block, B's overwrite-equivalences) against module
semantics on a fresh Debian host; the statoverride tolerance glob; heredoc quoting
choices (B's `${distro_codename}` literal vs the expanding jail/msmtp docs);
B's awk insert-before-COMMIT vs blockinfile insertbefore=LAST-match (single-COMMIT
assumption); C's read-loop TAB splitting; chpasswd-vs-`useradd -p` hash equivalence;
anything where the assembled whole (set -eu + section interactions, e.g. §3's ufw
calls before §6's enable) diverges from the two-play execution.
