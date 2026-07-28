# Round charter: ops-glue-residue (started 2026-07-28)

Conductor-maintained. Session: `research-ops-glue-residue`. Topic: making Dorc more
useful for the ops "glue phase" — coverage of territory larger ops tools can't,
won't, or poorly reach. Direction-setting round: broad coverage over minutiae; the
prize is early design-choices/seams with high retrofit cost that stretch coverage.

## Turn structure (sequential Opus research agents; conductor synthesizes between)

- turn-a-lifecycle-channels — pre-SSH payload channels + offline compiled-guard
  artifact constraints (feeds dir-offline-compile-guard-artifact). LANDED
  2026-07-28: 62 graded sources, `turn01-2026-07-28-notes.md` + `sources.json`;
  conductor adjudication below.
- turn-b-thin-transport-floors — prior-art sweep: why incumbents' reach-floors sit
  where they do (feeds dir-transport-byte-pipe-floor + fleet-book reservation
  evidence). PENDING. Brief deltas from turn A: serial/paste DROPPED from scope
  (rul-paste-tier-is-bottom-rung below); ADD the exclusion-check quadrant turn A
  flagged — per channel class, which Dorc lanes survive (probe? report lane?
  apply?), not just "can bytes arrive".
- turn-c-glue-idiom-reality — what real bootstrap/glue scripts contain: wait-loop
  density, inline-ssh sequencing, branch-on-facts (feeds
  dir-until-loop-glue-priority and friends). PENDING. Seeds added 2026-07-28:
  nix-world glue orchestrators (nixos-anywhere / deploy-rs / colmena) as pivot
  prior art — nixos-anywhere ~SUSPECT pivots through machine identities mid-run
  (ssh → kexec installer → reboot, host keys churning) — documented pain there
  is direct evidence for scope-typing + connection-dance requirements; also
  nix-bootstrap glue idioms in the wild (install-nix + apply-flake scripts).

## Strawman-book candidates (accreting; land at Research/notes/r26-glue-strawmen/)

- one per delivery channel from turn A (cloud-init user-data · installer %post ·
  BuildKit heredoc · Packer · dorc-run-under-chezmoi cousin as needed);
- the pivot book (controller creates VPS → until-ssh → in-host convergence;
  outer reachability guard folds the standup region dead on day N);
