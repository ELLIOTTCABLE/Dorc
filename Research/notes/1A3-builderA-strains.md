# 1A3 — builder-A strain note (requirements / packages / ssh / password-quality)

**Disclosure (read first):** This note AND the `section-A.sh` fragment it describes are
LLM-generated. They are one slice of an intentionally quality-varied *artificial testing
corpus* built to exercise a static-analysis tool — they are NOT real security code, were
never run, and are not deployed anywhere. Nothing here is reviewed, battle-tested ops.
Crucially, an artificial corpus like this *cannot expose the truth of real-world ops-code*:
real admin shell carries accidents, local knowledge, and history that a from-scratch
transcription of four Ansible roles structurally can't reproduce. Treat confidence marks
below as marking my reasoning about the *mapping exercise*, not about field reality.

Scope: I rendered four Ansible roles (`requirements`, `packages`, `ssh`,
`password-quality`) into POSIX-sh sections 1–4. The Ansible play was ground truth for
semantics; my Debian-admin persona governed only spelling. Line references are into
`section-A.sh`.

---

## Section 1 — requirements

### s-req-1 — sha512 password hash (`user.password | password_hash('sha512')`)
- Play construct → sh: Ansible hashes the password controller-side and hands the crypt
  string to `useradd -p`. I spelled it `useradd ... -p "$(openssl passwd -6 "$USER_PW")"`
  (fragment ~L24-29). `openssl passwd -6` emits a `$6$` sha512-crypt hash, which is exactly
  what `password_hash('sha512')` produces.
- Rejected: `mkpasswd -m sha-512` (from the `whois` package) — equally idiomatic but adds a
  package dependency not guaranteed present; `openssl` is effectively always there. Rejected
  passing the plaintext to `chpasswd` (no `-e`) as a separate step — two-step, and stores via
  a different path than the play's single hashed `-p`.
- +SURE the hash *format* matches. ~SUSPECT a nit: `useradd -p` writes the hash literally,
  so if `$USER_PW` produced a hash containing characters meaningful to the surrounding
  command it could bite — but command-substitution output is a single argument here, so
  it's fine. --WONDER whether a real admin would instead `useradd` then `chpasswd` for
  readability; I chose the single-call form to stay closest to the play's single field.

### s-req-2 — user creation home dir / group-replace semantics
- The Ansible `user` module defaults `create_home: yes` on Debian, so I added `-m`. Its
  `groups:` with no `append:` means *set supplementary groups exactly* — `useradd -G a,b,c`
  on a fresh account does the same. +SURE for the new-user case. ~SUSPECT divergence only if
  the user already exists (my `getent passwd` guard skips entirely then, whereas Ansible
  would reconcile groups/shell). I guarded creation because re-running `useradd` on an
  existing user errors; the play is naturally idempotent and mine should be too. Flagged
  because "skip if exists" is *not* identical to "converge if exists."

### s-req-3 — `dpkg-statoverride` already-exists tolerance
- Play construct → sh: the play runs the statoverride unconditionally and tolerates failure
  *only* when stderr contains `exist` (`failed_when: rc != 0 AND "exist" not in stderr`,
  with the upstream author's own comment admitting they were unsure how to do this in other
  languages). I mirrored exactly: capture combined output, and on non-zero exit re-raise
  unless the message matches `*exist*` (fragment ~L48-54).
- Rejected (persona tension, important): my Debian-admin instinct is to *check before add* —
  `dpkg-statoverride --list /bin/su >/dev/null || dpkg-statoverride --add ...`. I did NOT do
  that, because the play does not; the faithfulness rule (play governs semantics) overrides
  my persona here. A pre-check would also subtly change behavior (it tolerates *any* prior
  override on `/bin/su`, not just the exact one). Preserved the run-then-tolerate wart.
- ~SUSPECT the `case "$su_err" in *exist*)` is a faithful rendering of Python's
  `"exist" not in stderr` substring test. -GUESS on exact message text across
  dpkg versions ("already exists" vs other phrasings) — both contain `exist`, so the glob
  holds.

