# `dorc-receipt-local`

Default local keyset and immutable receipt-store edge. It stores exact signed bytes and supplies
validated capabilities; it does not interpret receipt semantics, render aid, or select algorithms.

- **`inv-roots-arrive-as-values`** — read no environment. CLI resolves controller roots once;
  explicit receipt-store siting never changes the standard configuration/key root.
- **`inv-read-and-write-opens-are-distinct`** — read-only open creates or repairs nothing and cannot
  narrow into write capability. Write readiness requires the complete validated keyset and required
  synchronization results.
- **`inv-keyset-completes-manifest-last`** — generate both independent keys before durable mutation;
  create exclusively; write/sync key documents before the manifest; reopen through the ordinary
  validator. Any existing incomplete/damaged member refuses use and never triggers regeneration,
  repair, or deletion.
- **`inv-store-is-immutable`** — the store mints typed final names and creates without replacement.
  No append, mutable latest pointer, fallback name, automatic retention, or unrelated cleanup.
- **`inv-owned-handles-authorize-operations`** — a pathname string is never enough to read, replace,
  or remove an entry. Use retained validated handles/owned-entry tokens; cleanup consumes only the
  object identity this attempt still owns.
- **`inv-every-io-act-is-injected`** — production and deterministic model implement the same sealed
  `LocalIo` operation vocabulary. Keep create, write, sync, inspect, enumerate, read, and owned
  removal separately faultable; add no convenience filesystem bypass.
- **`inv-bounds-precede-retention`** — apply file, entry-count, aggregate, and key-document limits
  before allocation or collection. Enumeration counts every entry through limit+1 before filtering;
  writer bounds never substitute for reader bounds.
- **`inv-platform-properties-stay-separate`** — represent file sync and directory-sync availability
  as independent typed properties; do not place Unix and Windows guarantees on one ordered grade or
  claim parity one platform cannot establish.
- **`inv-store-never-interprets-content`** — semantic parsing, signer trust, overlay validation,
  graph correlation, and report construction remain in `dorc-receipt`/CLI. Filename/header agreement
  is a store finding, not authority.

Provider additions, root-policy changes, key lifecycle/rotation, mutable storage, or weaker
publication paths require explicit human direction and quarantined review. Do not generalize the V1
local edge speculatively.
