# 27Q — the block-stdlib handoff: preconditions ledger + on-ramp

AI-authored (Fable conductor, r27-impl session close, 2026-07-17). The HANDOFF PACKET for
the block-stdlib conductor. By human ruling (typed 2026-07-17, in-session): the stdlib
MINTING happens under a NEW, human-led conductor — the human directs and spot-checks
(quality-critical oracles; a security-flavoured class they will structure themselves) —
so this note is deliberately a *preconditions ledger*, not a brief. Authority: root docs,
`spike/CLAUDE.md`, `plans/270` §2, `plans/27C`, and the `27D` ledger outrank this.

## §1 — Where the engine stands (what you inherit)

Everything `270` §2 named as the stdlib's precondition is BUILT and conductor-verified:
block-context CLOSED (wrapper-peel `27K` · payload-v1 `27L` · context-entry, inlined in
`27D` · integration `27N`) · pure-predicate carry DISCHARGED (`27O`) · the per-run PATH
shim MATERIALIZED (`27P` — entry-composed probes execute for REAL; the babby-sudo
acceptance runs on real records with an anti-masking proof). At the stack tip: 884 unit /
91 e2e / four gates, all conductor-verified own-hand. The wrapped-site pipeline is
end-to-end: peel → consent (`decide_entry`, two-axis) → entry-composed probe (only
oracle bytes) → context-qualified `FactKey` → readback → elide/guard/run, with
attribution on the why-lane at every cross-context outcome.

