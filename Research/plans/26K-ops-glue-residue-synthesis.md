# 26K -- ops-glue-residue: implementation plan + round synthesis

> Promoted notes->plans 2026-07-28 (human-directed): this is THE kBOOT /
> parent-child-sibling plan. §0 (the implementation plan) leads and is the
> actionable head; §1-§6 are the round synthesis it rests on. Ahistorical:
> rewrite in place as things land.

## §0 -- The implementation plan (human-ruled cuts, 2026-07-28)

Two lanes. The boundary law binding lane one (human-typed): lint/warns EMITTED
by existing analysis are fine -- zero new machinery, no new or changed
analysis/walking, "the moral equivalent of adding a printf", pure aid/render
planes only. Anything more belongs to the kernel-builder.

### §0a -- The fruit arc (one conductor-arc, unstaged; correctness-only)

- fruit-emit-hygiene-paste-rules -- renderer law on durable/paste-facing
  emissions: no physical line approaching the 4095-byte canonical-tty cap; no
  line beginning `~` (SOL/ssh-serial escape). Render-side only; pins ride in
  the render corpus. (This IS the first splice-floor damage-watch pin.)
- fruit-oracle-matched-zero-sites -- aid-plane warning aggregated from
  EXISTING verdict-provider results: an oracle that loaded and matched zero
  sites is announced (the silent-decline value-evaporation detector, B3's
  find; kWARN-rich tune-high era). Test: loaded-but-never-matching case.
- fruit-loop-does-not-loop-lint -- flag `for i in {1..N}` (brace-range as a
  literal word) under a plain-sh file: the loop runs once (k3s + the k8s docs
  both ship it). GATED by the boundary law: ships in this arc only if it is a
  pure lint-crate rule over existing parse output; if it needs any new
  walking, it moves to the kernel batch instead. Test: lint case.
- fruit-doc-no-secrets-payload -- one authoring-doc paragraph siting
  rul-no-secrets-in-payload (user-data is instance-readable on the major
  clouds; payloads carry code and probe-shaped reads, never credentials).
- Explicitly OUT: anything touching analysis/effect/plan kernel crates;
  CRLF-line-number and the rest of the pre-round polish queue (r26's own).

### §0b -- THE kernel sitting (FIRST in the important lane; design AND build, one batch, human-led, next context-window)

Titular entry, the big must: **local-exec as a supported, user-facing, tested
mode** -- TODO-owed, the pivot prerequisite, and the reason this sitting goes
first: local-exec needs epochs/scopes (the controller-as-target is itself a
second scope). One batch of machinery: epoch/pivot scope typing plus the
simple loops/waits that roll in naturally. This plan supplies needs, timings,
and consumer contexts; the invariant-snaking and sharp-edge-finding belong to
the sitting itself.

Needs-and-timings (feature -> first consumer -> when it must land):
- scope/generation slot (host x epoch; controller-minted; thin presence in
  coordinate/record, forced by invariance-carry attribution) -> local-exec
  itself -> AT the sitting, first.
- until/wait wall-transparency (pure-delay body + modeled condition: never
  elides, casts no wall; licensing untouched) -> the pivot's and boothook's
  first executable lines; every RR channel -> IN the batch (ratification of
  sit-wall-transparent-delay-loops happens in-sitting).
- epoch-transit classing (a `transits`-shaped mark; converged transit elides
  and casts no boundary -- the transit-relative law) -> pivot books' day-N
  fold -> the slot at the sitting; the SPELLING may land with transit-verb
  oracles later.
- inverse wait (wait-unreachable -> wait-back -> re-probe) -> reboot-shaped
  transits -> a later increment; design-note it now.
- connection-dance oracle facts (ssh-family; reachable != provisioned rider)
  -> the pivot's outer guard -> after local-exec, with stdlib revival.
- scope-ownership design (sit-scope-ownership; split-books stands meanwhile)
  -> FOLDS INTO this sitting's design half (its four arrivals are the
  agenda: host x epoch, --target-host, delegate_facts, NodeRestriction).
