# 27C — Context-entry probing: the escalation dial, the tolerance vouch, and the demoted transport lane

AI-authored (Fable, outside-review dialogue with the human, 2026-07-16). This is the
RESULTING DESIGN of the 27A/27B rescue arc — not a play-by-play; `27B` holds the
refutation argument (superseded as design), `27A` holds the wall inventory (still true of
the lane it analyzed). Authority: root docs and human-TYPED rulings outrank this; the §10
ack-ledger states exactly which pieces the human typed this dialogue — those are
rulings; every spelling marked STRAWMAN is mine and swappable. This note answers
`279f:ask-transport-disposition` and carries the substance of
`279f:ask-flag-boundary-recut`; block-context implementation-planning consumes it whole.

Product vocabulary per `27A`: product A = elision (attention), B = guards, C = flagged
risk-acceptance, D = lints/hints. Axis vocabulary per `24S`: user · fs-view · netns · ρ.

## §0 — The design in one screen

**Wrapped sites are answered by measurement in the site's denoted context, not by
transporting out-of-context measurements under claims.** A wrapped line elides (product
A, unflagged) when four independent consents align, each on its own subject:

1. **The admin's escalation dial** (`27C:dial-escalation-ternary`) licenses the probe
   lane to shift the execution context of oracle code — default posture: shifts
   licensed for *vouched* oracles, using only authority the connection already holds.
2. **The wrapper chain is fully modeled**: each wrapper's oracle peels the site and
   authors the *entry form* — the non-interactive bytes that enter its context
   (`27C:entry-form`). An unmodeled wrapper never peels; the line stays an opaque wall
   (unchanged law).
3. **The executed oracle function carries the tolerance vouch**
   (`27C:vouch-tolerates`) — a minted, per-function, axis-parameterized mark: "this
   body may be *executed* in contexts moved along these axes."
4. **The measurement itself succeeds**: entry failure or an in-context refusal is
   rc≥2 ⇒ can't-say ⇒ guard/run (`wall-empirical-rc`, unchanged).

Everything below that bar degrades along the existing ladder: in-context **guards** at
apply under the same dial×vouch license (blast-radius is lane-independent —
`27C:rule-lane-equivalence`); **conditional tails** contain residual guard-cascades
(`27C:route-conditional-tail`); the **transport lane survives, demoted to fallback**
(`27C:lane-transport-fallback`) — kind-tier `invariant:<axis>` and answer-invariance
claims, flag-tier only, for the cells measurement cannot reach. Mined sh idioms
(identity guards, honest reads, env-neutralization) are **lint-feeders only** — they
teach, corroborate, and predict; they never license (`27C:law-perfect-overlap`).

The one-sentence security story: **the probe lane never acquires authority — it re-uses
what the connection already holds, only for oracle bodies whose authors accepted
context-shifted execution, only when the admin's dial permits, and every shifted
execution is attributed to all three consenting parties.**

## §1 — What this resolves, and why measurement escapes the 27A trilemma

`27A` proved: any *claims-based* transport design loses one of {sound-by-default ·
minimal-oracle-stays-minimal · unflagged product A under wrappers}. The proof stands.
This design is outside its quantifier: no claim carries a measurement across a context
boundary, because the measurement executes *inside* the boundary. The license gap
(backing marks carry no completeness burden) never opens — nothing crosses. Referential
agnosticism is untouched: the engine still consumes only claims and probe outcomes; the
outcomes are simply produced in the right world.

The load-bearing unblocking: `24S`'s imp-1 ("probes never escalate") was proposal-tier,
never human-welded, and conflated two axes — the *mutation* weld (`kFAIL-withhold`,
which stands untouched) and *privilege* (which the root docs never mention). Its
defensible core is re-scoped as `27C:rule-reuse-never-acquire` (§2). The `an-privilege-
fact` r23 addendum (lane-PRIVILEGE, `23J`) was this design's deferred seat all along.

