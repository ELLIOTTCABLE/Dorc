# 290 — the aid-crate extraction MAP (mechanical spec for `288:phase-aid-crate-extraction`)

MAP-tier under `27U:map-then-execute-split`. This document is the EXECUTION SPEC: a fresh
executor runs it with ZERO re-derivation. Every claim below was re-verified in-tree at base
`fbbf88f1dad6ea7c7330d256125c30eed043da74` (lane branch `ai/r28-unify-p1`); `288` §2b's LOC
counts were taken at `ce460f6b`, and `git diff --stat ce460f6b..fbbf88f1` touches only
`Research/` + `spike/CLAUDE.md` — so §2b's inventory is byte-current at this base (+SURE).

Charter: `plans/288` §2 (architecture) · §2b (type inventory) · §2d (seams) · §2e (scout
census) · §8 phase 1 · §9 steering-sync. Authority order: root docs > `spike/CLAUDE.md` >
`288` > this file. Where this file and `288` §2b disagree, this file was verified later and
names the correction inline.

FLAGS for the conductor are collected in §9 and are NOT resolved here.

---

## §1 — What the executor produces

One atomic cutover commit (§8), preceded by ONE genuinely-additive, individually-green
prelude commit (the `SiteId` relocation, §6). Expected end-state: workspace builds, all four
gates green, `sh e2e/run.sh` 97/97, goldens and `.loom` cases BYTE-IDENTICAL (verified: no
golden or case file anywhere references a crate path — `rg -l 'dorc_core|dorc-core|crates/core'
crates/dorc-loom/cases/ e2e/` is empty, +SURE).

New tree:

```
spike/crates/aid/
  Cargo.toml
  CLAUDE.md            (§7 draft)
  src/lib.rs           (NEW: crate doc + Severity + Carrier + module wiring + 2 moved tests)
  src/diag.rs          (git mv from core)
  src/catalog.rs       (git mv from core)
  src/catalog_lock.rs  (git mv from core)
  src/tagged.rs        (git mv from core)
  src/narrative.rs     (git mv from core/src/evidence.rs + rename)
  tests/diag_tidy.rs                (git mv from core/tests)
  tests/catalog_defining_cases.rs   (git mv from core/tests)
  tests/span_precision_survey.rs    (git mv from core/tests)
```

Crate name `dorc-aid`, lib path `dorc_aid`, directory `crates/aid` — mirrors the
`crates/core` → `dorc-core` convention exactly. No `[[bin]]`.

---

## §2 — Move list: every item that MOVES, every item that STAYS

### §2a — Whole-file moves (core/src → aid/src)

| file | LOC @ base | verified | note |
|---|---|---|---|
| `diag.rs` | 3026 | ✓ matches `288` §2b | DiagCode/payloads/`SiteId`/`GroupingKey`/render seats/why-lens |
| `catalog.rs` | 597 | ✓ | `CatalogEntry`, `CONST_CATALOG`, template machinery, `#[path]`-includes the lock |
| `catalog_lock.rs` | 530 | ✓ | generated; a PRIVATE `#[path = "catalog_lock.rs"] mod catalog_lock;` inside `catalog.rs`, re-exporting only `CATALOG`. Both `include_str!` calls in `catalog.rs` (`:448`, `:456`) are file-relative and survive the move untouched. |
| `tagged.rs` | 114 | ✓ | `Field`/`RenderPart`/`RenderParts`. Zero `crate::` code deps (only two intra-doc links into `crate::catalog`, which stay valid). |
| `evidence.rs` → `narrative.rs` | 634 | ✓ | see §3 for the rename map |

Total moved from `core/src`: 4901 LOC + ~70 peeled from `lib.rs` ≈ 4971 (`288` §2b's "~5,000"
holds).

### §2b — Whole-file moves (core/tests → aid/tests)

All three of `core`'s integration tests are diagnostics tests and move wholesale:

| file | LOC | why it moves |
|---|---|---|
| `diag_tidy.rs` | 709 | the `DiagCode`/catalog tidy + retire-guard suite; scans crate sources for emit sites |
| `catalog_defining_cases.rs` | 574 | per-code defining-case coverage + the shrink-only ratchet |
| `span_precision_survey.rs` | 80 | `//!`-only doc fixture: the per-`DiagCode` caret-span survey backing `diag_tidy`'s `SPANLESS_SITE_PAYLOADS` allow-list |

`core/tests/` ends up EMPTY — delete the directory. (Core's remaining tests are all inline
`#[cfg(test)]` modules.)

### §2c — Peeled out of `core/src/lib.rs`

MOVES to `aid/src/lib.rs`, verbatim:

- `pub enum Severity { Error, Warning, Note }` + its doc-comment (`lib.rs:109–117`).
- `pub struct Carrier<T>` + `impl<T> Carrier<T>` (`pure`/`new`/`map`/`and_then`/`push`/
  `has_errors`/`into_parts`) + the doc-comment (`lib.rs:156–215`).
- The two Carrier tests from `core/src/lib.rs`'s `mod tests`:
  `carrier_threads_diagnostics_through_stages` (`:869`), `carrier_reports_errors_without_panicking`
  (`:882`). These are `288` §2e's "+2 test uses"; they are the ONLY items in core's inline test
  module touching the aid plane (+SURE — the other five tests are ValueGrade/Interner×2/auto-cell/
  context-keying and stay).
- The module wiring lines `pub mod diag; pub use diag::Diag;` (`:119–120`) · `pub mod catalog;`
  (`:122`) · `pub mod tagged;` (`:124`) · `pub mod evidence; pub use evidence::{CollapseEvidence,
  CollapseKind, TrustTier};` (`:153–154`) — deleted from core, re-minted in aid (with `evidence`
  → `narrative` and the type rename).

STAYS in `core/src/lib.rs` (everything else — re-verified item by item):
`AstId` · `LeafId` · `OracleFileId` · `BytePos` · `Span` (+`impl`) · `Symbol` (+`impl`) ·
`Interner` (+`impl`) · `OpaqueToken` · `KindId` · `ProviderId` · `Phase` · `Verdict` · `Rc` ·
`OutBytes` · `TopCause` (+`impl`) · `ValueGrade` (+`impl`) · `Predicted<T>` · `Channel` ·
`Observable` (+`impl`) · `Grade` · `SelectorId` · `EntityRef` · `FactKey` (+`impl`) ·
`FactBacking` · `AUTO_KIND_PREFIX` · `AUTO_SELECTOR` · `auto_fact()` · `is_auto_kind()` ·
`use std::collections::HashMap` · the `pub mod`/`pub use` lines for `prov` · `unord` · `claim` ·
`coord` · `room` · `escalation`.

> **CORRECTION to `288` §2b** (minor, non-load-bearing): §2b lists `TopCause` as living in
> `prov.rs`. It is defined in `lib.rs:356–403` (with `impl TopCause::describe`); `prov.rs` owns
> `ProvId`/`ProvArena`/`OriginKind`/`OriginNode`/`Parents`/`ProbeStamp`/`Variation`/`Witness`/
> `JOIN_PARENT_CAP`. Disposition is unchanged either way: both stay in `core`.

### §2d — Whole files that STAY in `core/src` (unchanged except §2f edits)

