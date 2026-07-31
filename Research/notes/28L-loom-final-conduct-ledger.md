# 28L — loom-final conduct ledger (finish the loom arc: everything editable, or ledgered-never)

AI-authored (Fable conductor, seated 2026-07-29; worktree `.claude/worktrees/loom-final`,
branch `ai/r28-loom-final` off `ai/main` @ `833bbe0b`). Authority: root docs,
`spike/CLAUDE.md`, human-typed rulings outrank. The MAP is `_loom-final-map-DRAFT.md`
@ `f0fef317` (worktree root; folds into git history at arc close). Companions: `plans/282` ·
`plans/288` · `notes/287` · `notes/28H` · `notes/28J` (its editable-today line is FALSE —
`LIVING_STATUS` `b1ea2090` bullet outranks it).

## §0 — Human directives (typed 2026-07-29, this seat; rewind-durable; ack-ledger)

- **dir-finish-the-loom-arc** — the side-quest ends now: every piece of project prose
  editable through looms, or on an explicit law-cited never-looms ledger. No deferrals, no
  phase-later. (The `288` §8 arc-close invariant, enforced at last.)
- **dir-prose-authorship-bent-this-arc** — the no-AI-prose law is bent FOR THIS ARC where it
  unblocks completion ("there's already AI prose; being-unable-to-edit-it is ensuring
  there's AI-prose for longer"). Dogfood round-trips may LAND; scaffolded cases may carry
  provisional prose. Bulk prose-quality rewrite remains NOT the goal;
  `27V:rul-error-authorship-tier` resumes at arc close.
- **dir-boundaries-are-the-product** — the arc's center is "how do we separate user-edited
  text from structural text", demanding one CENTRAL, CONSISTENT, STRUCTURAL solution, never
  per-case edge-detection ("beside a colon… beside the letters ERR… a newline in the
  middle"). Don't take anything as written; ship the correct product; bend anything that
  doesn't damage core Dorc.
- **rul-rust-and-loom-are-the-only-edit-surfaces** (human-typed; the sharpening) — editor
  persona: experienced Rust dev, zero loom experience. Everything between the `Diag` API and
  the loom file is a black box. `rust` and `loom` are the only valid edit surfaces;
  `dorc-loom` internals, `errorloom` internals, and both locks are valid for NEITHER
  persona, ever. A Rust value becomes loom-usable via one clean API act at the mint site;
  a loom editor missing a value goes to the Dorc-side implementation as an API consumer.
- **dir-remit-fence** — the remit is NOT "all user-aid"; it is looms-as-the-correct-edit-home
  for current and future aid, incl. a user-friendly variable add/edit path. No new
  text-emitting machinery in core Dorc.
- **dir-naive-reviewer-gate** — acceptance gains a blind low-tier reviewer: loom file + remit
  only; may read the file, tool refusals, and Diag-API rustdoc; may NOT read
  dorc-loom/errorloom source or the locks; must succeed and report chafe. Runs at X1-close
  and arc-close.
- **dir-no-compat / dir-churn-now** — backwards-compatibility explicitly not a concern
  (reaffirms `rul-strawman-formats-no-compat`); cheap mechanical churn is done NOW, unphased,
  routed to cheap models.
- **dir-fold-modes** — at close: `ai/main` in-flight ⇒ bring everything to one branch tip +
  clean worktrees; quiesced ⇒ one atomic ff from `ai/main` to the tip.

## §1 — The map, landed and verified

`f0fef317` (read whole by the conductor; load-bearing mechanism claims re-verified by hand:
the `-->`-only exclusion at `dorc-loom.rs:804`; the chrome-path placeholder mint at
`aid/diag.rs:2789` vs the faced words path at `said.rs:184`; `is_foreign_param == "detail"`
at `catalog.rs:246`; branch carries exactly the map commit). Headline verdict ACCEPTED:
**the stamped-provenance model is sound; the failure class traces to two render chains for
the same case bridged by byte-shape re-detection (K1–K6), one provenance omission (K8), one
addressing weakness (K7)**. New findings accepted: `fnd-added-help-is-silently-absorbed`
(87 codes on a silent-corruption path) · `fnd-canonical-payload-forces-a-loom-edit` (the
black-box law violated for 29/94 codes, one function) · `fnd-params-arm-is-not-forced` ·
`fnd-passthrough-prose-has-no-authored-words` (16 codes) · `AmbiguousCandidate` already
biting two of `28J`'s top-10 rows. Edit-path matrix: of 59 case-owned codes — 34 editable
today, 6 reflow-blocked, 14 placeholder-blocked, 5 passthrough-blocked; 35 ratcheted codes
have zero paths by construction, none structurally unreachable (§3a triage: three named
harness additions). Arrangement registry: ~11 of 135 entries faced.

One conductor correction: the mirror-threading census (map §2d) undercounts —
`CONST_ARRANGEMENTS` is also consumed at `lint/render.rs:119` and `cli/main.rs:243`/`:414`;
lanes thread by fresh census, never by the map's count.

## §2 — Rulings (conductor, under the §0 directives; map cites in parens)

- **rul-editability-is-stamped-never-re-derived** (§0.4) — ADOPTED AS LAW: editability is
  stamped at the ONE render seat and carried byte-exactly to the committed transcript; no
  component may re-derive structure, editability, or word boundaries from rendered-byte
  SHAPES. Enforcement gate `transcript_bytes_equal_production_bytes` is mandatory in X1.
  Steering-sync (aid/CLAUDE.md + spike/CLAUDE.md) lands with the build, not before.
- **rul-wrap-inside-the-render-seat** (§0.4; resolves tc-fixed-width-diagnostics) — ADOPTED.
  Production diagnostics become fixed-width at a named const (why-surface precedent,
  `render-form-unwelded` territory). K1–K6 delete. Wrap-engine: builder proposes at lane
  start (lean: weft if the `aid → weft` dep is clean, keeping ONE wrap-judgment engine; a
  single named local wrapper acceptable otherwise). Values are never re-spaced.
- **rul-section-addressing-by-index** (§0.5; K7) — ADOPTED: index-ordered alignment over the
  component sequence (reuse errorloom's LCS aligner), unique-consistent-assignment or
  refuse. Generic transport work; lands in errorloom.
- **rul-render-context-struct** (resolves tc-lookup-parameter-vs-render-context) — a
  `RenderCtx` carrying the arrangement lookup AND the width; no thread-locals.
- **rul-placeholder-wears-the-register-face** (§2a; K8) — ADOPTED per `28H` span ruling 4
  transplanted to the catalog seat: placeholder text stays computed, its span carries
  `(code, Field::Message, instance)`. Absorption→refusal is MANDATORY (the message section
  ends at the render's own message-run boundary). Production render bytes byte-identical.
- **rul-help-affordance-is-scaffold** (resolves tc-help-register-affordance) —
  h-scaffold-affordance: an explicit register-adding affordance seeds `help`, then the
  ordinary loop edits it; the absorption refusal names that affordance verbatim.
  h-accept-a-typed-help-line REJECTED (byte-shape detection); h-render-always REJECTED
  (ships placeholder debt to production).
- **rul-attached-markers-land** (§5; resolves tc-attached-marker-amends-the-grammar-rule) —
  ADOPTED, X1: `282:rul-double-brace-template-only` is AMENDED — the marker token is
  unchanged (`{{name}}`, whole, no interior whitespace/expressions); adjacency to
  punctuation/backticks becomes legal. Re-holing stays anchor-gated. `plans/282` §13
  rewritten in place at steering-sync. Grounds: 26/94 messages backtick-quote a variable —
  the refusal contradicts the house idiom.
- **rul-honest-firing-retires-stand-ins** (resolves tc-fixture-world-source) — shape (b):
  harness widening (parse/CFG diag union + records intake in X1; oracle-loading in X2)
  retires `canonical_payload`'s 29 hand-built payloads and `covered()`'s 23. Irreducible
  residue moves beside the payload types in `aid` (a Rust surface), never in `dorc-loom`;
  shape (a) is the sanctioned fallback ONLY for the oracle-loading 7 if X2 runs long.
- **rul-params-arm-forced-by-destructuring** (resolves
  tc-payload-param-binding-wants-a-macro) — the macro ban STANDS. First shape tried:
  exhaustive struct-destructuring (no `..`) in `params_of_raw` arms, so a new payload field
  is a compile error at the params seat, structurally, macro-free. The lexical census gate
  is the backstop where destructuring can't reach. Builder verifies feasibility and reports.
- **rul-in-file-loop-hint-minted** (resolves tc-in-file-loop-hint-is-frontmatter) — mint the
  closed frontmatter key; generator-owned text; a hygiene gate reads and verifies it (the
  closed-vocabulary law holds: the key IS read).
- **rul-variable-surface-is-block-plus-sections** (resolves
  tc-variable-surface-is-a-command-or-a-file) — the committed `vars --used` block
  (`282:rul-used-inventory-is-committed`, finally unblocked) + a `dorc-loom sections CASE`
  command (census + affordance + refusal-debugger) + `loom:vars`/`loom:scaffold` mise
  tasks. No further browse surface this arc.
- **rul-friction-batch-verdicts** (§4) — accepted as mapped: all GO except
  breadth-vs-first-failure (banked r30); AmbiguousCandidate PROMOTED into the boundary-weld
  lane; trailing-`\n` = TRIM at read-in, in the one named significance seat.
- **rul-full-driver-this-arc** — reconfirmed (X2 opener); `28H:ask-full-driver-this-arc-or-r30`
  is answered THIS ARC; the `288` §8 invariant is unreachable without it.
- **rul-refusals-name-the-next-command** (§8) — adopted arc-wide: every refusal on the
  authoring path ends with the exact command or edit that resolves it;
  `282:rul-internal-tool-sharp-edges` permits blunt, not unactionable. `UnknownParam` names
  the Rust path verbatim (map §8.3 item 4).

## §3 — Execute cut (X1 as four dispatches; X2 after)

- **D1 x1-boundary-weld** (Opus, this shared worktree, FIRST AND ALONE — moves goldens
  corpus-wide): wrap-in-seat + K1–K6 deletion + K7 index addressing + the
  byte-identity enforcement gate + the whitespace-family sweep. Empty-diff obligations:
  both locks; every e2e artifact `.sh` (the byte floor). Scoped bless in-lane sanctioned;
  conductor inspects the full golden diff at fold.
- **D2 editable-surface pack** (Opus, own worktree off D1): placeholder+help (§2a) ·
  attached markers (§5) · mirror-thread via RenderCtx (§2d, census-corrected) ·
  `editable_baseline`-via-`replay` seat collapse (§2c's judgment half).
- **D3 reach pack** (Opus, own worktree off D1, ∥ D2): rust-surface-weld (§2f: stand-in
  retirement per ruling, params destructuring, `UnknownParam`/rustdoc repairs) · harness
  widening (parse/CFG diags + records intake) · the 14 scaffold-and-author ratchet codes ·
  cheap lint/cli faces.
- **D4 churn pack** (Sonnet, after D2+D3 fold): tooling frictions (mise tasks, verbose
  output, trailing-trim, arity refusal, blast-radius gate scoping, deferred-marker
  visibility) · the 59-case `vars --used` regeneration · in-file loop hint · rustdoc
  mechanical repairs.
- **X1 gate**: naive-reviewer run (both remits) + conductor rehearsal + `mise run both
  gate:full-quiet`.
- **X2**: x2-full-why-driver (opener) · x2-reach-hard (oracle-loading 7) ·
  x2-reason-opener (`289:finding-reason-opener-still-hardcoded`) · x2-de-passthrough
  (PENDING the §4 scope answer) · ledger ratification + steering-sync + doc repairs
  (28J correction; spike/CLAUDE.md build-stands refresh; LIVING_STATUS; TODO-ADDTL).

## §4 — Open with the human

- **ask-de-passthrough-lane-ownership** — the 16 pure `sm {{detail}}` codes can never be
  loom-edited until emit sites stop composing prose into `detail`
  (`282:rul-passthrough-type-gated`). The prior owner is cited in `288` §6 as "the opaque
  sibling lane, `284`" — an ID absent from the visible tree (likely fenced). Conductor
  default if unanswered: pull x2-de-passthrough into this arc. If the fenced lane owns it,
  say so and it becomes a ledger row (blocked-on: that lane) instead.

## §5 — Landings (accretes)

- **amendment-width-is-a-render-parameter** (human-typed 2026-07-29) — production output
  must wrap to window width; hard-fixed production width is unacceptable for weft
  surfaces. `rul-wrap-inside-the-render-seat` is AMENDED: the seat takes width as an
  input; ONE canonical const governs only the committed transcripts (and the
  deterministic no-TTY fallback); the enforcement gate pins transcript == render AT the
  canonical width; terminal-width detection is an I/O-edge concern, never in aid/kernel.
- **rul-diagnostic-surface-renders-through-weft** (conductor, 2026-07-29, after
  first-hand absorption of `weft/CLAUDE.md` + `tree.rs`/`measure.rs`/`provenance.rs`/
  `aid/weave.rs`; supersedes the D1 brief's wrap-engine latitude — the human gated the
  restart on this understanding) — the boundary weld routes the diagnostic surface
  through a weft `Document` via the sanctioned `aid::weave` adapter, exactly as the why
  surface: message/help prose as Template-keyed runs (`(code, field, instance)`), params
  as Param-keyed runs, caret/source frames as `CodeBlock` nodes (gutters become weft
  geometry — `is_caret_gutter` dies as a consequence, not by deletion alone),
  severity/connective chrome stays computed (`Face::Part`-keyed; NO new registry rows in
  the weld lane). The `weave` bridge grows the Template facet arm
  (`facet_of` currently drops `Provenance::Template` to `WEFT_UNKEYED`). Grounds:
  `weft-pure-layout` is already (tree, width); `weft-total-cover-spans` is already the
  transport's attribution authority; one wrap-judgment engine
  (`28H:ask-shared-lexical-rulebook` discipline); the why-surface precedent (the map's
  §0.4 argument 1, now verified at source level).
- **macro-soft-tune banked** (human-typed 2026-07-29) — hold the no-macro line; no
  builder tuned toward macros; end-of-arc chafe checkpoint reads the reviewer-gate +
  rehearsal reports against the human's ~90-hold/~10-sugar-macro lean; the question
  surfaces to the human only if the macro-free shapes genuinely chafed. Their call.
- **ask-de-passthrough-lane-ownership RESOLVED** — `284` located: it IS behind the fence
  (`28A`:664 records the hand-off written to `quarantine-DO-NOT-READ/284`; the human's
  recollection differed). Human ruling: the work is ours. x2-de-passthrough is IN,
  designed from `plans/282` §8's core (sealed user-sourced-text type; `detail` payload
  audit; world-variant siblings); the human may optionally skim the fenced notes and
  relay anything load-bearing. Conductor process note: a broad `Research/` grep brushed
  two content lines from fenced files before the dir was excluded — minimal exposure,
  unused; future sweeps carry an explicit quarantine exclusion.
- **D1 restart** — the first D1 dispatch was stopped by the human before the weft
  absorption (zero commits; tree clean at `599e90ed`); relaunched with the weft-directed
  brief at this commit.
- **rul-weft-is-engine-not-law** (human-typed unweld, 2026-07-29; amends
  rul-diagnostic-surface-renders-through-weft ABOVE from weld to lean) — the LAW is
  engine-agnostic: a render seat must emit a byte-exact, total-cover, stamped part
  stream, and nothing may re-derive structure from byte shapes; the enforcement gate
  compares seat-vs-transcript whoever rendered. Weft is the design-priority engine (and
  D1's), never a transport requirement; a non-weft seat honoring the stamped-stream
  contract stays legal. Support both paths; prioritize weft for design attention.
- **rul-quarantine-binds-conductor-only** (human-typed, 2026-07-29) — the
  quarantine-DO-NOT-READ fence binds the FABLE CONDUCTOR, not builders or scouts. Every
  builder brief carries: mandatory read of `quarantine-DO-NOT-READ/
  AGENTS.for-builders-only.md` (that file only, unless it directs otherwise), follow it,
  NEVER report/quote its contents to the conductor; a material brief-conflict is
  reported only as "builders-only conflict — human adjudicates". String-transport
  builders (D2; the X2 de-passthrough lane) additionally read `284`. Exception, flagged
  unvetoed: the blind naive-reviewer gate reads neither (its blindness is the point).
- **fnd-284-landed-state** (quarantine scout, 2026-07-29; public citations only) — the
  284 hand-off is mostly UNBUILT: the type-gate is open (`is_foreign_param` is
  `param == "detail"`, aid/catalog.rs:246, self-documented "LATER"); the
  foreign-vs-our-words audit never started (23 `detail` sites, grown from 18); the
  2026-07-26 `display.rs` cluster landed the TERMINAL-SAFETY half only (encode_line/
  encode_foreign through params_of, tested). Sharp extra: `Said::Foreign` /
  `RenderPart::ForeignText` carry text in enum-variant fields — effectively public, so
  a string literal constructs "foreign" today; the seal needs a room-pattern inner type
  (`core/room.rs` precedent). Charterable entirely from public docs: `282` §0+§8 ·
  `288` §6 · `28A` §2m/§2r/§2s · AID-NEEDS rows · room.rs. Human ack (2026-07-29):
  whylog stays out-of-scope EXCEPT the mechanical type-enforcement ripple of the
  284-shaped work — no redaction machinery, no whylog redesign, no adapter layers;
  secret-taint sanitization stays with its own (unowned) row.
- **D1 LANDED + conductor-verified** — `3434ba1f`…`19f491fb` (8 commits). The
  diagnostic surface composes a weft `Banner`/`CodeBlock`/`LabeledRow` document through
  `aid::weave`; `render_cli` IS the part stream's `.text()` (string seat and parts seat
  cannot disagree); K1–K6 deleted with ZERO byte-shape survivors on the render/transport
  path; K7 landed as `errorloom/src/address.rs` (component-sequence alignment);
  `transcript_bytes_equal_production_bytes` live in the looms runner; width is a seat
  parameter (canonical const for transcripts). Bridge extension for D2: `Face::{Code,
  Register{field,paragraph}, Hole{field,param}}`, Template/Param facet arms,
  `weave::to_runs` inverse; the placeholder seam is ONE line (diag.rs:2764 still mints
  `Arrangement{"unwritten-placeholder"}`). Obligations held: locks byte-identical ·
  artifact `.sh`/`.out`/`.ran` under cli/tests untouched · 47 loom transcripts moved,
  transcript-body lines only · conductor own-hand `gate:full-quiet` GREEN at `19f491fb`
  (cold clippy; builder additionally reports both legs green). Form changes ride
  render-form-unwelded: flush-left continuations · weft `[ ` locus supersedes `-->` ·
  weft-measured gutters SUPERSEDE the `28A` §12 gutter aesthetic (flagged to the human;
  a weft geometry tune can revisit) · uniform trailing newline (D4's trim ruling gets
  one shape to bind). Residues routed: txtar-header-lookalike container escape → D2 ·
  wrap-inside-a-value → LEDGER (wants a weft run-level unbreakable mark; weft vocabulary
  change, not this arc) · zero-width-run inversion → `28H` ruling 1's seat-defect, take
  in an X-lane only if cheap · wrapped placeholder rendering → D2's face work.
  tc-rulings: `tc-catalog-values-are-unencoded-for-a-measured-surface` → DEFER to X2
  type-gating (D1's refusal to glyph-mangle 16 codes was correct);
  `layout_box(width, indent)` → absorbed by D2's `RenderCtx`. Housekeeping: D1
  relabelled its own commits (dsn misuse); `relabel-backup` branch awaits the human's
  hook-reserved delete.
- **D3 LANDED** (`ai/r28-loom-d3` @ `6160cf25`; both legs green own-hand at its tip,
  Win 1620 / WSL 1616; awaiting fold AFTER D2). The Rust-surface weld is TRUE:
  the 29 dorc-loom stand-ins + `covered()`'s 23 hand-builds collapsed into ONE
  fixture table at `aid/src/fixture.rs` (lexically fenced out of production;
  `whylog_diagnostic` correctly tried before the payload floor — the lock fixpoint
  caught the ordering); the §2f.2 re-walk now measures a rust-surface-ONLY edit set
  with the params arm COMPILE-FORCED (E0027) — all 94 `params_of_raw` arms
  destructure exhaustively, NO census backstop needed, macro line held; refusals
  explain themselves via `dorc-loom/src/refusal.rs` (`UnknownParam` names the
  vars command + the payload-struct path verbatim; `ArrangementValueSequenceChanged`
  names the editable words); rustdoc teaches the loom-naive Rust dev (slug→payload
  pointer; the spanless gate's path + lexical rule); ratchet 35 → 30 (5 honest-firing
  cases; 4 `example` fields re-derived, everything else byte-identical);
  `lint-clean-sentence` faced and proved editable. Tooling finds fixed in-lane:
  `MAX_RECEIPT_CASES` 64 → 512 (the loop would have hard-blocked at case 65) and
  `loom:compile`/`loom:promote` trailing-args were SILENTLY UNSCOPED (list went to
  the trailing `git diff`; promote ran whole-corpus) — both tasks now template the
  list. Byte floor held; artifact plane untouched; comment budget net −5.
- **fnd-three-map-corrections** (D3, all +SURE, measured) — (1) the 8 `records-*`
  codes + `records-fact-truncated` have NO production emitter (`records::deframe` is
  called only under `#[cfg(test)]`; production intake is `read_host_evidence` +
  `admit_unscoped_host_records`, emitting only `host-evidence-admission-refused`);
  (2) `corroborate_tolerance_over_identity` + `hint_heavy_context_no_vouch` have zero
  callers — their two codes are not lint-fireable; (3) the five "effect-plane,
  fireable today" codes need ORACLE-LOADING (X2), and `reserved-namespace-squat`
  fires only via `dorc plan`'s book lint, which no harness route runs.
- **rul-no-emitter-codes-are-blocked-rows** (conductor) — a catalog code whose only
  emitter is test-only/uncalled CANNOT have an honest defining case; forcing one
  would fake the firing. Disposition: the 9 records codes → ledger row
  "blocked-on-emitter-decision", FLAGGED TO THE HUMAN with the lean: if
  `records::deframe` is superseded residue of the r26 transport re-cut, DELETE the
  codes under no-compat; if the `262` records lane is still the intended intake, the
  wiring belongs to the records/r26-revival arc, not this one. The 2 tolerance-lint
  codes → X2 wires the two uncalled detectors into the validate path (kWARN-rich
  sanctioned; oracle-lane adjacent) and cases follow. Ratchet residue 30 accounted:
  ~6 lint-fireable-with-trigger-notes (mechanical; queued D4/X2) · 5 effect-plane +
  7 oracle-loading + 1 book-lint-route (X2) · 9 records (blocked) · 2 tolerance (X2).
- **Blocked-face verdicts** (D3 evidence) — `lint-fidelity-approximate/-raw`: only
  mintable via the real-tools lane, which the sanctioned-executor law bars from loom
  cases ⇒ LEDGER (lock-tier, law-cited). `lint-source-*` (8): the deliberate
  plain-text seat; facing = moving the stderr-envelope render into the lib parts
  stream ⇒ rides X2's driver extraction alongside `cli-usage-synopsis` + the
  plan-stderr chrome trio (D3 correctly stopped — head-on collision with D2's
  RenderCtx seats).
- **D2 LANDED** (`ai/r28-loom-d2` @ `ea7dd76c`; both legs green own-hand foreground,
  Win 1623/1 skipped · WSL 1619/1 skipped, cold clippy; folds FIRST). All six items:
  the placeholder wears its register's face (ONE line: `message_parts`' `None` arm
  mints `TemplateLiteral{code, Field::Message, ¶0}`; production bytes unchanged for
  all 14, zero re-bless, gate green throughout) · attached markers LEGAL
  (`AttachedMarker` + adjacency machinery deleted, net −24; the `282` §13 amendment
  doc-edit is the conductor's at steering-sync) · `HelpRegister{Absent, Unwritten,
  Written}` tri-state replaces `help: Option` · `dorc-loom add-register CASE help`
  affordance + `AddedLine` refusal naming it verbatim · `RenderCtx` threaded by fresh
  census (36 `main.rs` seats via one named production fn — accepted under
  `lib-target-is-a-loom-seam`: no main.rs seat can ever legally receive a mirror) ·
  `editable_baseline`-via-`replay` + shared `vars_inventory` (the 59-case committed
  inventory regen is UNBLOCKED) · txtar-header typed refusal at the render seat.
  Smoke: `add-register` proven end-to-end on `whylog-unwritten` and reverted — the
  committed dogfood landing stays the conductor's rehearsal.
- **Conductor ratifications on the D2 flags** — `tc-render-ctx-carries-the-catalog-too`
  → RATIFIED as an amendment to rul-render-context-struct: `RenderCtx{catalog,
  arrangements, frame}` — the mirror overrides BOTH tables or the author sees half
  their edit; unrepresentable beats remembered · `tc-help-placeholder-suffix` →
  RATIFIED: `[unwritten: <slug>.help]` (two identical placeholders in one render are
  ambiguous for author and alignment; spelling rides render-form-unwelded) ·
  **rul-a-laid-out-section-owns-no-line-breaks** (D2-proposed, conductor-RATIFIED as
  an instance of rul-editability-is-stamped-never-re-derived): a section's edited
  PROSE line-break count (Text fragments only; values belong to neither side) may not
  exceed what the render stamped inside that section — both counts from stamped
  provenance, never byte shapes; whole-PAGE sections exempt BY TYPE (their blank
  lines are authored, `28H` ruling 7). Consistency check done: v1's
  words-and-paragraphs model already excludes paragraph add/remove (`287` §8), so
  refusing added breaks forecloses nothing v1 grants; the map's own line-terminator
  proposal was measured broken post-weld (the render already ends in a stamped
  newline; no addressing-layer fence exists that spares wrapped registers) ·
  `tc-vars-drives-every-block-to-use-one` → accepted as-built; recursion is
  mechanism-bounded, real guard owed only if inventories ever nest.
- **tc-catalog-prose-is-not-normalized-at-read-in** (D2 flag) → ROUTED TO D4: catalog
  registers must store the `282` §3 normalized form (whitespace runs → one space
  within a paragraph) at compile read-in, in dorc-loom's ONE significance seat,
  beside the trailing-`\n` trim. Interaction spec: normalization runs BEFORE the
  AddedLine count, so a re-wrapped register normalizes to ≤ stamped breaks (the rule
  can only relax, never trip); a genuinely-added paragraph break still refuses (v1).
  Without this, wrapped-register edits store literal `\n` — width-coupled templates.
- **D4 queue additions** (from both lanes): `loom:add-register` task ·
  catalog-register read-in normalization (above) · the 59-case `vars --used`
  regeneration (now unblocked) · the ~6 remaining lint-fireable ratchet cases
  (trigger notes in hand). Worktree note: git-bash `mise` cannot see its trust
  entries in these worktrees (`\\?\C:\` vs `C:\` paths) — invoke `mise` from
  PowerShell; WSL needs one `mise trust`.
- **D3 REBASED + FOLDED** — `ai/r28-loom-final` @ `5eefc8b1` (ff; lane worktrees
  loom-d2/loom-d3 removed, merged branches deleted). Five collisions, all resolved as
  the union of both lanes' RULINGS: the 94 destructured `params_of_raw` arms under
  D2's `RenderCtx` signature (one arm textually conflicted; E0027 forcing intact) ·
  ONE refusal surface (`dorc-loom/src/refusal.rs`; D2's duplicate `explain` in
  edit.rs deleted; variants unioned; the `AttachedMarker` variant deleted outright —
  markers are legal now) · consumer bodies genuinely disjoint, whylog-before-the-
  payload-floor ordering proven surviving by the lock fixpoint · the LOCK never
  hand-merged: regenerated at the merged state, byte-identity fixpoints green,
  unscoped promote reports corpus-at-fixpoint (only possible via the 512 cap).
  **fnd-d2-arrangement-page-layout-panic** (real latent defect, D2's, caught by D3's
  first value-bearing chrome-line case): `editable_baseline`'s generation-lag guard
  LAID OUT `arrangement_page(slug)` — zero values into a 5-word sentence seat ⇒
  debug panic; fixed by splitting `arrangement_row` (existence check, no layout).
  NOTE for X2's chrome-face work: every `lint-source-*`/plan-stderr row is
  value-bearing; this split is what makes their cases possible. Reconciled
  `UnknownParam` text names the vars command + `params_of_raw` + the
  now-true compile-error fact. Gates at the tip: both legs green TWICE (Win 1631 /
  WSL 1627, 1 skipped each); byte floor held across the whole range. Spent backup
  branches awaiting the human's force-delete: `relabel-backup` ·
  `d3-prerebase-backup`.
- **Routing adjustment** — the ~7 remaining lint-fireable ratchet cases (firing-world
  authoring needs dialect judgment) move D4 → X2; D4 stays purely mechanical.
- **D4 dispatched** (Sonnet churn, worktree loom-d4): `loom:scaffold`/`loom:vars`/
  `loom:add-register` tasks (scoped-args template) + unswallowed compile output ·
  `dorc-loom sections CASE` (the census/affordance/debug command;
  rul-variable-surface-is-block-plus-sections' missing half) · catalog-register
  read-in normalization + trailing-`\n` trim in the ONE significance seat (spec: runs
  BEFORE the AddedLine count; paragraph breaks preserved; idempotent/fixpoint-stable)
  · the committed `vars --used` regeneration across every code-owned case (the
  sanctioned DORC_LOOM_DUMP flow — promote refuses structure changes by design) ·
  blast-radius dirty-gate scoping · arrangement arity compile-time refusal (rides
  D3's `arrangement_row` split) · `[deferred to e2e]` trial naming.
- **X2a LANDED** (`ai/r28-loom-x2a` @ `32a0887f`, base `77b2295c`; both legs green
  own-hand foreground, Win 1638 / WSL 1634; folds AFTER D4). The why-report assembly
  extracted whole: `cli/src/why.rs` (lib module; `WhyReport<'a>` 17-field Copy ctx;
  ~45 helpers private) + `cli/src/world.rs` (`WhyWorld::analyze` = the binary's own
  call sequence, handing out reports); main.rs 10389 → 7570; I/O edges stayed.
  `dorc why [<addr>]` drives in-process on BOTH chains through `live_why_parts` —
  and `transcript_bytes_equal_production_bytes` CAUGHT a real divergence on first
  contact (a trailing blank the container trims; production `dorc why <addr>` lost
  one trailing blank line, zero golden churn). Faces 28 → 67 of ~125; the edit loop
  proven on a why row end-to-end. `lint-source-*` (8) faced as a weft table.
  Reason-opener: two instances were already registry rows (map §6 stale); the live
  third (`CommandName::describe`'s resolved-dynamic sentence) migrated as
  `why-command-name` 0/1 (verbatim seed, fixpoint green); `CommandName::Literal`
  deliberately unregistered (the world's own word). Locks moved only by the 2-row
  hand-seed. Artifact floor untouched; zero blesses.
- **fnd-wrapped-rows-are-chunk-editable** (X2a, +SURE, measured) — a registry row
  wrapping at the canonical width renders as one editable section PER CHUNK (the
  layout newline closes sections despite the bridge's absorption rule — mechanism
  unconfirmed at altitude), so long rows are partially editable and a reword that
  moves the break point refuses with the ADD-A-HELP-LINE text — a
  rul-refusals-name-the-next-command MISS on top of a transport gap. The banked
  "wrap-inside-a-value → ledger" residue now bites real faces; PROMOTED from ledger
  to build: the x2d lane owns section-per-chunk repair + a correct refusal.
- **rul-ownership-declaration-adopted** (conductor; resolves the deferred
  `28H:tc-one-transcript-many-rows-ownership`, which X2a showed is now THE gating
  item for the arc-close invariant — faces exist but each case OWNS only its
  namesake row, so ~35 faced-but-foreign rows have no editable HOME short of ~35
  near-duplicate cases) — a case may DECLARE the row-set it owns in a closed
  frontmatter key; `is_case_owned`/`authored_words_are_case_owned` re-key to the
  declaration (filename stays the default singleton); one row, one owner,
  conflicts refuse loudly. x2d builds it.
- **fnd-case-frontmatter-overwrites-lock-metadata** (X2a, +SURE) — a new
  arrangement case's `when-used`/`why` silently REPLACES committed lock metadata at
  promote (X2a's own draft degraded five rows; the fixpoint caught it), and
  frontmatter scalars store surrounding quotes verbatim. x2d: absent-means-keep +
  a metadata-regression guard + quote normalization at the frontmatter seat.
- **tc-harness-records-vs-controller-scope** (X2a, flagged; conductor design
  guidance issued, X2b builds behind a CHECKPOINT) — ~35 why rows (the
  survival/guard/measured families) are un-faceable without probe RECORDS in the
  loom harness's world; the scope types are deliberately binary-private
  (rul-attribution-is-controller-minted). Guidance: dorc-loom IS the controller of
  its own in-process runs, and fixture records carry the FULL framed form
  (`28H:rul-fixture-records-enriched-not-reduced`), so the shape to propose is the
  REAL framed admission driven by dorc-loom-minted scope over fixture records —
  never an unframed side-door, never widening production scope visibility.
  X2b proposes the exact seam and STOPS for conductor review before building it.
- **Rulings on X2a's stopped items** — `cli-usage-synopsis`: DO IT in x2d (one lib
  parts seat: prefix + body + synopsis; the ~25-transcript re-bless is sanctioned
  churn) · plan-stderr trio: route RULED — mint ONE new closed frontmatter key
  (spelling latitude; e.g. `envelope: stderr`) that opts a plan-route replay into
  rendering the full stderr envelope; existing cases untouched; x2d builds it ·
  `TopCause::describe`'s 7 DECIDE-plane phrases: the enum→slug-map-in-aid shape
  (remediation_hint_slug precedent), routed to x2c's prose-site audit.
- **Revised X2 cut** — X2b (post-D4-fold): oracle-loading widening + the ~7
  lint-fireable cases + tolerance-detector wiring + the records-admission seam
  (checkpointed). x2c (parallel, post-folds): de-passthrough (reads 284) +
  TopCause migration. x2d (parallel, post-X2a-fold): ownership-declaration key ·
  wrap-chunk sections + refusal repair · metadata guard · usage-synopsis ·
  stderr-envelope key. Blind-reviewer round 1 + the conductor rehearsal fire at
  the post-D4 fold state (X1 surfaces are complete there).
- **D4 LANDED + FOLDED** (`ai/r28-loom-final` @ `621c6030`; both legs green at its
  tip, Win 1636 / WSL 1632). Landed: `loom:scaffold`/`vars`/`add-register`/`sections`
  tasks with `raw = true` (the timed-output swallow was mise-level; refusals now
  print) · `dorc-loom sections CASE` (the census/affordance command) ·
  register read-in normalization + trailing trim in the ONE significance seat
  (`edit.rs` beside `prose_line_breaks`; runs before the AddedLine count per spec;
  zero corpus movement; one pre-existing test legitimately re-fixtured to a real
  paragraph break) · blast-radius dirty gate (and closed a pre-existing gap: the
  arrangement lock had NO clean-check at all) · arity refusal via `catch_unwind`
  at dorc-loom's replay boundary — JUDGMENT ACCEPTED: `when_used` is free-form
  prose, statically parsing it would breach `inv-referent-agnostic`; the render
  seat's own debug-assert is the only true arity oracle; production untouched;
  the wrapper dies the day aid grows a fallible words-arity query ·
  `[deferred to e2e]` trial naming. VOCAB (human-typed): registry entries are
  **prose-components** now, not "rows"; ownership = filename ALWAYS contributes
  its implicit entry when it matches a slug, `owns:` needed only in
  multi-component homes.
- **rul-committed-inventory-retired** (conductor; SUPERSEDES
  `282:rul-used-inventory-is-committed`; PENDING HUMAN VETO — flagged in-chat) —
  D4 traced the committed `vars --used` block to a real architectural blocker:
  the fixpoint chain resolves replay inputs from a case's txtar SECTIONS, and a
  case cannot contain itself; landing it needs a bespoke self-reference rule in
  both chains. Meanwhile the block duplicates derivable data into ~65 files and
  re-churns every one of them on every future prose edit, forever — and the
  discoverability it bought is now served by `dorc-loom sections`/`vars` (landed)
  plus the in-file loop-hint key. RETIRED; the loop-hint's text includes the
  `loom:vars` invocation. EMPIRICAL CHECK: blind-reviewer round 1 probes
  variable discoverability with no committed inventory — if the naive reviewer
  cannot find their variables, this ruling reverses and the self-reference rule
  gets built instead.
- **assignment-catch** — `rul-in-file-loop-hint-minted` fell between the D2/D4
  briefs; x2d owns it (closed generated frontmatter key; names compile/promote
  AND the vars invocation; hygiene-gate-read so the closed-vocabulary law holds).
- **comment-clamp recalibrated** (D4 flagged the tension honestly) — the numeric
  budget binds NON-DOC inline comments only; doc-comments on new public items and
  test-rationale doc-comments follow crate law. Future briefs say so.
- **Board after the D4 fold** — X2a rebasing over it (agent resumed); X2b
  dispatched (oracle-loading widening + tolerance-detector wiring + the ~7
  lint-fireable cases + the records-admission seam PROPOSAL, checkpointed —
  builds nothing on that seam before conductor review); blind-reviewer round 1
  fired at this tip (X1 surfaces complete): three remits — overtype a
  placeholder, revise words + move/insert a variable, the full add-a-value-from-
  Rust story — allowed surfaces per rul-rust-and-loom-are-the-only-edit-surfaces
  (loom file, tool output/refusals, aid-crate rustdoc, mise task list),
  forbidden: dorc-loom/errorloom source, both locks, Research/, every
  CLAUDE.md/AGENTS.md, the quarantine.
- **X2a FOLDED** — `ai/r28-loom-final` @ `271d7921` (ff; zero rebase conflicts;
  every D4 interaction hand-verified by the lane — `catch_arity_panic` correctly
  encloses the new why path; the two normalization seats stay separate by field;
  both lock fixpoints + the byte gate re-proved across all 104 committed looms).
  Gate at the tip: Win 1643 / WSL 1639, 1 skipped each. Conductor's own-hand gate
  queued. Fresh-worktree note: WSL leg dies pre-compile without one `mise trust`.
- **x2d DISPATCHED** (worktree loom-x2d, off `271d7921`) — the loom-machinery
  closure lane: ownership-declaration key (filename-implicit + `owns:` only for
  multi-component homes; one component one owner, conflicts refuse naming both
  files) · wrap-chunk DIAGNOSIS then central fix (one section per prose-component
  per render; a reword may move the wrap point; the mis-directed add-register
  refusal replaced with the true next step) · metadata-regression guard
  (absent-means-keep; present-and-different on an existing component gates
  loudly; frontmatter quote normalization) · the in-file loop-hint key (names
  compile/promote + the loom:vars invocation) · the invocation-error parts seat
  (prefix + body + synopsis; ~25-transcript re-bless sanctioned;
  `cli-usage-synopsis` faced) · the `envelope:`-style closed key + one case
  facing the plan-stderr trio.
- **BLIND-REVIEWER ROUND 1 LANDED** (branch `ai/r28-loom-review1` @ `15cad37b`,
  throwaway — NEVER folds, contains probe prose; force-delete at close). 3/3 remits
  mechanically completed, 2/3 fully green end-to-end; the Rust-side escort verdict:
  the `DiagCode` rustdoc is "genuinely excellent", "the compiler names every site"
  HELD PRECISELY (one E0063, at the fixture, as designed). Fence caveat, disclosed
  honestly by the reviewer: the harness auto-injects CLAUDE.mds on directory touch,
  so the reading fence is porous in this harness — the reviewer distinguished
  tool-taught from leak-taught knowledge throughout; round 2 briefs accept this and
  demand the same bookkeeping. EMPIRICAL VERDICTS on the two open questions:
  `rul-committed-inventory-retired` HOLDS (`loom:vars` sufficed for variable
  discovery; the real gap was `{{name}}`-syntax teaching — text-level, routed) ·
  macro-chafe: NONE on the Rust side (supports the human's ~90 hold-the-line).
- **fnd-shared-fixture-collision** (reviewer, +SURE, reproduced via the full gate) —
  `overtype_placeholder_mints_words` borrows the LIVE `whylog-unwritten` case as its
  still-unwritten fixture: the sanctioned prose burn-down breaks `gate:full-quiet`
  while `test:looms` stays green, and the repair is invisible behind the loom-crate
  boundary. A D2 test-design bug (live-case precondition instead of a synthesized
  temp fixture). x2d item 7 fixes it and sweeps the suite for siblings.
- **reviewer-chafe-pack ROUTED to x2d** (items 8–13): CASE-arg resolution (bare
  slugs; both bases; refusal lists what it tried; real `--help`) · silent
  variable-to-literal degradation gains a loud compile-preview DISCLOSURE (omission
  stays legal per `282` §13) · `{{name}}`-syntax teaching in three texts (loop-hint,
  vars header, usage) · lock-not-clean refusal gains its remedy · `loom:sections`
  semantics pinned and self-described · texts written for a reader with NO doc
  access.
- **X2b LANDED** (`ai/r28-loom-x2b` @ `344f3c1b`, base `79032b57`; both legs green
  own-hand, Win 1654 / WSL 1650; rebases over the X2a fold next). Oracle-loading in
  the harness: `fire_book_analysis` runs the binary's whole book-side stage sequence
  in the binary's own order, oracle sections thread both chains, and a
  caret-source bug was caught and fixed in-lane (both replay seats were discarding
  `world_of_source`'s returned source — oracle-side carets pointed into the book).
  Second clean lib extraction: `cli/src/kinds.rs` (resolver/reaches machinery,
  verbatim, one implementation). Tolerance detectors wired at `validate` (reaches
  both consumers; zero corpus churn — NO existing fixture reads identity, so the
  corpus lacks any safe-across exemplar, noted for the stdlib era). Two MORE wrong
  trigger notes found + fixed (dropped `derive_lend_map` diags; carry diags gated
  behind wrapper presence) — the map's "trivially-fireable 11" was wrong for 3.
  Sixteen cases minted (`sm `-carried registers). Ratchet 30 → 12: 8 records
  (blocked-on-emitter, human) + 4 survival-lane (`footprint-incoherent` ·
  `touches-escalated` · `deriv-family-incomplete` · `wrapped-site-adoption-hint` —
  need the flag-gated survival/derivation lane extraction; correctly judged
  rebase-hostile while X2a moved 3000 lines of the same file).
- **rul-records-seam-approved** (conductor; settles
  tc-harness-records-vs-controller-scope) — X2b's `_x2b-records-seam-PROPOSAL.md`
  reading is ACCEPTED: a loom run mints exactly ONE scope; what appears is a second
  CONTROLLER (dorc-loom, of its own hermetic in-process run), not a second scope in
  one world — the law's re-entry triggers (transport, concurrency, retry,
  cross-host reuse, saved approval) are production multi-scope phenomena and none
  becomes representable. The `Framing::spike` precedent is the ONE named
  substitution point and the proposal builds on it: `admit_fixture_records(...)`
  takes no identity/host/nonce/attempt, fields stay private, no field-wise
  constructor, framed-form only, all three `Admission` arms honoured (`Refused`
  refuses the case). TWO RIDERS: (a) the entry point's rustdoc carries the law
  citation and this argument; (b) a lexical fence test in the `diag_tidy` family
  pins `admit_fixture_records(` to dorc-loom/test surfaces only
  (the fixture-payloads precedent). BUILD LICENSED — same lane continues: seam →
  survival-lane extraction (post-rebase; the lib-seam pattern, third instance) →
  the 4 survival cases. The ~35 records-gated why faces then become a final
  face-sweep continuation of the X2a machinery.
- **tc-cmdsub-inner-span-is-wrong** (X2b, +SURE symptom) — a PRODUCTION kernel bug,
  out of arc scope: every `cmdsub-inner-nonleaf` span points at byte 0 (offset
  body-relative, not rebased; width correct; suspect the AST node for
  `$( … )`-internal commands, `effect.rs:1605`'s input). FLAGGED TO THE HUMAN /
  next kernel arc; the committed case bakes today's render and re-blesses on the
  fix. Sibling note: `tc-lint-route-shows-the-whole-report` (noisy exemplar for
  wrapper oracles) rides render-form-unwelded, no action.
- **x2d LANDED** (`ai/r28-loom-x2d` @ `f7b70bb6`, base `271d7921`; both legs green,
  Win 1655 / WSL 1651; two ratified follow-ons running, then it folds BEFORE X2b's
  continuation). All six items + the seven-item reviewer chafe pack: `owns:`
  ownership key (filename-implicit; conflicts refuse naming both files;
  `lint-source-analysis-diagnostics` now homes the other six `lint-source-*`) ·
  metadata absent-means-keep + regression gate + quote normalization ·
  `edit-loop:` hint in all 75 canonical cases, gate-held, generator-owned (names
  compile/promote + loom:vars + the `{{name}}` mechanism) · the invocation-error
  parts seat (main, dorc-sh, and both loom chains share it; `cli-usage-synopsis`
  faced; 12 invocation transcripts re-blessed, bodies only) · `envelope: stderr`
  key + `cli-plan-summary-line.loom` homing the plan-stderr trio · a CLOSED
  frontmatter vocabulary in the looms runner (22 keys) · bare-slug/multi-base CASE
  resolution + real `--help` · the DROPPED-VARIABLES compile disclosure · the
  shared-fixture collision fixed (empty-register fixtures chosen at run time) ·
  lock-not-clean remediation text · `sections` self-describing headers.
- **fnd-wrapped-rows-are-chunk-editable SUPERSEDED** (x2d diagnosis, measured
  corpus-wide; X2a's symptom was real, its mechanism wrong) — the weave absorption
  rule WORKS: one editable section per wrapping prose-component at every width
  (247 committed sections; the only split is a passthrough, not a wrap). The true
  defect was `refuse_added_lines` counting the RENDERER's soft wrap on the
  baseline side while normalizing only the edited side — any reword changing the
  laid-out line count tripped `AddedLine`, whose text then misdirected to
  add-register. Fixed by counting breaks in the STORED form on both sides via the
  two EXISTING normalizer seats (no new judgment seat); a latent inverse hole
  closed with it (a 3-line register no longer grants added-paragraph budget).
  Round-trip pinned: reword-across-the-break at width 40.
- **x2d adjudications** — envelope digest KEPT (production truth; fixture-tier FNV
  drift-detector class) · `todo` note-only key ACCEPTED (documented in the
  vocabulary; asserts nothing) · aid→dorc-loom DEV-dependency ACCEPTED (one
  ownership resolver; production edges untouched; steering-sync documents it) ·
  per-edit owner check → BUILD (follow-on 1) · metadata promote-REFUSAL +
  acknowledgement flag → BUILD (follow-on 2; the suite gate was after-the-write).
- **Steering-sync debts registered for X3** (from x2d's fold-notes): aid/CLAUDE.md
  `cases-live-here` ("filename match") + the dev-dep note; cli/CLAUDE.md
  loom-form-is-the-same-battery (key vocabulary now closed + 22-key list home);
  28L already carries the ownership/loop-hint/envelope entries above.
- **x2d FOLDED** (`ai/r28-loom-final` @ `6b23c5d8`, ff after the two follow-ons landed
  at `2817e200`: the per-edit ForeignComponent refusal naming the owning file, and the
  metadata promote-refusal + `--accept-metadata` with per-item disclosure — suite gate
  KEPT as the unpromoted-drift net). x2c DISPATCHED (worktree loom-x2c, off
  `6b23c5d8`): de-passthrough + the foreign-text seal, per the fnd-284-landed-state
  work-list; reads 284 under the builders-only protocol. X2b recovered from a
  transient crash + a rate-limit interruption (seam commit landed; survival
  extraction resumes; rebases over the x2d fold at its close).
- **CONDUCTOR REHEARSAL EXECUTED** (2026-07-30, own hands, loom-final worktree;
  landings `d26bdbb4` + the whylog-absent revision commit; prose landed under the
  bend). Part 1, overtype `mark-unknown-verb`'s placeholder with two `{{var}}`s:
  compile interpreted both, promote flipped the lock `None → Some` with params +
  concrete example — AND CAUGHT A REAL DEFECT: **fnd-lint-route-rerender-reads-const-
  not-mirror** (+SURE, reproduced end-to-end) — promote re-rendered the TRANSCRIPT
  with the stale compiled-in catalog (placeholder), leaving lock and transcript
  disagreeing; after rebuild the fixpoint went red as predicted; recovered via the
  sanctioned DORC_LOOM_DUMP two-step. The lint-route re-render seat misses the
  mirror thread D2 landed elsewhere — the exact class `28H:finding-why-render-reads-
  the-const-not-the-mirror` named, one seat left. FIX ROUTED to X2b's fold (or a
  micro-lane if X2b closes without it): thread the mirror ctx through the lint-route
  re-render in `render_direct_replay`/`replay`'s lint branch; pin with a
  promote-then-immediate-fixpoint test on a lint-rendered catalog code. Part 2,
  revise WRITTEN prose + move `{{dir}}` mid-sentence on `whylog-absent` (why
  route): one-step, transcript re-rendered correctly with no rebuild — isolating
  the defect to the lint seat. Also exercised live: the lock-not-clean refusal
  (x2d's remediation text — correct, actionable), the loop-hint teaching, vars
  discovery. Full local suite green (1657/1657) at the rehearsal tip.
- **X2b FOLDED** (`ai/r28-loom-final` @ `16577d6b`; both legs green at its tip,
  Win 1680 / WSL 1676; self-rebased twice as the target moved, framed-source hunks
  verified surviving both times). The records seam landed AS RATIFIED:
  `cli/src/results.rs` behind the loom seam; `admit_controller_records` (production,
  takes the controller's framing + pre-bounded bytes — the read stays at main.rs) vs
  `admit_fixture_records` (no Framing/host/nonce/attempt — a fixture caller cannot
  name a host in principle); scope fields stay private with checks moved ONTO the
  type (`matches_claims` — the better reading of check-never-mint); rider (b)'s
  lexical fence `fixture_intake_is_unreachable_from_production` PROVEN TO BITE
  (needle injected, failed correctly, restored). Survival extraction: ~880 lines
  verbatim to `cli/src/survival.rs` (third seam instance);
  `survival_diagnostics(...)` = the binary's own sequence; `< results` traverses the
  REAL intake honouring all three Admission arms (Refused refuses the world — never
  an empty degrade); BONUS: X2a's world.rs had COPIED the probe-shipping helpers
  rather than sharing — duplication collapsed, drift-risk closed (glanced at fold:
  correct). Four survival cases honestly firing; `deriv-family-incomplete` is the
  seam's end-to-end proof (full framed records through the production intake path).
  RATCHET 12 → 8: only the human-flagged records-8 remain. Spent backups awaiting
  human force-delete: `x2b-prerebase-backup` · `x2b-prerebase2-backup`.
- **x2e DISPATCHED** (the final why-face sweep, worktree loom-x2e off `16577d6b`;
  the last build lane besides x2c): (1) the lint-route mirror fix
  (fnd-lint-route-rerender-reads-const-not-mirror) + a promote-then-immediate-
  fixpoint pin; (2) `WhyWorld::analyze` gains optional records + `consented` per
  X2b's §6 spec (RunClock::Absent on the loom path — a transcript must be a
  fixpoint; the refuse-the-case arm preserved; the stale RESIDUAL-SCOPE-CUT
  docstring rewritten); (3) mint measured/survival why cases until every reachable
  `why-*` prose-component has a face — the ~35 records-gated families from X2a's
  list; un-faceable residue LISTED with mechanical reasons for the ledger;
  (4) the framed-header admission refusal prints the expected header
  (rul-refusals-name-the-next-command).
- **D2/D3 dispatched in parallel worktrees** off the post-landing tip — D2
  editable-surface pack (`ai/r28-loom-d2`): placeholder face · absorption→refusal +
  help scaffold-affordance · attached markers (`282` §13 amendment) · RenderCtx
  mirror-thread (census-corrected seats incl. lint/render.rs:119, cli/main.rs:243/414;
  absorbs layout_box) · `editable_baseline`-via-`replay` · txtar-header escape; D3
  reach pack (`ai/r28-loom-d3`): stand-in retirement per rul-honest-firing (parse/CFG
  diag union + records intake; irreducible residue beside payload types in aid) ·
  params destructuring enforcement · UnknownParam Rust-path refusal + rustdoc repairs ·
  the 14 trivially-fireable ratchet cases · cheap lint/cli faces.
