# 1A6 — D1 crosscheck reconciliation: the mapping-fidelity pair, adjudicated

> LLM-generated (round-1A orchestrator); part of the artificial-corpus effort (see 1A1
> rul-1A-llm-disclosure) — not real security code; cannot expose the truth of real-world
> ops-code. Reconciles the two clean-context Fable crosscheck passes (neutral `xn-*`,
> adversarial `xa-*`) over `harden.sh`, records fix-vs-record rulings, and corrects
> builder-note claims. Crosscheck method per the adversarial-crosscheck skill: neutral +
> disowned-adversarial, identical inputs (artifact, play, guide — NOT the strain notes),
> coverage-not-calibration. Spend: 143k + 157k tokens. Convergence was heavy; convergent
> finds are the trustworthy core. Post-fix tree: `dash -n` PASS, 696 lines.

## §1 Headline: two systematic mapping-error classes, both MINE to own

- **err-shell-snippet-rc** (xn-1/xa-2 lynis pipeline; xn-2/xa-1 rkhunter --update;
  xn-4/xa-3 auditd rm/wget — all convergent, all +SURE, scratch-verified by both):
  Ansible `shell:` runs its snippet via `sh -c` WITHOUT errexit — only the LAST
  command's rc decides the task. The book ran those lines bare under `set -eu`,
  inverting three tolerated-failure semantics (the worst: as assembled, the book could
  NEVER complete a run — the missing-ansi2html pipeline rc-127 aborts at the final
  section, where the play mails an empty report and ends green; rkhunter --update even
  exits 2 on its SUCCESS-with-updates path). Root cause: MY builder briefs primed the
  bare reading ("both unguarded — preserve"); C complied and even reasoned the aborts
  as faithful. Seeding-feedback: the one-snippet-rc rule belonged in the briefs.
  **FIXED** (f-1/f-2/f-3): `|| true` on the tolerated lines — the admin-idiomatic
  tolerance spelling — with comments stating the play's rc semantics; the
  task-rc-deciding last commands stay bare.
- **err-handlers-endplay** (xn-3/xa-6, convergent, +SURE): Ansible handlers run ONCE at
  end of PLAY (after lynis), not at role end; only the ssh role's `flush_handlers` is
  inline. My dec-4 "end-of-section restarts" was wrong and my briefs mandated it; both
  passes flagged that the artifact's comments asserted fidelity exactly where it broke.
  Consequences they drove concrete: mid-run sections see restarted services where the
  play's don't; under mid-run failure the play restarts NOTHING (no force_handlers)
  while the old book had already restarted. **FIXED** (f-4): restarts moved to one
  end-of-play handlers block (ssh, ufw, psad, fail2ban — handler-definition order);
  §3's flush-point inline ufw restart retained; section-end comments corrected.

## §2 Other fixes applied

- **f-5** (xn-8/xa-10, convergent, scratch-verified): NOPASSWD sudoers guard/replace
  asymmetry — `grep -q "^$USER_NAME"` vs `sed "s|^$USER_NAME .*|"` (space required)
  silently no-ops on tab-separated or bare-prefix pre-state lines the play's bare
  `^{{user_name}}` regexp WOULD replace. Fixed by dropping the space (sed now congruent
  with grep AND with the play's sloppy-prefix wart, which is thereby preserved).
- **f-6** (xn-6/xa-8 secondary): `export DEBIAN_FRONTEND=noninteractive` added to the
  preamble — the apt module exports it per-task; bare `apt-get -y` can stall on
  conffile prompts mid-run, which no non-interactive runbook tolerates (and the
  spelling is a universal real-runbook reflex, not a Dorc-ism).
- 1A2's two open verifications closed by the pair: fix-1 (`upgrade: yes` ≡ safe, NOT
  dist) confirmed from module source by both — with the honest residual that the module
  actually runs `--with-new-pkgs` + force-confold, which the book deliberately does not
  spell (→ imp-module-defaults below); fix-4 (literal Port semantics incl. the
  both-ports append) confirmed by both across all single-Port pre-states.

## §3 Recorded-not-fixed: the module-impedance ledger (D4's primary feed)

Ruling rationale: the charter is admin-idiomatic sh, natural guards and NO further;
emulating module machinery (last-match edits, marker-managed surgical blocks, change
reporting, converge-on-existing) is exactly what real admins do NOT write. These
divergences are the measurement, not defects. Each is a matrix row candidate:

- **imp-change-detection** (xn-5/xa-7): modules report changed/unchanged for free →
  play restarts only on change; sh has no cheap spelling → B's flags are welded-1 and
  the flush-point ufw restart is unconditional, so converged re-runs over-restart.
  A's section shows the expensive honest alternative (cmp/grep -qxF gating). Both ends
  kept deliberately (rul-1A-defensiveness-range texture; prime run-delta material).
- **imp-lineinfile-lastmatch** (xn-9/xa-9 + xn-15): lineinfile replaces only the LAST
  regexp match; sed `s|re.*|val|` rewrites ALL (plus A's exact-line-anywhere
  early-exit). Coincides on stock configs (both passes verified the touched keys);
  diverges on duplicate-directive files — xa's cloud-image duplicate-PermitRootLogin
  case is the sharp version (play preserves the first-wins wart; book "fixes" it —
  safer and therefore unfaithful). Also: book validates/installs sudoers every run vs
  lineinfile only-on-change.
