# 1A1 — Round-1A gate record, source recon, and plan-of-attack

> Round-1A (H2SaLS target-acquisition; priming prompt = quarantined `1A0`). Orchestrator
> note, 2026-06-10. Gate synthesis was delivered in-conversation and passed; this note
> records the rulings it produced, the source recon, and the standing plan. Append-only.

## §1 Gate outcome + human rulings (2026-06-10, in-conversation)

Gate synthesis (two-users / book-vs-oracle / spelled-in-sh / kFAIL + wrong-concrete /
elision-as-replacement / engine-at-HEAD sketch) accepted without correction. Four
pushback items answered; these are **rulings**, quoted near-verbatim:

- **rul-1A-placement** (push-1): the corpus lands at `Research/corpora/H2SaLS/` (NOT
  `spike/e2e/corpora/`). "good call, actually. Research/corpora/H2SaLS/ is fine."
- **rul-1A-defensiveness-range** (push-2): not a single faithfulness pole but a *range*
  of defensiveness — oracles ≈ 0.5–1.0, book ≈ 0.0–0.5 on a notional scale; "occasional
  overlap is fine/realistic/desirable; an admin might reflexively guard something an
  oracle-author separately *also* guarded; how we can handle/benefit from that is one of
  the items-under-test. basically, varying your prompts to subagents might be a good
  idea."
- **rul-1A-filestate** (push-3): file-edit idioms are NOT generic oracle seeds; "file
  state *must* fall, structurally, out of how we handle file-manipulation-oracles, and
  how well we handle fd/stdout/stderr modeling and probing. be judicious and explore this
  space — possibly best of all if the corpus *explores that spectrum* in a realistic way
  a variety of human authors working together with different habits might." Specific
  config-surfaces with probeable facts (sshd via `sshd -T`, sysctl via `sysctl -n`) still
  get real oracles.
- **rul-1A-handlers** (push-4): end-of-section accumulated change-flag restarts accepted
  as the dominant convention; "more than one approach is not bad, as long as that's
  vaguely real-world-multiple-authors-realistic. don't stretch or artificially enforce."

## §2 Source recon (all read-only; +SURE throughout unless marked)

- **The guide**: `Research/sources.json` → `B-imthenachoman-h2sals-2026`, raw URL pinned
  at commit `5abb8c77`. No local cache existed in-repo; re-fetched to scratch and
  **sha256-verified against the graded entry** (`0b1100dc…` — exact match). 3971 lines.
  Scratch copy: `%TEMP%/dorc-1A-sources/h2sals-README.md`.
- **The Ansible rewrite (HTSALSWA)**: submodule dir on this machine is EMPTY
  (uninitialized). Did NOT init it (external-state mutation); instead recovered the pin
  read-only: parent repo toplevel is `C:/Users/ec/System`, whose `.gitmodules` gives
  `url = https://github.com/ELLIOTTCABLE/How-To-Secure-A-Linux-Server-With-Ansible.git`,
  and `git ls-tree` gives gitlink `34975f13406ec6541ee3c3a6499c0af1041e402d`. Shallow-
  fetched exactly that commit to `%TEMP%/dorc-1A-sources/HTSALSWA/` (checkout verified at
  that SHA). The user's fork — "the Ansible rewrite I use" — is the ground truth, not
  imthenachoman's upstream companion repo.
- **Play inventory**: two plays. `requirements-playbook.yml` (remote_user **root**):
  apt update; install sudo; groups `sshusers`/`suusers`/`sudousers`; create `user_name`
  w/ sha512-hashed pw + groups + bash; sudoers `%sudousers` lineinfile; `dpkg-statoverride
  --update --add root suusers 4750 /bin/su` with tolerated already-exists failure;
  passwordless-sudo lineinfile w/ `visudo -cf` validation; authorized_key from
  controller-local pubkey. `main-playbook.yml` (remote_user `user_name`, per-task become):
  roles in order **packages → ssh → password-quality → unattended-upgrades → firewall →
  mail → clamav → rkhunter → auditd → lynis**. 604 YAML lines total.