`claim.rs` (342) · `coord.rs` (861) · `room.rs` (243) · `unord.rs` (150) · `escalation.rs` (63)
· `prov.rs` (661). Verified: NONE of these six references `diag`, `catalog`, `tagged`,
`evidence`, `Carrier`, `Severity`, `Diag`, `CollapseEvidence`, `CollapseKind`, or `TrustTier` —
a grep over all six returns zero hits (+SURE; this is `288` §2e's "core's own aid-emission
census: EMPTY", re-verified in-tree). Zero cycles: `aid → core` only.

### §2e — Edits to files that STAY in `core`

Only `core/src/lib.rs` is touched, in four places beyond the §2c deletions:

1. `:14–16` — the crate-doc bullet "**No-throw stages (`dn-7`).** Every pipeline stage yields
   a [`Carrier<T>`] — a *result paired with accumulated diagnostics*…". The intra-doc link
   `[`Carrier<T>`]` becomes unresolvable (`rustdoc::broken_intra_doc_links = warn`, workspace
   lints). MOVE this bullet's substance to `aid/src/lib.rs`'s crate doc; in core, replace with
   plain prose naming `dorc-aid` without a link.
2. `:53–57` — `LeafId`'s doc says "the round-22 structured diagnostic ([`diag::SiteId`]) keys on
   it". After the §6 prelude, `SiteId` lives in `core/src/lib.rs`, so this becomes the local link
   `[`SiteId`]`. (If §6 is REJECTED, this must instead be de-linked to backticked plain text —
   `core` cannot link into `aid`.)
3. `:109–117` — `Severity` and its doc leave (§2c).
4. `:27–31` — the crate-root `#![expect(missing_docs, clippy::indexing_slicing, reason = …)]`
   STAYS AS-IS. Verified both lints still fire in the staying surface: `missing_docs` on
   `Span::{lo,hi}`, `Grade::{Must,May}`, `FactKey::{kind,entity,selector}`, `Interner::intern`,
   and every `pub mod` line; `clippy::indexing_slicing` on `lib.rs:259`
   (`&self.strings[sym.0 as usize]` in `Interner::resolve`). An unfulfilled `expect` WARNS, so
   this verification is load-bearing — do not "tidy" the list.

---

## §3 — Rename map (`288:rul-narrative-layer-naming`)

The rename rides the extraction commit. No aliases, no deprecation shims, no mapping layer —
`spike/CLAUDE.md:rul-strawman-formats-no-compat` and the standing
strawman-formats-never-compat-targets order.

### §3a — Decisions taken, one line of rationale each

| # | decision | rationale |
|---|---|---|
| dec-module-named-narrative | `core/src/evidence.rs` → `aid/src/narrative.rs` | Mandated verbatim by `288:rul-narrative-layer-naming`. |
| dec-collapse-narrative-type | `CollapseEvidence` → `CollapseNarrative` (47 occurrences, 5 files) | Mandated verbatim. |
| dec-collapse-kind-kept | `CollapseKind` KEPT unrenamed (56 occurrences) | The retired register is "evidence", not "collapse"; the type names the KIND OF COLLAPSE, which is still exactly true, and `288`'s ruling scopes to "the decision-inert record plane". Renaming buys nothing and widens the diff by 56 sites. |
| dec-trust-tier-kept | `TrustTier` KEPT unrenamed (53 occurrences) | Same reasoning, plus `AID-NEEDS:law-trust-tier-is-syntax` and `spike/CLAUDE.md:trust-tier-is-syntax` name it verbatim; renaming it would force a law-slug edit this phase is not scoped for. |
| dec-operand-cap-renamed | `EVIDENCE_OPERAND_CAP` → `NARRATIVE_OPERAND_CAP` (9 occurrences, ALL inside the moving file) | Carries the retired word in a `pub` name; zero cross-crate cost (grep-verified: no consumer references it). |
| dec-accessor-follows-type | `plan::SurvivalReport::collapse_evidence()` → `collapse_narrative()`, its private field likewise | The only PUBLIC identifier outside `aid` carrying the retired word. Leaving it keeps "evidence" alive at the most-read call site, which defeats the ruling's purpose. Contained: one `pub fn`, one private field, three call sites. |
| dec-locals-follow-type | local bindings / private fields / private fns spelled `*_evidence` in the collapse-mint lane rename to `*_narrative` | Consistency; all are private or local, so cost is confined to the explicit file list in §4d. |
| dec-hostevidence-untouched | the `HostEvidence*` family is NOT renamed | Different concept: host-supplied evidence ADMISSION (a license-adjacent measurement gate in `hostsim`/`plan::records`/`cli`, and the `HostEvidenceAdmissionRefused` DiagCode). ~141 occurrences. Renaming would be a semantic error. |
| dec-errorloom-untouched | `errorloom::EditRefusalEvidence`, `dorc-loom`'s `bounded_evidence`/`MAX_REFUSAL_EVIDENCE` NOT renamed | Third concept (transcript edit-refusal evidence bytes), owned by the generic loom layer; out of this crate's plane entirely. |
| dec-law-slugs-frozen | `AID-NEEDS:law-collapse-mints-evidence` and `spike/CLAUDE.md`'s `collapse-mints-evidence` bullet SLUGS stay as written; only their BODY prose gains the new type name | The brief's standing rule; re-slugging a registry law is a conductor act, not a mechanical rider. FLAGGED in §9. |

### §3b — Public surface of `narrative.rs` after the move

Unchanged names: `TrustTier` (+`from_vouch`) · `Operands<T>` · `ValueOperand` · `MintSpan` ·
`DeclineGate` · `DeclineClass` (+`token`/`from_token`) · `AuthoredReason` · `ChannelCoverage` ·
`EntryDegradeTag` · `EntryFailureTag` · `DemoteTag` · `RenderRefusalTag` · `Reserved` ·
`CollapseKind` (+ its constructors).
Renamed: `CollapseEvidence` → `CollapseNarrative` · `EVIDENCE_OPERAND_CAP` →
`NARRATIVE_OPERAND_CAP`.

### §3c — Internal imports inside the moved `narrative.rs`

| line | before | after |
|---|---|---|
| `:51` | `use crate::diag::SiteId;` | `use dorc_core::SiteId;` (after §6; else `use crate::diag::SiteId;` unchanged — diag moves too) |
| `:52` | `use crate::{Channel, JOIN_PARENT_CAP, LeafId, OracleFileId, OutBytes, Span};` | `use dorc_core::{Channel, JOIN_PARENT_CAP, LeafId, OracleFileId, OutBytes, Span};` |
| `:82` | `_vouch: &crate::ByVouch<P>` | `_vouch: &dorc_core::ByVouch<P>` |
| `:93` | doc-link `[`crate::Parents`]` | `[`dorc_core::Parents`]` |
| `:38` | doc-link `[`ProvId`](crate::ProvId)` | `[`ProvId`](dorc_core::ProvId)` |
| `:17`,`:19` | doc-links `[`core::room`](crate::room)`, `[`crate::room::mint_from_room`]` | `[`dorc_core::room`]`, `[`dorc_core::room::mint_from_room`]` |
| `:442` | (test mod) `use crate::{ByVouch, BytePos, LeafId, Rung};` | `use dorc_core::{ByVouch, BytePos, LeafId, Rung};` |
| `:23–33` | the `compile_fail` doctest body | `use dorc_aid::narrative::{CollapseNarrative, CollapseKind, TrustTier};` · `use dorc_core::room::mint_from_room;` · `CollapseNarrative::new(…)` · `# fn dummy() -> dorc_aid::diag::SiteId { … }` (or `dorc_core::SiteId` after §6) |

The seal survives: enforcement is type-level (private fields, no method yields a license-plane
input, `ProvId` is `!Ord`), not co-location — `288` §2c, confirmed by reading `room.rs`
(`mint_from_room` is `pub fn`, `aid` deps `core`, doctests see the dep graph). +SURE.

### §3d — Internal imports inside the other moved files

| file | line | before | after |
|---|---|---|---|
| `diag.rs` | `:30` | `use crate::{LeafId, ProvId, Severity, Span, TopCause};` | `use crate::Severity;` + `use dorc_core::{LeafId, ProvId, Span, TopCause};` (drop `LeafId` if §6 lands and `SiteId` imports it from core) |
| `diag.rs` | `:1662,1778,1790,1810,1829,1875,1922,2001,2012,2056,2063` | `&crate::Interner` | `&dorc_core::Interner` (11 signature sites) |
| `diag.rs` | `:2203` | `arena: &crate::ProvArena` | `&dorc_core::ProvArena` |
| `diag.rs` | `:2932,2934,2974,3007,3008` | `crate::ProvArena::new()`, `crate::OriginKind::TopCause` | `dorc_core::…` (test mod) |
| `diag.rs` | `:2477` | (test mod) `use crate::{BytePos, Interner};` | `use dorc_core::{BytePos, Interner};` |
| `diag.rs` | `:243,1059,2174` | doc-links `[`crate::DiagCode`]`, `[`crate::Diagnostic::span`]`, `[`crate::Exempt::Explanation`]` | PRE-EXISTING DEAD LINKS — `Diagnostic` and `Exempt` no longer exist anywhere in `core` (grep-verified) and `DiagCode` is not re-exported at core's root. De-link to backticked plain text as part of the move; see FLAG `flag-preexisting-dead-doclinks` (§9). |
| `diag.rs` | `:1962` | `use crate::tagged::RenderPart;` | UNCHANGED (tagged moves too) |
| `catalog.rs` | `:199` | `use crate::tagged::{RenderPart, RenderParts};` | UNCHANGED |
| `catalog.rs` | `:540` | `crate::diag::SiteId::leaf(crate::LeafId(0))` | `crate::diag::SiteId::leaf(dorc_core::LeafId(0))` |
| `catalog.rs` | `:503–507` | `is_case_owned` resolves `CARGO_MANIFEST_DIR.parent().join("dorc-loom/cases")` | UNCHANGED — `crates/aid` and `crates/core` are the same depth, so the relative resolution is identical. Verified. |
| `catalog_lock.rs` | `:4` | `use super::CatalogEntry;` | UNCHANGED |
| `tagged.rs` | — | (only intra-doc links into `crate::catalog`) | UNCHANGED |

### §3e — Edits inside the three moved test files

| file | line | edit |
|---|---|---|
| `diag_tidy.rs` | `:241` | `SCANNED_CRATES`: replace `"core"` with `"aid"` in the list (the retire-guard scans the crate that DEFINES the codes) |
| `diag_tidy.rs` | `:244,248` | doc/expect text "this test runs with cwd = `crates/core`" / "crates/core has a parent" → `crates/aid` (`crates_dir()`'s `CARGO_MANIFEST_DIR.parent()` logic is depth-invariant and needs no code change) |
| `diag_tidy.rs` | `:292,309,311,347` | "every scanned crate EXCEPT `core`" and `.filter(\|c\| *c != "core")` → `"aid"` (the production-emit-surface exclusion must follow the crate that holds the constructor + its `#[cfg(test)]` uses) |
| `diag_tidy.rs` | `:384,385,413` | `crates_dir().join("core/src/diag.rs")` → `"aid/src/diag.rs"`; `let diag_rel = "crates/core/src/diag.rs";` → `"crates/aid/src/diag.rs"` |
| `diag_tidy.rs` | `:551,687` | prose "Add its entry to core/src/catalog.rs" / "exists ONLY in core/src/diag.rs" → `aid/src/…` |
| `diag_tidy.rs` | `:4,47,201,202` | doc-links `[`dorc_core::diag::…`]` → `[`dorc_aid::diag::…`]` |
| `catalog_defining_cases.rs` | `:25–34` | `use dorc_core::diag::{…}` → `dorc_aid::diag`; `use dorc_core::tagged::RenderPart;` → `dorc_aid::tagged`; `use dorc_core::{BytePos, Interner, LeafId, Span, TopCause};` UNCHANGED |
| `catalog_defining_cases.rs` | `:419,472,473,561` | `dorc_core::catalog::…` → `dorc_aid::catalog::…` |
| `catalog_defining_cases.rs` | `:456–460` | `is_case_owned` — UNCHANGED (depth-invariant, as above) |
| `catalog_defining_cases.rs` | `:505` | `let rel = "crates/core/tests/catalog_defining_cases.rs";` → `"crates/aid/tests/catalog_defining_cases.rs"` — **load-bearing**: the shrink-only ratchet reads its committed baseline through `git show HEAD:spike/{rel}`. A missed edit makes `ratchet_only_shrinks` silently SKIP (it prints "no committed baseline — skipping" and returns). Note that on the cutover commit itself the OLD path is what HEAD holds, so the test will skip once and re-arm on the next commit; that is expected and matches the file's own "new file / no git" contract. |
| `span_precision_survey.rs` | — | no code; sweep the `//!` prose for `core/src/diag.rs` / `core/tests/` mentions and re-point |

---

## §4 — Import-rewrite inventory (grep-verified)

### §4a — Per-crate site counts

Regenerate at any time with (from `spike/crates`, msys/Git-Bash):

```sh
rg -cU --multiline \
  "dorc_core::(diag|catalog|tagged|evidence|Carrier|Severity|Diag\b|CollapseEvidence|CollapseKind|TrustTier)|dorc_core::\{[^}]*\b(Carrier|Severity|Diag|CollapseEvidence|CollapseKind|TrustTier)\b" \
  -g '*.rs' .
```

At base (`fbbf88f1`):

| crate | files | match-sites | gains `dorc-aid` dep |
|---|---|---|---|
| `cli` | 1 | 44 | YES |
| `oracle` | 17 | 30 | YES |
| `lint` | 10 | 28 | YES |
| `analysis` | 3 | 23 | YES |
| `dorc-loom` | 8 | 22 | YES |
| `plan` | 5 | 17 | YES |
| `coverage` | 1 | 5 | YES |
| `syntax` | 2 | 3 | YES |
| `hostsim` | 0 | 0 | **no** |
| `sweep` | 0 | 0 | **no** |
| `errorloom` | 0 | 0 | **no** (deps neither crate) |
| `core` (files that MOVE) | 3 | 14 | n/a |

> `288` §2d calls `oracle` "heaviest at ~56 Carrier sites". CORRECTION: `oracle` has 50 textual
> `Carrier` occurrences, but **15 of them in `oracle/src/predict/mark_grammar.rs` are a LOCAL
> `enum Carrier`** (the mark carrier, `#:` vs `:` — `mark_grammar.rs:89`) with no relation to
> `dorc_core::Carrier`. The real per-file EDIT count is far lower than the occurrence count
> because a bare `Carrier` usage needs no touch once its `use` line points at `dorc_aid`. See
> `flag-mark-grammar-carrier-collision` (§9) — this is the single biggest naive-sed hazard in
> the lane. `cli/src/main.rs` (44 sites, one file) is the real heaviest.

Definitive file list (49 consumer files + the 3 core files that move); regenerate with the same
rg invocation and `-l`:

```
analysis/src/cfg.rs                  analysis/src/effect.rs               analysis/tests/cfg.rs
cli/src/main.rs
coverage/src/lib.rs
dorc-loom/src/compile.rs             dorc-loom/src/consumer.rs            dorc-loom/src/edit.rs
dorc-loom/src/generate.rs            dorc-loom/src/lib.rs                 dorc-loom/tests/consumer.rs
dorc-loom/tests/coverage.rs          dorc-loom/tests/editable_render.rs
lint/src/finding.rs                  lint/src/production.rs               lint/src/render.rs
lint/src/source_analysis.rs          lint/src/source_external.rs          lint/src/source_oracle_solo.rs
lint/src/source_unmodeled.rs         lint/src/source_verdict.rs           lint/tests/adapters.rs
lint/tests/report.rs
oracle/src/carry.rs                  oracle/src/entry.rs                  oracle/src/lib.rs
oracle/src/marker.rs                 oracle/src/predict.rs                oracle/src/predict/derive.rs
oracle/src/predict/mark_grammar.rs   oracle/src/predict/parser.rs         oracle/src/reaches.rs
oracle/src/report.rs                 oracle/src/reserved.rs               oracle/src/resolve.rs
oracle/src/strip.rs                  oracle/src/touches.rs                oracle/src/validate.rs
oracle/src/verdict.rs                oracle/src/wrapper.rs
plan/src/erasability.rs              plan/src/invocation.rs               plan/src/lib.rs
plan/src/records.rs                  plan/src/whylog.rs
syntax/src/lib.rs                    syntax/src/parser.rs
```

Plus three files that carry only HARDCODED PATHS (no symbol imports), §4e.

### §4b — Step 1: the safe global path rewrite

Every `dorc_core::<moving-path>` occurrence — in code, in doc-links, in comments — is
unambiguous and can be rewritten mechanically. Run from `spike/crates`, over the explicit file
list above written to `/tmp/aid-files.txt` (msys `sed -i` is fine; avoid `find -exec` and avoid
`xargs` without `-d`):

```sh
# from spike/crates ; FILES is the §4a list, one path per line
while IFS= read -r f; do
  sed -i \
    -e 's/dorc_core::diag/dorc_aid::diag/g' \
    -e 's/dorc_core::catalog/dorc_aid::catalog/g' \
    -e 's/dorc_core::tagged/dorc_aid::tagged/g' \
    -e 's/dorc_core::evidence/dorc_aid::narrative/g' \
    -e 's/dorc_core::Carrier/dorc_aid::Carrier/g' \
    -e 's/dorc_core::Severity/dorc_aid::Severity/g' \
    -e 's/dorc_core::CollapseEvidence/dorc_aid::CollapseNarrative/g' \
    -e 's/dorc_core::CollapseKind/dorc_aid::CollapseKind/g' \
    -e 's/dorc_core::TrustTier/dorc_aid::TrustTier/g' \
    -e 's/dorc_core::Diag\b/dorc_aid::Diag/g' \
    "$f"
done < FILES
```

Ordering note: `dorc_core::Diag\b` MUST come after nothing in particular (no other rule
produces that string), but it must use `\b` so it never eats `dorc_core::DiagCode` — verified
that `DiagCode` is NOT re-exported at core's root (only `pub use diag::Diag;`), so any
`dorc_core::DiagCode` in the tree is already a dead doc-link and is handled by the
`dorc_core::diag` rule or §3d. Sanity check afterwards:
`rg -n 'dorc_core::(diag|catalog|tagged|evidence|Carrier|Severity|Diag\b|Collapse|TrustTier)' -g '*.rs' .`
must return zero hits.

### §4c — Step 2: the braced `use` splits (hand-edit; exhaustive list)

These are the ONLY sites the global rewrite cannot reach — a `use dorc_core::{…}` list mixing
staying and moving symbols. Each needs the moving names lifted into a second `use dorc_aid::{…}`
statement, keeping rustfmt's alphabetical order within each list. Exhaustive, verified:

| file | line(s) | current list | split into |
|---|---|---|---|
| `syntax/src/parser.rs` | 24 | `{BytePos, Carrier, Span}` | `dorc_aid::Carrier` + `dorc_core::{BytePos, Span}` |
| `syntax/src/lib.rs` | 36 | `Carrier` (single) | `use dorc_aid::Carrier;` |
| `analysis/src/cfg.rs` | 34 | `{AstId, BytePos, Carrier, Channel, LeafId, Span}` | `dorc_aid::Carrier` + `dorc_core::{AstId, BytePos, Channel, LeafId, Span}` |
| `analysis/src/effect.rs` | 33–36 | `{Carrier, Context, EntityRef, FactBacking, Interner, KindId, LeafId, OpaqueToken, ProviderId, SelectorId, Span}` | `dorc_aid::Carrier` + the rest on `dorc_core` |
| `analysis/tests/cfg.rs` | 27 | `{Channel, Severity}` | `dorc_aid::Severity` + `dorc_core::Channel` |
| `oracle/src/lib.rs` | 44 | `{Carrier, Interner, KindId, ProviderId, SelectorId, Symbol}` | `dorc_aid::Carrier` + rest |
| `oracle/src/resolve.rs` | 29 | `{Carrier, Interner, Symbol}` | `dorc_aid::Carrier` + rest |
| `oracle/src/strip.rs` | 35 | `{Carrier, Interner, Span}` | `dorc_aid::Carrier` + rest |
| `oracle/src/reaches.rs` | 43 | `{Carrier, Interner, Span, Symbol}` | `dorc_aid::Carrier` + rest |
| `oracle/src/touches.rs` | 33 | `{Carrier, Interner, Symbol}` | `dorc_aid::Carrier` + rest |
| `oracle/src/wrapper.rs` | 30 | `{Carrier, Interner, Symbol}` | `dorc_aid::Carrier` + rest |
| `oracle/src/verdict.rs` | 62 | `{Carrier, Interner, ProviderId, Rc, Span, Symbol}` | `dorc_aid::Carrier` + rest |
| `oracle/src/validate.rs` | 13 | `{Diag, Interner, Symbol}` | `dorc_aid::Diag` + `dorc_core::{Interner, Symbol}` |
| `oracle/src/predict/parser.rs` | 20 | `{Carrier, Diag, Interner, Span, Symbol}` | `dorc_aid::{Carrier, Diag}` + `dorc_core::{Interner, Span, Symbol}` |
| `plan/src/lib.rs` | 44–48 | `{AstId, ByVouch, Carrier, Channel, CollapseEvidence, CollapseKind, Dialect, EntityRef, FactBacking, Grade, Interner, KindId, Observable, OracleFileId, Predicted, Rc, Rung, Symbol, TrustTier, Verdict}` | `dorc_aid::{Carrier, CollapseKind, CollapseNarrative, TrustTier}` + `dorc_core::{AstId, ByVouch, Channel, Dialect, EntityRef, FactBacking, Grade, Interner, KindId, Observable, OracleFileId, Predicted, Rc, Rung, Symbol, Verdict}` |
| `plan/src/erasability.rs` | 38 | `Diag` (single) | `use dorc_aid::Diag;` |
| `cli/src/main.rs` | 69–72 | `{CollapseEvidence, CollapseKind, Interner, Observable, OutBytes, Predicted, ProvArena, Rc, Severity, Symbol, TrustTier, Verdict}` | `dorc_aid::{CollapseKind, CollapseNarrative, Severity, TrustTier}` + `dorc_core::{Interner, Observable, OutBytes, Predicted, ProvArena, Rc, Symbol, Verdict}` |
| `lint/src/finding.rs` | 11 | `Severity` (single) | `use dorc_aid::Severity;` |
| `lint/src/source_external.rs` | 15 | `{Interner, Severity}` | `dorc_aid::Severity` + `dorc_core::Interner` |
| `lint/tests/report.rs` | 5 | `Severity` (single) | `use dorc_aid::Severity;` |
| `dorc-loom/src/consumer.rs` | 22 | `{Interner, LeafId, ProvArena, Severity, TopCause}` | `dorc_aid::Severity` + `dorc_core::{Interner, LeafId, ProvArena, TopCause}` |
| `dorc-loom/tests/coverage.rs` | 9 | `{BytePos, Interner, LeafId, Span as SourceSpan}` | UNCHANGED (all stay) |

Single-path `use dorc_core::diag::…` / `::catalog::…` / `::tagged::…` / `::evidence::…`
statements (the large majority) are fully handled by §4b. That includes all of
`oracle/src/{marker,carry,report,predict,reserved,entry}.rs`,
`oracle/src/predict/{derive,mark_grammar}.rs`, `plan/src/{invocation,records,whylog}.rs`,
`lint/src/{render,production,source_oracle_solo}.rs`,
`dorc-loom/src/{lib,edit,compile,generate}.rs`, `dorc-loom/tests/{consumer,editable_render}.rs`,
`coverage/src/lib.rs`, and every nested `use` inside `#[cfg(test)]` modules.

### §4d — Step 3: the bare-identifier rename (explicit file list)

Run ONLY over these files (never workspace-wide — see §3a's exclusions):

```
aid/src/narrative.rs   aid/src/lib.rs
analysis/src/effect.rs
plan/src/lib.rs
cli/src/main.rs
coverage/src/lib.rs
sweep/src/drive.rs
```

```sh
sed -i -e 's/\bCollapseEvidence\b/CollapseNarrative/g' \
       -e 's/\bcollapse_evidence\b/collapse_narrative/g' \
       -e 's/\bmint_merge_evidence\b/mint_merge_narrative/g' \
       -e 's/\bEVIDENCE_OPERAND_CAP\b/NARRATIVE_OPERAND_CAP/g' <files>
```

Then hand-rename the remaining `*_evidence` LOCAL bindings, which are not worth a regex
(exhaustive, verified): `cli/src/main.rs:1155` `entry_evidence` · `:1157` `classify_evidence` ·
`:1176`/`:3140`/`:3143`/`:1573` `decline_evidence` · `:1437`/`:1579` `merge_evidence` ·
`:3356`/`:3363` `_collapse_evidence` (doc + param) · `analysis/src/effect.rs:1753` `_evidence`.
`plan/src/lib.rs:4941,5909` and `coverage/src/lib.rs:487` and `sweep/src/drive.rs:64` carry
`_collapse_evidence` tuple bindings (covered by the regex above).

MUST-NOT-TOUCH (assert zero diff in these after the pass):
`hostsim/src/lib.rs` (`read_host_evidence`, `HostEvidenceLimits`, `ScopedHostEvidence`) ·
`aid/src/diag.rs` (`HostEvidenceAdmissionRefused`, `HostEvidenceRefusalKind`) ·
`plan/src/{records,whylog}.rs` (`HostEvidence*`) · `errorloom/src/editable.rs`
(`EditRefusalEvidence`) · `dorc-loom/src/bin/dorc-loom.rs` (`bounded_evidence`,
`MAX_REFUSAL_EVIDENCE`). Verification command:
`rg -n '\bevidence\b|Evidence' -g '*.rs' crates/ | rg -v 'HostEvidence|EditRefusalEvidence|MAX_REFUSAL_EVIDENCE|bounded_evidence|law-collapse-mints-evidence|two-plane|Lane A'`
— every surviving hit must be a deliberate law-slug citation (§3a `dec-law-slugs-frozen`).

### §4e — Step 4: hardcoded cross-crate paths

Three files hold literal `core/src/…` paths pointing at the generated catalog lock, plus the
generated-file header. All must re-point at `aid`:

| file | line | before | after |
|---|---|---|---|
| `dorc-loom/src/bin/dorc-loom.rs` | 388 | `.join("../core/src/catalog_lock.rs")` | `"../aid/src/catalog_lock.rs"` |
| `dorc-loom/src/repository.rs` | 380 | `const CATALOG: &str = "spike/crates/core/src/catalog_lock.rs";` | `"spike/crates/aid/src/catalog_lock.rs"` |
| `dorc-loom/tests/consumer.rs` | 22 | `const CATALOG_PATH: &str = "crates/core/src/catalog_lock.rs";` | `"crates/aid/src/catalog_lock.rs"` |
| `dorc-loom/tests/consumer.rs` | 23 | `const CODE_PATH: &str = "crates/core/src/diag.rs";` | `"crates/aid/src/diag.rs"` |
| `dorc-loom/tests/consumer.rs` | 574 | `.join("../core/src/catalog_lock.rs")` | `"../aid/src/catalog_lock.rs"` |
| `dorc-loom/tests/fixpoint.rs` | 18 | `.join("../core/src/catalog_lock.rs")` | `"../aid/src/catalog_lock.rs"` |
| `dorc-loom/Cargo.toml` | comment block | "dorc-core OWNS its tagged-span vocabulary and takes NO dependency on errorloom" | "dorc-aid OWNS …" (the kernel-dep-cleanliness point is unchanged; only the crate name) |

Verification: `rg -n 'crates/core|\.\./core/src' -g '!target' spike/` must return only
`spike/Cargo.toml`'s `members` line (which legitimately still names `crates/core`).

---

## §5 — Workspace / Cargo.toml changes

### §5a — `spike/Cargo.toml`

```toml
members = ["crates/core", "crates/aid", "crates/syntax", "crates/analysis", "crates/oracle", "crates/plan", "crates/hostsim", "crates/sweep", "crates/cli", "crates/coverage", "crates/lint", "crates/errorloom", "crates/dorc-loom"]
```

(`aid` placed immediately after `core` — the list is dependency-ordered by convention.) The
`[workspace.lints.*]` tables are untouched and apply to `aid` via `[lints] workspace = true`.

### §5b — `spike/crates/aid/Cargo.toml` (NEW)

```toml
[package]
name = "dorc-aid"
version = "0.0.0"
edition.workspace = true
rust-version.workspace = true
publish = false

# The DESCRIBE plane (`288` §2a): narrative records, the diagnostic catalog, render seats, and
# the no-throw `Carrier`. Depends on `core` and NOTHING else — DST-clean by construction (no
# clock, RNG, filesystem, or network, directly or transitively), so the whole pipeline stays a
# pure function of its inputs inside deterministic-simulation tests. Adding a dependency here
# requires proving it carries no nondeterminism (see spike/CLAUDE.md `inv-determinism`).
[dependencies]
dorc-core = { path = "../core", version = "0.0.0" }

[lints]
workspace = true
```

Dep direction `aid → core`, one edge, no cycles. Confirmed against `288` §2e's census and
re-verified in §2d: nothing in the six staying core modules references any moving type, so
there is no `core → aid` back-edge to break. +SURE.

### §5c — Consumer `[dependencies]` additions

Add `dorc-aid = { path = "../aid", version = "0.0.0" }` immediately after the existing
`dorc-core` line in exactly these eight manifests:

`crates/syntax` · `crates/analysis` · `crates/oracle` · `crates/plan` · `crates/cli` ·
`crates/coverage` · `crates/lint` · `crates/dorc-loom`

UNCHANGED (verified they reference zero moving symbols and need no dep — they call functions
whose signatures mention `Carrier`, which works fine without naming the type):
`crates/hostsim` (incl. its `[dev-dependencies]`) · `crates/sweep` · `crates/errorloom`.

`cargo deny check licenses bans sources` is unaffected: no new external dependency, and the
existing exact-pins in `cli` are untouched.

### §5d — `aid/src/lib.rs` crate-root attributes

```rust
#![forbid(unsafe_code)]
#![expect(
    missing_docs,
    clippy::indexing_slicing,
    reason = "relocated round-19/22 seeded diagnostic code (288:phase-aid-crate-extraction); \
              ratchets away as layers are replaced"
)]
```

Both are REQUIRED and both are FULFILLED (an unfulfilled `expect` warns, and clippy runs
`-D warnings`). Verified in-tree: `missing_docs` fires on the moved `Severity` variants
(`Error`/`Warning`/`Note`), on `Carrier::{value, diags}` and `Carrier::push`, and on the
`pub mod diag/catalog/tagged/narrative` lines; `clippy::indexing_slicing` fires on
`diag.rs:1953–1954` (`&text[..split]` / `&text[split..]`). The moving files are otherwise
fully documented — a `#[expect]`-scan found exactly one local attribute in the whole moving
set (`diag.rs:1184`, `match_same_arms`/`too_many_lines` on `registry()`), which rides along
untouched. Core keeps BOTH lints in its own expect (§2e item 4).

---

## §6 — `SiteId` relocation (`288` §2d nicety) — **TAKE**, as an additive prelude commit

**Disposition: TAKE.** Recommended as the ONE separable, individually-green prelude commit.

Rationale:
- `SiteId`'s own doc (`diag.rs:900–905`) says it was promoted into `core` for
  `inv-site-keyed-results` ("the same `(leaf, member)` pair the cli's probe-records and the
  apply plan's steps share"); `LeafId`'s doc in `core/src/lib.rs:52–57` already
  forward-references it as the reason `LeafId` lives in core rather than `plan`. Both docs
  argue it is decide-plane identity, not describe-plane payload.
- It depends on exactly one type (`LeafId`) and nothing else. Zero friction.
- It removes 91 `SiteId` occurrences across 13 files from the cutover's blast radius entirely:
  after the prelude, `aid/src/diag.rs` carries `pub use dorc_core::SiteId;` and every existing
  `dorc_core::diag::SiteId` / `dorc_aid::diag::SiteId` path keeps resolving. The `288` §4/§5
  lint-unification and CLI-error phases both key findings by site and will want it in core.
- It also makes `narrative.rs`'s single core-ward import (`use dorc_core::SiteId;`) honest
  rather than an aid-internal re-export chain.

Prelude commit shape (individually green, zero behavior change):
1. `git mv`-free edit: move `pub struct SiteId` + `impl SiteId` (`diag.rs:906–920`) into
   `core/src/lib.rs`, immediately after `LeafId`.
2. In `diag.rs`, add `pub use crate::SiteId;` where the definition was, so every existing
   `dorc_core::diag::SiteId` path keeps working with ZERO consumer edits.
3. Retarget `core/src/lib.rs:53`'s `[`diag::SiteId`]` intra-doc link to `[`SiteId`]`.
4. Gates green; commit `(AI re core) Site the diagnostic site-identity beside the leaf id`.

`GroupingKey` / `FineKey` / `CoarseKey` stay in `diag.rs` and move with it — they are render/
rollup vocabulary, not identity. Flagged for the conductor only insofar as §9's
`flag-siteid-is-a-core-custody-call` records that this widens `core` by ~15 LOC against
`288:rul-core-stays-light-custody`; the counter-argument is that it is DECIDE-plane identity,
which is exactly what `core` is for.

If the conductor REJECTS: skip the prelude, keep `SiteId` in `aid/src/diag.rs`, apply §2e item
2's fallback (de-link `core/src/lib.rs:53`'s `[`diag::SiteId`]` to plain backticks), and use
`use crate::diag::SiteId;` in `narrative.rs`. Everything else in this spec is unchanged.

---

## §7 — DRAFT `spike/crates/aid/CLAUDE.md`

Registry discipline per `spike/CLAUDE.md`: one rule per bullet, greppable slug, grouped under
standing headers, APPEND rather than restructure, cite outside sources as `docID:slug`.

```markdown
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
- **collapse-mints-narrative** (`AID-NEEDS:law-collapse-mints-evidence` — law slug
  unchanged, type renamed) — every safety-narrowing (meet-to-⊤, refuse, decline, wall,
  demote, cancel) mints a decision-inert `CollapseNarrative` carrying the collapse's
  OPERANDS, demanded by the collapse constructor at the VALUE level. The nine mint sites
  live in `analysis` (1), `plan` (3), and `cli` (5) — NOT here; this crate owns the TYPE
  and its constructors, never the mint schedule.
- **narrative-eq-excluded-at-the-carrier** — `CollapseNarrative` derives `Eq`, but any
  fixpoint-iterated lattice value carrying one hand-writes `PartialEq` to EXCLUDE it (the
  `analysis::effect::Reach` precedent): an evidence-sensitive lattice `Eq` never
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
```

### §7a — What MOVES / gets POINTERS from existing registries

| source | bullet | phase-1 disposition |
|---|---|---|
| `spike/CLAUDE.md` User-aid block | `two-plane-aid-law` | STAYS at root (binds every collapse site in `analysis`/`plan`/`cli`); DUPLICATED into aid's registry — permitted, and warranted, by root registry discipline ("repetition is valid for genuinely deeply-critical invariants") |
| ″ | `collapse-mints-evidence` | STAYS at root; body prose renames the type to `CollapseNarrative`; a sharpened cousin lands in aid as `collapse-mints-narrative` |
| ″ | `trust-tier-is-syntax`, `render-form-unwelded`, `one-catalog-no-legacy`, `defining-case-catalog`, `error-authorship-tier`, `error-prose-conductor-flow` | STAY at root (they bind builders and conductors working anywhere); aid's registry carries the crate-local restatement |
| ″ | `replay-editability-is-provenance`, `replay-executor-ownership`, `decline-class-emission`, `report-lane-versioned-entry`, `report-surface-massaging-carve`, `whylog-write-only-replay`, `rul-chain-is-pull-only` | STAY at root, UNCHANGED — these are errorloom / oracle / plan / cli law, not aid-crate law |
| `spike/crates/core/CLAUDE.md` | `inv-no-throw-here` | REWORDED in core (see §8b); the `Carrier`-owning half lands in aid |

Net: NO bullet is deleted from `spike/CLAUDE.md` in phase 1. The block gains a co-siting
pointer only. See FLAG `flag-user-aid-block-relocation-depth` (§9) — §9 of `288` says
"User-aid block relocation POINTER", which I read as a pointer, not a wholesale move; the
alternative reading is a conductor call.

---

## §8 — Steering-sync edit list, drafted verbatim (`288` §9, phase 1 only)

### §8a — `spike/CLAUDE.md`

**Edit 1 — relocate `rul-strawman-formats-no-compat` into the stability-ledger region.**
It currently sits at the tail of `## Boundaries` (added by `fbbf88f1`), which is the wrong
home: it is a stability law, not a boundary. DELETE it from `## Boundaries` and INSERT it in
`## Language & off-ramp law`, immediately after the `stability-ledger` bullet, byte-identical
text:

```markdown
- **rul-strawman-formats-no-compat** — pre-user, EVERY versioned wire/format/env
  name (`dorc-lint-format/1`, `DREP_V1`, `dorc-whylog/1`, `dorc-records/1`, …) is
  strawman: rename/reshape in place, all sites in one commit; never an adapter, alias, or
  mapping from a historical spelling. "Permanent once published" clauses activate at
  publication, not before. Applies generally; *ask* the human if you suspect
  they want to pay the prices of backwards-compatibility over velocity/simplicity.
```

(Note the bullet gains the `**…**` slug emphasis its neighbours carry; it landed without it.)

**Edit 2 — the User-aid block header gains a co-siting pointer.** Replace the section header
line:

```markdown
## User-aid & diagnostics law (registry + laws: root `AID-NEEDS.md`; build phase: `27V`)
```

with:

```markdown
## User-aid & diagnostics law (registry + laws: root `AID-NEEDS.md`; build phase: `27V`; the
describe-plane CRATE and its crate-local sharpenings: `spike/crates/aid/CLAUDE.md` — every
type below lives in `dorc-aid`, never `dorc-core`, since `288:phase-aid-crate-extraction`)
```

**Edit 3 — the narrative rename, inside `collapse-mints-evidence`.** Replace:

```markdown
- **collapse-mints-evidence** (`AID-NEEDS:law-collapse-mints-evidence`) — every
  safety-narrowing (meet-to-⊤, refuse, decline, wall, demote, cancel) mints
  decision-inert evidence carrying the collapse's OPERANDS, demanded by the collapse
  constructor at the VALUE level (pure data; kernels stay pure — arena registration is
  post-pass per `22D`). Evidence is Eq-EXCLUDED from lattice equality (fixpoint
  termination, `22W` §2) and k-capped. `Unexplained` is constructible but renders
  self-advertisingly.
```

with:

```markdown
- **collapse-mints-evidence** (`AID-NEEDS:law-collapse-mints-evidence`; the law slug is
  unchanged, the TYPE is now `aid::CollapseNarrative` per `288:rul-narrative-layer-naming`)
  — every safety-narrowing (meet-to-⊤, refuse, decline, wall, demote, cancel) mints a
  decision-inert NARRATIVE record carrying the collapse's OPERANDS, demanded by the
  collapse constructor at the VALUE level (pure data; kernels stay pure — arena
  registration is post-pass per `22D`). The record is Eq-EXCLUDED from lattice equality
  (fixpoint termination, `22W` §2) and k-capped. `Unexplained` is constructible but renders
  self-advertisingly.
```

**Edit 4 — `two-plane-aid-law`, one clause.** Replace `Aid-evidence is decision-inert at the
TYPE level (sealed; …)` with `The aid-narrative plane is decision-inert at the TYPE level
(sealed; …)` — the rest of the bullet is byte-unchanged.

**Edit 5 — registry-discipline crate count.** Line 25: `**Registry discipline** (this file and
all seven crate files)` → `all eight crate files` (`crates/aid/CLAUDE.md` is the eighth;
existing seven are analysis/cli/core/hostsim/oracle/plan/syntax).

**Edit 6 — the reading-order line.** Line 19: `→ `spike/crates/<c>/CLAUDE.md` for the crate you
touch` is already generic and needs no edit. NO OTHER `spike/CLAUDE.md` edits land in phase 1
(the safety-block executor line and the loom-placement law are `288` §9's PHASE 5 items — do
not pull them forward).

### §8b — `spike/crates/core/CLAUDE.md` (trims; keep it LIGHT per `288:rul-core-stays-light-custody`)

**Edit 1 — the role line gains one sentence.** Replace:

```markdown
Role: the shared vocabulary every crate agrees on FIRST (dac-B: agree the types
before consumers build, or two incompatible graphs grow). Read `spike/CLAUDE.md`
first — its invariant clusters are this crate's law; this file carries only the
core-local sharpenings.
```

with:

```markdown
Role: the shared vocabulary every crate agrees on FIRST (dac-B: agree the types
before consumers build, or two incompatible graphs grow) — the DECIDE plane. The
DESCRIBE plane (diagnostics, catalog, render, narrative records, `Carrier`) is
`crates/aid`, which deps this crate and is never depended upon BY it (`288` §2a).
Read `spike/CLAUDE.md` first — its invariant clusters are this crate's law; this file
carries only the core-local sharpenings.
```

**Edit 2 — `inv-no-throw-here` reword.** Replace:

```markdown
- **inv-no-throw-here** — `core` is the no-throw spine; constructors return data,
  never panic.
```

with:

```markdown
- **inv-no-throw-here** — constructors return data, never panic. (`Carrier<T>`, the
  no-throw spine type itself, lives in `crates/aid`; `core` returns bare values and
  never accumulates diagnostics.)
```

**Edit 3 (only if §6 lands) — one bullet appended to `## Law — vocabulary discipline`:**

```markdown
- **site-identity-is-decide-plane** — `SiteId` (`leaf` + optional in-loop `member`) lives
  here beside `LeafId`, not in the describe plane: it is the identity two same-command
  sites must not collapse across (`inv-site-keyed-results`), shared by the probe-records
  lane, the apply plan's steps, and every diagnostic. `aid` re-exports it; it is never
  re-minted.
```

NOTHING else lands in `core/CLAUDE.md`. Specifically: do NOT add catalog, prose, loom, or
render notes — `288:rul-core-stays-light-custody` is explicit.

### §8c — NOT in phase 1

`AID-NEEDS.md` edits (`288` §9 assigns lint-namespace caveats to phase 3, CLI-error rows to
phase 4) · `spike/CLAUDE.md`'s safety-block executor line and loom-placement law (phase 5) ·
`AGENTS.md` (human's own hand) · `Research/LIVING_STATUS.md` (conductor). The
`Where the build stands` section of `spike/CLAUDE.md` is conductor-maintained; the executor
should NOT edit it.

---

## §9 — Commit sequencing + verification

### §9a — Commits

**Commit A (prelude, additive, individually green) — only if §6 is ACKED:**
`(AI re core) Site the diagnostic site-identity beside the leaf id`
Scope: `core/src/lib.rs` (+`SiteId`), `core/src/diag.rs` (`pub use crate::SiteId;`, delete the
definition), `core/src/lib.rs:53` doc-link. Zero consumer edits. Four gates green.

There is NO other genuinely-additive prelude. In particular: creating an empty `crates/aid` and
adding `dorc-aid` deps ahead of the cutover would be green but produces an unused-dependency
state and splits one mechanical change across two reviews for no benefit — do not do it.
(`288` §8's phase 0, `fix-lint-tally-pluralization`, is a DIFFERENT lane running `0∥1`; it is
not this executor's work.)

**Commit B (the atomic cutover) —**
`(AI re !! typ) Split the describe plane into its own crate and rename the record layer`
(gitlabels: `re` for the non-behavioural refactor, `!!` for widespread, `typ` for the Rust
type-structure move, `AI` mandatory. Do NOT name files, crates, or slugs in the message —
`AGENTS.md` project-management law.)

Everything in §2–§5 and §8 lands in this ONE commit: the eight `git mv`s, the new
`aid/{Cargo.toml,CLAUDE.md,src/lib.rs}`, the workspace member, the eight dep additions, the
import rewrites, the narrative rename, the hardcoded paths, and the steering-sync edits. It
must not be split: a partial state has `core` re-exporting types it no longer owns, and the
`288` §2d ruling is explicit ("one atomic cutover commit").

Suggested working order inside the commit (minimises compiler noise):
1. `git mv` the five src files + three test files.
2. Write `aid/Cargo.toml`, `aid/src/lib.rs` (with `Severity`/`Carrier`/two tests moved in),
   `aid/CLAUDE.md`.
3. Delete the moved items from `core/src/lib.rs`; apply §2e's four edits; `rmdir core/tests`.
4. Workspace member + eight dep additions (§5a, §5c).
5. §3c/§3d — fix the moved files' own imports.
6. §4b global path rewrite, then §4c hand-splits, then §4d bare-identifier rename.
7. §4e hardcoded paths.
8. §8 steering-sync edits.
9. Build; fix residue; gates.

### §9b — Verification plan

`28A:finding-incremental-clippy-serves-stale-clean` RIDER (binding for all r28 worktree
briefs): incremental caching on this Win/mise/worktree setup serves STALE-CLEAN clippy. Before
the clippy gate, cold the cache for EVERY touched crate:

```sh
# from spike/
mise exec -- cargo clean -p dorc-core -p dorc-aid -p dorc-syntax -p dorc-analysis \
  -p dorc-oracle -p dorc-plan -p dorc-cli -p dorc-coverage -p dorc-lint \
  -p dorc-loom -p dorc-hostsim -p dorc-sweep
```

Then the four gates, in order, all from `spike/` via mise (never `--no-verify`, never skip):

```sh
mise exec -- cargo fmt --check
mise exec -- cargo clippy --workspace --all-targets -- -D warnings
mise exec -- cargo deny check licenses bans sources
mise x -- typos spike        # from the WORKTREE ROOT, not spike/
```

Then, in order:

```sh
mise exec -- cargo build --workspace     # force fresh before trusting e2e
mise exec -- cargo test --workspace      # incl. the aid doctests — the compile_fail seal
sh e2e/run.sh                            # FOREGROUND, generous timeout; expect 97/97
```

Expected result — **ZERO behavior change**:
- `sh e2e/run.sh` → 97/97 (94 `e2e/cases/` + 3 `e2e/lint-cases/`; count the dirs, do not trust
  the literal — `cli/CLAUDE.md:count-drifts`).
- `git status` shows NO modification to any file under `e2e/cases/`, `e2e/lint-cases/`,
  `e2e/lint-real-cases/`, or `crates/dorc-loom/cases/`. Goldens and `.loom` transcripts are
  byte-identical; verified pre-emptively that none of them contains a crate path.
- `dorc-loom --test fixpoint` and the errorloom render-level fixpoint stay green (the lock file
  moves BYTE-IDENTICALLY; only the paths that FIND it change).
- **NEVER run `BLESS=1`.** Bless is exclusive and orchestrator-only; a golden diff in this lane
  is a BUG, not a re-bless trigger.

Anti-masking check specific to this lane: `git show --stat` on the cutover must show the five
moved src files as pure renames (`R100` / near-100) apart from the enumerated import edits. A
large content delta on `catalog_lock.rs` means the generator ran and the lock regenerated —
stop and investigate.

---

## §10 — Risks and FLAGS (flagged UP; not resolved here)

Flags a `tc-*`-shaped judgment call the conductor must rule on:

1. **`flag-mark-grammar-carrier-collision`** (RISK, highest) — `oracle/src/predict/mark_grammar.rs`
   defines a LOCAL `enum Carrier` (the mark carrier: `#:` vs `:`, line 89) with 15 usages,
   semantically unrelated to `dorc_core::Carrier`. Any workspace-wide `s/Carrier/…/` destroys
   it. §4b's rewrites are all prefixed with `dorc_core::` precisely to avoid this, and §4c never
   touches that file's `Carrier`. Mechanical guard: after the rewrite, assert
   `rg -c '\bCarrier\b' oracle/src/predict/mark_grammar.rs` still returns 15 and that file's
   diff contains only the `use dorc_core::diag::{…}` → `dorc_aid::diag::{…}` line.

2. **`flag-user-aid-block-relocation-depth`** (JUDGMENT) — `288` §9 says "User-aid block
   relocation pointer". §7a/§8a Edit 2 reads that as a POINTER (block stays at root, aid gets a
   crate-local registry that duplicates the deeply-critical bullets). The alternative reading is
   a wholesale MOVE of the block into `aid/CLAUDE.md`. Argument for the pointer reading:
   `two-plane-aid-law`, `collapse-mints-evidence`, `error-authorship-tier`, and
   `error-prose-conductor-flow` bind builders working in `analysis`/`plan`/`cli` who will never
   open `aid/CLAUDE.md`, and `rul-claudemd-fires-per-directory` cuts BOTH ways. Conductor call.

3. **`flag-law-slug-carries-retired-word`** (JUDGMENT, deferred by construction) —
   `AID-NEEDS:law-collapse-mints-evidence` and `spike/CLAUDE.md`'s bullet slug
   `collapse-mints-evidence` still spell the retired register. `AID-NEEDS.md` is a KEPT-CURRENT
   registry (so the brief's "historical docs stay as written" carve does not cover it), but
   `288` §9 schedules `AID-NEEDS` edits for phases 3–4. §3a `dec-law-slugs-frozen` therefore
   leaves both slugs alone and renames only body prose. Conductor: re-slug now, at phase 3, or
   never?

4. **`flag-siteid-is-a-core-custody-call`** (JUDGMENT) — §6 widens `core` by ~15 LOC against
   `288:rul-core-stays-light-custody`. The counter is that `SiteId` is decide-plane identity
   (`inv-site-keyed-results`) and `LeafId`'s existing doc already justifies it. TAKE recommended;
   the reject path is fully specified in §6 and costs nothing.

5. **`flag-preexisting-dead-doclinks`** (LATENT DEFECT, pre-existing) — `diag.rs:243`
   (`[`crate::DiagCode`]`), `:1059` (`[`crate::Diagnostic::span`]`), `:2174`
   (`[`crate::Exempt::Explanation`]`) reference items that do not exist: `Diagnostic` and
   `Exempt` were removed with the legacy lane (`one-catalog-no-legacy`), and `DiagCode` is not
   re-exported at core's root. `rustdoc::broken_intra_doc_links = "warn"` is in the workspace
   lint table, but the four-gate set does NOT run `cargo doc`, so these have been silently
   latent. NOT created by this lane. Recommend de-linking them to backticked plain text as part
   of the move (three lines, zero risk); flagging in case the conductor prefers a separate
   janitor commit or wants `cargo doc` added to the gate set (out of scope here).

6. **`flag-catalog-ratchet-baseline-skips-once`** (BEHAVIOURAL, expected, one commit only) —
   `catalog_defining_cases.rs`'s `ratchet_only_shrinks` reads its committed baseline via
   `git show HEAD:spike/crates/core/tests/catalog_defining_cases.rs`. On the cutover commit the
   path constant changes to `crates/aid/…`, which HEAD does not yet contain, so the test takes
   its documented "no committed baseline — skipping" branch exactly once and re-arms on the next
   commit. That is the file's own contract for a new file, but it means the shrink-only ratchet
   is unenforced for one commit. Do not "fix" it by pointing the constant at the old path.

7. **`flag-diag-tidy-scan-set-is-now-incomplete`** (PRE-EXISTING, worth a conductor look) —
   `diag_tidy.rs`'s `SCANNED_CRATES` is `{core, syntax, analysis, oracle, plan, cli, coverage,
   hostsim}` — it does NOT scan `lint` or `dorc-loom`, both of which now hold real emit-adjacent
   code. §3e re-points `core` → `aid` and changes nothing else. Widening the scan set is a
   separate, behaviour-visible decision (it could turn `every_catalog_variant_is_constructed`
   green for codes only constructed in `lint`'s tests) and is NOT taken here.

8. **`flag-prov-stays-core-is-frictionless`** (CONFIRMED, no action) — `288` §2b called
   prov-stays-core "the frictionless disposition". Re-verified: `prov.rs` references zero moving
   types, and exactly one diag payload (`CmdsubOperandTop`) reaches DOWN into `ProvId`/`TopCause`
   — the legal direction. No friction found. (`TopCause` is in `lib.rs`, not `prov.rs` — see
   §2c's correction; disposition unaffected.)

9. **`flag-compile-fail-doctest-crosses-the-seam`** (CONFIRMED, no action) — the `narrative.rs`
   `compile_fail` doctest feeding a record to `dorc_core::room::mint_from_room` works unchanged
   across the crate boundary (`aid` deps `core`; doctests see the dep graph; `mint_from_room` is
   `pub`). It is the ONLY doctest in the entire moving set. Its continued FAILURE to compile is
   the seal; `cargo test --workspace` is the gate that proves it.

10. **`flag-test-file-placement-is-provisional`** (SCOPE) — §2b puts all three moved tests in
    `crates/aid/tests/` as ordinary `.rs` integration tests. `288` §3 (`rul-flat-test-tree`) will
    later make `crates/aid/tests/` the primary `.loom` collection, where `.rs` tests and `.loom`
    data coexist by design ("Cargo compiles only top-level `tests/*.rs`"). No conflict, but the
    executor should NOT pre-create any `looms/` or `cases/` subdirectory — that is phase 5.

11. **`flag-hostsim-sweep-stay-dep-free`** (VERIFIED, watch item) — `hostsim` and `sweep`
    reference zero moving symbols and get no `dorc-aid` dep. They DO call functions returning
    `Carrier<T>` and destructure narrative-bearing tuples (`sweep/src/drive.rs:64`), which is
    legal without the dep (you may use a foreign type you cannot name). If the build demands a
    dep in either, something else changed — investigate before adding it, because adding
    `dorc-aid` to `hostsim` puts the describe plane inside the DST nondeterminism quarantine.

12. **`flag-catalog-lock-generation-wiring`** (MECHANICAL, covered) — §4e lists all six literal
    paths in `dorc-loom` pointing at `core/src/catalog_lock.rs` / `core/src/diag.rs`. Four are
    `CARGO_MANIFEST_DIR`-relative (`../core/src/…`), two are repo-relative strings used for git
    reads and receipt binding (`repository.rs:380`, `tests/consumer.rs:22-23`). A missed
    repo-relative one fails LOUDLY (git read error / receipt mismatch); a missed
    manifest-relative one fails loudly at `--test fixpoint`. No silent-failure mode identified.

---

## §11 — Disposition estimate: `prop-mint-completeness-hardening` (`288` §2c / §10 ask 1)

**Recommendation: PHASE 2 (`288:phase-mint-seam-and-scaffold`), NOT this lane's executor.**
Flagged, not built — `288` §10 records it as the arc's only still-open plan proposal, and this
map neither builds nor pre-empts it.

Reasoning:

- **It is a behaviour change, and this lane's whole contract is that there is none.** Promoting
  the merge-mint pairing `debug_assert` to a release-mode gate makes a previously debug-only
  check fireable in release. That is the exact class of change the atomic cutover's "goldens
  byte-identical / e2e 97/97" verification is designed to detect as a FAILURE. Co-landing them
  means a reviewer can no longer read a clean diff as proof of correctness.
- **Its blast radius is orthogonal.** The completeness test enumerates the nine collapse-mint
  sites across `analysis` (1), `plan` (3), and `cli` (5) — files this lane touches ONLY for
  import lines. Mixing an enumerating tidy-test into a rename-and-move diff makes both harder
  to review, against `27U:map-then-execute-split`'s whole point.
- **Phase 2 is where the reviewer's attention already is.** `288` §4 builds the mint seam
  (mirror-union, the caseless-slug guarantee, `dorc-loom scaffold`); a completeness gate over
  collapse-mint sites is the same subject matter one level down, and phase 2 is already going to
  exercise "a new thing must come to exist or go red" end-to-end.
- **Sizing (-GUESS):** small — the `mint_merge_evidence`-shaped post-pass mirrors
  `mint_top_causes` and the assert already exists (`analysis/src/effect.rs:1053` guards
  `mint_top_causes`; the merge-mint at `:1379` is the unguarded twin). A tidy-style test
  enumerating collapse sites against minted `CollapseKind` variants is `diag_tidy.rs`-shaped
  work and would naturally live in `aid/tests/` after this lane lands — a further argument for
  ordering it AFTER, not with.

Cheapest alternative if the conductor wants it sooner: land it as a THIRD, separate commit on
this lane AFTER the cutover is green, so the cutover's zero-diff property is verified in
isolation first. That costs one extra gate cycle and preserves the review property. It should
never be folded INTO commit B.
