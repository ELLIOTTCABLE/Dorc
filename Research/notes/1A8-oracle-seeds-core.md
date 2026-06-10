# 1A8 — core oracle seeds (package / service / ufw / user+group / statoverride)

> LLM-generated, part of an intentionally quality-varied ARTIFICIAL testing corpus for
> a static-analysis project (Dorc). NOT real ops code; the oracle seeds and the corpus
> they describe are FROZEN EVIDENCE and were NEVER executed (validation is `dash -n`
> parse-only plus reading). An artificial corpus cannot expose the truth of real-world
> ops-code. Builder-E (oracle ENGINEER, defensiveness ~0.9).

Seeds written to `Research/corpora/H2SaLS/oracles/`:
`package.oracle.sh`, `service.oracle.sh`, `ufw.oracle.sh`, `user.oracle.sh`,
`group.oracle.sh`, `statoverride.oracle.sh`. All `dash -n` clean.

Ground-truth I lifted the dialect from before writing (so the seeds match what the
spike's lifter/evaluator actually accept, not a guess):
- `oracle_effect <provider> <verb> <polarity> <selector>` — EXACTLY four literal args;
  polarity ∈ {establish, kill, query}; `''` = the ε-verb (`spike/crates/oracle/src/lib.rs`
  L484-525, L262-271).
- Inline annotation grammar is the 5-token `name : kind = "$N"`, or value-less
  `name : kind` for the nullary/Singleton case (`check/ast.rs` L88-111; `check/eval.rs`
  L301-317).
- The shipped probe is the `oracle_probe_<kind>` (or `oracle_probe_<kind>_<selector>`)
  body — NOT the check() body (st-2). A multi-selector kind with only a kind-default is
  UN-PROBEABLE ⇒ its sites run (`KindIndex::resolve_probe`, L216-242).
- **The evaluator's modeled sub-dialect is NARROW** and this constrained every check:
  `Word` = {Literal, SingleQuotedLiteral, Positional(n), `${n#prefix}`, Var}; anything
  else (incl. `$*`/`$@`/`$#`) lifts to `Word::Unmodeled` ⇒ ⊤ (`check/eval.rs` L336-355).
  `TestOp` = {`=`, `!=`} ONLY — there is **no** `-gt`/`-eq`/numeric test (`ast.rs`
  L138-144). `case` supports alternation arms (`-s|-G|-p`) and `*`; `shift`/`shift N`
  modeled (`ast.rs` L146-166, `eval.rs` L248-299). A `case` with no match and no `*`
  falls through with no effect (faithful sh).

Disclosure comment (verbatim header) is front-loaded in every seed.

---

## package.oracle.sh — apt-get/dpkg

What it models (harden.sh refs):
- `apt-get install -y <pkg>` (§1 L54 `sudo`; §11 L640 `gpg`, L665 `lynis`) →
  establishes `package:<pkg>#installed`. Probe `dpkg-query -W "$1"` (exemplar parity).
- install/purge single-entity core kept from the exemplar; `oracle_effect apt-get purge
  kill installed` retained for parity even though the book never purges (so a purge site
  resolves correctly instead of silently mis-resolving).

REFUSES (→ run; never wrong-elide):
- **The §2 twenty-operand line** (L130-150): `if [ "$2" = "" ]` gates the probe, so any
  multi-operand argv resolves NO probe ⇒ runs (R2-MULTIOP, the crate-CLAUDE regression
  class). Probing only `$1` (`apt-listchanges`) would elide the WHOLE install on a host
  that has package #1 but is missing package #20 (`unattended-upgrades`) — a priority-1
  under-execute. This is the single most important refusal in the seed.
- `apt-get update` / `apt-get upgrade` (L51/126/637/659/662; L127/638/663): no arm ⇒ no
  annotation ⇒ ⊤ ⇒ run (see um-pkg-2/3).

+SURE on R2-MULTIOP (directly pinned by `tests/check.rs::naive_oracle_without_operand_
guard_drops_trailing_operands_known_hazard` per the crate CLAUDE). +SURE the
twenty-operand line is the worst-case for it.

## service.oracle.sh — provider `service`, verb `restart`

What it models: `oracle_kind=service`, kind-default probe `service "$1" status` (read-
only). Provider is `service` (NOT `systemctl` — the book never uses systemctl); argv is
operand-FIRST (`service <name> <verb>`, unlike systemctl).

The book's ONLY verb is `restart` (§3 L188 ufw; §10 L628 auditd; end-of-play handlers
L686/689/692/695). **`restart` is given NO effect cell** and therefore ALWAYS runs —
this is the seed's headline (um-svc-1). The `#active` probe is declared (contract +
future-`start` use) but is NOT wired to restart.

