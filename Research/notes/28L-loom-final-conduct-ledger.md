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
- **D2/D3 dispatched in parallel worktrees** off the post-landing tip — D2
  editable-surface pack (`ai/r28-loom-d2`): placeholder face · absorption→refusal +
  help scaffold-affordance · attached markers (`282` §13 amendment) · RenderCtx
  mirror-thread (census-corrected seats incl. lint/render.rs:119, cli/main.rs:243/414;
  absorbs layout_box) · `editable_baseline`-via-`replay` · txtar-header escape; D3
  reach pack (`ai/r28-loom-d3`): stand-in retirement per rul-honest-firing (parse/CFG
  diag union + records intake; irreducible residue beside payload types in aid) ·
  params destructuring enforcement · UnknownParam Rust-path refusal + rustdoc repairs ·
  the 14 trivially-fireable ratchet cases · cheap lint/cli faces.