- **Cross-role wiring that must survive the rewrite**: ssh role pre-emptively adds ufw
  limit rules for port 22 AND `ssh_port`, then `meta: flush_handlers` to restart ufw
  *immediately* (before sshd_config edits); `notify: "firewall : restart ufw service"`
  crosses roles. Handlers: restart ssh / ufw / psad / fail2ban.
- **Vars** (`group_vars/variables.yml`): sshpub_location, root_pw (**never used by any
  task** — see dec-7), user_name, user_pw, ssh_port=55899, mail_to/from/smtp_server/pw/port.
- **Upstream warts noticed on first read** (preserve-semantics + flag policy, dec-8):
  `gecoschec` typo in the pam_pwquality line; lynis role pipes through `ansi2html` which
  no task installs; auditd role does unguarded `rm` of audit.rules (fails on re-run);
  rkhunter role's `cp -p` conf-copy is unguarded (clobbers `.local` on every run, unlike
  ssh-moduli's `creates:` guard); `psad --sig-update`, testmail, `rkhunter --update
  --propupd`, lynis audit are non-idempotent/network tail-tasks.
- **dash**: msys env HAS `/bin/dash` — the `dash -n` gate runs natively here.

## §3 Standing decisions (mine; flag-up-not-relitigate unless human overrules)

- **dec-1 (shape)**: ONE book file, `Research/corpora/H2SaLS/harden.sh`, sections in play
  order with the requirements-play as the first section (the two-play / two-remote_user
  split dissolves under "run as root on the target"; header comment records the original
  split). No sourcing, no multi-file — admin-natural for this genre, and keeps census
  line-xrefs trivial. ~SUSPECT right; revisit only if assembly exceeds ~1200 lines.
- **dec-2 (posture)**: `set -eu` at top (careful-admin reflex for a hardening runbook;
  NOT shaped to the engine — noting explicitly that this lands in the 207 errexit/YOLO
  tension and makes mutator-elision-under-set-e a headline D4 row, which is honest).
  No `pipefail` (not dash).
- **dec-3 (vars)**: play vars map to UPPER_SNAKE shell globals at top of file
  (`USER_NAME`, `USER_PW`, `SSH_PORT=55899`, `MAIL_TO`, …, `SSH_PUBKEY` holding the *key
  text* — the controller-local `lookup('file')` has no target-side analogue; an admin
  pastes the pubkey). Placeholder secrets stay obvious placeholders (`PASSWORD_HERE`).
- **dec-4 (handlers)**: per rul-1A-handlers — per-service change-flags
  (`sshd_changed=0` … `=1` after each mutating edit; one guarded restart at section end).
  EXCEPTION kept faithful: the ssh role's `flush_handlers` becomes an *inline* ufw
  restart right after the two pre-emptive rules (that early restart is load-bearing
  intent: firewall holes open before sshd changes).
- **dec-5 (fan-out)**: three Opus builders on contiguous role-groups with deliberately
  different personas/defensiveness (rul-1A-defensiveness-range):
  - **builder-A** (~0.4–0.5, meticulous; validates risky edits — `visudo -cf`,
    check-before-statoverride): requirements + packages + ssh + password-quality.
    Strain-note slug RESERVED: `1A3`.
  - **builder-B** (~0.1–0.2, scrappy heredoc-blaster, minimal guards): unattended-
    upgrades + firewall + mail. RESERVED: `1A4`.
  - **builder-C** (~0.3, mixed habits; faithful to the play's own `creates:` guards, no
    more): clamav + rkhunter + auditd + lynis. RESERVED: `1A5`.
  Fragments to `%TEMP%/dorc-1A-build/section-{A,B,C}.sh`; I assemble + preamble + gate.
- **dec-6 (anchor comments)**: each section header carries the guide anchor (title +
  line-range at pin `5abb8c77`) and the play file it renders. Plain human comments;
  census may key on them, Dorc never does (no kOOB breach — it's documentation, not
  config).
- **dec-7 (root_pw)**: unused-by-the-play var is OMITTED from the book, recorded as a
  finding (the play carries dead config; a transcribing admin drops it).
- **dec-8 (warts)**: upstream bugs/oddities are *preserved at the semantic level and
  flagged* in strain-notes, never silently fixed — D1 renders the play warts-intact
  (crosscheck targets MAPPING errors only, per 1A0).
- **dec-9 (scratch)**: all reference material + build intermediates live OUTSIDE the
  synced tree (`%TEMP%/dorc-1A-*`) — SyncThing hygiene; pinned URLs+SHAs in the corpus
  README make re-fetch mechanical.

## §4 Plan-of-attack (confidence-marked)

D1 (now): dispatch A/B/C in parallel → assemble `harden.sh` → `dash -n` + my line-by-line
read-through vs the play → commit → max-capability adversarial-crosscheck pair (Fable;
neutral "assess this mapping" + disowned-adversarial "find where the mapping breaks")
→ adjudicate in `1A6`, fix, re-gate, commit. +SURE this shape works; ~SUSPECT the
crosscheck yields 3–8 genuine mapping findings (the moduli/before.rules/Port-lookahead
spellings are the likeliest break-points).

D2: one Opus builder for a re-runnable census tool (`Research/corpora/tools/`,
sh+awk, pure text-in/text-out) + its run over `harden.sh`; I spot-verify counts by hand
on two sections; light verification instead of a full crosscheck pair unless the awk
gets clever. ~SUSPECT ~40–60 distinct external commands, with sed/grep/printf/systemctl
/apt-get dominating frequency.

D3: read 3–4 `spike/e2e/cases/*/*.oracle.sh` exemplars first (me + each builder);
fan out Opus seeds for top D2 rows — -GUESS the set lands ≈ {apt/dpkg, ufw, systemctl,
sshd_config-surface (probe `sshd -T`), sysctl?, user/group (useradd/groupadd/chpasswd),
fail2ban, ufw-before-rules/file-block edits as the *exploration* per rul-1A-filestate,
crontab, dpkg-statoverride}. Per-seed un-modelables recorded as primary findings.
Defensiveness 0.5–1.0 *varied across seeds* per rul-1A-defensiveness-range.

D4: I draft the matrix (per D2-item: freq × criticality × {sh-rewrite, oracle, engine}
difficulty, engine cited to specific inv-top-reject triggers / brk-N / an-* rows,
verified against spike code+corpus where load-bearing); hostile crosscheck pair on the
biggest claims; durable to `Research/plans/1A?-capability-matrix.md`.

**Forecast: hardest matrix rows** (-GUESS, to be falsified by evidence): the
blockinfile/lineinfile file-edit family (frequency×criticality maximal; engine needs
redirect-effects + heredoc bodies + the sed/grep idiom zoo); command-substitution in
argv (`$(hostname)`, key-paths — brk-4/an-top-surface); run-delta restarts under
change-flags (un-state-probeable class; needs the variable-flow the engine only has for
constants); the `dpkg-statoverride`-style tolerated-failure idiom (rc-consumption vs
fork-mutator-rc); pipes-into-`mail`/testmail-style non-idempotent network tail-tasks
(no oracle can vouch; pure MustRun — cheap engine-wise but a coverage-ceiling row);
`for`-loops over config k/v pairs IF builders write them (L2 handles literal lists —
partial-member rewriting still missing).

## §5 Process log (running)

- **proc-1**: gate ran on root-checkout copies of the human docs (root has uncommitted
  KNOBS/IMPLEMENTATION/TODO edits not in this worktree's branch point) — content drift
  watch stands.
- **proc-2**: a broad `Grep` over `Research/` for source-location incidentally surfaced
  TWO lines of the quarantined round-21 priming prompt (`210-…`) — enough to see that
  round-21 expects this round's corpus at this worktree's `Research/corpora/H2SaLS`
  (consumes in-place) with an ~80%-elision north-star. Quarantine otherwise respected;
  future greps exclude that dir. Flagging per the honesty rule, and it confirms
  rul-1A-placement's path matters beyond this round.
- **proc-3**: HTSALSWA submodule empty on this machine; recovered pin read-only (§2). If
  you'd rather I work from an *initialized* submodule, init it yourself and tell me —
  content-identical by SHA either way.
- **proc-4**: `gh search repos` returned empty for the companion-repo query (shrug;
  .gitmodules made it moot). Kagi not needed yet.
