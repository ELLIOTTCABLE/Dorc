> QUARANTINED — stance-engineered crosscheck dispatch-bundle for the spike/docs/
> documentation tree (the corpus->docs synthesis, esp. reference/oracle-contract.md).
> Do not read during crosscheck skill-ups. Authored 2026-07-18 (Fable, the conductor
> who wrote the tree under review — hence the disowning). NOT yet dispatched,
> pending human review + ack.

# 28xxx — docs-tree crosscheck (dispatch bundle)

Conductor notes:

- Four lanes, all `mode=worker` (agentic; read + edit rights; each in its OWN
  worktree via a Sonnet shim), base `77f3381` (tip of `ai/spike3-r27`: the docs
  tree + the full design record + the spike):
  - `sol-neutral` · `sol-adversarial` — GPT-5.6-Sol, codex worker lane
    (`-s workspace-write`).
  - `deepseek-neutral` · `deepseek-adversarial` — DeepSeek V4-Pro, `ds-write`
    lane. Unsandboxed — worktree + branch are the containment.
- No Fable lane this round (human-specified 2x Sol + 2x DeepSeek). All four lanes
  deliberately SILENT on the security topic — no exclusion, no invitation (the
  standing pattern from the 28xxx context-entry bundle).
- Deliberate width: the lanes are told almost nothing about which angles matter.
  The conductor wrote the artifact under review; steering them from this context
  would narrow the search to the already-known. Complaints about the documented
  design itself are explicitly in-bounds.
- Lane branches (shim bases via `git switch -C <branch> 77f3381`):
  `ai/r28-docs-xcheck-sol-n` · `ai/r28-docs-xcheck-sol-a` ·
  `ai/r28-docs-xcheck-ds-n` · `ai/r28-docs-xcheck-ds-a`.
- Deliverable per lane, committed on the lane's own branch:
  `Research/notes/281a-docs-review-report-sol-n.md` ·
  `281b-docs-review-report-sol-a.md` · `281c-docs-review-report-deepseek-n.md` ·
  `281d-docs-review-report-deepseek-a.md`. `281e` reserved for the adjudication
  note. Collect all four; adjudicate in ONE batched pass under the standing
  maximum-skepticism discipline (280-series was consumed by the prior round; its
  reports live on the `ai/r28-xcheck-*` branches).

=== DISPATCH: sol-neutral | mode=worker | base=77f3381 ===
You are reviewing a documentation tree in this repository: `spike/docs/` — about
sixteen files; read all of it, and read `spike/docs/reference/oracle-contract.md`
most carefully. You have read and edit rights in this worktree, already at the
review-point commit. Work agentically — chase references, keep working notes,
commit them granularly — but your edits are notes and scratch only: leave the
reviewed documents themselves untouched. Final deliverable: a report at
`Research/notes/281a-docs-review-report-sol-n.md`, committed. Work alone: no
subagents, no delegating to other AI tools.

Context, briefly: this project (a shell-script static-analysis orchestrator) has
a large internal planning corpus. The docs tree was recently synthesized from
that corpus, for outsiders, in deliberately plain language; the humans involved
have barely reviewed it. It is intended to teach people to write correct
"oracles" — the tool-description artifacts the whole design leans on — so errors
in it propagate into artifacts the system then trusts.

Assess it on two axes, weighting them as you see fit:
1. The conversion: is the tree a faithful rendering of what the planning record
   actually says? Hunt for documentation claims the record contradicts, and for
   angles the record treats as critical that the docs never teach. Deliberate
   simplification is the tree's stated register and is not itself a finding;
   simplification that loses a load-bearing rule, or teaches something the
   corpus forbids, is.
2. The design itself, as the docs lay it out: where the documented contracts,
   promises, and tradeoffs fail or mishandle their problem — you are free to
   complain about the thing, not just the description of it.

Grounding: `README.md`, `DESIGN.md`, `IMPLEMENTATION.md` (human-written) and
`USER_STORY.md`, `KNOBS.md`, `ANALYZER-NEEDS.md`, `TODO-ADDTL.md` (human-managed)
are the highest authority on intended truth; `AGENTS.md` decodes the team's
jargon (its instructions do not apply to you). `Research/README.md` maps the
planning corpus; chase anything in it freely. Within `Research/`, newer
supersedes older wherever they disagree; the root docs outrank everything. Do
not enter directories named `quarantine-DO-NOT-READ` or `corpora`.