### s-req-4 — `%sudousers` sudoers edit is UNVALIDATED (preserved wart)
- This is the strain I'm least comfortable leaving as-is. The play edits `/etc/sudoers`
  for the `%sudousers` line via a plain `lineinfile` with **no `validate:`** — unlike the
  passwordless edit (s-req-5) which *does* validate. So per the faithfulness rule I edit
  `/etc/sudoers` in place with `sed`/append and NO `visudo -cf` (fragment ~L40-46). My
  persona (defensiveness ~0.45) would *always* gate a sudoers write behind `visudo -cf`;
  an unvalidated `sed -i` on `/etc/sudoers` is exactly the footgun I guard against (a bad
  edit can lock out all privilege escalation). I deliberately left it ungated to match the
  play, and I am flagging it loudly. +SURE this is a real upstream inconsistency, not my
  invention (validate present on one sudoers edit, absent on the other).

### s-req-5 — passwordless-sudo edit (`validate: visudo -cf`, `mode 0440`, `create: yes`)
- Play construct → sh: I build a candidate (`mktemp` copy of `/etc/sudoers`,
  replace-or-append the `NOPASSWD` line), run `visudo -cf "$candidate"`, then
  `install -m 0440 "$candidate" /etc/sudoers` (fragment ~L58-72). That renders all three
  Ansible knobs: `validate` → the `visudo -cf` on the temp, `mode 0440` → `install -m 0440`,
  `create: yes` → moot since `/etc/sudoers` exists (the copy preserves it).
- Mapping awkwardness: Ansible's `validate` runs the validator against a temp rendering of
  the *proposed* file and only commits on success; my temp-file-then-install is the honest
  sh equivalent, but it is materially more code than the play's one task. The
  `lineinfile ... create: yes mode: 0440` is doing several things in one module call that
  do not collapse into one sh statement.
- Separately flagged: writing a per-user `NOPASSWD` line into `/etc/sudoers` is a *play
  addition with no guide rationale* (the guide only documents the `%sudousers` group line).
  I rendered it because the play has it, but it is not traceable to the README sections I
  was given. ~SUSPECT this is intentional convenience for the bootstrap user.

### s-req-6 — authorized_key install (controller file → `$SSH_PUBKEY` text)
- Play construct → sh: the `authorized_key` module reads a controller-local pubkey and
  idempotently appends it, managing `.ssh` (0700) and `authorized_keys` (0600) ownership.
  I resolved the home dir with `getent passwd "$USER_NAME" | cut -d: -f6`, `install -d`
  the `.ssh` dir, append-if-absent the key, then fix perms/owner (fragment ~L74-83).
- Rejected: `~"$USER_NAME"/.ssh` — POSIX sh does **not** expand a tilde followed by a
  variable (`~$USER_NAME` stays literal in dash), so that would silently write to a path
  named `~USERNAME`. `getent` is the correct defensive resolution. +SURE on the
  tilde-non-expansion (this is a classic dash gotcha and a genuine sh-vs-Ansible mapping
  trap: the module "just knows" the home dir; sh must go get it).
- ~SUSPECT my `grep -qF "$SSH_PUBKEY"` idempotency check is coarser than the module's
  (it matches the key as a substring anywhere in the file rather than as a whole managed
  entry), but for a single-key bootstrap it is equivalent in practice.

---

## Section 2 — packages

### s-pkg-1 — one-shot install + `upgrade: yes`
- Straightforward. `apt-get -y dist-upgrade` for `upgrade: yes` (Ansible's `yes` maps to a
  full dist-upgrade, not `upgrade: safe`), and a single `apt-get install -y` with the ~20
  packages kept as one command per the brief and the play. -GUESS on `dist-upgrade` vs
  plain `upgrade`: Ansible `apt: upgrade: yes` historically means `dist`/`full` upgrade;
  I went with `dist-upgrade`. If the corpus wants the most conservative reading this is a
  candidate to revisit (see tc-flags). No guide section exists for this role; header says so.

---

