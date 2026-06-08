# 17O — Round-17 adversarial crosscheck of 17N: findings ledger

> **What this is.** The durable dump of everything the adversarial-crosscheck of `plans/17N` (the named-kind
> synthesis) surfaced — kept whole so nothing is lost, **including findings already folded back into 17N,
> ones fought/rejected, and ones deferred.** Companion to `17N`; not a design doc itself.
>
> **Method.** `/adversarial-crosscheck`: a neutral pass + a disowned-first-person-adversarial pass, each in a
> clean context seeded with the human-authored design, arbitrated by the main agent (which *co-authored 17N*
> — a declared bias, so convergence-across-passes + live verification are the trust signals, not the
> arbiter's say-so). The load-bearing shell claim (F-OFFRAMP) was verified by direct `dash`/`bash` execution.
> Round 1: 2026-06-08 (+ one cancelled adversarial pass whose two unique catches are kept). Round 2: appended.
>
> **Status tags.** `FOLDED` (into 17N) · `FOUGHT` (considered; not a real problem / mitigated / human-rebutted)
> · `DEFERRED` (real, parked) · `TASK` (promoted to a recurring example/test). AI-generated + human-adjudicated;
> trust root docs over this.

## Round 1 — findings

### F-OFFRAMP — the inline-annotation escape-hatch breaks the off-ramp weld
`[convergent: neutral + adversarial + cancelled-pass + verified live]` · **FOLDED** (17N top paragraph) → `kTYANNOT`

The `dq-kOOB` inline-annotation lean (`local w : T = "$1"`; `return 0 : "$w" is Wombat#frocked`; dotted
`frobctl.check()`) is **not a behavioral no-op** and breaks the off-ramp weld + the behavioral-no-op
contract. Verified live: `dash` → `local: :: bad variable name` (aborts); `bash` → two warnings, `w` left
empty, rc 0 (silent corruption); dotted name → `dash -n` "Bad function name"; un-stripped `{a,b}`
brace-expands under bash, not dash. The off-ramp is *by definition without Dorc* (DESIGN L33, `ssh host 'dash
-s' <script`), so "strip it with `dorc escape-hatch`" (a) understates the work — it is a correctness-critical
source-to-source transpiler, not a regex strip — and (b) does not restore the *trivial* off-ramp. 17N uses
"breaks the off-ramp" as a welded kill for HM (kill-8) yet (old line 461) called this "moot" — an internal
inconsistency. **Consequence (the in-scope empty-deferral):** every `dq-kOOB` option is closed (sh-native:
none; env-vars: too DSL-y; comments: the forbidden floor; inline: off-ramp break) → `dq-kOOB` is
*unsolved-within-welds*, not provisionally-resolved. **Human ruling:** the real trade is a knob (`kTYANNOT`,
inline-ergonomic-but-off-ramp-hostile ↔ eol-comment-ugly-but-off-ramp-clean); whether a type-system exists
*at all* stays UNSETTLED. Note both `kTYANNOT` poles sacrifice a "spelled in sh" principle — inline sacrifices
the off-ramp (`kLANG`), eol-comment re-opens `kOOB`'s no-comment-parsing redline.

```sh
# STRAWMAN A — 17N's spelling; FAILS the off-ramp (each line a regression assertion under stock dash/bash)
frobctl.check() {                                          # dash -n: rc2 "Bad function name"
   local w : com.frobber.Wombat{defrocked,frocked} = "$1"  # dash run: rc2 "local: :: bad variable name"
   frobctl --is-frocked "$w"                               #   bash run: w left EMPTY, rc0 (silent corruption)
}
# STRAWMAN B — datum form (17N P1); off-ramp-CLEAN (parses + runs identically with/without Dorc)
oracle_kind=net.frobnitz.wombat
frobctl_check() { w="$1"; frobctl --is-frocked "$w"; }
frobctl_check wom || frobctl --frock wom                   # drop the oracle ⇒ book still runs (gradual-remove)
```

### F-ALGEBRA — `dq-entity-algebra` is not deferrable (≥enum floor + 16Q gate force structured)
`[neutral; in-scope empty-deferral]` · **FOLDED** (17N §4)

17N called this "lower-stakes, leave fuzzy", but 16Q §3 gates `q1-precision` (the spike's keystone) on
settling flat-vs-structured first ("wrong shape re-keys every transfer function and the substrate"), and the
≥enum floor's own systemd example forces structured (`#enabled` and `#active` are *independently*
mutation-gating). The deferral is empty across the design-range. **Human lean:** a recursive JSON-adjacent
structure (present-key = `true`, `!`-pun for false, values = direct types / kind-handles / nested structs;
bias under-coordination + over-completeness; absent ≠ asserted-false).

```sh
# STRAWMAN — a flat service:nginx wrongly discharges is-active from is-enabled ⇒ priority-1 under-execute
unit_enabled() { systemctl is-enabled --quiet -- "$1"; }   # svc#enabled
unit_active()  { systemctl is-active  --quiet -- "$1"; }    # svc#active
unit_enabled nginx || systemctl enable  -- nginx
unit_active  nginx || systemctl restart -- nginx            # flat key elides this wrongly when enabled ∧ ¬active
```

### F-FW3 — fw-3 (phase-agnostic verdict) stated as settled, but it is a possible "one-caller fiction"
`[neutral + 16P/16Q]` · **FOLDED** (hedge) / **DEFERRED** to the spike

17N's Seam states fw-3 ("engine emits phase-agnostic facts; the phased caller collapses them") as resolved.
16P **T11** records it was never load-tested with two callers and the naive realization *bakes itself*: *"a
per-phase `Bias::on_consumed_output` method was floated and not added — the result enum is apply-shaped
(`{Replaceable, Run}`, no `Withhold`), so a per-phase consumed-direction method would force the unbuilt probe
impl to return the wrong (apply-shaped) answer, baking exactly the posture the principle forbids."* 16Q §2b:
the probe-plan-builder *"is the only place `inv-superposition` ever acquires a real second caller … reveals
whether 'engine emits, caller collapses' holds with two real phased callers, or was a one-caller fiction."*
Carry the hedge; do not encode one phase-agnostic verdict lattice as proven. Connects to DP-3 (the three-valued
verdict cannot be emitted by `cmd | grep -q` — same rc-conflation as F-BLESSED).

### F-REJECT — F0/spine-1 "never reject" vs F5 "right to reject the annotation" (the trust-spectrum)
`[neutral]` · **FOLDED** (17N F5)

A rejecting annotation gate looks like the tool picking a safety-direction (spine-1 forbids). **Human
resolution (a trust-spectrum):** never infer-upward-to-reject; never reject on insufficient-context (both
⊤-run); *do* reject on ill-formedness, and ~SUSPECT on typing-conflicts — because annotating is Dorc-specific
work, so the error is **deferred off the day-1 path** (a gradual gradient, no cliff). Never reject plain
working sh, never pick a correlation's safety-direction.

### F-BLESSED — §7 blessed probes use shapes 17N itself bans
`[neutral]` · below-floor half = F-ALGEBRA · rc-conflation half **FOUGHT** (elision-direction-safe)

The strawmen wire `service → systemctl is-active` (one facet, below ≥enum) and `id -nG X | grep -qw G` (the
P2/DP-3 rc-conflation). The below-floor half is real (→ F-ALGEBRA). The rc-conflation half is *weaker* than
first stated: for the elision it fails safe (tool-failure → rc1 → "not a member" → run the usermod →
over-execute). The durable point: **blessing is not "free read-off-the-command"** — honoring ≥enum needs
per-kind facet-splitting (two commands for a service), and some kinds (group-membership) have *no* clean
blessed probe (`id` emits a *list* needing substring-match).

```sh
# STRAWMAN — the honest ≥enum service probe is TWO commands, not the one §7 implies is "free"
svc_enabled() { systemctl is-enabled --quiet -- "$1"; }
svc_active()  { systemctl is-active  --quiet -- "$1"; }     # discharging 'enable --now' needs BOTH converged
```

### F-GETENT-HOSTS — `getent hosts`/`ahosts` is non-hermetic (live DNS)
`[adversarial-only; verified true]` · **TASK** (canonical recurring example — see task list)

17N §7/P9 blesses `getent <db>` as uniformly read-only and lists `hosts`, but `getent hosts` routes through
nsswitch → DNS = live network + nondeterminism, colliding with `kVOLATILES-exclude` (hermeticity is a skip
*precondition*) and the `kFAIL-withhold` "the probe observes nothing of the world" story. `passwd`/`group`
are file-backed (fine); `hosts`/`ahosts` are not. **Read-only ≠ hermetic.** Keeper example — it is at once
(1) a buried side-channel (network) behind a "read-only" probe, (2) the canonical motivator for **annotating
network-access**, and (3) the eventual acceptance test for a network-access linter.

```sh
# STRAWMAN — a "host exists" guard blessed as read-only that actually hits the network
host_known() { getent hosts "$1" >/dev/null 2>&1; }   # 17N §7: "read-only, 3-outcome"; really: live DNS
host_known db.internal || echo would-provision         # shipped into the NON-mutative probe phase = a leak
```

### F-SEQUENCE — §8's cross-author wins need the deferred interproc/source-following core
`[neutral]` · **DEFERRED** (sequencing note)

The redis `www-data` and gh-runner `docker` elisions require `. /path` source-following and call-edge
modeling that 16P/16Q record as deferred (`q1-interproc`, T8/T2). 17N reports them as strawman *viability*,
not as spike-built — but F8 calls that machinery "welded must-handle", so the sequencing risk is real: do not
build the channel expecting the headline examples to fire before the analysis core lands.

### F-FAITHFUL — the faithful render would carry the annotations
`[cancelled-pass]` · **FOUGHT** (human-rebutted)

Concern: `render_apply` (faithful, `kFIDELITY`) shows the book line-for-line, so annotations would appear in
the plan as non-runnable lines unless the renderer strips them on the hot path. **Human rebuttal:** faithful
means "what-you-typed, typed-back-and-greyed-out"; if you typed annotations, you see typed annotations — no
contradiction. (Human went further: optionally surface *weak-inferred* types in-UI for accept/bake/promise,
patching the file on accept — a feature, not a bug.)

### F-ESCAPENAME — `dorc escape-hatch` collides with the `dorc <verb>` host-action vocabulary
`[cancelled-pass]` · **DEFERRED** (dissolves if `kTYANNOT` picks the eol-comment pole)

Every other `dorc <verb>` is a host action; a source-transform verb you must run *to off-ramp from Dorc*
inverts "the off-ramp is trivial." Low stakes; folds away if the off-ramp-clean pole wins.

## Carve-out for round 2 — do NOT re-report these (discussed enough)
F-OFFRAMP (annotation / off-ramp / `kTYANNOT`) · F-ALGEBRA (entity structure) · F-FW3 (verdict phasing) ·
F-REJECT (reject trust-spectrum) · F-BLESSED (blessed-probe floor / rc-conflation) · F-GETENT-HOSTS (the one
network example — *reusable as a template* for OTHER buried-side-channel kinds, but don't re-report `getent
hosts` itself) · F-SEQUENCE (interproc sequencing) · F-FAITHFUL · F-ESCAPENAME.

**Round 2's mandate:** *different, deeper* contradictions — ideally mined from real ops-script semantics
(Ansible playbooks/roles: wrong format, right semantics + hazards). Range wide.

## Round 2 — findings (2026-06-08; neutral + adversarial, in-union)

> Mandate: different/deeper than round 1, mined from real Ansible semantics. The spine **both passes hit
> independently**: 17N's effect-map is per-leaf, per-kind, and *state*-shaped; real ops idempotency is
> dominated by *cross-leaf, cross-kind, change-/event-triggered* effects (every Ansible `notify`/handler).
> Empirical claims verified live by the arbiter (dash/bash). Convergence noted per finding.

### R2-CHANGEDELTA — the effect-map carries states, not run-deltas; "do B because A changed" has no carrier
`[CONVERGENT: neutral R2-A + adversarial R2-1 — the round's headline]` · **OPEN (new; candidate DESIGN-level decision)**
`inc-7`'s effect is a transition of *one kind's* state; the dominant real pattern (write config → reload
service *iff the write changed this run*) is a **cross-kind, change-triggered causal edge** (`file:` →
`service:`) the per-kind effect-map cannot represent, `opt-3` (cross-*facet*-of-one-entity) doesn't reach, and
MUST-grade-to-correlate refuses to synthesize (no declared edge ⇒ no correlation ⇒ run). Two modes: the
*change-gated* form (`cp && changed=1; [ changed ] && reload`) needs Dorc to track the change-flag data-flow
*and* understand that eliding the config-write **removes its `changed=1` side-effect** (eliding a leaf changes
downstream data-flow — a real `q1-interproc` hazard); the *unconditional* form (redis `restart_service`)
cannot elide the reload at all without the absent cross-kind edge → run-every-bump (over-execute / value-prop
loss) or wrong `service#active` elision (under-execute). `kVOLATILES` makes config-freshness hard to cache,
worsening it. The deepest finding of the whole crosscheck — it is about the *model shape* (`inc-7`), not the
spelling. [strawman: webd config→reload, both modes]

### R2-CONTEXT — the wrapper-context deferral (§7) is EMPTY for the probe phase (welded-entangled)
`[CONVERGENT: neutral R2-B + adversarial R2-3; the in-scope empty-deferral]` · **OPEN (the §7 deferral is not free)**
Execution-context (`sudo -u`/`ssh host`/`docker exec`/`delegate_to`) parameterizes every instance-identity —
the same argv reads a *different cell* per (user, host, namespace), so §1's "same global state everywhere" is
false. Three probe-phase options, each breaks a welded/load-bearing constraint: (1) probe in default context →
wrong cell → under-execute; (2) probe in the declared context → `ssh delegate`/`sudo -u` *during plan* = a
network/context side-effect into a not-yet-applied target = `kFAIL-withhold` redline; (3) ⊤⇒run → no
context-wrapped effect ever elides → value-prop collapses on the fleet workloads `kSCHEDULE`/`kOBJECTIVE`
exist for. The reverse-DNS handle (C2) carries no context dimension (no depth at which a consumer says "match
same-context"). Identity is declared-not-inferred (F3/SF-1), so deferring context-as-kind defers a
*correctness precondition of the welded probe phase*. [strawman: `sudo -u` npm-global + `delegate_to` k3s
node-token]

### R2-TRAP — a mutating EXIT/ERR trap fires during the probe phase (`kFAIL-withhold` breach) — VERIFIED LIVE
`[neutral R2-F; verified]` · **OPEN (probe-inertness is whole-script, not leaf-local)**
`set -e; trap 'rm -rf …; systemctl stop …' EXIT; probe(){ …; }; probe` — confirmed: the mutating trap fires
when the shipped probe fails under errexit (and on normal exit). The `16Q` per-leaf inertness certifier
inspects only the probe *body*; the mutation lives in ambient `trap`/`set -e` reachable from the probe's
*failure/exit* path. So "certify the probe inert" is a whole-enclosing-script obligation (failure path
included), not leaf-local — a `kFAIL-withhold` *soundness* concern, earlier/stronger than F8's "model traps for
precision." Lands on `q1-probe-projection` (no trap context modeled today). [strawman: trap-bearing `myapp`
probe]

### R2-SHADOW — blessed `command -v X` corrupted by function/builtin shadowing — VERIFIED LIVE, unsafe direction
`[neutral R2-E; verified]` · **OPEN (blessing `command -v` is not free)**
`docker(){ :; }; command -v docker` → `docker`, rc 0, no binary (dash + bash). The blessed `tool → command -v
X` probe reports "installed" for a function/alias/builtin and **fails unsafe** (elide the install → priority-1
under-execute), triggered by Dorc's *own* function-named-helper / sourced-oracle idiom. The honest probe must
defend the shell's namespace (`command -v X` *and* it resolves to an executable file path). Distinct from
F-BLESSED's pipe/grep rc-conflation. [strawman: a `docker()` wrapper shadows the binary]

### R2-ORTRUE — `|| true` / errexit-masking destroys a lifted guard's verdict — VERIFIED LIVE, unsafe direction
`[neutral R2-D; verified]` · **OPEN (lifting the book's guards assumes un-masked rc)**
`svc_up(){ systemctl is-active --quiet "$1" || true; }` lifts as a probe that *always* reports holds (rc forced
to 0 — confirmed); and `set -e` is suppressed inside `if`-guards. The author wrote `|| true` for
errexit-survival (apply-time), indistinguishable to the lifter from a probe-time verdict → under-execute. The
two-users tension: the lazy *admin* writes `|| true`; the *engineer*'s lift-as-probe contract assumes rc means
something. A lifted guard's rc is a verdict only if the analyzer can prove it isn't errexit-masked (`|| true`/
`|| :`/`; true`) — often impossible → collapses "the careful book writes the oracle for free" on real code.
[strawman: `svc_up || true`]

### R2-IDCACHE — the §8 flagship elision is licensed by a NON-HERMETIC probe (`id -nG` ≠ `getent group`)
`[adversarial R2-2; sharpens F-BLESSED; mechanism documented, NOT locally reproduced]` · **OPEN**
§8 discharges via `getent passwd www-data` (authoritative, hermetic) but **elides** via `id -nG www-data | grep
-qw redis`. `id -nG` reads the *resolved/cached* group set (nscd/sssd; stale until next login — RH sol.
7115906), which diverges from `/etc/group`. Member-removed-but-cache-warm → `id` says member → Dorc elides a
needed usermod → priority-1 under-execute, *on the synthesis's flagship example*. `kVOLATILES-exclude`
disqualifies the volatile probe from licensing a skip — yet it is the skip-licensing probe. Plus a same-session
probe→apply→re-probe TOCTOU (Ansible inserts `meta: reset_connection` here). Fix: probe membership via `getent
group | field-4`, never `id`; forbid same-session re-probe of a mutated cell. [strawman: `member_resolved` vs
`member_authoritative`]

### R2-PROBEGATE — no coherent plan when a probe's *result* gates a downstream probe/mutation subtree
`[adversarial R2-4 only — suspect-until-checked; survives the arbiter's scrutiny]` · **OPEN (intersects plan/apply + `kFLATTEN`)**
Ansible check-mode (the mature prior-art for the probe phase) *documents* it cannot predict past a
register-gated `when:`. When leaf-A's probe verdict decides whether a subtree (with its own probes + mutators)
is live, the plan can't be built leaf-independently: conservative-⊤ ⇒ plan == script (value-prop gone where
guards nest — the target audience); speculate-both ⇒ ship probes (some inert *only because* the gate held; e.g.
`systemctl start` is not read-only) into possibly-dead branches ⇒ `kFAIL-withhold` breach; serialize
probe→decide→probe per gate-depth ⇒ the cross-network big-O that dominates, defeating flat parallel probing.
F-FW3 aimed one level up — *plan completeness* under data-dependent control flow, vs F-FW3's fail-direction.
Lands on `q1-probe-projection` + `q1-interproc`. [strawman: `getent`→`needs_add`→nested `start`+`usermod`]

### Round-2 end-list (lower; mostly fold into the above)
- **loop coalescing** (neutral R2-C / adversarial n-1): one `for`-leaf = N instance-establishments; per-leaf
  verdict can't carry per-item dispositions ⇒ whole loop ⊤⇒run (over-run, *safe*) — a value-prop cliff on the
  common loop-with-guard, not a correctness hole. Interacts with R2-CHANGEDELTA (one changed item re-causes all
  handlers).
- **`listen` topics** (adversarial n-2): notify fans many-to-many by topic ⇒ the cross-kind causal edge is
  *many-to-many*, making `opt-3` even less adequate. Severity for R2-CHANGEDELTA.
- **`changed_when`/`failed_when`** (adversarial n-3; neutral R2-H `changed_when:false`): "converged" is often an
  author predicate over *stdout text*, not rc — no clean sh spelling for "this state-test is `grep -q PATTERN`
  over stdout" ⇒ feeds `kTYANNOT` from a second direction; and Ansible's one-line `changed_when:false` no-op
  declaration is cheaper than Dorc's "write an oracle or get ⊤-run" (a `kBURDEN` asymmetry vs "we ask less").
- **`meta: flush_handlers` / ordering** (adversarial n-4): change-triggered effects carry a load-bearing partial
  order (apt-cache flush before install); `kSCHEDULE` + elision assume an effect-independence the notify/flush
  model violates.
- **block/rescue vs welded fail-fast** (neutral R2-I): a failed probe triggering *recovery* mutation vs the
  AGENTS "stop on unknown state" rule; flagged as a question, not asserted.

### Human dispositions (2026-06-08, post-presentation)
- **Terminology reconciliation — "blessing" *is* a stdlib oracle.** The human reads "blessing a builtin" as
  "ship a first-class, high-quality probe/oracle for that builtin/pattern, day 1" — a *stdlib oracle*, not a
  distinct magic mechanism. So "falls back to oracle-supplied, *not* blessing" was a false dichotomy
  (blessing ⊂ oracle-supplied). Re-read all "blessing" talk (175 Result D, 17N §7, F-BLESSED, R2-SHADOW,
  R2-IDCACHE) as "the *stdlib oracle's* probe must be good sh."
- **R2-SHADOW · R2-IDCACHE · F-BLESSED · F-GETENT-HOSTS → ORACLE-QUALITY class (not too concerning).** All
  reduce to "our stdlib oracles + oracle-writing guidance must be *good, battle-tested sh*, which is
  sometimes hard." Fix = a better spelling (executable-file check; `getent group` field-4, never `id`;
  per-database hermeticity). Kept as regression tests; **not** design holes.
- **R2-D (`|| true`) → ORACLE-QUALITY** (after elaboration): lifting a guard as a probe assumes its rc is an
  *un-masked* verdict; errexit idioms (`|| true`/`|| :`/`; true`) can violate that. The lifter must refuse
  to treat a masked rc as a verdict. Same bucket as R2-SHADOW; not a model hole.
- **R2-TRAP → non-issue in practice; keep as a probe-phase regression test.** Clarification (human): the
  probe is assembled from **oracle bodies** + minimal CFG chunks (replicated only where a probe is
  valid/inert solely under the same guard), chosen by analysis *of* the book — **not** from the book's
  *contents*. So a probe should never inherit the book's ambient `trap`s. The whole-script-inertness point
  is *true* (verified) and a good regression test (probe-construction must never carry ambient traps), but
  is not expected to fire.
- **R2-CONTEXT → DEFER (split).** `ssh host CMD` / `docker exec C CMD` are **eval-class pathological**
  (unanalyzable/unelidable) → defer/wontfix. `sudo`/become **is** important and almost certainly needs
  **first-class, baked-into-the-language** handling later (prior art: Ansible `become:` — but heed how much
  pain `become:` caused; care warranted). Defer both enumerated cases for now. (Far future: a mitogen-style
  intelligent-bouncer / multi-host-step / bastion could be two-for-one for the ssh/docker case.) Still rated
  a strong finding.
- **R2-PROBEGATE → RESOLVED (the probe model).** Dorc *speculates* — lifts read-only probes from the CFG and
  runs them concurrently (oracles *intercept*: `id__check` ships + replaces `id`), reserving the right to
  preserve a CFG fragment where a probe is valid/inert only under a prior guard. A probe-gated branch is
  resolved by *running the read-only probe for real* (unlike Ansible check-mode, which is blind past a
  register-gated `when:` because it does not run the gating task — Dorc's probes are read-only, so it can).
  The only residue — a gate on a *mutation's* result — folds into R2-CHANGEDELTA. Compiled-probe durable:
  `17x-strawmen/adversarial/compiled-probe.straw.sh`; inlined into `plans/17N` §3.
- **R2-CHANGEDELTA → RE-ASSESSED; severity downgraded (NOT a correctness hole with correct oracles).** Under
  the probe model: (a) the *well-guarded* form is HANDLED — the author's `changed` flag carries the cross-kind
  edge, and it is a *consumed observable* the elision discipline (16P witness) must **preserve, not
  synthesize**; this is a **`q1-precision` acceptance test** (track the `changed` variable across cp→reload),
  not a new effect-map dimension. (b) The *bare/unconditional* form over-executes safely (`kBURDEN` gradient);
  the priority-1 under-execute needs a WRONG oracle (`reload`→`is-active`) = oracle-quality. (c) Dorc must
  never elide a delta-gated effect via a *state*-probe, nor synthesize the cross-kind edge. The durable DESIGN
  content (the *un-probeable, change-gated effect class*) is filed as an **LLM-tagged TODO in `TODO.md`**
  (to-write-into-DESIGN). Strawmen: the run-delta forms in `r17-crosscheck-dorc-inputs.straw.sh`. Task #14.
- **Strawmen collected:** `Research/notes/17x-strawmen/adversarial/` — `r17-crosscheck-runnable.straw.sh`
  (verified shell-hazard demos) + `r17-crosscheck-dorc-inputs.straw.sh` (Dorc elision-decision inputs).
