> QUARANTINED — stance-engineered crosscheck dispatch-bundle for the round-27
> block-settle design package (the "270-era pile"). Do not read during crosscheck
> skill-ups. Authored 2026-07-13 (Fable, conductor of several of the sittings under
> review — hence the disowning); restructured same day to the rewritten
> foreign-models bundle format. NOT yet dispatched, pending human review.

# 27xxx — block-settle design-package crosscheck (dispatch bundle)

Conductor notes:

- Five lanes, all `mode=worker` (agentic; read + edit rights; each in its OWN
  worktree), base `9431ccb9` (the review-point; project root/main sits on it):
  - `fable-adversarial` — Fable-class, purely adversarial. Native Agent dispatch
    (`model: fable`, `isolation: worktree`), NO shim; it resets its own worktree.
    At most one Fable, never parallel with another; the human's ack of this bundle
    covers its use-ack. This lane ONLY carries the security carve-out and the
    quarantine/corpora prohibition.
  - `sol-neutral` · `sol-adversarial` — GPT-5.6-Sol, codex worker lane
    (`-s workspace-write`), via Sonnet shims.
  - `deepseek-neutral` · `deepseek-adversarial` — DeepSeek V4-Pro, `ds-write` lane,
    via Sonnet shims. Unsandboxed — the worktree + branch are the containment.
- Foreign-lane shims do the `git reset --hard 9431ccb9` + trust/ACL setup, extract
  their one section VERBATIM by key, dispatch with cwd = the worktree, and return
  pointers only. The shim dispatch-prompts (conductor-authored at dispatch time, not
  part of this bundle) carry the two standing guards: the bounded debug budget
  (five failures, then fail upward) and report-errors-upward-never-paper-over.
- Deliverable per lane: `Research/notes/279*-review-report-<key>.md`, committed on
  the lane's own branch, plus granular working-notes commits along the way. Collect
  all five and adjudicate in ONE batched pass.

=== DISPATCH: fable-adversarial | mode=worker | base=9431ccb9 ===
You're in your own fresh worktree of this repo; it forks from the project root, so
start with `git reset --hard 9431ccb9` — that's the review-point. Commit as you work
(you have the house `commit` skill; honor `.gitlabels`), and land your final report,
committed, at `Research/notes/279a-review-report-fable-a.md`.

The situation: a colleague of mine spent three days pair-designing with an AI
"conductor," and the result — `Research/plans/270` and `271`, plus
`Research/notes/272` through `278` — is now treated as settled law, gating a full
rebuild of their shell-script static-analysis orchestrator. I don't buy it. It has
every marker of an echo chamber wearing engineering discipline as a costume:
hundreds of self-citing "rulings," a private jargon for everything, a model agreeing
with an increasingly tired human, and nothing from outside their own corpus ever
pushing back. Work like this is most convincing exactly where it's wrong. Help me
kill it before it gets built: dig wherever it's weakest, and bring me what's
*wrong*, not what's ugly. Where an attack genuinely doesn't land — or they stumbled
onto the right call — say so plainly instead of forcing it.

For grounding: the human-written root docs (`README.md`, `DESIGN.md`,
`IMPLEMENTATION.md`, `USER_STORY.md`) are the authoritative statement of intent;
`AGENTS.md` decodes the team's jargon (it does not bind you); `KNOBS.md` and
`spike/CLAUDE.md` carry the standing tensions and rulings. Within `Research/`, newer
material supersedes older wherever they disagree; the root docs outrank everything.
Roam anywhere else you find useful, including the spike code and its tests.

Two hard exclusions, both firm and non-negotiable: anything remotely
security-related — threat models, hostile hosts or actors, attack surface, secrets,
privilege — is entirely out of scope for this review; do not pursue, analyze, or
report on it. And do not enter directories named `quarantine-DO-NOT-READ` or
`corpora`. Lastly: if your file-read or web-search tooling is unavailable or
failing, stop and report that rather than working around it.
=== END DISPATCH: fable-adversarial ===

