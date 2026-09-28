# strawman-3: outcomes past a wall

> STRAWMAN, non-normative. A conductor's exercise of the assay shape (`notes/30Y`) against a made-up cut of Dorc's outcome algebra, written 2026-09-27 to find where Alloy 6 and assay's conventions fight an author before either is built. Nothing in this directory is Dorc's design; every design decision here was invented to reach a compiling model, binds nothing, and is superseded by any ruled document on the same topic. The design-of-record for the region is `plans/239`, `notes/23O` § 2, `KNOBS` named mechanisms, `USER_STORY` stages 2 and 5, and `spike/CLAUDE.md`.

What it models: the per-line outcome of Dorc's plan, elide, guard, run, and survive, for a converged line below a line that may run, under the admin's `--risk-faultless-skips` flag, with footprints and backings as speech and the identity tier's answers as opaque claims. It is deliberately not 311 (that is the next, real translation); it consumes what 311 will one day derive.

Layout:

- `harness/assay.als` — the tool-owned harness, with `Line.above` in place of strawman-2's `before` (an Alloy 6 keyword).
- `spec/shared.md` — the tree-global module this strawman assumes; it differs from strawman-2's in leaving the definition of `Elided` to the document.
- `spec/outcomes-past-a-wall.md` — the specification: species, claims, laws, corpus checks, five load files, nine books.
- `build/outcomes-past-a-wall/` — what a compiler would generate from the two `.md` files, hand-written: one flat directory, module name equals file name, books as `book_<name>.als`, so `open` resolves in one directory without a models path. `report.json` holds EXPECTED results, never observed ones, until the runner replaces them.
- `FINDINGS.md` — every place the shape or Alloy fought the author, classed as it happened; the final division into correctness-chafe and tooling-tune is written at the end.

To run: every `.als` in `build/outcomes-past-a-wall/` is a root module; run each with the Alloy runner and compare against `report.json`. Laws are expected green except the two `attributionByRemoval*` checks, which are expected red on purpose (see the spec's § 5 and `FINDINGS.md`).
