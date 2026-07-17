# 27D — block-rebuild landings ledger (accretes per stage; the `24C` role for round 27)

AI-authored (Fable conductor, r27-impl session, begun 2026-07-17). One section per
landed `270:block-rebuild` stage: what landed, the conductor's checkpoint dispositions,
and the residue/handoff riders later stages own. Authority: root docs and human-typed
rulings outrank this; `plans/271`/`notes/277` are the specs the stages build against.
Append-only per stage; never rewrite a landed section.

## stage-1: corpus-respell — LANDED 2026-07-17 (bless-checkpoint-one STAMPED)

Commits `1112526..4f0d89d` on `ai/r27-corpus-respell`, fast-forwarded into
`ai/spike3-r27` (the r27 live lineage — supersedes `ai/spike3-r23` as the forward
branch). Builder: Opus lead + clamped Sonnet sweeps. Scope delivered: the dorc-lang
v0.1 corpus churn (153 oracle fixtures; `#`-selector marks; `:`/`:!`/`:?` sigil
family; `disturbs`/`disturbance_reaches_only` renames; `sm.dorc.*` re-keys;
`is_diverged` + `VerdictSense` hard-deleted per `24C:rul24-ditch-is-diverged`;
marker stamps + `#!/usr/bin/env dorc-sh` shebangs; comment-rip 185.7k → 20.1k bytes
against the ≤38.4k budget; forward-munge keying for KIND-keyed roles;
`Word::PositionalDefault` for the `${2-}` idiom; gate-1 compare verified already
order-insensitive per `262` §7).

**Conductor verification (own hand):** ancestry 8-ahead/0-behind; fresh
build + 636 unit tests + e2e 126/126 zero-xfail re-run green on the folded lineage;
leftover greps EMPTY across all six pattern classes (dotted role-defs,
touches/reaches defs, is_diverged, stringly emission, dot-selectors in marks,
unmunged kinds). Bless diff inspected by `24P` §4 delta-class census: label re-keys
(bulk), comment-echo shrinkage (rec-1 flow from the rip), ±newline bless-artifacts,
lax-order re-captures (builder verified identical multisets). Spot-read:
`285e30c` (the forward-munge `rekey_to_raw_kinds` bridge — the sharpest fix; the
respell had silently un-fired resolvers/reaches for dotted kinds and the first
bless masked it), the flagship golden, the nounset golden.

**Checkpoint dispositions (conductor-adjudicated):**

- disposition-books-comment-rip — ACCEPTED. The brief's "books byte-untouched"
  collided with the budget arithmetic (the human's own baseline counted books;
  books alone were 90k > the 38.4k target). Book comments ripped; golden echoes
  shrank accordingly. Surfaced to the human as flag-worthy; goldens churn freely
  per standing ruling.
- disposition-nounset-pin-strengthened — ACCEPTED. `guard23-nounset-book-survives`
  changed behavior: the `${2-}` sweep made the shipped guard body set-u-safe, so
  the converged-past-wall curl site now GUARDS-and-short-circuits (live
  `dpkg-query` re-check) instead of crash-falling-through. The 23C-fd2 pin is
  strengthened, not lost: guard-under-`set -u` now works rather than merely
  failing safe. (Builder's report said "elides" — imprecise; it is a guard
  short-circuit; the wall stands.)
- disposition-state-stored-only-in-inert — ACCEPTED with a stage-2 caution rider:
  the member parses (marks read as opaque Establish-shaped marks) but nothing
  consumes it; stage 2's auto-cell/classify work must not accidentally hand those
  marks live Establish semantics.
- disposition-sharefile-book-no-marker — ACCEPTED.
  `guard23-reingest-collision-verbatim`'s book is a stripped off-ramp artifact:
  marker-free is correct; the `reserved-namespace-squat` warning pin retained.

**Handoff riders (owner assigned):**

- rider-marker-enforcement-deferred → stage 2 (part B): markers stamped but the
  engine still treats them as comments; enforcement (dialect-in-unmarked = loud
  error) deferred because ~100 inline-source unit tests carry unmarked dialect.
- rider-command-keyed-backward-munge → stage 2 (part C): command-keyed roles kept
  the backward `_`→`-` un-munge, violating `24C:rul24-totalistic-munge`
  (a literal-underscore book word like `my_tool` silently finds no oracle).
  Bounded debt, corpus-absent today, must die before the stdlib.
- rider-dorc-sh-unbuilt → stage 6 (e2e-degraduation): the `dorc-sh` thin bin +
  whole-file `dorc strip` do NOT exist (only per-funcdef strip); the stamped
  shebangs are inert in e2e. Human steer recorded 2026-07-17: keep the strip's
  shebang-runner rewrite an extremely simple plain-text substitution
  (`dorc-sh`→`sh`-ish, per the `24Q` adjudication).
- rider-selector-charset-unenforced → stage 3: `277` §4b's loud-diagnostic on
  charset-violating selectors is not built (fragments stay opaque).
- rider-brace-alternation-unexpanded → stage 3: `#{a,b}` parses as one opaque
  token; per-token expansion wires when the claim algebra consumes it.
- rider-resolver-coverage-watch → stage 3: the `rekey_to_raw_kinds` bridge keys
  off kinds collected from establish/query backings + wall-candidate footprints;
  any lookup path outside that set silently misses resolvers — the
  entity-algebra-rebuild must subsume the bridge into the re-keyed coordinate
  machinery, not inherit it blind.
- rider-internal-rust-naming-debt → stage 3, in passing: `touches`/`reaches`
  survive as internal module/type names (`lift_touches`, `TouchesSet`),
  documented at each site; rename as the re-key churns those modules.
- finding-one-liner-candidates (for the stdlib quality-bar's both-shapes
  teaching): flagged-never-converted passthrough-shaped verdicts at
  strawman24-alias-symlink (writeconf), five service.oracle.sh copies
  (systemctl is-enabled delegation), strawman24-reach-static-service (enablesvc).

**Environment incident (surfaced to human 2026-07-17):** a SyncThing conflict file
(`effect.sync-conflict-…-PHNHRER.rs`) appeared INSIDE the stage-1 builder's agent
worktree mid-edit — device PHNHRER is live-syncing `.claude/worktrees/`, which the
standing exclusion intent says it must not. Inert this time (untracked, not
compiled); a real mid-edit corruption risk for future parallel agent work.
Cleanup + `.stignore` repair are human-owned.
