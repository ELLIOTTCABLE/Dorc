- README.md, DESIGN.md, TODO.md, and KNOBS.md are human-written;
  - consider re-reading them first if they are not in-context (important context in those is *not* duplicated into this AGENTS.md, intentionally);
  - do not edit them, under any circumstances - suggest edits to the user if you see clear incorrectness; and
  - trust them over the ocean of unreviewed, LLM-generated planning-slop in the Research/ folder.

- try to use reference-slugs in documentation and conversation:
  - source-ID-with-grading (as per the interactive-research skill instructions; [Z-slug-id-1995])
  - when generating 'lists' during conversation (a list of questions, a list of results, a list of nits ...), try and give them vaguely-unique slug-IDs (`nit-1. nit-2. nit-3.`) instead of bare Markdown lists (`1. 2. 3.`), to make it easier to refer-back (and help me see what *you're* referring-back to.)
  - similarly, reuse the named 'knobs' when referring to the shared-axis/"pair-in-tension" design-space components we're working with (see `KNOBS.md`)