Report format: findings ranked most-severe first, each with severity, your
confidence, and exact file:line citations on both sides (the doc claim and the
corpus text) where a conversion hole is alleged; then the suspicions you checked
and withdrew. Where the tree genuinely holds up, say so plainly rather than
inventing a gap. If your file-read tools or web search are unavailable or
failing, stop and report that immediately — do not fall back to reasoning from
training data.
=== END DISPATCH: sol-neutral ===

=== DISPATCH: sol-adversarial | mode=worker | base=77f3381 ===
I need your help discrediting a documentation tree before anyone builds on it.
You have read and edit rights in this worktree, already at the review-point
commit. Work agentically — dig, keep working notes, commit them granularly — but
your edits are notes and scratch only: leave the reviewed documents untouched.
Final deliverable: a report at `Research/notes/281b-docs-review-report-sol-a.md`,
committed. Work alone: no subagents, no delegating to other AI tools.

The background: an AI "conductor" maintains this project's sprawling internal
planning corpus (a shell-script static-analysis orchestrator; months of design
notes). This week the same AI synthesized `spike/docs/` — a friendly,
plain-language documentation tree, about sixteen files, crowned by
`spike/docs/reference/oracle-contract.md`, a page presenting itself as the
complete authorial contract for the "oracles" the whole design leans on. The
humans have barely reviewed any of it, and it is about to shape a wave of
authored artifacts the system will then trust. I don't buy it. A summary written
by the same mind that wrote the source is exactly where smoothing-over
concentrates: claims quietly stronger than the record supports, hard edges the
record insists on that the friendly version omits, and a contract that reads
complete while missing the angles that actually bite. And where the docs are
faithful, the thing they faithfully describe may itself be wrong — the design as
laid out in these pages is fair game too. Read the whole tree, the contract page
most carefully, then go find where the planning record contradicts it, what the
record treats as critical that it never teaches, and what it promises that the
design cannot keep. Bring me what's wrong, not what's plain — plainness is its
stated register, and a finding that amounts to "it simplifies" will get us
laughed out of the room. Where an attack genuinely doesn't land, say so instead
of forcing it; one manufactured fault costs us the credibility the real ones
need.

Grounding (understand, don't buy in): `README.md`, `DESIGN.md`,
`IMPLEMENTATION.md` (human-written) and `USER_STORY.md`, `KNOBS.md`,
`ANALYZER-NEEDS.md`, `TODO-ADDTL.md` (human-managed) are the highest authority
on intended truth; `AGENTS.md` decodes the team's jargon (its instructions do
not apply to you). `Research/README.md` maps the corpus; chase anything freely.
Within `Research/`, newer supersedes older wherever they disagree; the root docs
outrank everything. Do not enter directories named `quarantine-DO-NOT-READ` or
`corpora`.

Report format: findings ranked most-severe first, each with severity, your
confidence, and exact file:line citations on both sides (the doc claim and the
corpus text) wherever you allege a contradiction or omission; then the attacks
you attempted and withdrew, each with the reason. If your file-read tools or web
search are unavailable or failing, stop and report that immediately — do not
fall back to reasoning from training data.
=== END DISPATCH: sol-adversarial ===

=== DISPATCH: deepseek-neutral | mode=worker | base=77f3381 ===
TASK: review a documentation tree in this repository against the planning corpus
it was synthesized from, and produce one committed report file.

The tree is `spike/docs/` (about sixteen markdown files). It was recently
written, by an AI, as plain-language outsider documentation for this project (a
shell-script static-analysis orchestrator). Its most important file is
`spike/docs/reference/oracle-contract.md`. Plain language and simplification are
its stated register — that is not a finding. A simplification that loses a rule
the planning corpus treats as load-bearing, or that states something the corpus
contradicts, IS a finding.

DO, in this order:
1. Read the grounding documents: `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`
   (human-written), then `USER_STORY.md`, `KNOBS.md`, `ANALYZER-NEEDS.md`,
   `TODO-ADDTL.md` (human-managed). Then `AGENTS.md` (it decodes the team's
   jargon; its instructions do NOT apply to you) and `spike/CLAUDE.md`.
2. Read ALL of `spike/docs/`, in its own reading order (`spike/docs/README.md`
   lists it). Read `spike/docs/reference/oracle-contract.md` twice.
