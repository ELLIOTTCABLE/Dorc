# loom-final — the map (DRAFT; read-only lane, no fixes landed)

AI-authored (Opus map-builder, 2026-07-29, worktree `.claude/worktrees/loom-final`, branch
`ai/r28-loom-final`, base `833bbe0b`). MAP ONLY: every claim below was driven against the real
tools in this worktree; every probe edit was reverted; the tree is byte-identical to base apart
from this file. Authority: root docs, `spike/CLAUDE.md`, human-typed rulings outrank.
Companions: `plans/282` (loom-pipeline design authority) · `plans/288` (charter) ·
`notes/287` (errorloom as-built) · `notes/28H` (W4 ledger) · `notes/289` (unify ledger) ·
`notes/28J` (the human's worklist).

## §-1 — Assumptions carried from the re-charter (human-typed 2026-07-29)

- **asm-prose-authorship-bent-this-arc** — the no-AI-prose law is bent FOR THIS ARC where it
  unblocks completion. Consequence for every spec below: dogfood round-trips may LAND rather than
  revert, and scaffolded cases may carry provisional prose. A bulk prose-quality rewrite is still
  NOT the goal, and `27V:rul-error-authorship-tier` resumes at arc close.
- **asm-remit-is-the-edit-surface** — the remit is NOT "all user-aid". It is: make looms the
  correct place to edit CURRENT AND FUTURE user-aid, including a user-friendly way to add/edit
  VARIABLES, with fingers out of the compiled catalog. No new text-emitting machinery in core
  Dorc.
- **asm-no-compat-anywhere** — `282:rul-strawman-formats-no-compat` is reaffirmed: `.loom` syntax,
  both locks, the receipt format, and the transcript byte-form all reshape freely. Nothing below
  is priced with a migration shim.
- **rul-rust-and-loom-are-the-only-edit-surfaces** (human-typed 2026-07-29; the sharpening
  amendment) — the editor persona is an experienced RUST developer with ZERO loom experience.
  Everything between `Diag` (the Dorc-side diagnostic API — payload types, mint sites, mostly in
  `aid`) and the loom file is a LOOM BLACK-BOX. **`rust` and `loom` are the only two valid edit
  surfaces** — Rust when behaviour or data changes, loom when prose changes. `dorc-loom` internals,
  `errorloom` internals, `catalog_lock.rs`, and `arrangement_lock.rs` are valid for NEITHER
  persona, EVER. Two testable consequences: (1) turning a Rust value at an error-is-happening site
  into a loom-usable variable is one clean API interaction AT THAT SITE, after which the value
  flows through the black box automatically — listed by `vars --all`, insertable as `{{name}}`,
  `params`/`example` self-deriving through promote, with zero manual steps against intermediary
  artifacts; (2) a loom editor who wants a missing contextual value mid-edit goes to the Dorc-side
  implementation as an API CONSUMER and adds it there. Assessed in §2f; the teaching seat is the
  `Diag` API's rustdoc plus the tool's own refusals, never loom documentation.

---

# §0 — map-boundary-model-assessment

> The human's frame: boundaries ARE the product. The question is *how do we separate user-edited
> text from structural text*, and it needs one central structural answer, not per-case
> edge-detection.

## §0.1 — fnd-two-render-chains-bridged-by-a-guess (the root cause of the whole failure class)

The pipeline has **two render chains for the same case**, producing **different bytes**, joined by
a heuristic inverse. This single fact generates every refusal measured in §1.

| chain | entry | bytes | consumer |
|---|---|---|---|
| A — the *provenance* chain | `DorcConsumer::replay` (`dorc-loom/src/consumer.rs:437`) | the render seat's raw parts stream, **unwrapped** | `to_editable_render` → `EditableRender` → compile |
| B — the *committed* chain | `DorcConsumer::render_direct_replay` (`consumer.rs:1124`) → `reflow_to_canonical` (`consumer.rs:1257`) | the same bytes **hard-wrapped to 80 cols with hanging indents** | `render_case` → the looms-runner fixpoint; the committed `.loom` transcript |

`render_cli_parts` (`aid/src/diag.rs:2544`) never wraps — verified by reading: the title part is
`format!("{severity}[{slug}]: ")`, then the message parts verbatim, then `"\n  = {connective}: "`
verbatim. Production `dorc` therefore prints the message body FLAT. The committed transcript is a
corpus-only reflowed form (sanctioned by `282` §3 and the `282` §12 "canonical wrap" rider — the
human asked for hard wrapping because it is nicer to edit in-editor).

Because the committed bytes are not the render bytes, compile must **undo** the wrap before it can
attribute an edit — that is `unreflow` (`dorc-loom/src/bin/dorc-loom.rs:737`). And `unreflow`
cannot know what was wrapped, so it **re-detects structure from byte shapes**:

- `first.contains("]: ")` — "is line 0 a diagnostic title?" (`dorc-loom.rs:741`)
- `line.starts_with(indent)` — "is this a wrap continuation?" (`:804`)
- `!line.trim_start().starts_with("-->")` — the exclusion list; **`= help:`/`= note:`/`= repair:`
  are absent from it** (`:804`) → §1's six refusals
- `line.trim_start().starts_with("= ")` — "is this a help block?" (`:748`)
- `normalize_layout`'s `   = ` → `  = ` respell (`:767`)
- `is_caret_gutter`'s digits-then-spaces-then-`|` sniff (`:787`)

+SURE: **this is exactly the "oh, THIS time user-text was beside a colon" pattern the human
objects to, and it is the ONLY place in the pipeline where it lives.** The rest of the transport
is genuinely provenance-carried.

## §0.2 — The partition the re-charter asked for

### (a) CARRIED PROVENANCE — sound, keep, and make load-bearing

| component | seat | what it carries |
|---|---|---|
| the render-part stream | `aid/src/tagged.rs` `RenderPart::{TemplateLiteral, ParamValue, ForeignText, Arrangement, ArrangementPage, ArrangementWords, ArrangementValue}` | per-run: owner slug, register, instance, param name / value index |
| section accumulation | `dorc-loom/src/lib.rs:96` `to_editable_render` + `OpenSections` | which adjacent runs are ONE editable section; `Arrangement`/`ForeignText` close a section |
| the fragment series | `EditableSection(Text \| Variable)` | exactly where words re-divide around values (`28H` ruling 3) |
| within-section attribution | `errorloom::transport_edit*` (`errorloom/src/editable.rs:299`) | scalar-level alignment, inclusive variable boundaries, unique minimum-removal |
| the compiled series | `dorc-loom/src/compile.rs` | `Text \| Variable` → catalog/arrangement fields |
| lock generation + both fixpoints | `dorc-loom/src/generate.rs`, `crates/cli/tests/looms.rs` | byte-identity of the two locks and every committed transcript |

None of these consult byte shapes. They are the product, and they work: §1's 34 accepted edits, the
`{{var}}` move/delete/insert probes, and the arrangement placeholder mint all run through them
untouched.

### (b) RE-DETECTION HEURISTICS — the kill-list

| # | seat | file:line | what it guesses | replacement |
|---|---|---|---|---|
| K1 | `unreflow` | `dorc-loom/src/bin/dorc-loom.rs:737-764` | whether line 0 is a title; which lines are continuations | DELETE (see §0.4) |
| K2 | `join_continuations` | `:796-811` | continuation-vs-new-block, by indent + an exclusion list | DELETE with K1 |
| K3 | `normalize_layout` | `:766-776` | `= ` marker column; gutter padding | DELETE with K1 |
| K4 | `is_caret_gutter` | `:787-794` | "digits then spaces then `\|` = a source gutter" | DELETE with K1 |
| K5 | `reflow_to_canonical` | `dorc-loom/src/consumer.rs:1257-1278` | title split at `]: `; help split at `= ` + `: ` | MOVE INTO the render seat (§0.4) |
| K6 | `wrap_words` | `consumer.rs:1283-1305` | word boundaries via `split_whitespace()` over the WHOLE line, **rendered values included** | MOVE INTO the render seat; wrap runs, never values |
| K7 | `compile_section_edits`' anchor search | `dorc-loom/src/edit.rs:190-198` | which bytes belong to an intermediate section, by `rest.find(&anchor)` + a "does the anchor recur?" ambiguity test | replace with index-ordered alignment (§0.5) |
| K8 | catalog `message: None` rendered as `Arrangement` | `aid/src/diag.rs:2786-2790` | (not a guess — a provenance OMISSION; the placeholder is stamped as immutable chrome) | stamp the row's face, exactly as `said.rs:178-188` already does for arrangement rows |

K8 is not strictly re-detection, but it belongs on the same list: it is the other way the model
fails — a byte that IS user-editable prose is stamped as structure.

+SURE on K1–K6 and K8 by reading and by driving. ~SUSPECT that K7 is the complete list of
content-search addressing (I read `edit.rs` and `editable.rs` whole, but not `receipt.rs`'s
comparison paths).

## §0.3 — Stress: arbitrary future output text

Each row is the model's answer TODAY vs. under the §0.4/§0.5 proposal.

| stress | today | under the proposal |
|---|---|---|
| a value containing `\n` | `wrap_words` `split_whitespace()` (K6) silently turns it into a space in the committed transcript; the transcript stops being what the user sees; the value may then be un-anchorable | value runs are never re-spaced; the render seat wraps only WORD runs |
| a value containing two spaces | same collapse as above | preserved |
| a value containing `error[` / `]: ` | if it lands on line 0, `unreflow` still treats line 0 as a title (harmless); if a value's newline pushes `= ` to a line start, K2 mis-classifies | no classification exists |
| a value containing `-->` | `join_continuations` stops joining mid-title → prefix/suffix mismatch → `MarkerOutsideEditableSection` | no classification exists |
| empty value | zero-width `Variable` fragment survives transport, but `28H` ruling 1 already names empty-rendering values a SEAT defect (occurrence-keyed variants are the answer) | unchanged; still a seat question, not a transport one |
| two equal rendered values in one section | distinct IDs preserved; ambiguous minimum interpretations refuse (`287` §1) | unchanged, correct |
| glued punctuation around a NEW marker | `AttachedMarker` refusal — reproduced (§5) | needs `282:phase-adjacent-fragment-followup` regardless |
| multi-paragraph prose | paragraph add/remove is out of the v1 words-and-paragraphs model (`287` §8) | unchanged, correct-by-design |
| prose that looks like structure (a message beginning `= note: …`) | K2/K3 mis-classify it as a help block | no classification exists |
| structure that looks like prose (the `[unwritten: …]` placeholder) | stamped as chrome ⇒ 14 codes uneditable (§1) | stamped with its row's face (K8) |
| an added `= help:` under a spanless code with `help: None` | **silently absorbed into the message register** — see fnd-added-help-is-silently-absorbed below | refused, or (better) accepted as a real help register — §2a |

### fnd-added-help-is-silently-absorbed (+SURE, reproduced, NEW — not in the banked diagnosis)

Typing `   = help: probeword alpha bravo` under `cli-no-book-given`'s one-line message compiles
**green** and stores this as the MESSAGE register:

```
sm no book given (a positional path or --book=PATH) = help: probeword alpha bravo
```

(read out of `spike/target/dorc-loom/compile.receipt`, fields `ctext`/`concrete`). No refusal, no
warning. The same edit under a case that HAS a caret frame (`tolerates-unknown-dimension`) refuses
`MarkerOutsideEditableSection` — the `-->` exclusion accidentally protects it. So the corpus has
**87 codes** (`help: None`) sitting on a silent-corruption path, and **7** on the safe one.

This is the sharpest single argument for §0's thesis: a byte-shape classifier does not merely
refuse when it is wrong, it sometimes **accepts wrongly**. Under the fixed-reflow patch of §2b the
absorption *persists* (verified — the stored message just gains an embedded newline), so §2b does
NOT close it; §2a must.

## §0.4 — prop-wrap-inside-the-render-seat (the central structural answer)

**Rule to adopt** (proposed law, conductor to rule):

> **rul-editability-is-stamped-never-re-derived** — editability is stamped at the ONE render seat
> and carried byte-exactly to the committed transcript. No component may re-derive structure,
> editability, or word boundaries from the SHAPE of rendered bytes. The committed transcript is
> the renderer's bytes, verbatim.

The mechanism is a one-line consequence: **move the canonical wrap inside `render_cli_parts`**, so
the wrap decision is made where the part boundaries are known, and each wrapped run is emitted as
its own stamped part. Then chain A ≡ chain B, `reflow_to_canonical` and `unreflow` both die, and
K1–K6 go with them.

Three arguments this is the right cut, not a bigger one:

1. **The why surface already does it.** `cli::WHY_WIDTH = 92` (`cli/src/lib.rs:897`) and weft wraps
   *inside* the parts stream — which is exactly why `28H` banked "`unreflow` now demonstrably
   unnecessary for weft-rendered surfaces". The diagnostic surface is the last one still wrapping
   outside the provenance system. This is convergence onto an existing, proven seat, not a new
   design.
2. **It preserves the human's stated want.** `282` §12's canonical wrap exists because hard-wrapped
   transcripts are nicer to edit. Wrapping in the seat keeps that AND makes production output
   match the transcript — strengthening `282:rul-transcript-is-the-authoring-surface` rather than
   weakening it.
3. **It costs a product decision, not a product risk.** Production diagnostics become
   fixed-width-wrapped instead of flat. That is squarely inside `27V:rul-output-form-unwelded`,
   and the why surface set the precedent. **tc-fixed-width-diagnostics-is-a-product-choice** —
   flagged UP; the alternative (store flat transcripts, drop the wrap) also kills K1–K6 but loses
   the editing ergonomics the human asked for.

Verdict: **the stamped-provenance model is SOUND. There is no deeper rot.** The failures are all
one architectural wart — a post-hoc byte transformation outside the provenance system — plus one
provenance omission (K8) and one addressing weakness (K7). ~SUSPECT: I did not audit
`receipt.rs`/`receipt_store.rs` line-by-line, so a fourth wart could hide there.

## §0.5 — prop-section-addressing-by-index (K7)

`compile_section_edits` walks components in order but addresses an intermediate section's bytes by
`rest.find(&anchor)`, where `anchor` is the immutable run that follows it, and refuses
`AmbiguousCandidate` when that run recurs (`edit.rs:190-198`). Immutable runs in a laid-out report
are exactly the strings that recur (`"\n   "`, `": "`, `", "`), so **the more chrome faces a
transcript grows, the more of its middle becomes unaddressable.**

Measured live, on a committed case: editing `[unwritten: why-receipt-when-replayed]` or
`[unwritten: why-receipt-book-drifted]` in `crates/aid/tests/why-drift-analysis-suppressed.loom`
refuses `AmbiguousCandidate`; editing `[unwritten: why-receipt-dispositions-predicted]` (the last
receipt row before a blank line) succeeds. So the friction `28H` banked as "will bite first on a
multi-section why transcript" is **already biting, today, on two of the human's own top-10 rows**
(`28J` items 2 and 5).

Replacement: align the edited text against the component SEQUENCE (a longest-common-subsequence or
a bounded two-pointer walk over the immutable runs, taking the *unique consistent* assignment
rather than the *first* occurrence), and refuse only when no consistent assignment exists.
`errorloom::diff` already carries an LCS aligner (`errorloom/src/diff.rs`) built for exactly this
shape of problem — reuse, do not re-invent. This is generic transport work and belongs in
errorloom under `289:steer-errorloom-best-to-use`.

## §0.6 — Consequences for the rest of this map

Every §2 spec is written as an INSTANCE of §0.4/§0.5, not as a standalone patch:

- §2a (placeholder + help) = K8 + the help-register consequence of §0.4.
- §2b (reflow) = the K1–K6 deletion, i.e. §0.4 itself. It is no longer "fix
  `join_continuations`"; the one-line patch is proven sufficient for today's corpus (§1) but is
  precisely the edge-detection hack the human forbade, so it lands only as a bridge if §0.4 slips.
- §2c (whylog/lint provenance) = a seat completeness gap, orthogonal.
- §2d (mirror lag) = a seat lookup gap, orthogonal.
- §2f (variable semantic) = the OTHER boundary, and the same shape of defect. §0 is about the
  boundary between user text and structural text WITHIN a render; §2f is about the boundary between
  the two sanctioned edit surfaces (`rust` and `loom`). Both are violated the same way: a fact that
  should be carried structurally is instead re-derived or hand-duplicated somewhere it does not
  belong — a placeholder classed by byte shape in §0, a stand-in world hand-written inside the loom
  crate in §2f. One law, two faces.

---

# §1 — map-crux-verified

All three banked facts verified by driving `cargo run -p dorc-loom -- compile` in this worktree;
every probe reverted (`git status` clean at write time).

## §1.1 — fact-crux-placeholder-chrome: CONFIRMED, all 14 (+SURE)

Method: for each `message: None` code, replace its `[unwritten: <slug>]` bytes with three filler
words, run compile, revert.

```
$ cargo run --quiet -p dorc-loom -- compile --quiet crates/aid/tests/whylog-unwritten.loom
case: crates/aid/tests/whylog-unwritten.loom
refusal in replay 0: MarkerOutsideEditableSection
baseline: exact renderer provenance
edited:
error[whylog-unwritten]: probeword alpha bravo charlie
compile: 1 cases, 1 refused
```

Identical refusal for all fourteen: `marker-version-unrecognized` · `mark-unknown-verb` ·
`mark-rc-arity-exceeded` · `mark-standalone-rc-consumer` · `mark-hashcolon-malformed` ·
`host-evidence-admission-refused` · `whylog-unwritten` · `oracle-role-fn-unlifted` ·
`shared-cell-measurements-disagree` · `mark-on-and-or-list` · `transport-apply-failed` ·
`transport-crlf-refused` · `transport-not-attempted` · `transport-session-lost`.

Mechanism confirmed by reading: `render_body_parts_with` mints the placeholder as
`RenderPart::Arrangement { slug: "unwritten-placeholder" }` (`aid/src/diag.rs:2786-2790`);
`to_editable_render` maps `Arrangement` → `OpenSections::structure` → `RenderComponent::Structure`
(`dorc-loom/src/lib.rs:115`, `:289`), which is immutable and closes the open section. Contrast
`aid/src/said.rs:178-188`, where the arrangement placeholder is minted through
`crate::weave::words(unwritten_placeholder(slug), slug, occurrence)` — i.e. it keeps the row's
face. Proven live: overtyping `[unwritten: why-drift-analysis-suppressed]` compiles green
(`section: why-drift-analysis-suppressed.arrangement-line#0:8`).

`28J`'s "7 unwritten catalog codes, all loom-editable today" is doubly wrong: the count is 14, not
7, and none of them is editable.

## §1.2 — fact-reflow-swallows-help: CONFIRMED, and it is exactly the help-bearing spanless set (+SURE)

Six case-owned codes refuse an ordinary word edit to their MESSAGE:
`whylog-absent` · `whylog-book-desync` · `whylog-corrupt` · `whylog-version-refused` ·
`render-heredoc-refused` · `aid-unloaded-sibling-oracle`.

The set is not approximate — it is **exactly the codes with `help: Some(...)` and no caret frame**.
The catalog has 7 `help: Some` rows; the seventh is `cmdsub-operand-top`, whose `-->` frame trips
`join_continuations`' one exclusion, and it compiles green. A static scan of all 62 committed aid
cases for "a `= help:`/`= note:`/`= repair:` line after a `]: ` title with no `-->` between them"
returns those same six files and nothing else.

Blast radius is therefore larger than the register count suggests, because of
fnd-added-help-is-silently-absorbed (§0.3): the 87 `help: None` codes are not safe, they are
silently corruptible.

Fix proven: adding `&& !line.trim_start().starts_with("= ")` to `join_continuations`' predicate
(`dorc-loom.rs:804`) flips **all six to ACCEPTED**, and additionally makes the HELP register itself
editable (probed on `whylog-absent`'s help text: ACCEPTED). `cargo test -p dorc-loom` and
`mise run test:looms` both stay green under the patch. The patch was committed temporarily to get
past the dirty gate, then `reset --hard`ed away; the branch carries no fix.

## §1.3 — fact-whylog-provenance-hole: PARTIALLY CLOSED, one seat still open (+SURE)

`287` §11 deferred "generalizing `editable_baseline` to whylog cases (there `world_of` fails)".
As-built at this tip:

- `case_diag` (`consumer.rs:618`) and `baseline_from_render` (`:548`) **both** carry
  `.or_else(|_| Self::whylog_diagnostic(case))`. The compile/promote path therefore works.
- `editable_baseline` (`:414`) does **not** — it calls `Self::world_of(case)?` bare.

Consequence, measured over the whole corpus:

```
$ cargo run -q -p dorc-loom -- vars --used crates/aid/tests/whylog-absent.loom
dorc-loom: crates/aid/tests/whylog-absent.loom: no canonical world for `whylog-absent` (world-as-payload)
```

Four of 62 cases fail: `whylog-absent`, `whylog-book-desync`, `whylog-corrupt`,
`whylog-version-refused`. (`whylog-unwritten` passes — it is a `dorc plan` case with a
`canonical_payload` arm.) Every other case answers. So the hole is now exactly one function, and
`282:rul-used-inventory-is-committed` (the committed `vars --used` block per defining case,
deferred in `287` §11) is still blocked on it.

## §1.4 — The true per-code EDIT-PATH matrix (the census counted prose STATES; this counts paths)

Corpus totals, measured: **94 catalog codes** — 59 case-owned, 35 ratcheted lock-only. Message
registers: 14 `None`, 74 `sm `-migrated, 6 authored. `help`: 7 `Some`, 87 `None`. 22 messages carry
a `{{detail}}` passthrough hole, of which 16 are the pure form `sm {{detail}}`.

### The 59 case-owned codes

| bucket | count | verdict | root |
|---|---|---|---|
| **editable-today** | **34** | an ordinary word edit to the message compiles green | — |
| reflow-blocked | 6 | `MarkerOutsideEditableSection` | K1–K6 (§1.2) |
| placeholder-blocked | 14 | `MarkerOutsideEditableSection` | K8 (§1.1) |
| passthrough-blocked | 5 | `MarkerOutsideEditableSection` | the whole visible sentence is `ForeignText` |

The 34 editable-today: `authored-decline-class` · `authored-decline-class-unreadable` ·
`cli-file-not-found` · `cli-file-permission-denied` · `cli-file-unreadable` · `cli-flag-needs-value`
· `cli-flag-requires-mode` · `cli-flag-value-not-a-number` · `cli-flag-value-not-recognized` ·
`cli-flags-mutually-exclusive` · `cli-no-book-given` · `cli-shim-dir-unwritable` ·
`cli-strip-got-a-flag` · `cli-strip-needs-path` · `cli-unknown-flag` ·
`cli-unknown-flag-did-you-mean` · `cli-unknown-mode` · `cmdsub-operand-top` · `dangling-reference`
· `dorc-sh-exec-failed` · `dorc-sh-script-unreadable` · `dorc-sh-usage` · `lint-file-count-drift` ·
`lint-no-lintable-files` · `lint-required-tools-missing` · `lint-tool-absent` ·
`lint-tool-failed-without-findings` · `lint-tool-output-unparsable` · `missing-dialect-marker` ·
`munge-name-invalid` · `records-fact-truncated` · `tolerates-unknown-dimension` ·
`unmodeled-wall-inventory` · `verdict-terminal-pipeline`.

The 5 passthrough-blocked (case-owned, pure `sm {{detail}}`): `site-unresolvable` ·
`syntax-unsupported` · `escalation-policy` · `carried-across-substrate-axis` ·
`wrapper-peel-incoherent`. **fnd-passthrough-prose-has-no-authored-words** (+SURE): for these the
ONLY authored bytes in the render are the literal `sm ` prefix; every visible word is
`RenderPart::ForeignText` and therefore immutable by law
(`282:rul-passthrough-type-gated`). Editing them is not a transport problem — it is the
de-passthrough work (`288` §6's "opaque sibling lane, `284`"). **A loom will never be able to
edit these sentences until the emit sites stop composing prose into `detail`.** Corpus-wide there
are 16 such codes (11 more are ratcheted).

Partial-passthrough is a quieter version of the same: `cli-file-unreadable`,
`cli-shim-dir-unwritable`, `dorc-sh-exec-failed`, `dorc-sh-script-unreadable`, `whylog-corrupt`,
`aid-unloaded-sibling-oracle` each have `{{detail}}` mid-message, and `ForeignText` closes the
section — so the message SPLITS and only the pre-`detail` run is editable. Verified for
`cli-file-unreadable`: the compiled interpretation ends at the hole
(`sm probeword read {{kind}} \`{{path}}\`: `).

### The 35 ratcheted codes

Zero edit-paths, by construction: no case ⇒ no transcript ⇒ no render ⇒ no provenance. Triage in
§3a.

### Arrangement registry

135 entries / 109 distinct slugs (118 `why-*`, 12 `lint-*`, 5 `cli-*`); 22 `Words::Unwritten`,
112 `Words::Migrated`, 1 `Words::Authored` (`cli-help-page`).

Faced today (measured, not inferred): `cli-help-page` (page path) · `lint-advisory-preamble`
(green) · `lint-summary-sentence` (faced but value-locked — see below) · the ~9 receipt/drift rows
the two `why-drift-*.loom` cases stamp, of which 2 are `AmbiguousCandidate`-blocked (§0.5). So
**~11 of 135 entries have a face; ~124 are lock-only.** ~SUSPECT on the exact count: there is no
tool that enumerates a case's editable sections — see `ask-sections-command-is-the-census-tool`
in §2f.

**fnd-computed-words-are-invisible-to-an-author** (+SURE, reproduced): editing `error` in
`dorc lint: 1 error, 0 warnings, 0 infos across 1 file.` refuses

```
apply compiled section: ArrangementValueSequenceChanged { slug: "lint-summary-sentence",
  expected: ["v0","v1","v2","v3","v4","v5","v6","v7"], found: ["v0","v2",...,"v7"] }
```

because the pluralized noun is a computed VALUE (`289` §2b's `plural()` helper), not a word.
Correct refusal; zero discoverability — nothing in the transcript distinguishes the computed
`error` from the authored `across`. This is §2f's motivating case as much as §5's.

---

# §2 — map-fix-specs

Each spec is an instance of §0.4/§0.5. Sizes are ~SUSPECT unless marked.

## §2a — spec-placeholder-words-mint (Tier 1, the crux)

**Files/types.** `aid/src/diag.rs` `render_body_parts_with` (`:2759-2827`) — the `None` arm at
`:2786` and the `help` arm at `:2792`. `aid/src/tagged.rs` — no new variant needed.
`dorc-loom/src/lib.rs` `OpenSections` — no change. `dorc-loom/src/consumer.rs`
`apply_compiled_section` — the catalog applier must accept a section whose owner register is
currently `None` (it already does: `authored-decline-class`'s edit lands on a `Some` row; check the
`None` path).

**Shape.** Pattern-cite `28H` span ruling 4 (`ask-unwritten-placeholder-stays-editable`): *the
placeholder text stays computed (never a stored row), but its span keeps the row's face.* Mint the
catalog placeholder as a `TemplateLiteral`-class run keyed `(code, Field::Message, instance)` —
the same key an authored message would carry — so `to_editable_render` opens the message section
over it. The stored register stays `None`; the transcript edit is the WORDS-MINT path. This is
literally `said.rs:178-188` transplanted to the catalog seat, so the two registers finally behave
the same way, which is itself an argument for the change (`289:rul-arrangement-home-is-registry-plus-transcripts`
made them siblings; today they diverge).

**The `help: None` story.** This is the genuinely open design question, and
fnd-added-help-is-silently-absorbed forces an answer. Three shapes, argued from `two-surfaces`
(the transcript must be byte-identical to the production render):

- **h-render-a-help-placeholder-always** — every case-owned code renders
  `\n  = help: [unwritten: <slug>.help]` when `help` is `None`, faced like the message. Cost: a
  visible placeholder in PRODUCTION output for 87 codes. Rejected — it ships debt to users and
  contradicts `289` §2u's ruling that `help: None` is *completeness, not debt*.
- **h-scaffold-affordance** — `dorc-loom` grows an explicit `add-register CASE help` (or a
  frontmatter `wants: help` key) that seeds the register with a placeholder, after which the
  ordinary transcript loop edits it. The transcript stays byte-identical to production at all
  times; the author's discovery path is a named command, and the refusal below names it. **This is
  the recommendation.**
- **h-accept-a-typed-help-line** — teach the transport that a `= help:`-shaped line appearing where
  the render has none MINTS the help register. Rejected outright: it is byte-shape re-detection,
  i.e. the exact thing §0.4 outlaws.

Whichever is ruled, **the absorption must become a refusal first** — that half is not optional and
is not a design question. Mechanism: the message section's editable interior must end at the
render's own message-run boundary; today the trailing `"\n"` structure part is the only anchor and
anything before it is fair game. Concretely, emit a `RenderPart::Arrangement` line-terminator after
the message run (not only at the end of the whole render), so a section can never absorb a line
the renderer did not emit.

**Law interactions.** `282:rul-new-code-empty-loop` (this IS the missing half of the empty loop) ·
`27V:rul-error-authorship-tier` + `aid/CLAUDE.md` prose-three-state (unchanged: the register stays
`None` until words land) · `aid/CLAUDE.md` only-registry-bytes-are-editable (extends to "and the
computed placeholder wears its register's face") · `28F:rul-placeholders-are-computed` (respected —
nothing is stored) · `two-surfaces` (production bytes unchanged in the recommended shape).

**Test plan.** (i) overtype-placeholder round-trip for one `message: None` code, landing (per
asm-prose-authorship-bent-this-arc) — compile → promote → lock row flips `None`→`Some` → fixpoint
green; (ii) a refusal pin: an added `= help:` under a `help: None` code refuses with a message
naming the register-adding command; (iii) a byte-identity pin that production render bytes are
unchanged for all 14 unwritten codes (the placeholder text must not move).

**Risk.** Low. The change is one part-kind at one seat. The one real hazard is that
`spanless_mint_allow_list_is_exact` and `message_registers_are_sm_or_unwritten` both key on the
`None` state — both stay true.

**Size.** Small (½–1 day). Judgment work.

## §2b — spec-reflow-help-swallow → subsumed by prop-wrap-inside-the-render-seat

**Preferred (§0.4).** Move the canonical wrap into `render_cli_parts`, delete K1–K6.

- Files: `aid/src/diag.rs` (`render_cli_parts`, `render_staged_cli_parts`, `render_body_parts_with`)
  — wrap at a named `DIAGNOSTIC_WIDTH` const, emitting each wrapped run as its own part with the
  same key as its source run; `dorc-loom/src/consumer.rs` — delete `reflow_to_canonical`/
  `wrap_words`/`CANONICAL_WIDTH`; `dorc-loom/src/bin/dorc-loom.rs` — delete `unreflow`/
  `join_continuations`/`normalize_layout`/`is_caret_gutter` and the `unreflow(block.output())` call
  at `:524`.
- Corpus effect: every committed aid transcript re-blesses (the wrap point may move by a character
  where the hanging indent differs); every e2e `expected.out` carrying a plan-stderr diagnostic
  re-blesses. **Goldens churn freely** is already ruled (human standing ruling); promote FIRST,
  then rebuild, then bless (`two-bless-paths-split-by-directory`).
- Sibling word/whitespace bugs swept in the same lane (the `28H` lexical-judgment inventory's
  named families): leading-space drop at a line start · the transcript-edit trailing `\n` ·
  the absorption/collapse pair · zero-width-run divergence between the catalog and weft paths.
  All four are downstream of "two byte-forms"; the inventory deliverable `28H` banked for r30
  becomes cheap once one form remains.
- Standing 28H law applies: **any surviving word/whitespace judgment lands NAMED, one place per
  crate** — here that is exactly one place (the render seat's wrapper).
- Size: medium (2–3 days), of which the re-bless is mechanical churn. Judgment work: the wrap
  algorithm's placement relative to `ParamValue`/`ForeignText` runs (a value must never be
  re-spaced; a wrap may only fall between runs or inside a `TemplateLiteral`).

**Bridge (only if §0.4 slips).** The one-line predicate patch at `dorc-loom.rs:804`, proven
sufficient for today's six (§1.2), green under `test:looms` and `cargo test -p dorc-loom`. Land it
ONLY as an explicitly-labelled bridge with a deletion date, because it is another entry on the
edge-detection list the human ordered removed.

## §2c — spec-whylog-lint-provenance

**Files.** `dorc-loom/src/consumer.rs:414-433` — `editable_baseline` gains the same
`.or_else(|_| Self::whylog_diagnostic(case))` fallback `case_diag` (`:618`) and
`baseline_from_render` (`:567-569`) already carry, plus the `render_staged_cli_parts` seat for the
whylog stage prefix (`:496`) so the render matches the driven one.

**Better shape** (recommended): `editable_baseline` should not re-derive the world at all — it
should call `replay()` for the case's first block and hand the result to `baseline_from_render`.
That collapses two world-derivation paths into one and makes the `vars` command answer *the same*
render compile sees, which is the property `282:rul-used-inventory-is-committed` actually needs.
~SUSPECT this is ~40 lines net-negative.

**Also unblocks.** The committed `dorc-loom vars --used CASE` replay block per defining case
(`282:rul-used-inventory-is-committed`, deferred in `287` §11) — the second half of that deferral
was `render_direct_replay` support for the `vars` command, which `replay()` already has
(`consumer.rs:448-471`) but `render_direct_replay` does not. Add the arm; then regenerate all 59
cases.

**Test plan.** `vars --used`/`--all` answers for all 62 cases (a loop assertion, non-empty floor,
never a count); the 4 whylog cases specifically pinned; one committed `vars --used` block
round-trips through the fixpoint.

**Risk.** Low-medium: regenerating 59 cases with a new committed block is a large but mechanical
diff. Size: small-medium (1 day), mostly churn.

## §2d — spec-mirror-threading

**The estimate is wrong, downward.** `28H:finding-why-render-reads-the-const-not-the-mirror`
priced this at "~60 seats". Measured: `CONST_ARRANGEMENTS` is read at **five** places —
`aid/src/said.rs:129` (`Said::text`), `:180` (`sentence_runs`), `:226` (`words_text`), and
`cli/src/lib.rs:49`/`:56` (`usage_text`/`help_text`, which are deliberately faceless per
`a-registry-row-need-not-mint-a-span`). The `~60` figure is the count of `Said::Words`/`sentence`
CONSTRUCTION sites (87 across `cli/src/main.rs`, `cli/src/lib.rs`, `aid/src/diag.rs`) — but those
do not need the lookup.

**Seam.** Give the lookup to the RENDER seats, not the constructors: `Said::runs(&self, part,
lookup: &dyn ArrangementLookup)` and `Said::text(&self, lookup)`. Production passes
`&CONST_ARRANGEMENTS`; `dorc-loom` passes its mirror. Call sites needing the parameter: **17**
production `.runs(` seats (3 in `cli/src/lib.rs`, 14 in `cli/src/main.rs`) plus a handful of
`.text()` seats. Alternative with zero signature churn: a `ConstArrangements` that consults a
thread-local mirror — rejected (`inv-determinism`, and hidden state in a render seat is exactly the
kind of thing this project types away).

**Payoff.** One-step why-row authoring: edit → compile → promote → re-render, with no intermediate
red and no rebuild. Removes the interaction with the blast-radius dirty gate (§4) that currently
forces an intermediate commit.

**Size.** Small (½–1 day). Mostly mechanical (signature threading), one judgment call:
**tc-lookup-parameter-vs-render-context** — a bare `&dyn ArrangementLookup` parameter now, or a
`RenderCtx` struct that will also carry the width from §2b? Flagged UP; my lean is the struct,
because §2b introduces a second render-time parameter and threading twice is churn twice.

## §2e — spec-roundtrip-tests (the acceptance battery)

Precedent: `dorc-loom/tests/` already owns `arrangement_prose.rs`, `arrangement_lines.rs`,
`compact_line_prose.rs`, `consumer.rs` — in-process tests over `DorcConsumer` with temp-corpus
fixtures. Extend there; **do not** land builder prose in committed `crates/aid/tests/*.loom` cases
(exception: the two sanctioned dogfood landings named in §7).

| test | shape | pins |
|---|---|---|
| `overtype_placeholder_mints_words` | temp copy of a `message: None` case; compile; assert compiled series | §2a |
| `added_help_line_refuses_and_names_the_command` | assert the refusal CLASS and that its text contains the repair command | fnd-added-help-is-silently-absorbed |
| `help_register_edit_round_trips` | edit help text on `whylog-absent`-shaped fixture | §2b |
| `every_committed_case_survives_a_word_edit` | corpus loop: for each case, replace one authored literal word, assert compile accepts or refuses with a CLASSIFIED, expected reason | the §1.4 matrix, as a ratchet |
| `variable_insert_move_delete_duplicate` | four probes on one fixture (`{{name}}` insert from `--all`, move, omit, duplicate) | `282` §13 |
| `glued_marker_refuses_until_phase_5` | backticked `` `{{flag}}` `` ⇒ `AttachedMarker` | §5, and flips to green when §5 lands |
| `intermediate_section_edit_is_addressable` | a 3-section render whose immutable anchor recurs | §0.5 / K7 |
| `one_step_why_row_edit` | promote a why arrangement row and re-render without rebuilding | §2d |
| `transcript_bytes_equal_production_bytes` | for every aid case, the committed transcript == the render seat's bytes | §0.4 — **this is the law's mechanical enforcement** |

The last row is the important one: it makes rul-editability-is-stamped-never-re-derived a gate, not
a promise. `28J`'s "goldens churn freely" ruling covers the re-bless it forces.

Note `282:rul-strawman-formats-no-compat` explicitly licenses breaking `.loom`, lock, and receipt
formats — several of the above are cheaper if the receipt gains a section-boundary record.

## §2f — spec-variable-core-semantic (NEW, per the re-charter)

### The test: "an author reading a loom wants a value in their sentence — what must they touch, in what order, and how do they DISCOVER each step from where they are standing?"

**Today, for a value the payload already carries** (the good case):

1. The author is looking at `crates/aid/tests/<slug>.loom`. Nothing in the file names its variables.
2. They must know that `dorc-loom vars --all <case>` exists. It is not in `mise tasks`
   (`loom:compile` and `loom:promote` are the only loom tasks), not in the file, and not in any
   refusal. **Discovery score: 0.**
3. They type `{{name}}` into the sentence. If they glue it to punctuation — which the corpus idiom
   demands, 26 of 94 messages backtick-quote a variable — they get
   `Compile(AttachedMarker(TemplateVariableName("flag")))`. The refusal names the variable but not
   the workaround, and the workaround is "don't do that, put it in the middle of a word run".
4. `mise run loom:compile` prints the interpreted series — genuinely good, and the one place the
   author can see which of their words the compiler took as a hole.
5. `mise run loom:promote` writes the lock; `git --no-pager diff --word-diff` is in the task, but
   `MISE_TASK_OUTPUT=timed` eats the preview (§4).

**Today, for a value the payload does NOT carry**: `{{whatever}}` refuses `UnknownParam`. The
refusal does not say "this value does not exist on this diagnostic's payload; adding one is a Rust
change in `aid/src/diag.rs`'s payload struct and `params_of`". The author has no path.

**Today, for a value that is rendered but is not a variable** (a computed arrangement value, e.g.
`lint-summary-sentence`'s pluralized noun): the author sees an ordinary English word, edits it, and
gets `ArrangementValueSequenceChanged` — a refusal whose text names v-indices, not the word they
touched.

### prop-variables-are-declared-by-the-render-not-by-a-table

The honest core semantic already exists and is not exposed: **the render seat is the definition
site.** `params_of` (`aid/src/diag.rs:2249+`) is a per-payload match that names every value a code
can interpolate; `arrangement_variable(index)` (`dorc-loom/src/lib.rs:90`) names every value a
chrome line interleaves. Both are total, both are derivable, and neither is visible to an author.
Nothing new needs to be invented — it needs a SURFACE.

Proposed shape (conductor to rule):

- **v-in-file-inventory** — every defining case commits a generated `$ dorc-loom vars --used CASE`
  block, exactly as `282:rul-used-inventory-is-committed` already rules and `287` §11 deferred. The
  author standing in the file sees the variable names and their exact current values without
  leaving it. This is the single highest-value discoverability item in the whole map, it is already
  ruled, and §2c unblocks it.
- **v-refusals-name-the-next-command** — `UnknownParam` says which command lists the available
  ones (`dorc-loom vars --all <case>`) and, when the name matches nothing, that new values are a
  Rust payload change with the file to open. `AttachedMarker` says the supported spelling.
  `ArrangementValueSequenceChanged` says "`error` is a value this line computes, not a word you can
  edit; the words are: …".
- **v-values-are-visibly-values** — the `vars --used` block is the affordance that makes a computed
  word legible *before* the author edits it. No in-band marking in the transcript (that would break
  `two-surfaces`); the sidecar block is the whole answer.
- **v-sections-command** — `dorc-loom sections CASE` printing every editable section key and its
  `Text | Variable` series. This is simultaneously the author's "what can I actually edit here?"
  answer, the census tool this map lacked (§1.4's ~SUSPECT count), and the debugging tool for every
  transport refusal. Cheap: the data is already in `DorcEditableBaseline`.
- **the honest boundary** — a NEW payload field is Rust: `aid/src/diag.rs` payload struct +
  `params_of` arm + the emit site. That is accepted and correct (`asm-remit-is-the-edit-surface`
  forbids new text-emitting machinery in core Dorc, not new typed payload fields). The requirement
  is only that the path be LEGIBLE and that the refusal name it.

### §2f.2 — The Rust-side walk, MEASURED against rul-rust-and-loom-are-the-only-edit-surfaces

Method: I added an ordinary `pub probe_field: String` to `dorc_aid::diag::CliFileNotFound` and
followed the compiler until `cargo check --workspace --all-targets` was green, then reverted. This
is the literal "an experienced Rust dev adds a contextual value to an existing diagnostic" act.

**The measured edit set:**

| # | file | why | surface |
|---|---|---|---|
| 1 | `aid/src/diag.rs` — the payload struct | the value itself | **rust** ✅ |
| 2 | `aid/src/diag.rs` — the `params_of_raw` arm (`:2321`) | makes it loom-visible | **rust** ✅ — but see fnd-params-arm-is-not-forced |
| 3 | `cli/src/lib.rs:659` — the mint site | `E0063 missing field` at the error-is-happening site | **rust** ✅ (exactly the "clean API interaction AT THAT SITE" the law wants) |
| 4 | **`dorc-loom/src/consumer.rs:1420` — `canonical_payload`** | `E0063 missing field` | **LOOM INTERNALS** ❌ |

Then, with those four, the value flows through the black box with **zero further steps**:

```
$ cargo run -q -p dorc-loom -- vars --all crates/aid/tests/cli-file-not-found.loom
case: crates/aid/tests/cli-file-not-found.loom
{{kind}} = "book"
{{path}} = "webhost.sh"
{{probe_field}} = ""          <- appeared with no dorc-loom edit beyond the E0063 repair
```

`available_values` (`dorc-loom/src/edit.rs:368`) is `all_variables ∪ section-rendered`, and
`all_variables` is `params_of(...)` filtered by `is_foreign_param` (`consumer.rs:423-427`) — fully
generic, no per-field or per-type touch. `params` derive from compiled holes; `example` re-derives
from the compiled message and the defining payload at promote (`287` §11). **So consequence (1) of
the law is 3-steps-out-of-4 already satisfied**, and the whole gap is one function.

### The findings

- **fnd-canonical-payload-forces-a-loom-edit** (+SURE, reproduced) — `DorcConsumer::canonical_payload`
  (`dorc-loom/src/consumer.rs:~1380-1480`) hand-constructs payload structs for **29 of 94** slugs.
  Adding or renaming any field on those payloads is a hard compile error inside the loom crate. A
  Rust dev cannot finish their act without editing loom internals, which the law forbids outright.
  *Fix-spec* — **spec-worlds-move-to-the-diag-api**: the canonical stand-in payload belongs beside
  the payload TYPE, not in the consumer. Two shapes: (a) a `#[cfg(feature = "fixture")]` (or
  ordinary, gated) `CliFileNotFound::fixture()`-style constructor per payload in `aid/src/diag.rs`,
  with `canonical_payload` reduced to a slug→constructor dispatch that the compiler still forces to
  be total; (b) better, derive the stand-in from the DEFINING CASE rather than from Rust at all —
  `289:rul-worldless-route-honest-trigger` already prefers honest firing, and §3a shows every
  remaining stand-in can become a real firing with three harness additions. Shape (b) retires
  `canonical_payload` entirely and is the recommendation; (a) is the bridge if (b) slips. Note the
  same mechanism hits `aid/tests/catalog_defining_cases.rs::covered()` (23 hand-built payloads) —
  that one is inside `aid`, so it is a lesser violation, but it is the same
  "manual step against an intermediary artifact" and should die with the same fix.
- **fnd-params-arm-is-not-forced** (+SURE, reproduced) — a payload field with no `params_of_raw`
  arm compiles green and is **silently invisible to looms**: `vars --all` never lists it and
  `{{probe_field}}` refuses `UnknownParam`. Nothing in the type, the derive, or the compiler
  connects a payload field to its loom name. *Fix-spec* — **spec-payload-fields-declare-their-name**:
  either (a) a `diag_tidy`-style census gate asserting every public payload field appears in
  `params_of_raw` (cheap; lexical, in the same family as `spanless_mint_allow_list_is_exact`), or
  (b) make the connection structural so the compiler forces it. (b) is the right answer and is
  ordinarily a derive macro — **which `inv-no-unsafe`'s sibling law forbids** (`spike/CLAUDE.md`:
  "No authored macros (`macro_rules!`/proc-macros); standard `#[derive(...)]`s encouraged"). So
  (a), the census gate, is the available answer. **tc-payload-param-binding-wants-a-macro-we-cannot-write**
  — flagged UP.
- **fnd-foreign-param-is-a-name-convention** (+SURE, by reading) — `is_foreign_param(param)` is
  literally `param == "detail"` (`aid/src/catalog.rs:246-248`). A Rust dev who names a new field
  `detail` silently makes it immutable-foreign; one who puts host-sourced bytes in a NAMED param
  silently skips `display::encode_line` (`aid/src/diag.rs:2215`), because only foreign params are
  encoded. The rule ("named params are OURS; host bytes ride `detail`") is stated in `params_of`'s
  rustdoc — the right seat — but it is convention, not type. *Fix-spec* — reuse the `282` §8
  type-gated user-sourced text work (`x2-de-passthrough`): once foreign text has a TYPE, the name
  convention dies and the encode seat becomes total by construction.
- **fnd-spanless-lexical-gate-is-taught-only-to-agents** (~SUSPECT on impact) —
  `Diag::new_spanless_site`'s rustdoc (`aid/src/diag.rs:2120-2137`) does name the gate and the
  allow-list, which is the correct teaching seat, but (i) it points at `core/tests/diag_tidy.rs`,
  a path that no longer exists (the file moved to `aid/tests/` at `288:phase-aid-crate-extraction`)
  — stale doc; and (ii) it does not carry the LEXICAL constraint that the mint must spell
  `new_spanless_site(DiagCode::X(` literally, never through a helper — that lives only in
  `spike/crates/aid/CLAUDE.md` spanless-gate-is-lexical, an agent-steering file the Rust persona
  has no reason to read. *Fix-spec*: move both facts into the rustdoc; the CLAUDE.md bullet stays
  as a duplicate for agents.

### §2f.3 — Verdict against the law

Consequence (1) — "Rust act, then automatic": **one violation, one function, fixable.** The
resolver, the inventory, `params`, and `example` are already generic; nothing is per-field or
per-type. Consequence (2) — "loom editor goes to the Diag API": **blocked on discoverability, not
on architecture.** The path exists and is short; nothing tells the author it exists (the
`UnknownParam` refusal says nothing about payloads, `aid`, or Rust). Both halves land inside X1.

**tc-variable-surface-is-a-command-or-a-file** — flagged UP: does the in-file inventory (a committed
replay block) suffice, or does the human want a first-class `dorc-loom` browse surface? My lean:
the committed block, because it satisfies the naive-reviewer gate (§8) with zero prior knowledge.

**Size.** v-in-file-inventory rides §2c (churn). v-refusals + v-sections: small (1 day), judgment.
spec-worlds-move-to-the-diag-api: shape (a) small [C]; shape (b) rides x1-reach-cheap/x2-reach-hard.
spec-payload-fields-declare-their-name: small [J]. Rustdoc repairs: trivial [C].

---

# §3 — map-reach-triage

## §3a — The 35 ratcheted codes: can a defining case fire each?

Two harness facts govern everything here, both verified by reading:

- **fnd-book-route-sees-only-effect-diags** (+SURE) — `fire_book_analysis`
  (`dorc-loom/src/consumer.rs:1521`) runs `parse → cfg::build → value::analyze →
  effect::classify` and searches **only `effect::classify(...).diags`**. `parsed.diags` and
  `cfg.diags` are discarded. It also passes an EMPTY oracle set (`&[]`, default `KindIndex`,
  default `VerdictIndex`).
- **fnd-plan-route-never-reads-its-results** (+SURE) — `parse_direct_plan` accepts a `< file`
  redirect and `replay()` checks the file EXISTS (`consumer.rs:532-534`), but the bytes are never
  parsed: `world_of_source` takes only the book. So no records-lane code can fire.

Triage:

| bucket | codes | count | support needed |
|---|---|---|---|
| **trivially-fireable via `dorc lint <oracle>`** — the lint pipeline already runs oracle validate/predict/entry/wrapper/carry/reserved | `predict-out-of-dialect` · `predict-unterminated` · `munge-name-collision` · `reserved-namespace-squat` · `tolerates-over-identity-dependence` · `heavy-context-no-tolerance` · `lend-map-unknown-dimension` · `carry-netns-on-net-kernel-forbidden` · `mark-brace-verdict-single-cell` · `wrapper-entry-incoherent` · `footprint-incoherent` | 11 | none — `scaffold` + author a triggering `oracle.sh` section. Each ratchet entry already carries its trigger note. |
| **needs-harness-support: collect the parse/CFG diags** | `syntax-malformed` · `cfg-top-node` · `cfg-errexit-unknown` · `cfg-inline-refused` · `cfg-builtin-shadowed` · `depth-2-positional-unthreaded` | 6 | 3-line change: `fire_book_analysis` searches `parsed.diags ∪ cfg.diags ∪ effect.diags` |
| **fireable today via `dorc plan --book`** (effect-plane) | `cmdsub-inner-nonleaf` · `effect-kind-disagreement` · `redir-target-top` | 3 | none (`redir-target-top` also emits from `plan/src/erasability.rs`; ~SUSPECT the effect-plane arm reaches) |
| **needs-harness-support: load oracles in the book route** | `resolver-conflict` · `resolver-provider-collision` · `reaches-conflict` · `reaches-provider-collision` · `deriv-family-incomplete` · `touches-escalated` · `wrapped-site-adoption-hint` | 7 | `fire_book_analysis` must accept `*.oracle.sh` sections and build the `KindIndex`/`VerdictIndex` the same way `cli/src/main.rs` does. Medium: this is the "run the real pipeline over a materialized world" step the corpus has been deferring. |
| **needs-harness-support: feed probe-results into the fold** | `records-headerless-refused` · `records-glued-line` · `records-header-missing` · `records-sentinel-nonce` · `records-integrity-refused` · `records-torn-line` · `records-alien-line` · `records-late-line` | 8 | thread the `< probe-results.txt` bytes into a `dorc_plan::records` admission call. Small-medium, and it makes the `< file` syntax honest rather than decorative. |
| **cannot-fire-because-X** | — | 0 | none found |

**No ratcheted code is structurally unreachable.** The blockers are three named harness additions
(parse/CFG diag collection · oracle loading in the book route · records intake), together well
under a lane. `dorc-loom scaffold <slug>` (`dorc-loom.rs:127`) already writes the skeleton and
prints the next step; the same-slug coherence gate keeps a scaffolded-and-forgotten case red.

**tc-fixture-world-source** — flagged UP (see also §2f, which raises the same question from the
Rust side): `289:rul-worldless-route-honest-trigger` prefers real firing, and `canonical_payload`
(29 hand-built stand-ins) is the fallback. For the 15 needing harness support, is the answer "build
the harness support" (my lean — it retires stand-ins rather than adding them, AND it is the fix for
fnd-canonical-payload-forces-a-loom-edit) or "add 15 more `canonical_payload` arms" (cheap, but
grows the thing `287` calls a transitional twin and deepens the black-box violation)?

## §3b — Arrangement faceless rows: the face strategy

**The full why-driver extraction, priced.** `emit_why_report` (`cli/src/main.rs:5060`) takes **17**
arguments (the brief said 16; recount stands) and is fed by `run()`, which is **971 lines**
(`cli/src/main.rs`). `28H` items 9–11 stopped here deliberately and the conductor adopted
`prop-drifted-why-is-the-thin-driver` instead — a ~285-line extraction that bought the two
`why-drift-*.loom` cases and ~9 faced rows.

Cost of the FULL extraction: move the report-assembly closure out of `main.rs` into
`cli/src/lib.rs` behind the existing `lib-target-is-a-loom-seam` law, with every I/O edge staying in
`main.rs` (`VALUES cross the seam, QUERIES do not` — the `SourceMatch` precedent). The 17 arguments
are the measure: they are all pure data except `arena`/`interner`, so the extraction is code motion
plus one context struct. Estimate: **1 lane, 400–600 lines moved, high corpus byte-risk** (the why
transcripts re-bless). Rows it faces: the `why-*` chain families —
`why-reason-*` (9 rows) · `why-declines-*` (13) · `why-next-step*` (11) · `why-outcome-*` (9) ·
`why-vouch-payload-*`/`why-claims-*`/`why-derives-*` (5) · `why-tier-word` (7) ·
`why-participating-lines-*` (2) — i.e. **most of the 118 `why-*` entries**, which is most of the
registry.

**Rows that could be faced cheaply, without the driver extraction:**

- `lint-*` (12): `lint-advisory-preamble` and `lint-summary-sentence` are ALREADY faced through the
  existing aid lint cases (measured). `lint-source-*` (8) and `lint-fidelity-*` (2) need a
  `dorc lint --list-sources`-shaped replay or a `--verbose` lint case — small.
- `lint-clean-sentence`: needs a clean-lint aid case; the whole-product `lint-clean-run.loom`
  exists in `cli/tests` but is not a dorc-loom-compiled case. Small.
- `cli-plan-summary-line`, `cli-decision-digest-line`, `cli-why-pointer-line`: these are plan-stderr
  chrome. A `dorc plan` aid case that renders stderr would face them — but the aid `dorc plan`
  route renders only the DIAGNOSTIC, not the stderr envelope. Needs a route, not a driver: small-
  medium.
- `cli-usage-synopsis`: seat-appended to invocation errors; `289` §2p banked that lint argument
  errors carry no usage line. ~SUSPECT it is unfaced today; a `dorc` invocation-error case that
  renders the synopsis would face it. Small.

**Rows that genuinely stay lock-tier**, and why: the `usage_text`/`help_text` plain-text seats
(`cli/src/lib.rs:49,56`) read registry words as TEXT rather than stamping spans, per
`a-registry-row-need-not-mint-a-span`. That is per-seat facelessness by design — `cli-help-page` is
faced via the PAGE path, and `cli-usage-synopsis` reads as text where it is appended. Do not "fix"
a plain-text seat by stamping a span its surface's transport cannot anchor.

**Recommended cut**: do the cheap lint/cli faces in X1 (they are a handful of new cases), and take
the full driver extraction as the X2 opener. Answers `28H:ask-full-driver-this-arc-or-r30` in the
direction of *this arc*, because the arc-close invariant demands it: without it, ~100 registry rows
have no editable loom and the `288` §8 invariant is unmet by construction.

---

# §4 — map-friction-batch (go/no-go + size)

| friction (`28H`) | verdict | size | note |
|---|---|---|---|
| **blast-radius-scoped dirty gate** | **GO** | small | Measured live: a source edit anywhere in the repo makes `compile` refuse `dirty path outside selected prose edits: spike/crates/dorc-loom/src/bin/dorc-loom.rs`. I had to make a throwaway commit to run a diagnostic. Scope the refusal to touched looms + the two locks (`repository.rs:174-180`); repo dirt outside the blast radius is ceremony. Interacts with §2d: the const-vs-mirror lag currently forces an intermediate commit, which this gate then fights. |
| **structure-bless as a first-class path** | **GO** | medium | `282` §6 designs it; as-built the only route for an input-section edit is the blind `DORC_LOOM_DUMP` two-step (`aid/CLAUDE.md` authoring-a-replay-block-is-blind). Make it a compile MODE: re-drive, re-render, re-anchor surviving prose spans, show the diff. Mixed prose+input edits in one file refuse with the exclusivity message naming both paths. This is also what makes a scaffolded case's first fill non-blind — directly serving §8. |
| **promote preview swallowed by `MISE_TASK_OUTPUT=timed`** | **GO** | trivial | Reproduced: `mise run loom:compile` printed only `compile: 62 cases, 1 refused`, while the raw `cargo run` printed the full refusal evidence. Fix in the task, not at the call site (`never-filter-a-task`): make the loom tasks' interesting output unswallowable, or add `loom:compile:verbose`/`loom:promote:verbose`. |
| **scaffold mise task** | **GO** | trivial | `mise tasks` has `loom:compile` and `loom:promote` only. Add `loom:scaffold` and `loom:vars`. Zero-cost, and it is 100% of the discoverability story for §8. |
| **transcript-edit trailing `\n`** | **GO — trim** | trivial | Proposed argument: the stored register is WORDS (`282:rul-words-and-paragraphs-only`); a trailing newline is layout, and layout is the renderer's. Trim at read-in, in the ONE named significance seat (28H standing law). Fixpoint argument: trimming is idempotent and the generator re-emits the newline from layout, so the trim is a generator fixpoint — provable by the existing byte-identity gate. Do it before the human's rows diverge in shape. |
| **arity-slip compile-time refusal** | **GO** | small | Today an N-value row seeded with N+2 words panics in debug and degrades to `[unwritten:]` in release. The arity is knowable at compile time from `when_used`; refuse there. Related: `ArrangementValueSequenceChanged` is the runtime cousin and should carry the same explanation (§2f v-refusals). |
| **`fixpoint: executed` visibility marker** | **GO (cheap form)** | trivial | The looms runner prints a green trial for a `run:` loom whose transcript it did NOT fixpoint. libtest-mimic fights an `ok (deferred)` status; the cheap honest form is to name the trial differently (`<case> [deferred to e2e]`). |
| **breadth-vs-first-failure** | **NO-GO this arc** | medium | Gate short-circuiting reports one failure where a design red-line carries five. Real, but it is test-harness ergonomics, not an edit-path blocker; it does not stand between the human and a loom edit. Bank for r30. |
| **`AmbiguousCandidate` multi-section residue** | **GO — promote to Tier 1** | medium | NOT a friction any more: measured biting two of `28J`'s top-10 rows today (§0.5). It is K7 on the §0 kill-list and must land with §2b. |

---

# §5 — map-adjacent-fragment (`282:phase-adjacent-fragment-followup`)

**Reproduced.** Two shapes, both refused:

```
# duplicate a variable in parentheses
error[cli-flag-requires-mode]: sm --whylog is only valid with dorc why ({{flag}})
  -> refusal in replay 0: Compile(AttachedMarker(TemplateVariableName("flag")))

# the corpus's own idiom: quote the variable in backticks
error[cli-flag-requires-mode]: sm the flag `{{flag}}` is only valid with dorc why
  -> refusal in replay 0: Compile(AttachedMarker(TemplateVariableName("flag")))
```

**Blast radius, measured: 26 of 94 catalog messages already backtick-quote a variable.** So the
dominant house style for naming a flag, a path, a tool, or a command is the exact spelling an
author cannot newly write. Any new code minted from here inherits the trap.

**Does landing it threaten the transport invariants?** Assessed against the
`282` §13 rul-untouched-variable-preservation family:

- `282:rul-untouched-variable-preservation` — unaffected: it governs variables the author did NOT
  touch, identified before tokenization. A newly-typed attached marker is by definition touched.
- `282:rul-rendered-variable-offsets-may-move` — unaffected: it already permits an untouched
  backticked value to move (`287` Unit 1 acceptance pins exactly that).
- `282:rul-rehole-deliberately-stupid` — the risk lives HERE. Allowing attached markers means the
  compiled series can contain a `Variable` with no whitespace anchor on either side, so the
  *reverse* direction (re-holing a rendered value back into a marker) loses its anchor. Mitigation
  is already ruled: re-holing is an AID, and destroyed anchors require an explicit `{{name}}`. So
  attached markers are safe to WRITE as long as re-holing does not try to discover them.
- `282:rul-double-brace-template-only` — the grammar is already "one whole token, no attached
  punctuation". Landing this **changes that rule**, which is a `282` §13 amendment, not an
  implementation detail. Flag: **tc-attached-marker-amends-the-grammar-rule**.

**Recommendation: LAND IT, in X1, scoped to marker-adjacency only.** The honest permanent answer is
*not* refuse-with-workaround: the workaround is "phrase your sentence so the value is not quoted",
which contradicts 26 existing messages and would make the loom surface unable to express the
project's own house style. Cost: the compiler must split a text fragment at the marker boundary
rather than requiring whitespace delimiters — errorloom's transport is already scalar-level
(`editable.rs:295`: "the alignment is over Unicode scalars"), so the change is in `dorc-loom`'s
`compile.rs` marker scanner, not in errorloom's word model. ~SUSPECT: small (1 day). This is
consistent with `289:steer-errorloom-best-to-use` (the whitespace-only word boundary is artificial;
fix it in place, no adapters).

---

# §6 — map-ledger-draft (the loomability ledger)

Verdict vocabulary: **E** editable-today (proving path named) · **X1/X2** editable-after-that-spec ·
**N** never-looms (law cited) · **R** no-render-surface-yet.

| category | rows | verdict | proving path / law |
|---|---|---|---|
| catalog `message`, case-owned, ordinary | 34 | **E** | probed green (§1.4); `mise run loom:compile <case>` |
| catalog `message`, case-owned, spanless+help | 6 | **X1** | §2b |
| catalog `message`, case-owned, `message: None` | 14 | **X1** | §2a |
| catalog `message`, case-owned, pure passthrough | 5 | **X2** | de-passthrough (`282:rul-passthrough-type-gated`, lane `284`); until then the sentence is not ours to edit |
| catalog `message`, ratcheted (no case) | 35 | **X1/X2** | §3a: 14 need only `scaffold`+authoring (X1); 21 need harness support (X1 for parse/CFG+records, X2 for oracle-loading) |
| catalog `help`, 7 written | 7 | **X1** | §2b (the register becomes editable once the reflow stops swallowing it) |
| catalog `help`, 87 absent | 87 | **X1** | §2a's h-scaffold-affordance; today the attempt is silently absorbed |
| catalog `when_fires` / `why` | 94×2 | **E** | frontmatter of the defining case; not user-facing, but already loom-sited |
| catalog `example` | 94 | **N/derived** | generated from the compiled message + the defining payload (`287` §11); never authored |
| arrangement words — faced | ~11 | **E** | `cli-help-page`, `lint-advisory-preamble`, the `why-drift-*` rows |
| arrangement words — `why-*` chain rows | ~100 | **X2** | §3b full-driver extraction |
| arrangement words — `lint-*`/`cli-*` unfaced | ~15 | **X1** | §3b cheap faces (new lint/plan-stderr cases) |
| arrangement words — plain-text seats (`usage_text`) | 2 | **N** | `aid/CLAUDE.md` a-registry-row-need-not-mint-a-span — faceless PER SEAT by design; lock-edited |
| CLI help page | 1 | **E** | `crates/aid/tests/cli-help-page.loom` |
| CLI usage synopsis | 1 | **X1** | §3b; seat-appended |
| `--version` line | 1 | **N** | `289:rul-version-line-stays-code-owned` (the number is a value; per-version transcript churn) |
| invocation / `dorc-sh` / lint error codes | (in the 94) | **E/X1** | `288` §6 landed them as registry codes; they are in the matrix above |
| plan-render annotations (`plan/src/render.rs`) | ~12 emitters | **N** | `aid/CLAUDE.md` artifact-plane-strings-stay-out + `two-surfaces` — receipt-stripping byte-identity is a stronger claim than editability |
| probe/plan/apply artifact headers, guard preamble, `# replace[..]`/`# omit[..]` blocks | (same) | **N** | same law |
| machine formats — lint JSONL envelope, `--debug-argv`, `dorc-records/1` | 3 surfaces | **N** | same law at a different altitude (`aid/CLAUDE.md`); a machine renderer MAY expose editable regions later only by returning provenance (`287` §1) |
| layout, indents, group-header colons, the compact finding frame | — | **N** | `aid/CLAUDE.md` layout-is-not-a-word |
| tier-word SET (`SpeechAct` kinds) | 7 | **N (set) / E (spellings)** | `trust-tier-is-syntax`: the SET is typed law; only the words are registry rows (`why-tier-word`, 7 entries) |
| narrative classes (9 `CollapseKind`s) | 9 | **R** | `aid/CLAUDE.md` narrative-mints-outrun-renders + `289:seam-narrative-render-unconsumed` — minted at all nine, rendered at one. **Do not build the narrative render**; the ledger records these as no-render-surface-yet, covered at mint-time by the fixed machinery |
| report-lane notes (`27W` decline classes, free-form author emissions) | vocab + relay | **N (relay) / E (vocab words)** | unrecognized lines are RETAINED sanitized relays (`27W:rul-report-noise-tolerant`) — foreign text, never ours; the engine-owned verb/class words are catalog-sited |
| external tool relays (`shellcheck:SC2086`) | — | **N** | `288` §5: source-tagged relays forever |
| `remediation_hint` class prose | 4 rows | **E (lock) / X2 (face)** | `289` §2u homed them as `why-remediation-*` arrangement rows; faceless (`289:seam-whylens-render-seat`) |
| the `why()` reason OPENER | 1 | **X2** | `289:finding-reason-opener-still-hardcoded` — still a hardcoded `format!` in `aid/src/diag.rs`. **This is a live counterexample to the arc-close invariant and must be closed by X2.** |
| `dorc-loom`'s own CLI surface (refusals, previews, `vars`) | — | **N (as product) / X1 (as teaching)** | `282:rul-internal-tool-sharp-edges` — never product prose, so never loom-sited; but §8 makes it the ONLY teaching seat a Rust-persona editor may read, so its text is in scope for X1 even though its home stays Rust |
| the `Diag` API rustdoc (payload docs, `params_of`, `new_spanless_site`) | — | **N (as product) / X1 (as teaching)** | same: `rul-rust-and-loom-are-the-only-edit-surfaces` puts the teaching here, so §2f's rustdoc repairs are arc-scope even though rustdoc is never loom-editable |
| coverage/sweep/yardstick bins | — | **N** | instruments, never product (`288` §6 out-of-scope) |

**Ledger conclusion.** Nothing in the trawl is un-loomable for a *reason we cannot name*. The
never-looms set is three laws deep (artifact byte-floor · layout-is-not-a-word · per-seat plain
text) plus two register-shaped exceptions (`--version`, foreign relays). Everything else is X1 or
X2. The arc-close invariant of `288` §8 is reachable.

---

# §7 — map-execute-cut

Marked **[C]** mechanical churn (route to cheap models) or **[J]** judgment work.

## X1 — "the loom is the edit surface for everything that has a face"

| lane | contents | churn/judgment | zero-churn expectation |
|---|---|---|---|
| **x1-boundary-weld** | §0.4: wrap inside the render seat; delete K1–K6; §0.5/K7 index-ordered section addressing; the `transcript_bytes_equal_production_bytes` gate | **[J]** the wrap placement + the addressing algorithm; **[C]** the re-bless of every aid transcript and every plan-stderr `expected.out` | goldens MOVE (sanctioned). Empty-diff proof owed on: the two LOCKS (no prose changes), and every e2e artifact `.sh` (the byte floor must not move) |
| **x1-placeholder-and-help** | §2a: placeholder wears its register's face; added-help refuses and names the command; the ruled help-register affordance | **[J]** | production render bytes byte-identical for all 14 unwritten codes — empty-diff proof owed |
| **x1-provenance-completion** | §2c: `editable_baseline` via `replay`; the `vars` replay arm; committed `vars --used` blocks in all 59 cases | **[C]** the 59-case regeneration; **[J]** the seat collapse | locks unchanged; 59 transcripts grow one block each |
| **x1-mirror-thread** | §2d: `ArrangementLookup` (or `RenderCtx`) through `Said::runs`/`text`, 17 seats | **[C]** signature threading; **[J]** the parameter shape (tc-lookup-parameter-vs-render-context) | ZERO golden movement — empty-diff proof owed over both test trees |
| **x1-attached-markers** | §5: marker-adjacency in `dorc-loom/src/compile.rs`; the `282` §13 grammar amendment | **[J]** | zero churn; new tests only |
| **x1-reach-cheap** | §3a's 14 scaffold-and-author codes + `fire_book_analysis` diag-set widening + records intake; §3b's cheap lint/cli faces | **[C]** the case authoring is mechanical once the trigger note is in hand (each ratchet entry carries one); **[J]** the harness widening | 14–21 new cases; the ratchet SHRINKS by the same number; locks gain rows |
| **x1-tooling-friction** | §4's five GO-trivial items (`loom:scaffold`, `loom:vars`, verbose tasks, trailing-`\n` trim, arity refusal) + the blast-radius gate | **[C]** | one lock churn from the trailing-`\n` trim (one row today) |
| **x1-rust-surface-weld** | §2f: spec-worlds-move-to-the-diag-api (retire `canonical_payload`'s hand-built payloads) · spec-payload-fields-declare-their-name (the census gate) · the `UnknownParam` Rust-path refusal · the rustdoc repairs (slug→payload pointer, `new_spanless_site` path + lexical rule) | **[J]** the world-source decision; **[C]** the rustdoc repairs and the census gate | zero golden movement; the locks may gain `params` rows only if a new hole is actually used |

**X1 gate**: the naive-reviewer acceptance run (§8, BOTH remits), plus `mise run both gate:full-quiet`.

## X2 — "everything has a face"

| lane | contents | churn/judgment |
|---|---|---|
| **x2-full-why-driver** | §3b: extract the report assembly out of `main.rs`'s 971-line `run()` behind `lib-target-is-a-loom-seam`; mint why cases across the chain families; face ~100 `why-*` rows | **[J]** heavily; the 17-arg context struct is **[C]** |
| **x2-de-passthrough** | the 16 `sm {{detail}}` codes: type-gated user-sourced text (`282:rul-passthrough-type-gated`), sentences composed at emit sites de-passthrough into real templates with world-variant siblings (`AID-NEEDS:law-codes-vary-by-world-not-grammar`) | **[J]** — this is a code-splitting design act, not a rename |
| **x2-reach-hard** | §3a's 7 oracle-loading codes | **[J]** |
| **x2-reason-opener** | `289:finding-reason-opener-still-hardcoded` | **[J]** |
| **x2-ledger-ratification** | §6 as a committed durable; the ratchet emptied or every survivor law-cited | **[C]** |

**Ordering.** x1-boundary-weld FIRST and alone — it moves goldens corpus-wide and everything else
rebases over it. Then x1-placeholder-and-help ∥ x1-mirror-thread ∥ x1-attached-markers ∥
x1-rust-surface-weld (file-disjoint). Then x1-provenance-completion (it regenerates cases, so it
wants the transcript form settled) — and note x1-rust-surface-weld's shape (b) FEEDS
x1-reach-cheap, since retiring `canonical_payload` and making the ratcheted codes fire honestly are
the same act. x1-tooling-friction is independent and can run any time. X2 opens with
x2-full-why-driver.

**Sizing.** X1 ≈ 7 lanes ≈ 6–8 agent-days, of which roughly a third is [C]. X2 ≈ 4 lanes,
dominated by the driver extraction and the de-passthrough design. -GUESS on both.

---

# §8 — map-naive-reviewer-gate (what the acceptance gate demands)

The gate, as amended: a blind low-tier agent is handed a `.loom` path and a remit. It **may read**
the loom file, tool output and refusals, and the Dorc-side `Diag` API rustdocs. It **may not read**
`dorc-loom` source, `errorloom` source, or either lock. Remits include both *change this error
message* and *add this variable, which does not exist yet, from Rust*. It must succeed and report
every chafe. Per `rul-rust-and-loom-are-the-only-edit-surfaces`, the reviewer's permitted reading
IS the teaching surface: rustdoc + refusals.

## §8.1 — Walking remit A ("change this error message") today

| step | what the file/tool tells them | verdict |
|---|---|---|
| "what is this file?" | frontmatter keys `code`/`when-fires`/`why`, a `-- replay --` section | partial — the format is guessable |
| "may I edit this text?" | nothing. 25 of 59 cases refuse, and the refusal is a bare class name | **FAIL** |
| "what command turns my edit into the product?" | nothing in the file | **FAIL** |
| "what are the variables here?" | nothing in the file; `vars` is not a mise task | **FAIL** |
| "I typed a marker and it refused" | `Compile(AttachedMarker(TemplateVariableName("flag")))` | **FAIL** — names no repair |
| "I edited a word and it refused" | `MarkerOutsideEditableSection` | **FAIL** — names neither cause nor repair |
| "compile said OK, now what?" | `compile: 1 cases, 1 touched, receipt <path>` | partial — the receipt path is a hint, `promote` is unnamed |

## §8.2 — Walking remit B ("add a variable from Rust") today

| step | what the permitted surfaces tell them | verdict |
|---|---|---|
| "`{{host}}` refused — where does a variable come from?" | `UnknownParam(...)` and nothing else. No mention of payloads, of `aid`, or that Rust is the answer | **FAIL** — the refusal is the ONLY seat this persona can be taught from, and it is silent |
| "which Rust type holds this diagnostic's values?" | the loom names its `code:` slug; nothing maps slug → `DiagCode` variant → payload struct | **FAIL** — a slug-to-type pointer is one line and does not exist |
| "I found the payload struct — what else must I touch?" | `params_of`'s rustdoc states the rule well (named params are ours, engine formatters, passthrough rides `detail`) | **PASS** — this seat is genuinely good |
| "I added the field and it compiles — am I done?" | nothing. The value is silently loom-invisible until `params_of_raw` gains an arm (fnd-params-arm-is-not-forced) | **FAIL** |
| "the compiler sent me into `dorc-loom/src/consumer.rs`" | `E0063` at `canonical_payload` (fnd-canonical-payload-forces-a-loom-edit) | **HARD FAIL** — the gate forbids reading that file, and the compiler requires editing it |
| "I minted a whole new spanless code and a lexical gate failed" | `new_spanless_site`'s rustdoc names the gate but points at a moved path and omits the literal-spelling rule | **FAIL** (fnd-spanless-lexical-gate-is-taught-only-to-agents) |

## §8.3 — Minimum the gate demands (all inside X1)

1. **v-in-file-inventory** (§2f) — the committed `$ dorc-loom vars --used CASE` block. The file
   teaches its own variables. Already ruled by `282:rul-used-inventory-is-committed`.
2. **A loop hint carried by the case itself** — `edit prose below, then: mise run loom:compile
   <case> && mise run loom:promote <case>`. It is frontmatter, not product bytes, so `two-surfaces`
   is untouched. **tc-in-file-loop-hint-is-frontmatter** — flagged UP: `LOOM_KEYS` is a CLOSED
   vocabulary and an unread key is refused (`crates/cli/CLAUDE.md` loom-form-is-the-same-battery),
   so this is a key mint, not a comment.
3. **v-refusals-name-the-next-command** (§2f) — every refusal ends with the exact command or the
   exact edit that resolves it. `282:rul-internal-tool-sharp-edges` permits blunt dumps; it does
   not permit *unactionable* ones, and this gate is the reason.
4. **`UnknownParam` must name the Rust path** — "no value `host` on this diagnostic's payload.
   The values it carries are listed by `dorc-loom vars --all <case>`. To add one, add the field to
   its payload struct and its `params_of` arm in `spike/crates/aid/src/diag.rs`, then rebuild." That
   sentence alone converts remit B from a hard fail to a pass, and it is the only teaching seat the
   persona is allowed to see.
5. **spec-worlds-move-to-the-diag-api** (§2f) — until `canonical_payload` stops hand-constructing
   payloads, remit B is *unpassable by construction* for 29 of 94 codes. This is the one gate
   demand that is architectural rather than textual, and it must be in X1.
6. **`mise run loom:scaffold` / `loom:vars`** (§4) — so `mise tasks` alone is a complete map of the
   loop.
7. **Refusals must be reachable** — the blast-radius dirty gate (§4) refuses before any
   loom-specific message is printed, so a reviewer with any unrelated dirt in the tree never sees
   the real error. Measured: this bit me during this map's own probing.
8. **Rustdoc repairs** — a slug→payload pointer on `DiagCode` (or on each payload's doc line), the
   `new_spanless_site` path fix, and the lexical-spelling rule moved out of `aid/CLAUDE.md` into
   the rustdoc.

**Design consequence for §2a/§2b/§2f**: a fix that makes an edit *succeed* is worth less to this
gate than one that makes a failure *self-explaining*. Every spec must budget for its refusal text,
and every refusal must be readable by someone who may not open the crate that emitted it.

---

# §9 — tc-flags (never settled here) and confidence

**Flagged UP to the conductor:**

- **tc-fixed-width-diagnostics-is-a-product-choice** (§0.4) — moving the wrap into the render seat
  makes production diagnostics fixed-width. The alternative is flat transcripts. Product decision.
- **tc-lookup-parameter-vs-render-context** (§2d) — bare `&dyn ArrangementLookup`, or a `RenderCtx`
  that also carries §2b's width?
- **tc-variable-surface-is-a-command-or-a-file** (§2f) — committed in-file inventory vs. a
  first-class browse surface.
- **tc-honest-firing-vs-canonical-payload** (§3a) — build harness support for the 15 hard ratchet
  codes, or add 15 `canonical_payload` stand-ins?
- **tc-attached-marker-amends-the-grammar-rule** (§5) — landing adjacency amends
  `282:rul-double-brace-template-only`'s "no attached punctuation" clause. Law edit.
- **tc-in-file-loop-hint-is-frontmatter** (§8) — a new closed frontmatter key vs. nothing.
- **tc-help-register-affordance** (§2a) — which of h-render-a-help-placeholder-always /
  h-scaffold-affordance / (rejected) h-accept-a-typed-help-line. My lean: h-scaffold-affordance.
- **tc-payload-param-binding-wants-a-macro-we-cannot-write** (§2f) — the structural fix for
  fnd-params-arm-is-not-forced is a derive macro, which `spike/CLAUDE.md` forbids ("no authored
  macros"). Accept the lexical census gate instead, or revisit the macro ban for this one seat?
- **tc-fixture-world-source** (§2f/§3a; supersedes and absorbs tc-honest-firing-vs-canonical-payload)
  — do the stand-in worlds move beside the payload types in `aid` (shape a), or die entirely in
  favour of honest firing from the defining case (shape b)? Shape (b) satisfies
  `289:rul-worldless-route-honest-trigger`, retires 29 hand-built payloads AND 23 more in
  `catalog_defining_cases.rs::covered()`, and is the same act as §3a's harness widening — but it is
  the larger bite.

**Standing 28H law honoured**: no new word/whitespace/boundary judgment is proposed ad-hoc inside a
parsing function. §2b proposes exactly one named seat per crate (the render seat's wrapper), and
§4's trailing-`\n` trim lands in `dorc-loom`'s one existing significance seat.

**Confidence.**
+SURE: the §1 matrix (every cell driven, every probe reverted) · fnd-added-help-is-silently-absorbed ·
fnd-passthrough-prose-has-no-authored-words · fnd-two-render-chains-bridged-by-a-guess ·
fnd-book-route-sees-only-effect-diags · fnd-plan-route-never-reads-its-results ·
fnd-computed-words-are-invisible-to-an-author · fnd-canonical-payload-forces-a-loom-edit and
fnd-params-arm-is-not-forced (both reproduced end-to-end by adding a real payload field and
following the compiler, then reverting) · fnd-foreign-param-is-a-name-convention · the corpus counts
(94/59/35/14/74/6, 135/112/22/1, 26 backticked, 29 `canonical_payload` arms, 23 `covered()`
constructors, 17 `.runs(` seats, 5 `CONST_ARRANGEMENTS` reads, 971-line `run()`, 17-arg
`emit_why_report`) · the K1–K6 + K8 kill-list.
~SUSPECT: K7 is the complete list of content-search addressing (I did not read `receipt.rs` whole) ·
the ~11 faced arrangement rows (no tool enumerates them — hence v-sections-command) ·
`redir-target-top`'s effect-plane arm reaching through `fire_book_analysis` · §2c's
"~40 lines net-negative" · fnd-spanless-lexical-gate-is-taught-only-to-agents' practical impact
(the rustdoc IS the right seat and mostly says the right things; only the path is stale and the
lexical rule missing).
-GUESS: every size estimate; the X1/X2 lane split's wallclock.
--WONDER: whether wrapping inside the render seat wants weft (the box-model crate) rather than a
local wrapper — weft already solves this problem for the why surface, and a second wrapper is a
second lexical judgment seat, which is exactly what `ask-shared-lexical-rulebook` warned about.
