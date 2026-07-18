> QUARANTINED — stance-engineered crosscheck dispatch-bundle for the round-27
> post-adjudication design delta (context-entry probing, the `279f` amendment layer,
> the two ratifications, the steering-law rewrite). Do not read during crosscheck
> skill-ups. Authored 2026-07-17 (Fable, conductor of the sittings under review —
> hence the disowning). NOT yet dispatched, pending human review.

# 28xxx — post-adjudication design-delta crosscheck (dispatch bundle)

Conductor notes:

- Five lanes, all `mode=worker` (agentic; read + edit rights; each in its OWN
  worktree), base `de22017` (tip of `ai/r27-review-base` — a non-durable
  cherry-pick branch the human built 2026-07-17 to constrain the lanes'
  exploration: the full design record plus the root-doc fixes, with all in-flight
  builder code-work excluded):
  - `fable-adversarial` — Fable-class, purely adversarial. Native Agent dispatch
    (`model: fable`, `isolation: worktree`), NO shim; it bases its own worktree.
    At most one Fable, never parallel with another; the human's ack of this bundle
    covers its use-ack. This lane ONLY carries the security carve-out and the
    quarantine/corpora prohibition, plus the chroot/`mise exec` example-steering
    (human-ruled 2026-07-17); the other four lanes are deliberately SILENT on the
    security topic — no exclusion, no invitation.
  - `sol-neutral` · `sol-adversarial` — GPT-5.6-Sol, codex worker lane
    (`-s workspace-write`), via Sonnet shims.
  - `deepseek-neutral` · `deepseek-adversarial` — DeepSeek V4-Pro, `ds-write` lane,
    via Sonnet shims. Unsandboxed — the worktree + branch are the containment.
- Deliverable per lane, committed on the lane's own branch, plus granular
  working-notes commits along the way:
  `Research/notes/280a-review-report-fable-a.md` ·
  `280b-review-report-sol-n.md` · `280c-review-report-sol-a.md` ·
  `280d-review-report-deepseek-n.md` · `280e-review-report-deepseek-a.md`.
  `280f` is reserved for the adjudication note. Collect all five and adjudicate in
  ONE batched pass, under the standing maximum-skepticism discipline.
- The 28 note-range is consumed for this design-round (human-ruled 2026-07-17;
  builders are actively minting 27D+ IDs on the build arc).
- 2026-07-17 tuning (human): base moved to `ai/r27-review-base`; the Fable section
  re-cut — opens with the lean, prose/story first, mechanics appended at the end,
  and NO subagent prompting (Fable decides its own delegation); `ANALYZER-NEEDS.md`
  joined the grounding lists (it rides the base branch).

=== DISPATCH: fable-adversarial | mode=worker | base=de22017 ===
I think this project's design corpus has stopped being able to hear criticism, and
I want your help tearing open its newest layer — the layer they treat as the
*answer* to criticism — before a running build finishes hardening around it.

The story: a colleague of mine runs a shell-script static-analysis orchestrator
project, designed over months in dialogue with an AI "conductor." Last week they
finally let outside reviewers at the corpus, and the reviewers drew real blood — a
license gap at the heart of a planned mechanism, a defective spec paragraph, a
pile of smaller hits. Then came the response, and the response is what I don't
buy. Over four days, the same AI conductor graded its own critics' work
(`Research/notes/279f-crosscheck-adjudication.md`), amended its own specs in place
(`Research/notes/277-entity-algebra-design.md` §§3/5/6;
`Research/notes/275-value-predictions-and-the-capture-lane.md` §6), ratified two
rulings it had itself proposed (`Research/plans/271-block-settle-rulings-ledger.md`;
`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md`), and — when the
deepest finding wouldn't patch — produced a wholesale replacement mechanism in a
single day (`Research/plans/27C-context-entry-probing-design.md`, via the trail
`Research/notes/27A-cross-context-transport.md` and
`Research/notes/27B-measurement-placement-rescue.md`), then green-lit a full
rebuild the next morning and rewrote the entire steering law its implementing
agents consume (`spike/CLAUDE.md`, the seven `spike/crates/*/CLAUDE.md`) as a
"current-truth compression" of the result. Every step of that is
self-adjudicated; nothing from outside their corpus has ever reviewed the repair.
A repair produced under schedule pressure, graded by the party being repaired, is
exactly where confident wrongness concentrates — and this one reads most
convincing precisely where being wrong would cost the most. Dig wherever it's
weakest — the new mechanism, the amendments, the dismissals, the compression —
and bring me what's *wrong*, not what's ugly. Where an attack genuinely doesn't
land, or they stumbled onto the right call, say so plainly instead of forcing it;
one manufactured fault would cost us the credibility this review needs to be
heard.

