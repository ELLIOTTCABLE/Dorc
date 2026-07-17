# 27C — Context-entry probing and cross-dimension consumption (THE plan)

Plan-tier, kept-current: the implementor-facing spec for how Dorc answers wrapped
sites — probing inside contexts, the two admin flags, the oracle-side vouches, and the
fallback consumption of facts across dimension boundaries. Authority: root docs,
`spike/CLAUDE.md` rulings, and `plans/271` outrank this; §10 is the status ledger
(ruled vs STRAWMAN — every sh spelling here is STRAWMAN unless §10 says otherwise).
Companions: `notes/273` (the wrapper surface this extends: `cmd__predict()` +
`cmd__lend_map()`) · `notes/272` (store topology) · `notes/27A` (the transport-lane
wall inventory, historical) · `271:rul-flag-is-razor-residue` +
`271:rul-invariance-speech-act` (the governing rulings). Vocabulary: KNOBS
named-mechanisms; `273`'s dimensions/guest (never "axes"/"tail"); dimensions in play:
user · fs-view · netns · ρ.

## §0 — The design in one screen

A wrapped site (`sudo pipx install poddle` · `chroot /mnt/target apt-get install -y x`
· `ip netns exec blue sysctl -w …`) is answered, in order of preference:

1. **Measurement in the site's denoted context** (the default lane): the probe *enters*
   the context — re-using authority the connection already holds, never acquiring — and
   runs the oracle's own body there. The fact is born in the site's context; nothing
   crosses a dimension boundary. Requires: the admin's dial (default permits) × a
   modeled wrapper (entry form) × the executed body's tolerance vouch × the measurement
   succeeding. Every failure lands on can't-say → guard/run.
2. **Cross-dimension consumption without entry** (the fallback lane, for cells entry
   cannot serve): an unshifted measurement answers a wrapped site only via
   (a) **pure-predicate carry** (unflagged; substrate axes only) — a fact travels iff
   its marked backing kinds carry their owner's `invariant:<axis>` line (A) AND the
   engine proves the verdict body *read-set-closed*: everything influencing the verdict
   traces to the site's argv or a marked read, with no unmarked external input (B). The
   engine closes the "and nothing else" itself, so there is no human at-most claim and
   no flag; or
   (b) for ingredient (identity) axes, the kind-owner's typed invariance line × the
   admin's `--risk-faultless-skips`.
   Silence walls. A fact travels unflagged only where the engine has closed it (a);
   everything else walls.
3. **Guard at apply** (in-context, same license as 1), then **run** — with conditional
   tails containing the guard-cascade, so one wrapped wall no longer costs the book's
   tail its shape.

Two admin flags exist in this territory, and only two:

- the **escalation dial** (ternary; gates oracle-code blast-radius; §1), and
- **`--risk-faultless-skips`** (existing; `271:rul-flag-named-risk-faultless-skips`;
  gates every outcome resting on what no line can say — the survive tier, and the
  fallback lane's *ingredient-axis* residue (substrate axes are engine-closed via
  pure-predicate carry, unflagged; §4(a))). Never keyed to claim-types
  (`271:rul-no-claim-type-gating`); never a default; per-invocation.

Security story in one sentence: the probe lane never acquires authority — it re-uses
what the connection holds, only for oracle bodies whose authors accepted
context-shifted execution, only where the dial permits; every shifted execution and
every traveled fact is attributed to the parties who consented.

## §1 — The escalation dial (admin surface)

**The gated quantity is damaging-oracle blast-radius** — a mutating "check" is equally
damaging at probe time and inside an apply-lane guard, so the dial gates oracle-code
context-shifting in BOTH lanes (ruled). Book bytes at apply are untouched in every
position (the book escalating itself is not Dorc's act). ρ-only wrappers (`env`,
`nice`) shift no identity or view and sit outside the dial.

