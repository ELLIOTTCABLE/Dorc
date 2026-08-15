> (née `.claude/reports/r30-prompt-review-audit.md`; re-homed 2026-08-15 per `300` §5 no-parallel-durable-dirs.)

# Prompt-review audit — the r30 arc's steering additions

**Report-only.** Nothing was applied; no file outside this report was written, no
git command was mutating. Audited in the primary checkout at tip `e1e9008a`
("Land the verified-core and incident law in the crate registries").

- Criteria-as-of: 2026-05-04 (103 days — **AGING**, 91–180 day band). The audit ran
  against current criteria; staleness is approaching but a refresh is not yet
  indicated. Criterion IDs below (`P1`, `F2`, …) are the prompt-review skill's.
- Artifacts audited (6): `.claude/skills/conductor/SKILL.md` · `spike/CLAUDE.md` ·
  `spike/crates/core/CLAUDE.md` · `spike/crates/analysis/CLAUDE.md` ·
  `spike/crates/plan/CLAUDE.md` · `minispec/CLAUDE.md`
- Method: `git log -p` per file to isolate the arc's additions, then each addition
  read in its surrounding whole-file context, then every factual claim in the new
  text checked against the tree (mise tasks, crate/module names, symbol names,
  file paths, source behaviour). Findings below are only where an addition is
  wrong, unactionable, or fights standing text.
- I did not enter `Research/quarantine-DO-NOT-READ/`. Finding **B4** was
  established from a directory-name search alone.

**Headline:** the arc's additions are factually accurate almost everywhere — I
checked roughly twenty concrete claims (task names, flags, crate names, symbols,
paths, an "advisory" characterisation) and only two were wrong (**B4**, **F1**).
The real yield is four collisions with standing text (**B1**, **B2**, **B4**,
**D1**), of which **D1** is the one that could change what code does.

---

## Priority order for the adjudicator