- no-tty-refuses-survival (B3's kHALVES-collapse candidate) -> cheap to rule
  during this sitting's design half; rides the agenda, not the batch.
- terminology-unification rider (human head-state, typed 2026-07-31): the
  sitting's design half must produce ONE coherent worldview across
  dimensions / epochs / pivots / transits -- the human is suspicious of this
  round's "epoch is a temporal cousin of the 27C dimensions, not a fifth
  sibling" claim, suspects epochs are "just" dimensions-plus-a-temporal-
  analysis, and names the load-bearing symmetry: both time-passing-across-
  certain-commands and dimension-crossing-across-certain-commands share the
  same could / provably-could-not shift-global-state-and-perturb-assumptions
  structure. Prefer industry-standard PLT terms over the current coinages
  ("pivot", "epoch") where genuine equivalents exist. Explainers before
  solutions; not to be settled outside the sitting.

Consumer contexts the batch must not foreclose (one line each; details §2):
pivot day-N region-fold behind the reachability guard · the boothook's three
faces under no-semantic-fork · headless initContainer (guard==elide there) ·
hm shape-B store-path splice · relay topologies (capability = meet of legs;
relay host may be a managed target in the same book) · `nixos-rebuild
--target-host` declines · identity churn at pivots (everyone punts host keys;
phone_home is the unauthenticated prior art).

Explicitly NOT in the batch: redirect-routing (own sitting, after), os-release
(research-gated own sitting), anything survival-tier.

### §0c -- Banked prominently, design-first, NOT build-yet

- sit-stdin-copy-exec-amendment: the artifact-on-stdin hazard (two live wild
  specimens; family split 4-to-1 toward copy-then-exec; rset's
  confidentiality counter; judo's fd0 doctrine) wants a DESIGN SITTING before
  any `260` §5 change -- the human holds suspicions about the downsides
  (host-disk writes vs rul-probe-writes-only-what-it-owns adjacency;
  confidentiality; cleanup). No build scheduled.

### §0d -- The rest, ranked

- Human-any-time: the DESIGN.md footers (splice-floor; four-posture billing;
  agentless-vs-Big-Boys) · the remaining §5 sittings not folded above
  (redirect-routing AFTER the kernel sitting; os-release after field
  research; kTYANNOT carrier item stands DOWNGRADED to residue per the
  firewall weld).
- Tests posture: pins ride their lanes (no separate tests phase). The fruit
  arc carries the today-pinnable render pins; the kernel batch carries its
  own; fragment-render pins arrive with any future `compile`.
- Strawmen books: REMAIN AS-IS (human-ruled: historical, explicitly
  imagination-tier; any recut waits until they are borderline functional).

AI-authored (Fable conductor, human-directed throughout; written LAST per the
round charter, 2026-07-28). The round: make Dorc more useful for the ops "glue
phase" -- territory larger tools can't, won't, or poorly reach -- hunting early
design choices with high retrofit cost that stretch coverage. Five research
turns (183 graded sources), one repo inventory, three strawman builders, and a
blind sibling-mistake archaeology. Evidence base: `.claude/research/
ops-glue-residue/` (round-charter.md = the full adjudication ledger;
turn01-turn05 notes + sources/). Sibling deliverables landed with this note:
`KNOBS.md:kBOOT`  /  root `SIBLINGS.md`  /  `Research/notes/r26-glue-strawmen/`
(six imagination-tier books + companion notes + three SIBLINGS fragments).
Terminology: this note uses the round's slugs; `charter:` prefixes refer to
sections of the round-charter.

## §1 -- The round in one screen

Glue territory decomposes into lifecycle reach (pre-ssh birth -> decommission),
transport reach (which channels can carry a book at all), and topology reach
(pivot, fleet, controller-local). The load-bearing discoveries: Dorc's floor is
the lowest in the category (any byte-pipe to a POSIX sh -- the incumbents'
documented *emergency* floor is our normal mode); the guard-half works with no
controller round-trip, making delivery-day payloads a *face* of ordinary books
rather than a separate product; boot-books are the attention product at its
best, not its exception; and the payload-language choice (sh) is what makes the
transferable plan artifact possible at all -- the closest sibling's author
designed our artifact and his payload language blocked it for five years
(pyinfra 688/805).

## §2 -- Direction-setting conclusions

- **concl-boot-books-are-peak-attention-product** -- the epoch law is
  TRANSIT-RELATIVE (human check, `charter:` epoch section): a boundary exists in
  a plan only where the transit will actually run; a converged transit
  elides/omits and casts no wall, so on day N the admin's own outer reachability
  guard folds the entire standup dead through EXISTING machinery (guard-lift +
  value-flow `omit` -- zero new licensing). Boot sections sit at the top of
  books, the worst wall real estate -- day-N elision of the boot region protects
  everything downstream. Consequence, top-priority when stdlib work revives:
  **transit verbs (reboot, kexec-install, create-VM) lead the describability
  priority order** -- an unmodeled, unguarded transit walls every day and kills
  the class.
- **concl-offline-compile-is-a-face-not-a-product** -- `dorc compile`'s
  guard-only artifact is licensing-coherent today (guards need vouches, never
  probe facts) and is the day-zero *delivery face* of the same book whose day-N
  face is full elision. Its honest value is SAFETY-shaped and small (B1
  measured 5 inserted guards beside 6 admin-authored ones) -- never pitch it on
  attention. Three faces, one meaning (the chef-solo grave-rule: narrowing
  allowed, semantic forks never): controller-probed  /  compiled  /  RAW-SOURCE
  (the paste-the-unstripped-source test), the last being the strongest argument
  yet for the `#:` mark carrier -- there is no strip pass in a boot sequence.
  Rules minted: no credential material in payloads (IMDS-world-readable);
  size against the actual channel (EC2 16KB is the tightest, not the norm);
  the offline regime difference must always be an observable host fact, never
  a delivery flag (B2 proved it costs nothing).
- **concl-splice-is-the-floor** (human framing, verbatim-tier in
  `charter:rulings-batch-2`) -- fragments compiled to live inside other tools'
  generated scripts are a FLOOR TO MAINTAIN, strictly harder than paste
  (mechanism + safety + offramp + host-format durability), easy to make
  impossible via output-channel assumptions. One embeddable-output discipline,
  three consumers (paste block  /  offline payload  /  spliced fragment):
  hygiene wrapper, compile-time `exit` refusal, errexit-robustness (already
  holds -- the guard idiom was born errexit-exempt), self-contained lanes
  (already holds -- `${DREP_V1:-/dev/null}`), and embedding provenance (the one
  real machinery ask; kin to `111`'s locator DAG). Structurally grounded:
  incumbents cannot follow (payload-language pickling; cdist's manifests don't
  run). Home-manager is the cleanest specimen (store-path splice, shape B);
  Debian maintainer-scripts and cloud-init assembly are the family.
- **concl-transport-floor-is-the-lock-in; mechanisms are not** -- the knob is
  cut (`KNOBS:kBOOT`): floor assumptions (capability lattice, in-band sentinel
  doctrine, asymmetric byte-cleanliness) are the retrofit-hostile half and are
  leaned NOW; the machinery reaching the floor (transport v2, per-leaf lanes)
  is `kLOCKIN`-low and deferred. Resolution mechanism is human-ruled:
  **capability-probing per-feature, per-host, by Dorc** -- features matched to
  min-capabilities across non-monotonic target populations; never a
  degradation ladder, never the oracles' job.
- **concl-until-loops-are-load-bearing** -- supported from three independent
  directions (firstboot's canonical apt-lock fix idiom; every exec-API channel
  being poll-based; the pivot's critical path). First increment adopted at
  conductor tier, human ratification pending: a loop with a pure-delay body
  and an oracle-modeled condition NEVER ELIDES and CASTS NO WALL
  (wall-transparency deliberately severed from licensing -- the `getent`
  condition may never license, and disturbs nothing). Compile waits INTO the
  artifact wherever the awaited fact is host-observable (perf-law already
  decides this: never let a network boundary participate in iteration);
  controller-side waits are the pivot's sanctioned exception, including the
  INVERSE wait (wait-unreachable -> wait-back -> re-probe) that reboot-shaped
  transits need. Free diagnostic: the `{1..N}`-under-`#!/bin/sh` shape ships
  broken in k3s AND the k8s docs -- "this loop does not loop" is a pure
  kWARN-rich win.
- **concl-pivot-is-chartered** -- `ack-pivot-must-support` (human-typed):
  mid-book controller->new-host switching is category-required. The machinery
  bill is mostly already-owed items re-ranked: local-exec as a supported mode
  (its prerequisite), the connection-dance as ordinary ssh-oracle facts
  (engine mints nothing -- `ack-connection-dance-oracles-core`, with the
  reachable!=provisioned adequacy rider), until-loop modeling, and the
  scope-typing seam -- which this round hit FOUR independent times (hostxepoch
   /  `nixos-rebuild --target-host`  /  Ansible `delegate_facts` precedent  /  k8s
  NodeRestriction forcing a controller-scoped line). Ruling minted at
  strawman tier: a book has ONE scope; split books today; oracle-declared
  execution-scope is REJECTED (claim authors must never relocate someone
  else's mutations). Epoch is a temporal COUSIN of the `27C` dimensions, not a
  fifth sibling: no entry form can exist, it must never join `lend_map`
  vocabulary, invariance-carry works verbatim per-kind, guards are epoch-proof
  by construction, planned transit = analysis event vs unplanned churn =
  integrity event (both laws pre-exist; epoch is their shared substrate;
  "generation identity" was this concept's reserved seat).
- **concl-inside-is-alive-narrow-and-cheap** -- the two-conjunct discriminator
  (re-runs against mutable state AND host assists nothing) is corpus-native
  ("mandate idempotence by doc, assist none" is now a SIX-member ecosystem
  norm). Alive: k8s initContainer-writing-to-a-PV (structural -- probes are a
  validation ERROR where init scripts live), Jobs-on-PVs, systemd multi-line
  prep, devcontainers (low-stakes), Jenkins interpreter-selector (pet-agent
  gated). Dead: fresh-context CI, initdb.d. Thin: terraform null_resource,
  helm hooks (one level down, inside the image). dorc-inside is POSTURE, not
  machinery -- zero new engine seams. Candidate human ruling from B3:
  headless + same-machine collapses elide-vs-guard on both attention and
  wall-clock with guard strictly safer => **a no-TTY dorc-run refuses
  `kSURVIVAL-trusted` by construction** (the inside seat is the design's
  safest cell).
- **concl-rung-zero-paradigm-unification** (human-typed) -- for dorc-outside
  users, a ZERO-elision book still carries value: one language, one converge
  button, beside the Big Boys (the kubelet-check book demonstrates it
  honestly). The value ladder reads: rung 0 unification -> safety/guards ->
  perf -> attention; elision is the escalating payoff, never the entry ticket.
- **concl-staleness-is-physics-honesty-is-the-product** -- the sibling record
  proves the probe-time!=apply-time gap unfixable (pyinfra's v3 kept the
  prepare pass; every v2 "Limitation" survives under a softer heading). The
  differentiator is posture: stale-and-SHOWN plan + consent, guards as
  DEFAULT degradation, proceed-and-flag -- vs their opt-in, user-remembered
  `_if=`. This is the wrongness-ownership positioning (the why-claim was
  never "we print `# converged`"; it is "when we print it we can be wrong and
  we OWN that harder than anyone") with external evidence attached.
- **concl-sibling-death-modes** -- churn-under-contract (pyinfra: four majors
  of renames; an ordering mechanism rebuilt three times in three releases;
  a semantic change inside a minor) vs accretion-without-contract (cdist:
  migration docs end 2014; subsystems 5-8 years behind `--beta` at project
  death; an official untested-`sed` migration). Both validate the stability
  ledger's carve exactly; the flip-moment of
  `rul-strawman-formats-no-compat` at PUBLICATION is the discipline moment
  both point at. Peripheral faces rot (@winrm, @ansible, @hook) -- binds our
  offline/fragment faces to the one-pipeline rule; land or park explicitly,
  never indefinitely-beta.

## §3 -- Near-term limitations (keep in hand while enabling work proceeds)

- `dorc-run` is design prose only -- no binary, no test (the keystone of both
  settled inside stories). `dorc compile` likewise imaginary. Neither blocks
  design work; both block any live demo of this round's books.
- local-exec as a user-facing supported mode: owed (TODO), zero tests, and
  now a PIVOT PREREQUISITE.
- Privilege: unresolved; the round supplies its shape -- an early-bound
  `$SUDO`-prefix host fact threaded per-line (three tools independently), the
  `27C` licensing story as the distinctive half, mechanism adequacy =
  sudo+doas, and the named hazard that escalation historically breaks
  stdin-fed payloads (Ansible ships our floor-cell disabled over it).
- The stdin-consumption hazard in the shipped invocation (`260` §5) has TWO
  live wild specimens; copy-then-exec is load-bearing for pivot books.
  Human-owned amendment decision, banked-aside with mode sketches
  (`charter:rulings-batch-2`).
- Report lane breaks FIRST under channel degradation (exec-API caps are sized
  for eyes, not drains); RR channels need an overflow story eventually.
- Zero stdlib: every strawman leans on hand-oracles; the holes cluster where
  pivot walls hurt (timeout, ssh-keygen, curl, getent) and transit verbs top
  the revival priority. The os-release DOT-SOURCING wall blocks the densest
  idiom in the wild (~28k files) -- contracted-parse-of-a-specified-format is
  the candidate answer, human-owned.
- Standing sensitivities inherited: whylog holds raw host metadata
  (delegating predict arms are an exfiltration surface -- read-only !=
  safe-to-report); guard-tier class ruling still open
  (`fnd-classed-decline-unwalls-guard-tier`); the offline artifact breaks the
  wild norm of assumed target-side egress (a difference to state, not hide).

## §4 -- Stretch goals (banked, not scheduled)

- openwrt as a borderline livetest (human-typed): one box exercising the
  dialect floor, the DP-class channel cut (dropbear), a non-deb package
  oracle, and per-feature capability matching at once.
- Identity-bound-at-creation for pivots: every incumbent punts host keys at
  the pivot; the controller MINTED the machine (cloud-init `phone_home` even
  POSTs the host keys -- unauthenticated, so it relocates rather than closes
  the trust cell). Nobody occupies the close-it cell.
- The k8s API-shaped face (plugin letting dorc-lang ride inside manifests'
  raw-sh) and B3's compile-as-primary-k8s-face inversion (no binary in image,
  artifact in PR review, guards-only costs nothing there).
- Probe-lane-as-fleet-query product surface (fenced by plan-as-API
  discipline). Deferred with the fleet.
- The redirect-routing sitting (below) unlocking the write-if-changed idiom --
  potentially the highest-value single analyzer increment this round found.

## §5 -- The human-sittings queue minted by this round

Collated; each is human-owned, none blocks current work:
1. sit-redirect-routing-composes-oracle-channels -- B1's
   finding-redirect-writes-live-outside-argv + conductor reframe (oracles
   claim per-channel BEHAVIOR, the engine owns ROUTING, the fs kind-owner
   binds routed-stdout to File coordinates; never an engine completeness
   claim). Unlocks sh's most common mutation idiom.
2. sit-wall-transparent-delay-loops -- ratify the until-loop first increment
   (§2).
3. sit-scope-ownership -- line-said vs engine-derived execution scope (oracle-
   declared already rejected); the scope-typing seam's consolidated sitting
   (four arrivals; hostxepoch; split-books ruling standing meanwhile).
4. sit-no-tty-refuses-survival -- B3's kHALVES-collapse candidate ruling.
5. sit-kty-annot-carrier-population -- two builders independently: the `#:`
   carrier's constituency is books-that-run-before-their-tool (raw-source
   face), not comment-preferrers. Possible kTYANNOT delta.
6. sit-os-release-contracted-parse -- the densest-idiom wall (§3).
7. sit-260-stdin-amendment -- copy-then-exec vs pipe (+ `--strict` bundle /
   lint-tier sketches banked).
8. sit-oracle-matched-zero-sites-warning -- B3's silent-decline
   value-evaporation; kWARN-rich as the only detector for the class.
9. DESIGN.md footers (human's own writing): splice-is-the-floor; the
   outside/inside/alongside/does-it-all four-posture billing; the
   agentless-floor-vs-Big-Boys positioning (declarative cannot pair with
   agentless at scale; Dorc-and-the-residue).

## §6 -- Round deliverables index

`KNOBS.md:kBOOT` (the transport-floor tension, leaned at floor-tier)  /  root
`SIBLINGS.md` (three-posture framing table; builders' fragments + conductor
columns)  /  `Research/notes/r26-glue-strawmen/` (pivot-vps-standup  / 
userdata-boothook-web  /  installer-latecommand-base  /  nix-machine + hm-splice
shape-B pair  /  k8s-node-standup  /  k8s-initcontainer-pv-seed + manifest; all
frozen-evidence, never-execute)  /  this note  /  the round charter + turn01-05
notes + 183-source graded ledger under `.claude/research/ops-glue-residue/`.
