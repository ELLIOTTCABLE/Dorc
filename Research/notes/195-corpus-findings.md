# 195 — Corpus: runnable-execution e2e + the defects it surfaced

> Round-19 corpus build (Deliverable A: an *executing* acceptance harness; Deliverable
> B: breadth across the elision engine). Append-only — new `## F-N` / `## C-N`
> subsections as the build surfaces them; do not edit prior content in place.
> AI-authored, confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust root
> `DESIGN`/`KNOBS`/`README`/`IMPLEMENTATION`/`AGENTS` + `plans/191` over this, and
> `notes/193`/`194` (the keystone strain-log this continues) over the prose here.

## 0. What landed (the corpus + the harness)

The e2e harness (`spike/e2e/run.sh`) gained an **executing** acceptance gate
(Deliverable A, `ap-2` / `an-render-executability-check`): for a case that ships a
`mocks/` dir + an `expected.ran` golden, it **runs the rendered apply** under
`PATH=<case>/mocks` *only* — so the sole executable surface is the inert PATH-shims
(each logs `ran: <argv>` to `$DORC_LOG`, exits 0; nothing real runs) — and asserts the
sorted run-set. A `:`-stubbed (elided) command logs nothing; a `Run` command logs its
argv. This is strictly stronger than the prior `dash -n` (parses) + text-golden
(content): it proves the *right lines actually run*. The `dash -n` gate + text golden
stay as the analysis-only discipline for cases without `mocks/`.

26 e2e cases (was 9): 13 executed (`exec-*` + the two `headline-*`), 1 xfail
(`guard-elision-wrong`, the F1 defect-pin), the rest analysis-only. Workspace + e2e
green. The matrix coverage (charter Deliverable B), each pinned by one reasoned case:
- **entity-resolution** (`command_effect`): Operand (`exec-converged`), nullary
  Singleton (`exec-singleton-update`), non-literal `$PKG` ⇒ Opaque (`exec-opaque-var`,
  the strain-8 regression *executed*), multi-entity ⇒ Opaque (`exec-multi-entity`),
  pure-builtin doesn't poison (`exec-pure-builtin`). (dynamic-command ⇒ Opaque is left
  to the `command_effect_resolves_*` unit test — redundant to re-pin e2e.)
- **ambient gate** (`classify`/`Reach`): lone establish ambient (`exec-converged`),
  same-cell kill upstream ⇒ Written (`exec-same-cell-kill`), opaque upstream ⇒ Written
  (`exec-opaque-neighbour`), distinct selectors ⇒ both ambient (`exec-distinct-
  selectors`), poison-wall dead ⇒ install ambient (`exec-poison-wall-dead`), detached
  function body ⇒ MustRun (`exec-detached-fn`).
- **prove_replaceable**: ambient+Must+Converged+quiet ⇒ Replace (`exec-converged`),
  Diverged ⇒ Run (`exec-diverged`), consumed stdout ⇒ Run (`exec-consumed-stdout`),
  `/dev/null` exempt ⇒ Replace (`exec-devnull-exempt`), `cmd &` ⊤-contained ⇒ Run
  (`background-amp-runs`, analysis-only — see C-1).
- **render/runnable**: every case is `sh -n`-clean (the xfail's *non*-clean output is
  the point), and every executed case runs the asserted set.

## THE HEADLINE — a realistic book elides genuinely, *proven by running it* (+SURE)

`headline-pi-webhost` is a bare-mutation pi-webhost provisioning book (the DESIGN
lazy-admin: no guards, the admin lets Dorc supply idempotency from oracle probes) with
package / pkgindex / service / firewall oracle coverage. On a **fully-converged** host,
Dorc elides SIX bare mutations — `apt-get update`, `apt-get install -y nginx`, `ufw
allow 80/tcp`, `ufw allow 443/tcp`, `systemctl enable nginx`, `systemctl start nginx`
— and the **executed** apply runs ONLY the genuinely-not-converged work: `nginx -t &&
systemctl reload nginx` (config-test + reload, both un-modeled ⇒ correctly always-run)
+ the `echo`. `headline-partial` shows the mixed case (ports closed + index stale ⇒
`update` + `ufw allow`×2 run; install/enable/start elide), run-set asserted by
execution.

