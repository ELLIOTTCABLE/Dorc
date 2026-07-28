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
- turn-d-incumbent-featureset-comparison (human-proposed 2026-07-28,
  conductor-ACKED) — deep design/featureset comparison vs cdist + pyinfra:
  where Dorc is positioned; which contemplated value-adds are genuinely
  valueless to build because an incumbent has them locked down; candidates
  for honest "if you're doing X, just go use Y" README advice. SCOPE FENCE
  (conductor): per-feature effort-allocation ONLY, never product-viability
  relitigation — go/no-go stays welded GO (AGENTS.md market-value-hole
  fence); the brief must exclude "should Dorc exist". PENDING — human standing
  order 2026-07-28: dispatch AUTONOMOUSLY once turn C is synthesized; then
  STOP — the writing phase (Opus strawman-writers, knob entry, synthesis note)
  waits on explicit human ack.

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
- INSIDE candidates (post-turn-C verdicts; writing-phase ack pending): the
  k8s initContainer-writing-to-a-PV book (the structurally-ALIVE headliner
  seat; possibly the API-shaped/plugin face) and/or the systemd ExecStartPre
  multi-line-prep book on a pet box. Devcontainer seat alive but low-stakes —
  mention in the note, no book.

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
- CONDUCT RULE (human-typed 2026-07-28, after turn B ran three Opus
  sub-gatherers): research turn-agents do their work IN-SCOPE THEMSELVES — no
  sub-researcher dispatch, hard clamp in every research brief. The SOLE
  exception is the final strawman-WRITERS, who may spawn Sonnet doc-readers
  (each carrying the no-further-spawning clamp).

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

## Turn B adjudication (conductor, 2026-07-28; evidence = turn02 notes + sources/, committed 9718604f)

- ESCALATION, out-of-round, engine-relevant (flagged to human in-chat):
  **artifact-on-stdin has a documented stdin-consumption hazard.** The shipped
  invocation (`260` §5: `… '<remote-sh>' -s < artifact`) means any interior
  book command that reads stdin (a prompting apt-get on a drifted day, any
  filter run bare) consumes ARTIFACT BYTES — sh reads its script from fd0
  incrementally, so this corrupts parsing, not just the command. Prior art
  split 4-to-1 against pipe-by-default (cdist/judo/drist/remotely copy-then-
  exec; only rset pipes, for confidentiality, and grew `-t` for exactly this;
  judo closes fd0 as doctrine). The one-connection fix exists
  (`cat - >tmp && exec sh tmp` shape, keeps stdin free, no scp dependency).
  Candidate `260` §5 amendment — human-owned, NOT this round's to decide.
