# spike/crates/aid — CLAUDE.md

Role: the DESCRIBE plane (`288` §2a) — narrative records, the diagnostic catalog and its
generated lock, the render seats, the why-lens, and the no-throw `Carrier`. Everything a
user ever READS is minted or rendered here; nothing here may ever license anything. Read
`spike/CLAUDE.md` first (its **User-aid & diagnostics law** block is this crate's law);
this file carries only the aid-local sharpenings. Registry discipline: one rule per
bullet, slugged; append new entries to the matching section.

Companion registries: root `AID-NEEDS.md` (the aid-class registry + laws, cited as
`AID-NEEDS:law-…`) · `plans/282` (loom-pipeline design authority) · `plans/288` (this
crate's charter) · `notes/287` (errorloom as-built).

## Law — the two planes (the reason this crate exists)

- **aid-is-the-describe-plane** — `core` DECIDES, `aid` DESCRIBES. The dependency edge is
  `aid → core` and there is no other; a `core → aid` edge would mean a decision reading a
  narration. If you find yourself wanting one, you are about to violate
  `two-plane-aid-law` — stop and flag UP.
- **two-plane-aid-law** (`26C` §5b, human hard-ack; repeated from root because it is the
  single most important line in this crate) — the license plane fails toward unsureness;
  this plane fails toward narration with attributed confidence. License values flow INTO
  narrative freely, never back. Lint-clean licenses nothing.
- **narrative-is-sealed-by-type-not-place** (`288` §2c) — the seal is private fields, no
  method yielding a license-plane input, and `ProvId` being `!Ord` — NOT co-location with
  `core`. The `compile_fail` doctest in `narrative.rs` pins it against a real license
  consumer (`dorc_core::room::mint_from_room`) ACROSS the crate seam; if that doctest
  stops failing to compile, the seal is gone.
- **collapse-mints-narrative** (`AID-NEEDS:law-collapse-mints-narrative`, née
  law-collapse-mints-evidence) — every safety-narrowing (meet-to-⊤, refuse, decline, wall,
  demote, cancel) mints a decision-inert `CollapseNarrative` carrying the collapse's
  OPERANDS, demanded by the collapse constructor at the VALUE level. The ten mint sites
  live in `analysis` (1), `plan` (4), and `cli` (5) — NOT here; this crate owns the TYPE
  and its constructors, never the mint schedule. The schedule is gate-held from here all the
  same: `tests/narrative_completeness.rs` is a no-wildcard `match CollapseKind` plus a
  lexical census, so a new class cannot land without visiting a mint site.
- **narrative-mints-outrun-renders** (`289:seam-narrative-render-unconsumed`) — every class
  is minted; only `VerdictDecline` carrying an `authored_reason` is RENDERED. A missing
  narrative therefore omits SILENTLY (no `Unexplained` class exists, and `emit_why_lens`
  ignores its narrative slice by signature). Do not build a narrative render to close this —
  it is the deferred arrangement walker's, and surfacing it early welds output
  `27V:rul-output-form-unwelded` keeps free.
- **narrative-eq-excluded-at-the-carrier** — `CollapseNarrative` derives `Eq`, but any
  fixpoint-iterated lattice value carrying one hand-writes `PartialEq` to EXCLUDE it (the
  `analysis::effect::Reach` precedent): a narrative-sensitive lattice `Eq` never
  terminates. Nothing in this crate iterates a fixpoint, so its own `Eq` is free.
- **operands-are-pure-and-capped** — every operand is a `Copy` scalar or an interned
  handle; NO `ProvId`, NO `&mut ProvArena`, NO arena registration inside a
  `CollapseNarrative` (kernels stay pure, `22D` stage-1). Arena receipts are a SEPARATE
  post-pass. Lists are capped at `NARRATIVE_OPERAND_CAP` with the truncation count part
  of the type, never a silently-lossy `Vec::truncate`.

## Law — the catalog and its prose

- **one-catalog-no-legacy** (`27V:rul-kill-legacy-diagnostic`) — the structured `DiagCode`
  catalog is the ONLY diagnostics mechanism. Never add a second string-slug path.
- **defining-case-catalog** (post-`282`-flip) — every code has exactly ONE defining case;
  the committed transcript CASE is the authoring surface and `catalog_lock.rs` is DERIVED
  from it by `dorc-loom compile/promote`. `catalog_lock.rs` is `@generated` — hand-edits
  are refused or caught by the byte-identity fixpoint gate. Never hand-edit it; never add
  a hand-written row.
- **error-authorship-tier** (human-typed 2026-07-18) — builders mint codes and case
  structure with EXPLICITLY-EMPTY prose (`message: None`, rendering `[unwritten: <slug>]`);
  prose is a conductor/human act. Builders author ZERO user-facing strings, ever.
- **prose-three-state** (`27U` §4/§5; `28A` §2p) — a written register is `sm `-prefixed
  migrated builder text, or `[unwritten: <slug>]` (`None`), or unprefixed prose for a
  CASE-OWNED code. `message_registers_are_sm_or_unwritten` enforces it against
  `is_case_owned(slug)`. `[unwritten:]` is a legal resting state.
- **error-slugs-are-semantic** (`288:rul-error-slugs-are-semantic`) — code slugs are
  user-facing surface that becomes a real compat surface at publication. Mint them
  semantic-first, never as a file-naming decision.
- **trust-tier-is-syntax** (`AID-NEEDS:law-trust-tier-is-syntax`) — the epistemic tier of
  every rendered link is a typed `TrustTier` field rendered uniformly by arrangement code;
  prose fragments NEVER hand-write epistemics. The tier SET and its typed rendering are
  the law; the words ride `27V:rul-output-form-unwelded`.
- **render-form-unwelded** (`27V:rul-output-form-unwelded`) — wording, numbering,
  connectives, and arrangement shape are unstable-and-improving. Goldens pin content +
  structure and re-bless freely; never treat a current render as contract.

## Law — engineering substrate

- **aid-is-dst-clean** — pure data + render. No clock, RNG, filesystem, or network,
  directly or transitively — the same bar `core` holds (`inv-determinism`). The one
  filesystem read in the crate is `is_case_owned` inside `#[cfg(test)]`, resolving
  `CARGO_MANIFEST_DIR`; production code touches nothing. A dependency added here must
  prove it carries no nondeterminism.
- **inv-no-throw-here** — `Carrier<T>` lives here and is the no-throw spine: every
  pipeline stage returns it and never panics on malformed input. Errors are data.
  `unwrap`/`expect` never on untrusted-input paths (tests may).
- **inv-referent-agnostic-here** — resolving interned tokens to text in this crate is for
  DISPLAY and provenance only; never branch on resolved text.
- **inv-determinism-here** — deterministic `Ord`/`Hash` for anything used as a map key;
  never iterate a `HashMap`/`HashSet` where order is observable. Render output must be a
  pure function of `(payload, catalog, interner)`.
- **paths-are-manifest-relative** — the test-side `dorc-loom` case lookup resolves
  `CARGO_MANIFEST_DIR.parent().join("dorc-loom/cases")`; `dorc-loom` reaches BACK here for
  `../aid/src/catalog_lock.rs`. Both are depth-coupled to `crates/<c>/` — moving either
  crate breaks both, silently in one direction (a missing baseline SKIPS the ratchet gate).

## Boundaries

- Diagnostic emission SITES belong in the crate that made the decision, never here. This
  crate owns types, catalog data, and render; it never decides.
- Never edit `catalog_lock.rs` by hand (see `defining-case-catalog`).
- Never introduce a second prose home. Every user-facing string ends up loom-editable
  (`288` §1) — a hardcoded string here is debt with a name.