This is real, correct elision verified by execution. Pre-keystone NOTHING elided here
(`apt-get update` was doubly-unkeyable ⇒ Opaque ⇒ Reach::Top ⇒ poisoned everything
below it — `notes/193` §1). The oracle coverage the headline *needed* to elide is
exactly strain-5's measured gap, minus the F1 trap: **package** (install), **pkgindex**
(update — the keystone's distinct-cell), **firewall** (ufw allow), **service** (enable
+ start, distinct selectors). What it deliberately does NOT model — and the honest
reason it is *bare-mutation, not guarded* — is the `command -v nginx` / `$(hostname)`
guards strain-5 named: modeling `command -v` to un-poison it triggers **F1** (below), a
priority-1 wrong-elision. So the headline takes the form DESIGN actually pitches ("some
bare mutations with no guard at all, where Dorc would borrow a probe from an oracle")
rather than the guarded form, which is broken.

---

## F1 — priority-1 WRONG-ELISION: a guard command is elided, destroying the branch (+SURE; LOUD)

**STOP-AND-FLAG, the headline defect. Pre-existing; the keystone re-key *exposed* it.**

A command whose exit **status is consumed by a guard** (`if`/`while`/`&&`/`||`/`!`) is
classified `EstablishAmbient` and `:`-stubbed when the host reports the cell converged
— which **destroys the branch decision the guard exists to make**. Pinned as
`cases/guard-elision-wrong` (xfail). Reproduction (stock `command -v` tool-oracle +
package oracle; host: nginx present, package fact converged-or-not):

```sh
if ! command -v nginx >/dev/null 2>&1
then
   apt-get install -y nginx
fi
```

renders (apply) as:

```sh
# if ! command -v nginx >/dev/null 2>&1   # dorc: elided (already converged)
:
then
   apt-get install -y nginx
fi
```

— a dangling `then` (POSIX syntax error; the `ap-2` `dash -n` gate **catches** this
particular manifestation, exit 2). The *deeper* bug under the syntax error is an
**under-execution** (`kFAIL-perform` / never-under-execute, the welded redline): the
guard's whole job is "install only if nginx is absent"; eliding the guard removes that
gate. +SURE this is **pre-existing** — it reproduces with only the stock
`fixtures/package.oracle.sh` and an `apt-get install` used as a guard (`if apt-get
install nginx; then …; fi`), no new oracle needed.

**Root cause (traced to the live design record, `16P-T10` / `notes/.../16G` §3 §66).**
The round-16 observable-replace model decided **"no status gate"**: a leaf's exit
status defaults to rc-0 in the `true`-stub and that default is *vouched by the
establishes-contract* ("declaring `(provider,verb)` establishes F *is* the claim that
when F is converged this is a successful no-op, rc 0"). `16G` §66 is explicit:
"**Status consumers (`&&`/`||`/`$?`/`if`) do NOT trigger** [the liveness gate]," and
§2 `p-oror` even *pins as a non-gap* that `install … || start` stays replaced. That
reasoning is sound for a **post-condition** consumer (`install && start` — "did my
mutation succeed?"; eliding a converged install→`:` rc-0 correctly says "yes,
proceed"). It is **UNSOUND for a guard / pre-condition consumer** (`if ! command -v
nginx`, `cmd || install`), where the command is functioning as a *probe of world-state*
("is X already true?") and a *different branch runs* depending on the answer. The
starkest case is `command -v nginx` modeled `establish present`: it **establishes
nothing** (it is a pure query), so the establish-contract vouch is *vacuous* — yet
`prove_replaceable` mints a `ReplaceLicense` for it.

**Why the keystone exposed a latent bug (the sharp interaction, ~SUSPECT durable).**
Before the re-key, *every* guard command was un-oracled ⇒ `Opaque` ⇒ `Reach::Top` ⇒
`MustRun` (the safe side). **The poison wall was accidentally protecting against this
wrong-elision.** The keystone kills the poison (and is what *lets* a guard like `command
-v` be modeled at all), and in doing so removes the accidental guard, surfacing the
latent `16G` unsoundness. So "kill the poison wall" and "guards become wrongly elidable"
are two faces of the same change — the headline win and F1 are entangled.

**The safe fix (NOT applied — flagged to the seam owner; it is a soundness-tightening
adjacent to the `tc-*` set, mine to flag not resolve).** A command whose status is
consumed by a *conditional/guard* context must carry that as a consumed observable that
**blocks** the `ReplaceLicense`, exactly as a consumed `Stdout`/`Stderr` does today.
Concretely: the CFG's `Observable::Status` (which the model deliberately *excludes* from
the consumption set) must be *included* when the consuming context is a branch
(`if`/`while`/`&&`/`||`/`!`), distinct from when it is errexit (`set -e`) — because
errexit-status IS the establish-contract's domain (eliding a converged install must not
trip `set -e`), but branch-status is NOT. This is the safe (fewer-elisions) direction.
It re-opens the `16G` "load-bearing because under `set -e` every status is consumed"
tension: gating *all* consumed-status would never elide anything; the resolution is to
gate *branch*-consumed status only, leaving errexit-consumed status vouched. ~SUSPECT
the model's own `R2-PROBEGATE` already implies the right end-state: a guard should be
lifted into the *probe* (run read-only, for real), never `:`-stubbed in the apply.

**Process datum:** this is a near-exact replay of strain-8 (`notes/193`) — a deliberate
round-16 decision (`no status gate`), sound under the conditions it was made (guards
couldn't be modeled), turned unsound by a later capability (the keystone). The corpus
exposing it is the intended outcome (charter `ch-wrong` / "a corpus that exposes a real
defect is the best possible outcome"). `ap-2` earns its keep again here: the syntax-
error manifestation is caught by the gate; the *silent* manifestation is masked only by
render line-granularity (see F1b).

### F1b — the silent variant is masked only by render line-granularity (~SUSPECT)

The block-`if` always breaks syntax (eliding the condition orphans `then`/`fi`), so
`ap-2` reliably catches *that* shape. But the same wrong *classification* (guard ⇒
`EstablishAmbient` ⇒ license) is **render-masked** in inline forms: `cmd >/dev/null
2>&1 && rhs` or `if cmd; then body; fi` *on one line* keep a `Run` leaf on the line, so
`render_apply`'s "elide a line iff a Replace leaf is on it AND no Run leaf is" rule
renders the line verbatim — the wrong license is minted but never acted on. So today the
defect manifests as either (a) a caught syntax error (block-`if`, the common idiom) or
(b) a latent-but-render-masked license (inline). The classification is wrong in BOTH;
only render-luck and the `ap-2` gate stand between it and a shipped under-execution. A
leaf-exact render (the `seam-prov` fidelity work) would *remove the render-luck mask*
and turn (b) into a live wrong-elision — so the F1 fix must land in the
classification/liveness layer, NOT be left to render to paper over.

---

## F2 — the lifter cannot bind a probe for a hyphenated kind name (+SURE; minor, parser)

`oracle_probe_package-index() { … }` does not lift: a hyphen is not a valid POSIX
function-name character, so the parser does not see a `FuncDef` (the `oracle_probe_`
prefix match fails), the body lines parse as top-level "mutators," and `bind` reports
`oracle-missing-probe`. So a kind whose *name* contains `-` (e.g. the design-intent
`package-index`) cannot have a liftable read-only probe. Worked around in the corpus by
naming the update cell's kind `pkgindex` (hyphen-free), which lifts cleanly and
round-trips (`pkgindex#fresh`). This is a disposable-front-end limitation (charter
`ch-shape-anno`: massage inputs past the parser) — but worth recording because the
keystone's own canonical example (`package-index#fresh`, `notes/193` §1/§3) is spelled
with a hyphen, so the design's flagship cell-name is un-liftable as-written. The honest
options later: (a) the lifter maps a kind name to a probe via a different encoding than
a `oracle_probe_<kind>` *function name* (e.g. a `case`-dispatched single `oracle_probe`,
or an explicit `oracle_probe_for=<kind>` marker), or (b) bless only hyphen-free kind
names. Not resolved; flagged.

---

## F3 — the CLI does NOT enforce "can't-probe ⇒ can't-elide" (~SUSPECT; harness-contract fidelity)

`build_plan`'s injected `verdict_of` in the **CLI** is `results.get(fact_label).
unwrap_or(Unknown)` — it trusts the stdin probe-results *directly*, and never consults
the compiled `ProbePlan`. So a fact whose kind has **no declared probe** (so
`compile_probe` correctly ships *no* check for it) is STILL elided if stdin asserts it
converged. Reproduction: a `package` oracle with an `oracle_effect` but no
`oracle_probe_package`; `apt-get install -y nginx`; stdin `package:nginx#installed
converged` ⇒ the apply elides the install, even though the probe block is empty. The
engine's own `an-elision-predicate` ("a kind with an effect but no probe is absent from
`compile_probe`'s output ⇒ the apply runs it") IS enforced in the `hostsim` DST tests
(`apply2_unprobeable_fact_is_not_elided`, where `verdict_of` gates on
`probe.checks_fact`) — but the **CLI round-trip does not replicate that gate**.

Is it a real engine bug or a harness artifact? ~SUSPECT the latter, but it is a genuine
*fidelity gap*: in a real deployment the host runs the *shipped* probe and can only
answer facts the probe asked, so an unprobed fact would never come back ⇒ folds to
`Unknown` ⇒ run. The CLI's stdin is a stand-in for "the host's probe results," and a
faithful stand-in would carry *only* probed facts. `cli/CLAUDE.md` documents the
*missing-fact ⇒ Unknown ⇒ run* default (load-bearing, correct) but says nothing about a
*present-but-unprobed* fact. Consequence for Deliverable A: a high-fidelity executable
harness should derive the host's answers from the *shipped probe*, not from a hand-
authored stdin that can assert unprobed facts. Today's corpus side-steps it (every
elided fact's kind has a probe), so no corpus case depends on the gap — but the gap is
real and is the kind of thing seam #3 (cli/harness) must know. Flagged, not fixed (a CLI
`verdict_of` that intersects stdin with `probe.checks_fact` is the obvious tightening,
but it is a CLI-contract change, not corpus scope).

---

## C-1 — the executable harness's determinism boundary: no backgrounded commands (+SURE)

A backgrounded command (`cmd &`) cannot be in an *executed* case: the shim races the
shell's exit, so the `$DORC_LOG` write lands non-deterministically (measured 5/10 runs
logged, 5/10 raced) — a direct `inv-determinism` violation. So `background-amp-runs` is
kept **analysis-only** (`dash -n` + golden show the `&` rendered verbatim = `Run`/⊤-
contained, which is its whole point — no execution needed). The boundary, recorded for
whoever extends the harness: the executable gate is sound only for **synchronous**
apply artifacts; anything that detaches (`&`, and ~SUSPECT a daemonizing command that
double-forks even without `&`) must stay analysis-only or the harness needs a `wait`
(which would still not catch a double-fork). This is a small, real limit of "verify by
running it," not a Dorc bug.

## C-2 — the mock mechanism + why it is safe (the Deliverable-A decision, +SURE)

Mechanism chosen: **PATH-shims of the real command names** (not `hork`/`wombat`), so the
books stay *realistic* (`apt-get install -y nginx`, the actual idiom) while only inert
shims execute. The shim is `printf 'ran: %s %s\n' "${0##*/}" "$*" >>"$DORC_LOG"; exit
0` — note `${0##*/}` not `basename`, because PATH is mocks-only so even `basename` is
unreachable (proven: an early shim using `basename` failed `command not found`, which is
itself the safety property working). Safety, triple-checked: (a) PATH is the case's
`mocks/` dir *alone* during execution, so no real binary is reachable; (b) an un-shimmed
external command ⇒ `not found` ⇒ a loud non-zero ⇒ a failed case, never a real
mutation; (c) the interpreter itself is invoked by absolute path (`$checker_abs`), so
overriding PATH doesn't break the harness; (d) the shims only `printf`+`exit 0` — no
file/network/process mutation. The clean separation the charter wants (executed = mocks
only; analysis-only = real-ish names, only `-n`'d) is structural: a case is executed iff
it has a `mocks/` dir.

## C-3 — XFAIL support added to the harness (the known-defect discipline, +SURE)

A case with an `XFAIL` file asserts the *correct/safe* behavior and is **expected to
fail** the current engine; the harness reports `xfail <reason>` (counts as ok, suite
stays green) and suppresses the raw FAIL diagnostic as noise. A surprise pass ⇒ `XPASS`
(a loud suite failure: "known defect appears FIXED — promote this case"). This is the
standard expected-fail pattern, and it is what lets the corpus *carry* the F1 defect
honestly — pinned, un-papered-over, and self-alarming if it ever gets fixed — without
flipping the green signal red. (`guard-elision-wrong` is the only xfail today.)

## C-4 — probe-execution was NOT added; here is why (the honest scope call, ~SUSPECT)

Deliverable A also names "the probe is read-only + runs without mutating." I added
execution for the *apply* but kept the probe at `dash -n` only, deliberately: the probe
artifact today is the *stub* projection (`16P T16` — oracle bodies with `$1` unbound, no
real probe-plan-builder), so executing it is neither faithful (`$1` empty) nor
load-bearing (read-only enforcement is the `hostsim` `kFAIL-withhold` monitor's job, a
*separate* mechanism the charter scopes out of the CLI — `an-withhold-sandbox` is
unbuilt). Running the stub-probe would prove "this `{ … }` block parses and exits," not
"probes are inert" (a malicious oracle could mutate; only the sandbox/monitor catches
that). So `dash -n` on the probe + the existing `hostsim` DST monitor is the honest
coverage; executing the stub-probe would *overclaim*. Flagged so a later probe-plan-
builder (`F-FW3`) knows the harness has the hook (`exec_check` is artifact-agnostic —
point it at the probe once the probe is real).
