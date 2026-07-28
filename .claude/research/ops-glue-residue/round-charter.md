# Round charter: ops-glue-residue (started 2026-07-28)

Conductor-maintained. Session: `research-ops-glue-residue`. Topic: making Dorc more
useful for the ops "glue phase" — coverage of territory larger ops tools can't,
won't, or poorly reach. Direction-setting round: broad coverage over minutiae; the
prize is early design-choices/seams with high retrofit cost that stretch coverage.

## Turn structure (sequential Opus research agents; conductor synthesizes between)

- turn-a-lifecycle-channels — pre-SSH payload channels + offline compiled-guard
  artifact constraints (feeds dir-offline-compile-guard-artifact). DISPATCHED
  2026-07-28, in flight; artifacts `turn01-*` in this dir.
- turn-b-thin-transport-floors — prior-art sweep: why incumbents' reach-floors sit
  where they do (feeds dir-transport-byte-pipe-floor + fleet-book reservation
  evidence). PENDING.
- turn-c-glue-idiom-reality — what real bootstrap/glue scripts contain: wait-loop
  density, inline-ssh sequencing, branch-on-facts (feeds
  dir-until-loop-glue-priority and friends). PENDING.

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
