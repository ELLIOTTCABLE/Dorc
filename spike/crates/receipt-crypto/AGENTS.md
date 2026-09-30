# `dorc-receipt-crypto`

Concrete cryptographic and canonical-key-document adapters for `dorc-receipt`. Return primitive
results only; receipt trust, completeness, projection, and publication states are minted elsewhere.

- **`inv-algorithms-are-version-fixed`** — V1 is Ed25519 over the receipt-owned signing input and
  Age v1/X25519 for one rich overlay. File bytes never select algorithms, plugins, backends,
  passphrases, SSH recipients, or fallback behavior.
- **`inv-key-roles-never-meet`** — generate signing and encryption material independently. Keep
  documents, ids, APIs, and stored roots non-convertible; derive ids from public material only.
- **`inv-key-documents-are-library-owned`** — signing private material is canonical PKCS#8 DER;
  encryption material is one canonical Age identity line. Parse and reserialize with the owning
  library and require byte equality; invent no wrapper or alternate encoding.
- **`inv-private-material-has-one-narrow-exit`** — private documents/keysets are non-Clone,
  non-Default, non-serde, non-comparable, and content-redacted in `Debug`. Canonical bytes leave only
  through the scoped write callback; add no raw accessor.
- **`inv-adapters-do-not-own-policy-or-io`** — no filesystem, environment, root resolution,
  provider discovery, diagnostics, or receipt-state minting. Generation randomness enters only
  through the existing key-generator capability.
- **`inv-dependency-surface-is-deliberate`** — keep unused Age/Ed25519 features off. Algorithm,
  package, or feature changes move vectors and the lockfile under explicit review; do not add custom
  framing or cryptographic glue locally.

The rationale behind these fixed boundaries is quarantined. If a task appears to require crossing
one, stop for explicit human direction rather than introducing a convenience path.
