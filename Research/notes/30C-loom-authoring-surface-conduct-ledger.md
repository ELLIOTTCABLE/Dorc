# 30C — the loom authoring-surface lane (conduct ledger)

AI-authored (Opus conductor, 2026-08-17), from the human's live sitting. Lane branch
`ai/r30-loom-surface` off `ai/main` @ `fa046a37`. Discharges the respec half of
`307:work-edit-loop-line-eviction`; `307:work-loom-interior-hole-authoring` stays banked.
Authority: root docs, `spike/CLAUDE.md`, `crates/aid/CLAUDE.md`, `plans/282` all outrank.

## §1 — The archaeology: how per-case documentation blobs grew

The human asked how the corpus ended up with an LLM-authored `edit-loop:` prose blob
stamped into every loom, and whether one of their own rulings caused it. It did. The
chain, all documented in `28L`:

1. **`282:rul-used-inventory-is-committed`** (human-directed, `282` §13 "Variable
   discovery") — the taproot: *"each defining case **commits** a separate, generated
   replay entry… It deliberately **sits above/beside the flowing diagnostic** so the
   editor knows which rendered words require marker-preserving edits."* Generated derived
   data, committed into every authored data file, positioned beside the edit target.
2. **`28L:rul-variable-surface-is-block-plus-sections`** (conductor) — resolved
   `tc-variable-surface-is-a-command-or-a-file`. Framed as a choice, answered as **both**:
   commit the block AND ship `dorc-loom sections`. The "or" was never adjudicated.
3. **`28L:rul-committed-inventory-retired`** (conductor, marked pending-human-veto) — D4
   killed the block on mechanical grounds (fixpoint self-reference; ~65-file re-churn).
   Correct diagnosis, but the retirement **substituted rather than subtracted**: "the
   loop-hint's text includes the `loom:vars` invocation."
4. **`28L:rul-in-file-loop-hint-minted`** (conductor) — resolved
   `tc-in-file-loop-hint-is-frontmatter`. Framing again asks *where*, never *whether*.
   Landed in x2d across all 75 canonical cases, gate-held by `carries_the_edit_loop`.
5. **Blind-reviewer round 1 chafe** routed `{{name}}`-syntax teaching *into* the hint.
6. **Blind-reviewer round 2** rated it *"the single best piece of onboarding … no doc
   needed."* Ratification.
7. **x2k** added the prove step — a 176-case re-mint.

## §2 — Named failure shapes (generalizable; law candidates, unminted)

- **`taproot-derived-data-committed-per-case`** — the origin ruling committed *derived*
  data into *authored* files. Every downstream cost inherits from that one decision.
- **`retirement-preserved-the-premise`** — when the block died for mechanical reasons,
  nobody re-asked whether the goal still needed serving in-file. A substitution reads as
  a subtraction in a ledger. The sneakiest of the four.
- **`tc-framing-presupposed-existence`** — both `tc-`s asked *where/how*, never *whether*.
  A `tc-` phrased as a placement question structurally cannot return "neither".
- **`blind-reviewer-praise-is-structurally-biased`** — a first-encounter reviewer sees only
  the benefit of in-file documentation and never pays its recurring cost (they read one
  case once; they do not re-mint 176 files or read the 12th copy). Round 2's rave was
  therefore near-uninformative about whether the line should EXIST, and was read as
  endorsement of the mechanism. **Reviewer rounds measure first-encounter cost and are
  blind to corpus cost by construction.**

The churn argument ran backwards in the end state: a vars block re-churns when *that
case's* variables change (one file, exactly when you want to see it); the `edit-loop:`
line re-churned corpus-wide whenever the *teaching text* changed (twice). Per-case-on-real-
change was traded away for corpus-wide-on-taste-change, justified by churn.

## §3 — Corrected mechanism state (the previous conductor read was wrong)

The vars-inventory machinery was retired in D4 and **restored in x2j** ("the restored
vars-inventory (self-reference rule both chains + agreement guard; scaffold emits; NO
corpus churn per the human's lean)"). Nothing about the human's design is dead. Verified
at `fa046a37`:

- `dorc-loom vars --used <case>` works and renders `case: <slug>` + `{{name}} = value`.
- The self-reference rule exists in both chains — `SelfReference::{Allowed, Forbidden}`
  in `consumer.rs`. It is a **stratification**, not a fixpoint: the inventory derives from
  the render an edit compiles against, so the editable-baseline seat drives with
  `Forbidden` and declines the block (it carries no editable prose, so nothing is lost);
  every other seat drives `Allowed` and answers from the **in-memory `Case`**, never a
  materialized txtar section. Answering in-memory is what dissolved D4's "a case cannot
  contain itself".
- `scaffold` emits the block for every new case.
- **It is deployed in 1 of 271 committed cases** (`aid/tests/whylog-absent.loom`, plus one
  dorc-loom fixture). x2j's "NO corpus churn" lean left existing cases to opt in
  individually; nobody ever did.

End state was therefore exactly inverted from intent: the designed thing is opt-in and
unused; the blob that displaced it during its brief absence is mandatory and gated.

## §4 — Rulings typed this sitting (human)

- **`rul-this-is-a-global-flag`** — `--this` is a flag to `dorc-loom` (before the
  subcommand, git-shaped); `--all` is a flag to `vars`. A subcommand flag in the global
  position refuses. Not fancy parsing machinery — a small explicit split.
- **`rul-used-is-the-default-breadth`** — mode becomes optional on `vars`; `--used` is the
  default, `--all` the explicit widening. (Conductor proposal, human-accepted: "flagged,
  not bare, sounds good to me.")
- **`rul-vars-block-is-convention-never-gate`** — no mechanical enforcement that a case
  carries a vars block, ever. Build good CLI features, ensure they work recursively inside
  looms, then keep a Dorc-side *convention* for beautiful/local-focused/totalistic looms.
  Mechanism and convention; never a test.
- **`rul-vars-block-placement-is-per-case-judgment`** — lean: every case that currently
  uses vars, gently, dropped where it reads silly. An editor can trivially add one later.
- The `edit-loop:` line's eviction stands (`307`, HELL-NO typed 2026-08-15) and is
  sequenced AFTER the interactive QA session, so the session can judge whether anything in
  it is worth preserving in the replacement teaching surface.

## §5 — Stale-fossil finding: bless exclusivity

`spike/CLAUDE.md`'s "concurrent agents share one `target/`" (under BLESS-is-EXCLUSIVE) is
**stale for the worktree-per-agent regime**. `mise.toml`'s `[env]` resolves
`CARGO_TARGET_DIR` per-worktree via `{{config_root}}` on every leg, and a contributor
*without* mise gets cargo's default, which is also per-worktree because a git worktree is a
separate directory. Both paths land correct with no special setup. The rule now binds only
two agents inside the SAME worktree, which `worktree-file-access-law` already forbids.

What survives and still binds: bless regenerates from whatever binary exists at that
instant, so it must run against a freshly-built one — satisfied by "build first, in your
own worktree", not by cross-lane scheduling.

NOT YET EDITED into `spike/CLAUDE.md`: to be confirmed empirically (a bless in a fresh
worktree while another leg builds) before the steering file moves. Editing steering prose
on a read-and-reason is how the fossil got there.

## §6 — Lane state

- `lane-vars-target-selector` (Opus, dispatched): `--this` global selector, mode-defaulting,
  global-position flag refusal, `replay_within` target-less form under the unchanged
  `SelfReference` gate, `sections --this`, scaffold + the two committed carriers.
  Explicitly out of scope: the `edit-loop:` eviction and the corpus back-fill.
- THEN: interactive QA walk-through with the human (the flow is suspected of having rotted
  under inattention; the human calls sharp edges).
- THEN: eviction + back-fill as one informed pass.

## §7 — Open / carried

- `307:work-loom-interior-hole-authoring` — unchanged, banked. `28N` §3's
  per-fragment-owners "priced and declined" disposition is up for re-adjudication at that
  sitting; nothing builds toward it here.
- The four §2 failure shapes are unminted law candidates. Two are worth a home if the QA
  session confirms them: teaching-lives-in-the-tool-never-the-data (aid block,
  `spike/CLAUDE.md`) and reviewer-chafe-that-duplicates-is-conductor-adjudicated
  (conductor skill).
- `--all` reads mildly ambiguously once mode is optional (all *values*, every case).
  Pre-existing, not worsened; `--unused-too` is the clearer spelling if it ever grates.