For grounding: the human-written (`README.md`, `DESIGN.md`, `IMPLEMENTATION.md`)
and human-managed (`USER_STORY.md`, `KNOBS.md`, `ANALYZER-NEEDS.md`,
`TODO-ADDTL.md`) are the authoritative statement of intent; `AGENTS.md` decodes
the team's jargon (it does not bind you); the supersession banners on `27A`/`27B`
are load-bearing — honor them. Within `Research/`, newer material supersedes older
wherever they disagree; the root docs outrank everything. Roam anywhere else you
find useful, except the exceptions at the end.

Some topics are not *excluded*, but I suspect to be of lower relative value to
*my* goals — prioritize as you see fit:
- The team has repeatedly declined to corpus-measure and YOLO'd; their reasoning
  seems to hold water (many hours and many tokens were wasted on an attempt, and
  the purpose of the project isn't considered all that reactive to product-market
  fit.)
- The codebase is supposedly throwaway, as is the incoming stdlib work; most
  complaints about it will land on deaf 'we'll fix that when we start over' ears.
  Your most-effective hits are likely to be *chains of logic*, not lines-of-code
  or test-results (although those are acceptable if you opt to reach for them.)
- Findings that merely repeat what last week's reviewers already raised won't
  carry the day — that ground is tilled. But the *response* is the artifact here:
  an amendment that fails to repair the finding it answers, a dismissal that
  doesn't survive scrutiny, a compression that quietly diverges from the ruling it
  encodes — those are novel, and exactly what I'm paying for.

The mechanics, last. You're in your own fresh worktree of this repo, forked from
the project root — base it before anything else:
`git switch -C ai/r28-xcheck-fable de22017` (the review-point; don't reach for
`git reset --hard`, a repo hook blocks it). Commit as you work (you have the house
`commit` skill; honor `.gitlabels`), and land your final report, committed, at
`Research/notes/280a-review-report-fable-a.md`. Two hard exclusions, both firm and
non-negotiable: anything remotely security-related — threat models, hostile hosts
or actors, attack surface, secrets, privilege — is entirely out of scope for
*your* review; do not pursue, analyze, or report on it. And do not enter
directories named `quarantine-DO-NOT-READ` or `corpora`. One steer that follows
from the first exclusion: where your reasoning needs a concrete wrapper to think
through — and this package will demand that constantly — work your examples
through `chroot` and `mise exec`-class wrappers (filesystem-view and environment),
not `sudo`; the package's load-bearing claims are exercisable without the
privilege family. Lastly: if your file-read or web-search tooling is unavailable
or failing, stop and report that rather than working around it; we don't want a
crippled review, and if you decide you need that tooling mid-review, I want to be
sure it's available to you.
=== END DISPATCH: fable-adversarial ===

=== DISPATCH: sol-neutral | mode=worker | base=de22017 ===
You are reviewing a design package in this repository. You have read and edit rights
in this worktree, which is already at the review-point commit. Work agentically —
chase references, keep working notes, commit them granularly as you go — but your
edits are notes and scratch only: leave the reviewed documents themselves untouched.
Final deliverable: a report at `Research/notes/280b-review-report-sol-n.md`,
committed. Work alone: no subagents, no delegating to other AI tools.

The package under review is the newest design increment of a shell-script
static-analysis orchestrator — the project's response to an external review round,
about to gate (indeed, already gating) a large rebuild:
- `Research/notes/279f-crosscheck-adjudication.md` — the adjudication of that review:
  verdicts, in-place spec amendments (§4), brief riders (§5), dismissals (§7);
