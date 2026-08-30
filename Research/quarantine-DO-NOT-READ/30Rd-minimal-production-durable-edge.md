# 30Rd - Minimal production durable edge

> Tier: quarantined, implementation-facing security and reliability design.
> This document is the single source of truth for the V1 local key provider,
> receipt store, product assembly, and their testing discipline. `30Ra` owns the
> enclosing receipt architecture and security claims. `30Rb` owns the rest of the
> current-tree build. Where either discusses this subsystem, it points here rather
> than restating it.
>
> Authority: root human documents and later human rulings outrank this document.
> **[ACKED]** marks direct human direction. **REQUIRED**, **TYPE LEAN**,
> **IMPLEMENTOR CHOICE**, **LATER**, and **STOP** have the meanings assigned by
> `30Rb:how-to-read-requirements`.
>
> Human direction, 2026-08-25: a minimal production baseline is required for the
> spike. Version every persistent namespace and format; V1 spellings in filenames
> and directories are authorized. The arc must not remove the old whylog and leave
> Dorc unable to persist and read its replacement.

## 30Rd:result-in-one-screen

**[ACKED]** V1 ends with a usable durable path, not a cryptographic demonstration.
The shipped binary can:

1. resolve the role-typed per-user configuration root and either the standard
   per-user receipt store or the exact admin-selected `--receipts <folder>` store;
2. initialize exactly one local V1 keyset containing independently generated
   Ed25519 signing and Age X25519 encryption identities;
3. reopen and validate that keyset without replacing, repairing, or regenerating
   it silently;
4. publish signed plain and signed rich immutable receipts into one versioned
   local store;
5. restart, enumerate that store under bounds, verify/decrypt receipts with the
   local keyset, and answer `dorc why`/`dorc why --receipt-last`; and
6. gate the first potentially mutative dispatch on the required rich
   `ApplyIntent`, while retaining the ruled post-dispatch durable failure direction.

The baseline uses private key files under the per-user configuration root. That is
the researched default, not temporary weak crypto. Platform keychains, TPMs,
hardware tokens, organization key services, import/export, rotation, and custom
provider selection are later providers. The baseline's claims are deliberately
narrow: it protects receipt copies that travel without the configuration keyset;
it does not protect against the authorized account, controller compromise, or a
copy that contains both receipts and private keys.

The implementation is one vertical product route. Fixture signers, in-memory
sinks, and throwaway adapters remain useful instruments, but none can be selected
by the production binary through an environment variable, TTY state, receipt
contents, or fallback.

## 30Rd:why-this-is-v1

The old whylog is deleted by `30Rb`. Without both a concrete key provider and a
concrete receipt store, the replacement can neither create a valid document nor
retain one. An injected trait with fixture implementations is a test seam, not a
product capability.

The local-file baseline is the smallest provider supported by the research:

- SOPS and Age use a per-user configuration key file as an ordinary cross-platform
  baseline [A-sops-age-identity-discovery-2026].
- Platform credential stores fail routinely in headless, SSH, service-account,
  locked-session, and CI contexts [A-gcm-credential-stores-2026].
- Platform stores generally do not defend against arbitrary code running as the
  same user [B-gnome-keyring-security-faq-2024]
  [A-lawrence-chromium-local-data-encryption-2020].
- NIST separates authority to protect new data from authority to read old data and
  retains decryption keys while protected data still matters
  [A-nist-sp800-57-key-management-2020].

V1 therefore builds one honest, available baseline completely. It does not build a
provider framework with no usable provider, and it does not claim the baseline is a
complete key-management product.

## 30Rd:scope-and-non-goals

### Required in V1

- one local V1 keyset and one local V1 receipt store, at the standard location or
  an exact admin-selected `--receipts <folder>`;
- automatic non-interactive first-use key generation on a write path;
- read-only opening that never initializes or mutates key state;
- separate signing and encryption key material, IDs, adapters, and file formats;
- strict versioned keyset metadata and versioned persistent names;
- exclusive restrictive creation at each platform's honest baseline;
- open-handle validation before private key bytes are interpreted where the
  platform exposes the needed metadata;
- bounded receipt creation, publication, enumeration, and reading;
- typed publication grades and failure reasons;
- no silent regeneration, fallback, key import, or provider dispatch;
- a production binary route using the concrete provider/store;
- deterministic model/fault tests and native filesystem integration tests; and
- old-whylog removal only after the replacement survives a process restart.

### Explicitly later

- platform keychains, DPAPI wrapping, Secret Service, TPM, Secure Enclave, hardware
  tokens, KMS, plugins, commands that produce keys, SSH-key reuse, or passphrases;
- user-selected provider backends or provider paths;
- key rotation, revocation, era migration, re-encryption, trust import/export, or
  organization-managed verification keys;
- automatic recovery, cleanup, or repair of an incomplete or damaged keyset;
- automatic receipt retention or deletion;
- custom key roots or key/store provider selection;
- source archive, key escrow, secure deletion, cryptographic erase, or backup
  automation;
- multi-process locking beyond exclusive creation and explicit conflict states;
- a user-facing convenience/hardening profile; and
- stronger Windows ACL inspection until a maintained safe implementation is
  selected and reviewed.

No LATER item receives an empty field, trait method, compatibility reader, dummy
provider, or speculative command in V1.

## 30Rd:security-and-reliability-claims

### What the baseline claims

- A valid receipt was signed by the private signing key corresponding to the
  controller-configured verification identity.
- Rich opaque material can be opened only with the corresponding Age identity.
- Signing and encryption keys are different generated values used through
  different APIs.
- A keyset recognized as ready was completely initialized and its manifest agrees
  with both private keys.
- A required publication witness was minted only after the store reported the
  platform-specific required grade.
- Existing files are never replaced by key initialization or receipt publication.
- Receipt read-back remains report-only under `30Ra`/`30Rb`.

### What the baseline does not claim

- A signature does not make host-influenced or authored content true.
- A private file does not protect keys from the same user, controller root, memory
  inspection, paging, crash dumps, or a compromised Dorc process.
- Separate configuration/state paths do not protect a whole-home or whole-machine
  backup containing both.
- Unix mode bits and Windows inherited ACLs are not equivalent guarantees.
- Synchronization does not promise survival against every filesystem, device,
  hypervisor, network volume, sync client, or power-loss behavior.
- Zeroization does not prove that no copy remains in allocators, kernels, filesystems,
  backups, or hardware.
- Removing a file does not securely erase it.
- An incomplete keyset is not automatically recoverable, and a missing initialized
  key is not automatically replaceable.

These limitations must be carried into later documentation and diagnostics. No
surface may call either projection `secret-free`, `scrubbed`, or `safe to share`.

## 30Rd:component-and-dependency-boundaries

### Required dependency direction

