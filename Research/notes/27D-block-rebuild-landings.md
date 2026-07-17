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

## stage-2: typeless-floor — LANDED 2026-07-17 (conductor checkpoint PASSED)

Commits `296643a..ea77549` on `ai/r27-typeless-floor`, fast-forwarded into
`ai/spike3-r27`. Builder: Opus solo. Full landing record: **`notes/27E`** (the
builder's note — auto-cell mechanism, kernel seam shape, tc-flags, deferrals).
645 unit / 128 e2e green; ZERO existing-golden churn (two new exemplar cases
only). Conductor spot-checks: `auto_or_opaque` mint conditions (concrete argv
only, no-check-resolved only, mapped-provider keyed) and the
`survival::disjoint` auto-kind may-touch fence — both per-spec, slug-cited.

**Checkpoint dispositions (conductor-adjudicated):**

- disposition-marker-gate-home — tc-marker-gate-home CONFIRMED as built: the
  dedicated cli-edge pass is the right home (kernel/lift purity; the gate fires
  at the user boundary; the ~100 unmarked-dialect unit tests exercise lift
  MECHANICS, which is test-scope, not product-scope). The lift-level+stamp
  alternative is retired.
- disposition-set-e-floor-limitation — tc-set-e-blocks-auto-elide ACCEPTED as a
  principled product limitation (inv-probe-sourced-values: the markless floor
  cannot fabricate the book command's rc, so under `set -e` only the guard tier
  is reachable; marked verdicts declare converged-rc and elide). RECORD FOR THE
  STDLIB TEACHING DOCS: "the bare floor under `set -eu` buys guards, not
  elisions — the first mark buys the elide back."
- disposition-positional-literal-model — the founding one-liner licenses because
  the lexer routes `$@` to `Word::Literal("$@")` — a WRONG MODEL whose outcomes
  are caught by rc-authority (the shipped body is authored bytes; the probe's
  real rc is the verdict; wrong-concrete static reads decline⇒run). Accepted for
  now, NOT blessed as design: rider-positional-modeling-hardening assigned to
  stage value-recipe-reshape (route bare `$@`/`$*`/`"$*"` to ⊤ per
  inv-top-reject; model quoted `"$@"` as concrete positionals per
  `24C:fd-headline-oneliner-gap`; the e2e pin must stay green across the change).
- disposition-effect-check-punt — ACCEPTED (the typed sizing rider licensed
  exactly this: not nearly-free ⇒ punt, zero guilt). The seam now exists;
  re-homing decision deferred to conductor discretion post-block.
- disposition-invited-rooms-pass-forward — ACCEPTED; the compile-failure pin
  obligation travels to whichever stage first mints a hint-lane/license-lane
  type split (likely block-context's descend-don't-license types).

**Handoff (owner assigned):**

- rider-raw-ship-repair-with-otelcol → stage-2b (dedicated dispatch, next): the
  `24J` raw-ship repair (`271:rul-only-oracle-bytes-ship` + three riders) plus
  the COUPLED A6 otelcol-boilerplate retirement; build surface per `27E`
  (`connected_check_pipes` + `compile_probe` connected path + `render_sh`).
  Also carries the `279f` §5 connected-probe riders: SIGPIPE-flap as a named
  nondeterminism class (why-lane note on rc-141 sink landings; hostsim race
  injection; any future `--exit-code` computes from divergence-of-world, never
  raw sink-landings).
- rider-positional-modeling-hardening → stage value-recipe-reshape (above).

**Environment incident (surfaced to human 2026-07-17):** a SyncThing conflict file
(`effect.sync-conflict-…-PHNHRER.rs`) appeared INSIDE the stage-1 builder's agent
worktree mid-edit — device PHNHRER is live-syncing `.claude/worktrees/`, which the
standing exclusion intent says it must not. Inert this time (untracked, not
compiled); a real mid-edit corruption risk for future parallel agent work.
Cleanup + `.stignore` repair are human-owned.