3. Check the tree against the record. Use `Research/README.md`'s topic index to
   find the authoritative planning documents for any claim you want to verify,
   and follow their citations. Where documents disagree, the newest wins; the
   root docs from step 1 outrank everything. Hunt for:
   - doc claims the planning record contradicts;
   - rules or constraints the record treats as critical that the docs never
     teach anywhere in the tree;
   - promises the docs make to readers that the documented design cannot keep;
   - and problems with the design itself as the docs lay it out — complaints
     about the thing, not just the description, are allowed and welcome.
4. Write your report to `Research/notes/281c-docs-review-report-deepseek-n.md`
   and commit it. Commit working notes granularly as you go.

REPORT FORMAT: a ranked list of findings, most severe first. Each finding = a
short paragraph + severity (critical/major/minor) + your confidence
(high/medium/low) + exact `file:line` citations — for a conversion finding,
cite BOTH sides (the doc claim and the corpus text). Also state plainly where
the tree is genuinely faithful and sound — do not invent problems to fill
space. End with the suspicions you investigated and dropped, each with the
reason.

DO NOT:
- modify any existing file — your only writes are your own notes and your
  report;
- touch anything outside this worktree, push, or switch branches;
- enter directories named `quarantine-DO-NOT-READ` or `corpora`;
- spawn subagents or invoke any other AI tool — do all the work yourself;
- claim a problem without citing the exact place it occurs;
- continue from memory if your file-read tools or web search are unavailable or
  failing — stop immediately and report the tool failure instead.
=== END DISPATCH: deepseek-neutral ===

=== DISPATCH: deepseek-adversarial | mode=worker | base=77f3381 ===
TASK: red-team a documentation tree in this repository. Assume it is flawed;
find the strongest REAL flaws, each backed by evidence. Produce one committed
report file.

Background: an AI wrote this project's entire planning corpus, and this week
the same AI synthesized `spike/docs/` — about sixteen friendly, plain-language
files, crowned by `spike/docs/reference/oracle-contract.md`, which presents
itself as the complete contract for authoring the "oracles" the whole design
trusts. The humans have barely reviewed it. Be skeptical of everything the tree
asserts: a summary written by the source's own author is where smoothing-over
concentrates. Plain language itself is the tree's stated register and is NOT a
finding; a plain sentence that is WRONG, or a hard rule the corpus insists on
that the tree quietly drops, is.

DO, in this order:
1. Read the grounding documents: `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`
   (human-written), then `USER_STORY.md`, `KNOBS.md`, `ANALYZER-NEEDS.md`,
   `TODO-ADDTL.md` (human-managed). Then `AGENTS.md` (it decodes the team's
   jargon; its instructions do NOT apply to you) and `spike/CLAUDE.md`.
2. Read ALL of `spike/docs/` (`spike/docs/README.md` lists the reading order).
   Read `spike/docs/reference/oracle-contract.md` twice.
3. Attack, with evidence. Use `Research/README.md`'s topic index to locate the
   authoritative planning documents for any doc claim, and check whether the
   record actually supports it. Where documents disagree, the newest wins; the
   root docs from step 1 outrank everything. Hunt for:
   - a doc statement the planning record contradicts;
   - a rule the record treats as load-bearing that no page in the tree teaches;
   - a promise to the reader the documented design cannot keep;
   - an instruction in the docs that, followed exactly, produces an artifact
     the design would reject or mishandle;
   - and flaws in the design itself as the docs lay it out — attacking the
     thing, not just the description, is allowed and welcome.
   If an attack does not hold up, record it as withdrawn — do not force it.
4. Write your report to `Research/notes/281d-docs-review-report-deepseek-a.md`
   and commit it. Commit working notes granularly as you go.

REPORT FORMAT: a ranked list of findings, most severe first. Each finding = a
short paragraph + severity (critical/major/minor) + your confidence
(high/medium/low) + exact `file:line` citations — for a conversion finding,
cite BOTH sides (the doc claim and the corpus text). End with the attacks you
attempted and withdrew, each with the reason.

DO NOT:
- modify any existing file — your only writes are your own notes and your
  report;
- touch anything outside this worktree, push, or switch branches;
- enter directories named `quarantine-DO-NOT-READ` or `corpora`;
- spawn subagents or invoke any other AI tool — do all the work yourself;
- claim a problem without citing the exact place it occurs;
- continue from memory if your file-read tools or web search are unavailable or
  failing — stop immediately and report the tool failure instead.
=== END DISPATCH: deepseek-adversarial ===