```text
dorc-receipt
   pure models, grammar, reader/writer states, capability traits

dorc-receipt-crypto -> dorc-receipt, age, ed25519-dalek
   concrete crypto adapters; no filesystem, environment, root resolution, or store

dorc-receipt-local -> dorc-receipt, dorc-receipt-crypto
   local keyset, roots, filesystem publication, enumeration, and fault seams

dorc-cli -> dorc-receipt, dorc-receipt-crypto, dorc-receipt-local
   production assembly and policy; the only environment/clock root-selection edge
```

**REQUIRED:** `dorc-plan`, `dorc-core`, `dorc-analysis`, and `dorc-aid` do not
depend on `dorc-receipt-crypto` or `dorc-receipt-local`. Filesystem and key
availability cannot enter analyzer decisions.

**REQUIRED:** `dorc-receipt-local` does not parse receipts semantically, mint
trusted receipt states, render diagnostics, inspect host bytes, or select a crypto
algorithm. It stores exact already-signed bytes and supplies already-validated key
capabilities.

**REQUIRED:** `dorc-receipt-crypto` continues to return primitive results only.
Trust, completeness, projection, and publication witnesses remain minted by
`dorc-receipt`/the orchestrator at their existing seats.

**TYPE LEAN:** add `spike/crates/receipt-local/` rather than folding filesystem and
key lifecycle into `receipt-crypto` or duplicating the store inside `cli`.
Suggested modules:

```text
receipt-local/src/
  lib.rs
  roots.rs          standard controller root resolution inputs and validated roots
  io.rs             narrow filesystem operations and fault-injection vocabulary
  keyset.rs         V1 initialization/open state machine
  key_document.rs   canonical key-document handling through crypto adapters
  store.rs          immutable V1 receipt publication/read/enumeration
  names.rs          typed persistent names and exact parsers
```

The modules may be combined where scanning improves. Keyset and receipt-store state
machines must not be combined: they have different roots, lifecycles, and failure
consequences.

## 30Rd:versioned-persistent-names

**[ACKED]** Every V1 persistent namespace visibly carries `v1`, even where the
contained format also carries `/1`. The redundancy is intentional: an operator can
identify an era before opening a file, and a future implementation never needs to
reinterpret an old unversioned name.

### Key paths

Under the resolved Dorc configuration root:

```text
receipt-keys-v1/
  keyset-v1/
    signing-private-v1.pk8
    encryption-private-v1.age
    keyset-manifest-v1.txt
```

### Receipt paths

Under the resolved Dorc state root by default:

```text
receipts-v1/
  plan-v1-<order>-<receipt-id>.dorc-receipt
  apply-intent-v1-<order>-<receipt-id>.dorc-receipt
  apply-outcome-v1-<order>-<receipt-id>.dorc-receipt
```

With `--receipts <folder>`, `<folder>` is the exact store directory: Dorc appends no
second `receipts-v1` component. The administrator-chosen directory name is not a Dorc
namespace; every Dorc-owned filename and contained format still carries V1.

`<order>` is exactly 20 decimal digits encoding the controller-observed Unix
millisecond. The same `ReceiptOrderToken` is inside the signed V1 receipt header and
must equal the filename token. `<receipt-id>` is exactly 64 lowercase hexadecimal
characters. The filename alphabet is ASCII, lowercase, and case-fold/normalization
invariant [A-basu-name-collisions-case-2023].

The ordering component is authenticated store metadata, not authority, world
freshness, attribution, or part of the receipt ID. `dorc why --receipt-last` uses it
to derive the newest root only after filename/header agreement. Clock rollback can
change local selection order and must not change graph edges or semantic narration.
Multiple receipts at the maximum order form a cohort: graph predecessors collapse
beneath a terminal member, while several incomparable terminal members report
ambiguity rather than receiving a random receipt-ID tie-break.

No hostname, username, target, source path, policy name, argv, or opaque-derived
value enters a V1 filename. Directory listings reveal receipt count, species, and
local ordering; that leakage is accepted and documented.

Unknown names and sync-client conflict names are never parsed as receipt identities.
They count against the enumeration budget and produce bounded store findings; they
are never deleted automatically.

## 30Rd:controller-root-resolution

Root resolution happens once at the CLI edge and yields absolute controller-owned
values. No receipt, host byte, source text, key file, or environment value from a
managed host may influence it.

### V1 standard roots

| Platform | Configuration root | State root |
|---|---|---|
| Windows | non-empty `%APPDATA%\dorc` | non-empty `%LOCALAPPDATA%\dorc` |
| macOS | `$HOME/Library/Application Support/dorc` | `$HOME/Library/Application Support/dorc` |
| other Unix | non-empty `$XDG_CONFIG_HOME/dorc`, else `$HOME/.config/dorc` | non-empty `$XDG_STATE_HOME/dorc`, else `$HOME/.local/state/dorc` |

An absent/non-absolute required base is `ControllerRootUnavailable`. There is no
fallback to cwd, a repository, a temp directory, cache directory, runtime directory,
or one root standing in for the other. On macOS the two role-typed roots intentionally
resolve to the same OS application-support path; the versioned key/store
subdirectories preserve role but do not claim path or backup separation.

Standard controller environment variables are read only by the process edge. Tests
may set those standard variables to sandbox paths; there is no Dorc-specific
environment variable that selects a fixture provider, test key, volatile sink, or
weaker policy.

### Root redirection and validation

V1 has no custom key-root flag. `--receipts <folder>` is the one explicit store-root
surface: it is controller argv, resolved once to an absolute path at the CLI edge;
it never changes the standard configuration/key root. A standard OS root or the
explicit store landing may traverse a user-managed symlink, junction, mount, or
roaming profile. The implementation:

1. resolves the selected product/store root once;
2. rejects a non-directory landing;
3. on Unix, requires the landing to be owned by the effective user and not writable
   by group/other;
4. on Windows, accepts the per-user profile landing under its inherited ACL and
   states that it has not independently reconstructed the DACL policy;
5. opens/retains an ownership-bearing directory handle where the platform/library
   exposes one; and
6. permits only fixed, typed, single-component internal names beneath the landing.

Internal keyset directories and key files may not be links/reparse redirects. A
pre-existing internal link is a conflict, not something to follow. Exclusive create
must be used for every internal object.

On Unix/macOS, authority-bearing child opens/creates/removals are relative to the
retained validated directory handle with non-follow semantics. On Windows, V1 uses
the strongest maintained safe handle-relative operation available and revalidates
opened objects; where safe Rust falls back to a standard-root pathname, that race is
part of the explicitly weaker accepted Windows baseline rather than an implied
capability guarantee. No caller-side pre-check is represented as closing it.

The Windows baseline is intentionally weaker. It relies on ordinary per-user profile
isolation and exclusive creation, exactly as the old store already did. It must not
be rendered as equivalent to Unix owner/mode verification. Selecting a maintained
safe ACL implementation is LATER and requires a new review; a stale wrapper or local
unsafe FFI must not be introduced casually.

### Clean-profile root bootstrap

