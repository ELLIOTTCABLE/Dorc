# Synthesis — inline prose transclusion for `dorc explain`: prior art

Gather-and-grade errand (2026-07-20). Prior art for the "unoccupied middle": true inline transclusion of
shared teaching-prose into flowing composed `explain`/concept pages, edited THROUGH the composed view.
Poles already known: (a) duplicate-per-page (rustc E-codes), (b) link-out-to-shared (Elm hints, shellcheck
wiki). This note answers goals 1–5 with graded source IDs and confidence marks. It does NOT design Dorc's
mechanism — that adjudication is elsewhere. Working log + verbatim citations: `turn01-2026-07-20-notes.md`.
Grades: A = peer-reviewed / canonical first-party; B = core-author or solid practitioner primary; C = decent
but shallow/vendor. `[graded-by subagent]` marks grades a gathering subagent made and I did not re-read (the
one A-grade legal anchor I re-verified in my own context).

---

## The money question, answered

**The middle is under-occupied because it is UNTRIED at this exact spec — NOT because the general shape has
been shown to fail. Its one consistently-documented failure mode is *authoring-out-of-context*, and that is
precisely the failure Dorc's compose-view authoring is built to remove.** ~SUSPECT (this is the convergent
read across five decorrelated domains, but it is inference from adjacent systems, not a direct trial of Dorc's
spec).

Unpacking that, four load-bearing claims:

1. **No surveyed system combines single-source dedup WITH in-composed-view authoring for mid-size teaching
   prose.** The poles are occupied; the diagonal is empty. rustc duplicates and doesn't dedup; Elm/shellcheck
   dedup by linking out; Wikipedia has the transclusion machinery but chose summarize-and-link for article
   prose; DITA/AsciiDoc/mdBook transclude but you edit the fragment *out* of its composed context. +SURE.

