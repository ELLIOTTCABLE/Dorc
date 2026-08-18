# 30Ib — the static-loading build map and deviation ledger

> Builder's working record for the `Research/plans/30I` lane. `30I` is the semantic
> authority; this is the crossing map its work-order §1 asks for, plus the running
> deviation ledger the completion report draws from. Not a design document.

## §1 — The one crossing, mapped

Measured in-tree at `7d04f066`.

**Filesystem reads and source-vector appends** — all in `cli/src/main.rs`:
`resolve_oracle_paths` (the `-o`/`--oracle-dir` list) · the `oracle_srcs` read ·
`read_sourced_oracles` (the transitive oracle-side `.` reader) · `read_books`.
The vectors they produce (`oracle_paths`/`oracle_srcs`) fan out through
`world::source_table` into `source_paths`/`source_srcs`/`source_refs`, which every
downstream seat consumes. Widening them widens all consumers at once.

**`funcenv` load inputs** — `analysis::funcenv::load_sites` reads a `.`/`source`
site's operand through `SourceLiteralPlane::literal_text(node, 1)` and matches the
resulting text against `DefinitionTable::by_path`, whose keys are the CLI-spelled
oracle paths (`world::definition_table`). `command_transfer` then applies that
path's flat `Vec<DefId>`. There is no cwd anywhere in the chain, and no model of a
sourced file's own top level.

**Include-tree and custody** — `cli::sourcing::include_tree` (pure) plus
`core::CustodyClosures::from_edges`, built identically in the binary and in
`world::WhyWorld` (`one-definition-table-two-drivers`).

**Emission seats that carry loaded bytes** — `oracle::closure::HelperIndex` (the
hoisted declaration snapshot) · `plan::build_vouches` · `compile_probe`'s three
ship closures · `Plan::pinned_definitions` → `Plan::render_apply`, whose edit unit
is a `SpanEdit` over the book's own bytes.

**Spine** — `core::spine::SpineLoadDecision` is minted in `main.rs`'s spine
population from contested/never-live/helper-conflict causes; it records a name and
a custody, and today records no load OCCURRENCE at all.

**Provenance** — `core::prov` is the collapse/why arena; `DefinitionId` is the
definition-grade row key. Neither carries a multi-stage locator.

**Plan projection** — `Plan::render_apply` (span edits + the hoisted preamble) is
the only artifact assembler; `main.rs` prints probe-then-apply to stdout and
nothing else writes a file.

## §2 — Deviation ledger (OPEN; for conductor/human adjudication)

Rows are added as they are taken. None is self-endorsed.

## §3 — Owed

Rows are added as they are found.