What was NOT unblocked, and stays true from `27A`: the tool-internal frame-problem half
is permanent; unmodeled wrappers wall; claims still cannot license access
(`wall-measurement-reach` — measurement is the thing that changed, exactly as that
wall's own text anticipated); and the transport lane's default polarity for fs-view and
netns remains WALL.

## §2 — The authority model: four cells, one dial

### The cells (human-defined, 2026-07-16 — these words are the ruling)

- `27C:cell-root-reuse` — **connection is root; re-use licensed** (DEFAULT): "all
  user-changing for probe purposes is licensed, by default, if the engine has the
  permissions to do it upon connect." Covers `sudo`/`su` targets, `chroot`, `netns`
  entry — uniformly, since a root connection performs all of them with zero new
  credentials.
- `27C:cell-no-shift` — **the blast-radius flag**: "no user-changing for
  probe-purposes is licensed, because we can't tell what user-changing might be giving
  bad oracles more-damaging behaviour." Maximally defensive for sensitive machines.
  Oracle code executes only as the connection identity, in BOTH lanes; book bytes at
  apply are untouched (Dorc has no hand in the book escalating itself).
- `27C:cell-granted-acquire` — **connection non-root, explicit acquisition granted**:
  "you give us a tool/command/key/something, and we connect non-root, then
  *specifically* elevate when required." Opt-in product C; mechanism design DEFERRED
  (sketch: a pre-probe, pre-ack, one-shot `sudo -v`-style moment per host, opt-in by
  flag/config, credential never stored — kSTATE-fenced; possibly polkit/`run0` on
  modern hosts). Ships after the other cells.
- `27C:cell-static-identity` — **the residue**: no permission available; the probe
  runs as the connection identity, full stop. Wrapped modeled sites render per the
  fallback ladder. Human priority ruling: the non-root columns are an easy
  best-effort win, NOT a primary goal.

### Why there is no "demotion is safe" ordering (`27C:rule-no-privilege-order`)

Rejected explicitly; record the evidence so nobody re-mints it: root→postgres grants
*net-new capability*, not less (peer-auth `psql` succeeds as postgres where root-qua-
root is refused; NFS root-squash makes root read *less* than the target user); from a
non-root base, "lateral" swaps are mechanically up-then-down through the superuser
anyway; MAC systems (SELinux/AppArmor) un-order root further. User-swaps admit no
computable safety relation. The implementable predicate is the cells': *can the
connection perform this shift with zero new credentials.* The gated quantity is
**damaging-oracle blast-radius**, and it is lane-independent (`27C:rule-lane-
equivalence`, human-acked): a mutating "check" is equally damaging at probe-time and
inside an apply-lane guard, so the dial and the vouch gate BOTH lanes' oracle-code
context-shifting. (ρ-only wrappers — `env`, `nice` — shift no identity/view and are
outside the gate.)

A "constrain the superuser" sub-dial (license shifts except to uid 0) is implementable
almost free at the peel (the `-u` operand resolves) but is only coarsely monotonic
(lateral damage to a service user is not ordered against root damage) —
field-demand-contingent, not built (`27C:seam-no-root-targets-subdial`).

### The dial (STRAWMAN names — human-uttered, unbikeshedded)

- `--no-probe-escalation` — cell-no-shift.
- `--probe-escalation` — THE DEFAULT: shifts licensed, **only for functions carrying
  the tolerance vouch**.
- `--escalate-any-probe` — shifts licensed for unmarked oracles too; the admin
  knowingly overrides absent author consent (for fleets of unvouched third-party
  oracles; accepts the blast-radius alone).

This is the both-sides protection: the default requires author-consent (the vouch) AND
admin-consent (the default dial state); either side can unilaterally withhold. Naming
wrinkle: the strawman names say "probe" but the semantic scope is both lanes, per
rule-lane-equivalence.

Consent legibility rider (`27C:render-authority-disclosure`): the plan header discloses
authority use in one line — "probe re-uses connection authority (root): →postgres (1
site), chroot /mnt/target (2); forbid with --no-probe-escalation."

Boundary reading, chosen-defensive and cheap to flip
(`27C:rule-lifted-guards-gated-too`): probe-time execution of *lifted in-book guard
material* that itself contains a context-shift is gated by the same dial — under
cell-no-shift it stays apply-time-only. (How in-book hand-written guards yield probe
value at all remains a danced-around corner — see §9 open-corners.)