Root opening has separate read-only and create-capable paths. The read-only path used
by `why` never creates a missing directory. The create-capable path:

1. identifies the nearest existing ancestor of the fixed standard root;
2. validates that ancestor under the platform posture above;
3. creates each missing fixed component one at a time, restrictively and exclusively
   where the platform permits, never with an uninspectable recursive convenience call;
4. after a creation race, opens and validates the winner rather than assuming it is
   equivalent to the requested directory;
5. rejects a link/reparse landing at every Dorc-owned component;
6. synchronizes each newly created directory and the parent whose entry made it
   reachable where the platform provides directory synchronization; and
7. returns the validated product-root handle/value used by all later child operations.

This protocol owns creation of the Dorc product roots, `receipt-keys-v1`, and the
standard `receipts-v1` or exact explicit store directory. `keyset-v1` retains its
stricter one-winner initialization semantics.
On Windows, directory synchronization is recorded unavailable rather than simulated.

On Unix, every Dorc-owned configuration/state/key/store directory is created with
mode `0700`, and every private key, manifest, and receipt file with mode `0600`, in
the same create operation that makes the object visible; the process umask may narrow
but never widen them. A later chmod is not the creation mechanism. Existing key
directories/files are refused if any group/other permission bit is present. Existing
receipt-store roots are refused if group/other writable; receipt file readability is
still treated as sensitive and newly created files remain `0600`. On Windows, all
these objects inherit the validated per-user profile landing's ACL under the explicitly
weaker baseline.

## 30Rd:key-roles-and-library-settings

### Signing role

- Ed25519 signs the exact DSSE PAE bytes already defined by `30Ra`/`30Rb`.
- `ed25519-dalek` uses strict verification.
- Features are exactly the minimum needed for `std`, `pkcs8`, and `zeroize`; do not
  enable serde, batch, digest/prehash, hazmat, legacy compatibility, PEM, or key-file
  convenience writers.
- The private document is canonical unencrypted PKCS#8 DER emitted and parsed by
  the library. `dorc-receipt-local` never invents an Ed25519 private-key encoding.
- Loading parses DER, reserializes through the same library, and requires byte
  equality with the bounded input before accepting it as canonical V1 material.

The current `ed25519-dalek` edge disables default features and enables only `std`,
which also disables its optional `ZeroizeOnDrop` implementation. V1 MUST enable the
`zeroize` feature. This is defense in depth, not a memory-erasure claim.

### Encryption role

- Age v1/X25519 remains the sole rich-overlay recipient.
- The private document is exactly one canonical `age::x25519::Identity::to_string()`
  result followed by one LF.
- Loading accepts exactly one line, no comments, blank lines, leading/trailing
  whitespace, CRLF, additional identities, SSH identities, commands, or plugins.
- It parses through Age, serializes back through Age, and requires exact byte
  equality before accepting the identity.
- The public recipient and `EncryptionKeyId` derive from the parsed private identity;
  no caller supplies either.

Age already uses `SecretString` and zeroizes serialization intermediates. The local
provider must not convert the private identity to an ordinary `String` except inside
the smallest write callback required by the filesystem edge. Its provider wrapper is
non-`Clone` and redacts `Debug`, even though the upstream identity type is cloneable.

### Separation requirements

- Generate signing and encryption keys independently. Never derive either from the
  other or from a shared stored root.
- Never convert Ed25519 material to X25519 or vice versa; the upstream API explicitly
  recommends separate keys for signing and encryption.
- Keep `SigningKeyId` and `EncryptionKeyId` as non-convertible newtypes.
- The crypto crate, not the local store, owns parsing/generation/serialization of
  private key documents.
- Private key documents are non-`Clone`, non-`Debug`, non-serde, and expose no raw
  accessor. A narrow consuming/write callback is allowed where the filesystem edge
  must persist canonical bytes.

## 30Rd:keyset-manifest-format

`keyset-manifest-v1.txt` is public structural metadata stored privately for one
permission policy. Its exact bytes are:

```text
dorc-receipt-keyset/1
signing-key-id <64-lower-hex>
encryption-key-id <64-lower-hex>
keyset-end
```

Every line ends LF, including `keyset-end`. EOF follows immediately. The file has no
comments, blank lines, alternate order, algorithm names, filenames, provider names,
paths, timestamps, optional fields, or ignored bytes. Algorithms, encodings, and file
names are fixed by keyset version 1. The parser accepts only the writer's form and is
bounded to 256 bytes before parsing.

The manifest does not select a backend or grant trust. Its role is completion plus
cross-file agreement. Loading independently derives both IDs from the private key
documents and compares them to the manifest.

The key-ID derivations are part of V1 and never caller-selected:

```text
SigningKeyId = SHA-256(DSSE-PAE(
  "application/vnd.dorc.receipt.v1.signing-key-id",
  exact 32-byte Ed25519 public verification key))

EncryptionKeyId = SHA-256(DSSE-PAE(
  "application/vnd.dorc.receipt.v1.encryption-key-id",
  canonical lowercase Age X25519 recipient text))
```

The exact domains, public-material encodings, and manifest examples receive committed
vectors. No generic digest constructor or `[u8; 32]` conversion crosses these domains.

## 30Rd:v1-local-edge-limits

These are injected V1 policy values, not forever protocol promises:

| Limit | V1 value |
|---|---:|
| keyset manifest bytes | 256 |
| signing private document bytes | 256 |
| Age private identity document bytes | 256 |
| one persistent filename bytes | 192 |
| entries in one store enumeration | 4,096 |
| aggregate receipt bytes admitted into one graph build | 256 MiB |

One receipt remains bounded by `30Rb:version-one-limit-policy` (64 MiB outer).
Enumeration always walks to entry limit + 1 before classifying names. Graph reading
checks the aggregate budget before retaining each next document and does not allocate
from a filename, record count, or file-declared length. Widening any local-edge limit
requires boundary-minus/at/plus tests and allocation review; lowering is permitted if
it does not invalidate a required product route.

## 30Rd:keyset-initialization-state-machine

### Read and write entry points stay separate

```text
open_for_read(roots)
  -> LocalReadKeysV1 { verification, decryption } | ReadUnavailable

open_or_initialize_for_write(roots, store_presence, generator)
  -> ReadyForPublication | InitializationRefusal
```

`open_for_read` is mutation-free. `dorc why` calls only it. Missing keys remain a
report state; asking why must never create a new identity that cannot open the receipt
being examined.

`open_or_initialize_for_write` may initialize only when the entire `keyset-v1`
directory is absent AND a bounded read-only check establishes that `receipts-v1` is
absent or empty. Any recognized receipt, partial receipt, unknown entry, inaccessible
entry, enumeration failure, or overflow produces `KeysetMissingWithExistingStore` and
forbids generation. This prevents whole-keyset loss from becoming an unannounced new
key era while old receipts remain. It never initializes a missing member of an
existing keyset.

### Generation precedes durable mutation

