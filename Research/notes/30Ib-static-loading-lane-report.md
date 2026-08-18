# 30Ib — the static-loading lane report (LIVING)

> Builder's as-built report for the `Research/plans/30I` lane. `30I` is the semantic
> authority and the work order; this file is *as-built state* — where the load model
> lives, what consumes it, what is pinned where, what is OPEN, and what a fresh
> builder does next. Not a re-plan and not a chronology (git carries that).
>
> A successor who has read `30I` and nothing else should resume from this file
> without re-deriving any decision below.

## §1 — The crossing, as found (`30I` work-order §1)

Measured in-tree at `7d04f066`.

- **Filesystem reads / source-vector appends** — all in `cli/src/main.rs`:
  `resolve_oracle_paths` · the `oracle_srcs` read · `read_sourced_oracles` (the
  transitive oracle-side `.` reader) · `read_books`. Their two vectors fan out
  through `world::source_table` into `source_paths`/`source_srcs`/`source_refs`,
  which every downstream seat consumes.
- **`funcenv` load inputs** — `analysis::funcenv::load_sites` reads a `.`/`source`
  operand via `SourceLiteralPlane::literal_text(node, 1)` and matches the text
  against `DefinitionTable::by_path`, whose keys were the CLI-spelled oracle paths.
  No cwd anywhere in the chain; no model of a sourced file's own top level.
- **Include-tree / custody** — `cli::sourcing::include_tree` (pure) plus
  `core::CustodyClosures::from_edges`, built identically in the binary and in
  `world::WhyWorld` (`one-definition-table-two-drivers`).
- **Emission seats carrying loaded bytes** — `oracle::closure::HelperIndex` ·
  `plan::build_vouches` · `compile_probe`'s three ship closures ·
  `Plan::pinned_definitions` → `Plan::render_apply`, whose edit unit is a
  `SpanEdit` over the book's own bytes.
- **Spine** — `core::spine::SpineLoadDecision` records a name plus a custody and
  today records no load OCCURRENCE at all.
- **Provenance** — `core::prov` is the collapse/why arena; `DefinitionId` is the
  definition-grade row key. Neither carries a multi-stage locator.
- **Plan projection** — `Plan::render_apply` is the only artifact assembler;
  `main.rs` prints probe-then-apply to stdout and nothing else writes a file.

## §2 — As-built: the load model's seats

*(Filled in per slice. A seat listed here is the ONE home for its question.)*

- `core::loadpath` — where a `.` operand lands (`30I:rul-dot-resolves-as-sh`).
  `resolve_against_cwd(cwd, target)` is the sh `.` rule (slash-less ⇒ `None`, a
  `PATH` search outside v0); `against_cwd(cwd, path)` is the plain path-operand
  join and is also the CANONICAL KEY every loaded source is filed under.
  Pure text, no filesystem, no environment — so both kernels (`analysis`) and the
  edge (`cli`) share one rule and cannot drift.

## §3 — Scope lines held (read these before widening anything)

- **No un-walling in this lane.** A book's `.` and a book-position `command -v`
  stay unmodeled ⇒ ⊤ ⇒ they wall. `silence-licenses-nothing`. The loader may
  resolve WHICH file a `.` names while the analyzer still walls the emitted
  command; the two answers never contaminate each other. Un-walling is a
  licensure widening routed to the human
  (`FORFEITS:forfeit-command-v-poison-wall` CAPTURE; `30G:b8-book-side-unwalling`).
  The specimens' `expected.ran` assert RUN SETS, and a walled line still runs, so
  nothing in the executable specification needs it.
- **Only MARKED dorc-lang targets enter the load model.** A book `.` of an
  unmarked file is ordinary shell the engine does not model, is never inlined,
  and its bytes are never rewritten (`30I` §7.2; mode 3). This is what keeps
  `floor30-dot-loader-function-errexit` byte-identical: its `child.sh` is
  unmarked, carries a top-level `return` and a failing command, and must stay
  exactly where its author put it.

## §4 — Deviation ledger (OPEN; for conductor/human adjudication)

*(None self-endorsed. Rows added as taken.)*

## §5 — Open questions

*(Rows added as found.)*

## §6 — Next steps for a fresh builder

*(Rewritten each slice.)*

1. Finish the cwd-parity slice: thread the invocation cwd from the CLI edge into
   `source_table`/`definition_table`/`funcenv`, pin the e2e analysis cwd, repair
   the fixtures the rule moves.
