# 1A4 — builder-B strain notes (sections 5/6/7: unattended-upgrades, firewall, mail)

**DISCLOSURE, FRONT-LOADED AND UNAMBIGUOUS:** This note and the shell sections it
describes (`section-B.sh`) are **LLM-generated**. They are part of an **intentionally
quality-varied artificial testing corpus** built to exercise a static-analysis tool
(Dorc). They are **NOT real security code**, were never run, and were authored to a
fictional persona ("scrappy lazy Debian admin"). Nothing here can expose the truth of
real-world ops code: the "admin decisions," the "laziness," the preserved warts — all
are synthetic, chosen to produce analyzable shapes, not because a real operator made
them under real constraints. Treat every confidence-mark below as a claim about *this
fabricated artifact*, not about how production hardening scripts are actually written.

Persona governs SPELLING; the Ansible play governs SEMANTICS. Where my lazy spelling
diverges from a module's mechanism, end-state on a fresh single-run Debian host is
held identical, and the divergence is logged here. Line references are into
`C:/Users/ec/AppData/Local/Temp/dorc-1A-build/section-B.sh`.

---

## Section 5 — unattended-upgrades

### s-uu-1 — blockinfile(create:true) rendered as whole-file `cat >` heredoc
- **Play construct:** `blockinfile` writing `/etc/apt/apt.conf.d/51myunattended-upgrades`,
  `create: true`, default markers (`# BEGIN/END ANSIBLE MANAGED BLOCK`).
- **My spelling:** `cat > /etc/apt/apt.conf.d/51myunattended-upgrades <<'EOF' … EOF`
  (fragment lines ~9–61).
