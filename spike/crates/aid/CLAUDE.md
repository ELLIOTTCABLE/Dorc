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
- **defining-case-catalog** (post-`282`-flip) — every code has exactly ONE defining case,
  at `crates/aid/tests/<slug>.loom` (see cases-live-here); the committed transcript CASE is
  the authoring surface and `catalog_lock.rs` is DERIVED from it by
  `dorc-loom compile/promote`. `catalog_lock.rs` is `@generated` — hand-edits
  are refused or caught by the byte-identity fixpoint gate. Never hand-edit it; never add
  a hand-written row.
- **error-authorship-tier** (human-typed 2026-07-18) — builders mint codes and case
  structure with EXPLICITLY-EMPTY prose (`message: None`, rendering `[unwritten: <slug>]`);
  prose is a conductor/human act. Builders author ZERO user-facing strings, ever.
- **prose-three-state** (`27U` §4/§5; `28A` §2p) — a written register is `sm `-prefixed
  migrated builder text, or `[unwritten: <slug>]` (`None`), or unprefixed prose for a
  CASE-OWNED code. `message_registers_are_sm_or_unwritten` enforces it against
  `is_case_owned(slug)`. `[unwritten:]` is a legal resting state.
- **arrangement-registry-is-the-chrome-home** (`289:rul-arrangement-home-is-registry-plus-transcripts`)
  — render-owned CHROME (help/usage pages, structure words, preambles, summary lines) lives in a
  SECOND generated table, `arrangement.rs` + the generated `arrangement_lock.rs`, keyed by
  arrangement-slug + an optional occurrence. Same pipeline as the catalog: mirror-union generation,
  the byte-identity lock gate, the per-case render fixpoint, `dorc-loom promote` publishing both
  locks. A page case declares `arrangement: <slug>` where a code case declares `code:`; the two
  corpora partition the collection.
- **only-registry-bytes-are-editable** — a `RenderPart::Arrangement` span is chrome the renderer
  COMPUTED (immutable structure); only `RenderPart::ArrangementWords`, minted solely by
  `arrangement::push_arrangement_words`, is an edit region. Never stamp a computed string with a
  registry slug: an edit would rewrite an entry the render does not read.
- **artifact-plane-strings-stay-out** (`two-surfaces` / rec-1) — the registry is a RENDER-plane
  home. Anything landing in the byte-floored `.sh` artifact stays hardcoded: every emitter in
  `plan/src/render.rs` (probe/plan/apply headers, the guard-preamble banner, `# replace[..]` /
  `# omit[..]` provenance blocks, the deriv/resolv/reach banners and record scaffolds) writes
  artifact comment bytes, and receipt-stripping byte-identity is a stronger claim than
  editability. Machine formats are out for the same reason at a different altitude: the lint JSONL
  envelope, `--debug-argv`, the records lane. Migrating any of these is a LAW change, not a lane
  item.
- **layout-is-not-a-word** — the registry stores words. Pure layout and punctuation — indents,
  group-header colons, line breaks, the compact finding's `  <line>:<col> <sev> [<src>:<code>] `
  frame — stay `RenderPart::Arrangement`, and are deliberately NOT migrated. `render-form-unwelded`
  already keeps arrangement SHAPE free to churn; putting shape in an editable entry would weld it.
- **arrangement-prose-marker-is-typed** — the catalog's three prose states carry over, but the
  migrated marker is the `Words::Migrated` VARIANT, not an in-band `sm ` prefix: chrome renders
  verbatim into product bytes, so an in-band marker would make a migration a visible product
  change. `authored_words_are_case_owned` is the gate (the `message_registers_are_sm_or_unwritten`
  twin).
- **arrangement-words-are-a-sequence-nothing-splits** (`289:rider-arrangement-home-anticipates-chains`;
  re-cut at the W4 span fold, `28H`) — entries store ORDERED WORDS: a chrome line with interpolated
  values stores its fixed runs and the seat interleaves the computed values
  (`arrangement::sentence_words`, the one arity seat). Nothing ever splits one chrome line across
  SECTIONS — but WITHIN its one section, the LINE-field path re-splits an edit on the compiled
  `Text | Variable` fragment series (`apply_arrangement_line_edit`): the value sequence must equal
  the stamped `v0…v{n-1}`, by name, in order; reorder/drop/duplicate is a narrow refusal. The PAGE
  path (`apply_arrangement_page_edit`) keeps the verbatim no-re-split contract and
  `ArrangementIsSequenceStructured`. Word-boundary INFERENCE stays unbuilt — the fragment series is
  stamped provenance, never guessed.