If you only act on some of this: **D1** and **B1** first (both actively mislead a
builder), then **B4**/**F1** (wrong paths in laws), then **B2**/**B3** (worktree
safety, and the arc's own near-miss is about exactly this), then the rest.

---

## Proposed edits

### D1 — `sug-converged-check-now-contradicts`

`spike/crates/analysis/CLAUDE.md:12-19` (standing text, contradicted by the arc's
new `solve-is-certified-only` at `191-199`)

Plain-language gloss: the file's **first** bullet tells a builder "always check the
`converged` flag"; the arc's **last** bullet says "never use `converged` as a trust
gate." Same file, ~180 lines apart, opposite instructions.

> `solve` carries an iteration cap and returns `converged == false` instead —
> every correctness-critical caller MUST check it (`trust_reach` is a
> per-consumer obligation, never ambient).

→

> `solve` carries an iteration cap rather than hanging, but the cap flag is
> ADVISORY: correctness-critical callers take their answer from
> `solve_certified` and consume its `SolveConsistency` at a named floor
> (`solve-is-certified-only`). `trust_reach` stays a per-consumer obligation,
> never ambient.

Criterion: **P7** (one term/one story per concept) + **P8** (scope stated
explicitly). Rationale: the two bullets give opposite instructions about the same
flag, and the stale one is the file's opening bullet — the one a
skim-reading builder is likeliest to act on. The new text is the true one:
`analysis/src/effect.rs:1789` derives `trust_reach` from
`reach_consistency.is_consistent()`, not from `converged`, and `solve` is now
`pub(crate)`. Confidence: **SURE**.

*Same-cluster, out of scope (source comment, not a prompt artifact):*
`spike/crates/analysis/src/solve.rs:13` still reads "A correctness-critical caller
MUST check `converged`." Whoever applies D1 should sweep that line too.

---

### B1 — `sug-fmt-chafe-conflicts-with-task-law`

`spike/CLAUDE.md:859-861` (arc addition)

Plain-language gloss: this new bullet teaches every future agent to bypass the
mise task layer for `fmt` — which is the exact behaviour the same file bans forty
lines earlier, and which root `AGENTS.md` calls out as the thing to fix rather
than work around.

> - **fmt-under-agent-env** — `mise run fmt` wraps `hk fix --all`, which the agent
>   session's `HK_FIX=0` turns into refuse-without-rewriting; the working agent
>   spelling is `mise exec -- cargo fmt --all --manifest-path spike/Cargo.toml`.

→

> - **fmt-under-agent-env** (KNOWN CHAFE — fix the task, don't spread the
>   workaround) — `mise run fmt` wraps `hk fix --all`, which the agent session's
>   `HK_FIX=0` turns into refuse-without-rewriting. Until a task covers it, the
>   interim spelling is `mise exec -- cargo fmt --all --manifest-path
>   spike/Cargo.toml`; report the chafe upward rather than propagating the raw
>   invocation into briefs or scripts.

Criterion: **P8** (scope) + **P3** (why). Rationale: as written this is a bare
standing exception to `spike/CLAUDE.md:798` ("**Use the mise
tasks — never hand-derive an invocation.**") and `:821-822` ("Reach for raw
`mise exec -- cargo …` only for something no task covers, and consider adding the
task instead"), with no signal that it is an exception or that the fix is owed;
root `AGENTS.md` is explicit that tooling chafe gets fixed, not swallowed.
Confidence: **SURE** on the conflict; **SUSPECT** on the wording, since the real
fix (an `fmt` task that detects `HK_FIX=0`, or an `fmt:agent` task) would delete
the bullet instead — which is the better outcome if you want to spend the ten
minutes.

---

### B4 — `sug-quarantine-path-is-wrong`

`spike/CLAUDE.md:1002` (standing text; now in tension with the arc's new
conductor-skill quarantine section)

Plain-language gloss: the "never read the quarantine" law names a directory that
does not exist. The real quarantine is one level up.

> - **Never read** `Research/notes/quarantine-DO-NOT-READ/` (including spike2 code)
>   or `Research/corpora/` unless the orchestrator explicitly hands you a pointer.

→

> - **Never read** `Research/quarantine-DO-NOT-READ/` (including spike2 code) or
>   `Research/corpora/` unless the orchestrator explicitly hands you a pointer.

Criterion: **P8** (a rule that names nothing binds nothing). Rationale: verified —
`Research/quarantine-DO-NOT-READ/` is the only quarantine directory in the tree;
there is no `Research/notes/quarantine-DO-NOT-READ/`. The arc's new conductor-skill
section (`.claude/skills/conductor/SKILL.md:92`) uses the correct path, so the two
steering files now disagree, and an agent that resolves the disagreement by
"the path in the law doesn't exist, so the law is stale" walks straight into the
quarantine. Confidence: **SURE**. Note `Research/corpora/` may be the same class
of error — I did not check it, deliberately, since it borders the standing
corpus/H2SaLS fence.

---

### F1 — `sug-trustedbase-path-is-wrong`

`minispec/CLAUDE.md:41-42` (arc addition — the conductor-finalised unit contract)

Plain-language gloss: the trusted-base file is named at the wrong path; it lives a
directory deeper than the text says.

> Generic-dictionary hypotheses
> (`LawfulClone`/`LawfulEq`) are NAMED TRUSTED-BASE entries in
> `Minispec/TrustedBase.lean` — governed shared vocabulary;

→

> Generic-dictionary hypotheses
> (`LawfulClone`/`LawfulEq`) are NAMED TRUSTED-BASE entries in
> `Minispec/Vocabulary/TrustedBase.lean` (module
> `Minispec.Vocabulary.TrustedBase`) — governed shared vocabulary;

Criterion: **P8**. Rationale: verified — the file is at
`minispec/Minispec/Vocabulary/TrustedBase.lean`, and all three law units import it
as `Minispec.Vocabulary.TrustedBase`. In a file whose stated job is "what binds
anyone standing in this directory," and where the correction itself requires the
frontier+human-authorized lane, a dangling path is disproportionately expensive.
Confidence: **SURE**.

*Same-cluster, spec surface — needs the authorized lane, not a builder:* the same
wrong path appears in three unit docstrings —
`minispec/Minispec/JoinIsAssociative.lean:20`, `JoinIsCommutative.lean:17`,
`JoinIsIdempotent.lean:16`. Per `law-spec-touch-frontier-human-only` these are
frontier-only, human-authorized edits. Worth batching with F1 in one authorized
pass.

---

### B2 — `sug-step-zero-git-spelling-mismatch`

`spike/CLAUDE.md:1014-1017` (standing text, contradicted by the arc's sharpened
`worktree-file-access-law` at `:1034-1045`)

Plain-language gloss: the arc just added a law saying "always spell mutating git
as `git -C <your absolute worktree path>`, never behind a `cd`" — because a
vanished worktree silently drops your shell into a sibling tree. But the
mandatory step-zero recipe, in the same file, still spells a mutating `git switch`
with no `-C`, at the single moment the agent's working directory is least
trustworthy (a fresh, possibly-reaped harness worktree).

> - **step-zero** (worktree agents only — `isolation: worktree` bases agents on a
>   possibly-stale `main`): `git switch -C <task-branch> <current-lineage-branch>`
>   (today: `ai/spike3-r23`), verify the tip hash matches what the conductor
>   stated, verify `pwd`; **step-0.5**: `mise trust`.

→

> - **step-zero** (worktree agents only — `isolation: worktree` bases agents on a
>   possibly-stale `main`): verify `pwd` FIRST, then
>   `git -C <that absolute path> switch -C <task-branch> <lineage-branch the
>   conductor named>` per `worktree-file-access-law`, then verify the tip hash
>   matches what the conductor stated; **step-0.5**: `mise trust` — and again
>   inside WSL before the first `mise run both` (`wsl-trust-per-worktree`).

Criterion: **P7** (one spelling per concept) + **P4**. Rationale: the arc's new
law is only as strong as the recipes that obey it, and this is the recipe every
worktree agent runs first. Folding in the `wsl-trust-per-worktree` cross-reference
is the same-edit-cost fix for the fact that that bullet currently sits ~170 lines
away from the only step that acts on it. Confidence: **SURE** on the `-C` respell;
**GUESS** on bundling the WSL clause here versus leaving it where it is.

---

### B3 — `sug-lineage-branch-hardcoded-stale`

`spike/CLAUDE.md:1016` (standing text; folded into B2's replacement above, listed
separately so it can be taken alone)

Plain-language gloss: the recipe hardcodes "today: `ai/spike3-r23`" as the branch
to base work on. That branch is seven rounds gone; the live lineage is `ai/main`.

> `(today: `ai/spike3-r23`)`

→ delete the parenthetical; the replacement text in B2 says "the lineage-branch
the conductor named," and every brief already carries it.

Criterion: **P6** (no time-sensitive language; use a pattern that cannot rot).
Rationale: verified — `ai/spike3-r23` no longer exists in the branch list; the tip
lineage is `ai/main` with r30 lanes hanging off it. A worktree agent that follows
this literally bases its branch on a nonexistent-or-ancient ref, which is the
exact failure the surrounding step-zero exists to prevent. A hardcoded branch
name in a file this long will rot again; deleting it is the durable fix.
Confidence: **SURE**.

---

### B5 — `sug-minispec-summary-drops-human-auth`

`spike/CLAUDE.md:856-858` (arc addition)

Plain-language gloss: the spike-side one-line summary of the minispec access law
says "builders never edit content there" — which a conductor correctly reads as
"…so I may." The authoritative law says nobody edits it without explicit human
authorization.

> `minispec/` is SPEC SURFACE under its own CLAUDE.md's
>   access laws — builders never edit content there; the catalogue lock's promote is a
>   spec-side act whose review is the git diff.

→

> `minispec/` is SPEC SURFACE under its own CLAUDE.md's
>   access laws — NOBODY edits content there without explicit human authorization,
>   conductors included; the catalogue lock's promote is a spec-side act whose
>   review is the git diff.

Criterion: **P8** (a summary that narrows a law's scope is a scope error).
Rationale: `minispec/CLAUDE.md`'s `law-spec-touch-frontier-human-only` is a
two-part lock (frontier-class **and** explicit human authorization); the spike-side
paraphrase keeps only the model-class half, and the audience most likely to read
only `spike/CLAUDE.md` is precisely the audience the dropped half binds.
Confidence: **SURE** on the gap; the sanctioned-repetition rule in `AGENTS.md`
already licenses restating a critical invariant, so this costs nothing.

---

### C1 — `sug-name-the-verified-core-modules`

`spike/crates/core/CLAUDE.md:124-134` (arc addition, `sorted-facade-law`)

Plain-language gloss: the new rule bans raw `BTreeMap`/`BTreeSet` "in verified-core
code" without saying which code that is. Taken literally against the referenced
skill (which says the verified core "lives in those two crates"), the rule would
condemn 151 existing uses in `analysis` and 16 in `core` — so a literal reader
either starts a mass refactor or writes the rule off as dead. Its sibling bullet
gets this right and names its modules.

> raw
>   `BTreeMap`/`BTreeSet` never appear in verified-core code (checker and reference
>   implementations included — the `verified-core-discipline` skill's code-shape rule).

→

> raw
>   `BTreeMap`/`BTreeSet` never appear in the verified-core modules — today
>   `core::sorted` and `analysis::lattice`, plus the checker and reference
>   implementations (the `verified-core-discipline` skill's code-shape rule).
>   Elsewhere in these crates the ordinary `inv-determinism` rule stands: raw
>   `BTree*` is fine where order is observable.

Criterion: **P8** (Opus reads literally and will not silently infer the intended
scope). Rationale: verified — raw `BTree*` appears in `core::{contested, coord,
lib, prov, sorted, unord}` and in `analysis::{cfg, effect, erase, funcenv, value}`,
so the unscoped reading is false today; the arc's own next bullet already spells
the correct scope as "the TRANSLATED algebra tier (`core::sorted`,
`analysis::lattice`)", and the aeneas fence config confirms exactly those two
modules. Confidence: **SURE**.

---

### D2 — `sug-mirror-the-translation-fence`

`spike/crates/analysis/CLAUDE.md` — add to the new `## Law — the solve-certifier`
section (or beside it)

Plain-language gloss: the rule "don't return borrows from closures" binds
`analysis::lattice`, but it is written down only in `core/CLAUDE.md`. The agent who
would break it is editing `analysis` and reads `analysis/CLAUDE.md`. Breaking it
fails silently — translation emits ill-typed Lean that only `verify:lean` catches.

Proposed addition:

> - **translation-fence-binds-lattice-too** — `lattice.rs` is inside the TRANSLATED
>   algebra tier: `core/CLAUDE.md` `keep-borrows-out-of-closure-returns` binds it
>   verbatim (no closure returning a borrow of its argument, no `mem::replace`
>   inside `.map`, no `unwrap_or_else(<trait method>)` — spell the `match` cousin).
>   A reintroduction breaks Aeneas translation SILENTLY; only `verify:lean` sees it.

Criterion: **P8** (state scope where the reader is) + **B2**-adjacent
(reachability of the routine path). Rationale: of every rule the arc added, this is
the one with a silent failure mode and a genuinely-not-obvious cause, and it is
filed in the one crate registry its likeliest violator will not open. Confidence:
**SURE** that the gap is real; **SUSPECT** on placement (a fourth section versus a
line in the existing dangers section — the dangers section is arguably the better
home, since its stated theme is "each one is a latent wrong-elision or a hang").

---

### E1 — `sug-fold-rederivation-into-survival`

`spike/crates/plan/CLAUDE.md:177-188` (arc addition)

Plain-language gloss: the new survival re-derivation law was appended as a brand-new
section at the bottom of the file, while a `## Law — the survival tier` section
already exists at line 114. Survival law now has two homes, and the file's own
header rule says to append into the matching section.

Two acceptable shapes, conductor's pick:

1. **Lower churn** — leave the new section where it is, and add one pointer bullet
   to `## Law — the survival tier`:
   > - **and-then-re-derived** — every survival additionally passes
   >   `rederivation-is-demote-only` (below) before the plan ships.
2. **Cleaner** — move the `rederivation-is-demote-only` bullet into
   `## Law — the survival tier` and drop the new section header entirely.

Criterion: **P7** (consistent structure) + the file's own stated registry
discipline at `:8` ("append to the matching section"). Rationale: a reader who
greps the survival section for everything binding a survival currently gets five
bullets and misses the gate that can demote them all. Confidence: **SURE** on the
finding; the two shapes are equally correct, so **V1** says pick one — I'd take (1)
if you want r30 provenance to stay visible in the section headers, (2) otherwise.

---

### A1 — `sug-direct-builders-imperatively`

`.claude/skills/conductor/SKILL.md:89-92` (arc addition)

Plain-language gloss: the quarantine section states what builders must have been
told, in the passive voice, rather than telling the conductor to tell them. The
conductor's actual action item — put this path in every builder brief — is only
implied.

> Opus/Sonnet and foreign-lineage models *may* read inside the quarantine; and in
> particular, *must* be directed to read this file before any other work:
>
> `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`

→

> Opus/Sonnet and foreign-lineage models *may* read inside the quarantine. Put
> this path in every such builder's brief, as its first read, before any other
> work:
>
> `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`

Criterion: **P4** (imperative form) + **P1** (tell Claude what to do). Rationale:
the surrounding paragraphs are conductor-addressed imperatives; this one silently
switches to describing a property of builders, and the standing memory note
("builders read `AGENTS.for-builders-only.md` first — brief = the pointer") says
the brief-carries-the-pointer step is the one that gets dropped. Path verified
correct. Confidence: **SURE**.

---

### A2 — `sug-afk-default-when-unsure`

`.claude/skills/conductor/SKILL.md:161-168` (arc addition; human-directed
2026-08-15 — **needs the human's ack, not just the conductor's**)

Plain-language gloss: the rule fires "whenever it's clear the human is AFK," but
never says what to do when it *isn't* clear — and it usually isn't. As written the
rule quietly defaults to off.

> **AFK glossing rule** (human-directed, 2026-08-15): whenever it's clear the
> human is AFK — reading only your chat output, not the output-files, ledgers, or
> documents you're citing —

→

> **AFK glossing rule** (human-directed, 2026-08-15): whenever the human may be
> AFK — reading only your chat output, not the output-files, ledgers, or documents
> you're citing — which is the assumption unless you have positive evidence they
> are reading along,

Criterion: **P8** (an unstated default is an off switch under literal reading).
Rationale: the trigger is unobservable from inside the session, and the costs are
asymmetric — over-glossing costs one line per finding, under-glossing costs the
human an untriageable report, which is the failure the rule was minted to fix. The
default-on reading is also what the human's standing preferences already ask for
(explainer-style status updates; unroll slugs in conversation). Flagging
explicitly because this *widens* a human-authored rule: it should get a cheap
explicit ack rather than being applied silently. Confidence: **SUSPECT**.

---

### A3 — `sug-frontmatter-description-truncated` (OUT OF ARC SCOPE)

`.claude/skills/conductor/SKILL.md:3` (standing text from the file's original
commit — flagged because it is cheap and ships in every session)

Plain-language gloss: the skill's one-line description ends mid-sentence: "…with
instructions for". It renders that way in the skill listing every session sees.

> description: Only for loading upon command ("you are a conductor"), this sets you up as a top-level conductor, with instructions for

→

> description: Sets you up as a top-level conductor for this project — subagent tiering, the required reading-guide, git/worktree hygiene, ledgering, and planning posture. Load on explicit command ("you are a conductor"), not by inference.

Criterion: **F2** (description leads with the use case) + **F4** (explicit trigger
phrasing) + **F1** (well within the 1024-char cap). Rationale: a dangling sentence
fragment is the model's only summary of this skill at selection time; it costs
tokens every session and conveys nothing after the parenthetical. Genuinely
outside the arc's additions — take it or leave it. Confidence: **SURE** that it is
a defect; **GUESS** that this exact wording is what you want.

---

### B6 — `sug-reaping-needs-concrete-names`

`spike/CLAUDE.md:864-871` (arc addition)

Plain-language gloss: the post-mortem bullet correctly insists on
`pkill -9 -x <name>` (exact-match) rather than `-f`, but never says which names.
An agent that has to guess the process name defeats the whole point of `-x`.

> Reap explicitly with exact-name `pkill -9 -x <name>` — never
>   `-f`, which once matched the killer's own wrapper shell.

→

> Reap explicitly with exact-name `pkill -9 -x <name>` — never `-f`, which once
>   matched the killer's own wrapper shell. The names worth knowing:
>   `cbmc`, `kani-driver`, `kani-compiler` (extend as lanes land).

Criterion: **P8**. Rationale: an incident-response instruction that requires the
responder to guess an operand is not executable under pressure, which is when it
gets read. Confidence: **GUESS** on the specific names — no kani task exists in the
primary checkout's `mise.toml` (the lane is still in flight on
`ai/r30-lane-kani`), so whoever lands that lane should supply the authoritative
list. Do not apply this one from my guess; treat it as a task for the kani lane's
close-out.

---

## Considered and rejected

- **minispec `frontier-class` self-identification** — I expected to flag that a
  Sonnet builder cannot reliably know whether it is "frontier-class," so the access
  law binds on an unverifiable self-assessment. Rejected: the law is a double lock
  ("ONLY frontier-class models … **and** only with explicit human authorization"),
  and the authorization half is checkable by the reader and fails safe. Reordering
  to lead with the checkable half is defensible but not worth churning a
  human-ack'd access law.
- **`minispec/CLAUDE.md` "byte-budget advisory 8KB"** — checked whether "advisory"
  understated a real gate, since `verify:check`'s description calls it a
  "tripwire." It does not: `spike/verify/src/check.rs:23` — "Printed, never fatal."
  Text is accurate.
- **`verify-lane-family`'s classification of `verify:report`** — the line reads
  "`verify:report -- --with-lean` are opt-in Linux/WSL lanes," which matches
  `mise.toml` exactly (bare `verify:report` is cheap-tier; the flag is the slow
  lane). No edit.
- **Adding `verify:*` to the mise task code block in `spike/CLAUDE.md`** — the
  block is already deliberately selective (it omits `gate:full-quiet`, `both`,
  `fmt`, `livetest`, `prose:census` too), with the rest carried in bullets. Not a
  gap, a convention.
- **AFK rule sitting under the `### Ledgering` heading** — it is a reporting law,
  not a ledgering one, but the paragraph immediately above it is also about the
  final chat message, so context carries it. Heading churn not justified.
- **`spike/CLAUDE.md:24` "this file and all eight crate files"** — there are nine
  crate `CLAUDE.md` files (aid, analysis, cli, core, hostsim, oracle, plan, syntax,
  weft) plus `spike/docs/CLAUDE.md`. Stale by one, but no agent behaves differently
  for it. Fix it if you are in the file anyway.
- **Double blank line after the quarantine block** (`SKILL.md:101-102`) —
  formatting only; the skill's own cost-gate excludes format-only edits.
- **`spike/CLAUDE.md` "Where the build stands" lacks an r30 bullet** — the section
  self-declares as the one drift-expected section and points at `LIVING_STATUS.md`.
  Sanctioned drift, not a criteria finding. Noted below anyway since you are
  closing an arc.

---

## Adjacent observations (non-target files — for the conductor's queue)

- **`obs-skill-points-at-old-lean-home`** —
  `.claude/skills/verified-core-discipline/SKILL.md:166-168` says "the
  sparing-algebra model and its report live under
  `.claude/research/refinement-types-industrial-cost/spike-lean-sparing/`." The r30
  arc landed `minispec/` as the reviewable Lean surface with `spike/verify/`
  tooling. Both trees exist, so nothing dangles — but this skill is loaded by
  agents specifically to orient them near the verified core, and it currently
  points only at the pre-arc home. Highest-value item on this list; it is a
  one-sentence fix and the file is outside my remit.
- **`obs-solve-rs-doccomment-mirrors-conflict`** — `analysis/src/solve.rs:13`
  carries the same superseded "MUST check `converged`" instruction as D1. Source
  comment, same sweep.
- **`obs-build-stands-section-lacks-r30`** — `spike/CLAUDE.md:762` is dated
  2026-08-13 and predates the minispec landing, the solve-certifier, and the
  survival re-derivation. Sanctioned drift by the section's own contract; still,
  an arc-closing conductor probably wants the bullet.

---

## Provenance for a successor

Everything above was verified against the tree at `e1e9008a`; the checks that
matter, so you need not redo them:

- `Research/quarantine-DO-NOT-READ/` is the only quarantine dir (directory-name
  search only; I did not enter it). → B4
- `minispec/Minispec/Vocabulary/TrustedBase.lean` exists; `Minispec/TrustedBase.lean`
  does not; the three units import `Minispec.Vocabulary.TrustedBase`. → F1
- `effect.rs:1789` sets `trust_reach` from `reach_consistency.is_consistent()`;
  `solve_certified` lives at `certify.rs:443`; the lexical fence is at
  `certify.rs:1369`. → D1
- raw `BTree*` counts: `core` 16 across 6 files, `analysis` 151 across 5 files; the
  aeneas fence (`spike/verify/aeneas/Cargo.toml`) scopes translation to the
  `sorted` and `lattice` modules. → C1
- branch list holds `ai/main`, `ai/r30-conduct`, `ai/r30-lane-kani`; no
  `ai/spike3-r23`. → B3
- `mise.toml:451-489` — every `verify:*` name and the `-- --with-lean` flag in the
  arc's text match; `verify:check` genuinely rides both legs cheaply. → no finding
- `dorc-sparing-reference` is the real package name; `plan/src/rederive.rs`,
  `wall_walk_survival` (`plan/src/lib.rs:3707`), and
  `plan/tests/sparing_differential.rs` all exist. → no finding
- `Research/notes/300`, `301`, `303`, `304` and `Research/plans/302` all exist as
  cited. → no finding