The production key generator obtains both independent secret values in memory before
creating `keyset-v1`. If entropy/key generation fails, no keyset path is created.
Age's own generator is an accepted nondeterministic edge; Ed25519 secret bytes come
from the production CSPRNG edge. Tests replace the complete generator capability,
never a nonce or internal Age primitive.

### Exclusive initialization sequence

1. Read-only inspect the V1 store under a complete `limit + 1` bound; require absent
   or empty.
2. Resolve/create and validate the configuration root through the root-bootstrap
   protocol.
3. Generate both key documents in memory.
4. Exclusively create `receipt-keys-v1` if absent; validate it after any race.
5. Exclusively create `keyset-v1` with restrictive directory creation. This is the
   first-use concurrency arbitration point.
6. Exclusively create, write, and synchronize `signing-private-v1.pk8`.
7. Exclusively create, write, and synchronize `encryption-private-v1.age`.
8. Derive IDs from the in-memory/round-tripped keys and construct the exact manifest.
9. Exclusively create, write, and synchronize `keyset-manifest-v1.txt` LAST.
10. Synchronize `keyset-v1` and every newly created containing directory where the
   platform exposes a meaningful
   operation.
11. Reopen through the ordinary write-validation path before returning
    `ReadyForPublication`.

No receipt may be signed or sealed before step 11 succeeds.

### Failure and concurrency

- A process that loses the `keyset-v1` directory creation race discards its generated
  keys. It may load the winner only if the winner's manifest is already complete and
  validates for writing; otherwise it returns `IncompleteOrInProgress`, without
  waiting, deleting, adding, or regenerating.
- Any failure after `keyset-v1` creation leaves an incomplete keyset that cannot be
  used. V1 does not automatically remove or repair it.
- An existing `keyset-v1` without a valid final manifest is `IncompleteOrInProgress`.
  It is never treated as first use, even if one or both key files appear valid.
- An existing manifest with a missing, malformed, redirected, permissive, mismatched,
  or unreadable key is a typed unavailable/damaged state. It never triggers generation.
- Synchronization failure fails this attempt. Do not retry synchronization and do not
  trust same-attempt page-cache readback [A-rebello-fsync-failures-2020]. A later
  read-only process may inspect what the filesystem presents. A later write process
  must validate and successfully synchronize the existing key files, manifest, and
  required directory ancestry before exposing signing/sealing capabilities as
  `ReadyForPublication`.
- Cleanup may remove only objects created and still owned by this attempt. Cleanup
  failure is reported; it never broadens into pathname deletion. Leaving an incomplete
  keyset is safer than removing an object whose identity is uncertain.

V1 recovery from an incomplete directory is explicit operator action after inspecting
the typed state. No automatic repair command is built. Because no manifest means no
keyset was ever licensed for receipt publication, removing an incomplete V1 directory
can be documented as reinitialization rather than key rotation. V1 MUST ship a typed
diagnostic/defining case and a concurrency-safe manual procedure requiring all Dorc
processes to be stopped and the exact incomplete versioned directory to be moved aside
for inspection before retry. Builders supply no prose; the conductor/human authors it
through the loom. The tool performs no removal on the user's behalf.

## 30Rd:keyset-open-and-validation

The normal open path validates before exposing any capability:

1. Open the resolved keyset directory and reject a link/reparse/non-directory.
2. Open the manifest and both key documents without following a final-component
   redirect.
3. Require regular files, explicit small size bounds, and no unexpected additional
   hard-link assumption. File identity checks are used where safely reachable, but
   V1 makes no universal hard-link uniqueness claim.
4. On Unix, inspect the opened handles before reading: effective-user ownership and
   no group/other permissions on the private files; keyset directories have no
   group/other write/read/search bits. The OpenSSH handle-before-read pattern is the
   precedent [A-openssh-authfile-perm-ok-2026].
5. On Windows, require the standard per-user root, reject reparse private members,
   and rely on inherited profile ACLs under the stated weaker guarantee.
6. Read each file through its independent byte bound.
7. Strictly parse and canonical-round-trip the manifest, signing document, and Age
   identity.
8. Derive both public materials and IDs and compare them to the manifest.
9. Construct role-specific read availability. A valid signing document + matching
   manifest ID exposes local verification material even when the Age identity is missing
   or damaged; decryption remains unavailable. Only the CLI read edge that owns this
   validated local keyset may wrap a checked receipt as locally authenticated. Neither
   role-specific read state can sign, seal, publish, initialize, or mint dispatch.
10. On a write open, require both roles and successfully synchronize the validated
    key documents/manifest/required directory ancestry before constructing
    `ReadyForPublication`.

### Key availability states

Keep at least these distinct:

```text
NotInitialized
KeysetMissingWithExistingStore
IncompleteOrInProgress
RootUnavailable
TemporarilyUnavailable
MissingAfterInitialization { role }
MalformedKeyDocument { role }
NonCanonicalKeyDocument { role }
PermissionRefused { role_or_directory }
ManifestMismatch { role }
UnsupportedKeysetVersion
VerificationReady
RichReadReady
ReadyForPublication
```

Do not collapse these into `Option`, generic I/O failure, or `PermanentlyLost`.
Research shows that falsely classifying a key as permanently unavailable encourages
callers to discard still-recoverable encrypted data. Nothing in these states deletes
receipts or keys.

## 30Rd:loaded-keyset-capabilities

**TYPE LEAN:**

```rust
pub struct LocalReadKeysV1 {
    verifier: Ed25519Verifier,
    opener: Option<AgeOpener>,
    status: LocalReadKeyStatusV1,
}

pub struct LocalWriteKeysV1 {
    signer: Ed25519Signer,
    verifier: Ed25519Verifier,
    sealer: AgeSealer,
    opener: AgeOpener,
    readiness: KeysetSynchronizedForPublicationV1,
}
```

The exact ownership may use internal shared references to avoid duplicating secret
material. Required effects:

- no public fields or raw secret access;
- no `Clone`, serde, `Default`, equality, ordering, or hash implementation;
- redacted `Debug` naming only the type and public key IDs if needed;
- read-only opening can return local verification material without an opener; write
  readiness remains all-or-nothing and requires both roles plus synchronization;
- receipt core treats supplied verification material as self-asserted; local-policy
  authentication is the private CLI envelope minted only after this keyset validates;
- an unknown receipt signing ID does not become locally authenticated and does not cause provider
  discovery;
- key ID lookup never selects a plugin, process, command, path, network call, or
  alternate algorithm; and
- dropping the object invokes the upstream zeroization support where available.

The current `VerificationKeyResolver` may expose only the loaded local verification
key in V1. A receipt naming another ID becomes `UnknownSigningKey`; V1 does not scan
directories, import embedded public material, or try arbitrary providers.

## 30Rd:local-receipt-store

### Store contract

`LocalReceiptStoreV1` owns the validated `receipts-v1` root and the only production
implementations of receipt publication/source capabilities. It:

