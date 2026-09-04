# Opaque accrual review 30-reviewC

- Review identity: `30-reviewC`
- Pass: `initial` (no prior CONSTRAIN for this pass)
- Assigned range: `ai/main..19285ac5`, relayed with immutable base `fd29010c`
- Effective immutable range reviewed: `fd29010cf0f35a44292ab7a4de2eb589adf8087a..19285ac5d48e4d5d4b7e5b3cb797994325c5be17`
- Reviewed HEAD: `19285ac5d48e4d5d4b7e5b3cb797994325c5be17`
- Subject: the completed r30 `30X` testing-architecture rebuild.

## Range metadata

- +SURE The relayed symbolic base no longer resolves to the relayed hash: current `ai/main` is
  `bc7986c1ef8e6e6edb36a013c62fa78f1d4fae56`, while the relay records
  `fd29010cf0f35a44292ab7a4de2eb589adf8087a`.
- +SURE Both hashes are ancestors of the supplied tip. The three commits between them are the
  docs-only mainline recut described by the relay. The review therefore used the immutable recorded
  base, which is the conservative superset and preserves every 30X design and build commit.

## Evidence inspected

- +SURE `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` was present and read completely
  before the range was inspected.
- +SURE The immutable range's ancestry, 150-commit log, changed-path inventory, and diffstat were
  inspected. The working tree was clean before this report and inventory maintenance.
- +SURE The design of record, conductor ledger, `30-reviewB`, and the current relevant law in
  `spike/CLAUDE.md`, `spike/crates/cli/CLAUDE.md`, and `spike/crates/aid/CLAUDE.md` were inspected.
- +SURE The new composition roots and seam implementation were inspected directly:
  `cli/src/seam.rs`, `cli/src/compose.rs`, `bin/dorc.rs`, `bin/dorc-harness.rs`, `bin/dorc-sh.rs`,
  `cli/Cargo.toml`, and the moved POSIX-shell resolver.
- +SURE The process and in-process runner routes were inspected directly: runner sandbox setup,
  session environment, seam defaults, shell-session construction, scripted transport, deterministic
  `ModelIo` receipt world, and the apply receipt path.
- +SURE The existing local receipt edge was followed far enough to establish how roots select the
  keyset and store, how an existing keyset is reopened for writing, and how the selected seams feed
  identities and key initialization.
- +SURE The shipped-binary environment test, seam conversion tests, lexical fixture fences, and
  workspace target metadata were inspected. `cargo metadata --no-deps` reports `dorc-harness` as an
  unconditional ordinary binary target with no required feature.
- +SURE The supplied green test evidence was accepted as operational evidence. This review did not
  rerun the full gates; the concern is representational and composition-level, not a claim that a
  supplied test failed.

## Qualifying concern

### Fixture durable authority is represented as an unauthenticated path and can meet production state

- +SURE The range says `HarnessRootsSeam::Pinned(PathBuf)` denotes a runner-owned throwaway root, but
  `HarnessSeams::from_env` constructs it directly from any non-empty
  `DORC_SEAM_ROOTS=pinned:<text>`. It does not establish ownership, exclusive creation, containment,
  or even the documented absolute-path condition. The type therefore represents every ordinary OS
  path, including the same configuration and state roots the production row resolves.
- +SURE A loom session may export that variable before a later `dorc` block. The process driver gives
  the session unrestricted shell semantics and the in-process driver models exported assignments;
  both feed the same parser. The runner's initial scrub and initial root value do not preserve
  runner ownership after the session begins.
- +SURE The shared CLI also accepts `--receipts=<path>` below the seam. For a pinned harness bundle,
  `production_receipt_edge_over` replaces the store role with that controller path without proving
  that it remains beneath the runner-owned root. Thus even a correctly initialized root value does
  not confine every durable write the harness can request.
- +SURE These are not inert test values. `dorc-harness` drives the same `compose::run`, `NativeIo`,
  `LocalReceiptEdgeV1`, keyset opener, signer, sealer, and receipt grammar as production. If the
  path-valued harness root names an existing production profile, the write path reopens the existing
  production keyset; seeded key entropy is unused. The resulting document has production format and
  production signature standing, while its identity, time, attempt nonce, observation, or transport
  may have come from harness seams. No durable field records that provenance because byte-identical
  production semantics are an intentional test property.