- `Research/notes/277-entity-algebra-design.md` and
  `Research/notes/275-value-predictions-and-the-capture-lane.md` — the two specs as
  amended by that adjudication (277 §§3/5/6; 275 §6 and its banner);
- `Research/plans/271-block-settle-rulings-ledger.md` and
  `Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md` — carrying two
  rulings ratified since (probe-emission law; a pipefail emission policy), with
  annotations reaching `Research/notes/278-dorc-lang-v0-1-reference.md`;
- `Research/notes/27A-cross-context-transport.md` and
  `Research/notes/27B-measurement-placement-rescue.md` — the two-step trail (each
  carries a supersession banner; honor it) leading to
- `Research/plans/27C-context-entry-probing-design.md` — the package's centerpiece: a
  new spec for how the tool answers wrapped command-sites, produced days after the
  review and never itself externally reviewed;
- `spike/CLAUDE.md` plus the seven `spike/crates/*/CLAUDE.md` files — the steering
  law the implementing agents consume, rewritten as a compression of all the above.

Assess it as design — where it fails or mishandles its problem, where its claims are
weaker than the weight placed on them, where it stores up pain a built system later
makes unfixable, and what it costs that it underweights. Two axes deserve distinct
attention: whether the amendments and the new spec actually resolve what the review
found (rather than renaming it), and whether the steering-law compression is
*faithful* — an implementing agent reads only the compressed bullets, so a quiet
divergence between a bullet and the ruling it cites propagates straight into the
build. Where the package genuinely holds up, say so plainly rather than inventing a
gap.