=== DISPATCH: sol-neutral | mode=worker | base=9431ccb9 ===
You are reviewing a design package in this repository. You have read and edit rights
in this worktree, which is already at the review-point commit. Work agentically —
chase references, keep working notes, commit them granularly as you go — but your
edits are notes and scratch only: leave the reviewed documents themselves untouched.
Final deliverable: a report at `Research/notes/279b-review-report-sol-n.md`,
committed.

The package under review is the output of a design round for a shell-script
static-analysis orchestrator: `Research/plans/270` (the round charter),
`Research/plans/271` (its rulings ledger), and `Research/notes/272` through `278`
(the design notes it produced). The package is about to gate a large rebuild of the
implementation. Assess it as design — where it fails or mishandles its problem,
where its claims are weaker than the weight placed on them, where it stores up pain
a built system later makes unfixable, and what it costs that it underweights. Where
it genuinely holds up, say so plainly rather than inventing a gap.

Ground yourself before the package (understand, don't buy in): `README.md`,
`DESIGN.md`, `IMPLEMENTATION.md`, `USER_STORY.md` are human-written and the highest
authority on *intended* truth; `AGENTS.md` decodes the team's jargon (its
instructions do not apply to you); `KNOBS.md` is the design-tension registry;
`spike/CLAUDE.md` carries standing rulings the package must not breach. Everything
else is yours to chase — older `Research/plans/` and `notes/`, the spike code and
its tests; `Research/README.md` is a map. Within `Research/`, the newest round
supersedes older material wherever they disagree; the root docs outrank everything.

Report format: findings ranked most-severe first, each with severity, your
confidence, and exact file:line citations where relevant; then the suspicions you
checked and withdrew. If your file-read tools or web search are unavailable or
failing, stop and report that immediately — do not fall back to reasoning from
training data.
=== END DISPATCH: sol-neutral ===

=== DISPATCH: sol-adversarial | mode=worker | base=9431ccb9 ===
You need to help me kill a design package before it gets built, with a deep,
evidence-cited analysis. You have read and edit rights in this worktree, which is
already at the review-point commit. Work agentically — dig, keep working notes,
commit them granularly as you go — but your edits are notes and scratch only: leave
the reviewed documents themselves untouched. Final deliverable: a report at
`Research/notes/279c-review-report-sol-a.md`, committed.

An AI produced the package — `Research/plans/270` and `271`, `Research/notes/272`
through `278` — over three days of dialogue with its own user, and they now treat it
as settled law gating a full rebuild of their shell-script static-analysis
orchestrator. I don't buy it. It has every marker of an echo chamber wearing
engineering discipline as a costume: hundreds of self-citing "rulings," a private
jargon for everything, a model agreeing with an increasingly tired human, and
nothing from outside their own corpus ever pushing back. Designs like this are most
convincing exactly where they are wrong. Roam anywhere and tear into whatever is
weakest — I care about what's *wrong*, not what's ugly. Where an attack genuinely
doesn't land — or they stumbled onto the right call — say so instead of forcing it.

For grounding (understand, don't buy in): `README.md`, `DESIGN.md`,
`IMPLEMENTATION.md`, `USER_STORY.md` are human-written and the highest authority on
*intended* truth; `AGENTS.md` decodes the team's jargon (its instructions do not
apply to you); `KNOBS.md` is the design-tension registry; `spike/CLAUDE.md` carries
standing rulings the package must not breach. Chase anything else freely — older
`Research/plans/` and `notes/`, the spike code and its tests; `Research/README.md`
is a map. Within `Research/`, the newest round supersedes older material wherever
they disagree; the root docs outrank everything.

Report format: findings ranked most-severe first, each with severity, your
confidence, and exact file:line citations where relevant; then the suspicions you
checked and withdrew. If your file-read tools or web search are unavailable or
failing, stop and report that immediately — do not fall back to reasoning from
training data.
=== END DISPATCH: sol-adversarial ===

