# 282 — the transcript-case prose pipeline (working name: `wordloom`)

AI-authored (Fable conductor, 2026-07-19, from the two-day design dialogue with the
human; every §0 ruling is human-typed unless marked as a lean). PLAN-OF-RECORD for an
implementor. Authority: root docs, `spike/CLAUDE.md`, root `AID-NEEDS.md` outrank.
Companions: `notes/27U` (the aid-phase as-built ledger this plan corrects the direction
of) · `notes/27V` (the aid build-phase plan whose §3 catalog language this plan
supersedes-in-part) · `notes/27W` (decline classes; untouched) · `plans/281` (the
annotation mark-grammar — THE spec of the authored line-annotation surface; sibling
round-28 work). Round charter: `plans/280` (unminted at this writing; this plan
stands alone until it exists).

Implementor read-first: root `README.md`/`DESIGN.md` → `spike/CLAUDE.md` (User-aid
law block + Boundaries + Build/test/run) → root `AID-NEEDS.md` (Law section) →
`notes/27U` §1/§4 (what exists; the three-state prose protocol being partially
retired here) → `core/src/catalog.rs` + `core/tests/catalog_defining_cases.rs` +
`core/tests/diag_tidy.rs` (the as-built starting point) → this plan whole.

## §0 — Ruling ledger (the design is settled; spellings inside are latitude)

Human-typed, 2026-07-18/19 sitting:

