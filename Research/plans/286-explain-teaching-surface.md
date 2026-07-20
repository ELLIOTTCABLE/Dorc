# 286 — the explain surface: density registers + command-block transclusion

AI-authored (Fable, design rubber-duck sitting WITH the human, 2026-07-20; every §0
ruling is human-typed unless marked strawman/lean). DESIGN-TIER, BUILD-PUNTED — no
round owns the build; banked because the design firmed enough to preserve. Serial ID
claimed per `28A` §0b. Authority: root docs, `spike/CLAUDE.md`, root `AID-NEEDS.md`
Law outrank this. Companions: `plans/282` (the errorloom transcript-case pipeline this
extends) · `notes/27U`/`27V`/`27W` (the aid as-built this sits atop) · `notes/28A`
(the r28 ledger; generation-flip state) · USER_STORY "Recovery". Research base:
`.claude/research/explain-prose-reuse/synthesis.md` (28 graded sources, gathered
2026-07-20; the conductor hand-verified the two most design-favorable anchors).
Implementor read-first, at unpark: `282` whole → `AID-NEEDS` Law → this doc → the
research synthesis.

## §0 — Ruling ledger (human-typed 2026-07-20 unless marked)

- **`286:rul-explain-is-the-fourth-tier`** — `dorc explain <slug>`: a
  rustc-`--explain`-shaped, assumes-no-knowledge, multi-paragraph teaching surface,
  per-CODE and per-CONCEPT, in-shell and CLI-first (in-shell is version-locked to the
  binary — a correctness property, given verdicts-never-stable; a URL describes a Dorc
  you may not be running). It claims the reserved "first-encounter" register slot
  (`AID-NEEDS:aid-error-catalog-explainers`); the existing prose ladder below it
  stands unchanged (plan reason-tail · machine line · terse · deep). Pull surface,
  wide-open per `law-pull-runs-wide-open`; the only push delta is a pointer line on
  error renders, deduped per code per run (the rustc pattern).
- **`286:rul-explain-register-carve`** — explain prose is OPTIONAL-per-code with a
  fallback ladder (code-explain → shared concept expressions via embeds/links); the
  per-code-full-prose-no-fallback law stays intact for the message/terse registers.
  Grounds: sibling codes are world-state variants by law and genuinely share
  background; mandatory multi-paragraph per-code prose is the documented
  heavyweight-authoring-mandate failure (`22A:concl-7`, the rustc-Fluent downgrade).
- **`286:rul-ai-voice-only-in-explain`** — the standing project line: NO AI-authored
  user-facing prose anywhere in Dorc EXCEPT the explain surface. Disclosed via a
  per-register provenance stamp rendered by arrangement code (never an invoke-time
  LLM; generation is offline authoring, disclosed in output). Consequence, noted:
  taken literally this makes `spike/docs/` non-conforming; resolved by the docs-home
  strawman below (`286:seam-spike-docs-absorption`).
- **`286:stw-single-docs-home`** — STRAWMAN-grade (human-typed as strawman): explain
  pages become the single authoritative docs home — no docs outside them, period,
  except perhaps a website landing page. One source generates all channels (CLI
  first; GitHub-Markdown/HTML second); static channels are backfilled with case
  worlds as test-data. `spike/docs/`'s reader-path arc would become an ordered index
  over concept pages.