The authority rule (`27C:rule-reuse-never-acquire`, ruled): the probe lane exercises
only authority the *connection* already holds — a root connection performs user-shifts,
`chroot`, and netns entry with zero new credentials; a non-root connection performs
none of them. No prompting, no credential handling, no password lanes in the probe
path. There is **no privilege ordering** (`27C:rule-no-privilege-order`, ruled):
"demotion" is not a safety category (peer-auth grants postgres what root-qua-root is
refused; NFS root-squash makes root read *less*; lateral shifts route through the
superuser mechanically) — the only implementable predicate is *can the connection do it
with zero new credentials*.

The dial (names STRAWMAN, human-uttered):

- `--no-probe-escalation` — no oracle code ever executes under a shifted context, in
  either lane. Wrapped modeled sites run (an out-of-context check is wrong-world and
  licenses nothing). Maximally defensive, for sensitive machines. Chosen-defensive
  reading (cheap to flip): this also gates probe-time execution of *lifted guard
  material* whose recognized argv-shape sits under a wrapper.
- `--probe-escalation` — THE DEFAULT: shifts licensed **only for functions carrying
  the tolerance vouch** (§2). Both-sides consent: author's mark × admin's default.
- `--escalate-any-probe` — shifts licensed for unmarked oracles too; the admin
  knowingly overrides absent author consent and owns the blast-radius alone.

The four operational cells consent must be traced through (all four implemented,
ruled): root-connection + dial-permits (default) · root-connection +
`--no-probe-escalation` · non-root + explicit acquisition mechanism (opt-in; UX
deferred, `27C:open-cell-granted-acquire-ux` — sketch: a pre-probe, pre-ack, one-shot
`sudo -v`-class moment, credential never stored) · non-root + nothing (the residue;
best-effort tier by ruling, not a primary goal).

Consent legibility (`27C:render-authority-disclosure`): the plan header discloses, in
one line, which contexts the probe will enter and under what — "probe re-uses
connection authority (root): →postgres (1 site), chroot /mnt/target (2); forbid with
--no-probe-escalation".

A "shifts-except-to-uid-0" sub-dial is implementable nearly free at the peel but only
coarsely meaningful; field-demand-contingent, not built
(`27C:seam-no-root-targets-subdial`).

## §2 — The tolerance vouch (oracle surface)

`27C:vouch-tolerates` asserts, for exactly the function carrying it: **"this body's
effects are read-only by design, not by privilege-starvation — executing it in a
context shifted along the named dimensions will not mutate."** The content is real and
author-held: a check can be non-mutating as an unprivileged user only because its
opportunistic writes fail on EACCES (`pipx list` scaffolds `~/.local/pipx` when it can;
tools refresh caches and locks where permitted). The vouch is the strong property,
typed knowingly.

It does NOT claim: anything about the *answer* (answers are supposed to differ per
context — that is the point of measuring in place); answer-invariance (§4's separate,
stronger territory); anything about other functions; any change to what verdicts
license at the tool's sites.

Spelling and siting (STRAWMAN): a bare-mark statement in the settled sigil family,
inside the function body, reachability-scoped like every in-body act (place it in a
`case` arm to vouch per-verb); strips whole-statement; per-dimension via
brace-alternation (`: tolerates:{user,fs-view}`):

```sh
pipx__is_converged() {
   : tolerates:user                          # STRAWMAN spelling; the mark is the vouch
   verb="$1"; shift
   case "$verb" in
   install) pipx list --short 2>/dev/null | grep -q "^$1 " ;;
   *) return 2 ;;
   esac
}
```

Per-function always (sh has no family construct; no file-scope ceremony exists and none
is added); a two-member oracle marks each member it wants shiftable. Per-dimension
always (ruled: universal "safe anywhere" vouching is gradual-enhancement hostile) — the
babby template carries `: tolerates:user`; fuller sets live where the effort already
lives (stdlib, wrapper, kind tiers). The author's real envelope under the default dial
is *the contexts this book's own wrapper chains denote on this host* — the same
contexts a hand-written guard for those sites would have run in; the universal envelope
is a published-oracle concern, policed by the stdlib quality bar (two-user differential
CI; tracer read-set diffing where available).

Why a minted mark, not a mined idiom — `27C:law-perfect-overlap` (ruled, general): *a
boolean may be spelled-as-sh only when the sh-behaviour and the licensed meaning
overlap perfectly — never wanted apart, never apart wanted.* No candidate idiom meets
the bar (§6); per be-sh-or-be-very-not-sh, the spelling goes hard-non-shell.