- **a-chrome-line-is-one-section** (né a-chrome-line-is-one-span; amended at the W4 span fold per
  the `28H` adjudication) — a value-bearing chrome line renders as ONE editable SECTION, which may
  hold interleaved `ArrangementWords` + `ArrangementValue` fragments; it is never fragmented into
  SEPARATE sections fenced by computed runs. The 2026-07-24 lesson stands in that form: the
  transport anchors sections on the immutable text BETWEEN them, and a bare digit is not an anchor
  — many-SECTIONS was the breakage, one-section-many-fragments is the landed fix. Layout — a
  line's trailing newline included — stays computed, so a render never ends inside an editable
  span.
- **a-registry-row-need-not-mint-a-span** — a seat may read registry words as PLAIN TEXT
  (`arrangement_text`/`words_text`) instead of stamping spans, and `dorc_cli::usage_text` does.
  Facelessness is PER-SEAT, not global: the why-reason rows render span-stamped through the weft
  bridge in the why report while the plan-stderr lens reads the same rows as plain text —
  edit-home is wherever a DRIVEN transcript stamps them, the lock otherwise. Do not "fix" a
  plain-text seat by stamping a span its surface's transport cannot anchor.
- **passthrough-is-type-gated** (`282:rul-passthrough-type-gated`; `296` is the census) — a hole is
  foreign because its VALUE is, never because the param is named `detail`. `aid::foreign` seals it:
  `ForeignBytes` is minted only at an I/O edge (`from_os_error`, or the loudly-named and lexically
  fenced `from_io_edge`) and has no raw accessor; `ForeignText` is what comes back through the
  display seat, and it is what `Said::Foreign`, `RenderPart::ForeignText` and `ParamText::Foreign`
  carry. `is_foreign_param` is GONE — do not re-derive a name convention. A sentence you composed at
  an emit site belongs in the code's register with typed holes; if a signature blocks you, that is
  the seal working. `weft::Run::foreign` still takes a `String` (`weft-deps-nothing`), so
  `weave::foreign` is the seat that binds the seal.
- **error-slugs-are-semantic** (`288:rul-error-slugs-are-semantic`) — code slugs are
  user-facing surface that becomes a real compat surface at publication. Mint them
  semantic-first, never as a file-naming decision.
- **trust-tier-is-syntax** (`AID-NEEDS:law-trust-tier-is-syntax`) — the epistemic tier of
  every rendered link is a typed `SpeechAct` field rendered uniformly by arrangement code;
  prose fragments NEVER hand-write epistemics. The tier SET and its typed rendering are
  the law; the words ride `27V:rul-output-form-unwelded`. `SpeechAct` is deliberately
  UNORDERED (`28F:rul-speechact-rename`; née `TrustTier` — "tier" squatted genuinely-ordered
  vocabulary); the one genuine semantic ordering over the same seven kinds is the
  `Knowability` projection, minted at the ONE seat `SpeechAct::knowability`.
- **render-form-unwelded** (`27V:rul-output-form-unwelded`) — wording, numbering,
  connectives, and arrangement shape are unstable-and-improving. Goldens pin content +
  structure and re-bless freely; never treat a current render as contract.
- **said-is-the-parts-vocabulary** (W4; `28H`) — `said.rs` (`Said::{Words, Value, Mark,
  Foreign, Parts}`) is the one parts vocabulary between seats and weave; `chain.rs`
  (`ChainModel`/`LinkSelection`/`Relevance`/`LinkRef`) is the kTASTE type room — links ALL
  retained, selection edges by `LinkRef` never a sorted position, `Relevance` never a bool,
  residue stored as parts. `Said::Foreign` ENCODES AT MINT; **`Said::Mark` carries
  glyphs/punctuation ONLY, never English words** (a worded Mark dodges the
  every-string-is-a-row law).
- **arrangement-key-third-axis-reserved** (`28H` ruling) — the registry key is
  `(slug, occurrence)` and occurrence is SPENT on position/arity discriminators; a future
  kTASTE register axis is a THIRD key axis — a cheap lock reshape under
  `rul-strawman-formats-no-compat`. Never overload occurrence with register.
- **selection-is-goal-derived-here** — default renders curate toward a typed, inspectable,
  DERIVED user-goal (`AID-NEEDS:law-selection-is-goal-derived`): derive the goal first,
  select backwards from it, per-item because-values; curating WRONG is the failure mode,
  density is a tuning (spike-era posture: generous). Binds the first real selection
  machinery; the as-built kTASTE types are its reserved room.

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
- **arrangement-lock-is-generated-too** — `arrangement_lock.rs` is `@generated` by `dorc-loom`
  exactly as `catalog_lock.rs` is; hand-edits are caught by
  `generated_arrangement_lock_reproduces_the_committed_bytes`. Seeding a MIGRATED row by hand is
  the one sanctioned hand-write (the `sm `-row precedent), and the gate proves the seed is a
  generator fixpoint. Seeding `Words::Unwritten` by hand is sanctioned on the same footing and is
  the ordinary way a new chrome slug arrives before anyone has words for it: the generator carries
  the variant straight through from the mirror, `authored_words_are_case_owned` does not bind it,
  and the render shows the greppable placeholder until a case owns the row.