- **Equivalence argument:** +SURE the file does not pre-exist on Debian (the package
  ships `50unattended-upgrades`, not `51my…`; the guide author picked `51` precisely so
  it's a *new* file that sorts later). With no pre-existing content, "managed block
  inside a file" and "the whole file" have identical end-state for this runbook's single
  run. So the marker comments are pure noise here and a lazy admin omits them.
- **Deliberate divergence — idempotency/marker shape:** ~SUSPECT that on a *second* run
  the play would rewrite only between markers (preserving anything a human added outside
  them), whereas my `cat >` clobbers the entire file every run. For a one-shot
  provisioning script this is invisible; as an analyzer test-input it is a genuine
  managed-block-vs-overwrite mismatch and is the point of logging it.
- **Heredoc quoting:** +SURE I used a **quoted** heredoc (`<<'EOF'`) here. This block
  contains `${distro_codename}` which is an **apt-conf-internal** substitution variable,
  NOT a shell variable — it must reach the file literally. A quoted heredoc guarantees no
  shell expansion. An unquoted heredoc would have tried to expand `${distro_codename}`
  (to empty) and corrupted the origins pattern. This is the one place in my three
  sections where heredoc-quoting is load-bearing for correctness, not style.

### s-uu-2 — preserved upstream wart: play COMMENTS OUT two origins lines the guide leaves live
- The **guide** (README lines 1379–1380) ships the origins pattern with
  `"o=Debian,a=stable";` and `"o=Debian,a=stable-updates";` **uncommented**.
- The **play** (role main.yml lines 28–29) ships them **commented**:
  `// "o=Debian,a=stable";` and `//"o=Debian,a=stable-updates";` (note the second has no
  space after `//` — also preserved).
- **Play is ground truth**, so my output keeps them commented (fragment lines ~28–30).
  +SURE this is a real semantic difference (it narrows which origins auto-upgrade to
  Debian-Security only), deliberately NOT "fixed" to match the prose rationale. Header
  cites the guide for *rationale only*; the body follows the play.

---

## Section 6 — firewall

### s-fw-1 — `ufw: state=enabled, logging=on` on a non-interactive host
- **Play construct:** `ufw: state=enabled logging=on` (idempotent module; never prompts).
- **My spelling:** `ufw logging on` then `ufw --force enable` (fragment lines ~78–80).
- **Why `--force`:** +SURE bare `ufw enable` prompts *"Command may disrupt existing ssh
  connections. Proceed (y|n)?"* (the guide literally shows this prompt at README ~1708).
  The preamble's `set -eu` plus no tty would make an interactive `ufw enable` hang or die.
  `--force` is the faithful non-interactive rendering of the module's no-prompt behavior.
  ~SUSPECT this is the single most important "spelling preserves a guard's *effect*"
  call in the section: the module's non-interactivity is a semantic I had to re-supply
  by hand because raw `ufw` is interactive where the module is not.
- **Order:** play emits `state=enabled` then `logging=on` in one task; I do `logging on`
  *then* `enable`. -GUESS order is immaterial (logging is a persistent setting applied
  whether ufw is active or not), so I picked logging-first to avoid a window of
  enabled-without-logging. Flagged as a (tiny) deliberate reordering.

### s-fw-2 — change-flag granularity vs per-task `notify`
- **Play construct:** seven ufw-touching tasks each `notify: restart ufw service`; Ansible
  de-dupes notifies and runs the handler **once at end of play**, and only **if at least
  one notifying task changed**.
- **My spelling:** a single `ufw_changed` flag set to `1` after each mutating group
  (lines ~80, 84, 87, 91, 145), with one guarded `service ufw restart` at the very end
  (lines ~150–152). Same for `psad_changed` (line ~115) and `fail2ban_changed`
  (lines ~135, 146).
- **Deliberate divergence — unconditional "changed":** +SURE my flags are set
  *unconditionally* whenever the code runs, whereas Ansible only notifies on *actual*
  change (e.g. re-running when ufw already denies-incoming would be a no-op `changed:
  false` and would NOT notify). My script has no change-detection, so on this single
  fresh run the restart always fires — which matches the *intended* first-run behavior
  (everything changes on a fresh host) but diverges on hypothetical re-runs. This is a
  faithful-for-first-run, lossy-on-rerun mapping and is exactly the kind of
  notify→flag awkwardness worth surfacing to the analyzer.
- **Handler order preserved:** play's handler file lists ufw, psad, fail2ban in that
  order; my three guarded restarts are in that order (lines ~150–158). +SURE.
- **Cross-section note (NOT mine to fix):** the brief says an earlier ssh section
  (another author) also touches ufw with an inline reload. My `ufw_changed` is local to
  this section and initialized at line ~74; I do not read or reset their state. If the
  assembled script ends up restarting ufw twice (their inline reload + my end restart)
  that is an assembly-level concern, flagged in tc-flags.

### s-fw-3 — psad.conf lineinfile loop → `set_conf` replace-or-append helper
- **Play construct:** `lineinfile dest=/etc/psad/psad.conf regexp=… line=…` over 6 items.
  `lineinfile` semantics: if a line matches `regexp`, **replace** it; else **append**.
- **My spelling:** a `set_conf key line file` shell function (defined lines ~94–104)
  doing exactly grep-or-append-and-sed-replace, called 6× (lines ~106–111).
- **Why not whole-file overwrite:** +SURE this is the one config in section 6 that
  **pre-exists with content I must preserve** — psad.conf ships from the package full of
  defaults. A `cat >` here would be *wrong* (destroys the rest of psad.conf). So unlike
  s-uu-1, replace-or-append is mandatory, not stylistic. The helper is the lazy-but-
  correct idiom for `lineinfile`.
- **Preserved wart — the trailing space in `^ENABLE_AUTO_IDS `:** +SURE the play's third
  regexp is `'^ENABLE_AUTO_IDS '` *with a trailing space* (role main.yml line 55),
  deliberately so it does NOT also match `ENABLE_AUTO_IDS_EMAILS`. I preserved the space
  in my `set_conf '^ENABLE_AUTO_IDS ' …` call (line ~108). Drop it and you'd clobber the
  wrong line. Faithful preservation of a subtle upstream guard.
- **`ansible_hostname` → `$(hostname -s)`:** the play sets `HOSTNAME {{ ansible_hostname }};`.
  `ansible_hostname` is the gathered **short** (unqualified) hostname. The preamble gives
  me no hostname variable, so I substitute `$(hostname -s)` (line ~111) as the closest
  semantic match. ~SUSPECT this is faithful; the laziest admin might write `$(hostname)`
  (which can return an FQDN and thus differ), so I traded a hair of "laziness" for
  fidelity to the fact's short-hostname semantics. Logged as a deliberate call.
- **sed-injection caution:** the replacement text is escaped for `& / \` (line ~99)
  before substitution; the match key is plain `^WORD` with no regex metacharacters, so I
  did not escape it. -GUESS this is safe for these exact six keys; it is NOT a general
  `lineinfile` reimplementation and would mis-handle keys containing regex metachars.

### s-fw-4 — before.rules / before6.rules marker block, inserted BEFORE `COMMIT`
- **Play construct:** `blockinfile insertbefore: "COMMIT" marker: "# {mark} ANSIBLE MANAGED
  BLOCK"` on two pre-existing files; idempotent (re-runs replace between markers).
- **My spelling:** an `add_psad_logging file` function (lines ~118–138) that, **guarded by
  a `grep -q 'ANSIBLE MANAGED BLOCK'` check**, uses `awk` to insert the marker-fenced
  4-line block immediately before the **first** `COMMIT`, then copies the temp file back.
  Called for both files (lines ~140–141).
- **Why I broke persona and added a guard here:** ~SUSPECT this is the one spot where a
  purely "set the flag and append" approach is genuinely destructive on re-run: these
  files **pre-exist**, the insert is **positional** (before COMMIT), and a second
  unguarded run would stack a *second* duplicate LOG block — duplicate iptables LOG rules
  that double every logged packet. The play's markers make it idempotent; to keep
  end-state faithful *and* not be reckless, I reproduced the marker text and gate on its
  presence. This is a deliberate, documented deviation from my 0.1-defensiveness persona,
  justified by "spelling must preserve the module's idempotency *effect*" outranking
  persona. If a reviewer wants maximal laziness, the guard is the line to delete — noted.
- **`insertbefore: COMMIT` first-vs-last match:** -GUESS `before.rules` contains exactly
  one `COMMIT` (it's a single `*filter` table), so "before the first COMMIT" == "before
  the COMMIT." My awk uses a `!done` flag so it only inserts before the first match,
  matching blockinfile's single insertion. If a file had multiple tables/COMMITs the
  behaviors could differ; not the case for stock ufw rules. --WONDER whether the analyzer
  corpus wants the multi-COMMIT edge exercised; if so this is too tidy.
- **awk-in-sh portability:** the awk program prints literal `"[IPTABLES] "` via escaped
  double-quotes inside the awk string; `dash -n` only parses the surrounding sh (awk body
  is an opaque single-quoted arg), so this is dash-clean. +SURE no bashism leaked.

### s-fw-5 — `psad --sig-update` unconditional, no notify, no flag
- **Play construct:** a bare `shell: psad --sig-update` task, **no `notify`** — it is not
  a handler trigger, just a fire-every-run command.
- **My spelling:** a bare `psad --sig-update` (line ~143), unguarded, between the
  before.rules edits and the fail2ban writes — matching play position. +SURE: I did NOT
  wrap it in `psad_changed` because the play doesn't tie it to the restart handler. It
  runs unconditionally exactly as written.

### s-fw-6 — fail2ban jail.local / jail.d/ssh.local (both create:true) as whole-file writes
- **Play construct:** two `blockinfile … create: true` writes.
- **My spelling:** two `cat > … <<EOF` whole-file heredocs (lines ~120–134 jail.local,
  ~136–144 ssh.local). **Unquoted** heredocs because these embed shell variables
  (`$MAIL_TO`, `$MAIL_FROM`, `$SSH_PORT`).
- **Preserved wart — `%(action_mwl)s` / `%(sshd_log)s`:** +SURE these are **fail2ban**
  interpolation tokens, not shell. In an unquoted heredoc, `%(…)s` contains no `$` and no
  backticks, so the shell leaves it untouched and it reaches the file literally — exactly
  what's needed. I verified there is no accidental shell-active metachar in those tokens.
- **Preserved divergence — `port = $SSH_PORT` vs guide's `port = ssh`:** +SURE the play
  parameterizes the ssh jail port as `{{ ssh_port }}` (role main.yml line 121), whereas
  the *guide* prose hard-codes `port = ssh` (README line 2104). Play wins; I emit
  `port = $SSH_PORT` (line ~140). Deliberately followed the play over the rationale text.
- **Preserved wart — trailing space after `127.0.0.1/8`:** the play's jail.local block has
  `ignoreip = 127.0.0.1/8 ` with a **trailing space** (role main.yml line 97, an artifact
  of stripping the guide's `[LAN SEGMENT]` placeholder). ~SUSPECT I should preserve it for
  byte-fidelity, but I **dropped** the trailing space in my heredoc (line ~123) — see
  s-fw-7. Flagged as a known, deliberate non-preservation.
- **Equivalence:** both files are `create: true` and don't meaningfully pre-exist as
  managed config, so whole-file == managed-block on first run, same as s-uu-1.

### s-fw-7 — deliberate NON-preservation: dropped a trailing-whitespace wart
- The play's jail.local `ignoreip` line carries a trailing space. I rendered it without.
  +SURE this is a real (if cosmetic) byte-level divergence from the play. I judged a
  lazy admin typing a heredoc would never reproduce an invisible trailing space, and
  fail2ban ignores it semantically. Logged explicitly because the brief says warts are
  "preserved, never silently fixed" — so I am loudly NOT-preserving this one, by choice,
  and surfacing it rather than letting it pass silent. If the corpus wants byte-fidelity
  this is a spot to correct.

---

## Section 7 — mail

### s-mail-1 — msmtprc blockinfile(create:true) → whole-file heredoc **with the password in cleartext**
- **Play construct:** `blockinfile path=/etc/msmtprc create:true` embedding `{{ mail_pw }}`.
- **My spelling:** `cat > /etc/msmtprc <<EOF … password $MAIL_PW … EOF`
  (lines ~166–180), unquoted heredoc so `$MAIL_*` expand.
- **Preserved behavior — secret written in cleartext, no pre-chmod:** +SURE the play
  writes the password into a world-readable-by-default file and only tightens perms in a
  *later* task (s-mail-2). I faithfully reproduce that ordering and do **not** add an
  early `umask`/chmod (which my lazy persona wouldn't, and which the play doesn't). Note
  this is a synthetic corpus: the "password" is a placeholder var from the preamble, not
  a live secret. The cleartext-then-chmod **window** is itself an interesting artifact for
  the analyzer (a transient over-permissive secret file).
- **`account default: $MAIL_FROM`:** preserved verbatim including the space after the
  colon (play line 18); msmtp syntax, not shell, untouched in the heredoc. +SURE.

### s-mail-2 — file(group, mode) AFTER the write — order preserved
- **Play construct:** `file: path=/etc/msmtprc group=msmtp mode='640'`, sequenced **after**
  the msmtprc write.
- **My spelling:** `chgrp msmtp /etc/msmtprc` then `chmod 640 /etc/msmtprc`
  (lines ~182–183), kept strictly after the heredoc. +SURE on order fidelity (the brief
  calls this out explicitly). Split into two commands because the `file` module sets group
  and mode atomically but POSIX sh has no single primitive; -GUESS the two-command order
  (chgrp then chmod) is immaterial since neither depends on the other.
- **Mapping awkwardness:** the `file` module would also (re)assert ownership idempotently
  and could set `owner`; the play sets only `group`+`mode`, so I emit only those two. No
  `chown`. +SURE faithful to what the play actually specifies.

### s-mail-3 — /etc/aliases & /etc/mail.rc lineinfile(regexp, create:true) → reuse `set_conf`
- **Play construct:** `lineinfile create:true` with `regexp`/`line` — replace-or-append —
  on `/etc/aliases` (2 items: `^root:`, `^default:`) and `/etc/mail.rc` (`^set sendmail`).
- **My spelling:** reuse the **same `set_conf` helper defined back in section 6**
  (lines ~185–188). `/etc/aliases` pre-exists on Debian, so replace-or-append is correct
  (not overwrite); `/etc/mail.rc` may not exist, but `set_conf`'s grep-fails→append branch
  handles create-if-missing for the single line. +SURE this covers `create: true`'s
  intent for a one-line file.
- **Cross-section coupling (flagged):** ~SUSPECT relying on a function defined in section
  6 from inside section 7 is a real assembly-order dependency. It's in-persona (a lazy
  admin defines a helper once), and the orchestrator concatenates B's sections in 5/6/7
  order so it's in scope — but it couples my two sections and would break if reordered or
  if another author's interleaved section redefined `set_conf`. See tc-flags.
- **`/etc/mail.rc` create semantics caveat:** -GUESS if `/etc/mail.rc` is absent,
  `grep … "$file"` errors ("No such file"), the `if` takes the else branch, and
  `printf >> "$file"` creates it — net effect matches `create: true`. But the failed grep
  prints a stderr message and (more importantly) under `set -e` a **non-matching** grep
  returns 1 — I rely on grep-1 being consumed by the `if` condition (which it is; `set -e`
  does not trip on a condition in `if`). +SURE the `if grep` placement neutralizes
  `set -e` here; this was deliberate, not accidental.

### s-mail-4 — testmail sends real mail every run — preserved
- **Play construct:** `shell: echo "Testmail content" | mail -s "Testmail subject" {{ mail_to }}`.
- **My spelling:** identical pipeline, `echo … | mail -s … "$MAIL_TO"` (line ~190).
  +SURE: kept verbatim including that it fires on **every** run with no guard — the brief
  says "yes, it sends mail every run; keep it." Not wrapped in any change-flag (it isn't a
  handler). The only change is quoting `"$MAIL_TO"`.

---

## tc-flags — cross-cutting judgment calls I deliberately did NOT settle

- **tc-1 (assembly: double ufw restart).** Another author's ssh section reloads ufw inline;
  I also restart ufw at my section end via `ufw_changed`. Whether the assembled book
  restarts ufw twice — and whether that's acceptable or should be de-duped at assembly —
  is **not mine to decide**. I kept my end-of-section restart faithful to the play's
  handler and left coordination to the orchestrator. (KNOB-ish: cross-section
  handler-coalescing.)
- **tc-2 (helper scope coupling 6→7).** `set_conf` is defined once in section 6 and reused
  in section 7 (s-mail-3). I bet on the orchestrator preserving B's 5/6/7 order and on no
  interleaving author shadowing the name. If the assembly model is "each section must be
  self-contained / independently hoistable," this is wrong and `set_conf` should be
  duplicated into section 7 (or lifted to the preamble). Left unsettled.
- **tc-3 (idempotency philosophy, whole sections).** I rendered every `create: true`
  blockinfile as overwrite (s-uu-1, s-fw-6, s-mail-1) — faithful first-run, lossy on
  re-run — but added a re-run guard for the positional before.rules insert (s-fw-4). That
  inconsistency (overwrite-freely vs guard-the-positional-insert) is a deliberate but
  arguable line I drew on "destructiveness of a second run," not a principled global
  stance. A reviewer could reasonably push it either way (guard nothing = maximal lazy;
  guard everything = not-this-persona).
- **tc-4 (change-detection absent).** None of my `*_changed` flags reflect *actual* change
  (s-fw-2); they're set unconditionally. Faithful to a fresh-host first run, but they do
  NOT model Ansible's "only notify if changed." Whether the corpus wants real
  change-detection modeled (so the analyzer sees conditional restarts) is unsettled — I
  judged it out-of-persona for a lazy one-shot script.
- **tc-5 (byte-fidelity vs admin-plausibility on whitespace warts).** I preserved some
  upstream warts exactly (commented origins s-uu-2, trailing-space regexp s-fw-3, fail2ban
  `%(...)s` tokens) but dropped one (jail.local `ignoreip` trailing space, s-fw-7). The
  cut-line — "preserve semantic/structural warts, drop invisible-whitespace ones a human
  wouldn't retype" — is a judgment call, not settled policy. If the test goal is exact
  byte reproduction of the play's rendered output, s-fw-7 is wrong.
- **tc-6 (`hostname -s` vs `hostname`).** Picked `-s` to match `ansible_hostname`'s short
  form (s-fw-3) over the laziest `$(hostname)`. If the corpus wants "what a careless admin
  literally types," `$(hostname)` (FQDN-capable) might be the more in-character — and
  more bug-bait — choice. Left as-is, flagged.
