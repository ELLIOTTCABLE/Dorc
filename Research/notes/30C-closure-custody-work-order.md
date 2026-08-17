# 30C — The closure/custody work order (`28Q` §8 stage-ii, phase 3: closures and sourcing)

The executable spec for the `lane-custody-closures` phase-3 build, written against the code as it
stands at `73359e0f` so a cold successor executes from here and re-derives nothing. Phases 1–2
(the custody-composite repair, definition-grade row keying, the `Measurement` separation) are
LANDED and are this document's floor, not its subject.

Grades: `+SURE` measured in this tree · `~SUSPECT` read but unmeasured · `-GUESS` inference ·
`--WONDER` open question. Slugs are minted here and referenced elsewhere as `30C:<slug>`.

## §1 — The ruled contract (the ground; nothing below may contradict it)

Human-typed 2026-08-17, `307:§ack-implementation-open`, restated verbatim-in-substance:

- `rul-only-oracle-sourcing-mints-speakers` — the ONLY speaker-minting merge is an explicit
  **top-level `.`** in a **marked (dorc-lang) file**, of a file that is itself
  **dorc-lang/oracle-code with NO top-level commands**. Already law at
  `oracle/CLAUDE.md rul-speaker-minting-is-oracle-sourcing-only`.
- `rul-inertness-is-contract-never-engine-fact` — load-inertness is HYGIENE/CONTRACT. The license
  grounds on the MARKER plus the no-top-level-commands CONTRACT; refusals ATTRIBUTE to the
  contract; no diagnostic, doc-comment, narrative, or code comment anywhere may claim an engine
  PROOF of inertness. This is the modifier on every `28Q` §9 pin 1/2/9/10 ack.
- `rul-cli-coloading-composes-nothing` — naming several files on one command line is INGESTION.
  It composes no custody, no family, no speaker — permanently, until the human redesigns ingestion.
- `rul-book-sourcing-mints-no-speaker` — a book's `.` mints no speaker. Its only book-side value
  is UN-WALLING, and that lands ONLY together with `28K:res-book-ships-its-load-closure` (the
  sourced closure materializes at apply) plus the executing e2e cell.

Two rulings from `28Q` §9 pin 10, closed [TYPED 2026-08-17]:

- membership stays **granular to the needed licensure/state/behaviour** — no global
  propagate-everything rule is built; each consumer asks its own question with its own
  conservatism.
