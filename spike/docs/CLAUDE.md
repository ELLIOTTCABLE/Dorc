# spike/docs/ - working agreement for these documents

This tree is human-facing documentation for Dorc: the product, taught from first
principles, to outsiders. It is generated-and-maintained by agents, but its register is
completely different from everything else in this repository. Read this file before
editing or adding anything under spike/docs/.

## What this tree is

Two audiences, two subtrees, very unequal effort:

- `running-books/` - for the admin: the person pointing Dorc at a script they already
  have. Small, low-churn, low-priority.
- `writing-oracles/` - for the engineer: the person describing a tool so Dorc can act
  on it. This is the focus. These documents exist to produce *quality* oracles when
  read by humans, and they drive the stdlib authorship work.
- `reference/oracle-contract.md` - the one authoritative reference page: denser and
  completionist, for someone who has already learned the concepts and is writing
  battle-grade oracles. It enumerates every obligation, license, tradeoff, and
  failure-mode, but stays concrete and task-focused - never abstract reporting.

Everything else in the tree follows a gradual-enhancement arc: each document teaches
only the concepts needed at that point in the reader's growth, building on the previous
ones. `README.md` (in this directory) is the router and reading path.

This tree will eventually gain mechanical siblings (CLI reference, generated API docs).
It must stand alone without them: concepts, not invocations.

## Style law (hard rules, no exceptions)

These documents are the anti-corpus. The Research/ planning-ocean is by-LLMs, for-LLMs,
and dense; these docs are by-LLMs, for-humans, and simple. The habits that serve the
corpus are failure-modes here:

1. No jargon, no reference-slugs, no insider terminology in prose. Every Dorc concept
   is unfolded from first principles before it is used. If a sentence would only make
   sense to someone who has read the corpus, rewrite it. Dorc's own product vocabulary
   (book, oracle, probe, plan, elide, guard, kind, selector, wall) is fine - but each
   term is taught, in order, before it is leaned on.
2. Unroll. Multiple paragraphs beat one dense one. If a concept is hard, build it up
   slowly with a concrete sh example; do not compress.
3. Markdown discipline, strictly budgeted:
   - at most one `*emphasis*` per two paragraphs; zero `**strong**`, anywhere;
   - lists only for genuinely list-shaped content (menus, checklists, tables of
     outcomes); prose otherwise. Register split (human-calibrated, 2026-07-18):
     this binds the learning path and admin docs; `reference/` material relaxes
     it - heavy bullets and dense per-item detail are acceptable, even
     preferred, for contract material;
   - code-spans and fenced sh blocks are unrestricted - prefer showing actual sh over
     describing it;
   - readable sentence-case headers; no decorative markup;
   - every character ASCII. No em-dashes, no curly quotes, no arrows, no unicode
     symbols of any kind. Spell out "less than or equal", or write `>=` in a
     code-span.
4. Concept-focused, not CLI-focused. Command names, flag spellings, and plan render
   formats churn; do not anchor prose to them. Where an invocation is needed for
   grounding, mark it illustrative. Verdict behavior (what elides on a given day) is
   explicitly unstable-and-improving; never present a specific plan output as a
   stable promise.
5. Opinionated best practices are wanted. These docs should make someone a better
   shell author, not just a Dorc user. For general (non-Dorc) shell practice, terse
   references to the standard sources are fine; where a practice interacts with
   Dorc's contracts, unroll the rationale fully in place.
6. sh examples use 3-space indentation, quote-as-law, printf-doctrine, and the
   dialect these docs themselves teach. An example that violates the contract it is
   teaching is a bug - as is one using spellings unverified against the grammar
   (when unsure a decoration parses, annotate outside the code).
7. Teach the when, not only the what. Every construct's documentation answers
   "when would I reach for this?" (and when not) alongside its syntax and
   semantics - in the reference especially, where a member with no when-guidance
   is an incompleteness bug (human-calibrated, 2026-07-18).

## The one sanctioned exception: quoted footers

Prose carries zero corpus references. Instead, a major section may end with a single
HTML comment footer citing the corpus slugs and documents it was synthesized from, for
machine grepping when the design moves:

    <!-- quoted: spike/CLAUDE.md rul-rc-partition; 271:rul-zero-one-inversion-pair -->

These footers are maintenance metadata, invisible to rendered output. Keep them terse;
one line where possible. They are the mechanism by which a later agent finds every doc
section affected by a design change: grep the slug, re-synthesize the section.

## Correctness sources and precedence

On any conflict, the corpus outranks these docs, and these docs get re-synthesized
(never locally patched into disagreement). Authority order:

1. Root docs: `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `USER_STORY.md`,
   `KNOBS.md` (human-audited).
2. `spike/CLAUDE.md` (the invariant registry; densest correct summary).
3. The dialect and contract notes: `Research/notes/278` (dorc-lang v0.1 reference),
   `Research/notes/277` (entity algebra and mark grammar), `Research/notes/276`
   (dialect rulings), `Research/plans/27C` (wrapper/context spec),
   `Research/notes/23O` (settled oracle-contract law), `Research/notes/27Q`
   (stdlib authoring preconditions).
4. The e2e fixtures under `spike/e2e/cases/` for as-built concrete spellings.

Where the design is still provisional (some role-member names, some mark spellings,
all CLI surface), the docs teach the current spelling and say so lightly, without
turning the prose into a hedge-forest. One sentence of "this spelling may still
shift" at first use is enough.

## The reader-path rubric

The engineer docs are sequenced to match how a real author's effort evolves, and each
document must leave the reader with something that works:

1. Understand what Dorc can and cannot see (mental model).
2. Write a two-minute oracle that makes their own tool's line disappear when
   converged (first success; steepest part of the value curve).
3. Internalize the probe bargain (never mutate; fail toward run) before widening.
4. Name state so their facts compose with strangers' facts.
5. Cover a real tool honestly (decline-by-default breadth).
6. Model behavior per-channel (predictions).
7. Make claims about what their tool disturbs (footprints; the priced tier).
8. Describe wrappers and contexts.
9. Steward a vocabulary (kind ownership).
10. Write everything in the portable dialect, defensively.
11. Bring the engine into the authoring loop: class declines, lint as you write,
    read back what an admin will see.
12. Publish, own, and maintain what they shipped.

The reference page then serves the reader who has completed the arc. When editing,
check changes against this path: a concept must not be used before the point where the
path teaches it.

Community quality is the underlying goal - oracles get published and other people's
machines act on them - but the prose stays focused on the reader's own immediate task;
it never lectures them about their duties to an ecosystem they have not joined yet.
The costs land where they are priced: in the contract page and in the specific
features that spend trust.
