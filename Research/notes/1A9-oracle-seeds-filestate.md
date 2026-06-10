# 1A9 — oracle seeds: the file-state spectrum (Builder-F)

**Disclosure (read first):** This note AND the five `*.oracle.sh` seeds it
describes are LLM-generated, one slice of an intentionally quality-varied
*artificial testing corpus* built to exercise a static-analysis tool (Dorc).
They are NOT real ops code, were NEVER run (the probe bodies name real
read-only commands — `grep`, `sshd -T`, `cmp`, `test` — but are frozen
evidence, validated by `dash -n` and reading only), and are deployed nowhere.
An artificial corpus like this *cannot expose the truth of real-world ops-code*:
real admin shell and real oracle-authoring carry accidents and local knowledge
a from-scratch exercise structurally can't reproduce. Confidence marks below
mark my reasoning about the *modeling exercise*, not field reality.

Builder-F persona: pragmatic-middle engineer (defensiveness ~0.6). The seeds
work for the shapes in front of them and DOCUMENT the corners they don't,
rather than over-fitting refusals or faking coverage. Charter was EXPLORATION:
how does file-manipulation state fall out of oracle structure? The findings
below outrank the code.

Files (all in `Research/corpora/H2SaLS/oracles/`, all `dash -n` clean):
- `sshdconf.oracle.sh` — per-surface (semantic, `sshd -T`)
- `confline.oracle.sh` — generic line-in-file (textual, `grep`, path×line)
- `confblock.oracle.sh` — whole-file managed block (content, `cmp`)
- `crond.oracle.sh` — cron.d drop-in (the "job vs file" identity test)
- `fetched.oracle.sh` — wget-fetched file (presence-yes, currency-no)

---

## The spectrum verdict (what I learned BUILDING it)

The question: file-edits have NO single provider command — the same `sed -i`
touches anything — so how does file state fall out of oracle structure?
Per-SURFACE kinds vs a generic file kind vs provider-keyed effects. Building
all three ends of the spectrum produced one dominant structural finding and a
clean boundary.

### concl-1: provider-keyed EFFECTS for file mutation are structurally impossible (+SURE for this corpus)

This is the sharpest thing I learned, and it held at EVERY point on the
spectrum. The oracle dialect keys effects on `(provider-command, verb)`:
`oracle_effect apt-get install establish installed`. File mutation has no such
token to key on, for two independent reasons that compound:

- **f-1: the mutating "verb" is shell SYNTAX, not a command.** The book's
  dominant write is `printf '%s\n' "$line" >> "$file"` (append, 11 sites) and
  `cat > file <<EOF` (overwrite, 8 sites). `printf`/`cat` write to STDOUT; the
  FILE mutation is the `>>` / `>` REDIRECT. A redirect is not a command token —
  there is nothing to put in the `<provider>` slot of an `oracle_effect` line.
  You cannot write `oracle_effect '>>' establish ...`. (+SURE — traced against
  census `append >>`=11, `redirect >`=14.)
- **f-2: the actual mutator command is GENERIC.** `sed -i` (11 sites) IS a real
  command token, but one `sed -i "s|$re.*|$line|"` edits ANY file via ANY
  script. To bind it to a specific surface/path/line, the resolver would have to
  PARSE the `s|...|...|` argstring — which the analyzer must NEVER do
  (`check()` annotates operands by position; it does not interpret argstrings;
  exemplars + `inv-referent-agnostic`). So even the real command can't be
  surface-keyed. (+SURE.)

Consequence: **every file-state kind I built declares ONLY a read-side `query`
effect** (`grep`/`sshd -T`/`cmp`/`test`). None can declare a sound `establish`
for the write. The "did the edit converge?" question is answerable only by
PROBING (read the resulting state), never by attributing an establish-effect to
the editing command. ~SUSPECT this is the deep reason `rul-1A-filestate` says
file-state must fall out of *probe/fd modeling*, not out of generic
mutation-effect seeds: there is no mutation-effect to seed.

### concl-2: per-surface vs generic is a SEMANTIC-LIFT tradeoff, and the boundary is "does a value-query command exist?" (~SUSPECT)

Both ends READ via a query command; they differ in what the query MEANS:

- **Generic (`confline`)**: probe is `grep -qxF <line> <path>` — textual. Answers
  "is this literal line in this file?" Cheap, universal, but blind to semantics:
  it can't see a value set via a default, an `Include`, or a differently-spelled-
  but-equivalent line. Identity is (path × line) — the TWO-part problem (concl-3).