## §3 — The oracle-side act: the tolerance vouch

### Semantics

`27C:vouch-tolerates` asserts, for exactly the function that carries it: **"this body's
effects are read-only by design, not by privilege-starvation — executing it in a
context moved along the named axes will not mutate."** The claim has real content the
author uniquely holds: a check can be honestly non-mutating as an unprivileged user
*because its opportunistic writes fail on EACCES* — `pipx list` scaffolds
`~/.local/pipx` when it can; tools refresh caches and locks when permitted. An author
who only ever ran their check unprivileged has verified the weak property; the vouch is
the strong one, typed knowingly.

What it does NOT claim: nothing about the *answer* (answers are supposed to differ per
context — that is the point of measuring in-context); nothing about answer-invariance
(§5's separate, stronger claim); nothing about other functions in the file; no change
to what the function's verdicts license at its tool's sites (the existing vouch rules
are untouched).

### Siting and spelling (STRAWMAN — chosen per the human's directive to pick and run)

A **bare-mark statement** in the existing sigil family, sited in the function body,
reachability-scoped like every in-body act (place it in a `case` arm to vouch per-verb):

```sh
pipx__is_converged() {
   : tolerates:user                          # STRAWMAN: the tolerance vouch, user axis
   verb="$1"; shift
   case "$verb" in
   install) pipx list --short 2>/dev/null | grep -q "^$1 " ;;
   *) return 2 ;;
   esac
}
```

Multi-axis via the `277` brace-alternation: `: tolerates:{user,fs-view,netns}`. Strip
behavior: bare-mark statement ⇒ deleted whole-statement (existing law; body-top siting
never disturbs the last-status-affecting-statement rule). Per-function throughout — sh
has no family construct, and this design adds no file-scope ceremony; a two-member
oracle marks each member it wants shiftable (`predict` and `is_converged` each ship).
Per-axis by construction — the human's gradual-enhancement correction: the babby
template carries `: tolerates:user` (the dominant axis); "safe under ANY chroot, ANY
netns" is never demanded of anyone; unnamed axes simply stay unlicensed for that
function. Published/stdlib oracles carry the fuller sets, where the effort already
lives.

Why a minted mark and not a mined idiom: the human's razor, recorded as a general
design law — `27C:law-perfect-overlap` (human-typed 2026-07-16): **"a boolean switch
can only be spelled-as-sh if there is perfect overlap between the sh-spelling and the
meaning: it will never be wanted when the sh-behaviour isn't wanted, and never unwanted
when the sh-behaviour is wanted."** No candidate idiom met the bar (§6); by the
be-sh-or-be-very-not-sh doctrine, that means mint hard-non-shell. (This law wants
propagating to the standing-rulings surface — human act, reported in §11.)

Bar-softening context an author should hear (docs/teaching material): under the default
dial the execution envelope is not "anywhere" — it is *the contexts this book's own
wrapper chains denote on this host*, the same contexts a hand-written guard for those
sites would have run in. The universal envelope only matters for published oracles at
strangers' sites — the stdlib-quality-bar tier (two-user differential CI, tracer
read-set diffing), not the babby tier.