REFUSES / un-modelable: see um-svc-1. I declared the `#active` probe as the kind-DEFAULT
(not `oracle_probe_service_active`); with zero effects either form is inert, but the
kind-default avoids a `MISSING_PROBE` error while keeping the seed honest. ~SUSPECT a
reviewer may prefer the explicit per-selector spelling for F-BLESSED fidelity (tc-svc).

## ufw.oracle.sh — limit/allow/default/logging/--force enable

What it models: `oracle_kind=firewall`; effects `ufw limit establish allowed`,
`ufw allow establish allowed`. Kind-default probe `ufw status | grep -qF "$1"`
(exemplar parity).

Book verbs: `limit in <port>` (§3 L183-184; §6 L417), `allow out <port>` (§6 L421 loop),
`default deny <dir>` (§6 L413-414), `logging on` (§6 L409), `--force enable` (§6 L410).

REFUSES (→ run): **all of this book's rules refuse the probe** via the single-word guard
`[ "$2" = "" ]` (see um-ufw-1) — they all run. `default`/`logging`/`enable` have no arm
and no effect ⇒ run (global singletons, um-ufw-1). No `delete`/`reset` verb declared
(um-ufw-2 makes a deletion-elision unsafe).

+SURE the multi-word-entity problem is real and forced the refusal: the annotation can
denote exactly one argv WORD, and `"$*"` lifts to ⊤ (verified against `Word` enum).

## user.oracle.sh + group.oracle.sh — useradd / groupadd, getent query

**group** (§1 L57-59): `groupadd <name>` verbless → `group:<name>#present`, probe
`getent group "$1"`. The CLEANEST check-then-act in the book; the book's own guard IS
this probe. `getent group` is file-backed (hermetic — contrast F-GETENT-HOSTS). +SURE.

**user** (§1 L64-69, the `-m -s /bin/bash -G … -p "$(openssl passwd -6 …)" "$NAME"`
form): verbless → `user:<name>#present`, probe `getent passwd "$1"`. The argparse is
deliberately richer than the exemplar's bare flag-strip: a `while [ "${1#-}" != "$1" ]`
loop with an inner `case … shift 2` consumes VALUE-taking flags (`-s/-G/-p/…`) so the
operand bind lands on the username, not `/bin/bash`. (The exemplar's bare loop would
stop at `/bin/bash` and bind the WRONG entity — a concrete wrong-resolution hazard the
richer loop fixes; +SURE this is a real exemplar gap for this argv shape.)

REFUSES / un-modelable: the user entity has 4 facets and only `#present` is honestly
probed (um-user-1). #password-by-value is the headline un-probeable. #shell/#groups are
probeable-but-unguarded and left out.

## statoverride.oracle.sh — dpkg-statoverride --add / --list

What it models (§1 L84): `dpkg-statoverride --update --add root suusers 4750 /bin/su`.
Effect `dpkg-statoverride add establish overridden`; probe `dpkg-statoverride --list
"$1"`. The action is a FLAG, so the check derives the verb by assignment (`verb=add`,
`--list`→`list`) — a legitimate dialect move. Entity is the PATH (last of four
operands): the check strips flags, then `shift shift shift` past user/group/mode to land
on it. The cleanest check-then-act PAIR (`--list` query ↔ `--add` establish); the
oracle-grounded probe REPLACES the book's scrappy `*exist*` stderr-match (L85-89).