- **Per-surface (`sshdconf`)**: probe is `sshd -T | grep -qx 'permitrootlogin no'`
  — semantic. Answers "is rootlogin EFFECTIVELY no?", seeing merges/defaults/
  Match-blocks the textual probe misses. The entire payoff is that `sshd -T`
  exists as a value-query for this surface.

**The boundary I'd draw:** a per-surface kind is worth it exactly when the
surface has a real effective-value query (`sshd -T`, `sysctl -n`, `ufw status`,
`getent`). When it doesn't, you fall back to generic textual `confline`/
`confblock`. The corpus's file targets split cleanly: sshd_config HAS `sshd -T`
(→ per-surface); psad.conf / rkhunter.conf / pam.d / fail2ban jail.local / msmtprc
/ cron.d / sources.list.d do NOT have a value-query (→ generic line/block). So
the spectrum is not a design preference — it's dictated by whether the surface
ships an introspection command. (~SUSPECT this generalizes; -GUESS there exist
surfaces with a query I'd not know about, shrinking the generic bucket.)

### concl-3: identity arity differs across the spectrum, and the generic end has a genuine annotation problem (+SURE)

The annotation idiom `name : kind = "$1"` binds exactly ONE operand as the
KindId. The spectrum's three ends need different arities:

- `confblock` / `crond` / `fetched`: entity = **path** (one operand). Clean —
  no problem. Annotate the path, done.
- `sshdconf`: entity = **the surface** (a singleton — "this host's sshd"). The
  common `sshd -T` form has NO operand for it (it reads the implicit local
  config). I bind the implicit `/etc/ssh/sshd_config`, or the explicit `-f`
  path when present. Slightly awkward (an implicit-singleton annotation) but
  honest.
- `confline`: entity = **(path × line)** — genuinely TWO-part. The decision
  (`confline.oracle.sh` header): annotate the **PATH** as the KindId (it's the
  durable, cross-oracle-shareable referent — another oracle can name the same
  `/etc/ssh/sshd_config`), and carry the **line/pattern** as a per-site DETAIL
  the resolver binds into the probe argv (`$2`) but does NOT mint a KindId for.
  The probe `oracle_probe_confline()` reads `"$2"` (line) against `"$1"` (path).
  This is a real departure from every exemplar (all of which have one-operand
  entities) and the single most interesting structural strain. ~SUSPECT it's
  the right call, but it means confline:`<path>` is a COARSER identity than the
  edit's true granularity — two different lines in the same file share a
  KindId, and the engine must keep them apart by the carried detail, not the
  key. -GUESS this pushes complexity into the engine's site-keying
  (`inv-site-keyed-results` already keeps sites distinct, so maybe it's fine).

### concl-4: convergence claims on file-state are FILE claims, not RUNNING claims — the restart gap (+SURE, headline)

`sshdconf` forced this out. `sshd -T` re-PARSES the file; it does NOT introspect
the running daemon (sshd has no live-config query). The book edits the file
(L262…) then defers `service ssh restart` to an end-of-run change-flag (L685).
Between edit and restart, the file says the new value but the daemon serves the
old. So a probe — textual `grep` OR semantic `sshd -T`, both read the FILE —
reports CONVERGED the instant the file is edited, BEFORE the restart that makes
it true. **Any apply-elision built on a file-state probe silently assumes the
restart already ran.** The host-truth a probe cannot see is "what the live
process loaded." This is not fixable at the oracle layer (no daemon-introspect
query exists); it's a property of the surface, and the right move is to DOCUMENT
that sshdconf convergence is a file-parse claim (which I did, in-file). The
run-delta / change-flag side is wholly un-host-probeable (um-file-restart-1).

### concl-5: "converge-by-construction" inverts the useful probe to the PRE-write check, which needs un-knowable bytes (+SURE)

`confblock` forced this. `cat > file <<EOF` overwrites unconditionally → the
file IS the bytes afterward, definitionally → the post-write probe is trivially
"yes". The USEFUL probe is the PRE-write one ("does the file ALREADY equal the
intended bytes?", licensing elision of the overwrite). That needs the oracle to
KNOW the intended bytes — and it usually can't, because the heredocs are
UNQUOTED (census: 8 unquoted vs 2 quoted) and interpolate runtime `$VAR`s
(`destemail = $MAIL_TO`, `password $MAIL_PW`) the oracle can't resolve
statically. Content-equality is honest ONLY for quoted-heredoc / no-`$VAR`
blocks (the unattended-upgrades file L340 `<<'EOF'`, cisofy.list L656). For the
rest, the intended image exists only after the controller expands it at run
time. (This is the same un-knowable-bytes wall `crond` hits on its `MAILTO=$MAIL_TO`
body, L554.)

---

## Per-seed detail

### sshdconf (per-surface showcase)
- **Models:** harden.sh §3 — the `set_sshd_line` loop options (L262-273), Port
  (L281-294), the mozilla block options (L197-236). Probe `sshd -T` (semantic,
  effective config), kind-default `sshd -t` (parses-at-all). Per-option
  selectors for the 11 options the book fixes to a clean scalar
  (permitrootlogin, passwordauthentication, x11forwarding, permitemptypasswords,
  maxauthtries, logingracetime, clientaliveinterval, clientalivecountmax,
  maxsessions, allowtcpforwarding, port).
- **Refuses / records:** Subsystem (L273 — value embeds `-f AUTHPRIV -l INFO`,
  version-brittle, not a clean scalar); Protocol 2 (L219 — removed in modern
  OpenSSH, `sshd -T` never emits it → a converged-value probe would assert a
  never-present key); HostKey (L201-203 — multi-valued, set-equality not
  scalar). Many other settable options (AllowGroups, ListenAddress, MaxStartups,
  PermitUserEnvironment, …) are probeable in the SAME shape but OMITTED to keep
  the seed a focused honest subset — the finding is the shape, not the row count.
- **Alternatives rejected:** (a) `grep` on the file instead of `sshd -T` —
  rejected because it misses effective values from defaults/Match/Include
  (the whole per-surface point). (b) declaring an `establish` on the
  setting-side — impossible (concl-1; the writes are generic `sed`/`printf >>`).
- **Effect declared:** `oracle_effect sshd '' query effective` (read-only). NO
  establish (concl-1, concl-4).

### confline (generic line-in-file)
- **Models:** the `set_sshd_line` (L166-178) and `set_conf` (L427-437)
  grep||append and grep→sed-replace idioms — the corpus's single most common
  file-edit shape (`grep`=14, `sed`=11, `printf`=15). Two selectors: `confline`
  (`grep -qxF`, exact line) and `confline_pattern` (`grep -q`/`-Eq`, by pattern);
  DISTINCT questions (a `^Port ` pattern matches multiple lines), neither
  discharges the other — same per-selector logic as the `service` exemplar.
- **Identity decision:** (path × line) → annotate PATH as KindId, line as a
  carried `$2` detail (concl-3). Documented in-file.
- **Refuses / records:** multi-file / multi-pattern grep (`grep -rl pat d1 d2`),
  stdin grep (`grep pat` in a pipe) → bind nothing, site runs (sound). The
  `grep -Eq` ERE-dialect flag is STRIPPED by the resolver and the probe re-runs
  BRE `grep -q` — a fidelity gap (an ERE-only pattern could mis-evaluate);
  benign for this corpus (anchored literals) but recorded as strain-confline-E.
- **THREE-OUTCOME HAZARD (um-file-2):** `grep` rc conflates 1=no-match with
  2=tool-failure; the two-outcome probe wrapper can't tell "line absent" from
  "couldn't look".
- **Effect declared:** `oracle_effect grep '' query present` only. The
  establish/kill side is UN-KEYABLE (concl-1) — printf/`>>` has no token, `sed`
  is generic. This is the seed where the provider-keyed-effect impossibility is
  most visible.

### confblock (whole-file managed block)
- **Models:** the `cat > file <<EOF` overwrites (51myunattended L340, jail.local
  L475, ssh.local L494, msmtprc L516, cisofy.list L656) + the cmp-gated mozilla
  block (L239-259). Identity = path alone (no arity problem). Probes: `[ -s ]`
  (existence floor, valid for every target) + `confblock_content` (`cmp -s`
  byte-identity, honest only for static-bytes blocks).
- **Alternatives rejected:** a hash probe (`sha256sum`) instead of `cmp` —
  equivalent, `cmp` is the book's own idiom (L250, L303) so it's the faithful
  spelling. Declaring content-convergence for unquoted-heredoc blocks — rejected
  (un-knowable bytes, concl-5).
- **Refuses / records:** `cmp ... -` (stdin) and process-substitution forms (no
  stable path to key). Same three-outcome `cmp` rc hazard (um-file-2).
- **Effect declared:** `oracle_effect cmp '' query content` only (concl-1).

### crond (cron.d drop-in — the identity test)
- **Models:** harden.sh L553-557 (`cat > /etc/cron.d/clamav-daily` + chmod).
- **THE IDENTITY FINDING (um-cron-1):** a cron.d drop-in is JUST A FILE. The
  "entity = cron JOB (schedule×command×user)" abstraction does NOT survive: cron
  has no query for effective cron.d entries (`/etc/cron.d/*` is directory-scanned,
  no `crontab -l` listing, running crond's set un-queryable). So "job" collapses
  to "file" and crond is structurally a thin alias of confblock (path-keyed,
  presence-probed). CONTRAST: a USER crontab WOULD be job-keyed (`crontab -l -u`
  is a real per-user query) — a different kind. The lesson is that the job
  abstraction is an illusion *for the drop-in form the book uses*.
- **Effect declared:** `oracle_effect test '' query present` (presence floor;
  body has runtime `$VAR` L554 so content is un-knowable, concl-5).

### fetched (wget-fetched file)
- **Models:** L627 (`wget -P` → audit.rules), `get_url` L642-647 (`wget -O` →
  cisofy key, called L646).
- **THE CURRENCY FINDING (um-fetch-1):** presence is trivially probeable
  (`[ -e ]`); content/version is NOT. The book re-fetches every run with NO
  version pin, NO checksum, NO etag — the local file carries no identity tying
  it to a known-good upstream. To claim "converged" (up-to-date) honestly, the
  oracle would need a content identity the book never establishes (pinned
  sha256, upstream version). Strongest honest claim = PRESENCE, which explicitly
  does NOT mean current (a stale/truncated prior download is "present"). A
  `fetched_pinned` selector (`sha256sum -c`) is present in the seed to DOCUMENT
  the missing identity, unused because the book pins nothing.
- **Effect declared:** `oracle_effect test '' query present` only. I
  deliberately did NOT declare `wget '' establish present`: present!=current, so
  an establish would invite unsound currency-elision on the unguarded re-fetch.
  (Sound for the `creates:`-guarded gpg case, unsound for the unguarded
  re-fetch; the book uses both, so declare neither — concl-1 corollary.)

---

## Un-modelables catalogue (the corpus tail)

Each: what it is, why it resists, and the SHAPE of what would be needed.

- **um-file-1 (provider-keyed file establish):** the headline. File mutation has
  no provider token to key an `establish` on (concl-1: `>>`/`>` are syntax;
  `sed`/`cat` are generic). What would be needed: NOT an oracle feature — a
  recognition that file-state convergence is probe-sourced (read the result),
  never effect-attributed. There is no establish to declare. (+SURE)

- **um-file-2 (three-outcome probe rc):** `grep`/`cmp`/`test` rc conflates
  "negative result" (no-match / differ / absent) with "tool failure" (unreadable
  file, bad regex, missing operand) — two outcomes where the probe needs three
  (converged / not-converged / couldn't-determine). The two-outcome wrapper
  silently maps "couldn't look" to "not converged", which under apply means
  "act" (kFAIL-perform — safe direction, but it defeats elision spuriously on a
  transient read error). What would be needed: a probe protocol distinguishing
  rc>1 / stderr-nonempty as a third "indeterminate" channel, separate from the
  boolean convergence. The one-Observable `{Effect,Status,Stdout,Stderr}` tuple
  COULD carry it (Stderr nonempty ⇒ indeterminate) but the current grep-rc
  wrapper throws it away with `2>/dev/null`. (+SURE this is real; ~SUSPECT the
  Observable model already has the slot.)

- **um-file-restart-1 (run-delta change-flag → deferred restart):** the
  `sshd_changed=0 … =1 … if [ "$sshd_changed" -eq 1 ]; then service ssh restart`
  pattern (L160/L177/L685, and the four firewall flags L404-406/L685-696). This
  is RUN-DELTA state: "did THIS run change anything?" It is NOT host-probeable at
  all — no probe can see "was this file edited during this particular execution".
  The only sound signal is the AUTHOR'S OWN FLAG DATAFLOW: trace `sshd_changed`
  from its `=1` assignments (inside each real-change branch) to the `if` guard.
  What would be needed: the engine's value-flow following the flag variable
  (which `19H` builds for constants) extended to a boolean "tainted-changed"
  lattice, so the restart's necessity is derived from whether any upstream
  mutating branch was taken. This is the ONE place the convergence signal lives
  in the book's control-flow, not on the host. (+SURE it's not host-probeable;
  ~SUSPECT the flag-dataflow is the intended sound path.)

- **um-mail-1 (testmail / report mail — pure action, no state):** `echo ... |
  mail -s ... "$MAIL_TO"` (L541) and `mail -A report ...` (L673). Sends an email
  — a pure ACTION with NO persistent host state to probe. There is nothing to
  converge; it must run every time (MustRun). No oracle can vouch it; cheap
  engine-wise (just never elide) but a hard coverage ceiling. What would be
  needed: nothing — it is correctly un-elidable. (+SURE)

- **um-rc-rkhunter-1 (`rkhunter --update` exits 2 on SUCCESS-with-updates):**
  L611 (`rkhunter --update || true`). rc meaning is TOOL-SPECIFIC: 0=no updates,
  1=download error, 2=updates installed (success!). Dorc holds rc opaquely
  (`inv` rc-opacity); WHICH rc means converged is oracle-DECLARED. Sketch of
  declaring it: an oracle for rkhunter would need a per-verb rc-semantics map —
  something like `oracle_rc rkhunter --update {0,2}=ok 1=fail` (NO such dialect
  primitive exists today; the exemplars only have `oracle_effect` and
  `oracle_probe_*`). What would be needed: an rc-meaning declaration primitive,
  consumed where the book reads the rc (here the `|| true` swallows it, so it's
  moot — but a book that branched on rkhunter's rc would need it). (+SURE the rc
  semantics are as described; -GUESS the primitive shape.)

- **um-validator-1 (`visudo -cf` — read-only query about a CANDIDATE file):**
  L102 (`visudo -cf "$sudoers_tmp"`). Interesting POLARITY: it's a read-only
  QUERY (validates, mutates nothing) but about a CANDIDATE file (a freshly-built
  temp), not converged live state. It asks "WOULD this file be valid?" — a
  pre-commit gate, not a convergence check. What would be needed: a "validator"
  effect class — `query` polarity, but its entity is a transient candidate, not
  a durable surface; the result gates the SUBSEQUENT `install` (L103). The
  dialect's `query` fits the polarity but not the transient-entity semantics;
  recording the polarity mismatch. (~SUSPECT a validator is a distinct effect
  shape worth a primitive.)

- **um-validator-2 (`gpg --dearmor` is `creates:`-guarded → actually
  probeable!):** L650-653. The book itself gates it: `if [ ! -e
  /usr/share/keyrings/cisofy-archive-keyring.gpg ]`. The author HANDED us the
  probe — existence of the `.gpg` IS the convergence check (`[ -e ]`). This one
  is NOT un-modelable; it's cleanly presence-probeable (fetched/crond shape).
  Recording it on the catalogue specifically to say it ESCAPES the catalogue:
  an author's own `creates:` guard is the oracle the author wrote inline. (+SURE)

- **um-pure-1 (`openssl passwd -6` / `getent passwd | cut` — pure functions,
  consumed stdout):** `useradd ... -p "$(openssl passwd -6 "$USER_PW")"` (L65-67)
  and `user_home=$(getent passwd "$USER_NAME" | cut -d: -f6)` (L109). These are
  PURE FUNCTIONS whose STDOUT is consumed into a larger command. `openssl passwd`
  is a deterministic-modulo-salt transform (no host state); `getent passwd |
  cut` READS a host fact (the home dir) into a variable. What would be needed:
  for `getent`, this is genuinely a host QUERY (passwd database) and COULD be an
  oracle (`getent '' query <db>`), with the consumed stdout flowing as a value —
  but the value is consumed-into-argv (`$user_home/.ssh/...`), so it's a
  Stdout-channel consumption, not a convergence question. `openssl passwd` is a
  pure transform with nothing to probe. Recording: the consumed-stdout idiom is
  an Observable.Stdout flow, orthogonal to file-state convergence. (~SUSPECT
  getent-as-oracle is viable; +SURE openssl is pure-no-state.)

- **um-audit-1 (`lynis audit` / `psad --sig-update` — non-idempotent network
  audits):** L671-672 (`lynis update info`, `lynis audit system | ansi2html`),
  L473 (`psad --sig-update`). Network-touching, non-idempotent, no clean
  convergence target (an audit RUN produces a report; "converged" isn't
  meaningful). Like um-mail-1, these are MustRun actions. `psad --sig-update`
  refreshes signatures (could in principle be version-probed, like fetched, but
  psad exposes no clean signature-version query). What would be needed: for the
  audits, nothing (un-elidable by nature); for sig-update, a signature-version
  query that doesn't exist. (~SUSPECT)

- **um-statoverride-1 (`dpkg-statoverride --add` with tolerated-exists rc):**
  L84-89. Runs unconditionally, tolerates ONLY the "already exists" failure
  (`case "$su_err" in *exist*`). This is rc-consumption crossed with
  stderr-matching: the book reads BOTH rc (`||`) and stderr (`*exist*`) to
  decide. What would be needed: the Observable model's Status AND Stderr
  channels both consumed, with the oracle declaring that rc!=0 + stderr~exist is
  the converged case (a stat-override already present). Genuinely probeable in
  principle (`dpkg-statoverride --list /bin/su` IS a query) — the book just
  doesn't probe, it acts-and-tolerates. Recording as a Status+Stderr
  co-consumption case. (~SUSPECT the --list query makes it a real oracle.)

---

## tc-flags — cross-cutting judgment calls I did NOT settle (flag-up, not relitigate)

- **tc-F1 (confline path-as-KindId):** I chose PATH as the confline KindId with
  line-as-carried-detail (concl-3). This makes confline:`<path>` coarser than
  the edit's true (path×line) granularity; the engine's site-keying must keep
  same-file different-line sites apart. ALTERNATIVE not taken: a synthetic
  composite KindId (path+line hash) — rejected because it's not a durable
  cross-oracle referent and smells like engine-side argstring parsing. Wants an
  orchestrator/human ruling on whether coarse-path-identity + site-keying is the
  intended model, or whether file-line entities need a real two-part KindId
  primitive the dialect lacks.

- **tc-F2 (two kinds keying the same `test` provider):** `crond` and `fetched`
  BOTH declare `oracle_effect test '' query present` and a `test__check`. Within
  each file there's no collision (one oracle per file), but at the ENGINE level a
  `[ -e X ]` site cannot be uniquely routed to crond-vs-fetched — two oracles
  claim the same provider command. The exemplars never share a provider across
  oracles (each keys a distinct command). This suggests EITHER (a) generic
  builtins like `test`/`[ ` should NOT be oracle-provider-keyable at all (they're
  the book author's own guards, not an oracle's owned command), OR (b) routing
  needs path/entity disambiguation beyond the command token. I lean (a) — but
  flagging hard, because three of my five seeds lean on `[ -e ]`/`cmp`/`grep` as
  "providers" and that may be a category error the dialect should forbid. This
  is the biggest design question the build surfaced. (~SUSPECT (a) is right.)

- **tc-F3 (read-only commands as "providers"):** related to tc-F2 and broader.
  `grep`, `cmp`, `sshd -T`, `test` are READ commands; the book uses them as
  guards/probes, not as the thing-being-orchestrated. Keying `oracle_effect` on
  them models "this read command, when seen, is a query of kind K" — which is
  how the `tool`/`pkgstate` exemplars treat `command -v`/`dpkg -s`. So there IS
  exemplar precedent for read-command-as-query-provider. But for file-state it
  produces the tc-F2 collision (many kinds, same few read commands). Unresolved:
  whether the query-provider model scales when the query commands are a tiny
  shared set (`grep`/`cmp`/`test`) rather than per-tool (`command -v`/`dpkg -s`).

- **tc-F4 (sshdconf reports file-truth, labeled as convergence):** concl-4. The
  per-option probes read `sshd -T` (file-parse), but the apply-relevant truth is
  the running daemon. I documented this in-file and report it, but did NOT
  encode any restart-gap guard (none is possible at the oracle layer). Flagging
  that sshdconf "converged" verdicts are file-claims; whether the engine should
  treat a file-state convergence as needing a paired restart-effect-discharge
  (um-file-restart-1) before licensing elision is an engine/human call.

- **tc-F5 (honest-subset vs exhaustive):** sshdconf models 11 of ~25 settable
  options; confblock models content for 2 of ~6 blocks; I omitted the rest as
  same-shape repetition. If the corpus/matrix wants exhaustive per-option /
  per-block coverage for capability-counting, that's a different (mechanical)
  job. I optimized for surfacing the SHAPE and its limits per the EXPLORATION
  charter. Flagging the coverage choice.
