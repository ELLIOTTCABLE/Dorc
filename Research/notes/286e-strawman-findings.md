# 286e — the explain-surface paper-prototype: findings

AI-authored (Fable, 2026-07-20; same sitting as `plans/286`). The `286` §3 first-act
paper-prototype, executed early at human direction: `286a`–`286d` are whole-cloth
strawman case files in the proposed committed shape — inert documents, nothing
parses or executes them; every format detail in them (frontmatter keys `concept:` /
`covers:`, flag spellings, fallback renders, gutter conventions) is invented here
and is NOT spec. Base shape derived from the real
`spike/crates/dorc-loom/cases/cmdsub-operand-top.txt`; registers declared as
multiple sequential invocations in the replay tail, per the sitting's ruling.
Ruled-in-passing, banked here: case files take the **`.loom`** extension (human,
2026-07-20).

The set, chosen to shine AND to bite: `286a` `wall` (full three-register concept;
one concept-aside embed; one case-replay embed) · `286b` `unmodeled` (minimal leaf
concept; no page register — forced into existence by 286a's embed, which is the
dependency pressure demonstrating itself) · `286c` `plan-lifecycle` (the
soup-include: one expression covering {probe, plan, apply}; no terse register;
covers-routing) · `286d` `cmdsub-operand-top` (the real code case grown an explain
replay: code-specific prose, an ambient-world exhibit, a concept-aside embed, the
error-render pointer line).

## Where it shone

- **`286e:find-composed-authoring-held`** (n=1, the premise's first live test) —
  writing the wall page WITH the unmodeled aside visible in place forced re-reading
  the aside in its host context; the seam-quality question answered itself at
  authoring time. The transclusion-blindness failure the research named did not
  have room to occur.
- **`286e:find-plan-replays-self-ground`** — a case-replay embed of a `dorc plan`
  render (286a's worked example) needs no accompanying book excerpt: the plan
  render *displays the book's lines*. Dorc-specific luck; a `dorc why` or apply
  embed would dangle without more context. Embed rule sketch: plan-replays embed
  bare; other replays need a book-section rider or self-contained teaching cases.
- **`286e:find-gutter-does-double-duty`** — the `   > ` gutter reads well in a
  terminal, is Markdown-blockquote-native for the web channel, and incidentally
  keeps embedded `$ dorc ...` invocation lines from colliding with the replay
  parser's column-0 `$ ` block delimiter. That coordination is load-bearing: a
  page render must never emit a column-0 `$ ` line, or the case format mis-splits.
  Cheap gate at bless.
- **`286e:find-exhibit-rides-the-scratch`** — 286d's "in your last plan" exhibit
  needed zero new mechanism: the explain replay simply follows the plan replays in
  the case's shared sequential scratch (`282` §2 designed exactly this for
  `dorc why --last`), and the ambient instance is there. Live behaviour is the
  same sequence in your shell.
- **`286e:find-short-code-pages-work`** — the code page stayed short and specific
  (the value-unknowable vs effect-unboundable distinction is prose only that code
  needs) while embeds carried the background; the `286:rul-explain-register-carve`
  fallback ladder felt right in the hand.

## Where it bit

- **`286e:find-terse-glosses-drift`** — terse registers are flat (stratification)
  and jargon-free by necessity: they cannot embed and cannot afford to teach a
  term, so they GLOSS their dependencies in-sentence (wall-terse says "a command
  ... Dorc knows nothing about" where the page embeds unmodeled properly). Those
  glosses are hand-curated synonyms of other concepts' registers — an unguarded
  consistency surface between a concept's registers and every dependent's gloss.
  No machinery catches a gloss going stale when its concept's prose is re-ruled.
  Candidate mitigations (none chosen): a glossary lint keyed on concept slugs
  appearing as plain words; or accepting it as the price of flat terse registers.
- **`286e:find-hand-sync-held-at-n3`** — the three copies of wall-terse and two of
  unmodeled-terse across files came out word-identical (verified by stream-diff,
  modulo gutter re-wrap) — but only under the best conditions the corpus will ever
  see: one author, one sitting, copies written minutes apart. Every later edit is
  the case the transport's single-edit-home + fan-out re-render exists for; the
  strawmen cannot demonstrate that half, only make its absence felt.
- **`286e:find-fallbacks-were-invented-on-the-spot`** — two renders had no ruled
  behaviour and forced inventions: bare `dorc explain unmodeled` with no page
  register (rendered: summary + an honest "(no full page ... deepest register
  available)" line) and `dorc explain probe` under `covers:` routing (rendered:
  one redirect line + the soup's summary + a pointer to the full page). Both need
  real rulings; the covers-routing one hides a genuine fork — whether a covered
  concept may ALSO own its own registers, and who wins.
- **`286e:find-register-optionality-is-load-bearing`** — the soup has no terse (a
  one-sentence rendering of three concepts is not worth authoring); codes take no
  density flags at all (a code's terse IS its diagnostic message; only the page
  register exists at the code level). Both asymmetries felt correct and neither is
  designed; `286:fork-register-flag-naming` widens to "which registers exist per
  page-kind."
- **`286e:find-exhibit-sequence-honesty`** — the committed transcript's exhibit
  renders because the explain invocation FOLLOWS a plan in the same scratch; a
  fresh-shell `dorc explain cmdsub-operand-top` has no ambient instance and
  renders no exhibit. Copy-paste honesty therefore holds for the replayed
  SEQUENCE, not the lone command — a fresh reader typing only the last command
  gets a (correctly) exhibit-less page. Must be stated in the spec; arguably the
  exhibit block wants a tiny provenance head of its own ("in your last plan:")
  which it has, and which doubles as the explanation of its own absence.
- **`286e:find-no-timestamps-in-corpus-exhibits`** — the exhibit deliberately
  omits measured-at times: committed transcripts must be byte-stable, and live
  exhibits will want times. Same shape as `286:rul-freshness-stamp-flag-gated`
  (display divergence between corpus and live, resolved by flag/config, corpus
  side pinned OFF). The DST fixed-clock alternative would instead pin a fake time
  into the corpus; either works, one must be chosen.
- **`286e:find-example-library-pressure`** — 286a's worked example references an
  invented case (`plan-first-wall`) that does not exist; real embeds reference
  real corpus cases, so teaching examples become first-class corpus citizens that
  someone must author and maintain. That is probably a feature (examples inherit
  bless-honesty) but it is authoring work the ladder's sizing should count.
- **`286e:find-corpus-scale-reality`** — the wall page renders at ~45 lines; a
  full concept set (dozens) plus 10^3 code pages at even fallback grade is a real
  prose corpus. The design's own levers (optional registers, soup-includes,
  lazy-burn-down, ai-voice-with-review-tiers) are what make it tractable; none of
  them are optional in practice.

## Banked spellings (strawman-tier, for the eventual spec to accept or replace)

Flags: `--terse` (sentence) · `--summary` (paragraph) · bare = deepest available.
Frontmatter: `concept: <slug>` for concept cases; `covers: <slug list>` on a
soup-include; `code:` unchanged for code cases. Sidebar: three-space indent +
`> ` gutter; first line the invocation (concept-asides) or a labelled head
(`worked example (case <slug>):` / `in your last plan:`). Pointer line in error
renders: `= more: 'dorc explain <slug>' tells the full story` (dedup per code per
run). Fallback lines: "(no full page is written for this concept yet; ...)" and
"'X' is part of the Y explainer: ... (for the full treatment: dorc explain Y)".
