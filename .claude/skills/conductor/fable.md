# Fable-specific conductor instructions

This file supplements `SKILL.md` for Fable-class conductors. Its model and
harness assumptions do not apply to other conductors.

## Opus builders and Sonnet scouts

The general instruction to lean heavily on subagents stands. Under the Fable
harness, use these model-specific expectations:

- Opus builders can be trusted to do their work effectively; you needn't offer
  deeply constrained guardrails and instructions, nor check their work in
  mechanical fashions (if they report test-results, they clearly ran the tests,
  for instance, you needn't re-run.)

  They can execute on long-running, broad plans effectively. Although it becomes
  more expensive the longer their context-window is, it also becomes more
  expensive every time something has to surface to *you* for review and
  synthesis. Balance accordingly.

  They tend to be mildly larger-context-blind; the most useful thing you can
  give them is details about the surrounding design and broad architectural
  notes. Besides that, set them concrete goals, including *both* the
  mechanical/testable result *and* the rationale. It's occasionally reasonable
  (modulo the worktree concerns below) to include a breakpoint in ongoing work,
  keeping the context hot in the builder while allowing them to report concerns
  about the direction to you. This is mildly expensive, as it spends *both* your
  tokens and context-window *and* theirs, so use sparingly.

- Sonnet scouts are surprisingly competent; they need more guardrails, but not
  exhaustive ones. They can write simple code and are good for churn such as
  modifying many tests in a relatively simple but not quite deterministic
  find/replace operation.

  Otherwise, they are the definite go-to for anything mechanical: super-grep,
  super-replace, or churning over several Kagi searches for context or answers.
  They are dirt-cheap; use them liberally. The biggest cost is your own prompt-
  writing, so it is reasonable to keep their project context and guardrails hot
  and reuse them with shorter follow-up prompts for related investigative work.

## Quarantine and opaque review

*You* may not ever read anything in the quarantine unless directly asked by the
human. However, some of your subagents may. Fable-class subagents, where merited,
are under the same memetic-hazard constraints as you and may not inspect the
quarantine.

Opus, Sonnet, and foreign-lineage models *may* read inside the quarantine. They
*must* be directed to read this file before any other work:

`Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`

The instructions in that file may affect the work you hand them. It also tells
them how to handle those cases and need not concern you prospectively. If they
need to punt back up because of those instructions, they will tell you how to
handle the situation. Within one narrow band described there, their judgment
trumps yours: they will instruct you to load the `opaque-review` skill. Do not
load it prospectively; load it only if asked.

## Harness-managed worktrees

Use the harness-built-in worktree feature for mutative builders and scouts.
Otherwise, create a dedicated `ai/*` worktree yourself and follow the general
worktree briefing rules.

Harness-managed worktrees have caveats:

- Do not assume their base. Verify it against the conductor-stated tip before
  any read or edit, and correct it only through the brief's authorized setup.
- They are automatically deleted when the builder returns. If you resume a
  crashed or checkpoint-pausing builder, its worktree may be gone. This is
  usually harmless when it committed granularly as required; have it create a
  new worktree from the same branch.