- mints names from typed species, version, controller order token, and receipt ID;
- accepts complete signed immutable bytes only;
- creates under the final name with no replacement;
- writes under the outer receipt bound;
- synchronizes according to the platform baseline;
- never appends, truncates an existing file, or writes a mutable latest pointer;
- enumerates every directory entry to `limit + 1` before filtering;
- reads recognized files under an independent byte bound;
- never follows a receipt entry link/reparse redirect;
- never deletes, retains, prunes, repairs, or renames existing entries in V1; and
- returns typed failures rather than `Option` or ignored I/O.

The current capability traits are too weak for this edge: `publish(name: &str, ...)`
lets callers supply path-shaped strings, `names() -> Vec<String>` is unbounded and
infallible, and `read(name)` loses entry ownership. Replace or wrap them so the local
store, not a generic caller, mints names and bounded entry handles.

**TYPE LEAN:**

```rust
trait ReceiptPublisher {
    fn publish_required_v1<D, P>(
        &mut self,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<D, P>,
        policy: LocalRequiredReceiptPolicyV1,
    ) -> Result<RequiredLocalPublicationV1<D, P>, PublishFailure>;
}

trait ReceiptCollection {
    fn enumerate(
        &self,
        limits: StoreReadLimits,
    ) -> Result<BoundedReceiptEntries, EnumerateFailure>;

    fn read(
        &self,
        entry: &OwnedReceiptEntry,
        limits: &ReceiptLimits,
    ) -> Result<BoundedReceiptBytes, StoreReadFailure>;
}
```

Exact trait placement is implementor choice. A string name is never sufficient to
read, remove, or claim ownership of an entry.

`RequiredLocalPublicationV1<D, P>` has private fields and is bound to the exact
receipt ID, species/projection, signed-body digest, order token, policy identity, and
platform property set checked by this publication. The fixed V1 production method
does not accept an arbitrary weaker grade. Fixture/volatile publishers return a
disjoint proof type that cannot satisfy the production permit mint.

For `ApplyIntent`, the exact-image witness and required publication remain inside one
private `PublishedApplyIntentV1` value. The permit mint consumes that value atomically;
a caller cannot pair publication of one intent with another intent's image witness or
policy witness.

### Publication protocol

1. Receive immutable complete signed bytes plus typed species/ID.
2. Validate aggregate size before opening the filesystem.
3. Mint the exact V1 filename.
4. Exclusively create the final file with restrictive creation.
5. Write all bytes. Do not append or retry a partial write into another name.
6. Synchronize the file.
7. Synchronize the containing directory where meaningful and reachable.
8. Mint the publication proof only after every operation required by active policy
   succeeds.

Direct final-name creation intentionally permits a concurrent reader or sync client to
observe an incomplete file while it is being written. Such a file lacks a complete
signature trailer/EOF and can never parse as complete. This is preferable in V1 to a
cross-platform temp/rename protocol whose no-replace and directory-sync guarantees are
not uniformly reachable. A crash may leave a partial file; the reader reports it as
`IncompletePublication { state: InProgressOrAbandoned }` and no later writer replaces
it. Mere presence cannot prove which side of that distinction occurred.

If write or synchronization fails, publication fails. Do not retry `fsync`; a retry may
report success after dirty pages were discarded. Available production removal APIs act
by name rather than conditionally on object identity, so cleanup returns unavailable on
both platforms and leaves the incomplete file as bounded partial evidence. The
deterministic model may exercise successful cleanup because its node identity genuinely
conditions the operation; production does not claim that outcome.

### Publication grades are not one ordered enum

Windows cannot provide the same directory synchronization operation as Unix/macOS.
Do not derive `Ord` and pretend every platform grade lies on one universal ladder.
The proof records independent properties, at minimum:

```text
exclusive-final-name-created
complete-bytes-written
file-synchronized
directory-synchronized | directory-sync-unavailable-on-platform
```

The V1 required local baseline is:

- Unix/macOS local filesystems: exclusive creation, complete write, file sync, and
  containing-directory sync;
- Windows: exclusive creation, complete write, and `FlushFileBuffers`/`sync_all` on
  the file, with directory sync explicitly unavailable;
- filesystems that reject the required file synchronization: publication refusal;
  no silent demotion; and
- volatile/in-memory/throwaway sinks: fixture grade only, never sufficient for a
  production required `ApplyIntent`.

The policy check consumes a typed proof of these properties, not a numeric comparison.

## 30Rd:store-enumeration-and-last-selection

Enumeration is hostile local input even though same-user compromise is outside the
confidentiality claim:

- collect at most `entry_limit + 1`; overflow is explicit;
- count unknown names, directories, links, conflict files, and inaccessible entries
  against the walk budget;
- never assume enumeration is a stable snapshot; POSIX explicitly leaves concurrent
  additions/removals unspecified [A-posix-readdir-spec-issue8-2024];
- group recognized names by parsed order only after the bounded walk; retain every
  member of a maximum-order cohort and sort that cohort deterministically for display;
- reject malformed recognized-prefix names rather than normalizing them;
- open a selected entry under the stored root and verify its type before reading;
- re-bound the entire file independently of writer policy; and
- compare internal version/species/order/receipt ID against the filename after
  signature verification/parsing, retaining mismatch as a report finding.

`dorc why --receipt-last` derives a root from the greatest recognized filename order.
After bounded verification and parsing, maximum-order members that are typed graph
predecessors of another cohort member collapse beneath that member. One remaining
terminal root is selected; several incomparable terminal roots report ambiguity rather
than receiving a random-ID or species tie-break. A partial, damaged, unknown-key, or
otherwise unreadable maximum-order member remains a report state and never triggers
fallback to an older complete root. An incomplete direct-final member is
`InProgressOrAbandoned`, not proof of failure.

`--receipt-id <id>` retrieves one exact root from the selected/default store.
`--receipt <file>` admits one explicit report-only root file. `--receipts <folder>`
selects the store used for lookup or publication and may resolve typed siblings for an
explicit file. The three root selectors are mutually exclusive. With no selector,
receipt-reading `why` uses `--receipt-last` semantics.

The engine automatically follows only typed edges needed by the rooted explanation.
It may enumerate the bounded store to discover reverse edges, but disconnected receipt
DAGs never join the answer. Missing/unknown/partial required siblings remain graph
findings. `--all` changes explanation depth only and never selects store entries.

## 30Rd:product-assembly-and-policy

### One production composition root

The binary has one function that assembles the production durable edge from values:

```text
ControllerRoots
  + LocalReceiptIo
  + ReceiptIdEntropy
  + ProductionKeyGenerator
  + ControllerClock
  + ReceiptPolicyWitness
  -> LocalReceiptEdgeV1
```

No alternate production assembly exists. The CLI does not select fixture capabilities,
fixed IDs, in-memory stores, or static keys from environment, receipt bytes, command
shape, or TTY presence.