REFUSES: no `--remove` verb (book has none; a deletion elided on a can't-tell `--list`
is unsafe). Value-divergence (existing override with wrong mode) unmodelled (um-stat-1).

---

## Un-modelables (shape of what WOULD be needed)

- **um-pkg-2 — `apt-get upgrade`'s entity is un-enumerable.** "Upgrade every installed
  package" has no single nameable entity and no per-entity establish. WOULD need: a
  kind whose entity is "the installed-set" with a probe that is true iff NO upgradable
  package exists (`apt-get -s upgrade | grep -q ^Inst` style) — a SET-emptiness probe,
  not a per-entity presence probe, and still volatile (depends on cache freshness).
  --WONDER whether a set-emptiness selector is even coherent in the per-entity selector
  algebra. Left out.
- **um-pkg-3 — `apt-get update` cache-freshness is volatile + a different kind.** It is
  `pkgindex#fresh` (the spike's own `exec-singleton-update/pkgindex.oracle.sh` kind),
  NOT `package`. One-kind-per-file means it belongs in a separate `pkgindex.oracle.sh`;
  and even there freshness is time-relative (`kVOLATILES`), so the right disposition is
  a DECLARED nullary effect with NO probe ⇒ update sites run. I left it out of
  package.oracle.sh entirely rather than conflate kinds. +SURE on the kind-separation;
  ~SUSPECT a sibling `pkgindex.oracle.sh` should own freshness if the corpus wants it.
- **um-svc-1 — restart-convergence is not host-state-probeable (the big one).** "The
  live daemon has loaded the CURRENT on-disk config" has no read-only host fact:
  `service status` rc proves RUNNING, not config-current; a daemon on STALE config reads
  #active. So eliding restart on #active is a priority-1 under-execute exactly for this
  book's config-reload restarts. WOULD need: either (a) a config-generation/version the
  daemon exposes and that disk can be compared to (daemon-specific, e.g. a reload-
  timestamp vs file mtime — non-general, non-hermetic), or (b) an oracle-declared,
  human-sanctioned "restart is cheap, never elide" disposition (which is just "no effect
  cell", what I did). #enabled has no `service`-native read at all (systemctl/runlevel
  territory). +SURE restart is un-elidable here; +SURE this generalizes (every restart
  in the book is a config-reload).
- **um-ufw-1 — a ufw rule's entity is a multi-word TUPLE, un-annotatable.** Identity is
  (action × direction × port × proto × from/to); the inline annotation denotes ONE argv
  word and has no join-word (`"$*"`→⊤). Annotating just the port conflates `in 22` with
  `out 22`. WOULD need: either a multi-word entity annotation (a real dialect extension —
  the entity is a normalized rule-string), or a structured rule kind with direction/
  action/port as sub-fields (the `an-entity-shape` recursive algebra). Until then,
  refuse. +SURE.
- **um-ufw-2 — `ufw status | grep` is a 2-outcome probe where 3 are needed.** Root-gated
  (`status` errors unprivileged) AND prints "inactive, 0 rules" when ufw is off — both
  collapse a tool-failure / firewall-down into "rule absent". The book ADDS rules (§3
  L183-184) BEFORE enabling ufw (§6 L410), so the pre-enable window reads every rule
  absent for the wrong reason. WOULD need: a three-outcome `ufw status` reader that
  distinguishes exit-failure / inactive / present-rule (parse `Status:` + rc separately,
  not pipe-into-grep) AND a privilege check that reads UNKNOWN (not absent) unprivileged
  (the `q-probe-privilege` gap named in the firewall exemplar). +SURE.
- **um-user-1 — useradd #password-by-value is un-probeable.** `-p` takes an ALREADY-
  crypted hash; the stored hash is in root-only /etc/shadow; `openssl passwd -6` mints a
  RANDOM salt per call so re-crypting the plaintext yields a different hash every time.
  No read-only deterministic "is the stored hash this password?" without extracting the
  existing salt and re-crypting (not a clean idempotent read). WOULD need: salt-
  extraction + re-crypt (root-only, fragile) — and even then it is a value-comparison,
  not a presence probe. So #password can NEVER gate elision. (#shell/#groups ARE
  probeable but unguarded; I left them out — eliding on #present matches the book's own
  presence-only guard but silently skips reconciling the other facets useradd sets.)
  Confirmed against the Debian `useradd(8)` man page (read-only fetch). +SURE.
