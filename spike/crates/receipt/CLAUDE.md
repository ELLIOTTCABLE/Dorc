# `dorc-receipt`

Pure recorded-data boundary: exact receipt format, models, graph, source/locator custody,
and the public report API. No filesystem, environment, provider implementation, or rendering.

- **`inv-literal-document-bytes`** — verification and parsing consume the same immutable byte
  slices. Never normalize, reserialize, recover unknown fields, or accept alternate spellings.
- **`inv-reader-writer-states-only-narrow`** — advance only through the private typestate
  transitions. Keep trust, completeness, species, projection, and detail availability separate;
  partial never becomes complete by conversion.
- **`inv-plain-rich-shapes-differ`** — plain cannot represent opaque detail. Rich owns exactly one
  fully-accounted reverse overlay; validate every slot in both directions before releasing detail.
- **`inv-recorded-values-stay-recorded`** — `Reingested`, recorded influence, dispositions,
  locators, and report facts never convert to live claims, licenses, plans, or operation inputs.
  Re-derivation produces receipt-owned report types only.
- **`inv-report-is-the-public-read-boundary`** — consumers read recorded explanations through
  `receipt::report`. Its fields are closed typed states; arbitrary values have no raw/string or
  revealing formatting access and leave only through the class-aware encoder interface.
- **`inv-graph-edges-are-explicit`** — correlate only typed receipt identities. Missing edges stay
  missing; filename/order never mints an edge, and disconnected graphs never join one question.
- **`inv-source-locators-keep-byte-fidelity`** — general-sh source content and locator spans use the
  exact acquired-byte domain. Durable locators mirror the existing stage DAG and never convert back
  to a live locator; do not add line-only or fuzzy/moved-source identity.
- **`inv-identities-never-cross-domains`** — preserve species-, key-role-, plan-, image-, and
  presentation-specific newtypes and domain encodings. A digest or file-provided id grants no
  stronger state.
- **`inv-format-changes-are-one-cutover`** — this is unpublished: reshape writer, reader, vectors,
  limits, and all callers together; add no compatibility parser or alias.

Changes to grammar/content, trust or projection states, authority-capable mints, graph semantics,
or arbitrary-value exits require explicit human direction and quarantined review. Stop rather than
locally widening one to unblock a caller.