- diamond loading is **effectively allowed**, keyed to RESOLUTION-identity (bytes × what the
  file's own sourcing resolved to); differing bytes refuse.

## §2 — The custody model (the one design decision this order makes)

`dec-custody-is-containment-not-equivalence`. Custody is an ASYMMETRIC containment relation over
the include-tree, never an equivalence class over files.

    custody(F) = {F} ∪ transitive descendants of F along SPEAKER-MINTING edges

An edge `P → C` is speaker-minting iff: `P` is a marked file that is **not the book**; the `.`/
`source` sits at `P`'s TOP LEVEL; `P`'s target word is a source-literal path the driver resolved;
and `C` satisfies the contract (marked, no top-level commands).

The licensure question is exactly one, asked at exactly one seat:

    does the VOUCHER's custody reach the RESOLVED declaration's file?

Why containment and not a scalar unit-id compared with `==` (`28Q` §2, the `28R` review's finding):
a scalar merges siblings. Entry `E` sourcing strangers `A` and `B` would give `A` and `B` one
unit-id and dissolve the fence. Containment gives the ruled shape directly — an ANCESTOR edge takes
custody of what it sources, SIBLING/COUSIN edges fence (`28M` §10
`dir-ownership-is-transitive-inclusion`), and the two-strangers case leaves `A` and `B` mutually
fenced beneath `E`'s custody of the composition.

`fnd-book-edges-are-not-speaker-edges` [+SURE, from `rul-book-sourcing-mints-no-speaker`]: because
book `.`-edges are excluded from the relation, the two-strangers-under-a-BOOK case never even
reaches the fence — the book contributes no edges, so `A` and `B` are their own roots by
construction. The `E`-is-an-oracle case is the one the containment shape is load-bearing for.

`dec-definitioncustody-carries-the-relation-not-a-root`. `core::DefinitionCustody` today is a
STAMP: measured, nothing in the workspace compares two of them for a decision (`+SURE` — the only
readers are `plan/src/lib.rs:542` wrapping it into `LicenseCustody::Vouched`, and the mint sites at
`1747`/`1875`; the sole DECISION seat is `HelperIndex::resolve`'s `chosen.file != asker`). So the
re-key keeps the newtype's *shape* and moves the DECISION behind a core-sited containment
predicate. `28M` bitem3's seam promise ("internals change, consumers still only compare") holds in
substance with zero consumer churn; the deviation from its letter is that the compare is
asymmetric (`reaches`), which containment forces. Recorded as a deviation in §8.

## §3 — Enumerated seats (file:line at `73359e0f`)

**Load/dialect surface**
- `spike/crates/oracle/src/load_inert.rs:59` `item_is_load_inert` — admits `FuncDef` and wordless
  `Simple` only; `. ./other.sh` is refused (pinned negatively at `:204`). THE amendment seat.
- `spike/crates/oracle/src/load_inert.rs:31` `lint_load_inert` — the marked-file refusal; its
  diagnostic is `DiagCode::OracleFileNotLoadInert`. Refusal attribution re-words to the CONTRACT
  (`rul-inertness-is-contract-never-engine-fact`); prose is `[unwritten:]`-tier and
  conductor/human-owned (`error-authorship-tier`) — builders mint structure, never words.

**Closure/custody**
- `spike/crates/oracle/src/closure.rs:223` `HelperIndex::build(srcs, book)` — indexes a source only
  when its whole top level passes `item_is_load_inert`; `book` is threaded explicitly.
- `spike/crates/oracle/src/closure.rs:403` `HelperIndex::resolve` — `chosen.file != asker` at
  `:426`. THE custody predicate.
- `spike/crates/oracle/src/closure.rs:335` `closure_for(file, body)` — the walk; `contributing`
  drives per-file constant capture.
- `spike/crates/oracle/src/closure.rs:505` `agree` — the load-edge collision report; differing
  bytes refuse. The diamond rule re-keys its identity to bytes × resolved sourcing.
- `spike/crates/core/src/lib.rs:127` `DefinitionCustody` + `:129` `of_defining_file`.
- `spike/crates/core/src/definition.rs:86` `DefinitionId::custody` — the ONE crossing.
- `spike/crates/analysis/src/funcenv.rs:599` `custody_of_source_index` — the index→custody crossing.

**The 15 `HelperIndex` construction/consumption sites** (every one must receive the closure data or
provably not need it):
- `cli/src/main.rs:735` (build, book = last), `:2596`, `:2633`, `:2731`, `:4823` (build, `None`)
- `cli/src/survival.rs:391`, `:1142`, `:1356`, `:1404`, `:1921`
- `cli/src/world.rs:170` (build), `:880`, `:921`
- `plan/src/lib.rs:1694`, `:5769`, `:5821`
- `coverage/src/lib.rs:561`, `sweep/src/drive.rs:165`

**Sourcing resolution (existing, book-side)**
- `analysis/src/funcenv.rs:1195` `load_sites` — splits `.` sites into resolvable/unresolvable
  against `DefinitionTable::definitions_of_path`; target must be a `SourceLiteralPlane` literal.
- `analysis/src/funcenv.rs:1143` `sourced_definitions`; `:164` `set_loadable` keys by the CLI path
  STRING exactly.
- `cli/src/world.rs:665` `definition_table` — the ONE reader of role funcdefs; `set_loadable(path)`
  per CLI oracle path.
- `cli/src/main.rs:2232` `source_table` — oracles in CLI order, then the book LAST.
- `cli/src/main.rs:335` `resolve_oracle_paths` — CLI positional + `--oracle-dir` glob. THE seat a
  sourced-file read must extend (I/O at the edge only, `io-at-edges-only`).

`fnd-sourced-files-are-not-read-today` [+SURE]: nothing reads a file named by a `.` that was not
also named on the CLI. `definitions_of_path` matches the literal target against CLI path strings,
so today's `. ./x.oracle.sh` only resolves when `x.oracle.sh` was co-loaded. Phase 3's oracle-side
sourcing therefore needs a real CLI-edge LOADER (read, contract-check, append to the source vector,
record the edge), not merely a predicate change.

## §3a — As-built (what a successor inherits)

BUILT and green on the full corpus:

- `oracle::load_inert` admits a statically spelled top-level `.` in a marked file, as CONTRACT.
  `source` stays refused — `dash` has no such builtin, so it fails the two-binary floor.
  `item_is_static_load` is the one reader of what a file sources.
- `core::custody::CustodyClosures` — the containment relation of §2, with `singletons` as the
  no-sourcing world every tree-less lane passes (which is what keeps the corpus byte-identical).
- `dorc_cli::sourcing` — the PURE include-tree derivation both drivers call; `main.rs`'s
  `read_sourced_oracles` is its filesystem half, and it widens `(oracle_paths, oracle_srcs)` at ONE
  seam so every downstream seat inherits the sourced files without its own change.
- `HelperIndex::with_include_tree` + the re-keyed `resolve`: three suspensions — unresolved load,
  outside custody, contested-within-custody.
- Cells: three `floor30-*` ground-truth manifests; `pin28-helper-package-entrypoints-lift`
  respelled as sourcing (loads ONE file, closure reaches the apply guard);
  `pin30-swapped-entrypoints-source-the-helpers` (an admin's own entrypoints over the vendor's
  helpers). `28M` §8's commissioned property is now pinned in both directions.
- `p-x-blessed-toplevel-source` PROMOTED (XPASS) and its registry row dropped.

MEASURED FINDINGS worth a successor's attention:

- `fnd-empty-index-shortcut-shipped-bare` — `closure_for` returned an empty closure before checking
  anything when the index held no declarations, so a file whose `.` target failed to load shipped
  its verdict body BARE and the probe answered rc 127. Caught by the package cell, fixed by
  ordering the suspension ahead of the shortcut. Any future early-return in that function inherits
  the same trap.
- `fnd-corpus-runs-from-a-throwaway-cwd` — the e2e runner drives every case from a scratch sandbox
  with ABSOLUTE oracle paths. Working-directory-relative `.` resolution is therefore unexercisable
  by any corpus cell, and unusable by any admin who does not `cd` into the package first. This is
  what forced §8's `dev-sourced-paths-resolve-against-the-sourcer`.

## §4 — Build items (the ordering that was followed; the remainder is item 8)

`ord-fixtures-precede-consumers` — floor-differential cells land BEFORE the engine behaviour that
consumes them (`28Q` §8 stage-i's measure-first precedent); ordinary golden cells for new behaviour
land AFTER it (the human's lean: pin the future, never pin the hole).

1. **`b1-floor-fixtures-for-sourcing`** — the ground-truth manifests, `floor30-*` pattern
   (`floor30-helper-collision-across-frames.loom` is the template: a sentinel-manifest book,
   `expected.emitted` = the shells' own bytes, minted ONCE via `mise run bless:floor -- <case>` on
   the WSL leg, never churned). Gate-9 flattens a case dir and copies only top-level FILES, so a
   floor cell cannot nest directories — which is why nothing here measures path nesting. The three
   that landed, with what dash ∩ posh actually said:
   - `floor30-sourcing-is-transitive` → `mid` / `leaf`. A sourced file's own `.` runs while the
     sourcer loads, so the grandchild binds too. This is what licenses `custody` being transitive.
   - `floor30-diamond-source-binds-once` → `shared` / `a` / `b`. A second `.` of the same bytes
     changes nothing AND each entry keeps its own declarations — both halves, because a dedup that
     dropped an entry's names would look exactly like a pass.
   - `floor30-sourced-file-shares-one-environment` → `book` / `rebound`. A sourced file sees the
     sourcer's bindings and can rebind them. THE reason the custody fence is an engine licensure
     policy and can never be inferred from sh: there is no shell-observable sense in which two
     sourced siblings are separated.
   The sibling-fence cell first sketched here was DROPPED on measurement: sibling separation is not
   an sh fact at all, so it belongs at the unit tier (`core::custody`'s tests), not the floor.

2. **`b2-load-inert-admits-top-level-dot`** — `item_is_load_inert` admits a top-level `.`/`source`
   whose target word is statically resolvable; refusal for a contract-violating target attributes
   to the CONTRACT. Greening trigger for `p-x-blessed-toplevel-source`. Both conjuncts of that pin
   must green together — the whole-file refusal is what costs the file its own declarations.

3. **`b3-the-sourced-file-loader`** — the CLI-edge read: from each marked oracle's top-level `.`
   sites, resolve the target relative to the sourcing file's directory, read it, contract-check it
   (marked + no top-level commands), append it to the source vector, and record the edge. Failure
   to read or to satisfy the contract REFUSES with contract attribution and the closure does not
   form (the site's licensure falls back to today's answer — the safe direction).
   `--WONDER`: whether a sourced file also joins the `--oracle-dir` ambient prefix or only the
   closure. Lean: it joins the SOURCE VECTOR (so its definitions are visible exactly as sh would
   bind them after the `.`), because `rul-unsure-falls-toward-sh-parity`.

4. **`b4-custody-closure-computation`** — the include-tree → `custody(F)` relation, built at ONE
   seat from the recorded edges, threaded to `HelperIndex`. Diamond dedup keys to
   RESOLUTION-identity (bytes × what the file's own sourcing resolved to); differing bytes refuse
   through the existing `agree` path.

5. **`b5-rekey-the-custody-predicate`** — `HelperIndex::resolve`'s `chosen.file != asker` becomes
   `!custody(asker).contains(chosen.file)`; `DefinitionCustody` internals follow per §2.
   Single-closure world (today's whole corpus) stays byte-identical: with no oracle-side `.`,
   `custody(F) = {F}` and the predicate is the phase-1 one, term for term.

6. **`b6-blessing-keying-family-rooted`** — NOT BUILT, and correctly so. The keying is acked, but
   the thing it would key does not exist: `dorc_oracle::build_dialect` mints from predict-derived
   cells through a whole-unit `dialect_minting_source` fold and has no reachability or blessing
   notion at all. The blessing mechanism itself is `28Q` §9 pin 13
   `pin-blessing-reach-elevation` — UNRULED, "one typed line owed" — and it interacts with the
   verdict-word enrollment question that is out of this lane's scope. Building a key for an absent
   lock is the multi-phasic-scaffolding pattern the project's own law warns against; the keying
   lands with its consumer.

7. **`b7-repin-the-discarded-package`** — `pin28-helper-package-entrypoints-discarded.loom` re-pins
   via a thin entrypoints ORACLE that SOURCES the helpers (the spelling the ruling leaves); the
   in-book entrypoint form stays as an honest DECLINE cell. The in-book spelling is PERMANENTLY
   de-licensed — a book mints no speaker even with `.`.

8. **`b8-book-side-unwalling`** — STOPPED on a ruling gap; this is the lane's named remainder.
   The brief conditions it on landing together with `28K:res-book-ships-its-load-closure` and an
   executing e2e cell whose middle sub-case is "the artifact run from an isolated cwd". That
   sub-case is what exposes the gap.

   A book's top-level `.` SURVIVES in the artifact — the byte-floor (`two-surfaces`) keeps the
   book's own bytes, and nothing today licenses the engine to touch that line. So bundling the
   sourced declarations into the apply preamble does not make the artifact portable: the surviving
   `.` re-sources them where the tree is present (harmless — `floor30-diamond-source-binds-once`
   measures the idempotence) and FAILS where it is not, fatally under `set -e`, which the corpus's
   own books carry. The two exits are (i) the artifact keeps requiring its sourced tree, in which
   case bundling buys nothing and `res-book-ships-its-load-closure` means "ship the directory"; or
   (ii) the engine neutralises a book line it has never neutralised before, which is an
   attention-honesty and byte-floor ruling — `rul-attention-honesty` territory, and squarely the
   cross-cutting kind `inv-superposition` says a component may not settle for itself.

   Nothing was built toward either pole. The un-walling half alone would be the largest
   blast-radius change in this build (it grants downstream licences wholesale), so landing it
   without its runtime half is exactly what the brief forbids.
   `FORFEITS:forfeit-book-sourcing-walls` therefore stands, rewritten to current truth: the
   oracle-side half is captured, the book-side half names the blocking question.

9. **`b9-xfail-sweep`** — after EACH landing, re-check `p-x-blessed-toplevel-source`,
   `p-x-definition-grade-keying`, `p-x-helper-unset-f-across-files`, `p-x-regional-helper`
   (`internal-tooling/src/xfail.rs` `PINS`; the last three carry horizon `r31:closure-custody`).
   A pin that greens is promoted and its registry row removed; a pin that does not is
   RE-HORIZONED with `Horizon::Deferred{was, now, why}` — never weakened, never deleted.
   `mise run xfail:census` renders what is owed.

## §5 — Gates

- `syn-single-frame-byte-identical` / single-CLOSURE byte-identity: today's whole corpus produces
  byte-identical output except cells named in the drift enumeration (§6). This is the migration
  gate on EVERY commit.
- `empty-world-byte-identical`: no oracles loaded ⇒ byte-identical. Unconditional.
- New floor cells: measured once, byte-stable on re-mint; disagreement BETWEEN binaries is the
  floor's verdict and is never blessed away.
- Checker gates (`notes/300` §2): certifier + sparing re-derivation green over the full corpus.
- `mise run gate:quick-quiet` in the hot loop; `mise run both gate:full-quiet` + `mise run
  bless:dry` FOREGROUND at close. Windows leg FIRST (`preflight-bounds-before-spend`: the WSL
  build cache inflates `vmmemWSL` and the Windows RAM probe reads it as pressure).
- `mise run bless:floor` is WSL/*nix only (git's Windows userland has no `posh`) and rides the
  trial filter — scope it to the named case, always.

## §6 — Expected golden drift (the enumeration; anything outside it is a STOP)

- `pin28-helper-package-entrypoints-discarded.loom` — re-pinned to the sourcing spelling (b7).
- `pin28-helper-package-entrypoints-lift.loom` — the co-loaded package re-spells as a sourcing
  package; the lift returns through custody rather than co-loading.
- The new `floor30-*` cells (b1) and the new decline/e2e cells (b7/b8) — additions, not drift.
- `crates/aid/src/catalog_lock.rs` — only if a code is minted; through the promote ceremony
  (`two-bless-paths-split-by-directory`: promote FIRST, rebuild, THEN the e2e bless).

## §7 — Fences (hands off; a Spine lane runs concurrently)

`plan::records`/whylog/spine surfaces · `core::influence` · `core::spine` · `crates/aid` catalog and
arrangement beyond decline-reason needs. Out of scope with no rulings: the committee fence's
permanence (build-as-spiked stands, marked unratified) · verdict-word enrollment · any
multihost/book-to-host mapping · `LIVING_STATUS.md` (conductor's fold).

Verified-core proximity: this lane does NOT touch `core::sorted`, the lattice/solver, the
certifier, the sparing reference model, or `minispec/`. If a signature change ripples into
`spike/verify/`, that is the alarm working — report, never widen a fence's allow-list.

## §8 — Deviations recorded at authoring time

- `dev-docid-is-30c-not-30f` — the brief expected `30F`; the `Research/**/30*` glob shows `30A`
  and `30B` are the highest taken, so the lowest free letter is `30C`.
- `dev-prelude-a-was-already-landed` — `28Q` §1 item 1's file-grade tripwire retired at
  `c34e996a` (predecessor's last doc commit). Residual: the item still explains itself by contrast
  with the retired shape, which brushes plans-are-ahistorical; trimmed to plain current truth.
- `dev-custody-compare-is-asymmetric` — §2's containment shape, against `28M` bitem3's "consumers
  still only compare" letter. Zero consumer churn, so the promise holds in substance.
- `dev-sourced-paths-resolve-against-the-sourcer` — THE ONE TO REVIEW FIRST. A `.` target resolves
  against the sourcing FILE's directory, not the working directory, which diverges from POSIX and
  therefore from `rul-unsure-falls-toward-sh-parity` (human-typed, LOAD-BEARING, and it names name
  resolution explicitly). The case for it, and why it was not treated as a STOP: an oracle's
  top-level `.` NEVER EXECUTES — bodies reach the artifact by transplant and `dorc strip` erases the
  marked file's text — so there is no runtime behaviour to be at parity with, and the construct is a
  loader directive, which every loader resolves against the including file. Under the
  working-directory rule the ruled deliverable is unreachable: `fnd-corpus-runs-from-a-throwaway-cwd`
  makes it unexercisable by any corpus cell and unusable by any admin who has not `cd`-ed into the
  package. It is NOT a correctness fork — declarations only ever ship from a file the engine read
  and contract-checked, and an unresolved target suspends — so the cost of being wrong is UX, not a
  wrong elision. `dorc_cli::sourcing::resolve_against` is the single function to change, and
  `a_target_resolves_against_its_sourcers_directory` is the test that would go red.
- `dev-two-new-decline-reasons` — `ContestedWithinCustody` and `UnresolvedLoad` joined
  `VouchedCompositionReason`. Structure only; both render `[unwritten:]` and their prose is a
  conductor/human act (`error-authorship-tier`).
- `dev-diag-slug-still-says-not-load-inert` — the code `oracle-file-not-load-inert` now fires for a
  CONTRACT violation, and its slug still reads like an engine proof. Renaming is a catalog +
  defining-case + prose act, so it was left alone and is proposed upward instead.