2. **Where the general shape (compose prose from shared fragments) has been tried, the ONE root cause of every
   failure is the fragment being edited/authored without its assembled surroundings** — "fine chunking obscures
   context" [B-baker-chunking-translation-2011]; "Sentence Salad... caused by translator's working on one
   segment at a time" [B-tm-friend-or-foe-deck-2018]; conref "is not context-aware... content may not be
   suitable in a new context" [C-stilo-conref-limitations-2023]; the cafeteria-of-includes is "a pain to edit
   and manage" [B-tomjohnson-single-sourcing-over-2025]. Dorc's `282:rul-transcript-is-the-authoring-surface`
   attacks exactly this. +SURE that out-of-context is the shared root; ~SUSPECT that compose-view authoring
   fully neutralises it (untested at Dorc's spec).

3. **The naive worry is partly inverted by the decorrelated evidence: DUPLICATION, not sharing, is the
   documented drift-driver, and single-source-update is the recognised CURE.** "In 21 years... I've never seen
   a firm successfully keep the common language in multiple templates consistent over time"
   [B-henley-draft-like-a-pro-2018]; document assembly is "a complete fix" for copy-paste incoherence
   [B-adams-dysfunction-causes-cure-2013]; Wikipedia's accepted-duplication default carries a standing
   {{Sync}}-divergence tax [B-wikipedia-summary-style-2026]. So inline reuse is not obviously the *riskier*
   choice versus duplicate-per-page. +SURE for the legal domain; ~SUSPECT that it generalises to teaching prose.

4. **But the real seam is SEMANTIC, not visual, and it is genuinely dangerous.** The peer-reviewed anchor:
   reuse-by-rote "can be emptied of any recoverable meaning," and the error-correcting feedback that catches
   drift in ordinary writing does NOT fire where a fragment is reused blindly [A-blackhole-boilerplate-choi-gulati-scott-2017].
   The risk for Dorc is not ugly seams in the rendered page; it is a shared concept-paragraph silently ceasing
   to fit a host page as sibling codes diverge, with nothing to flag it. +SURE this is the sharpest risk.

Net: the middle is a real, defensible, largely-empty cell. The evidence neither blesses nor damns it; it tells
you *which* failure to engineer against (semantic fit-drift + out-of-context editing) and *which* worry to
discount (visual/coherence seams and the fear that sharing is inherently riskier than duplicating).

---

## The frontier (where each domain sits on the three poles)

| Domain / system | duplicate-per-page (a) | link-out-to-shared (b) | inline-transclude middle |
|---|---|---|---|
| rustc `--explain` | ✔ default, doctested | — | — |
| Clang diagnostics | ✔ (99% empty) | — | (TableGen doc-strings, per-code) |
| Elm | — | ✔ hints/*.md per-concept | — |
| staticcheck | ✔ generated, family-clustered | — | — |
| Wikipedia articles | ✔ accepted duplication (default) | ✔ {{Main}} summarize+link | ~ {{Excerpt}}/LST (cautious exception) |
| DITA / AsciiDoc / mdBook | — | — | ✔ include/conref (edit OUT of context) |
| Legal doc-assembly | ✔ copy-paste (the villain) | — | ✔ clause library (single-source) |
| Translation memory | — | — | ✔ segment reuse (the seam cautionary tale) |
| memoQ TMS | — | — | ✔ segmented store + unsegmented preview (BOTH views) |

The one system that most resembles Dorc's intended middle — decomposed storage with a whole-content composed
view for the human — is a translation TMS as its co-founder describes it [B-memoq-segmentation-2025], and the
one interpretive norm that most resembles Dorc's coherence goal is the legal "read all provisions together as
one coherent whole" [B-yieldpoint-frankenstein-slip-2024].

---

## goal-1 — Wikipedia transclusion practice for PROSE

**Flowing-prose transclusion between articles is technically supported but deliberately marginal; the chosen
norm is summarize-and-link, explicitly accepting duplication.** +SURE.

- The dominant article-prose norm is Summary style: a subtopic gets its own article + a {{Main}} hatnote and an
  in-place summary, "even if this produces some duplication between the parent and child articles"; there is
  "no need to conserve paper" [B-wikipedia-summary-style-2026]. Duplication is *chosen over* transclusion, and
  its cost is a standing {{Sync}} tag for when parent and child drift.
- Prose transclusion DOES exist as the {{Excerpt}} template ("selective transclusion... instead of writing a
  summary that is essentially a duplicate") — but it is offered as a cautious exception, recommended "only with
  consensus and when articles are rapidly evolving," and it carries historical-rendition hazards
  [B-wikipedia-summary-style-2026]. ~SUSPECT that this is a *conscious* rejection of prose transclusion vs mere
  path-dependence — the guideline reads as genuine preference, but I did not read the talk-page debates.
- Labeled Section Transclusion (LST) is where prose transclusion nominally lives, and it reveals the friction:
  **it cannot pass parameters** to the transcluded section, its tags are fragile ("PLEASE DO NOT REMOVE... must
  not be tampered with"), cannot be template-generated, and its real strongholds are Wikisource book-page
  reconstruction and talk-archiving — not encyclopedia article prose [B-wikipedia-lst-help-2026]. LST thrived on
  Wikisource (reassembling scanned pages) and effectively died for cross-article prose. +SURE.
- The subst-vs-transclude norm (goal-1's "fork-on-include") is codified: subst (permanent copy) is mandated for
  one-time dated/boilerplate messages that must NOT change with the template; transclusion is kept for
  standardizing/formatting content; and **templates with ParserFunctions must NOT be substituted** because the
  conditional constructs survive verbatim — a clean statement of the parameterization boundary
  [B-wikipedia-substitution-2026]. The named drawback of subst is losing single-source-update AND losing the
  ability to find all instances. +SURE.
- Blast-radius management is real and layered: composite pages keep their own protection/history separate from
  components; editing a source page "can impact all target pages... known as 'breakage'"; shared sections must
  carry an explanatory hatnote or "easily confuse novice editors and readers"; and the sandbox/testcases +
  high-use-template protection convention exists precisely for highly-transcluded templates
  [B-wikipedia-transclusion-help-2026]. (I did not fully read the dedicated high-risk-templates page; the
  protection/sandbox story is assembled from the Transclusion + Substitution pages and search results — ~SUSPECT
  on the finer TemplateData/protection-tier specifics.)
- **The parameterization complexity arc is the cautionary tale.** Wikipedia's own "sordid history":
  {{qif}} → ParserFunctions → Scribunto/Lua [B-wikipedia-lua-2026]. Parameterized prose escalated until it left
  the ordinary-editor authorable surface entirely — into the Module: namespace, CPU-capped, maintained by a
  small priesthood. The tell for Dorc: the `count_fruit` example does pluralization via a Lua ternary
  (`n == 1 and 'banana' or 'bananas'`) — grammatical agreement was pushed into a scripting language *because
  wikitext could not express it*. This is direct, decorrelated corroboration of Dorc's grammatical-composition
  anti-goal: the moment prose reuse needs agreement/pluralization, template syntax is not enough, and the honest
  paths are (i) a Turing-complete escape hatch off the authorable surface, or (ii) refusing to compose at that
  granularity (Dorc's paragraph-floor). +SURE this arc happened; +SURE it corroborates the anti-goal.

## goal-2 — technical-writing single-sourcing (DITA, conref/keyref, the backlash)

**The reuse granularities that survived sit at the EXTREMES; the mid-size shared-prose-paragraph middle is
exactly what rotted, and the rot is always authoring-out-of-context.** +SURE.

- Survived at the coarse end: whole-file / whole-topic includes (a whole chapter, a self-contained topic). At
  the fine end, two things survived: (i) *code* fragments pulled by named tag/anchor — the overwhelmingly
  dominant use of every lightweight include mechanism (mdBook: include "is usually used for inserting code
  snippets"; AsciiDoc's tagged-region examples are all code) [B-mdbook-includes-2024],
  [B-asciidoctor-tagged-regions-2024]; and (ii) tiny substitution variables (product name, version). +SURE.
- Rotted: mid-size shared prose paragraphs transcluded into many composed pages — Tom Johnson's cafeteria "document
  consisting entirely of various includes... what a pain to edit and manage" [B-tomjohnson-single-sourcing-over-2025];
  conref's element-boundary floor + "not context-aware" [C-stilo-conref-limitations-2023]; the recurring
  heading-level-mismatch symptom when a fragment carries its own structure into a new host.
- The backlash literature is anchored by Mark Baker (field-leading structured-writing critic, deep pre-2020
  digital garden). Two flagship arguments:
  - *Frankenbook* = composing from fragments without information-design discipline: "all the pieces extracted
    from an original set of books... threaded together into one monstrous info-glob" [A-baker-frankenbooks-must-die-2012].
    Its most actionable idea is a design lever, not a prohibition: "make sure that Frankenbooks are NOT the path
    of least resistance... easy to create a Frankenbook with a map; hard with a query." The compose *mechanism*
    shapes the outcome. +SURE this is the load-bearing reframing for Dorc.
  - *Reuse is a tactic, not a strategy*: "a single source of text" ≠ "a single source of truth." BUT the
    disconfirming nuance that matters most for Dorc: **"reusing text where you would have been writing
    substantially the same text anyway is clearly the right thing to do... reusing common material between
    [product variants] only makes sense"** [A-baker-reuse-tactic-not-strategy-2014]. Dorc's ~17 sibling
    command-substitution codes sharing one "what unmodeled means" paragraph is *exactly* Baker's blessed case
    (same text you'd write anyway across variants), not his condemned case (collapsing distinct expressions of
    an idea). +SURE this distinction lands in Dorc's favour for the specific shared-concept-across-siblings use.
- Style-guide rulings on repeat-vs-link:
  - Google (canonical, verified): **inline your OWN context** — "provide help in context rather than linking...
    Define a term. Briefly explain a concept" — and link OUT only for *someone else's* standards/product;
    "each link is a chance for the reader to leave the page and lose their place" [A-google-style-cross-references-2025].
    Dorc's shared concepts are its own content, so this rules toward inlining, not linking. +SURE.
  - Write the Docs ARID (canonical, verified): "Accept (some) Repetition In Documentation" — strict DRY fails
    for prose — YET "eliminate content overlap between separate SOURCES" to prevent parallel maintenance
    [A-writethedocs-principles-2024]. The resolution (repeat in *output*, unique in *source*) is nearly a spec
    for Dorc's decomposed-template / inlined-render model. It also states the authoritative norm: "incorrect
    documentation is worse than missing documentation." +SURE.
- Scope caveat: even the DITA critic concedes structured single-sourcing pays off "for large teams trying to
  standardize... in global contexts" and still *likes the conref/keyref reuse primitives themselves*
  [B-tomjohnson-10-reasons-dita-2015]. The failure is drafting/tooling discipline, not the reuse primitive.
  ~SUSPECT (much Thread-1 failure evidence is one author + vendor blogs; the independent corroboration is Write
  the Docs and the Sphinx/mdBook include docs).

## goal-3 — error/teaching surfaces (rustc, Clang, Elm, staticcheck)

**All surveyed error surfaces sit on a pole; none occupy the inline-transclusion middle. The most valuable
lesson is Clang's, and it is about AI-authorship, not structure.** +SURE.

- rustc `--explain`: per-code markdown, one standalone `E####.md` per code, no cross-page dedup (pole a);
  mandatory for new codes; teaching-oriented ("help users understand *why*"); **examples are doctested** via the
  error-index-generator — the honesty mechanism directly analogous to Dorc's executable transcript cases
  [B-rustc-dev-guide-error-codes-2026]. Live-target note: the governing format RFC 1567 is flagged "largely
  outdated" and being re-standardised in 2026 (draft RFC 3370). +SURE.
- Clang: **already has per-diagnostic TableGen doc-strings** (`DiagnosticDocs.td`, `gen-diag-docs`) but "99% of
  diagnostics do not have documentation." When a contributor proposed AI-generating the missing docs, maintainers
  rejected it on the norm that **"users... ought to be able to expect their compiler vendor's docs to be
  authoritative and because of that also 100% correct"**, and "incorrect documentation is worse than no
  documentation"; a maintainer also argued docs should be "versioned with clang versions"
  [B-clang-diag-docs-rfc-2024]. This is the sharpest datum for Dorc's AI-mass-authorship wrinkle: the exact wall
  Dorc's uniquely-permitted AI authorship must clear, and it is amplified by inlining (one wrong shared paragraph
  is wrong on every page it reaches). +SURE.
- Elm: the terminal error links OUT to 16 standalone per-CONCEPT hint files (`hints/*.md` — bad-recursion,
  comparing-custom-types, import-cycles...), reused across many error scenarios (pole b)
  [B-elm-hints-comparing-custom-types-2019]. Note the granularity: hints are per-*concept*, not per-error-code,
  which is exactly Dorc's "families share background concepts" structure — but Elm links to them rather than
  inlining. Elm's error-message-catalog ("a collection of Elm programs that trigger error messages") is a direct
  ancestor of Dorc's transcript-case corpus [B-elm-compiler-errors-for-humans-2015]. +SURE.
- staticcheck: ~150 checks, code-prefixed (SA1000...), each a standalone generated Explanation, clustered into
  families (SA1 stdlib, SA2 concurrency, SA4 no-op) [C-staticcheck-checks-index-2026] — the pole-(a) model at
  Dorc-like scale-and-family-clustering. +SURE.
- On "do users follow explain pointers": I found no hard usage data either way. The indirect signals are Elm's
  and Clang's assumption that the linked/looked-up page must stand on its own, and Google's cognitive-load
  argument that every link loses some readers. --WONDER on actual follow-through rates (a genuine evidence gap;
  no source measured it). I did not separately mine shellcheck's wiki or TypeScript/Clang-generated-doc usage;
  characterised shellcheck only as per-code community wiki pages from search results — ~SUSPECT on its drift/
  maintenance specifics.

## goal-4 — decorrelated seam evidence (legal assembly; translation memory)

**Two independent domains that ran the "assemble prose from a shared parameterized library" experiment for
decades converge on the same lesson: the dangerous seam is semantic/contextual, the safe granularity is the
self-contained unit, and the cure for drift is single-source + read-as-whole.** +SURE.

Legal document assembly:
- The peer-reviewed anchor [A-blackhole-boilerplate-choi-gulati-scott-2017] (VERIFIED in my own context): reuse
  "without reflection" erodes shared meaning ("rote usage," "encrustation," the "black hole"), and normal
  error-correction feedback fails exactly where reuse is blind. The decorrelated model of Dorc's worst case: a
  shared concept-paragraph that silently no longer fits its host as siblings diverge, unflagged.
- The practitioner consensus runs OPPOSITE to the naive worry: duplication is the villain
  ("never seen a firm keep duplicated common language consistent," [B-henley-draft-like-a-pro-2018]); single-
  source assembly is "a complete fix" [B-adams-dysfunction-causes-cure-2013]; sharing "flows through to
  everything that depends on it" but raises ownership/versioning stakes [B-avvoka-maintenance-at-scale-2026].
  [graded-by subagent for these three; I re-verified only the Black Hole anchor.]
- Parameterization limit: "templates become brittle... deeply nested conditional statements... collapse under
  their own weight"; fix = keep the fragment DUMB, pre-resolve logic to flags, reference a centralized clause
  library by placeholder [B-automationlogs-common-mistakes-2026]. Independent restatement of the Wikipedia Lua
  arc. [graded-by subagent.]
- The interpretive norm and a real seam-failure: Yieldpoint v Kimura [2024] EWCA — a multi-million-dollar dispute
  turned on special conditions inserted by *adaptation* onto a template; the court's cure was to "read all the
  contractual provisions together" [B-yieldpoint-frankenstein-slip-2024]. Seam harm lands at the parameterization/
  graft point; the governing norm is read-as-coherent-whole. [graded-by subagent.]

Translation-memory segmentation:
- The granularity cautionary tale in named form [B-tm-friend-or-foe-deck-2018]: "peep-hole translation"
  (authors phrase text to be recyclable → less cohesive); "Sentence Salad" (each segment grammatical, the
  paragraph incoherent — "caused by translator's working on one segment at a time"); "Primacy of Existing
  Translation" (awkward-but-not-wrong reuse gets accepted, entrenching bad style). When the tool's *unit* is the
  reuse unit, that unit becomes the ceiling on cohesion. This is the strongest single warning against reuse at
  too fine a granularity — and it validates Dorc's paragraph-floor (a paragraph carries its own internal
  cohesion; a sentence does not).
- The counter-datum and the feasibility proof [B-memoq-segmentation-2025]: memoQ's co-founder argues the answer
  is NOT to abolish segmentation but to keep BOTH views — "a TMS must be able to look at the same content through
  various levels of segmentation, or without it" — shipping "unsegmented preview next to the segmented content
  view." Decomposed store + composed view is a defended, shipped pattern. Dorc is not inventing that half.
- (Literate-programming transclusion, noweb/org-mode, was in scope as optional and I did not pursue it — the
  legal + TM evidence already saturates goal-4. Flagged, not chased.)

## goal-5 — authorship-in-situ (THE CORE cross-cut: where the EDIT happens vs where the READER reads)

**Edit-through-a-composed-view is a real, shipped pattern in two systems; the universal failure everywhere else
is edit-in-a-vacuum; Dorc's premise lands on the right side of that line — but no system has proven it at
teaching-prose granularity.** ~SUSPECT.

- It exists natively: Wikipedia composite pages let you edit a transcluded section *from the composite view*, and
  the save propagates to the source [B-wikipedia-transclusion-help-2026]. (But mainspace norm discourages storing
  article text in templates precisely because it is "more difficult to edit" — so the edit-through capability
  exists yet is steered away from for article prose.)
- It is defended as architecture: memoQ's segmented-store / unsegmented-composed-view [B-memoq-segmentation-2025].
- Everywhere the edit happens in a fragment vacuum, coherence rots: Frankenbook [A-baker-frankenbooks-must-die-2012],
  fine-chunk translation [B-baker-chunking-translation-2011], Sentence Salad [B-tm-friend-or-foe-deck-2018],
  context-unaware conref [C-stilo-conref-limitations-2023]. The consequence for contributor behaviour is concrete:
  editing "In chunk_a make these changes, in chunk_r make these changes" is the named pain
  [B-tomjohnson-single-sourcing-over-2025].
- What "held" when edit-in-context was preserved: the single annotated in-context template (legal,
  [B-henley-draft-like-a-pro-2018]) and "provide the context on the page" (Google, [A-google-style-cross-references-2025]).
- The residual risk that in-composed-view authoring does NOT solve by itself: the SEMANTIC seam. You can edit a
  shared paragraph in one host's full context and still break its fit in the *other* hosts you can't see. That is
  the Black Hole failure [A-blackhole-boilerplate-choi-gulati-scott-2017] and the reason the legal norm is
  read-*all*-hosts-together. +SURE this is the unsolved-by-compose-view residual; it points at a
  fit-across-all-hosts check as the thing the design must supply, but naming that mechanism is out of scope here.

---

## Evidence AGAINST inline prose transclusion (disconfirming collection)

Collected deliberately; this is the case a design adjudication must clear.

1. **The rotted middle IS mid-size shared prose paragraphs.** Techcomm's survivors are the extremes (whole-file
   or context-free atom); the exact granularity Dorc wants is the one that failed in practice.
   [B-tomjohnson-single-sourcing-over-2025], [C-stilo-conref-limitations-2023], [B-mdbook-includes-2024]. ~SUSPECT
   the failure was granularity-intrinsic vs discipline-intrinsic (the sources attribute it to out-of-context
   editing, which Dorc changes).
2. **Wikipedia had the machinery and chose against it for prose.** With full transclusion + LST + {{Excerpt}}
   available, the article-prose norm is summarize-and-link + accepted duplication; prose transclusion is a
   cautious exception and mainspace discourages template-stored article text.
   [B-wikipedia-summary-style-2026], [B-wikipedia-transclusion-help-2026]. +SURE this is the chosen norm;
   ~SUSPECT whether it reflects a considered rejection or MediaWiki path-dependence.
3. **The semantic seam is real and un-caught.** Blind reuse erodes fit-with-host, and the error-correcting
   feedback of normal writing does not fire. [A-blackhole-boilerplate-choi-gulati-scott-2017]. +SURE.
4. **Parameterized prose has a hard complexity ceiling.** Both Wikipedia (→ Lua priesthood off the authorable
   surface) and legal templates ("collapse under their own weight") hit it; the moment reuse needs agreement/
   pluralization, template syntax is insufficient. [B-wikipedia-lua-2026], [B-automationlogs-common-mistakes-2026].
   +SURE (and this actually supports Dorc's anti-goal, but it is a hard boundary on how far the middle can scale).
5. **AI-mass-authorship amplifies the correctness risk under inlining.** The authoritative-docs norm is strict,
   and one wrong shared paragraph is wrong on every page it reaches. [B-clang-diag-docs-rfc-2024],
   [A-writethedocs-principles-2024]. +SURE the norm exists; ~SUSPECT how much inlining amplifies vs duplication
   (duplication spreads the same wrong text too, just without single-point-fix).
6. **The seam bites hardest at the parameterization/graft point**, with litigated real-world harm.
   [B-yieldpoint-frankenstein-slip-2024]. -0/~SUSPECT (single case, law-firm secondary note).

Countervailing (why the case-against is not decisive): every item 1–2 failure is attributed by its own sources
to authoring-out-of-context, which Dorc's compose-view premise directly changes; items 3–6 identify *what to
engineer against* rather than showing the middle cannot work.

---

## Surprises (valuable, outside the goal-set)

- **The naive worry is partly inverted.** The decorrelated legal domain's verdict is that DUPLICATION, not
  sharing, is the documented drift-driver; single-source is the cure. The seam risk survives but relocates from
  visual/coherence (imagined) to semantic fit-drift (real). [B-henley-draft-like-a-pro-2018],
  [A-blackhole-boilerplate-choi-gulati-scott-2017].
- **"Both views" is named prior art.** memoQ's segmented-store + unsegmented-preview is a shipped,
  practitioner-defended instance of decomposed-store / composed-view — Dorc is not inventing that half.
  [B-memoq-segmentation-2025].
- **ARID is almost a spec for Dorc's model.** Write the Docs' "repeat in output, unique in source," reached
  independently by the docs community, maps directly onto decomposed-template / inlined-render.
  [A-writethedocs-principles-2024].
- **The real design lever is incentive-shaping, not a yes/no on transclusion.** Baker: "easy to Frankenbook with
  a map, hard with a query" — the compose mechanism decides the outcome; make coherence the path of least
  resistance. [A-baker-frankenbooks-must-die-2012].
- **The canonical pole-(a) prior art is itself mid-redesign.** rustc's long-explanation format (RFC 1567) is
  being re-standardised in 2026 (draft RFC 3370) — a live target worth watching before treating rustc as a fixed
  reference. [B-rustc-dev-guide-error-codes-2026].
- **Dorc's grammatical-composition anti-goal has strong decorrelated corroboration.** Two unrelated domains
  (Wikipedia templates, legal templates) independently hit the same wall and either escaped into a scripting
  priesthood or told authors to keep fragments dumb. [B-wikipedia-lua-2026], [B-automationlogs-common-mistakes-2026].

---

## Evidence gaps / what I did NOT chase (for honest triage)

- No hard usage data on whether users follow `explain` pointers (--WONDER; genuine gap).
- shellcheck wiki drift/maintenance and TypeScript diagnostic docs: characterised from search results only, not
  fully read (~SUSPECT). Error-surface coverage is carried by rustc/Clang/Elm/staticcheck.
- Wikipedia high-risk-templates / TemplateData protection tiers: assembled from adjacent pages, not the dedicated
  page (~SUSPECT on finer specifics).
- Literate-programming transclusion (noweb/org-mode): in-scope-optional, not pursued; goal-4 saturated without it.
- Six legal-domain grades are `[graded-by subagent]` (the gathering subagent read them; I re-verified only the
  A-grade Black Hole anchor in my own context). Treat those six as provisional-but-corroborated.