Read-first for the new conductor: `Research/LIVING_STATUS.md` (the on-ramp; stacking/fold
state) → `27D` end-to-end (the round's ledger: every landing, every disposition) →
`plans/27C` (THE wrapper/context spec, kept current through 2026-07-17) → landing notes
`27N`/`27O`/`27P` as needed → this note's §2 before ANY oracle is authored.

## §2 — Preconditions and teachings that bind stdlib authoring

- **teach-marked-command-not-cmdsub** (`27O`; `27C` §4(a) v1 note): the carry-closable
  verdict idiom is the MARKED-COMMAND rc-partition form — the tool reads the marked cell,
  its rc IS the verdict. The cmdsub-comparison spelling (`[ "$(tool -n "$1")" = "$2" ]`)
  is NOT closable at v1 (walls, safe). Stdlib verdicts that want carry must be spelled
  marked-command; the teaching template must teach that form FIRST.
- **limitation-context-blind-stage1-wall** (`27N`/`27D`): a RUNNING wrapped mutator walls
  downstream converged establishes context-BLIND (safe direction; over-execute).
  Stdlib-era books will feel it; the survival-tier follow-on (context-qualified
  wall-sparing) is UNOWNED. Fixture/book authors: order matters.
- **precondition-minting-line-threading** (`27D` block-close seam list; `27I`
  disposition): attribution-at-sparing-verdict requires the minting LINE (source span)
  — MUST land before the stdlib mints selector-bearing `disturbs` claims. UNOWNED;
  schedule it as an early errand of the stdlib block.
  <!-- /* superseded 2026-07-18: now OWNED — the user-aid build phase (notes/27V,
  mech-minting-line-threading) discharges it pre-stdlib. Also new for oracle
  authoring: classify deliberate declines as you author (notes/27W §5 guidance —
  the DREP report-lane emission; rationale in a comment ON the arm, shown never
  parsed). */ -->
- **quality bars, all three, plus the adjudicability list**: the `279f` §5 stdlib
  quality-bar adds (incl. the banked lint candidate "verdict/observe mark on a
  constant-rc line") · the wrapper-oracle bar `24S:A6` (peel cross-check, argparse
  lints, self-vouch/self-footprint, expressibility) · the carrier bar `24T:P-A4`
  (which-arg-is-code arity gates, parse-and-resolve cross-check, reconstruction
  differential, dorcism-in-payload lint) · and the **adjudicability build-list
  `24S:A4`** — MUST land before kinds go community-shared; no round owns it yet.
- **MH2 prerequisite-acts** (TODO-ADDTL; `.claude/research/versioning-mh2/`): at stdlib
  dispatch, reserve the final version-role names in the munge-lint; oracles born with
  machine-observed stamps (capture-at-authoring); the §4b charset-confirm was marked DUE
  after stage-3 landed the charsets — confirm hash-shaped canonical forms stay reachable.
- **authoring templates = the landed fixture shapes**: `tolerates:` as a bare colon-line
  in-body (recorded divergence from `27C` §2's shorthand — de-facto standing) · the
  `sudo` oracle fixture (dual-peel coherent predict/lend_map + `-n` entry form; see
  `27C` §9 for the siting-discharge spellings, STRAWMAN) · `invariant:<axis>` lines
  inside `kind__state_stored_only_in()` (`272`/`277` §4e; netns-on-net-kernel is
  model-forbidden with a loud diagnostic) · the one-liner-vs-full both-shapes teaching
  candidates banked at `27D` stage-1 (finding-one-liner-candidates).
- **seam-psa-sudo-path-weave** (`27P`; `274` §9): an env-scrubbing real sudo drops the
  shim PATH ⇒ rc-127 ⇒ can't-say ⇒ run (safe but value-losing). The stdlib sudo entry
  form should consider PATH-preservation explicitly; the spike mock preserves PATH
  deliberately and keys pass-through on the `-n` spelling (fixtures churn if the entry
  form respells).
- **yardstick-measurement** rides the stdlib block (`270` §2): the may-alias sensitivity
  number the `24O` item-25 flip-or-confirm decision is waiting on.
- **rider from r26** (`26B:ask-trial-counts-capture-walls`): the revived field trial
  counts capture-walls; stdlib-era fixtures shouldn't paper over `$(…)` walls that the
  trial wants to measure.

## §3 — Standing process law (verified this round; binds successors)

Fold protocol: rebases/merges/lineage ref-moves are HOOK-RESERVED to the human ("the
user does the review-and-rebase pass on AI commits"); conductors verify zero-overlap +
behind-count, request the human commands, then re-verify gates on the folded result.
Merges from `main` batch at round-close. Builders hold at tip and never rebase. The
comment budget counts INLINE comments only (mandated `///` doc-comments and generated
fixture content excluded; quality checked by eye at checkpoint). Naming discipline per
`270` §1. Sonnet-tier always clamped against sub-spawning. The quarantine
(`Research/notes/quarantine-DO-NOT-READ/`) stays unread — `27M` is intentionally there
by human ruling. `LIVING_STATUS` conduct fences carry the rest.

## §4 — The forward seam list (unowned; carry, don't lose)

predict-inner carry + predict-inner entry-composed (both mirror `27N`'s verdict-first
interim) · cmdsub-VALUE modeling (unlocks the `27C` exemplar spelling for carry) ·
peel-into-`command_effect` migration question (kSTATE-adjacent) · cross-kind backing
members (derive-model widening; rides r26's capture era) · effect-check re-homing (seam
built, punted) · real-executor shim choreography (atomicity/cleanup; `27P`) ·
multi-link chain e2e · tc-stage-ship-triplication cleanup · batch-5 + E3 e2e tails ·
survival-tier context-qualified wall-sparing (the context-blind wall's fix) · the
round-close merge batch — **DISCHARGED 2026-07-18 at `a651fe8`**: the r27 stack merged
into `ai/main`, the three-way 27C union completed (stack rulings ⊕ the
`27C:rationale-vouch-vs-completeness-gate` §2 block + non-vacuity + cross-ref), merged
tree conductor-verified (884/0 unit · 91/91 e2e · four gates). Residual topology
(`main` vs `ai/main`; the frozen `ai/spike3-r27` ref) is the human's; the "two asks"
inside `de1b09c`'s commit-era dialogue were never surfaced to this conductor — the new
conductor should ask the human whether they were handled in their own session.