Library/e2e drivers receive explicit roots as values. The real binary resolves the
standard key root and the standard or `--receipts` store root, then calls the same
assembly and publication functions. `--receipt <file>` enters only the report reader
and never this publication assembly.

### Plan and why behavior

- A plan write path may initialize the keyset on first use.
- Default plan receipt projection is signed rich.
- Plan analysis/rendering remains available if publication fails, but the failure is
  visible and the invocation must not claim a persisted whylog.
- `dorc why` is read-only with respect to keys, receipts, and managed hosts: it opens
  existing keys/store, initializes nothing, and performs no receipt cleanup or host
  observation. A separate report-layer source resolver may perform the already-ruled
  bounded controller-source reads for digest matching; that resolver is not part of
  `LocalReceiptEdgeV1` and cannot affect receipt trust or action.
- Missing encryption material may still permit bounded authenticated skeleton reading
  if the signing verification key is available; missing signing material keeps the
  receipt in its distinct unverified/unknown-key state.

The exact plan process exit behavior and user-facing prose remain catalog/human work.
The data model must preserve the distinction regardless of prose.

### Apply behavior

- Required rich `ApplyIntent` publication uses the concrete local keyset and local
  store.
- `MutationDispatchPermit` is minted only by moving the exact prepared intent through
  image accounting and required local publication; the published value owns that chain.
- Key initialization, key opening, sealing, signing, store creation, write, or required
  synchronization failure before permit consumption refuses mutation.
- A plain receipt never satisfies the required arm.
- V1 has no configured or fixture bypass. Tests exercise private transitions or earn
  required publication through the deterministic model/native throwaway store.
- The default production apply route must acquire the genuine standup identity required
  by `30Rb` and required publication or refuse before mutation.
- Once the permit is consumed, durable-only failure to publish `ApplyOutcome` is
  reported and does not abort otherwise coherent execution.
- Transport, execution, target, attribution, generation, and mutation-integrity
  failures remain separately typed and are never caught by a durable fallback.

If post-dispatch rich publication fails while signing remains available, a signed plain
terminal receipt may be attempted only where existing policy explicitly permits it. Its
failure is another durable finding, not an execution-integrity failure. No opaque bytes
fall through to stdout/stderr or clear storage.

A persisted `ApplyIntent` records prepared intent, requested policy, and
pre-publication prerequisites, not achieved publication grade, dispatch eligibility,
permit consumption, or dispatch history. The achieved local publication proof is an
ephemeral input to the permit mint and is not reconstructed from a discovered file.
Absent a correlated outcome, the graph reports publication eligibility and dispatch as
unknown; it never rounds intent presence up to authority spent.

## 30Rd:key-backup-recovery-and-rotation-posture

V1's private key files are ordinary backupable files. The product must eventually state:

- rich receipt detail is unrecoverable without the matching Age identity;
- old signatures remain verifiable only while matching public verification material is
  available and trusted;
- copying configuration and state together defeats most separation;
- copying only state preserves the intended at-rest separation;
- deleting/replacing a key is not receipt cleanup and not secure erasure; and
- filesystem/sync backups may retain both deleted keys and removed receipts.

V1 adds no export, escrow, backup, or rotation command. Operators can back up the
versioned keyset directory using ordinary file tools; later UX may make that safer and
clearer. No diagnostic instructs a user to delete an initialized keyset as a generic fix.

The receipt format already carries key IDs and the resolver API is keyed, so future eras
need not change receipt v1 merely to identify another key. The local V1 provider exposes
one ready keyset only. Rotation starts a separately reviewed provider/keyset version or
era and retains old read keys while old receipts matter. A replacement key is never
derived from its predecessor, and existing receipts are never rewritten silently.

## 30Rd:testing-is-part-of-the-security-boundary

**[ACKED]** Test/development praxis is a first-class design requirement. The goal is
not merely that crypto functions have unit tests; the real product path must be easy to
exercise without writing to a developer's profile or weakening production selection.

### Four test layers

1. **Pure receipt kernel:** existing fixed vectors, strict grammar, overlay accounting,
   graph, compile-fail, and mutation tests. No filesystem or random key generation.
2. **Crypto adapter interoperability:** real Age and Ed25519 adapters over fixed fixture
   material, standard vectors, strict verification, canonical key-document round trips,
   and real randomized seal/open semantic tests. Ciphertext bytes are not goldened.
3. **Key/store state models:** deterministic injected key generator, clock, and virtual
   I/O with fail-before/fail-after control over every durable operation. This owns crash,
   race, and compound-failure coverage.
4. **Native product routes:** the concrete local provider/store under isolated standard
   config/state roots on Windows and Unix/WSL, driven through the actual binary and a
   fresh second process. Apply/why runs the production transport adapter against the
   central e2e harness's inert `PATH=mocks-only` transport executable; no real mutator
   executes and no runtime selector chooses a fixture receipt provider/store.

No layer substitutes for another. A virtual filesystem cannot establish native ACL,
link, synchronization, or sharing behavior. Native tests cannot economically enumerate
every failure interleaving. Fixed fixture keys cannot prove production key generation is
reachable, and random crypto output cannot serve deterministic protocol goldens.

### Production path under test

The e2e harness sets only the standard root variables for its throwaway sandbox:

- Windows: `APPDATA` and `LOCALAPPDATA`;
- Unix: `XDG_CONFIG_HOME`, `XDG_STATE_HOME`, and an isolated `HOME`; and
- macOS when available: an isolated `HOME` feeding the standard Application Support
  path.

It does not set a Dorc-specific test-provider/key/store variable. The binary resolves
those roots and constructs the same `LocalReceiptEdgeV1` as an ordinary invocation.

At least one acceptance case runs:

```text
process 1: plan -> initialize keyset -> publish rich PlanReceipt
process 2: why --receipt-last -> reopen keyset -> verify -> decrypt -> explain
```

And one apply acceptance runs through the actual binary under inert transport mocks:

```text
process 1: prepare exact image -> publish rich ApplyIntent -> consume permit
           -> SimDriver/hostsim -> publish ApplyOutcome
process 2: why --receipt-last [--all] -> reopen -> correlate the rooted species -> explain
```

The acceptance asserts the concrete receipt files and concrete versioned key files
exist only beneath the sandbox roots. It also asserts no old-format whylog was written.
The second process must render a known opaque sentinel that was available only by
opening the rich overlay; skeleton-only output cannot satisfy the case. Two independent
clean sandboxes must produce different signing IDs and different encryption IDs, while
reopening one sandbox preserves both IDs exactly. The apply trace must place the
required local publication event before the simulated dispatch call.

### Deterministic I/O model

`dorc-receipt-local` owns a narrow internal I/O vocabulary rather than a generic fake
filesystem. Operations should correspond one-for-one to security-relevant acts:

