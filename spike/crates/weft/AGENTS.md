# crates/weft — the firewalled formatting engine (steering law)

Weft is the segregated render/layout mini-product (`28E:rul-tree-render-is-a-
firewalled-crate`; conduct record `Research/notes/28F`). Consumers hand it
well-structured, homogeneous, totalistic data; it prints. Registry discipline:
one rule per bullet, greppable slugs, APPEND to sections.

## Firewall & dependency law

- **weft-deps-nothing** — zero dependencies, std only; weft knows no Dorc type.
- **weft-one-way-firewall** — engine → adapter → weft, one direction only.
  `dorc-core`/`analysis`/`plan`/`oracle` must NEVER dep weft; printing can
  never drive engine logic; the adapter seat is `aid::weave` + cli composition.
- **weft-geometry-vs-words** (`28F:rul-weft-geometry-vs-words`) — weft
  self-mints WORDLESS geometry only (`===` frames, ` | `, quotes, truncation
  glyphs), stamped `Arrangement`/`Face::Part`-keyed; every English word —
  headers, tier verbs, labels, the `OR` connective — arrives from the consumer
  as a row-backed run. No word-shaped literal may be born inside weft.

## Layout law

- **weft-pure-layout** (`28E:prop-pure-layout-function`) — render is a pure
  function of (tree, width): no clock, RNG, fs; resize = recompute; goldens pin
  at 80 and 40; width-40 stays readable by wrapping, never silent truncation.
- **weft-named-table** — cross-box alignment is a NAMED TABLE: rows join by
  name; the table resolves all column stops in ONE left-to-right prefix-sum
  pass in the measure walk (`measure.rs`); wrapping happens strictly after
  stops fix — no fixpoint, only ordering. Structural proximity is the
  anonymous-table case of the same code; never re-introduce per-column or
  ad-hoc local measurement (the withdrawn attempt and its failure mode are in
  `git log -- spike/crates/weft`).
- **weft-layout-asserts-measure** (`28F:rul-layout-asserts-measure`) — the
  paint walk asserts each member's actual left edge against the measure walk's
  prediction; the width-sweep test drives it 12–140. Any divergence is a bug in
  whichever walk drifted; never weaken the assert to ship.
- **weft-table-degrades-whole** (`28F:rul-table-degrades-whole`) — a table
  stacks/hangs as a UNIT: any member's vote stacks every member, and stacked
  bodies indent from the table's stop so a degraded table stays square. Code
  blocks contribute minimum 0 and never vote (they overrun byte-honest).
- **weft-gutter-is-a-lead** — a rank gutter is a LEAD folded into `stop[0]`,
  not a column of its own.

## Output & provenance law

- **weft-ascii-forever** (`28E:rul-ascii-output-forever`) — every output byte
  is printable ASCII or `\n`; enforced by test; permanent.
- **weft-total-cover-spans** — render output is (bytes, total-cover span map):
  every byte belongs to exactly one provenance run; skeleton whitespace is
  Arrangement. The map is the loom transport's attribution authority; never
  make it optional.
- **weft-foreign-encodes-at-mint** — not-ours bytes enter ONLY through the
  foreign constructor, which encodes at mint (`\xNN` outside `0x20..=0x7e`;
  backslashes stay verbatim so authored sh reads as written — the documented
  ambiguity is the accepted cost). Foreign runs are never editable prose.
- **weft-register-slot-reserved** — every node carries the register/density
  slot and criticality mark (`28E:prop-register-per-node`; kTASTE's data-model
  law): semantics marks critical-vs-summarizable, the renderer rules layout
  (`28E:rul-renderer-owns-layout`). Do not foreclose the two-goals×densities
  model; do not build register machinery here without a conductor ruling.
- **weft-joins-are-dag-shaped** (`28E:nit-why-steps-are-a-dag`) — join nodes
  keep branch structure in the types even while renders are
  linear-with-restatement; retrofitting DAG shape later is high-lock-in.

## Deferred seams (do not build without a conductor ruling)

- The sh-formatter with teeth · sh highlighting (own-lexer only, when it
  comes) · ANSI color (edge decoration, absent from committed transcripts) ·
  doc-algebra reflow optimization · incremental layout · TUI.