- **hand-seeded-rows-match-the-serializer-order** — a hand-seeded row is only a fixpoint if it is
  spelled in the SERIALIZER's field order, because the gate compares BYTES, not fields: arrangement
  rows are `slug · occurrence · when_used · why · words`, catalog rows are `slug · when_fires · why ·
  params · example · message · help`. A row with the right values in the wrong order fails the
  byte-identity gate with no hint that ORDER is what moved — copy the field sequence from an
  existing row rather than from a struct definition.
- **cases-live-here** (`288:rul-slug-decides-loom-placement`, landed at
  `288:phase-flat-tree-move`) — `crates/aid/tests/<slug>.loom` IS the primary loom
  collection: every canonical case for a registered aid-slug, flat, beside this crate's
  `.rs` tests. That siting is deliberate — it makes THIS file the registry that fires on
  every loom edit (`288:rul-claudemd-fires-per-directory`). Cargo compiles only `tests/*.rs`,
  so the data files are inert here; the runners that drive them are
  `crates/cli/tests/{e2e,looms}.rs`. The first why-faced REPORT cases are the
  `why-drift-*.loom` pair (W4): each declares `arrangement: <its own slug>` — a BORROWED
  page key (the report-case key design is open — `28H`) — and owns exactly its namesake
  row (`is_case_owned` is a filename match; a foreign row edited through a multi-row
  render refuses via `authored_words_are_case_owned`).
- **paths-are-manifest-relative** — the case lookup (`is_case_owned`, in both `catalog.rs`
  and `catalog_defining_cases.rs`) is now manifest-LOCAL — `CARGO_MANIFEST_DIR/tests/
  <slug>.loom` — so it survives a crate move. What still reaches ACROSS is `dorc-loom`:
  `../aid/tests` for the corpus and `../aid/src/catalog_lock.rs` for the lock, both
  depth-coupled to `crates/<c>/`. Moving EITHER crate breaks both, and one direction fails
  silent: an empty corpus read makes the corpus-wide gates pass VACUOUSLY. `fixpoint.rs`'s
  surviving lock gate therefore asserts a NON-EMPTY corpus before it generates — never soften
  that back into a silent empty vec. (Its render-fixpoint half is gone: the ONE render-fixpoint
  authority is `crates/cli/tests/looms.rs`, per committed loom.)

- **authoring-a-replay-block-is-blind** — nothing fills a NEW replay block in place: you append
  `$ <command>` with no output, and the case is then red twice over (the same-slug hygiene gate
  first, since empty output surfaces no slug; the render fixpoint second). The supported loop is
  `DORC_LOOM_DUMP=<dir> mise run test:looms -- <case>`, which writes the CANDIDATE
  transcript — commands re-driven, outputs filled — to `<dir>/<case>.loom` on either failure; copy
  it over the case and re-run. `dorc-loom promote` cannot do this job: adding a command changes
  bytes outside the replay-output islands, which it refuses as a non-prose change.
- **seam-tolerated-nondeterminism-stops-at-the-run-log** — the declared `tolerate:` vocabulary
  (`crates/cli/CLAUDE.md` tolerate-is-a-closed-vocabulary) normalizes the RUN LOG only
  (`expected.ran`, `head-expected.ran`); no normalizer is applied to `expected.out` or to a loom's
  replay-output bytes. So a rendered surface that ever acquires an honest nondeterminism has NO
  declared-class escape hatch and can only be made deterministic at the source. Named, not built:
  extending the vocabulary to rendered output is a design question (what a normalizer may touch in
  bytes a human authors prose into), not a mechanism to add on the way past.

- **spanless-gate-is-lexical** — `spanless_mint_allow_list_is_exact` is a LEXICAL grep
  for `new_spanless_site(DiagCode::X(` at the emit site: every mint spells its payload
  LITERALLY, never through a factored helper (two builders hit this same red, 2026-07-24;
  the gate catches helpers, but write the literal form first and save the round-trip).
  Doc-comments containing the needle shape also trip it — keep examples needle-free.

## Boundaries

- Diagnostic emission SITES belong in the crate that made the decision, never here. This
  crate owns types, catalog data, and render; it never decides.
- Never edit `catalog_lock.rs` by hand (see `defining-case-catalog`).
- Never introduce a second prose home. Every user-facing string ends up loom-editable
  (`288` §1) — a hardcoded string here is debt with a name.