- **`286:rul-registers-cohabit-never-compose`** — a concept owns co-located DENSITY
  REGISTERS — a one-sentence aside, a one-paragraph treatment, a full-page dive —
  authored separately and NEVER derived from one another by composition
  (summarization is information-lossy with different priorities; you would never
  write the sentence version inside the paragraph version). Each register has a CLI
  face (`dorc explain --terse <slug>`-shaped; flag spellings are latitude, NB the
  naming collision with the diag catalog's terse/deep registers — pick deliberately)
  which is simultaneously its forcing-function, its product surface (the expert's
  "remind me"; the future LSP hover payload), and its editing surface.
- **`286:rul-transclusion-is-a-command-block`** — shared prose enters a host page
  ONLY as a set-off block (Markdown-blockquote-ish) whose first line is a CLI
  invocation; NEVER spliced into a host paragraph. This is quotation, not
  composition: fit-with-host-voice is never claimed — the visible invocation is
  deliberate user-facing DISOWNERSHIP-OF-VOICE, an excuse for fragments to mismatch
  in register and tone.
- **`286:rul-dorc-embeds-are-validated-links`** — for dorc-command blocks the
  invocation line is window-dressing: content renders IN-PROCESS at the one render
  seat (the `28A` §2n composition precedent — composition is in-process-faithful;
  CLI framing is the e2e corpus's job), with copy-paste honesty as the obligation
  (typing the shown command yields the shown text). Functionally the block is an
  INTERNAL LINK: fixed-format, validated, FAIL-FAST at authorship (an embed naming a
  nonexistent slug refuses at bless). The supported invocation set starts small and
  closed and extends deliberately; it only APPEARS free-form to the end-user.
- **`286:rul-case-replay-embeds`** — the second embed species: actually-EXECUTED CLI
  teaching examples, spelled approximately "embed <other case's replay block> as
  rendered here" — borrowing the errorloom case corpus as the example library, so
  input/output teaching examples carry the rustc-doctest honesty property (they
  re-execute at bless; they cannot go stale).
- **`286:rul-command-in-output-out-ceiling`** — the complexity ceiling, stated as a
  DATA-MODEL law: content enters a page only as {local paragraph | validated
  dorc-embed | case-replay embed | exhibit}; anything not expressible as
  command-in/output-out/output-lands-in-file is nack'd as scope-creep, permanently.
  No other composition primitive will ever exist. This is where the
  grammatical-composition anti-goal becomes structural: a CLI invocation is a
  parameter surface no pluralization/agreement engine can pass through (the
  research's Wikipedia-Lua arc and legal nested-conditional collapse both hit
  exactly this wall; we build the wall in on purpose).
- **`286:rul-soup-include-not-include-soup`** — the block-soup failure mode is
  answered by a SOUP-INCLUDE: when N concepts are habitually explained together,
  author ONE coherent combined summary-explainer of all N (a first-class
  multi-concept expression) and embed THAT wherever all N groundings are needed at
  one altitude. Re-explaining a concept in multiple pieces of prose is legal —
  never over-D.R.Y. at the cost of teaching flow.
- **`286:rul-freshness-stamp-flag-gated`** — the provenance footer (ai-voice +
  last-edited): the DATA is an absolute per-register date + provenance bit stamped
  at promote (via errorloom's git trait; no runtime git dependence — end-user
  installs carry the stamp inside the compiled catalog); the DISPLAY is
  flag/config-gated and OFF for all tests and the committed corpus (tests must be
  stable; artificial stability is acceptable and often the only kind achievable).
  Live surfaces may relativize ("43 days ago") from the stamp. Alternative if ever
  wanted: DST fixed input-clock — same stability, more machinery.
- **`286:rul-edit-in-situ-leaned-into`** — the section-copy dodge (authoring explain
  prose as a raw case section outside the transport) was considered and gently
  nacked: the design leans into errorloom's edit-in-composed-form and probes how
  nasty the transport gets in practice; the retreat (annotated editing, `282` §5)
  stays priced if it really fights us.
- Staging (acked): `dorc explain` (slug-indexed teaching) ships first;
  `dorc why N --explain` (the license chain interleaved with concept prose — the
  flagship naked-trust chain wearing teaching clothes) is a second, later product on
  the same machinery. `why` stays decision-indexed; `explain` stays concept-indexed;
  footers cross-point.

## §1 — Architecture (mechanism digest)

- **registers-as-replays** — a concept's case file carries one replay block per
  register (`$ dorc explain wall`, `$ dorc explain --terse wall`, …); prose-bless on
  a block edits that register; `282:rul-multi-replay-per-case` is the carrier
  unchanged. A register IS a view; no new storage or authoring flow exists.
- **`286:inv-every-expression-has-a-face`** — every editable prose unit has exactly
  ONE edit-home, and every edit-home is a rendered, user-facing surface (a
  register's replay block in its concept's case). Fragment-files-in-a-vacuum — the
  research's one universal root-cause of prose-reuse rot — are unrepresentable.
- **`286:inv-registers-stratify`** — sentence and paragraph registers are FLAT leaf
  prose (may embed nothing); only the page register may embed other concepts'
  sentence/paragraph blocks, case-replays, and exhibits; a page never embeds a
  page. Cycles are unrepresentable by construction. The granularity ladder:
  aside→sentence · sidebar→paragraph · see-also link→full page.
- **expressions-not-concepts** — a register entry is one EXPRESSION of a concept for
  one density/audience; a concept may own several (including multi-concept
  soup-includes). Transclusion is available-NEVER-mandatory: no dedup lint, no
  similar-prose detection, ever; pages wanting locally-fitted wording write local
  prose. The whole enforcement story is the incentive lever (one command line is
  lazier than writing the paragraph — coherence as the path of least resistance,
  the Baker frankenbook lever).
- **the ambient-world contract** — an embedded block renders against the AMBIENT
  world: in the committed corpus, the defining case's materialized book+world
  (which is also the static-channel test-data backfill, for free); live, your
  run/whylog. Instance-interleave mode (`--last`-flavored exhibits: your actual
  book, spans, values between the paragraphs) is the SAME mechanism over the live
  world — the tree is parameterized over the world with zero additional machinery,
  the same way plans are functions of the probed world.
- **catalog dependency edges** — embeds mint first-class edges (page → slug),
  giving cycle detection, regeneration completeness, and dangling-slug refusal at
  bless (self-enforcing; subsumes a see-also lint). Editing a shared register
  re-renders every embedder; the git word-diff across all of them IS the
  read-all-hosts-together review affordance, mechanized.
- **edit-routing, collapsed** — an edit inside an embedded block is an edit to
  generated output → refused; the redirect pointer is ALREADY PRINTED in the render
  (the invocation line names the home). Register edits happen only at their face.
  Fork-on-edit does not exist; a host wanting different words writes local prose.
- **review-tier follows blast-radius** — terse registers are the highest-
  transclusion, one-sentence units: human review concentrates there (an afternoon
  across the whole concept set); leaf per-code page prose rests at disclosed
  ai-voice indefinitely (`[unwritten:]` remains the legal resting state — missing
  beats incorrect, the Clang/WtD norm). The per-register stamp carries
  reviewed-vs-ai state; single-point-fix is the flip side inlining buys.

## §2 — Research adjudication (compressed; full detail in the synthesis)

Money answer: the inline-reuse-with-in-composed-authoring middle is UNTRIED, not
refuted (~SUSPECT — convergent inference across five domains, no direct trial). The
command-block form then dissolves most of what stood against the naive inline-chunk
version: the Black-Hole semantic seam (quotation never claims fit-with-host; the
residual shrinks to stale-RELEVANCE, a mild failure); Sentence-Salad granularity
(registers are authored standalone-for-reuse — the TM failure needs units extracted
from flow); Wikipedia's shared-sections-need-a-hatnote finding (the invocation IS
the hatnote); the parameterization ceiling (enforced by the ceiling law rather than
approached). What stood FOR it: duplication-not-sharing is the documented
drift-driver in the decorrelated legal domain; memoQ's decomposed-store +
composed-view is shipped prior art; Write-the-Docs ARID ("repeat in output, unique
in source") is nearly this model's spec; Baker's blessed case (reuse text you'd
write substantially-the-same anyway — our sibling families) vs his condemned case
(collapsing distinct expressions) maps onto registers exactly — the register set
institutionalizes multiple expressions per concept. Wikipedia's choice AGAINST
prose transclusion dissolves on inspection (their LST had no parameters; their
summary-style duplication serves the different-expression case, which we serve with
registers, not transclusion). The Clang AI wall ("incorrect documentation is worse
than missing") is the constraint `286:rul-ai-voice-only-in-explain` must keep
clearing: met by [unwritten:]-stays-legal, mechanically-honest examples
(case-replay embeds), the disclosure stamp, and blast-radius-scaled review. Watch
item: rustc's explain format is being re-standardized (draft RFC 3370, 2026) —
glance before treating rustc as a fixed reference.

## §3 — Build shape (at unpark; nothing owned today)

New machinery, each modest: register fields + explain flags · in-process embed
expansion at the render seat · the dependency-edge parse + refusals · promote
stamps (date + provenance) · case-replay embed-by-reference · the error-render
pointer line (deduped per code per run). Owed by the transport first, gating
page-register prose editing only: the v2 paragraph model (`282`'s named seam —
guaranteed to fire here, since explain authoring adds/removes paragraphs) · the
glued-param re-hole seam (`28A:rul-glued-param-rehole-seam`). FIRST ACT at unpark,
before any machinery: paper-prototype — hand-write ONE real concept page (`wall`)
in the target shape (prose + command-blocks + an exhibit) and read it; the
composed-view authoring experience of writing flowing prose AROUND blocks you
cannot edit in place is the single untested interaction in this design.
(EXECUTED early, 2026-07-20, human-directed: strawman case files `286a`–`286d`
(`.loom`, ruled) + findings `286e` — the composed-authoring premise held at n=1;
new bites banked there: terse-gloss drift, fallback rulings owed, exhibit
sequence-honesty, example-library pressure.)

## §4 — Open forks & seams

- `286:fork-register-flag-naming` — the explain density flags vs the diag
  catalog's terse/deep register names; one vocabulary or two, pick deliberately.
- `286:fork-review-tier-law` — does blast-radius-scaled review become a typed rule
  (stamp-gated) or stay posture?
- `286:fork-edit-through-host-someday` — refuse-with-pointer → direct fan-out
  editing from a host page (MediaWiki's edit-through-composite precedent exists);
  only after the review-affordance story is proven.
- `286:seam-fit-drift-nudge` — downgraded by the quotation form; nothing built;
  re-open only if the paper-prototype shows stale-relevance biting.
- `286:seam-why-explain-staging` — the chain-interleaved teaching render; second
  product, same machinery.
- `286:seam-spike-docs-absorption` — if `286:stw-single-docs-home` firms, the
  `spike/docs/` tree owes a migration story (reader-path arc → ordered concept
  index); until then the AI-voice line reads as scoped-to-shipping-surfaces.
- `286:seam-lsp-hover-convergence` — the terse register is the natural
  publish-hover payload (`AID-NEEDS` LSP surface).
- `KNOBS:kFLOW` registration owed at unpark: this design is a kFLOW position — the
  report FORM (set-off blocks, not woven flow) is chosen precisely so the simple
  authorable mechanism suffices; the refused extreme stays refused structurally.
- Instance-selection UX for contextualized mode (a code firing at N sites; which
  instance fills the exhibits) — undesigned, small, decide at build.

## §5 — Confidence

+SURE: the §0 ledger (human-typed in-session, 2026-07-20); the as-built substrate
claims (`282`/`27U`/`28A` read same-day; the multi-replay and in-process-render
precedents are load-bearing and verified in-doc, though the Rust itself was not
read this sitting). ~SUSPECT: that composed-view authoring around uneditable
blocks reads/edits well (the paper-prototype exists to answer exactly this); that
the transport's v2 paragraph model stays small. -GUESS: build sizes; the eventual
concept-set cardinality (dozens, not hundreds, if soup-includes pull their
weight).