```text
resolve/open validated root
create directory exclusively with requested private policy
create file exclusively with requested private policy
open existing file without final-component follow
read bounded bytes
write all bytes
synchronize file
synchronize directory or report unavailable
enumerate bounded entries
inspect opened object identity/type/owner/mode
remove this-attempt-owned object
```

No `write_file(path, bytes)` convenience may collapse create, ownership, write, and sync
into one un-faultable operation. Conversely, do not model every OS syscall; model the
semantic boundaries the production implementation promises.

The production implementation and deterministic model implement the same private
trait. The trait is not exported from the crate, and production code cannot accept a
test implementation from CLI input.

### Failure sweep

For key initialization and receipt publication, inject failure immediately before and
after every modeled durable operation, then restart from the resulting modeled disk.
Every state must land in exactly one closed outcome.

Required keyset assertions:

- failure before keyset-directory creation leaves no keyset path;
- failure after directory creation but before final manifest yields
  `IncompleteOrInProgress` on restart;
- no incomplete state exposes signer/sealer/opener/verifier capabilities;
- no restart regenerates or overwrites any existing key file;
- absent whole keyset plus any non-empty/unknown/overflowed store state refuses
  generation as `KeysetMissingWithExistingStore`;
- a manifest observed after a failed synchronization may support read-only inspection,
  but no write capability appears until a later write-open re-synchronizes the complete
  validated keyset ancestry;
- manifest is always the last recognized completion act;
- manifest/key ID mismatch refuses;
- permissive/redirected/wrong-owner Unix key files refuse before secret parsing;
- a concurrent loser never mixes its generated key with the winner's keyset; and
- cleanup failure does not delete an unowned path.

Required store assertions:

- create collision never overwrites;
- every prefix/truncation of a receipt remains partial, never complete;
- write/sync failure mints no required publication proof;
- directory-sync-unavailable is distinct from directory-sync-failed;
- a post-dispatch store failure does not become an execution failure;
- unknown/conflict names count against enumeration limits but mint no receipt;
- the newest partial candidate prevents silent fallback to an older complete receipt;
- filename/internal version/species/order/identity disagreement remains a finding;
- equal maximum-order candidates remain an ambiguity cohort;
- concurrent enumeration changes never create an authority edge; and
- a cleanup error cannot erase an existing receipt.

### Compound failures

Following SQLite's long-running fault-injection practice, inject an I/O failure while
handling a prior failure [A-sqlite-how-tested-crash-io-2026]. Minimum combinations:

- key write failure plus cleanup failure;
- manifest synchronization failure plus diagnostic/store failure;
- receipt write failure plus attempted removal failure;
- pre-dispatch publication failure plus transport proving it was never called;
- post-dispatch publication failure plus transport/execution failure, proving the latter
  is not caught as durable-only; and
- concurrent first-use initialization plus one process crash.

### Native filesystem cases

Run on both Windows and Unix/WSL:

- fresh first use and reopen;
- existing valid keyset;
- incomplete keyset;
- pre-existing key filename, directory, link, and platform redirect where constructible;
- receipt filename collision;
- read-only/full/unwritable roots where safely constructible;
- bounded directory overflow;
- concurrent first-use processes;
- concurrent receipt publishers;
- process restart after publication; and
- Unix owner/mode assertions on a real Unix filesystem, never drvfs.

Tests must not claim power-loss durability beyond what their mechanism actually
injects. A child-process crash test is valuable for close/drop behavior; it is not a
power-cut test.

## 30Rd:test-and-fixture-fences

- Fixed private fixture keys live only in a narrow test allow-list already guarded by
  `fixture_payloads_are_unreachable_from_production`-class lexical tests.
- Production crates contain no fixture private key bytes, deterministic receipt IDs,
  static nonces, or constructors selected by environment presence.
- Test-only constructors are `cfg(test)` or live in test-support crates that production
  targets cannot depend on.
- The real local provider is exercised by ordinary workspace tests; it is not hidden
  behind an opt-in real-tools lane.
- A dependency graph gate proves `dorc-receipt-local` is unreachable from analyzer/core
  crates and fixture support is unreachable from production CLI files.
- Compile-fail tests prove private key documents, loaded keysets, publication proofs,
  and required dispatch witnesses cannot be directly constructed, cloned, defaulted,
  deserialized, or converted across roles.
- A lexical two-way allow-list enumerates every production call to keyset initialization,
  key loading, and local receipt publication. A stale entry fails; a new caller is a
  governed review event.
- No golden or snapshot contains a newly generated private key. Tests assert behavior and
  public IDs, not secret bytes.
- Failure messages and Debug output are checked for absence of key bytes, key document
  text, source paths outside permitted diagnostics, and opaque receipt fields.

## 30Rd:dependency-and-secret-memory-discipline

Required dependency changes:

- enable `ed25519-dalek` `zeroize` and `pkcs8` explicitly;
- keep serde, PEM convenience, hazmat, legacy, batch, and prehash off;
- keep Age plugins, SSH recipients, async, CLI helpers, passphrases, and unstable APIs off;
- use the already selected Age and Ed25519 package versions/features unless a separate
  reviewed dependency update is intentionally undertaken; and
- include the exact feature/lockfile diff in review.

Secret-bearing values:

- use upstream secrecy/zeroizing containers for serialized private material;
- minimize and scope `ExposeSecret`/equivalent calls to the exclusive write operation;
- never log, format, compare, hash for diagnostics, or include private bytes in an error;
- derive public IDs from public material, never from private serialization;
- avoid extra copies when reading and parsing; zeroize mutable intermediate buffers;
- hold loaded private keys no longer than the command invocation needs; and
- make no `mlock`, no-swap, crash-dump, or complete-memory-erasure claim in V1.

## 30Rd:implementation-sequence

This sequence joins `30Rb` after its receipt crypto/model APIs are stable and before
old whylog deletion. Do not parallelize the state-machine foundation or production CLI
assembly.

### Stage D0 - contracts, names, and test model

- Add crate-local steering law for every invariant in this document.
- Add the `dorc-receipt-local` crate and dependency fences.
- Define exact V1 path/name types, manifest vectors, key/store failure enums, platform
  publication property types, and private I/O operation vocabulary.
- Commit valid/invalid manifest and filename vectors before parser growth.
- Build the deterministic I/O model and failure schedule skeleton before production I/O.

Exit: types and vectors compile; no production key generation or store is selectable.

### Stage D1 - canonical key documents

- Enable the reviewed dependency features.
- Add crypto-owned Ed25519 PKCS#8 generation/parse/serialize with zeroizing buffers.
- Add crypto-owned canonical Age identity generation/parse/serialize.
- Add public-ID derivation and non-aliasing compile-fail tests.
- Add fixed-vector and real randomized semantic interop tests.

Exit: canonical key documents round-trip and no filesystem exists in the crypto crate.

### Stage D2 - keyset state machine

- Implement root validation and platform-private creation.
- Implement generation-before-create, exclusive `keyset-v1`, ordered key writes,
  manifest-last completion, synchronization, and ordinary reopen validation.