- STANDING RULE minted for dir-offline-compile-guard-artifact (Chef's grave):
  chef-solo died of a TWO-CODE-PATH split (`if Chef::Config[:solo]` forks in
  recipes; local-mode's stated win = "one less code path"). Rule:
  **offline/compile mode may NARROW (compile-time refusals) but must never
  FORK semantics** — same book, same meaning, different delivery; no
  offline-only dialect, no behavior keyed on delivery mode. Applies equally to
  any future fragment/splice render.
- THE KNOB THESIS, assembled (entry's spine, evidence-grade):
  - adbd's source contains the capability lattice as a literal 2×2 (PTY/raw ×
    protocol/no-protocol): rc-visibility and stream-separation are INDEPENDENT
    capabilities; a PTY forecloses separation even with framing. Docker states
    the same law inversely (separation only via 8-byte framing, lost under
    TTY). Dorc's floor triple = one coherent lattice cell.
  - adb shell-v1 and k8s exec-v1 BOTH lost the exit code and bolted framed
    side-channels on in v2; Docker never fixed it (rc needs a second
    exec-inspect request). Dorc's in-band sentinel = the v2 move made in v1.
    Azure classic run-command reports NO exit code at all ⇒ on that channel
    the sentinel is REQUIRED, not defensive (best single validation of `26A`
    stop-2).
  - Ansible's pipelining cell IS Dorc's floor triple — and Ansible ships it
    DISABLED because sudo/requiretty conflicts with stdin-fed payloads.
    Reframe pole A: not "we assume SSH" but "we assume the pipelining cell";
    named hazard: privilege escalation is what historically breaks stdin-fed
    payloads (intersects the open sudo/privilege gap + 27C context entry).
  - Floor is ASYMMETRIC: byte-clean stdin outbound, but no incumbent promises
    a byte-clean RETURN path (ansible raw advises base64; SSM is UTF-8-only).
    NB engine intake law (rul-host-bytes-bounded-before-admission) already
    treats return bytes as bounded untrusted raw — the design was ahead here.
  - RR-class truncation is a sentinel PLACEMENT constraint and the clouds
    disagree on which end survives: AWS keeps HEAD (24K stdout/8K stderr),
    Azure classic keeps TAIL (4096 bytes). GCP has no real primitive (1024-
    char inline cap; rc redefined as compliance tristate where exiting 0 is
    an ERROR). SSM Session Manager is a proven hard non-target (stdin and
    stream-separation are mutually exclusive at agent source). Proxmox/qga is
    the best-behaved RR member (rc+signal+truncation flags+file pair; stdin
    cap 1MiB-CLI vs 64KiB-REST inconsistency noted).
- LANE-SURVIVAL (the Q5 table lives in turn02 notes; four consequences):
  (1) the REPORT lane breaks first under degradation, not apply — RR caps are
  sized for human output, not drains; RR needs an explicit overflow story
  (independently arrived at the aws_ssm-needs-S3 shape); (2) probe survives
  wherever ANY return path exists — the OS class boundary is exactly where
  turn A put it; (3) only `142`'s live topology genuinely needs DP+F, and the
  DP→DP+F bridge is prior art (`dd`-over-exec; adb/k8s in-band framing);
  (4) the FIFO fast-lane loses its point on DP — degradation paths exist all
  the way down.
- VALIDATIONS (cheap citations for the synthesis note): pyinfra ships
  probe/plan/apply and documents the staleness trap + `_if=` runtime-guard
  answer ⇒ the elide-vs-guard cut is ARCHITECTURE-FORCED, not a Dorc quirk ·
  cdist's own stated regret is CONNECTION COUNT (one remote_exec per explorer;
  ControlMaster bolted on; MaxSessions ceiling) ⇒ one-artifact-per-phase is
  the evidenced differentiator vs closest prior art · Mitogen retires
  "lower the floor" (payload IS python) while proving pipe-sufficiency and
  via=-chaining · incumbents' emergency floor (ansible `raw`, salt-ssh `-r`)
  = Dorc's NORMAL mode (framing gift) · Ansible answers capability-poor
  channels three ways: synthesize OOB (aws_ssm S3, CONFIRMED + stronger than
  suspected — even module .py transit rides S3), RELOCATE to controller
  (`_remote_is_local` for network gear — incumbent-shipped precedent for
  pivot-style "run this on the controller instead"), or synthesize files from
  the pipe (`ssh_transfer_method: piped` = dd if/of).
- SCOPE/PIVOT prior art: `delegate_facts` — Ansible grew an explicit
  fact-attribution opt-in THE MOMENT one play addressed two hosts (facts
  default-assign to inventory_hostname, not the producing host): direct
  incumbent hit on the attribution-scope re-entry trigger. DeHaan's 2012
  announcement names push's irreducible advantage in ack-pivot-must-support's
  exact shape ("do THIS here, hop over there, do THAT… where pull breaks
  down") and names the bootstrap-over-ssh regime. Fabric's own pitch for its
  task-major model is that it IS shell-script logic ⇒ the natural inline-ssh
  spelling competes against nothing better.
- Cautions kept: no prior art found for REFUSING to trust channel rc (weak
  evidence in both directions; report as neither novelty nor error) · the
  books-reading-stdin question flagged, not answered (rset -t vs judo
  fd0-close doctrine) · cdist first-party sources blocked (code.ungleich.ch
  HTTP 500; graded from 2016 fork snapshot) — human-as-debugger candidate ·
  GCP exec output cap contradicted between two primaries (100K vs 512K).
- Conductor pre-writing reading list (before knob entry firms): Radman's
  "Minimalist scripted configuration" deck (argues against the naive
  `ssh host < script` form and lands on a refined one; his rinstall(1)
  change-detecting rc convention = the guard-shaped primitive) · the adb
  2×2 source · rset's sibling transports · aws_ssm plugin doc. All archived
  under sources/.
- Turn C brief additions from B: test Radman's enumerated naive-form failures
  against real glue scripts (don't re-derive) · wait-loops gained SECOND
  independent support (every RR channel is poll-based ⇒ in-artifact until-
  loops are the only waiting seat there) · pyinfra `_if=` = the shipping
  baseline for branch-on-facts comparison · read inline-ssh sequencing
  evidence against delegate_facts + the DeHaan framing.
- Label ruling applied: research-doc commits are `(AI dsn new/re)`; `rsr` was
  a stray (agents warned by hook, allowed; no history rewrite).

## Rulings batch (human-typed 2026-07-28, post-turn-B synthesis)

- ack-splice-floor-framing — the splice-vector theorization ACCEPTED, with the
  human's own sharpening: **"splice is the floor"** trumps "paste is the
  floor" because splice is STRICTLY HARDER — paste needs copy-friendliness and
  shortness but no purity/safety/defensive compilation; splice ADDS mechanism
  and safety while ALSO needing paste-ish properties (offramp; readable/
  durable inside the host's format). Never the core target; instantly a FLOOR
  TO MAINTAIN — watch how other work damages it; easy to make
  borderline-impossible via output-channel assumptions. Human will personally
  work it into a DESIGN footer (their edit, not ours). Consequences kept from
  the theorization: dir-splice-vector-fragment-mode joins the round's
  directions; fragment-grade render = the same embeddable-output posture as
  paste-block + offline artifact (ONE discipline, three consumers); its
  constraint set (hygiene wrapper · compile-time `exit` refusal ·
  errexit-robustness [already holds] · self-contained lanes [already holds] ·
  embedding provenance/source-map [the one real machinery ask, kin to `111`'s
  locator DAG]); shapes A (splice compiled text) and B (splice a store-path
  invocation line — nix-idiomatic, full local probe/plan/apply); the
  chef-solo NO-SEMANTIC-FORK rule binds all of it. Nix strawman book gains an
  hm-inside face (shape B).
- ack-stdin-hygiene-banked-aside — the stdin-consumption escalation is REAL
  but SET ASIDE (potential standalone r26 dig, human-owned timing). Sketches
  banked verbatim-tier: close-stdin-early as OPTIONAL hygiene
  (gradual-enhancement ⇒ lint-tier, or a CLI flag rolled into a `--strict`
  bundle); keep supporting pipe; possible default = "pipe to US, we collect
  and ship as an artifact when possible, flag-off for pure pipe";
  confidentiality (rset's write-nothing-remote rationale) noted as possibly
  important, undecided.
- ack-lane-survival-defer — lane-survival machinery is critical and needs
  deeper digging ("transport effectively needs a v2") but GENTLY DEFERRED:
  the FLOOR assumptions are the high-lock-in part to determine NOW; the
  mechanisms that reach the floor consistently/safely are low-lock-in and
  deferrable.
- **rul-capability-probing-per-feature** (human-typed, bank EARLY — shapes the
  knob entry directly): capability-probing is DORC's job, per-feature,
  per-host — never the oracles'. No gradual-enhancement through
  tiers/layers/ladders of transport capability: match each FEATURE to its
  MIN-CAPABILITIES and provide the full set of features each host's
  capability-set licenses, across a weird, heterogeneous, NON-MONOTONIC target
  population (the web's feature-detection-not-user-agent-sniffing lesson).
  Kills any "degradation ladder" model in the knob entry; the lattice is a
  per-feature requirements table, not a ranked ladder.
- cdist first-party sources: human confirmed the site is fully down (no
  render, no access) — stays blocked; archive.org at writing time if needed;
  the 2016-fork grading stands disclosed.

## Turn C adjudication (conductor, 2026-07-28; evidence = turn03 notes, committed 736ae26b; harness security-banner reviewed — transient classifier error, commit scope verified clean)

- HEADLINE, wait-placement cut — agent marked ~SUSPECT, conductor UPGRADES to
  doctrine-decided: a controller-side wait-loop is O(retries) CONNECTIONS (each
  `until ssh` iteration a fresh handshake; nixos-anywhere runs four such loops
  per invocation); an in-artifact wait is one connection. Existing perf-law
  already rules this ("never let a network boundary participate in
  iteration"): compile waits INTO the artifact wherever the awaited fact is
  observable from inside the host; the pivot's own waits (host
  existence/reachability — definitionally unobservable from within) are the
  sanctioned controller-side exception. Crosses dir-until-loop-glue-priority ×
  ack-pivot-must-support.
- Lint gem, no elision machinery needed: k3s's `#!/bin/sh` installer AND the
  k8s docs both ship `{1..N}` brace-ranges under POSIX sh — dash-verified: the
  loop runs ONCE ("this loop does not loop"). A bounded-retry shape-recognizer
  is a pure kWARN-rich diagnostic payoff. First-party wait verbs exist across
  mature tools (`pg_ctl -w`, helm `--wait`, devcontainer `waitFor`) =
  delegation-oracle targets; cloud-init's EIGHT-valued status separates
  `degraded done` from `done` (an incumbent shipping wrong-but-not-broken as
  first-class).
- Wild confirmations: port-open≠login-works hit by an author in comments
  (adequacy rider, reachable≠provisioned) · the stdin-consumption hazard has a
  LIVE specimen (heredoc-to-login-shell with interior sudo) · heredoc-with-
  controller-interpolation is the median multi-host shape (env-marshalling
  answer and bug in one) · terraform states copy-then-exec is for CONTEXT
  PRESERVATION, not perf (family now 5-to-1 vs pipe) · terraform's
  controller/target mktemp limitation is an asymmetry in Dorc's FAVOR
  (ship-one-artifact ⇒ host's own mktemp+trap work; k3s does exactly this).
- Privilege trichotomy = the shape the sudo gap must model: three independent
  tools compute root/`sudo`/`doas` ONCE into a `$SUDO` prefix var and thread
  it through every mutating line — an EARLY-BOUND HOST FACT, not a per-line
  decision. And nixos-anywhere's `get-facts.sh` is a hand-rolled Dorc probe
  artifact (20 lines POSIX sh, one connection, key=value out) — strongest
  single validation of the round.
- dorc-INSIDE verdicts (full table in turn03 notes): ALIVE-narrow = k8s
  initContainer-on-PV (STRUCTURAL: k8s mandates idempotence by doc and
  REJECTS readinessProbe at validation exactly where setup scripts live —
  headliner answered in k8s's own words) · k8s Job/CronJob-on-PV (weaker) ·
  systemd ExecStartPre multi-line prep (unit-level gates never reach lines) ·
  devcontainer lifecycle (low-stakes). THIN = terraform null_resource (alive
  exactly where HashiCorp says don't go) · helm hooks (payload is argv-on-
  image, not raw sh). DEAD = CI runners (fresh workspace by design; GitLab/
  Jenkins verify folded to turn D) · docker-entrypoint-initdb.d (gated on
  empty datadir, verified). Caveat kept: ALIVE cells' typical content is
  SMALL (init-container population dominated by wait-loops); the live cell is
  "init container that WRITES to a PV". Cross-cutting: mandate-idempotence-
  assist-nothing is now a SIX-member ecosystem norm; THREE incompatible
  tri-state "nothing to do" rc conventions exist in the wild (systemd
  ExecCondition / GCP OS-policy / rinstall) — the missing shared primitive IS
  the oracle rc contract (strong hole-filling argument for the note).
- PIVOT findings, all design-bending: identity churn is the pivot's DEFINING
  property — every incumbent PUNTS host keys (`StrictHostKeyChecking=no` +
  known_hosts=/dev/null hardcoded; terraform disables validation by default).
  OPPORTUNITY note for the knob/synthesis: the controller MINTED the machine
  — identity could be bound at creation instead of punted (nobody does this).
  Facts do NOT survive the pivot (nixos-anywhere re-imports facts post-kexec)
  ⇒ scope-typing needs host×EPOCH, not host; pre-pivot connection facts are
  invalidated by the pivot. New wait shape: INVERSE wait (wait-for-
  UNREACHABLE, then wait-back, then re-probe) — reboot-shaped pivots. And the
  pivot is where nixos-anywhere independently invents sentinel-over-channel-rc
  (`|| true` + grep a teed log) — second external validation of `26A` stop-2,
  at the exact case that forces it.
- rul-capability-probing-per-feature GROUNDED (adequately, honestly short of
  the homelab-inventory claim): community.openwrt exists BECAUSE one fleet
  member can't run the incumbent's payload language (ash modules; per-host
  gather_facts toggles; per-FEATURE requirements rows); three newer≠more-
  capable instances (DNF5 < DNF3 config-manager; busybox setsid w/o --wait;
  Alpine timeout w/o -t). Behaviour-probing-not-name-probing is the wild
  idiom too (`setsid --wait true` test-run; wait-for-it greps timeout's usage
  text).
- Radman's list TESTED: the real gap is the CHANGE-DETECTING rc (k3s
  hand-rolls PRE/POST_INSTALL_HASHES sha256-compare = "converged: content
  match" hand-written in a first-tier installer); partial-apply-by-STATE vs
  hand-declared phase flags (k3s ~20 SKIP_* knobs; nixos-anywhere --phases) =
  the clearest value claim of the turn; staging REFUTED for ship-one-artifact;
  caveat banked: the real-world norm assumes TARGET-SIDE EGRESS (curl-in-
  everything), which the self-contained offline artifact deliberately does
  not. Splice hygiene wrapper has first-party precedent (three canonical
  installers wrap against partial download; tailscale says why in a comment).
- Ordinal frequencies (lower-bound code-search, ordering only): branch-on-
  facts ≫ generic retry/wait ≫ port-probe waits ≫ multi-host ssh loops ≫
  until-ssh — with the standing caveat that until-ssh is rare-but-critical-
  path (frequency is the wrong priority signal for the pivot).
- Gaps folded into turn D: cloud-init phone-home (the REVERSE pivot — target
  announces readiness; nothing else covers that direction) · GitLab/Jenkins
  custom-shell verification of the DEAD verdicts.

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
