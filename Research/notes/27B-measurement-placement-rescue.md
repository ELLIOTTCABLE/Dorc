# 27B — Outside review: the 27A dead-end refuted (measurement placement over claims transport)

AI-authored (Fable, clean-context outside review, 2026-07-16; human-directed "rescue"
brief: distrust the corpus's own conclusions, refute `27A` if refutable). Authority: root
docs and human-TYPED rulings outrank this; **nothing herein is acked — the ask-ledger is
§7 and starts empty.** Product vocabulary per `27A` (products A/B/C/D). Certainty markers
throughout per house style.

## §0 — Verdict in one paragraph

`27A` is a sound negative result about the wrong lane. Its walls, its license-gap
analysis, and its trilemma are (I'm +SURE) correct *for claims-based transport* — the
design family where a measurement taken in context X is licensed, by somebody's authored
claim, to answer for context Y. The dead-end conclusion then follows only via an
unstated premise: that claims-transport is the ONLY route to product A under wrappers.
It is not. Two routes sit outside every box `27A` draws, both cheaper than the machinery
they replace: **(1) move the measurement into the site's own context** — ship the
already-vouched check under the book's own wrapper bytes, exercising only authority the
invoker demonstrably holds (`27B:route-probe-in-context`, §3); and **(2) let residual
elisions condition on guard outcomes at runtime** — the guard-cascade lazily
materialized, killing the whole-book-tier severity claim (`27B:route-conditional-tail`,
§4). Route 1 dissolves the trilemma in every measurable cell (sound + minimal-oracle +
unflagged, simultaneously); route 2 contains the damage in the rest. The §3 user-axis
fork — the "one outstanding typed decision" — becomes moot in measured cells and
default-walled in unmeasured ones, which is the answer `279f:ask-transport-disposition`
was missing: not (a) defer, (b) new ceremony, or (c) closable-read-sets, but **(d) stop
transporting; measure in place.**

## §1 — What 27A actually proves (conceded, steelmanned)

Before the refutation, the concessions — these stand, and nothing below relitigates them:

- con-license-gap: the `:?` backing mark carries no completeness burden (`24D`), so *no*
  authored positive disclosure can license cross-context consumption. +SURE, and
  permanent (the 233-tier impossible bar).
- con-tool-internal-half: the frame problem's tool-internal half (per-asker state
  consulted inside a binary) is invisible to any analysis, forever. +SURE.
- con-trilemma-as-scoped: over designs where the measurement stays out-of-context and a
  claim carries it across, {sound-by-default · minimal-oracle-stays-minimal · unflagged
  product A} — pick two. +SURE. Note the wording `27A` §1 itself uses: "any
  **claims-based** design picks two." That quantifier is load-bearing; see §2.
- con-transport-default-wall (fs-view, netns): correct as the *transport* polarity —
  a measurement demonstrably taken in the wrong world must not cross by default.
- con-refuted-list: everything in `27A` §5 stays refuted. Neither route below appears in
  it (verified item-by-item; the nearest miss is the apply-start wave, distinguished in
  §4).

## §2 — The frame errors (why a sound analysis reached an unsound conclusion)

### finding-imp-one-unwelded — the load-bearing invariant was never ruled

Every step of the descent stands on imp-1: "Probes never escalate. The engine knows
privilege, never acquires it" (`24S` §0). Provenance audit:

- `24S` is proposal-tier, acked "explicitly *for* hands-on-experimentation purposes, not
  as settled design" (its own header). imp-1 cites `an-privilege-fact` — an S-grade
  ANALYZER-NEEDS row whose own text carries the escape: "r23 addendum: lane-PRIVILEGE (a
  vouch promotes oracle code into the book's elevated context) — **deferred** surface"
  (`23J`). Deferred, not refuted. imp-1's other cited ground is `kFAIL-withhold` — which
  welds *never mutate in the probe lane*, and says nothing about privilege. Read-only
  and unprivileged are different axes; the threat model's architecture line ("probe
  (read-only) ─then─ apply (root)", `102`) fused them into one picture, and three
  documents later `27A` §1 calls the fusion a "standing invariant."
- The root docs promise probe **non-mutation, modulo vouches** — "vouched-safe-to-run /
  non-mutative" (DESIGN); "probing requires an oracle to vouch that a check is safe to
  run in the read-only probe phase" (USER_STORY); "plan stage doesn't mutate, as long as
  your handwritten oracles don't" (DESIGN priorities). I'm +SURE, from a full read, that
  **no root-doc sentence promises an unprivileged probe.** The prohibition lives
  entirely in AI-tier text.
- The corpus itself has already demoted it: `273` §6 — "imp-1 ('probes never escalate')
  becomes EMERGENT — enforced by the oracle-lane read-only contract + the task-10b
  effect-check, never by engine name-knowledge." An emergent property of one design is
  not a fence around all designs.
- The built system already crosses the line's spirit in the apply lane:
  wall-guard-without-escalation ships the oracle's own sh under the book's own `sudo`,
  as root, on every guarded apply — priced in `27A` §2 as a "marginal-risk note," and in
  the threat model as E4 ("a malicious/buggy oracle runs with full privilege on apply" —
  priced, mitigated). The same bytes, the same privilege, minutes earlier, is a timing
  delta on an already-accepted exposure — not a new species of risk.

### finding-trilemma-scope — the impossibility was proven for a lane, applied to the product

`27A`'s trilemma sentence self-limits: "any **claims-based** design picks two." A design
that never consumes a cross-context claim — because the measurement executes in the
site's own context — is outside the quantifier. The descent explored the claims lane
exhaustively (polarity tables, vouches, invariance declarations, closures) and, finding
it unsound, concluded the *product* was cornered. The measurement lane was fenced off
three documents earlier by finding-imp-one-unwelded, so it was never re-examined when
the claims lane died. Classic lock-in of exactly the shape `kLOCKIN` exists to name.

### finding-cascade-overstated — the whole-book-tier severity assumes a static plan

"One guarded `sudo` line degrades the entire tail to guard/run — product B for the rest
of the book, every apply, forever" (`27A` §1). Two overstatements:

- The survival tier already existed for this: a guarded wall with a declared
  `disturbs`-footprint licenses disjoint downstream survivals under the standing flag —
  same machinery as running walls. "Forever" is the no-footprint, no-flag cell.
- More fundamentally, the plan need not decide the tail pessimistically at plan time —
  see `27B:route-conditional-tail` (§4). A guarded wall is a *may*-run wall; "may" is
  resolvable at apply time by the guard's own outcome, and the tail can be compiled to
  condition on it. The cascade is real only per-apply-where-the-wall-actually-fires,
  which in the steady state (the product's own stated dominant case) is never.

### finding-premise-unmeasured — "sudo wraps most mutating lines" was never sized

The whole-book severity multiplies by wrapper density, which is asserted, not measured
(~SUSPECT unmeasured: `24S` §0's evidence is qualitative — per-line sudo on personal
machines, one `su -` in the trial book, shellcheck teaching `sudo sh -c`). Three
deployment regimes matter, and the problem lives in only part of one:

- **Root-connected professional ops** (the incumbent norm: Ansible `remote_user: root` /
  global become; every agent-based tool runs as root): the probe context IS root.
  User-axis wrappers in such books point *downward* (`su - postgres`, `sudo -u www`) —
  and entering a demoted context **acquires nothing**; it is consistent even with imp-1
  as written. The r25 trial itself ran root (`26A`); its `su - postgres -c` "permanent
  wall" was never invariant-forced — it was unbuilt machinery (probe-form composition,
  `273` §6, gated on task-14) plus the transport frame. The flagship real-contact
  pain-point dissolves with zero authority questions.
- **Automation/cloud users**: NOPASSWD sudo is the norm (cloud-init default users, CI
  runners, homelab automation accounts). `sudo -n` succeeds; authority is held.
- **Interactive personal machines**: the operator writing `sudo` into their book *is*
  the sudoer — the wrapper in the book is direct evidence the authority exists at plan
  time (a book you cannot sudo is a book you cannot apply). Password-gated sudo may
  refuse a non-interactive probe (`sudo -n` fails cleanly) — that cell degrades to
  guards, exactly today's floor.

### finding-walls-not-exhaustive — the descent banked its own escape and discontinued anyway

`27A` §2's fence-escalating-probes contains the fs≠netns≠sudo taxonomy and is annotated
"banked taxonomy for the later dedicated pass (**human-directed**)." The ack-ledger's
matching entry: "needs a later probe-inside pass." Discontinuing the project *before
running the pass the human directed* is the decision the record least supports. The §2
product-C boxing ("authority-tier consent, never a default") is Fable's synthesis — it
appears nowhere in the §6 typed-ack ledger.

## §3 — route-probe-in-context (the headline route)

**One sentence:** for a wrapped site, the probe ships the oracle's already-vouched check
*under the site's own wrapper bytes* — `sudo -n -u postgres <check>`, `chroot /mnt/target
<check>`, `ip netns exec blue <check>` — executing it inside the context it answers for,
exercising only authority the invoking user already holds, and degrading to guard-tier
(today's floor) wherever that authority is absent.

No transport occurs. The license gap does not open because nothing crosses a boundary:
the fact is measured, keyed, and consumed in one context. Referential agnosticism is
untouched (the engine still consumes only claims and probe outcomes — the outcomes are
simply produced in the right world). The tool-oracle stays 100% context-blind (`24S`
§2c's no-wrapper-awareness headline survives byte-for-byte: the engine wraps the
*invocation*; the author writes nothing). The wrapper-oracle already owns the peel and
the axis knowledge (`24S` §2b); the composed shipping form is `273` §6's own drafted
shape (`sudo__…` composing over the inner predict) — pointed at the real wrapper instead
of a simulated ρ.

### The three walks, re-run

- Walk 1 (`sudo pipx install poddle`): probe runs `sudo -n <pipx-check> install poddle`.
  The check's `pipx list` consults **root's** tree. Poddle absent ⇒ diverged ⇒ the line
  runs; root gets poddle. The knife cell never fires — not because anyone promised
  completeness, but because the measurement happened where the state lives. If `sudo -n`
  fails (password-gated): can't-enter ⇒ guard (wall-empirical-rc's own logic, one level
  up). Alice's own unwrapped `pipx install x` elsewhere probes bare and elides on her
  own tree. Two sites, two contexts, two correct answers; the context algebra keeps its
  bookkeeping job (keying facts to contexts), sheds its licensing job.
- Walk 2 (`chroot /mnt/target apt-get install -y openssh-server`): probe runs
  `chroot /mnt/target dpkg -s openssh-server` (root-invoker: free; alice: via `sudo -n`).
  Reads the **target's** database ⇒ diverged ⇒ runs; the image ships sshd. Bonus
  correctness: on first provisioning, `/mnt/target` isn't mounted yet at probe time —
  the entry itself fails ⇒ can't-say ⇒ runs. Both world-states answer correctly, where
  transport answered viciously wrong and the default-wall answered safely-but-valuelessly.
- Walk 3 (`ip netns exec blue sysctl -w …`): probe reads `ip netns exec blue sysctl -n …`
  — the **blue** namespace's value ⇒ diverged ⇒ runs. The per-axis polarity puzzle (one
  kind, fs-invariant yet netns-variant) evaporates: no polarity table is consulted when
  no fact crosses an axis.

And the sharpest exhibit, from `24S` §2's own walkthrough book: line 8's guard is the
admin's *own hand-written* `sudo crontab -l | grep -q renew` — a wrapped read the raw
book executes on every single run today, pre-Dorc, by the admin's own authorship. The
current design refuses to lift it (Stage D: "honestly UNPROBEABLE from outside
(imp-1)"), breaking the Half-B continuity bet ("years of defensive habit turn out to
have been hand-written oracle material all along") precisely and only for wrapped
guards. Under this route it lifts exactly like its unwrapped twin at USER_STORY stage 1.

### Compatibility with the banked walls (none is breached; most are subsumed)

- wall-measurement-reach: explicitly carves this route out — "no vouch, mark, or flag
  **short of escalated probing** changes that." This is that, minus the acquisition:
  exercised-not-acquired authority.
- wall-conjunction-composition: entering the full chain (`sudo chroot … <check>`)
  measures the exact composed context — conjunction turns from any-axis-walls into free.
- wall-statement-located-on-its-subject: strengthened — the verdict-function's yes is
  once again a statement about its own behavior *in the context it executed in*.
- wall-agnosticism-homes: unchanged — an unmodeled wrapper still never peels, so nothing
  un-authored is ever entered; entry requires the wrapper-oracle's peel + axis
  declaration, which is exactly `24S` §2b's existing contract.
- wall-empirical-rc: the degrade ladder rides it — entry failure (sudo -n refusal,
  chroot ENOENT) is rc≥2 ⇒ can't-say ⇒ guard/run.
- wall-verdict-locality, wall-kind-store-protect-downward, lint-who-am-I-taint,
  lint-differential-two-user-CI: all retained, now serving the no-authority residue and
  the stdlib quality bar rather than carrying the product.
- wall-values-same-context: in-context capture makes the value plane same-context *by
  construction* where entry succeeded; the `275` §6 refusal stands for transport (which
  no longer carries anything).

### The consent question (the real substance of the old imp-1)

What imp-1 actually protects, once mutation is separated out, is a consent story: "plan
runs nothing you didn't hand it." Three observations reframe it:

1. **The wrapper in the book is the consent artifact.** The operator wrote `sudo` into
   the line, aimed the book at this host, and typed `dorc plan` intending `dorc apply`.
   Exercising the operator's own authority, at their command, to run the author-vouched
   read the apply will otherwise run minutes later (as a guard, same bytes, same
   privilege) is not acquisition — it is scheduling.
2. **Every incumbent's dry-run already works this way** (~SUSPECT with high confidence,
   worth a citation pass): Ansible check-mode executes module checks under `become`;
   Chef why-run and Puppet noop run as root agents. Dorc's unprivileged probe is the
   ecosystem outlier, and none of those tools flags privileged dry-runs as a consent
   event.
3. **The probe's own traversal side-effects get the design's own treatment**: the
   wrapper-oracle vouches its traversal observables (sudo: auth-log line + timestamp
   refresh) as acceptable-in-the-read-lane — the exact self-effects idiom `24S` §2b
   already specifies for elision. A wrapper whose entry cost is a real mutation gets no
   traversal vouch ⇒ never entered ⇒ wall. Authored, attributed, per-wrapper.

Polarity: this wants to ship as **default-on with graceful empirical degrade**, plus a
safety-polarity opt-out (`--no-context-probes`, STRAWMAN name) for audited/fragile
estates — the same flag-species as fence-strict-posture-ratchet, pointing the same
direction: the rare cautious user types the flag; the everyday path stays flagless
(no ambient-flag rot, per the human's own razor). Tiering is available if wanted
(demotion + axis-entry default-on; ascending-sudo default-on-with-first-use-notice) —
a dial, not a fork.

### The one genuinely new risk, priced

A broken vouch (a check that mutates) now mutates **in-context at plan time** — wider
blast radius than the same broken check running bare-at-plan (today) or wrapped-at-apply
(today, in the guard). This is a real worsening and must be said plainly. Its shape:
same species as the existing exposure (DESIGN: "If your oracle mutates, tough shit" —
the plan promise was always vouch-conditional), same mitigations (structural vouch;
two-user differential CI; attribution to the check's author by name), narrowed further
by the stdlib quality bar, plus one new mechanical option: the `077` tracer lane
(seccomp/eBPF observe backstop — a live constraint, already carried) applies verbatim to
in-context check runs. Weigh it against the alternative it replaces: the claims lane's
failure mode was **unattributable under-execution** — the cardinal sin, ranked strictly
worse by the design's own priority order, and "ours" by the horizon doctrine. A
plan-lane mutation from a broken vouch is attributable, repairable, and bounded.

### What it costs to build

Less than the lane it replaces (~SUSPECT): the peel and axis declarations are `24S` §2b
as-is; the composed shipping form is `273` §6 as-drafted with the simulation target
swapped for the real wrapper; probe emission batches per (host, context) — one
`sudo -n sh -c '…'` segment per context, O(contexts) auth-log noise, exec overhead
dominated by SSH RTT (the perf doctrine's own math). Dropped outright: the per-axis
polarity table as a licensing device, `invariant:<axis>` as a measurement-license, the
completeness audit, the §3 fork and its massage-conditions. hostsim grows
context-qualified verdict injection — which the transport design needed anyway. The
residual `kSTATE`/hostile-host surface is unchanged (no cross-host or cross-run state).

## §4 — route-conditional-tail (containing the residue)

For walls that remain (no-authority cells, unmodeled tools): the guard-cascade's
severity comes from deciding the tail at plan time under "the wall MAY run." Compile the
"may" instead. Each guarded wall sets a flag iff its fallback actually executed:

```sh
( sudo pipx_check install poddle ) || { sudo pipx install poddle; dorc_wall_8=1; }
…
[ -n "${dorc_wall_8-}" ] && { ( systemctl_check enable --now nginx ) || systemctl enable --now nginx; }
```

(STRAWMAN rendering; the real form rides the existing errexit-door machinery.) Tail
lines conditioned on the wall keep their probe-time elision license along the
wall-didn't-act branch — which is sound by the design's own laws: an elided/short-
circuited wall ran nothing, so the tail's facts face exactly the accepted probe→apply
staleness and nothing else. The fired branch is the ordinary stage-2 guard, in-sequence,
at its own position — no staleness by construction, fails toward run.

- **Not the refuted apply-start wave**: no hoisting, no re-probe pass, no post-ack
  confession or replan — the plan disclosed both branches up front; apply-time
  divergence stays proceed-and-flag. It is the guard-cascade itself, evaluated lazily.
- **On the permitted side of the TOCTOU ruling** (spike/CLAUDE.md): the re-verification
  is keyed to a *named, in-book, potentially-responsible cause* — the wall that acted —
  which is the ruling's own "hork-catching is in" side; and the ruling routes posture
  changes to "the re-verification placement-spectrum round," i.e. this route already has
  a sanctioned home (`23O` §4).
- **Render, honestly**: under welded rul-attention-honesty, a may-execute line is never
  hidden — these lines render as guards, at most dimmed, annotated "executes nothing
  unless line 8 acts." The *execution* win is unconditional (zero steady-state
  check-tax — the conditional short-circuits before the check runs — and full
  drift-safety on fired days). Whether a provably-inert-unless-line-8-acts line may
  someday join the elided lines behind the non-verbose fold is a human product ruling;
  nothing here presumes it.
- **Cheapener, from the corpus's own shelf**: `236b`-alt2 kind generation-probes — a
  bracket read around a fired wall revalidates every crossing fact of a kind in O(1),
  "completeness largely by construction (state supervenes on the substrate), not
  testimony." Revive it as the fired-branch fast path; it never needed to die with the
  claims lane.

## §5 — The honest residue (what no route rescues)

- Operators with genuinely no plan-time authority (password-refused sudo, restricted
  reviewers, CI): wrapped sites guard; tails conditional. This is today's floor, now
  contained to the cell instead of the book.
- Unmodeled wrappers (`doas` with no oracle): opaque, run, wall — unchanged, correct.
- Root-only state under a no-authority invoker: guards forever — and note the claims
  lane never rescued this cell either (a vouch cannot conjure an unreadable
  measurement); nothing was lost.
- The tool-internal frame-problem half *within* a single context: exactly the standing
  converged-vouch adequacy gap (`233`), untouched in either direction.
- The sh-visible taint, two-user CI, honest-read idiom, why-lens context notes: all
  still worth building as specified in `27A` §2's D-tier.

## §6 — What this does to the standing decision queue

- `279f:ask-transport-disposition`: answered by option (d) — measure in place; transport
  dies as a mechanism. Register-backed value transport (analytic) stands; world-cell
  transport is mooted where entry succeeds and honest-walled where it doesn't.
- `27A` §3's user-axis polarity fork: **moot** — no typed decision needed. No permissive
  default ships anywhere; no knife cell exists; babby-under-sudo value arrives through
  measurement, not vouches.
- `24S`'s context algebra: retained as bookkeeping (context-keyed fact identity,
  disjointness, hints) — relieved of measurement-licensing.
- Block-context planning: the wrapper-oracle brief gains the traversal-vouch clause and
  the entry ladder (`27B:route-probe-in-context`); probe emission gains per-context
  segments; task-14's probe-form re-derivation now also gates the composed-entry form
  (it was already gating composition).
- The trial revival (`270` §5): re-run the `255` book with entry enabled — both of its
  permanent walls (`su - postgres -c`; `$(hostname)` via the block-context capture
  floor) are expected to dissolve; that is a falsifiable first-blood prediction this
  note stakes.

## §7 — Ask-ledger (typed acks wanted; empty until the human types)

- ask-unweld-imp-one: re-scope imp-1 to its defensible core — "the probe lane never
  *acquires* authority the invoking user has not already granted it; entry failure
  degrades to guard" — explicitly permitting demotion and held-authority entry.
- ask-direct-probe-inside-pass: run the human-directed probe-inside pass (the `27A` §7
  re-entry pointer) as a block-context lane, with §3 above as its seed, *before* any
  discontinuation decision is finalized.
- ask-conditional-tail-sanction: admit route-conditional-tail to the placement-spectrum
  round's scope (its sanctioned home), including the 236b-alt2 revival.
- ask-falsification: accept the §6 trial prediction as the kill-test — if entry +
  capture do not dissolve the `255` book's two walls on a real host, this note's
  headline is wrong and should be struck.
