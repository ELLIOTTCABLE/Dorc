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

Lineage (steal, don't invent): cram / Mercurial t-tests, Go `txtar` +
`testscript`, `rustc tests/ui --bless`, insta, terraform-plugin-docs. The one
genuinely novel leg is the **diff-driven extraction of edits back into the message
catalog**.

This is self-consumed internal tooling. It does not get the gradual-enhancement,
friendliness, or accountability properties of a shipped product — sharp edges are
fine; it just needs to work. Refusals are blunt dumps.

## The case-file format

A case is one txtar archive with a `---`-fenced flat-YAML frontmatter head:

```
---
code: motd-refused
when-fires: the leaf-exact render would elide a heredoc-bearing leaf
---
-- book.sh --
#!/bin/sh
cat <<EOF >/etc/motd
hello
EOF
-- probe-results.txt --
site 0 effect=holds
-- replay --
$ mytool plan --book=book.sh < probe-results.txt
render: error[motd-refused]: refusing to elide the heredoc-bearing leaf
$ mytool plan --book=book.sh --format=jsonl < probe-results.txt
{"envelope":"lint/1","code":"motd-refused"}
```

- **Frontmatter** is an opaque flat map to errorloom (`key: value` scalars and
  `key:` + `- item` lists; nested structures refuse). The schema — which keys mean
  what — belongs to the consumer.
- **File sections** are materialized verbatim (LF-only; CRLF refuses) to a temp
  dir before the replay runs. Names may be `/`-joined paths; absolute or
  `..`-climbing names refuse.
- **The replay section** (always last) is a sequence of `$ `-prefixed command
  blocks, each followed by exactly what the command printed. Commands run
  **sequentially in one shared temp cwd**, so state flows between them by design.

### Case-hygiene gates

Blunt refusals, all generic: CRLF in any section; a replay-output line that parses
as a txtar marker (there is no escaping — a committed file could never hold it, so
this is caught on fresh output at bless); a captured line leaking the sandbox's
absolute path (cwd is the temp dir, so paths render relative); and a configurable
**required-token** gate — name a frontmatter key and every replay block's output
must surface its value (e.g. every replay must mention its own `code`).

## CLI — the generic cram mode

The `errorloom` binary is the fully-generic cram tool. The environment is entirely
caller-injected (`env -i`-style): nothing ambient leaks in.

```
errorloom run   [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...
errorloom bless [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...
```

- `run` materializes each case, executes its blocks, and compares each block's
  combined (`2>&1`) output to the committed transcript. It exits nonzero on any
  drift. Byte stability under re-execution is the run gate.
- `bless` re-runs and re-inlines each block's actual output (structure-bless), then
  applies the hygiene gates and writes the case back.

`--path` dirs are searched for each command's program and become the child `PATH`;
`--env K=V` sets exact variables. Policy such as inert mocks is the consumer's — a
`--path` pointing at a mocks dir keeps real tools out.

The **prose-promote** flow is deliberately *not* in the CLI: it needs consumer
callbacks (see below).

## Library — the promote flow and the two bless modes

Prose-promote is library-only because it needs a [`Consumer`] (a baseline tagged
render for a case · apply field-edits · re-render a case) and a two-method [`Git`]
(`head_version_of` · `dirty_paths`; a subprocess-`git` impl ships, a `FakeGit` is
provided for tests). errorloom drives the loop; the catalog and case schema stay
consumer-side.

Two bless modes under a **mechanical exclusivity law — never both in one bless**:

- **prose-bless** (`prose_bless`): structure is frozen; an author edited words in a
  transcript. Legal only when the touched-set is case-files-only and the catalog is
  clean. errorloom re-renders each dirty case from the current catalog (the
  baseline), word-diffs it against the author's edited transcript, attributes each
  change through the span map, re-holes param values, regenerates the catalog, and
  re-renders the corpus. A **baseline-verify** re-renders with current state and
  requires it to match HEAD's transcript *everywhere except prose regions* — a
  mismatch means the structure moved, so it refuses with "structure-bless first".
  This verify *is* the never-both law.
- **structure-bless** (`structure_bless`): catalog prose is frozen; code or
  arrangement changed. Legal only when the touched-set is code-only with case prose
  untouched. Every transcript is regenerated from scratch; prose provably cannot
  have drifted because it only flows from the unchanged catalog.
- Both classes dirty, or a dirty (hand-edited) catalog, **refuse**.

The **CI fixpoint gate** (`fixpoint_check`): every committed case must re-render to
its own committed bytes. A catalog hand-edit — prose or metadata — moves the render
off the committed transcript, so the gate catches it. This is what lets the
promote flow be trusted without a hand-maintained authorship roster.

## The span-map contract

A consumer's tagged renderer hands `promote` a [`TaggedRender`]: the rendered bytes
plus a [`Span`] map classifying every run as a [`Region`]. The map is validated on
construction to be a **gap-free, non-overlapping total cover** of the render bytes
— region lookup must be total. Whitespace and other structure between prose runs
are covered as `Arrangement` runs; there are no holes.

The regions:

- **`TemplateLiteral { key, paragraph, instance }`** — the field's own prose words,
  the ONLY editable class.
- **`ParamValue { key, param, instance }`** — interpolated payload; editing it
  refuses (it is data, not prose).
- **`ForeignText { param }`** — passthrough foreign text; editing it refuses.
- **`Arrangement { slug }`** — render-owned structure (numbering, connectives, tier
  words); edit it by structure-bless, not prose-bless.

**Instance discriminators** (`instance: Option<InstanceId>`,
`28A:rul-tagged-render-emits-instance-ids`): a field may render more than once in a
transcript. When a consumer stamps each render with an `InstanceId`, promote groups
spans into instances by *exact* identity; when absent, it falls back to structural
inference (paragraph/adjacency). Opting in is per-key all-or-nothing.

## Status

Pre-1.0, experimental, extracted from a design spike. `publish = false` until a
LICENSE is chosen. The prose model deliberately starts small — words and
paragraphs only; paragraph *restructuring* (adding/removing breaks) refuses.
Expect the API to move; growable public enums are `#[non_exhaustive]`.
