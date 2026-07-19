# errorloom

**Executable transcript cases as the authoring surface for a CLI tool's
user-facing prose.**

Most tools store their error/help/status strings in a catalog — a table of
message templates — and authors edit that table. The problem: whoever writes a
string at line 700 of a catalog is in *tool-author headspace*. They know things
the user doesn't; they can't see the carets, the surrounding output, or what the
message looks like in context. The prose drifts away from what a user actually
experiences.

errorloom inverts the direction. The authoring surface is the **executable
transcript case**: a recorded run of the tool (input state + the exact bytes the
command printed). Authors — human or LLM — edit the *rendered transcript*, seeing
exactly what a user sees. errorloom then extracts the edits back into the catalog
by a word-level diff, attributed through the render's own provenance. The catalog
becomes a derived artifact; the committed transcript is the source of truth.

Lineage: cram / Mercurial t-tests, Go `txtar` + `testscript`, `rustc tests/ui
--bless`, insta. The one genuinely novel leg is the **diff-driven extraction of
edits back into the message catalog** — this crate.

## What this crate is (and isn't)

This is the **layer-1 transport engine** — the pure, deterministic core, generic
over an opaque consumer key, holding no types from any particular tool:

```
promote(baseline: &TaggedRender<K>, edited: &str, params: &ParamTables<K>)
    -> Result<PromoteOutcome<K>, Refusal<K>>
```

- **`TaggedRender<K>`** — a baseline render's bytes plus a `Span` map that
  classifies every run as a `Region`: `TemplateLiteral` (the field's own prose,
  the only editable class), `ParamValue` (interpolated data), `ForeignText`
  (passthrough), or `Arrangement` (render-owned structure). The consumer's
  renderer produces this; errorloom validates it is a gap-free cover.
- **`promote`** — word-diffs the baseline against the edited text, attributes
  each change through the span map, re-holes instantiated param values back into
  `{holes}`, and returns the new stored `FieldTemplate` per edited field.
- **`Refusal`** — a closed set of blunt refusals (edited payload, edited
  structure, ambiguous attribution, ambiguous re-hole, contradictory edits). No
  suggestions, no fuzzy matching: it dumps both word streams, the region table,
  and the offending hunk, and the caller exits nonzero. This is internal tooling
  with sharp edges, not a gradual-enhancement product.

The one hard-tested guarantee: an edit confined to one template region round-trips
exactly (promote → regenerate → re-render reproduces the edited words), modulo
whitespace normalization. It is checked by a seeded property test over
randomly-generated fake consumers.

**Not here (later layers):** the txtar + YAML-frontmatter case container, the
sequential replay runner, bless-mode orchestration (prose-bless vs
structure-bless, the git gating), a CLI, and a full toy end-to-end consumer. The
public API is shaped for those consumers, but this crate builds none of them.

## Status

Pre-1.0, experimental, extracted from a design spike. `publish = false` until a
LICENSE is chosen. The prose model deliberately starts small — words and
paragraphs only; paragraph *restructuring* (adding/removing breaks) is not yet
supported and refuses. Expect the API to move.