## Section 3 — ssh

### s-ssh-1 — the early inline ufw restart (`meta: flush_handlers`)
- Play construct → sh: two `ufw limit` rules (ports 22 and `$SSH_PORT`) followed by
  `meta: flush_handlers`, which here forces the notified "restart ufw" handler to run
  *immediately, inline*, instead of at the end. I rendered the two `ufw limit in` rules
  then an inline `ufw reload` right there (fragment ~L104-112). This is the one place the
  notify/handler pattern is NOT deferred — I left a comment marking it deliberate.
- Mapping awkwardness: `flush_handlers` is a control-flow construct with no sh analogue;
  "run the queued restart now" just becomes "put the restart command here." +SURE the
  ordering is preserved. ~SUSPECT `ufw reload` vs `ufw --force enable`/`systemctl restart
  ufw` — the handler name says "restart ufw service" but ufw is *not yet enabled* at this
  point (a later author's firewall section enables it), so a literal service restart is
  near-meaningless and `ufw reload` is the closest honest rendering. Flagged in tc-flags:
  the exact ufw verb is a judgment call I'm not fully settling.

### s-ssh-2 — `blockinfile` → marked, change-detected block (BIGGEST mapping awkwardness)
- Play construct → sh: `blockinfile` does replace-or-insert of a multi-line block between
  auto-managed markers, and only notifies on change. POSIX sh has nothing like it. I spelled
  (fragment ~L118-186): pick my own marker comments (a transcribing admin would NOT write
  "ANSIBLE MANAGED BLOCK"), build a desired file = (current with any old marked region
  `sed`-deleted) + fresh block, extract the current marked region, and `cmp -s` it against
  the freshly-rendered block to decide whether to install + set `sshd_changed=1`.
- This is the **primary mapping pain** of the whole section. A single declarative module
  becomes ~25 lines of `mktemp` + range-delete `sed` + heredoc + `cmp`. Several sub-strains:
  - the markers must be regex-safe in `sed` addresses (mine use `>>>`/`<<<`/`#`, all
    sed-literal); ~SUSPECT fine but a `.`-bearing marker would have bitten.
  - change-detection is hand-rolled to mirror "notify only on change"; an admin who didn't
    care about idempotent restarts would just rewrite unconditionally and always restart.
    I chose the change-detecting form because the play's notify *is* change-gated. -GUESS
    this is the spelling most analyzers will find hardest to follow (control flow + temp
    files + heredoc + cmp), which I assume is partly the point of including it.
  - the block body is a literal-heredoc (`<<'EOF'`) so the `@`/`,` in cipher lists and the
    `#` comments pass through unexpanded. +SURE that's correct; a non-quoted heredoc would
    try to expand nothing problematic here but quoting is the safe idiom.
- Content fidelity: I transcribed the **play's** block body, which differs from the guide's
  prose block — e.g. the play's block has `Protocol 2` and does NOT contain `Subsystem`,
  `X11Forwarding`, `PermitRootLogin`, `HashKnownHosts`, or `RequiredRSASize`, whereas the
  guide's documented block does. The play moved several of those into the lineinfile loop.
  I followed the play. +SURE this guide-vs-play divergence is real and intentional on the
  play author's part.

### s-ssh-3 — the 13-item lineinfile loop → `set_sshd_line` helper (function used)
- Play construct → sh: a `loop:` of `{regexp, line}` doing replace-or-append per item. I
  wrote a helper function `set_sshd_line line_re line_val` (fragment ~L88-102) and called it
  12 times, with the Port item broken out separately (see s-ssh-4). This is the one place I
  judged a function genuinely idiomatic — 12 near-identical edits inline would be far worse
  to read. The brief explicitly permits a helper here; flagging per instruction.
- Spelling: `grep -qxF "$line_val"` first (exact line already present → no-op, mirrors
  Ansible idempotency + change-gated notify), else `grep -q "$line_re"` →
  `sed -i "s|$line_re.*|$line_val|"`, else append. `sshd_changed=1` only on real change.
- Mapping awkwardness / nits:
  - Ansible `lineinfile` operates on the *last* matching line; my `sed s|...|` replaces
    *every* matching line. ~SUSPECT benign for these anchored single-occurrence keys, but
    it is a real semantic difference if a key appeared twice. Flagged.
  - the regexps are passed to BOTH `grep` (BRE) and `sed` (BRE) — fine because they are all
    simple `^Keyword` anchors with no metacharacters. -GUESS this holds only because the
    play's regexps happen to be metacharacter-free (the Port one is the exception, handled
    separately precisely because it is *not*).
  - using `|` as the `sed` delimiter avoids clashes with the `/` in
    `Subsystem sftp internal-sftp -f AUTHPRIV` etc. +SURE that was necessary.