- **`282:rul-transcript-is-the-authoring-surface`** — the purpose of the whole
  machinery: humans AND LLMs authoring user-facing prose look ONLY at what a user
  sees — same headspace, same error-model, including carets, visible code, and what
  is NOT visible. Writing a user-facing string at line 700 of a monotone pass is the
  named footgun (analyzer-headspace: you know things the user doesn't). Therefore
  the committed, executable transcript CASE is the authoring surface, and the
  compiled catalog is DERIVED from it — restoring the direction
  `AID-NEEDS:law-defining-case-catalog` always specified ("committed catalog
  intermediate regenerated ONLY by explicit promote; the lag IS the assertion") and
  reversing the as-built inversion (`27U` d1/d4b: hand-authored `catalog.rs`,
  fixpoint-only promote).
- **`282:rul-new-code-empty-loop`** — minting order for every new code: builder
  mints the slug with NO prose → the render shows a loud placeholder → the author
  supplies executable, triggering input-data (the case) → conductor/human writes
  the words while LOOKING AT that render. Prose cannot exist without a case; the
  machinery functions as the templating engine that produces exactly the
  what-does-the-user-know report.
- **`282:rul-words-and-paragraphs-only`** — the only authored value is an ordered
  series of words, grouped into paragraphs (two-plus linebreaks = paragraph break).
  ALL other formatting is render-owned and slurped/discarded at read-in (LLMs wrap
  words strangely; that noise must die at the boundary). The model may grow later;
  it starts this small.
- **`282:rul-multi-replay-per-case`** — one case file may carry MULTIPLE replay
  blocks (`--verbose` / `--terse` / machine-format views of the SAME input-state);
  every replay in a defining case must surface the case's code-slug (the same-slug
  coherence gate).
- **`282:rul-passthrough-type-gated`** — passthrough prose (foreign text riding a
  `detail`-style hole) is gated by TYPE, not by lint: the hole's type is a
  tagged-at-the-I/O-layer user-sourced value, NEVER constructible from a string
  literal. This reuses machinery the codebase already believes in (user-sourced
  data is distrusted and tagged for an ocean of independent reasons); our own
  sentences physically cannot ride a passthrough.
- **`282:rul-own-crate-own-tests`** — the machinery is its own small crate with its
  own tests. Reuse outside Dorc is nice-not-priority: honor the cheap two-layer
  split (§5), invest nothing further.
- **`282:rul-internal-tool-sharp-edges`** — this is self-consumed internal tooling.
  It does NOT get the product's properties (gradual enhancement, friendliness,
  accountability). Sharp edges are fine; it just needs to WORK. Refusals may be
  blunt dumps. (Composes with `27V:rul-aid-survives-the-spike`: what is keeps-tier
  is the DATA — catalog, corpus, format — and the transport-correctness property;
  the tool's UX is not.)
- **`282:rul-arrangement-words-exempt-v1`** — walker-owned structure words
  (connectives, tier words, the epilogue frame) are exempt from transcript-editing
  at v1; they stay co-located in arrangement code. (Seam: §10.)
- **`282:rul-frontmatter-txtar-container`** — the container is txtar-with-
  YAML-frontmatter: structured metadata at the head, file/CLI state as txtar
  sections after. Precedent the human located: terraform-plugin-docs acceptance
  tests; always lean on prior art for formats and tooling.
- **`282:rul-git-repo-dependence-accepted`** — the tool may depend on running
  inside a git repository; the access mechanism is conductor latitude (taken:
  subprocess `git` behind a trait — §6).

Conductor leans, standing unvetoed:

- **`282:lean-machinery-now-prose-lazy`** — build the machinery now; burn prose
  down deliberately lazily (`sm `-tier text stays legal); the concentrated
  prose-quality sprint waits for a surface-stability moment (field-trial prep is
  the natural one). Rationale: r26 reshapes records/renders and block-stdlib mints
  many new codes — sentence-polish now is investment into wet cement; the
  machinery's value (the `282:rul-new-code-empty-loop`) starts immediately.
- **`282:lean-flat-frontmatter-subset`** — the frontmatter is a FLAT YAML subset:
  `key: value` string scalars (+ string lists if needed), delimited `---` fences;
  nested structures refuse. Keeps "very barely structured", dodges YAML footguns,
  keeps LLM-familiarity. Parser latitude: hand-rolled subset or a vetted dep.
- **`282:lean-terse-single-paragraph`** — terse-register fields gate to one
  paragraph; multi-paragraph is legal only in deep/prose registers. Cheap to
  change; the human's stated position was "idk".

## §1 — What the thing is (one screen)

A case file is one txtar archive: frontmatter metadata, embedded source files
(book, oracles), embedded world (probe results; later per-host), and one final
replay section holding one or more `$ dorc …` command lines each followed by
EXACTLY what that command prints. At bless time the commands are literally
executed in a materialized temp dir and the output is re-inlined.

Two bless modes under a mechanical exclusivity law:

- **prose-bless (promote)**: structure is frozen. An author edited words inside a
  transcript. A word-level diff between the machine's own re-render and the edited
  text — attributed through the renderer's provenance tags — identifies exactly
  which catalog fields changed; the catalog is regenerated; every case showing
  those codes is re-rendered and overwritten.
- **structure-bless**: catalog prose is frozen (the file is clean). Arrangement or
  engine code changed. Every transcript is regenerated from scratch; prose
  provably cannot have drifted because it only flows from the unchanged catalog.
- Never both in one bless. A touched-set spanning both classes refuses.

Prior art (steal, don't invent): Mercurial t-tests / cram (`$ cmd` + inlined
output, bless re-runs), Go txtar + testscript (archive + driven commands), rustc
`tests/ui` + `--bless` (corpus-scale golden discipline), insta (snapshot review
flow), terraform-plugin-docs (txtar + frontmatter marriage). The ONLY novel leg is
the diff-driven extraction back into the catalog (§5).

## §2 — The case-file format

```
---
code: render-heredoc-refused
when-fires: the leaf-exact render would elide a heredoc-bearing leaf
why: kFAIL-perform; arch-1 d-6
---
-- book.sh --
#!/bin/sh
cat <<EOF >/etc/motd
hello
EOF
-- probe-results.txt --
site 0 effect=holds
-- replay --
$ dorc plan --book=book.sh < probe-results.txt
render: error[render-heredoc-refused]: leaf-exact render refuses to elide ...
  = help: split the heredoc body to its own leaf, or mark the kind un-elidable
$ dorc plan --book=book.sh --format=jsonl < probe-results.txt
{"envelope":"dorc-lint-format/1", ...}
```

- **Frontmatter** (flat subset, `282:lean-flat-frontmatter-subset`): `code` (the
  defining slug; absent for non-defining corpus cases), `when-fires`, `why`.
  Params are NOT declared here — they derive from the payload type in code, as the
  existing `promote_refreshes_params` machinery already does. Keep the key set
  minimal; every addition is a format commitment.
- **File sections**: verbatim, LF-only (materializer pins LF; CRLF in a section is
  a bless-time refusal). Multihost convention (format-neutral, build nothing now):
  `hosts/<name>/probe-results.txt` section names; the replay command names hosts.
- **The replay section** (always last): a sequence of blocks; each block = one
  `$ `-prefixed command line + its inlined output (until the next `$ ` line or
  section end). Commands run SEQUENTIALLY in one materialized temp dir with a
  shared per-case scratch — required for run-then-`dorc why --last` sequences (the
  whylog flows between commands). Each command's spelling is the user-shaped
  invocation with case-relative paths; harness-only environment must not appear
  (framed records in fixtures, not `DORC_ALLOW_LEGACY_RESULTS`).
- **Machine-format replays**: a replay whose render carries no human prose regions
  (e.g. `--format=jsonl`) is whole-block structural — never prose-editable, always
  regenerated. This is how machine-envelope coverage lives in the same case as the
  human views (`282:rul-multi-replay-per-case`), replacing the retired fragment
  goldens (§8).
- **Coherence gates**: a defining case's every replay must surface its `code` slug
  at least once; a case whose replay output contains a line that parses as a txtar
  marker refuses at bless (no escaping exists — sharp edge, acceptable).
- txtar caveats honored: newline-termination on round-trip; text-only. Rust `txtar`
  crates exist but are ~100 lines to own — vendor-or-implement is builder latitude
  with a one-line rationale.

## §3 — The prose model (words and paragraphs)

- Authored value = ordered paragraphs of ordered words. Read-in normalization:
  within a paragraph every whitespace run (including single newlines) collapses to
  one space; two-plus consecutive newlines = one paragraph break; nothing else
  exists. Catalog prose fields store the normalized form.
- The renderer owns ALL layout: wrapping (committed transcripts render at ONE
  pinned canonical width), indentation, paragraph spacing, connectives. Live
  surfaces may wrap adaptively; the corpus does not.
- Registers: `282:lean-terse-single-paragraph`.
- Param values in canonical case worlds must be non-empty and word-distinctive
  (a bless-time lint on the case): an empty or common-word value is un-anchorable
  by the re-holer (§5).

## §4 — The tagged render

The renderer already fills templates at a single seat (the catalog table +
`tier_word` + arrangement code are the only prose sources by construction —
verified in the as-built). Extend that seat to emit, alongside the bytes, a span
map classifying every output run as exactly one of:

- **TemplateLiteral(code, field, paragraph-index)** — the code's own prose words.
- **ParamValue(code, field, param)** — interpolated payload data.
- **ForeignText(param)** — passthrough text (§8's type; display of tainted bytes).
- **Arrangement(arrangement-slug)** — numbering, connectives, tier words, carets,
  gutters, code excerpts, blank structure.

The map is the attribution authority for §5; the word-diff is only alignment.
Nothing here may leak into product surfaces: the tagged render is a tool-mode
output (the walker gains a `render_tagged` twin, or the map rides beside the
bytes; latitude).

## §5 — The transport engine (the generic crate layer)

Layer 1, zero Dorc types: `(baseline bytes + span map, edited text) → per-region
word edits | refusal`. Layer 2, the Dorc adapter: produce tagged renders; apply
field-edits to the catalog. Stop abstracting there (`282:rul-own-crate-own-tests`).

- Tokenize both sides to word streams (whitespace-split; paragraph breaks are
  tokens). Align by word-level diff (Myers/patience over tokens; latitude).
- Attribute each changed token through the baseline map:
  - inside TemplateLiteral → collect as a prose edit for `(code, field)`;
  - inside ParamValue / ForeignText → REFUSE ("that's the payload, not prose")
    unless consumed by re-holing below;
  - inside Arrangement → REFUSE ("that's structure — structure-bless it");
  - insertions at region boundaries with ambiguous attachment → REFUSE.
- **Re-holing**: after extracting a field's new word-sequence, replace every
  occurrence of each declared param's instantiated value word-sequence with its
  hole. Authors may also literally type `{param}` in an edit (substituted at
  re-render — the sanctioned transient break of see-what-the-user-sees). Ambiguous
  or overlapping matches → REFUSE.
- **Consistency rules**: contradictory edits to two instances of one template →
  REFUSE. Prose edits are legal ONLY in the code's defining case; an edit to the
  same template's render in any other case → REFUSE with a pointer (strengthens
  `AID-NEEDS:law-one-defining-case-per-code`).
- Refusals are blunt (`282:rul-internal-tool-sharp-edges`): dump both word
  streams, the region table, and the offending hunk; exit nonzero. No suggestions,
  no fuzzy matching, ever. If refusals prove too annoying in practice, the named
  retreat is region-annotated editing (faint markers in a transcript during
  editing only) — decide then, not now.
- **The one hard-tested property** (the crate's reason to have its own tests): for
  any edit confined to one TemplateLiteral region, promote → regenerate →
  re-render reproduces the edited words exactly, modulo whitespace normalization.
  Property-test it (seeded random word edits across seeded random span maps), plus
  a table of every refusal class.

## §6 — Bless modes, git gating

- Dependence: prose-bless requires a git repository (`282:rul-git-repo-dependence-
  accepted`); plain regenerate/run works anywhere. Access = subprocess `git`
  behind a two-method trait (`head_version_of(path)`, `dirty_paths()`), fake impl
  for tests. Judgment recorded: the needs are two trivial queries; gix/git2 are
  heavy deps (license/deny friction) for a tool whose every real host has the
  binary; the trait IS the gix swap seam if the mini-product ever wants embedding.
- **Mode inference + gates**: classify the touched-set (git). Case-file edits only
  + clean catalog ⇒ prose-bless legal. Rust/arrangement edits + clean catalog +
  case prose untouched ⇒ structure-bless legal. Both classes, or a dirty catalog
  (hand-edit detection) ⇒ REFUSE.
- **prose-bless**: baseline = re-render with current catalog+code; verify it
  matches HEAD's transcript everywhere EXCEPT prose regions (this verification IS
  the never-both law — structure drift means "structure-bless first"); run §5;
  regenerate the catalog; re-render every affected case; overwrite; the review
  surface is the resulting git diff (`--word-diff` matches the granularity).
- **structure-bless**: regenerate everything from scratch; all churn provably
  structural; bulk-reviewable.
- **CI fixpoint gate**: promote over the committed corpus reproduces the committed
  catalog byte-identically. This catches ANY hand-edit of the generated catalog —
  prose or metadata — and is what retires the `CONDUCTOR_AUTHORED` roster (§8).

## §7 — The execution harness

- Materialize sections to a temp dir (LF pinned); run replay commands in order
  with a controlled environment (`env -i`-style, PATH pinned to the built `dorc` +
  the case's inert mocks; cwd = the temp dir so paths render RELATIVE — absolute
  host paths in a transcript are a bless-time refusal); capture combined output
  (`2>&1`) v1 — a command wanting stream separation spells its own redirection in
  the command line. Flag: combined-capture interleaving is deterministic only
  because dorc's surfaces are effectively single-stream per invocation; if that
  ever breaks, split-capture is the escape hatch.
- Safety: identical rails to the e2e harness (inert mocks only, no real mutators,
  worktree-local, no network). Case execution is git-free.
- Determinism: same DST discipline as e2e; committed transcripts are byte-stable
  under re-execution or the bless refuses. Multihost transcripts additionally
  require deterministic cross-host output ordering — a named CHECK on the r26
  reactive work (`26B` confluence targets plans, not stderr streams), not work here.

## §8 — Dorc integration (the adapter, and what changes at home)

- **The catalog becomes fully generated**: prose fields, metadata (from
  frontmatter), params/example (from code — existing machinery). The committed
  `catalog.rs` stays the compiled, diffable intermediate; its EDIT surface is the
  corpus. Hand-edits die (caught by §6's fixpoint gate).
- **Transitional carry-forward**: promote sources prose from cases-where-they-
  exist ∪ current-catalog-prose-where-not, so the 35 case-less codes keep their
  `sm `-tier prose mid-migration. THE RATCHET (as-built:
  `DEFINING_CASE_RATCHET`, a shrink-only allowlist gate — entries may be removed,
  never added, so coverage only grows) is REDEFINED to "codes whose prose is not
  yet case-owned"; the completeness gate stays covered ∪ ratchet == all slugs.
- **Placeholder semantics**: `[unwritten: <slug>]` stops being a stored string;
  an absent prose field renders the placeholder at render time
  (`282:rul-new-code-empty-loop`). The `sm ` migration markers survive as ordinary
  words until rewritten — at the transcript surface.
- **Roster retirement**: builders still author zero prose
  (`27V:rul-error-authorship-tier` stands); enforcement moves from the
  `CONDUCTOR_AUTHORED` gate-roster to promote-privilege (BLESS-law:
  orchestrator-only) + the §6 fixpoint gate.
- **`282:rul-passthrough-type-gated` work**: mint (or extend `OutBytes` into) a
  user-sourced text type under the sealed-room pattern (`core::room` precedent) —
  constructible ONLY at I/O edges (parser input relays, tool stderr, host bytes);
  passthrough catalog holes type to it; string literals cannot reach it. Audit
  every current `detail: String` payload: genuinely-foreign relays wrap at their
  edge; sentences composed at emit sites from OUR words de-passthrough into real
  templates (world-variant siblings where needed —
  `AID-NEEDS:law-codes-vary-by-world-not-grammar`). Convergence note: this same
  type is what `an-output-sanitization` will key on later — the taint tag does
  double duty.
- **What retires**: the 17 unit-tier fragment goldens + `DORC_DEFINING_BLESS`
  (superseded by case transcripts); the roster; the stored-string placeholder.
  **What stays**: the `DiagCode` enum + typed payloads + registry; `diag_tidy`'s
  emit-site gate (the fires-half backstop) — though defining cases now REALLY
  fire, closing `27U:finding-corpus-blind-edge-codes`; the e2e corpus and its
  plan-render goldens (different product surface); machine-envelope shape
  assertions (move to a machine-format replay block or stay unit-tier; latitude).
- Registry/law sync (root `AID-NEEDS.md` law wording, `spike/CLAUDE.md` aid block)
  rides the INTEGRATION landing, not this plan-mint — one sync commit when the
  direction is built truth, not paper truth.

## §9 — Phases (the implementor's ladder; serial, each gated)

1. **phase-transport-crate** — layer-1 engine standalone (tokenizer, aligner,
   attribution, re-holer, refusal classes) + the property test + refusal table.
   No Dorc contact. This lands FIRST because it is the only novel/risky leg: if
   transport fights reality, we learn it before anything is rewired.
2. **phase-tagged-render** — the walker's span-map twin + pinned-width mode +
   the ParamValue/Arrangement/ForeignText classification. Product renders
   byte-unchanged (a golden-stability gate proves it).
3. **phase-container-and-runner** — txtar+frontmatter parse/materialize, the
   sequential replay runner on the e2e rails, inline-on-bless, the coherence
   gates. Cram-style self-tests.
4. **phase-generation-flip** — promote v2 (extract → regenerate catalog →
   re-render corpus), the git mode-gates, the carry-forward, the CI fixpoint
   gate, roster retirement. THE direction lands here.
5. **phase-backport** — the 17 existing defining cases become case files (their
   canonical payloads already exist as constructors; worlds authored per case);
   fragment goldens + `DORC_DEFINING_BLESS` deleted; ratchet re-pointed.
6. **phase-de-passthrough** — the §8 type + edge audit + template extraction for
   our-words passthroughs. (Day-tier estimate; mostly mechanical.)
7. **phase-lazy-burn-down** — ratchet shrinks opportunistically
   (`282:lean-machinery-now-prose-lazy`); the `records-*` corruption tail (~30h
   as-estimated in `27U` §7 item 3) is explicitly NOT this dispatch's scope; new
   codes are born through the empty loop from phase-4 onward.

Dispatch shape per standing law (safety block · step-zero/0.5/one · sonnet clamp ·
comment budget · four gates + foreground e2e · granular commits · builders author
zero user-facing prose — placeholders only). Phases 1–3 are one builder lane each
at most; 4 is the two-phase (proposal → go) checkpoint shape (`27U` §5
map-then-execute-split) because it rewires the catalog's ownership.

## §10 — Out / deferred (named seams)

- Arrangement-prose promotion (connectives/tier-words as transcript-editable
  entries) — the kFLOW seat; v2, after the corpus exists to argue over.
- gix/library embedding; SARIF-style exports; any reuse packaging beyond the
  layer split. Rename latitude on `wordloom` itself.
- Split-stream capture; TTY-adaptive width in transcripts.
- Multihost transcript determinism — the r26 check (§7).
- The prose-quality sprint — scheduled at a surface-stability moment, not here.
- The prose-register schema (terse/deep/first-encounter) + remediation-hint prose
  migration (`27U` §7 item 7) — pairs with the human's slow `sm `-rewrite pass;
  wants its own short sitting once transcripts exist to stare at.

## §11 — Confidence

+SURE: the as-built inventory this plan corrects (code-verified 2026-07-19); the
single-seat render fill (verified — the tagging is an extension, not a refactor);
the ruling ledger (human-typed in session). ~SUSPECT: transport stays a small
boring algorithm under the refuse-loudly posture (the property test + refusal
table is the tripwire; the annotated-editing retreat is priced); the carry-forward
keeps migration monotone (no code loses prose mid-flight). -GUESS: phase sizing
(1–3 feel like one lane each; 4 is the risky one and is checkpointed accordingly).