- +SURE The range simultaneously declares `dorc-harness` as a normal unconditional `[[bin]]` target
  in the same package as the shipped binary. `publish = false`, the alternate name, and the no-seam
  runtime warning do not prevent ordinary release builds or artifact collection from producing it;
  no public-artifact exclusion gate exists in the inspected range.

## Qualification and delta scope

- +SURE The concern is introduced by this range. Before it, the shipped composition read fixture
  selectors directly, which was already undesirable context, but the range's advertised repair is a
  new public constructor contract, new sibling executable, new persistent shell configuration
  surface, and a shared production receipt implementation. The delta newly makes "pinned path means
  runner-owned root" a type/API assumption while discarding the fact needed to enforce it.
- ~SUSPECT The combined route meets the first qualification half. It crosses the fixture/production
  composition boundary, durable identity and signing boundary, user-authored loom/session contract,
  executable artifact inventory, and every current or future caller taught to treat
  `HarnessRootsSeam::Pinned(PathBuf)` as ownership evidence. This is not a local escaping defect.
- ~SUSPECT The combined route meets the second qualification half. Once a fixture-authored document
  is placed in a production store under the production keyset, the persisted representation carries
  no fact that can distinguish it later. Repair after continued accrual requires discarding or
  externally reconstructing provenance, auditing stores and every path-capable harness caller, or
  changing durable/public semantics. Continued use of the raw path constructor and unconditional
  binary target also turns their removal into compatibility and build-pipeline work rather than a
  narrow pre-release correction.
- +SURE The relevant quarantined construction law already requires runner-created throwaway roots,
  production-inexpressible fixture types, confinement away from default persistence, and public
  artifact exclusion. The implementation does not merely harden that law incompletely; it gives the
  fixture row a representation that can denote the forbidden production state while claiming the
  type makes it unrepresentable.

## Threat-model judgment

- ~SUSPECT This silently invalidates the current fixture/production threat boundary expressed by
  `sinv-production-fences`. The prior human disposition accepted an unrestricted developer shell but
  retained structural separation between fixture mechanisms and production roots/artifacts. Whether
  arbitrary shell authority also licenses production-indistinguishable durable minting through the
  harness is a product-level boundary choice not made by that disposition.

## Why this is not CONSTRAIN

- +SURE A complete repair cannot be truthfully reduced to one local constructor substitution. It
  must settle how runner ownership is carried across a persistent shell and multiple child
  processes, how every durable path override is confined, whether direct harness invocation remains
  supported, and what production/release artifact selection guarantees. A pathname, marker file,
  environment token, parent directory convention, warning, or package flag is not by itself the
  required capability.
- ~SUSPECT The accepted unrestricted-shell boundary makes the policy inseparable from the
  construction: the human must decide whether the harness itself is barred from production durable
  authority even though the surrounding developer shell is intentionally uncontained. Issuing a
  mechanical directive would silently make that adjudication or leave a bypass.

## Non-qualifying observations excluded from the verdict

- +SURE The predictable temporary-directory cleanup, session sentinel grammar, moved shell lookup,
  and individual parser validation gaps were considered only as context. Standing alone they are
  localized, rediscoverable, and repairable at a few choke points, so they do not affect this
  accrual verdict.
- +SURE The human-adjudicated decision that ordinary loom sessions are powerful unrestricted
  developer tooling was not re-litigated and is not an independent finding.
- ~SUSPECT No other changed authority mint, durable projection, host-evidence route, or aid-plane
  rendering change independently meets both qualification halves.

## Hidden invariant inventory maintenance

- +SURE `sinv-production-fences` was updated in place, not duplicated. The addition makes explicit
  that a session/env/argv path is not a runner-ownership witness, that all harness durable overrides
  remain confined to the owned capability, and that fixture seams may never reopen a production
  keyset. Review identity `30-reviewC/opaque-accrual` was appended.

## Confidence

- +SURE High on the representation and routing facts: they are direct properties of the parser,
  public types, composition root, receipt opener, CLI override, and Cargo target metadata.
- ~SUSPECT Moderate-high on accrual qualification. The irreversible loss of fixture provenance in a
  production-signed durable and the explicit mismatch with the retained threat boundary distinguish
  this from an ordinary packaging or path-validation defect.

## Final outcome

NACK