### s-ssh-4 — `^Port (?!22$)` PCRE negative-lookahead (trickiest single item)
- Play construct → sh: the regexp uses a Python/PCRE negative lookahead — "a Port line whose
  value is not exactly 22." The play's own comment states the intent: *if `Port 22` is
  present exactly, leave it (new-machine safety); otherwise ensure `Port $SSH_PORT`.* POSIX
  BRE/ERE have no lookahead, so I spelled the branch explicitly (fragment ~L189-199): if an
  exact `Port 22` line exists → no-op; elif any `^Port ` line exists → replace with
  `Port $SSH_PORT` + mark changed; else append `Port $SSH_PORT` + mark changed.
- **Important divergence I deliberately chose** (per the brief, spelling the *comment's
  intent*, not Ansible's literal behavior): literal `lineinfile regexp:'^Port (?!22$)'` on a
  file containing only `Port 22` would find *no match* (the lookahead excludes it) and
  therefore *append* `Port $SSH_PORT` anyway — leaving BOTH `Port 22` and `Port $SSH_PORT`
  in the file. That contradicts the comment's "leave it." The brief told me to spell the
  comment's intent, so my "Port 22 present → do nothing, no append" branch *diverges from
  the literal Ansible execution*. This is the single most consequential semantic judgment in
  the fragment. +SURE about the literal-Ansible behavior (lookahead → no match → insert).
  ~SUSPECT the comment's intent (which I implemented) is what the author actually wanted and
  the literal behavior is itself an upstream bug. Surfacing both so the corpus owner can pick.
- Also note: the no-op branch correctly does NOT set `sshd_changed` (leaving Port 22 is not
  a mutation), so it cannot trigger a restart on its own. +SURE that is the faithful reading
  of change-gated notify.

### s-ssh-5 — DH moduli shorten (`shell` + `creates:` + `changed_when 'differ:'`)
- Play construct → sh: already shell. `creates: /etc/ssh/moduli.short` → `[ ! -f
  /etc/ssh/moduli.short ]` guard; pipeline (`cp`; `awk '$5 >= 3071' | tee moduli.tmp`;
  `if ! cmp moduli moduli.tmp; then mv ...`) preserved verbatim; `changed_when` keyed on
  `'differ:'` in stdout → I set `sshd_changed=1` *inside* the `if ! cmp` true-branch, i.e.
  only when `cmp` reports a difference (fragment ~L201-214). That mirrors the change-gated
  restart precisely.
- ~SUSPECT one faithful-but-odd detail preserved: the guard file `moduli.short` is created
  by the *first line of the guarded block itself* (`cp moduli moduli.short`), so the block is
  self-disabling on re-run — a slightly unusual `creates:` choice, but it is the play's, so I
  kept it. The `tee` writes `moduli.tmp` to stdout too (noise on a real run); preserved
  because the play does. +SURE the pipeline semantics match.

### s-ssh-6 — notify/handler → single guarded restart at section end
- The handler `restart ssh service` (`service: name=ssh state=restarted`) fires once if any
  notifying task changed. I init `sshd_changed=0` at the top of the section and gate one
  `service ssh restart` at the end (fragment ~L216-218). Each mutating edit sets the flag;
  the ufw flush (s-ssh-1) is the documented inline exception. +SURE this is the faithful
  render of Ansible's collapse-notifies-into-one-handler-run.
- Mapping note: Ansible would run the handler exactly once regardless of how many tasks
  notified; my single end-of-section `if` gives the same once-only behavior. The semantic
  the flag encodes ("did *anything* change") is what drives it.

---

## Section 4 — password-quality

### s-pam-1 — `gecoschec` upstream typo (preserved verbatim)
- Play construct → sh: a single `lineinfile` on `/etc/pam.d/common-password`,
  `regexp: '^.*pam_pwquality.so.*$'`, replacing with the long pwquality options line. I
  spelled replace-or-append keyed on the same regex (fragment ~L226-236).
- **Preserved wart:** the `line:` value ends with `gecoschec`, which is a typo for the real
  pam_pwquality option `gecoscheck` (reject passwords containing the GECOS/account name).
  As written, `gecoschec` is not a recognized option and pam_pwquality would ignore it (or
  warn), so this hardening control silently does nothing. The guide prose at README L1288
  even *documents* it as `gecoschec` → "do not allow passwords with the account's name," so
  the typo is upstream in both the guide and the play. Preserved verbatim per the
  faithfulness rule; flagged here. +SURE it is a typo (correct option is `gecoscheck`).
- Mapping nit: the play's regexp `^.*pam_pwquality.so.*$` is broad (matches the line
  anywhere `pam_pwquality.so` appears). My `sed "s|^.*pam_pwquality.so.*$|...|"` replaces the
  whole matched line, which is faithful. ~SUSPECT if two such lines existed my `sed` rewrites
  both vs Ansible's last-match — same caveat as s-ssh-3.