- **um-user-2 — getent's CONSUMED-STDOUT use.** The book does `getent passwd "$NAME" |
  cut -d: -f6` (L109) to read the HOME DIR, whose VALUE is consumed downstream
  (`install -d "$user_home/.ssh"`, L110). A presence probe returns only rc; it cannot
  reproduce that consumed stdout. WOULD need: getent as a provider with a `query` effect
  that PREDICTS the field-6 stdout value — a stdout-PRODUCING query, not a yes/no probe
  (the one-Observable Stdout channel, predicted). The current `user` probe is presence-
  only. ~SUSPECT this is the most interesting cross-cutting shape for the observable
  model (it is a query whose Stdout, not Status, is consumed).
- **um-stat-1 — statoverride value-divergence.** `--list <path>` confirms an override
  EXISTS for the path, not that it is (root, suusers, 4750). A wrong-mode override reads
  present ⇒ `--add` elided ⇒ wrong mode persists. Matches the book (its `*exist*`
  tolerance also does not correct a divergent override; `--add` won't overwrite without
  `--force-statoverride-add` — confirmed against the dpkg-statoverride man page), so it
  is faithful. WOULD need: a value-exact probe parsing `--list`'s user/group/mode/path
  columns and comparing all three (stdout-content comparison). +SURE on the rc semantics
  (man-page confirmed: `--add` fails the sanity check when an override already exists).

## Top-3 un-modelables (ranked)
1. **um-svc-1** (restart-convergence un-probeable) — broadest: every `service … restart`
   in the book, and it is structural, not fixable in sh.
2. **um-ufw-1 + um-ufw-2** (ufw rule entity is a multi-word tuple AND the status-grep
   probe is 2-outcome) — together they make EVERY ufw rule in §3/§6 un-elidable.
3. **um-user-1** (password-by-value un-probeable) — the salt-randomization argument is a
   clean, general impossibility result for credential facets.

## tc-flags (cross-cutting; NOT settled here — flag up, don't resolve)
- **tc-svc** — service.oracle.sh declares the would-be-`start` probe as the kind-DEFAULT
  `oracle_probe_service` vs the explicit `oracle_probe_service_active`. With zero effects
  both are inert; the choice matters the moment a `start`/`enable` verb is added (F-
  BLESSED multi-selector floor). Orchestrator/human call.
- **tc-pkgindex-owner** — should `apt-get update` freshness be modelled at all for this
  corpus, and if so in a sibling `pkgindex.oracle.sh` (matching the spike exemplar)? I
  deliberately left it out of package.oracle.sh (one-kind rule). Cross-seed decision.
- **tc-getent-stdout-provider** — um-user-2 wants getent as a `query` provider whose
  Stdout (home dir) is predicted/consumed. This is a one-Observable-model decision
  (Stdout channel prediction), not a single-oracle call; flag to the observable-model
  owner.
- **tc-verb-from-flag** — statoverride derives its verb by assignment from a `--add`/
  `--list` FLAG (not a positional). I believe this is in-dialect (the engine reads
  whatever is bound to `verb`), but I did not see an EXEMPLAR doing flag-derived verbs;
  ~SUSPECT it works, --WONDER if the lifter's verb-extraction has an unstated assumption
  that the verb is argv-positional. Worth a confirming test.
- **tc-useradd-flagtable** — user.oracle.sh hard-codes the value-taking-flag set
  (`-s/-G/-p/-g/-d/-u/-c/-k`). This is a per-tool argparse contract the oracle owns; if
  a book uses a long-form (`--shell`) or `=`-joined (`-s=/bin/sh`) flag the loop
  mis-consumes. The book uses only the short forms present, so it is correct HERE, but
  the brittleness is a general oracle-authoring concern (how much argv surface must an
  oracle's check() replicate?).