- Implement read-only open separately from write initialization.
- Run the complete fail-before/fail-after and restart sweep.

Exit: model tests prove every interruption state and native tests open a first-use keyset
on Windows and Unix.

### Stage D3 - immutable local receipt store

- Implement typed V1 filename mint/parse.
- Replace/wrap the weak generic sink/source calls with bounded ownership-bearing APIs.
- Implement exclusive final-name publication and independent read/enumeration bounds.
- Implement platform publication property proofs, the explicit `--receipts` root, and
  `--receipt-last` partial-candidate/root-derivation behavior.
- Reuse tested old-store mechanisms only where they satisfy this document; do not carry
  old indexing, retention, flags, or format assumptions forward.

Exit: native and modeled store tests are green; no automatic deletion exists.

### Stage D4 - production CLI vertical route

- Link `dorc-receipt-crypto` and `dorc-receipt-local` as production dependencies.
- Construct `LocalReceiptEdgeV1` from standard roots in the real binary.
- Route plan, apply intent, outcome, and why through it.
- Ensure `why` never initializes.
- Ensure required apply publication consumes the concrete platform proof.
- Ensure the default real apply path obtains `30Rb`'s genuine ready-session identity,
  traverses the concrete required-publication gate, and dispatches through that same
  held session under inert e2e transport. Safe refusal remains required for failure
  cases but cannot satisfy this exit; no bypass route exists.
- Add separate-process plan/why and apply/why acceptance cases.

Exit: the shipped binary writes and reads V1 receipts without fixture receipt
capabilities, and its production apply orchestration completes once under inert
transport after concrete required publication.

### Stage D5 - remove old durable

Only after D4 exit:

- delete old whylog format/parser/writer/store and every old flag/fixture/consumer listed
  in `30Rb`;
- delete any duplicate local-store implementation superseded by
  `dorc-receipt-local`;
- retain no compatibility reader or alias;
- run a lexical census proving one production key provider, one production receipt
  store, one receipt reader, and one receipt writer remain; and
- run `mise run both gate:full-quiet`, then the conductor's `gate:arc` at arc close.

## 30Rd:v1-acceptance-and-exit

The minimal production durable edge is complete only when all hold:

1. A clean profile can run the real binary and produce a rich V1 PlanReceipt.
2. A second process can open the same keyset and explain that receipt.
3. The real binary's production apply orchestration publishes rich intent through the
   production local durable edge before the first inert mocked dispatch and
   publishes/correlates an outcome afterward.
4. No fixture key, fixture ID source, volatile sink, or test provider is linked/selectable
   in the production route.
5. Signing and encryption private documents use separate canonical library-owned
   encodings and non-convertible roles.
6. Key initialization interruption at every durable step yields absent, incomplete, or
   ready; never a usable partial keyset.
7. An initialized missing/damaged key never regenerates or triggers plain fallback.
8. A wholly absent keyset with any existing/unknown/overflowed V1 store state refuses
   first-use generation.
9. Read-only opening can verify an authenticated skeleton when only the encryption role
   is unavailable, without exposing any write capability.
10. Required publication proves the honest platform baseline and a failed required
   publication makes zero dispatch calls.
11. Post-dispatch durable-only failure does not stop coherent simulated execution and
   does not catch integrity failures.
12. Filename and signed header agree on V1/species/order/receipt ID; maximum-order
    cohorts and newest partial/damaged candidates never silently select older history.
13. Windows and Unix native tests exercise the concrete store/provider; Unix permission
    assertions run on a real Unix filesystem.
14. Incomplete-initialization recovery has a typed defining case and human-authored
    manual procedure; the tool itself removes nothing.
15. Old whylog code, default store, flags, fixtures, and production consumers are gone.
16. The normal successful plan/apply path writes a receipt by default.
17. The dependency and lexical fences are non-empty and two-way.
18. No user-facing prose was authored by builders outside the loom process.

## 30Rd:stage-local-stop-conditions

STOP and return to the security designer/conductor if implementation appears to need:

- one file containing or deriving both signing and encryption secrets;
- automatic regeneration after any V1 keyset path exists;
- first-use generation while any V1 store entry, unknown entry, enumeration failure,
  or overflow prevents proving the store absent/empty;
- automatic deletion/repair of an incomplete or damaged keyset;
- a fallback from rich failure to plain without an already-existing explicit policy
  witness;
- a Dorc-specific environment variable selecting test keys, fixture provider, volatile
  storage, or weaker publication;
- custom cryptographic key serialization instead of the library-owned Age/PKCS#8 forms;
- a new provider backend, plugin, key command, passphrase, import path, or algorithm;
- local unsafe/FFI or a stale permission dependency to claim Windows ACL parity;
- pathname-based delete after key/store object identity is uncertain;
- retrying synchronization after failure;
- treating a complete-looking same-attempt page-cache read as proof after sync failure;
- a generic string path/name accepted by the authority-bearing publication API;
- a single ordered publication-grade enum that erases platform differences;
- `dorc why` creating keys, repairing state, probing a host, or mutating the store;
- hiding a latest partial receipt by falling back to an older complete one;
- removing the old whylog before the real-binary restart acceptance is green;
- disabling the production provider/store in ordinary tests because it is inconvenient;
- goldening random ciphertext or private key material;
- weakening a failure, compile-fail, lexical, or native-platform test to make the gate
  pass; or
- claiming production-complete key management, secure deletion, same-user protection,
  or cross-platform guarantee parity from this baseline.

## 30Rd:builder-handoff-checklist

Every implementing lane reports:

1. exact base and relevant steering files read;
2. every persistent V1 path/token introduced;
3. every secret-bearing type and where it is generated, serialized, exposed, and dropped;
4. every constructor/mint and complete production caller census;
5. every filesystem/randomness/clock operation and its injected edge;
6. the exact platform guarantee implemented on Windows and Unix/macOS;
7. every fail-before/fail-after point exercised and restart outcome;
8. every production/fixture fence added;
9. exact dependency feature and lockfile changes;
10. focused tests, both-platform gate, and separate-process acceptance results;
11. old durable surfaces deleted or still blocking stage exit; and
12. any judgment call returned rather than resolved locally.

## 30Rd:accepted-v1-limitations

The spike deliberately accepts:

- a plaintext private-key file protected by ordinary user-account filesystem isolation;
- weaker Windows permission verification than Unix;
- no automatic key recovery or rotation;
- no automatic receipt retention, so store growth can eventually block required
  publication;
- visible incomplete receipt files after crash;
- direct final-name publication rather than temp/rename;
- Age's internal OS randomness outside deterministic replay;
- local ordering based on a controller wall-clock hint that can move backward;
- key and receipt copies co-propagating under whole-profile backup/sync; and
- no claim against same-user malicious code.

These are bounded scope choices, not invisible TODOs. Any widening of claims or use
outside the spike reopens the relevant provider, store, backup, retention, and platform
questions before release.