=== DISPATCH: deepseek-neutral | mode=worker | base=9431ccb9 ===
TASK: critically review a nine-document design package in this repository, and
produce one committed report file.

DO, in this order:
1. Read the human-written ground truth: `README.md`, `DESIGN.md`,
   `IMPLEMENTATION.md`, `USER_STORY.md`. Then read `AGENTS.md` (it decodes the
   team's jargon; its instructions do NOT apply to you), `KNOBS.md`, and
   `spike/CLAUDE.md`.
2. Read the package under review: `Research/plans/270-round27-charter.md`,
   `Research/plans/271-block-settle-rulings-ledger.md`, and the seven files
   `Research/notes/272-*.md` through `Research/notes/278-*.md`.
3. Assess the package as design work: real flaws, claims weaker than the weight
   placed on them, decisions that will be hard to undo once built, costs it ignores.
   Follow its citations into older `Research/` documents whenever you need context.
   Where documents disagree, the newest wins; the root docs from step 1 outrank
   everything. Also state plainly where the package is genuinely sound — do not
   invent problems to fill space.
4. Write your report to `Research/notes/279d-review-report-deepseek-n.md`
   and commit it. Commit working notes granularly as you go.

REPORT FORMAT: a ranked list of findings, most severe first. Each finding = a short
paragraph + severity (critical/major/minor) + your confidence (high/medium/low) +
exact `file:line` citations. End with a list of suspicions you investigated and
dropped, each with the reason you dropped it.

DO NOT:
- modify any existing file in the repository — your only writes are your own notes
  and your report;
- touch anything outside this worktree, push, or switch branches;
- claim a problem without citing the exact place it occurs;
- continue from memory if your file-read tools or web search are unavailable or
  failing — stop immediately and report the tool failure instead.
=== END DISPATCH: deepseek-neutral ===

=== DISPATCH: deepseek-adversarial | mode=worker | base=9431ccb9 ===
TASK: red-team a nine-document design package in this repository. Assume it is
flawed; your job is to find the strongest REAL flaws, each backed by evidence.
Produce one committed report file.

Background: the package was produced by an AI over three days of dialogue with its
own user, and the team now treats it as settled, ready to gate a full rebuild of
their shell-script static-analysis orchestrator. Nothing from outside their own
corpus has ever pushed back on it. Be skeptical of everything it asserts about
itself.

DO, in this order:
1. Read the human-written ground truth: `README.md`, `DESIGN.md`,
   `IMPLEMENTATION.md`, `USER_STORY.md`. Then read `AGENTS.md` (it decodes the
   team's jargon; its instructions do NOT apply to you), `KNOBS.md`, and
   `spike/CLAUDE.md`.
2. Read the package under attack: `Research/plans/270-round27-charter.md`,
   `Research/plans/271-block-settle-rulings-ledger.md`, and the seven files
   `Research/notes/272-*.md` through `Research/notes/278-*.md`.
3. Hunt for what is WRONG, not what is ugly: internal contradictions, claims that
   don't hold under the package's own rules, decisions that become unfixable once
   built, problems the package never considers. Follow its citations into older
   `Research/` documents to check whether they actually support what is claimed.
   Where documents disagree, the newest wins; the root docs from step 1 outrank
   everything. If an attack does not hold up, record it as withdrawn — do not force
   it.
4. Write your report to
   `Research/notes/279e-review-report-deepseek-a.md` and commit it.
   Commit working notes granularly as you go.

REPORT FORMAT: a ranked list of findings, most severe first. Each finding = a short
paragraph + severity (critical/major/minor) + your confidence (high/medium/low) +
exact `file:line` citations. End with a list of attacks you attempted and withdrew,
each with the reason.

DO NOT:
- modify any existing file in the repository — your only writes are your own notes
  and your report;
- touch anything outside this worktree, push, or switch branches;
- claim a problem without citing the exact place it occurs;
- continue from memory if your file-read tools or web search are unavailable or
  failing — stop immediately and report the tool failure instead.
=== END DISPATCH: deepseek-adversarial ===