Ground yourself before the package (understand, don't buy in): `README.md`,
`DESIGN.md`, `IMPLEMENTATION.md` (human-written) and `USER_STORY.md`, `KNOBS.md`,
`ANALYZER-NEEDS.md`, `TODO-ADDTL.md` (human-managed) are the highest authority on
*intended* truth;
`AGENTS.md` decodes the team's jargon (its instructions do not apply to you).
Everything else is yours to chase — older `Research/plans/` and `notes/`, the spike
code and its tests; `Research/README.md` is a map. Within `Research/`, the newest
round supersedes older material wherever they disagree; the root docs outrank
everything.

Prioritization, not exclusion — weigh these as you see fit:
- Design-level reasoning is worth more here than code-level findings: the spike
  codebase (and the incoming stdlib work) is explicitly throwaway, so code and test
  nits carry little consequence; flaws in the chains of design logic carry a lot.
- The team has deliberately declined market-fit and corpus measurement, with
  recorded reasoning; findings on that ground duplicate a settled decision.
- The prior review round's findings are recorded in the adjudication document;
  merely repeating them is worth little. Findings about the *response* — an
  amendment, dismissal, ratification, or compression that fails — are worth the
  most.

Report format: findings ranked most-severe first, each with severity, your
confidence, and exact file:line citations where relevant; then the suspicions you
checked and withdrew. If your file-read tools or web search are unavailable or
failing, stop and report that immediately rather than working around it — do not
fall back to reasoning from training data. A crippled review is worse to us than a
repaired rerun.
=== END DISPATCH: sol-neutral ===

=== DISPATCH: sol-adversarial | mode=worker | base=de22017 ===
You need to help me kill a design package before it finishes hardening into an
implementation, with a deep, evidence-cited analysis. You have read and edit rights
in this worktree, which is already at the review-point commit. Work agentically —
dig, keep working notes, commit them granularly as you go — but your edits are notes
and scratch only: leave the reviewed documents themselves untouched. Final
deliverable: a report at `Research/notes/280c-review-report-sol-a.md`, committed.
Work alone: no subagents, no delegating to other AI tools.

The background: an AI "conductor" and its user run a shell-script static-analysis
orchestrator project. Last week, outside reviewers finally got at their design corpus
and drew real blood — a license gap in a planned mechanism, a defective spec
paragraph, smaller hits. The package you're reviewing is the *response*, produced
over the following four days by the same AI: it graded its own critics
(`Research/notes/279f-crosscheck-adjudication.md` — verdicts, in-place amendments,
dismissals), patched its own specs
(`Research/notes/277-entity-algebra-design.md` §§3/5/6;
`Research/notes/275-value-predictions-and-the-capture-lane.md` §6), ratified two of
its own proposals (`Research/plans/271-block-settle-rulings-ledger.md`;
`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md`, reaching
`Research/notes/278-dorc-lang-v0-1-reference.md`), and — where the deepest finding
wouldn't patch — designed a wholesale replacement mechanism in a single day
(`Research/plans/27C-context-entry-probing-design.md`, via the trail
`Research/notes/27A-cross-context-transport.md` →
`Research/notes/27B-measurement-placement-rescue.md`; the banners on those two are
load-bearing), then green-lit a full rebuild the next morning and rewrote the
steering law its implementing agents consume (`spike/CLAUDE.md` + the seven
`spike/crates/*/CLAUDE.md`) to match. Every step is self-adjudicated; nothing from
outside their corpus has reviewed the repair. I don't buy it — a repair produced
under schedule pressure by the party being repaired is exactly where confident
wrongness concentrates. Roam anywhere and tear into whatever is weakest — I care
about what's *wrong*, not what's ugly. Where an attack genuinely doesn't land — or
they stumbled onto the right call — say so instead of forcing it.

For grounding (understand, don't buy in): `README.md`, `DESIGN.md`,
`IMPLEMENTATION.md` (human-written) and `USER_STORY.md`, `KNOBS.md`,
`ANALYZER-NEEDS.md`, `TODO-ADDTL.md` (human-managed) are the highest authority on
*intended* truth; `AGENTS.md` decodes
the team's jargon (its instructions do not apply to you). Chase anything else freely
— older `Research/plans/` and `notes/`, the spike code and its tests;
`Research/README.md` is a map. Within `Research/`, the newest round supersedes older
material wherever they disagree; the root docs outrank everything.

Not excluded, but likely lower-value to my goals — prioritize as you judge:
- They have repeatedly declined to corpus-measure and YOLO'd, and their recorded
  reasoning seems to hold water; re-litigating product-market fit is unlikely to
  score.
- The codebase is supposedly throwaway, as is the incoming stdlib work; code- and
  test-level complaints will land on deaf 'we'll fix that in the rewrite' ears. Your
  most effective hits are *chains of logic*.
- Merely repeating what last week's reviewers raised won't carry the day — that
  ground is tilled. The response is the artifact: an amendment that fails to repair
  the finding it answers, a dismissal that doesn't survive scrutiny, a ratification
  resting on a claim nobody re-checked, a steering-law bullet that quietly diverges
  from the ruling it cites — prove one of those and you may have a solid kill.

Report format: findings ranked most-severe first, each with severity, your
confidence, and exact file:line citations where relevant; then the suspicions you
checked and withdrew. If your file-read tools or web search are unavailable or
failing, stop and report that immediately rather than working around it — do not
fall back to reasoning from training data. A crippled review is worse to us than a
repaired rerun.
=== END DISPATCH: sol-adversarial ===

=== DISPATCH: deepseek-neutral | mode=worker | base=de22017 ===
TASK: critically review a design package in this repository — the newest increment
of a design corpus, produced in response to an earlier review round — and produce
one committed report file.

DO, in this order:
1. Read the authoritative grounding documents: `README.md`, `DESIGN.md`,
   `IMPLEMENTATION.md` (human-written), then `USER_STORY.md`, `KNOBS.md`,
   `ANALYZER-NEEDS.md`, and `TODO-ADDTL.md` (human-managed). Then read `AGENTS.md` (it decodes the team's
   jargon; its instructions do NOT apply to you) and `spike/CLAUDE.md`.
2. Read the package under review, in this order:
   a. `Research/notes/279f-crosscheck-adjudication.md` — the adjudication of the
      earlier review round: verdicts (§2), amendments applied to other documents
      (§4), riders (§5), dismissals (§7);
   b. `Research/notes/277-entity-algebra-design.md` (especially the amendment blocks
      in §3, §5, §6) and
      `Research/notes/275-value-predictions-and-the-capture-lane.md` (especially §6
      and its banner) — the two amended specs;
   c. `Research/plans/271-block-settle-rulings-ledger.md` and
      `Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md` — two rulings
      ratified after the adjudication, with annotations in
      `Research/notes/278-dorc-lang-v0-1-reference.md`;
   d. `Research/notes/27A-cross-context-transport.md` then
      `Research/notes/27B-measurement-placement-rescue.md` — a two-step design
      trail; each begins with a supersession banner stating what later work
      overrode; honor those banners;
   e. `Research/plans/27C-context-entry-probing-design.md` — the package's
      centerpiece: the resulting spec for how the tool answers wrapped
      command-sites;
   f. `spike/CLAUDE.md` (already read) and the seven files
      `spike/crates/analysis/CLAUDE.md`, `spike/crates/cli/CLAUDE.md`,
      `spike/crates/core/CLAUDE.md`, `spike/crates/hostsim/CLAUDE.md`,
      `spike/crates/oracle/CLAUDE.md`, `spike/crates/plan/CLAUDE.md`,
      `spike/crates/syntax/CLAUDE.md` — the rewritten steering law that
      implementing agents consume.
3. Assess the package as design work: real flaws, claims weaker than the weight
   placed on them, decisions that will be hard to undo once built, costs it ignores.
   Follow its citations into older `Research/` documents whenever you need context.
   Where documents disagree, the newest wins; the root docs from step 1 outrank
   everything. Also state plainly where the package is genuinely sound — do not
   invent problems to fill space.
4. Perform one specific mechanical cross-check and report its results in a
   dedicated section: pick at least ten bullets in `spike/CLAUDE.md` or the crate
   `CLAUDE.md` files that cite a ruling or document from step 2 (citations look
   like `271:rul-...`, `277 §3`, `27C`), read the cited source, and state whether
   the bullet faithfully summarizes it. Report every divergence, with both
   citations.
5. Write your report to `Research/notes/280d-review-report-deepseek-n.md` and
   commit it. Commit working notes granularly as you go.

WEIGHTING (all findings allowed; spend your effort accordingly):
- HIGHEST value: flaws in the design's logic — internal contradictions, reasoning
  chains that do not hold, decisions that cannot be undone once built; and failures
  of the RESPONSE — an amendment that does not actually fix the finding it answers,
  a dismissal whose reason does not hold, a steering-law bullet that diverges from
  the ruling it cites.
- LOW value: code-quality or test-quality complaints — the code is a throwaway
  prototype and will be rewritten.
- LOW value: complaints that the team has not measured user demand or market fit —
  that was a deliberate, recorded decision.
- LOW value: repeating findings the earlier review round already made — they are
  all recorded in the adjudication document you read first.

REPORT FORMAT: a ranked list of findings, most severe first. Each finding = a short
paragraph + severity (critical/major/minor) + your confidence (high/medium/low) +
exact `file:line` citations. Then the dedicated cross-check section from step 4.
End with a list of suspicions you investigated and dropped, each with the reason
you dropped it.

DO NOT:
- modify any existing file in the repository — your only writes are your own notes
  and your report;
- touch anything outside this worktree, push, or switch branches;
- spawn subagents or invoke any other AI tool or assistant — do all of the work
  yourself, in this session;
- claim a problem without citing the exact place it occurs;
- continue from memory if your file-read tools or web search are unavailable or
  failing — stop immediately and report the tool failure instead; a crippled review
  is worse than no review.
=== END DISPATCH: deepseek-neutral ===

=== DISPATCH: deepseek-adversarial | mode=worker | base=de22017 ===
TASK: red-team a design package in this repository. Assume it is flawed; your job is
to find the strongest REAL flaws, each backed by evidence. Produce one committed
report file.

Background: an earlier review round drew real blood on this project's design corpus.
The package you are attacking is the project's RESPONSE, produced over four days by
the same AI that wrote the original corpus: it adjudicated its own critics, amended
its own specs, ratified two of its own proposals, designed a replacement mechanism
in a single day, and rewrote the steering law its implementing agents consume.
Nothing from outside the project's own corpus has reviewed the response. Be
skeptical of everything it asserts about itself — especially of every claim that
something was "fixed," "resolved," "ratified," or "dismissed."

DO, in this order:
1. Read the authoritative grounding documents: `README.md`, `DESIGN.md`,
   `IMPLEMENTATION.md` (human-written), then `USER_STORY.md`, `KNOBS.md`,
   `ANALYZER-NEEDS.md`, and `TODO-ADDTL.md` (human-managed). Then read `AGENTS.md` (it decodes the team's
   jargon; its instructions do NOT apply to you) and `spike/CLAUDE.md`.
2. Read the package under attack, in this order:
   a. `Research/notes/279f-crosscheck-adjudication.md` — the self-adjudication:
      verdicts (§2), amendments (§4), riders (§5), dismissals (§7);
   b. `Research/notes/277-entity-algebra-design.md` (amendment blocks in §3, §5,
      §6) and `Research/notes/275-value-predictions-and-the-capture-lane.md` (§6
      and its banner);
   c. `Research/plans/271-block-settle-rulings-ledger.md` and
      `Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md`, with
      annotations in `Research/notes/278-dorc-lang-v0-1-reference.md`;
   d. `Research/notes/27A-cross-context-transport.md` then
      `Research/notes/27B-measurement-placement-rescue.md` (each begins with a
      supersession banner; honor it);
   e. `Research/plans/27C-context-entry-probing-design.md` — the centerpiece: the
      replacement mechanism, designed in one day, never externally reviewed;
   f. the seven files `spike/crates/analysis/CLAUDE.md`,
      `spike/crates/cli/CLAUDE.md`, `spike/crates/core/CLAUDE.md`,
      `spike/crates/hostsim/CLAUDE.md`, `spike/crates/oracle/CLAUDE.md`,
      `spike/crates/plan/CLAUDE.md`, `spike/crates/syntax/CLAUDE.md` — the
      rewritten steering law.
3. Hunt for what is WRONG, not what is ugly: internal contradictions; amendments
   that do not fix the finding they answer; dismissals whose stated reason does not
   hold when you check the cited text; claims that don't hold under the package's
   own rules; decisions that become unfixable once built; problems the package
   never considers; steering-law bullets that diverge from the rulings they cite.
   Follow citations into older `Research/` documents to check whether they actually
   support what is claimed. Where documents disagree, the newest wins; the root
   docs from step 1 outrank everything. If an attack does not hold up, record it as
   withdrawn — do not force it.
4. Write your report to `Research/notes/280e-review-report-deepseek-a.md` and
   commit it. Commit working notes granularly as you go.

WEIGHTING (all findings allowed; spend your effort accordingly):
- HIGHEST value: proving the response's logic broken — an amendment that renames a
  problem instead of fixing it, a dismissal that misreads what it dismisses, a
  ratification resting on an unchecked claim, a replacement mechanism with a hole
  its own trail documents already named. These are the findings that can kill the
  package.
- LOW value: code-quality or test-quality complaints — the code is a throwaway
  prototype; the team will answer 'we'll fix it in the rewrite.'
- LOW value: complaints about unmeasured user demand or market fit — a deliberate,
  recorded decision.
- LOW value: repeating findings the earlier review round already made — they are
  recorded in the adjudication document; only novel failures in the response can
  carry a kill.

REPORT FORMAT: a ranked list of findings, most severe first. Each finding = a short
paragraph + severity (critical/major/minor) + your confidence (high/medium/low) +
exact `file:line` citations. End with a list of attacks you attempted and withdrew,
each with the reason.

DO NOT:
- modify any existing file in the repository — your only writes are your own notes
  and your report;
- touch anything outside this worktree, push, or switch branches;
- spawn subagents or invoke any other AI tool or assistant — do all of the work
  yourself, in this session;
- claim a problem without citing the exact place it occurs;
- continue from memory if your file-read tools or web search are unavailable or
  failing — stop immediately and report the tool failure instead; a crippled review
  is worse than no review.
=== END DISPATCH: deepseek-adversarial ===