Hints drive adoption: an unvouched function under a wrapped site renders run/guard with
"line 8 would elide if pipx's oracle vouched context-tolerance (one line:
`: tolerates:user`)".

## §3 — Entry and composition (engine mechanics)

**Only oracle bytes execute in the probe lane** (`271:rul-only-oracle-bytes-ship`,
RATIFIED law): the shipped form for a wrapped site is the composed-oracle form with the
context made real — the wrapper-oracle's **entry form** wrapping the inner oracle's
body, invoked with the site's argv. The site's wrapper chain *selects* which contexts
are entered; every executing byte is oracle-authored. In-book hand-written guards
contribute recognition only — **argv flows, bytes do not**: a lifted guard is the
oracle's predict invoked with the book's argv.

The entry form is a wrapper-family member alongside `cmd__predict()` and
`cmd__lend_map()` (`notes/273`; member name STRAWMAN):

```sh
sudo__enter() {                # argv = the site's peeled sudo argv with the composed
   sudo -n "$@"                # inner probe invocation in guest position.
}                              # Non-interactive by construction: -n fails, never prompts.
```

- `cmd__predict()` remains the read-only *model* — its closure bodies never contain
  real escalation; that law stands. `cmd__enter()` is the ONE licensed seat for real
  context entry, existing only for this lane, shipped only under dial × vouch.
  Authoring it is the traversal vouch (authoring-is-vouching): the wrapper author
  answers for entry self-effects in the read lane (sudo: an auth-log line, a timestamp
  refresh — modeled, elide-alongside). A wrapper whose entry cost is a real mutation
  authors no entry form ⇒ its contexts are never entered.
- **Siting is part of the vouch** (`27C:rul-entry-denoted-siting-vouch`, ruled
  2026-07-17): authoring an entry form also vouches that entering via it lands the
  guest in the site's *denoted* context. Policy-matching wrappers (sudo, doas) route
  on the full command line including the guest, so the entry invocation (probe in
  guest position) may match a different rule — different runas/CWD/CHROOT — than the
  site's own bytes would; measured-in-the-wrong-world verdicts are the cardinal-sin
  shape entering through the primary lane. Where the author cannot verify correct
  siting — by structural insensitivity (chroot/env/mise fix context from their own
  argv, satisfying the clause for free), by interrogating the tool (`sudo -n -l`
  -class policy reads, self-vouched ordinary body-code), or by a standalone
  same-head tripwire invocation — the form declines: unverifiable ⇒ rc≥2 ⇒ can't-say
  ⇒ guard/run. Entry-form bodies stay ordinary pre-entry sh ending in `"$@"` verbatim
  in command position; the not-in-command-position variant (an authored in-guest
  preamble wrapping `"$@"`) is PUNTED — complex policy falls to the correct floor,
  the value picked up later if warranted.
- An unmodeled wrapper never peels ⇒ opaque line, wall (unchanged law). Unauthored
  entry cannot misfire because it does not exist.