Hint machinery (the gradual curve's rung): an unvouched function under a wrapped site
renders the site as run/guard with the stage-2-style attribution — "line 8 would elide
if pipx's oracle vouched context-tolerance (one line: `: tolerates:user`)".

## §4 — Execution mechanics

### Only oracle bytes enter (`27C:rule-only-oracle-bytes-enter`)

Book bytes never reach the probe lane (standing law, `271:rul-only-oracle-bytes-ship`,
ratification riding task-14). The shipped form for a wrapped site is the `273` §6
composition with the simulation target swapped for reality: **the wrapper-oracle's
authored ENTRY FORM composing the inner oracle's body**, invoked with the site's argv.
The site's wrapper chain *selects* which contexts are entered; every byte that executes
is oracle-authored. STRAWMAN entry-form shape, a role-sibling in the wrapper-oracle's
family (`24S` §2b gains one contract point):

```sh
sudo__enter() {                # STRAWMAN: argv = the site's peeled sudo argv, tail =
   sudo -n "$@"                # the composed inner probe invocation. Non-interactive
}                              # by construction: -n fails rather than prompts.
```

Authoring the entry form IS the traversal vouch (the existing authoring-is-vouching
shape): the wrapper author answers for the entry's own self-effects in the read lane
(sudo: an auth-log line, a timestamp refresh — disclosed, modeled, elide-alongside per
`24S` §2b). A wrapper whose entry cost is a real mutation gets no entry form ⇒ its
contexts are never entered ⇒ fallback lane. Chains compose recursively
(`sudo chroot /t CMD` ⇒ sudo-entry(chroot-entry(inner))), every crossed axis entered
together, under the existing nesting bound; any crossed axis lacking {entry form ×
dial × vouch} ⇒ the whole site takes the fallback ladder for that boundary. Transitive
delegation *inside tool oracles* (`doas__predict() { sudo "$@" ;}`) remains unshippable
— entry lives only in the wrapper-oracle's entry-form seat, where it is licensed and
attributed.

### The degrade ladder (all failure directions safe)

Entry refused (`sudo -n` fails, password required) or impossible (chroot target
unmounted, tool absent inside the view — rc 127) or the in-context check declines
(rc≥2) ⇒ can't-say ⇒ the site guards (apply-lane, in-context, same license) or runs.
Nothing false is ever produced by a failed entry; the fallback lane (§5) may still
apply where its own licenses hold.

### Batching, cost, DST

Probe emission batches per (host, context): one entered segment per distinct context —
O(contexts) entry overhead and auth-log noise, not O(sites); exec cost dominated by SSH
RTT per the standing perf doctrine. hostsim grows context-qualified verdict injection
(the transport design needed the same seam); e2e gains two-context fixtures under inert
mocks. No cross-host or cross-run state is introduced (kSTATE untouched).

### Apply-lane guards and conditional tails

A wrapped site that measured converged-but-unlicensed, or can't-say, guards at apply:
the check invocation executes under the site's context via the same entry composition,
immediately ahead of the original bytes (which survive verbatim, always). Residual
guarded walls no longer cost the whole tail (`27C:route-conditional-tail`, carried from
`27B` §4): each guarded wall sets a flag iff its fallback body actually executed; tail
lines conditioned on it keep their probe-time elision license along the didn't-act
branch (sound: a short-circuited wall ran nothing) and degrade to their ordinary
in-position guards along the fired branch. Zero steady-state check-tax (the conditional
short-circuits before the check). This is the IDENTIFIED-CAUSE side of the TOCTOU
ruling ("hork-catching is in"); its sanctioned design home is the placement-spectrum
round; render under welded attention law: may-execute lines are never hidden — at most
dimmed, annotated "executes nothing unless line 8 acts"; any collapse into the
non-verbose fold is a future human product ruling. The `236b`-alt2 kind
generation-probes revive as the fired-branch fast path (one bracket read revalidates a
kind's crossing facts in O(1); completeness by construction of the substrate).

## §5 — The demoted transport lane (`27C:lane-transport-fallback`)

Transport — consuming a measurement taken in context X for a site in context Y under a
license — SURVIVES, demoted from primary lane to fallback, serving exactly the cells
measurement cannot reach: cell-no-shift, cell-static-identity, unenterable axes,
unvouched functions under the default dial.

Its licenses there, all flag-tier (the survival-class flag; never default, per the
standing `27A` §5 refutation of flagless kind-located licenses):

- **kind-tier `invariant:<axis>`** (grammar already in `277`): genuinely
  axis-invariant kinds — kernel-state kinds under user (a sysctl answer is the same
  for any asker) or fs-view — let an unshifted measurement answer a wrapped site.
- **oracle-tier answer-invariance** (STRAWMAN sibling spelling, same family:
  `: answer-invariant:user`): the on-subject statement "this body's ANSWER does not
  vary along the axis" — the massaged site-scoped-vouch species of `27A` §3, now
  located on the measuring body (fixing that arc's mis-location objection). Distinct
  from and orthogonal to `tolerates:` (pipx: tolerant, not invariant; a hermetic
  `/etc`-reader: both).

This lane is deliberately NOT fully designed here: it is fenced (flag-tier, on-subject,
composable with kind corroboration), its governing bar is `27A` §3's massage-conditions,
and it gets built only when the fallback cells' field pressure earns it. The `275` §6
value-transport refusals stand; register-backed value transport (analytic) stands;
world-cell values follow the same demotion (in-context capture where entered; fallback
fences otherwise).

## §6 — Mined idioms: lint-feeders only (`27C:rule-mined-idioms-advisory`)

Human-typed: "worth tracking/recognizing; do not solve the problem for us." The
inventory, kept because each is real, purposeful sh that a good author writes anyway —
consumed for hints, why-lens text, plan-time decline prediction, and mark-corroboration
lints, NEVER for licensing:

- `27C:idiom-demand-guard` — `[ "$(id -u)" -eq 0 ] || return 2` (and `id -un` named-
  user forms): the most-written defensive preamble in ops; in verdict position it lands
  on the blessed can't-say. Mining predicts declines without shipping ("check demands
  root; site provides www-data") and self-selects at runtime. Unrecognized spellings
  cost value, never correctness.
- `27C:idiom-honest-read` — visible `$HOME`/`$USER` reads
  (`test -f "$HOME/.foobar/synced" || return 2`): the per-asker tool's honest spelling;
  already the `272` who-am-I token set; keys facts to contexts defensively.
- `27C:idiom-neutralize` — `LC_ALL=C`, `GIT_CONFIG_GLOBAL=/dev/null`, `XDG_*`/`HOME`
  overrides, `env -i PATH=… tool`: hermetic-invocation habits; serve the kVOLATILES
  weld directly; shrink per-user variance but cannot close the tool-internal half
  (unset `HOME` falls back to getpwuid inside the binary).
- `27C:idiom-dependency-guard` — `command -v dpkg >/dev/null || return 2`: the
  hand-written fs-view degrade.
- `27C:idiom-subject-explicit` — `crontab -u root -l`, `git -C /srv/repo …`,
  `--root`/`--admindir` flags, absolute paths: answer-invariance written into argv.

Corroboration lints run both directions: a `tolerates:` mark on a body with visible
identity-dependence draws "are you sure?"; heavy context-handling with no mark draws
the one-line hint. The two-user differential CI (stdlib MUST) upgrades to tracer
read-set diffing (the `077` seccomp/eBPF observe lane) where the CI container allows —
catching per-asker reads even when answers coincide.

Why none of these license: each fails `27C:law-perfect-overlap` — over-trigger
(a `$USER` in a log line is not consent) or under-trigger (bashism spellings), and a
boolean rider on imperfect overlap is the forbidden middle ground.

### The axes differ in utterability, by design (`27C:rule-axis-utterability`)

`27A`'s criterion-syntactic-visibility returns as a *spelling* observation, not a
polarity rule: **user** is sh-utterable (guards, honest reads — full mined-lint ladder
plus the vouch); **fs-view** is half-utterable (dependency guards, os-release checks,
subject-explicit `--root` flags — partial lint coverage; an optional fs-aware read-arm
role-sibling for image-tooling authors is a named extension seam,
`27C:seam-fs-read-arms`, expert-tier because it breaches the `24S` §2c tool-oracles-
never-mention-wrappers referendum and must stay opt-in); **netns** has no idiom at all
— engine/wrapper/kind-tier and measurement-only, with no author-facing aspiration.

## §7 — Residual holes (who bleeds, honestly)

- `27C:hole-password-sudo` — cell-static-identity probes: wrapped sites guard at apply
  instead of eliding at plan. Bites interactive-password personal machines and
  MFA-corporate estates; best-effort tier by human ruling; conditional tails +
  generation-probes contain the drifted-day cost.
- `27C:hole-app-auth` — state behind application-level credentials no OS context
  grants (DB passwords, API tokens): can't-say forever; oracle authors read on-disk
  state or decline. No design reaches this; none ever did.
- `27C:hole-unvouched-oracles` — under the default dial, unmarked third-party oracles
  never shift: their wrapped sites run/guard with hints until the author adds one line
  (or the admin types `--escalate-any-probe`). The ordinary gradual-enhancement rung.
- `27C:hole-unmodeled-wrappers` — no wrapper-oracle, no peel, opaque wall. Unchanged;
  tiny expert authorship class.
- `27C:hole-bad-oracle-blast` — the priced trade of the default cell: a lying vouch
  mutates in-context at plan time. Attributed (three named consents), bounded,
  clamped by: cell-no-shift for sensitive machines; the CI/tracer bar where adoption
  concentrates; degrade-only containment jackets where the host offers them free
  (`systemd-run --property=ProtectSystem=strict`, seccomp observe — never
  load-bearing, `27C:seam-containment-jackets`). Weighed against the lane it replaces,
  whose failure mode was unattributable under-execution — the cardinal sin.
- The tool-internal half *within* one context — the standing `233` converged-vouch
  adequacy gap — is untouched in both directions.

## §8 — User stories (renders ILLUSTRATIVE; spellings STRAWMAN)

### Story: the babby author's sudo line (the old knife cell, defused)

Alice's book: `sudo pipx install poddle`. Her nine-line oracle, unvouched: the plan
renders the line as `runs` with the hint above. She adds `: tolerates:user` (one line,
told by the hint). Next plan, default dial, root-ish connection: the probe enters via
the sudo entry form, `pipx list` reads **root's** tree, poddle absent ⇒ `runs:
diverged` — root gets poddle. Once applied, subsequent plans elide the line. Her own
unwrapped `pipx install httpie` site continues to probe bare against her own tree. Two
sites, two contexts, two correct answers; no transport, no polarity fork, no flag.

### Story: the 24S walkthrough book, re-run

`deploy.sh` (alice on web1, `24S` §2): with the stdlib vouched and the connection able
(cell-root-reuse), lines 3–6 elide when converged exactly as `24S` Stage C hoped — and
line 8 (`sudo crontab -l … || … | sudo crontab -`), Stage D's "honestly UNPROBEABLE
from outside," becomes an ordinary measured site: the entered check reads root's
crontab. Under `--no-probe-escalation` the same book renders Stage-B-shaped: wrapped
sites run/guard, honest walls, tails conditional — the sensitive-machine admin chose
that, once, legibly.

### Story: chroot provisioning (the vicious walk, now correct in both worlds)

`chroot /mnt/target apt-get install -y openssh-server`: entered check reads the
TARGET's dpkg database ⇒ diverged ⇒ runs; the image ships sshd. Pre-mount, entry fails
⇒ can't-say ⇒ runs. The old transport design answered this wrongly almost every time
(the host nearly always has the package); the old wall answered it valuelessly.

### Story: the field-trial book (`27C:prediction-trial-walls-dissolve`)

The `255` book's two permanent walls — `su - postgres -c` (root connection: entered
demotion) and the `$(hostname)` guard (the block-context capture floor) — are both
predicted to dissolve on revival. Falsifiable first-blood: if a real host does not bear
this out, this note's headline is wrong and should be struck.

## §9 — Implementation guidance (the spike builds ALL of it)

Human ruling: all four cells get implemented so the dial exists; consent (dial state ×
vouch presence × entry availability) is traced through the code and applied to contain
blast radius. Specifics are the implementing agent's. Build-list deltas for
block-context briefs:

- wrapper-oracle contract: + entry-form role-sibling (peel already specified);
  entry forms non-interactive by construction; authoring-is-vouching for traversal.
- probe emitter: per-(host,context) segments; composed entry forms; only-oracle-bytes
  law observed (task-14 gates the composition ratification — UNCHANGED, still the one
  human checkpoint).
- mark surface: `tolerates:` (and reserved `answer-invariant:`) in the bare-mark
  grammar; swappable-stub discipline per the rul-guard-license precedent (build
  against a stub, cheap to re-spell).
- CLI: the ternary dial; the plan-header authority-disclosure line.
- hostsim: context-qualified verdict injection; e2e two-context fixtures (inert
  mocks impersonating contexts).
- lints: the §6 corroboration set; the two-user CI (+tracer diff) at the stdlib bar.

Open corners, named: `27C:open-in-book-guard-value` (how hand-written in-book guards
yield probe value under only-oracle-bytes — danced around repeatedly, still unsettled;
the defensive cell-no-shift reading of §2 applies meanwhile);
`27C:open-cell-granted-acquire-ux` (deferred mechanism); the §5 fallback-lane license
design (fenced, unbuilt); netns entry-form details (root-only, `ip netns exec` /
`nsenter` — wrapper-oracle author's problem, same seat).

## §10 — Ack-ledger (only what the human TYPED counts)

Human-typed this dialogue (2026-07-16), rulings unless later reversed:
- The four cells, in the human's own wording (§2), including cell-root-reuse as
  DEFAULT and the non-root columns as best-effort-not-primary.
- No down/up privilege ordering; open to a cheap monotonic sub-dial if one exists,
  skeptical one does ("difficult to be careful and specific of").
- Blast-radius is the gated quantity; probe/apply equivalence ("that damage isn't
  apply-time/probe-time specific in any particular way").
- The ternary dial, including `--escalate-any-probe`, and vouch-required-at-default
  ("only functions for explicitly-escalation-vouched oracles").
- The oracle-side explicit vouch must exist (the double-end), per-function-leaning
  ("shell doesn't have families"); axis-blind universal vouching rejected as
  gradual-enhancement hostile; aspire to non-dimension-blind authorship; effort moves
  to where effort already lives (kind/wrapper/stdlib tiers).
- Mined idioms: recognize, never solve with ("do not solve the problem for us");
  `27C:law-perfect-overlap` verbatim-paraphrased in §3; "that means go hard
  non-shell."
- Transport lane survives (correction of `27B`'s overclaim), scoped: measurement
  carved out "a large-ish default-case — specifically sudo, specifically
  default-mode."
- Only oracle bytes ship; wrapper-oracles author what executes; "apply-time-bytes
  never reach probe-time" (with the human's own uncertainty flag re in-book guards).
- No backwards compat exists ("there is no backwards to compat").
- Spelling/siting delegated: "Choose a strawman siting and spelling ... I won't
  bikeshed this any further."

Mine (proposal-tier until contact or ack): every STRAWMAN spelling (`tolerates:`,
`__enter`, the flag names' semantics-notes, `answer-invariant:`); the
lifted-guards-gated-too reading; conditional-tail details and its render posture; the
§5 fallback fencing; the seams (`fs-read-arms`, `containment-jackets`,
`no-root-targets-subdial`); the trial prediction.

## §11 — Consumers and re-entry

- **block-context implementation-planning** consumes this note whole; it discharges
  `279f:ask-transport-disposition` (disposition: measure-in-place primary; fallback
  lane fenced flag-tier) and carries the substance `279f:ask-flag-boundary-recut`
  needs (the dial IS the recut: outcome-centric, both-sides consent).
- **task-14** (structural-vouch re-derivation): unchanged, still gates probe-form
  composition — now including entry-form composition.
- **KNOBS**: two candidates to REPORT, not mint (KNOBS is human-authoritative): the
  escalation dial (a mode-knob; poles ~ blast-radius-containment vs wrapped-site
  value) and the tolerance vouch's place in the kCONTRACT-RUNGS story (a new,
  explicitly-opt-in rung — consistent with rungs-default since the product ships
  fresh).
- **spike/CLAUDE.md standing rulings**: `27C:law-perfect-overlap` deserves promotion
  to the rulings block (human act; it will govern every future spelled-as-sh
  decision).
- **ANALYZER-NEEDS** `an-privilege-fact`: re-scoped per §1 (annotated in place this
  pass).
- **stdlib quality bar**: two-user CI upgraded with tracer read-set diffing; the babby
  template gains `: tolerates:user` and the honest-read teaching.
- **Superseded/annotated by this pass**: `27B` (design superseded; argument stands);
  `27A` (§3 fork mooted in the default cell, re-posed flag-tier in the fallback lane;
  imp-1 re-scoped; walls re-read as transport-lane law); `24S` imp-1 and Stage-D
  line-8; `273` §6's never-shippable corner (entry forms are the licensed seat).