---

## tc-flags — cross-cutting judgment calls I did NOT settle

- **tc-1 (ufw verb):** `ufw reload` vs a literal service restart vs `ufw --force enable` for
  the early `flush_handlers` (s-ssh-1). I picked `reload` because ufw is not yet enabled, but
  the handler is *named* "restart ufw service." The assembler / a later firewall-section
  author should confirm which verb they want, since this crosses an author boundary.
- **tc-2 (sed replaces all vs Ansible last-match):** across `set_sshd_line` (s-ssh-3), the
  Port branch (s-ssh-4), the `%sudousers`/`NOPASSWD` sudoers edits (s-req-4/5), and pam
  (s-pam-1), my `sed s|...|` rewrites *every* matching line whereas Ansible `lineinfile`
  edits only the last. Benign for these single-occurrence anchored keys; would diverge on
  duplicates. Not resolved — would need a "replace last only" idiom (`tac`/`sed` tricks) that
  no plain admin would actually write. Left as the natural sh spelling.
- **tc-3 (Port-22 comment-intent vs literal Ansible):** s-ssh-4. I implemented the comment's
  intent (leave Port 22, add nothing) which *diverges from literal lineinfile* (which would
  append `Port $SSH_PORT` alongside `Port 22`). The brief directed the comment's intent;
  flagging that the literal play execution differs, in case the corpus wants literal-fidelity
  over intent-fidelity.
- **tc-4 (`apt upgrade: yes` → dist-upgrade vs upgrade):** s-pkg-1. Chose `dist-upgrade`.
  Not certain the corpus wants the aggressive reading.
- **tc-5 (skip-if-exists vs converge):** s-req-2 (user) and the group loop guard. I guard
  creation to stay idempotent/non-erroring, which means I do NOT reconcile an *already
  existing* user's groups/shell the way a fresh Ansible converge would. Acceptable for a
  bootstrap runbook, but it is a behavioral narrowing vs the declarative module.
- **tc-6 (change-detection fidelity):** I hand-rolled change-gating (block `cmp`, line
  `grep -qxF`, moduli `cmp`, Port branches) to mirror Ansible's notify-only-on-change. A
  lazier-but-still-realistic admin would mutate unconditionally and restart every run. If the
  corpus prefers the simpler always-restart spelling for some sections, that is a different
  realism point on the quality axis — flagging that I chose the more-careful end.