- Chains compose recursively (`sudo chroot /t CMD` ⇒ sudo-entry(chroot-entry(inner))),
  all crossed dimensions entered together; any crossed dimension lacking
  {entry × dial × vouch} ⇒ the site takes §4/§5 for that boundary. Transitive
  delegation inside tool-oracle bodies (`doas__predict() { sudo "$@" ;}`) stays
  unshippable.
  - **The composition algebra** (per `27Xf:cr-nested-wrapper-composition-rider-dropped`
    — the `279f` §5 rider this plan dropped, restated as build-spec; ruled cells
    human-typed 2026-07-17; supersedes the interim block on the impl branch): lend/ρ
    composition across a peel chain is the POINTWISE fold, outermost-first, per
    dimension, each link's `lend_map` evaluated against that link's own peeled argv.
    - Identity element = full lend (`compose(x, full) = x`): a chain of all-full links
      (`nice`, ρ-only wrappers) denotes ambient.
    - **⊤ propagates** (`27C:rul-top-absorbs-absolute-maps`, HUMAN-ACKED 2026-07-17):
      a MISSING key at ANY link ⇒ ⊤ for that dimension for the whole chain — one
      silent link walls the dimension, never inherits a neighbor's lend, and there is
      NO overwrite-rescue through an inner ABSOLUTE map (an inner `sudo -u root` under
      an unknown middle stays ⊤: whether the inner link even succeeds, and how its
      argv resolved, both depend on the unknown middle). Skip-the-middle reasoning may
      hold in raw static sh-analysis; it never holds in machine-state logic (human's
      framing, verbatim intent).
    - **Mapped-lend composition is dimension-owned, engine-internal**
      (`27C:rul-dimension-owned-compose-ops`, HUMAN-ACKED 2026-07-17): each dimension
      fixes ONCE, in the engine — never on the authored surface — both its compose op
      and the FRAME of the emitted value in the `lend_map` contract. user: absolute
      overwrite (`sudo -u alice` denotes alice whoever the caller was). fs-view:
      caller-relative (chroot-in-chroot composes by path — `chroot /t` inside
      `chroot /mnt` denotes `/mnt/t`; relative-to-⊤ is ⊤). A wrapper author emits a
      single-step string per `273` §3 and never reasons about nesting;
      `inv-referent-agnostic` holds (the engine applies the dimension's op to opaque
      values — the value's meaning was asserted by the wrapper oracle, its frame by
      the dimension contract, never inferred from the tool).
    - **Cross-link ρ-threading**: each inner link's argv is ρ-resolved under the ρ
      composed so far (`env DBUSER=postgres sudo -u "$DBUSER"` resolves through the
      outer link's claimed ρ); an unresolvable value stays the ⊤-value
      (identifies-with-nothing, `273` §3). The fold is pointwise in that dimensions
      never mix — EXCEPT ρ, which threads through every link's argv-resolution by
      construction.
    - Canonical context key = the folded per-dimension normal form, never the chain's
      syntax — order-sensitive exactly where the folds genuinely differ
      (`sudo -u postgres nice cmd` and `nice sudo -u postgres cmd` share one key;
      `env A=1 sudo cmd` and `sudo env A=1 cmd` do not — sudo scrubs). Batching and
      fact-keying both consume this key.
    - **Fold-entry coherence** (`27C:rul-fold-entry-coherence-failfast`, HUMAN-ACKED
      2026-07-17, scope-narrowed in the acking dialogue): where `lend_map` and the
      entry form STATICALLY disagree — peel-position divergence (the two members'
      `"$@"` reach different tail positions) or argv-flow divergence (the entry body
      visibly drops/transforms args the fold consumed) — plan-time fail-fast, the
      declarations-genuinely-contradict category (dual-peel pattern, third instance).
      The check's reach is exactly sh-structure: whether an entry invocation actually
      EFFECTS the declared dimension-shifts is tool-semantics the engine never holds
      (`inv-referent-agnostic`) — that claim is part of the traversal vouch
      (authoring-is-vouching; wrong ⇒ attributed authored error, the
      `hole-bad-oracle-blast` species), never statically detected. The coarse case is
      already structural (a declared-crossed dimension with NO entry member ⇒ §4/§5,
      member-existence); runtime corroboration is lint-tier only, following §6's
      dimension-utterability ladder (a who-am-I read corroborates user in-context;
      netns has no utterance). Corroborate-and-warn, never license, never fail-fast.
    - DST pins: the nested permutations above · chroot-in-chroot path composition ·
      ⊤-in-the-middle poisons through an inner full lend AND an inner absolute map ·
      unresolvable `sudo -u "$VAR"` ⇒ ⊤-value key, walls.
- Degrade ladder, every direction safe: entry refused (`sudo -n` failure), impossible
  (chroot target unmounted), missing dependencies inside the view (rc 127), or an
  in-context decline (rc≥2) ⇒ can't-say ⇒ guard/run. Identity-demanding checks
  self-select at runtime (§6's demand-guard idiom); the engine never synthesizes
  authority a site's chain doesn't denote.
- Batching: one entered segment per (host, context) — O(contexts) entry overhead and
  auth-log noise; wall-clock dominated by SSH RTT per standing perf doctrine.
- DST: hostsim grows context-qualified verdict injection; e2e grows two-context
  fixtures under inert mocks; kSTATE untouched (no cross-host/cross-run state).

## §4 — Cross-dimension consumption without entry (the fallback lane)

For cells entry cannot serve (`--no-probe-escalation`, the non-root residue, unvouched
functions, unenterable dimensions), an *unshifted* measurement may answer a wrapped
site only through these:

- **Pure-predicate carry across a substrate axis** (unflagged; the opted mechanism,
  human-opted 2026-07-17; `27C:mech-pure-predicate-carry`). A fact measured in the
  ambient context travels across a substrate boundary (fs-view, netns) — *unflagged* —
  iff BOTH hold:
  - **(A) authored axis-invariance:** every *marked* backing kind of the fact carries
    its owner's `invariant:<axis>` line (`277` §4e — the same surface (b) uses,
    generalized to substrate axes). Authored, attributable: a wrong line is the
    kind-owner's pointable error (vouch-species), never the flag's naked at-most.
  - **(B) engine-verified read-set closure:** the engine proves, by conservative
    sh-taint over the verdict body, that everything influencing the verdict rc (data
    AND control-flow — branch conditions included) traces to either the site's argv
    (flowing as argv through the author's argparse, `271:rul-argv-flows-bytes-do-not`)
    or a marked-observe read — with *no unmarked external input* anywhere. The engine
    reads the author's *marks* and the *sh structure*, never a tool's semantics
    (`inv-referent-agnostic` holds): an unmarked command-substitution, an untracked env
    read, an opaque call, a clock/RNG touch each DISQUALIFY. Default-disqualify on any
    construct not on the audited pure-construct safe-list; the pass fails safe (a
    missed-safe body loses its elision, never carries a hidden read).

  (B) *closes* the completeness residue `279f` §3 refused to hand the flag: it removes
  the open-world "and nothing else" by structure rather than accepting it, so the carry
  is unflagged-sound *modulo the analyzer's conservative pass and the kind-owner's line*
  — the same trust tier as the guard-lift and dead-branch-omit, both already unflagged
  (the flag is for HUMAN at-most claims; this residue is engine-closed, not accepted).
  It lifts a shape authors already write — the hermetic, argument-driven predicate
  (`[ "$(sysctl -n "$1")" = "$2" ]`: expected value in argv, live value from the
  invariant substrate) — and correctly walls the shape that hides an ambient read
  (`want=$(cat "/etc/policy/$1"); [ "$(sysctl -n "$1")" = "$want" ]`), because that
  body genuinely depends on the shifted filesystem. **Scope: substrate axes only.** The
  user/identity dimension is EXCLUDED — a user-shift changes *access* to the very reads
  the body performs (a structurally-closed body can still flip its answer under EACCES),
  so (B) does not certify it; user rides (b) or the primary enter-context lane.
  **netns caveat:** `net.*` sysctls are per-netns, so the substrate/kind model must
  carry that distinction — an owner must not, and the model should not let them, claim
  `invariant:netns` for network kernel state; fs-view has no analogue.
- **The kind-owner's invariance line × `--risk-faultless-skips`**: for ingredient-borne
  dimensions, consumption-across requires the kind-owner's *explicit positive*
  mark-line inside `kind__state_stored_only_in()` (settled grammar; STRAWMAN token
  `: user-invariant`). The line is the sayable component — when false it is the
  attributable wrong line (vouch-species), and the `272` §3 who-am-I derivation runs
  as a CONTRADICTION-CHECKER against it (typed invariance + identity ingredients in
  one body = declarations-genuinely-contradict fail-fast), never as a license. What
  the line cannot say — that a foreign measuring body's answer depends on *nothing
  beyond* that store (backing marks carry no completeness burden, `24D`; the `279f`
  §3 finding) — is faultless anatomy, so by `271:rul-flag-is-razor-residue` ("claims
  own what lines can say; the flag owns what no line can say") the composed outcome
  rides ONLY under `--risk-faultless-skips` — the same one flag, absorbing this form
  by outcome-class. When it bites, the bite is the flag's named product: a skip with
  no single human fault; the next plan re-measures and self-heals; version-skew on
  invariance lines is MH2's named tightening.
- **Otherwise: wall.** Silence licenses nothing; derivation keys and hints but never
  licenses (never-derive-separation stands); no per-dimension permissive default
  exists anywhere.

Recorded v1-defer option if even this ceremony is unwanted: honest-walls-for-worlds
(no cross-dimension consumption at all). Values follow the same shape:
register-backed value transport is analytic and stands; world-cell values are captured
in-context where entered, and otherwise sit behind the same fallback fences.

## §5 — Guards and conditional tails (residue containment)

A wrapped site that cannot elide guards at apply: the entry-composed check invocation
immediately ahead of the original bytes (which survive verbatim, always), under the
same dial × vouch license. Residual guarded walls no longer cost the whole tail
(`27C:route-conditional-tail`): each guarded wall sets a flag iff its fallback body
actually executed; tail lines conditioned on it keep their probe-time elision license
along the didn't-act branch (sound: a short-circuited wall ran nothing) and degrade to
ordinary in-position guards along the fired branch. Zero steady-state check-tax (the
conditional short-circuits before the check runs). This is the IDENTIFIED-CAUSE side of
the standing TOCTOU ruling (hork-catching is in); detailed design belongs to the
placement-spectrum round; render under welded attention law (may-execute lines never
hidden — at most dimmed, "executes nothing unless line 8 acts"; any fold into the
non-verbose count is a future human product ruling). The `236b`-alt2 kind
generation-probes revive as the fired-branch fast path (one bracket read revalidates a
kind's crossing facts in O(1)).

## §6 — Mined idioms: lint-feeders only

Ruled: recognize, never license. Each of these is purposeful defensive sh, consumed for
hints, why-lens text, decline prediction, and mark-corroboration — and each fails
`27C:law-perfect-overlap` as a license (over-triggers on incidental text, under-triggers
on spelling variants; only lint-tier tolerates that):

- `27C:idiom-demand-guard` — `[ "$(id -u)" -eq 0 ] || return 2` (the ubiquitous
  preamble; in verdict position lands on the blessed can't-say; self-selects at
  runtime; mining it predicts declines without shipping).
- `27C:idiom-honest-read` — visible `$HOME`/`$USER` reads (the per-asker tool's honest
  spelling; the `272` who-am-I token set; keys facts defensively).
- `27C:idiom-neutralize` — `LC_ALL=C`, `GIT_CONFIG_GLOBAL=/dev/null`, `XDG_*`/`HOME`
  overrides, `env -i` (hermeticity habits; serve kVOLATILES; cannot close the
  tool-internal half — unset `HOME` falls back to getpwuid inside the binary).
- `27C:idiom-dependency-guard` — `command -v dpkg >/dev/null || return 2` (the
  hand-written fs-view degrade).
- `27C:idiom-subject-explicit` — `crontab -u root -l`, `git -C`, `--root`/`--admindir`,
  absolute paths (answer-independence written into argv).

Corroboration lints run both directions: a `tolerates:` mark over visible
identity-dependence → "are you sure?"; heavy context-handling with no mark → the
one-line hint. Dimension utterability differs by design and the lint ladder follows it:
user is sh-utterable (full ladder) · fs-view half-utterable (dependency/os-release
guards; an optional fs-aware read-arm for image tooling is a named expert-tier
extension, `27C:seam-fs-read-arms`, opt-in because it breaches tool-oracle
wrapper-blindness) · netns not utterable (engine/wrapper/kind tier only).

## §7 — Residual holes (who bleeds)

- `27C:hole-static-identity` — non-root, no acquisition: wrapped sites guard at apply
  instead of eliding at plan. Best-effort tier by ruling; conditional tails +
  generation-probes contain the drifted-day cost.
- `27C:hole-app-auth` — state behind application-level credentials no OS context
  grants: can't-say forever; authors read on-disk state or decline. No design reaches
  this; none did.
- `27C:hole-unvouched-oracles` — under the default dial, unmarked oracles never shift;
  hint-driven one-line fix, or `--escalate-any-probe`.
- `27C:hole-unmodeled-wrappers` — no oracle, no peel, wall. A tiny expert authorship
  class fixes each once.
- `27C:hole-bad-oracle-blast` — the default cell's priced trade: a false tolerance
  vouch mutates in-context at plan time. Attributed (three named consents), bounded;
  clamps: `--no-probe-escalation`, the CI/tracer bar, degrade-only containment jackets
  where the host offers them free (`27C:seam-containment-jackets` — never
  load-bearing). The default path adds ZERO new under-execution risk; every
  under-execution risk in this plan lives behind `--risk-faultless-skips` or in the
  pre-existing converged≠no-op adequacy gap (`233`).

## §8 — Worked stories (renders illustrative; spellings STRAWMAN)

- **Babby sudo:** `sudo pipx install poddle`, unvouched oracle → runs, hint. The author
  adds `: tolerates:user` → next plan enters via sudo's entry form; `pipx list` reads
  ROOT's tree → diverged → runs; after apply, elides. Their unwrapped `pipx install
  httpie` site probes bare against their own tree. Two sites, two contexts, two correct
  answers; nothing traveled, no flag typed.
- **Chroot provisioning:** `chroot /mnt/target apt-get install -y openssh-server` — the
  entered check reads the TARGET's dpkg database → diverged → runs (the image gets
  sshd); pre-mount, entry fails → can't-say → runs. Correct in both world-states.
- **Sensitive estate:** the same book under `--no-probe-escalation`: wrapped sites run,
  honest walls, tails conditional — chosen once, legibly, in the flag whose name says
  what it protects.
- **Trial prediction** (`27C:prediction-trial-walls-dissolve`, falsifiable): the `255`
  book's two permanent walls (`su - postgres -c`; the `$(hostname)` guard via the
  block-context capture floor) dissolve on revival. If a real host disagrees, this
  plan's headline is wrong — strike it.

## §9 — Build list and open corners

All four dial cells implemented (so the flags exist); consent (dial × vouch × entry
availability) traced through the code; specifics are the implementing agent's. Deltas
for block-context briefs:

- wrapper-oracle contract: + the entry-form member (non-interactive by construction;
  authoring-is-vouching for traversal; predict closure bodies still never escalate).
- probe emitter: per-(host,context) segments; entry composition rides the ratified
  `271:rul-only-oracle-bytes-ship` law and its build riders.
- mark surface: `tolerates:` in the bare-mark grammar; the kind-side invariance line
  in `state_stored_only_in()` (task-12 grammar batch); swappable-stub discipline for
  both spellings.
- CLI: the ternary dial; `--risk-faultless-skips` extended to the fallback lane's
  outcomes; the plan-header authority-disclosure line.
- hostsim: context-qualified verdict injection; two-context e2e fixtures under inert
  mocks.
- lints: the §6 set; the stdlib bar gains the two-user differential CI (+ tracer
  read-set diffing where the container allows); + the siting-corroboration nudge — a
  policy-matching wrapper's entry form with no visible siting verification → hint.
- stdlib sudo entry form (siting-vouch discharge; spellings STRAWMAN): the `-l`
  complexity gate (`sudo -n -ll` showing any per-command routing attributes —
  `CWD=`/`CHROOT=`/role — ⇒ decline; blanket-rule hosts pass) + a standalone
  same-head tripwire (a separate `sudo -n <probe-head> <uid-check>` ahead of entry,
  spelled with the SAME command word as the composed probe so path-granular rules
  route both alike; landed-as-wrong-uid ⇒ decline). Known punt riding
  `27C:rul-entry-denoted-siting-vouch`: the tripwire's routing-equivalence couples
  to the engine's shipped probe-head spelling; no compat surface is minted for it
  now — if the head changes, the stdlib tripwire is re-authored alongside.
- **read-set closure pass (§4(a)-(B); the spike's load-bearing validation):** the
  conservative sh-taint that decides read-set closure over a verdict body —
  default-disqualify, an audited pure-construct safe-list, taint over data AND
  control-flow. Build it in the spike and prove it in practice before any
  battle-hardening. DST battery: a closed body (`[ "$(sysctl -n "$1")" = "$2" ]`)
  carries across fs-view · a straddler (unmarked `$(cat …)`) walls · a marked read of a
  non-invariant kind walls · env / opaque-call / process-subst / clock inputs each
  disqualify · a branch condition that reads a varying store disqualifies · `net.*` not
  carried across netns · the empty-oracle world is byte-identical (no closure attempted).

Open corners: `27C:open-cell-granted-acquire-ux` (acquisition mechanism, deferred) ·
the §4(a) pure-predicate carry is the opted substrate-fallback mechanism (`27Xf`
Tier-1), its closure pass the spike's to build-and-prove; honest-walls-for-worlds
remains the degradation if the pass is not yet built · netns
entry-form details (root-only; `ip netns exec`/`nsenter`; the wrapper-author's seat) ·
conditional tails' render fold (human product ruling, later) ·
`27C:law-perfect-overlap` wants promotion into the standing-rulings surface (human
act).

## §10 — Status ledger

RULED (human-typed; durable homes cited): the four cells + reuse-never-acquire + the
ternary dial + vouch-required-at-default (this plan, 2026-07-16) · blast-radius
lane-equivalence (ditto) · no privilege ordering (ditto) · the tolerance vouch exists,
per-function, per-dimension; universal vouching rejected (ditto) · mined idioms are
lint-feeders only; `27C:law-perfect-overlap` (ditto) · one trust flag, outcome-scoped:
`271:rul-flag-is-razor-residue` · `271:rul-flag-named-risk-faultless-skips` ·
`271:rul-no-claim-type-gating` · kind invariance is an explicit speech-act; derivation
keys/contradiction-checks, never licenses: `271:rul-invariance-speech-act` ·
only-oracle-bytes, argv-flows-bytes-do-not: `271:rul-only-oracle-bytes-ship`
(RATIFIED) · non-root columns best-effort (2026-07-16) · **pure-predicate carry** is
the opted substrate-fallback mechanism (human-opted 2026-07-17;
`27C:mech-pure-predicate-carry`, `27Xf` Tier-1): unflagged carry gated on authored
axis-invariance (A) + engine-verified read-set closure (B); substrate axes only, user
excluded; the old engine-warranted-unflagged carried-by row is RETIRED (it rested on
tool-semantics the engine may not hold). The conservative-closure pass is the spike's
obligation to discharge and prove in practice — a correctness surface, not a nicety.
· **entry-form siting vouch** (human-acked 2026-07-17;
`27C:rul-entry-denoted-siting-vouch`, `27Xf:cr-sudo-entry-not-guest-insensitive`
discharged): authoring an entry form vouches correct siting in the site's denoted
context — verify (structural insensitivity · tool interrogation · same-head
tripwire) or decline; discharge is authored body-code, never engine machinery;
`"$@"` verbatim in command position is the ONLY entry shape (in-guest preamble
variants PUNTED — complex policy falls to guard/run).
· the §3 composition-algebra ruled cells (2026-07-17):
`27C:rul-top-absorbs-absolute-maps` (⊤ propagates uniformly; no overwrite-rescue —
machine-state logic never skips the middle) · `27C:rul-dimension-owned-compose-ops`
(compose ops + value-frames are engine-internal, fixed once per dimension; the
authored surface is unchanged single-step `lend_map` emission) ·
`27C:rul-fold-entry-coherence-failfast` (static argparse/argv-flow divergence between
`lend_map` and the entry form ⇒ plan-time fail-fast; entry EFFECTIVENESS is
traversal-vouched, never detected — corroboration lint-tier per the §6 ladder).

STRAWMAN (swappable stubs; conductor's): every spelling and member/flag name —
`tolerates:`, `__enter`, the dial names, the `user-invariant` token's placement
details · the sudo siting-discharge spellings (§9: the `-l` gate + same-head
tripwire) · the lifted-guards-gated-too reading of `--no-probe-escalation` ·
conditional-tail mechanics and the generation-probe revival · the seams
(`fs-read-arms`, `containment-jackets`, `no-root-targets-subdial`) · the trial
prediction.
