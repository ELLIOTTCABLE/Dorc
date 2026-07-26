# 28I — the webhost red-line extraction (direction-by-markup, distilled)

AI-authored (Fable conductor, 2026-07-26). The human red-lined
`whygallery-webhost-whole.loom` @ `f4f48316` ("Wishful-thinking redline;
non-functional / aspirational") as direction-by-markup — deliberately
non-compiling, "mildly discardable", written because markup was cheaper than
prose. This note is the conductor's distillation, per the human's license; the
`28E` §8 markup-round precedent applies (deltas are design directions extracted
by demonstration). Weight: stronger than chat-leans, looser than rulings — the
human explicitly did not sweat compilable precision. The commit itself is
expected to be discarded by its author.

## Directions extracted

- **red-site-comment-correlation-form** (the `28H:rul-fixture-records-enriched-
  not-reduced` lean made CONCRETE): site lines gain aligned trailing comments,
  `site 4 effect=absent rc=1  # 12|apt-get (Package:oldpkg)` — the `N|command`
  reference form, command word only (no argv), then an OPTIONAL parenthesized
  coordinate hint: `(...)` when uninteresting, real coordinates when
  load-bearing, several comma-joined when one site measures several cells.
  Kind-prefixes DROPPED in the hint (`Package:oldpkg`, not `sm.dorc.…`) —
  display brevity in a comment, not a wire form. Columns aligned. → span lane
  adopts for new/regenerated fixtures (writer-side; the reader already
  tolerates `#` lines).

- **red-simple-invocations-concretized** (`28H:item-simpler-why-invocations`
  now has target spellings, sketch-tier):
  - book as a POSITIONAL: `dorc apply … book.sh -o …`, no `--book=`.
  - explicit MODE word where the old replay had none.
  - `dorc why --risk-faultless-skips` — otherwise BARE: book, oracles, records
    all recovered from the receipt.
  - `dorc why book.sh:15 --all` — file-qualified address positional, nothing
    else needed.
  - NEW FLAGS sketched on the apply replay: `--no-dispatch --dump` — the
    harness/spike posture spelled EXPLICITLY as product flags, ~SUSPECT so the
    bare form stays reserved for the real executor era and a replay command
    reads as what it actually does. Sketch-tier; the mode-vocabulary question
    is a product decision, not a lane rider.
  - **ask-why-carries-risk-flag-or-reads-receipt** (--WONDER, needs the human):
    the sketch passes `--risk-faultless-skips` to `dorc why` even though the D1
    work reads the risk-profile from the receipt's RECORDED argv. Deliberate
    (re-consent to render survived elisions?) or sketch-slip? One-line answer
    wanted before the span lane's why-arm bakes an invocation shape.

- **red-artifact-regeneration-header** (new surface direction): rendered
  artifacts open with a product-voice header — "COMPILED APPLY OUTPUT", an
  exact scoped regeneration command, an inputs list with per-file SHA256
  digests, and an explicit "feel free to remove this header if you manually
  edit; Dorc does not require it to proceed" note. The old header (wire-grammar
  teaching prose about the return channel) leaves the artifact entirely —
  product bytes stop carrying harness/teaching voice.
  - **ask-apply-header-vs-byte-floor** (design tension, flag not fold): the
    two-surfaces law (rec-1) holds the shipped apply artifact byte-floored and
    receipt-free, "byte-identical under receipt-stripping". A removable
    digest-bearing header is plausibly exactly the sanctioned receipt-material
    shape (strippable ⇒ floor intact) — but that reading should be RULED, not
    assumed, before anyone builds the header.
  - NB the sketch attaches the header to the first artifact in the round-trip
    stream (position-wise the probe); the label says APPLY. Read as loose
    sketch: the direction is "artifacts carry regeneration headers", not a
    per-artifact placement spec.

- **red-fixture-comments-product-voice**: fixture book/oracle comments
  hardwrap ~80 cols; harness-internal chatter leaves product-shaped text (the
  "every tool here is an INERT mock" line dropped from the book comment — it
  belongs to the case's harness sections, not the book; the
  `strawman24-errexit-defeats` corpus-slug citation dropped from a
  product-shaped comment, replaced by plain-language reasoning). These are
  INPUT edits — the human lands them via their own loom edits (the
  structure-bless path, once it exists); builders do not sweep fixture comments.

- **red-why-render-nits**: the receipt header block appears on per-line asks
  (`dorc why book.sh:15 --all`) too, not only argless why · participating-lines
  gutter numbers align to the left margin (`12 | apt-get…`, not indented under
  the bracket) · a blank separator line between the records-end sentinel and
  the apply artifact's shebang in the round-trip stream. Render-form-unwelded
  territory; adopt opportunistically where W4 lanes touch those seats, W5
  otherwise.

## Standing lens (human-typed, same sitting; binds ALL loom-infra work)

**product-vs-internal-carve**: errorloom is a product too. Every loom-infra
change asks: would a third party want this? does it belong in errorloom's
narrow scope — genuine quality-of-life for ANY errorloom user — or is it bloat
tied to Dorc's specifics (which belongs in dorc-loom / the consumer)? Aligns
with `282:rul-generic-executor-consumer-dispatch` (generic errorloom knows no
Dorc names) and the `289` publish-narrow lean; propagate into every
loom-touching brief.