- **imp-blockinfile-truncate** (xn-13/xa-5): `create:true` blockinfile preserves
  content OUTSIDE its markers and surgically maintains the block; B's `cat >` truncates
  whole files. Equivalent on virgin hosts (byte-verified all five blocks); destroys
  foreign config on drifted hosts (jail.local/msmtprc are plausibly admin-owned).
  Severity dispute (xa: mapping-breaking-under-pre-state; xn: subtle-drift) recorded;
  my call: in-charter persona behavior, highest-value impedance specimen in the corpus.
- **imp-blockinfile-anchoring** (xn-13/xa-13/xa-14): insertbefore = LAST unanchored
  match vs awk first-`^COMMIT`; multi-table (`*nat`) before.rules diverges in BOTH
  directions; existence-only marker guard never repairs drifted block content; A's sed
  range-delete runs to EOF on an orphaned BEGIN marker (real sed hazard; blockinfile
  leaves text intact). All edge-pre-state; recorded.
- **imp-module-converge** (xn-7/xa-4): `user:` on an existing user converges groups
  (exact-set), shell, AND password (fresh salt every run — the play resets it every
  rerun!); the book's getent guard skips entirely. xa's sharp consequence: pre-existing
  user not in sshusers + §3's AllowGroups ⇒ play converges access, book locks out.
  Also xa: §1 needs host-side openssl pre-§2 (play hashes controller-side) — minbase
  edge. Both recorded; skip-if-exists IS the admin idiom.
- **imp-geturl-force** (xn-12/xa-11): get_url defaults force:no (download only if dest
  missing; offline-safe re-runs; atomic via tempfile); book wget re-fetches every run,
  non-atomic, aborts re-runs when cisofy.com unreachable.
- **imp-cron-identity** (xn-11/xa-12): cron.d drop-in vs root-crontab-with-MAILTO —
  MAILTO blast-radius shrinks from all root jobs to this file (C documented; both
  passes confirmed job-line byte-equivalence).
- **imp-module-defaults** (from 1A2, enriched): proto-less ufw rules, `--with-new-pkgs`
  + force-confold, DEBIAN_FRONTEND (now partially spelled), create_home, statoverride
  stderr-only matching (xn-14/xa-16), authorized_key key-blob dedup vs whole-line grep
  (xn-16b/xa-17) — the class "modules encode defaults no admin spells".
- Cosmetics, recorded as-is: marker texts/absence, the dropped trailing space
  (B-s-fw-7), openssl-vs-passlib hash rounds (~SUSPECT, plaintext-equivalent).

## §4 Builder-note corrections (their notes stand; this supersedes)

- C s-aud-1 / s-rkh-3 / s-lyn-4: the "latent abort is faithful" reasoning was wrong —
  the play tolerates those failures (one-snippet rc). Superseded by §1; C's tc-2
  (set -e sensitivity of those findings) was the right instinct, wrong conclusion.
  C's s-lyn-4 aside about pipeline rc was also muddled (1A2 §4) — the pipeline rc IS
  127; what saves the play is the snippet continuing past it.
- A s-pkg-1: `upgrade: yes ≡ dist` claim wrong (≡ safe); my fix-1 confirmed, nuanced.
- A s-ssh-4 / tc-3: comment-intent reading overturned for literal (1A2 fix-4); both
  passes endorse literal as the play's behavior; A's documentation of both readings was
  exactly right conduct.
- B tc-4: upgraded from "lossy on hypothetical re-runs" to the named
  imp-change-detection row.

## §5 What the pair confirmed HOLDING (D4 confidence anchors)

Byte-fidelity of every templated block (mozilla sshd block, 51myunattended-upgrades,
msmtprc, both fail2ban files, pam line incl. gecoschec, cisofy .list, clamscan line,
psad/rkhunter k/v sets, 20-package list); §5's quoted-heredoc `${distro_codename}`;
unquoted-heredoc safety elsewhere; all `set -eu` micro-traps (statoverride capture,
`[ -z ] && continue`, `if ! cmp`, cmp-against-heredoc); rkhunter tab-table read-loop
mechanics; ufw CLI spellings ≡ module rules; the visudo candidate-file dance; moduli
rendering incl. changed_when ≡ cmp-branch; flush-point position; two-play dissolution
(every main-play task is become:true — verified task-by-task); root_pw omission.

## §6 Process notes

- Convergence rate was striking: every severity-ranked finding above cosmetic appeared
  in BOTH passes independently (the adversarial pass added sharper consequence scenarios
  + two edge finds xa-13/xa-14). Per the skill's calibration rule, adversarial-only
  finds stay ~SUSPECT: xa-14's blockinfile-orphan-marker contrast is mechanically right
  about the artifact (sed range to EOF) but ~SUSPECT about blockinfile's exact behavior;
  recorded with that mark.
- Neither pass saw the builder strain notes; that they independently re-derived nearly
  every builder-flagged divergence (and graded several UP) validates the clean-context
  design and the notes' honesty both.
- agent-ids: neutral ace42317cb4c6ff33, adversarial a169de12fb0b3a8a8 (full reports in
  session transcript; this note is the durable record).
