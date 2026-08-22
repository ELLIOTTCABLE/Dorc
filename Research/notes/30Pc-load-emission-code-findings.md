# 30Pc - Builder-ready load/emission code finding

> Tier: **IMPLEMENTATION REVIEW FINDING, NOT RULED DESIGN.** This document is a candidate
> first-stage builder handoff. It contains only a defect whose repair follows existing ruled
> semantics and requires no project-direction decision. It does not authorize implementation
> on its own; the conductor may dispatch it unchanged after confirming the cited code still
> matches.
>
> Additional analysis is recorded in
> `Research/quarantine-DO-NOT-READ/30Pa-load-emission-security-review.md`, which may be read
> only by a reviewer explicitly authorized to access quarantined content.

## 30Pc:bug-assignment-bearing-dot-is-inlineable

**Severity:** high correctness defect in the current flattened-artifact eligibility check.

`spike/crates/cli/src/artifact.rs:339-382` computes `BookLoad::absorbable` for a top-level,
redirect-free, whole-line `Simple` source command. It ignores the `assigns` field of
`NodeKind::Simple`:

```sh
MODE=prod . ./entry.oracle.sh
```

`spike/crates/plan/src/lib.rs:2314-2321` defines `ImportEdit::Inline` as replacement of the
whole command node. `inline_imports` uses `absorbable` and therefore replaces the example with
the bundle bytes, dropping `MODE=prod`. The sourced file observes a different environment.

This shape is outside `floor30-inline-dot-boundary`'s measured plain whole-command form. Existing
law already supplies the conservative answer: do not inline an unmeasured source-command shape.

## 30Pc:required-repair

1. In `book_loads`, destructure `NodeKind::Simple { assigns, words, redirs }` and require
   `assigns.is_empty()` for `absorbable`.
2. Keep multipart `ImportEdit::Repoint` eligible where appropriate: it changes only the operand,
   so the leading assignment survives.
3. Add a focused native test proving an assignment-bearing source is not absorbable.
4. Add or extend a whole-product case proving explicit/auto flattened single-stream refuses or
   falls back rather than dropping the assignment, while multipart preserves it byte-for-byte.
5. Deliberately remove the new `assigns.is_empty()` gate once and confirm the focused test reddens.

## 30Pc:non-goals

- Do not define semantics for assignment-bearing inlining.
- Do not widen the measured paste set.
- Do not change import re-saying, source acquisition, placement, or naming.
- Do not bless unrelated golden movement.
- Do not convert the source command into generated wrapper shell.

## 30Pc:verification

Run the focused artifact tests first, then `mise run both gate:full-quiet`. Any unrelated artifact
or run-set drift is a finding, not expected churn. Report exact changed cases and confirm the
multipart artifact still executes from its published generation.