- kubelet-check node book (rung-zero; k8s controller asked "node exists +
  healthy?"; facts-about-H-minted-from-elsewhere);
- nix-machine book (human-added 2026-07-28, dotfiles-adjacent): guarded
  curl|sh nix install → clone flake → `nixos-rebuild switch`/`home-manager
  switch` delegation → residue lines. Strongest all-in-one exhibit (nix's own
  bootstrap glue is README-shaped); delegation-oracle bookend to the
  ansible-decline story — nix's convergence verb is content-addressed
  (current-system store path vs expected closure), the SOUNDEST delegation
  check any incumbent offers.

## Lane: dorc-INSIDE adjudication (human-added 2026-07-28; tiny, kill-friendly)

Human stance (typed): real-world proven need is dorc-OUTSIDE; they are "a heavy
declarative/static/cattle guy" who intends non-Dorc tools for everything
conceivable; dorc-inside was added for symmetry, and a round KILLING it as
having any use anywhere is an acceptable outcome. Directive: only design what's
actually useful (incl. their own future k8s usage) — never toward fiction.

- Structure: (1) Sonnet repo-inventory of existing dorc-inside material —
  LANDED 2026-07-28 (report in conductor context; key results below);
  (2) 2–3 in-the-wild questions folded into turn C's brief; (3) an explicit
  adjudication section in the synthesis note.
- Inventory key results (Sonnet, 2026-07-28; conductor-adjudicated):
  - SETTLED tier: the chezmoi story + Ansible rung-1 (USER_STORY) plus a
    collated good-guest rule-set (24R §1c R1–R7: byte-transparency, no-TTY,
    whylog-to-file, no-second-state-DB, local-exec-R1, off-ramp) — the
    embedded-transparency law MOSTLY EXISTS already, scattered; TODO's item is
    consolidation, not invention.
  - CONVERGENCE: 24R §1c already frames the durable niche as conjunct (b)
    verbatim — "four tools (chezmoi, yadm, dotbot, home-manager) mandate
    script idempotence by doc and assist none of it." The escape-hatch
    discriminator is corpus-confirmed, not new.
  - NEW-TO-ROUND seat: GitHub Actions `shell: dorc-run {0}` (repurp-finding
    77/80/87) — routes every run-block through Dorc with zero platform
    cooperation; strongest API-adjacent seat already explored.
  - STATUS FACT for the synthesis note's limitations half: `dorc-run` exists
    ONLY as design prose — no binary, no source, no test anywhere (+SURE,
    filesystem-verified); only `dorc-sh` is built (and carries a second,
    internal eval-reentry sense — don't conflate).
  - The two genuinely-unresolved inside-posture design tensions (both
    design-tier, no web research needed): ansible `script` under ssh forces
    `-tt` ⇒ stderr merges into stdout (host tool itself breaks
    byte-transparency); Salt `stateful:` last-line self-report collides with
    byte-faithful passthrough (finding76/78).
  - REJECTED (with reasons, keep rejected): docker-build RUN (string-keyed
    cache, structurally moot; carve-out: warm-base run→mutate→commit loop);
    Talos (no shell); home-manager ACTIVATION slot (blocks spliced into one
    generated bash script, no per-block file — NB constrains the nix strawman:
    wrap home-manager as PARENT, never ride inside its activation); the k8s
    GitOps outer reconcile loop (ceded; distinct from the still-open
    script-content question).
  - Gap-list for turn C triage (named-never-worked): helm hooks ·
    postStart/preStop · terraform_data+triggers · devcontainer lifecycle ·
    yadm/mise-tasks/git-hooks · GitLab/Jenkins.
  - Terminology caution: DESIGN's three-posture happy-parent/child/sibling is
    the only RATIFIED taxonomy; the four-posture billing cut is today's
    human-typed framing, not yet durable — keep distinct in all deliverables.
- CORRECTION to the positioning-note grounding (staleness-audit catch): the
  DESIGN "pluggable deployers" prose survives, but its inventory-consumption/
  transport half was SUPERSEDED by executorless+own-the-transport
  (STALENESS-AUDIT drift-transport row). The k8s-plugin idea stands on its own
  merits as a NEW proposal — do not present it as reviving the old one.
- Adjudication discriminator (human-refined 2026-07-28, two conjuncts): a
  dorc-inside seat is REAL iff (a) the wrapped script RE-RUNS AGAINST MUTABLE
  STATE and (b) the wrapper does NOT itself provide the full
  check/idempotence/attention machinery for that seat. Reformulation: real
  seats are the host ecosystem's ESCAPE HATCHES — where it hands you raw sh
  and disclaims (chezmoi run-scripts, ansible shell/script tasks: both settled
  stories fit). Kill-path: seats whose ecosystems' native machinery already
  covers them (k8s probes/operators/Jobs own WORKLOAD-level convergence — open
  question whether that reaches script CONTENT), or whose escape hatches are
  fresh-context (CI runners, image bake — turn A: Docker convergence
  structurally moot). A-priori candidates to test in turn C: systemd
  ExecStartPre prep on pet boxes; k8s initContainers/Jobs touching persistent
  volumes; self-hosted-runner deploy steps; the terraform
  null_resource+remote-exec poor-man's-config-loop idiom.
- Positioning note (human-typed 2026-07-28): inside-strawmen may be API-SHAPED
  — e.g. a Dorc kubernetes plugin letting dorc-lang + oracles function inside
  the raw-sh sections of machine-managed content, saving the user writing
  k8s-language constructs for everything — NOT literally exec-ing the dorc
  binary as-is. Grounding: DESIGN's pluggable-orchestrator aspiration
  ("integrate Dorc into your pyinfra scripts" deployers); kLANG unbreached
  (authored content stays sh; the plugin is delivery/integration). Strengthens
  rung-zero: "one language for all your residual sh, including the sh trapped
  inside your k8s manifests."
- Note: dorc-inside is POSTURE, not machinery — dorc-run + byte-transparency
  rules cover it with zero new engine seams; the kill-outcome costs nothing
  designed. Embedded-transparency hygiene (TODO.md) is owed REGARDLESS (the
  chezmoi story already ships it).
- Strawmen: add up to two inside-books to r26-glue-strawmen/ ONLY if seats
  survive adjudication; idiomatic-or-nothing.

## Deliverable execution plan (human-directed 2026-07-28)

- CORRECTED (human 2026-07-28): Fable still does DOC writing — the conductor
  writes the final synthesis report (deliverable 2) ITSELF, LAST, once all else
  is done (so it can cite the landed knob + strawmen). Strawmen books
  (deliverable 3) go to Opus writers. Knob entry (deliverable 1): conductor
  lean = Fable-written too (small, design-dense, "doc writing" in spirit) —
  cheap ack wanted, will proceed on that reading absent objection.
- Opus writers MAY dispatch Sonnet doc-readers for their respective tools'
  documentation; every Sonnet brief carries the no-subagent clamp verbatim.
- Strawmen grounding bar (human-typed): strawman in the DORC-LANG sense
  (imagination-tier spellings are fine) but grounded in real-world, functioning
  docs for the respective tool/feature — a realistic design-target that handles
  a reasonable number of the warts of that siting. Not fiction-tier channel
  sketches.

## Deliverables (human-set, 2026-07-28)

1. **deliverable-knob-transport-floor** — a new KNOBS.md entry for
   transport-floor/dependence (SSH-focused vs. broad byte-pipe). Name candidates
   pending human pick (see ask-knob-name-pick below). KNOBS is edit-and-commit for
   in-place human review.
2. **deliverable-synthesis-note-r26** — synthesis report at the
   highest-at-the-time unused r26 slug (26K free as of 2026-07-28; 26A–26J taken).
   Content: direction-setting conclusions, stretch-goals AND near-term limitations
   to keep in hand while other enabling work proceeds.
3. **deliverable-strawmen-books-dir** — an r26 dir of IMAGINATION-TIER strawman
   books, one per delivery channel, each demonstrating the all-in-one property:
   where the non-Dorc standup of the same little service/machine is a <tool>-file
   PLUS an sh prep-block, ours is a single standalone file that just works, no
   prep. Proposed siting: `Research/notes/<noteID>-strawmen/` sharing the
   synthesis note's ID (pending human ack). Every book carries a frozen-evidence
   header: imagination-tier, not-runnable, never-execute (spike safety law).

## Ack ledger (only what the human has TYPED counts)

- ack-pivot-must-support (2026-07-28): "mid-book switching from
  controller-commands to now-live-brand-new-host commands is a must-support for
  *any* of this category." Elevates the two-stage pivot (controller-local lines
  create a machine → later lines run against it) from exploration to category
  requirement. NOT an ack of full fleet-book topology (dir-fleet-book stays
  reserve-only).
- ack-deliverables-trio (2026-07-28): the three deliverables above.
- ack-three-sequential-turns (2026-07-28): three sequential Opus subagents, one
  per area, conductor synthesizes.
- ack-connection-dance-oracles-core (human-typed 2026-07-28): the in-book
  connection-dance minting facts VIA ORACLES is "fairly core to the process" —
  the engine never mints connection facts (that would open a second licensing
  seat vs structural-vouch-only); an `ssh` oracle's own connection-test arm
  mints host-scoped facts through existing machinery; transport-layer
  connection success stays operational/decision-inert. Rider caution
  (human-typed): "can access" rules out almost nothing — reachable≠provisioned
  is the adequacy-gap's transport cousin (cf converged≠no-op, `24U` §2); the
  connection fact licenses only its own narrow cell; the WIDE fold is always
  the admin's own guard-line judgment, attributed there.
- ack-rung-zero-paradigm-unification (human-typed 2026-07-28): for
  dorc-as-parent ("dorc-outside") users, a ZERO-elision book still carries
  value — same language/paradigm/converge-button as the books that do elide
  (e.g. dotfiles-on-Dorc + K8s-everywhere-else: the k8s-node book's only guard
  asks the k8s controller "node exists + healthy?"). Retires the
  "idempotence-wrapper positioning" worry entirely: paradigm-unification is
  rung 0 of the value ladder, elision the escalating payoff, not the entry
  ticket. The human's outside/inside/alongside/does-it-all-but-mediocre
  four-posture billing framing is NOT durably written anywhere (DESIGN's happy
  parent/child/sibling is the cousin) — candidate DESIGN.md addition,
  human-owned; synthesis note carries it as human-typed round framing
  meanwhile. Strawman-book candidate: the kubelet-check node book (also
  exercises facts-about-H-minted-from-elsewhere → scope-typing seam).
- rul-paste-tier-is-bottom-rung (human-typed 2026-07-28): paste-tier is IN-scope
  as a product bottom-rung, OUT-of-scope as research ("brutally obvious"): a
  book may `printf` a block for the human to paste into a web-console and wait
  until complete/available — always-available, last-ditch, and inherently
  outside probe/attention machinery ("it just is"). Residue kept: two
  emit-hygiene rules for any paste-facing emission (physical lines well under
  4095 bytes — the canonical-tty line-discipline cap; never begin a line with
  `~` — SOL/SSH-serial escape). Serial research dropped from turn B.

## Turn A adjudication (conductor, 2026-07-28; evidence = turn01 notes)

- dir-offline-compile-guard-artifact SURVIVES with its novelty claim RESTATED:
  the cell is "empty of convergence machinery, not of channels."
  `#cloud-boothook` is already a plain-sh every-boot user-data format whose own
  docs hand-write a once-per-instance guard; GCP startup-script is every-boot
  semantics on a major cloud; k3s re-runs its installer as the upgrade path.
  Nobody ships GENERAL convergence machinery into the payload cell — and
  cloud-init actively disclaims re-running ("must never be done on a production
  system"). Ignition's rationale is the principled counter-axiom
  (modification ⇒ re-provision) — engage it, don't route around it; Talos is
  genuinely conceded (no shell at all).
- Agent's front-caveat RE-SCOPED (human pushback 2026-07-28, conductor sold):
  guard-only-ness binds the day-zero EXECUTION REGIME only (probe-less
  delivery), never the boot-book ARTIFACT CLASS. Across the lifecycle,
  boot-books are the attention product at its BEST: day-N, the controller
  probes, the admin's own outer reachability guard
  (`if ! ssh -o BatchMode=yes host true; then <standup>; fi`) lift-and-folds
  the entire standup region DEAD — that is `omit` (value-flow license, cheaper
  than elide, zero per-line vouches for the interior), and an omitted region
  casts NO WALLS, which matters maximally because boot sections sit at the TOP
  of books (the worst wall real-estate; stage-5 economics). Boot territory is
  also the kPROBING VALUE-band extreme (huge setup-cost : tiny check-cost) and
  stdlib-friendly. (Freebie-fact idea RETRACTED same day — engine minting
  facts from transport events would be a second licensing seat; the connection
  fact is the ssh-ORACLE's to mint, host-scoped, existing machinery — see
  ack-connection-dance-oracles-core.) Trade to state honestly in the synthesis
  note: the coarse wrapper folds the interior UNCHECKED on ssh-answers days
  (no drift-healing inside the region until finer oracles arrive — same as the
  admin's bare-sh behaviour, no worse; gradual enhancement behaving normally).
  Surviving caveat, narrow: books that NEVER see a controller again (pure
  fire-and-forget, the k3s regime) never collect the attention payout; the
  "idempotence-wrapper positioning" worry shrinks to that regime only. The
  agent's "oracle coverage matters least offline" stays refuted (coverage is
  linear offline; topology effects, not coverage need, are what vanish).
- Standing design rules harvested (cheap now, standing forever):
  rul-no-secrets-in-payload (user-data is IMDS-readable — world-readable
  on-box; artifact carries code and probe-shaped checks only, never credential
  material); per-channel size ledger (EC2 16KB raw is the TIGHTEST mainstream
  cap — GCP 256KB, Azure/DO 64KB; size against the target channel, not
  folklore); the two paste-facing emit-hygiene rules (above); busybox `echo`
  divergence CONFIRMS the existing printf-doctrine rather than demanding new
  law; document the instance-id/cache re-run-on-clone trap in any artifact
  story.
- dir-until-loop-glue-priority STRENGTHENED materially: the community's
  canonical firstboot fix (apt/dpkg lock race, unfixed for years) is literally
  `while fuser …lock…; do sleep 1; done` — an idiom StatusIterated blocks
  unconditionally today; and cloud-init's own "log errors but proceed" paradigm
  makes half-applied firstboot the NORMAL case — the re-runnable guarded
  artifact is the repair story.
- busybox-ash floor: closer to dash than posh is (ash is a dash fork); installer
  environments (d-i, subiquity context) are busybox — so the floor question was
  load-bearing and lands FAVOURABLY. Small flag for kWHICHSH housekeeping
  (outside this round): posh pin 0.14.1 vs sid's 0.14.5 drift; posh source
  unfetchable online — human-as-debugger candidate.
- Turn-A residue parked: runcmd exit-propagation contradiction (minor,
  cloud-init detail); Nix/Bazel content-addressed comparison unread; oracle-
  author/probe-phase quadrants unexamined (→ turn B).

## Consequences already visible from ack-pivot-must-support

- The attribution-scope law's named re-entry trigger ("any second scope becoming
  representable", spike/CLAUDE.md:rul-attribution-is-controller-minted) fires the
  moment one book's lines execute against two different hosts — scope-typing
  moves from someday to near-term-seam in the synthesis note.
- ssh-as-context-entry (plans/27C machinery) becomes the likely spelling family;
  strawmen may try spellings (imagination-tier), the knob entry stays
  spelling-agnostic.
- Readiness-waits (until-ssh-up loops) sit on the pivot's critical path — links
  turn-c's until-loop evidence to the pivot requirement.

## Settled asks

- knob name (human 2026-07-28: not kTRANSPORT — "transport" stays unreserved;
  offered kINIT/kBOOT/or-something): conductor lean **kBOOT**, poles
  `kBOOT-ssh-assumed ↔ kBOOT-any-byte-pipe`; entry prose must state scope
  explicitly (the axis also covers degraded channels on MATURE machines —
  container-exec, SSM — not only literal boot). Awaiting cheap typed ack.
- strawmen dir (human-typed 2026-07-28): `Research/notes/r26-glue-strawmen/`.

## Grounding: r26 as-built state relevant to this round (read 2026-07-28)

- The transport seam is ALREADY channel-shaped, not ssh-shaped: `SessionDriver`
  = ship-one-artifact-to-one-host-once (stdin-fed bytes, stdout/stderr back,
  timeout); three drivers exist (ssh-subprocess, local-subprocess, sim) plus
  the livetest container-CLI seam (`DORC_CONTAINER_CLI`, docker-generic).
- The completion sentinel (`26A` stop-2) rides IN-BAND on stdout carrying `$?`
  — the channel's own rc is never trusted. Consequence: rc-visibility is NOT
  an assumed channel capability; today's real floor is {byte-clean stdin,
  separate stderr, non-echoing}. Container-exec passes that floor today;
  serial/paste does not (echo + merged streams). This weakens the feared
  "SSH-isms baking in" retrofit risk considerably.
- The one channel-capability-heavy commitment still ahead: `142:Resolution`'s
  eventual live topology (per-leaf diagnostic files, FIFO fast-lane) assumes a
  file-ish side-channel; single-channel whole-artifact collected-after is the
  sanctioned degenerate start and is what's built.
- Known current limitations that bound near-term glue claims (for the synthesis
  note's limitations half): N=1 only, no fleet kernel; local-exec owed as an
  explicitly-supported user-facing mode (TODO.md — and it is a PREREQUISITE of
  ack-pivot-must-support's first half, book-lines-running-on-controller);
  privilege/sudo unresolved (firstboot payloads typically run as root, day-N
  as a user — an asymmetry the offline-artifact story must address); guard-tier
  class ruling open (`fnd-classed-decline-unwalls-guard-tier`); streaming/TUI
  deferred; whylog holds unsanitized host metadata; CRLF gate live at plan and
  apply intake.
