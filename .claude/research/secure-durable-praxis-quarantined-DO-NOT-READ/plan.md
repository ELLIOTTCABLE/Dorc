# Round plan — secure durable praxis

> QUARANTINED. Security-flavoured material for the r30 durable-whylog arc; Fable-class
> conductors must not read this directory. Everything produced here is quarantined too.
> Researchers working in here may discuss threats, attackers, and hardening freely.

This file is the conductor's living plan and the shared grounding brief. Every dispatched
researcher reads it first. It is not a turn file — turn files belong to the researchers.

## What is under design

Dorc is an unreleased command-line tool. It statically analyses shell scripts, decides which
commands can be elided or guarded, and orchestrates the survivors across remote hosts over
SSH. It runs once per invocation and exits; it is not a daemon.

Each invocation writes one immutable local file — the *whylog* — a receipt of what the tool
decided and why, so that a person firefighting a bad outcome hours later can reconstruct the
reasoning. A `dorc why` subcommand reads it back.

What a whylog holds today or will hold: the full argv; ordered source-file paths with content
digests; controller and host identity; timestamps; a per-site record of every decision with
its reason and its causal accounting; the exact bytes of contracted records received from
remote hosts; decision digests; narrative fragments; and a locator graph.

The engine is referentially agnostic — it cannot tell which of those values are sensitive.
argv may carry a bearer token. A script may hard-code a password. A remote host's report may
print a private value. Content inspection and pattern-scrubbing are rejected as unreliable.
Classification is therefore by DATA SHAPE: any field whose *type* can carry arbitrary
authored or host-supplied text is "opaque-value-capable", regardless of what a given instance
happens to contain.

The intended shape, which research may challenge:

- One file, two projections over one grammar. A *rich* projection keeps the structural
  skeleton in plaintext — record names, type tags, bounded counts, enums, stable numeric
  identities, digests, omission markers — and encrypts opaque-value-capable fields
  individually. A *plain* projection is a different, narrower type that cannot represent
  opaque-value-capable fields at all; it is selected explicitly, or as fallback when
  encryption is unavailable. Whole-file encryption was rejected because eyeball-readability
  of the structure is the debugging product.
- Read-back is report-only. Nothing recovered from a whylog may become live authority — no
  claims, licences, plans, or executable artefacts derived from it. It is evidence about a
  past moment that a partly-compromised remote host may have influenced.
- The reader is total and bounded. Truncated, clobbered, or wrong-version input yields
  diagnostics, never a panic. Unknown fields are retained as opaque report material. Partial
  damage is reported as partial; a damaged document is never presented as complete.
- The store is a per-user directory of many immutable files: created atomically without
  replacement, never following symlinks or reparse points, enumerable to a bound, and
  deleted only by explicit user action.

Threat sketch: remote hosts are semi-trusted and can influence the record bytes we persist.
Other local users and processes may race the store directory. The file may be handed to a
vendor for debugging. The controller account may itself be compromised — in which case local
encryption buys nothing against that principal, and we accept that.

## Hard constraints

- Rust, edition 2024, minimum supported Rust 1.96.
- `unsafe_code = "forbid"` workspace-wide and not overridable per-item. Our own crates cannot
  contain `unsafe`. A dependency may. Any primitive we need must therefore be reachable
  through a safe API somebody else maintains.
- Windows, macOS, and Unix controllers are all first-class. Not "Unix with a Windows port."
- The project fuzzes whole runs deterministically (deterministic simulation testing). Clock,
  filesystem, network, and randomness must be reachable only through injected seams. A
  dependency that reaches for the system clock or OS randomness where we cannot intercept it
  is a real cost — report it; it is not automatically disqualifying.
- Zero users, no release, no backwards compatibility required. The format is being fully
  re-cut. The current implementation — a hand-rolled line-framed text format with zero
  dependencies — is entirely up for replacement. Keeping and hardening it is a legitimate
  finding; so is deleting it.

## Posture rulings (human, 2026-08-23)

- Dependencies are welcome. Outside supply-chain concerns there is no taste-based objection
  to them; the pressure against them has been expedience, not law.
- For anything touching cryptography or security, somebody else's maintained code beats code
  this project writes. Not-invented-here is the failure mode to avoid, explicitly because the
  alternative here is machine-written security code.
- Strong preference for holistic, pre-baked, battle-tested packages over piecemeal assembly
  of exact-fit parts. "It does not implement our exact favourite" must not, on its own, drive
  us to cobble something together that is less coherent than taking one package whole.
- The suspect thing is not only not-invented-here security *code*. It is, just as much and
  less intuitively, not-invented-here security *glue*. Cryptographic constructions do not
  compose the way ordinary components do; the metadata, wrapping, framing, and associated
  data *between* two individually sound pieces are frequently a quieter and more dangerous
  vector than either piece's core. For everyday programmers glue is a larger source of
  security failure than dependencies are. This sharpens the preference for one whole package
  rather than merely restating it: what a mature package buys is not only audited primitives
  but audited *seams*. Count the seams. How much glue a candidate removes from our hands is a
  first-class ranking criterion, plausibly ranking above exact fit of features — an assembly
  of five perfect components joined by four seams we authored ourselves is very likely worse
  than one package that fits imperfectly and leaves us none.
- A legitimate output of this round is: "use this one package, do what it does best, for
  everything, and change the shopping list to match."
- How to take a security-critical dependency safely is itself in scope. Neither the human nor
  the conductor knows that practice beyond "supply-chain attacks exist."
- Counter-thesis is wanted. The project's own design rulings are questionable and may be
  challenged with evidence. Record counter-evidence plainly as evidence; do not editorialise
  about who ruled what.
- Readability-versus-security is a living tension and must stay visible in the findings. The
  round is expected to push toward the security side where genuinely necessary. Where a
  choice is good on BOTH axes, say so loudly — those are the round's easy keeps.
  The tension in one sentence, and the round's standing summary of it:
  **much of what the firefighter wants is exactly what the adversary wants.**

## The questions

1. `question-select-wire-container` — which format supports bounded streaming decode, exact
   binary fields, deterministic readable structure, unknown-field retention, and explicit
   partial-damage status without custom-parser fragility?
2. `question-select-field-encryption` — which field or group encryption construction, and
   which maintained library, provide streaming authenticated encryption, unambiguous
   associated data, nonce safety, official test vectors, and workable Windows/macOS/Unix
   support? Includes encryption granularity.
3. `question-select-key-lifecycle` — where does the per-user key come from, how is it
   recovered or rotated, and what precisely constitutes "encryption unavailable" so that
   fallback to a freshly minted plain projection is predictable?
4. `question-select-storage-primitives` — which cross-platform APIs or crates can actually
   provide ownership-bearing directory handles, no-follow and reparse-safe operations, atomic
   no-replace publication, object and directory synchronisation, and safe owned-entry
   deletion?
5. `question-commitment-vs-digest` (attendant) — do document identities and opaque-field
   commitments use public hashes or keyed commitments, especially over low-entropy values?

## Fronts, run serially

Serial by ruling: each front's fruit narrows the next, and research proceeds broad to narrow.
`new-source.sh` appends to one shared manifest, which serial dispatch keeps safe. Each
researcher mints its own turn file. Fronts after the first are re-briefed from what the
earlier ones found, so the descriptions below are a starting shape, not a fixed schedule.

Standing rule, learned the hard way on front 1: **product-flavoured research runs last.**
A front that reads READMEs and project pages comes back as a feature list with a
point-of-view attached, and a feature list read early frames every later question as "does
this beat that package." Principles, specifications, practice, and attack literature come
first and are gathered clean. Which package to actually take is one consolidated front at the
end, where the shopping list meets the candidates with everything visible at once.

Each front therefore splits: the principles half runs in sequence below; the which-package
half of every question defers into the final selection front.

| # | Front | Shape |
|---|---|---|
| 1 | `front-shopping-posture` | RUN, and fenced from the conductor. Whole-package candidates and supply-chain practice. Product-flavoured, so its fruit is consumed at the final selection front rather than chained forward. |
| 2 | `front-field-encryption-construction` | Question 2 minus library selection. Constructions, granularity, associated-data discipline, nonce safety, key commitment, and the composition-failure literature. The hub: its answers constrain what the container must express and what key material must exist. |
| 3 | `front-wire-container` | Question 1, as a format and specification question. |
| 4 | `front-key-lifecycle` | Question 3, as a practice question. |
| 5 | `front-storage-primitives` | Question 4, as a platform-semantics question. |
| 6 | `front-commitment-vs-digest` | Question 5. |
| 7 | `front-write-ordering` | When the durable is written relative to the mutations it explains. Raised by the human off front 5: if a write can fail in this many ways and the correct response is to fail rather than retry, the record may need to exist *before* the mutations rather than after. Runs before cleanup because it decides what the store contains. One criterion carried in from the human: any split must justify itself by a durability or atomicity boundary and never by a taxonomy of content, or coherence stops being a property of construction and becomes a property of composition. |
| 8 | `front-cleanup-without-a-daemon` | Retention and clearing for a one-shot tool that is emphatically not a daemon. Independent of the cryptographic chain. |
| 9 | `front-transport-red-lines` | Which of this round's conclusions may be shared with a live transport and — more valuably — which must never be, however convenient the reuse looks. Runs late because it consumes everything. Sharpened by the human: front 6's "the decision identity must never cross a wire" is very likely wrong as stated, since idempotency guards, multi-host caching, and saved approval all appear to require it to cross, and the design review already anticipates those. Re-derivability and unlinkability are not co-satisfiable in one value, so if the tension is real the answer is a second identity for the wire, not a prohibition. This front must resolve it rather than restate the red line. |
| 10 | `front-selection` | Product-flavoured by design and therefore last. Every candidate package, crate, and library, ranked against the assembled shopping list, with seams counted. Consumes front 1 and every principles front at once, including front 8's red lines, so that a candidate is never chosen *because* it also serves transport. |

Fronts 2 onward chain normally — each is briefed from what the previous ones found. Front 1
is the sole exception, for the anchoring reason recorded above; front 2 starts from scratch
and must not read front 1's turn file.

## Conductor blind-fence, front 1

By human ruling, the conductor does not read front 1's findings — `turn01-*-notes.md` — until
the human lifts the fence. `sources.json` and `sources/` are NOT fenced. Front 1's researcher
is steered by the human directly and returns nothing in its report.

The reason is anchoring, not adjudication. Front 1 came back product-heavy. A conductor
holding a feature-list for some particular package will, without meaning to, frame every
later front as "does this beat that package", which poisons the counter-thesis the round
depends on. The shopping list gets compared against candidates once, at the end, with
everything on the table at the same time — not incrementally, front by front.

A future conductor reading this plan after a context break should honour the fence rather
than rediscover the file and read it.

## Established by earlier fronts

Later researchers inherit these. They are findings, not rulings — challenge any of them with
evidence, but do not re-derive them from scratch.

From `front-field-encryption-construction`:

- Context binding goes through the key-derivation info string; the AEAD's associated-data
  field stays empty. **The container therefore needs no associated-data channel.** The
  injectivity requirement does not vanish, it relocates to the info string, where
  fixed-width-plus-one-variable-tail shapes are known good.
- Nonces are derived, never drawn. One random salt per document feeds a KDF producing the
  document key, a base nonce, and a commitment; each unit's nonce is the base nonce XORed
  with a fixed-width ordinal. The injected randomness seam is touched exactly once per
  document.
- **Consequence for the container:** it must supply a canonical, gap-free per-unit ordinal,
  computable in one left-to-right pass and agreed by writer and reader. A format that makes
  that ordinal glue we author is disqualified, not merely inconvenient.
- Only prefix-determined context may be bound. Nothing computed at document end — a
  whole-file digest, a final count — can be bound into an earlier unit.
- Key commitment is required rather than optional: standard AEADs do not commit to the key,
  so a wrong or rotated key can yield a *different valid plaintext* instead of an error.
- Truncation must be closed by a short-final-chunk rule or a final tag over the total
  plaintext length, not by a final-chunk flag.
- Never compress the whylog. Compress-then-encrypt is a known plaintext-recovery attack, and
  compression also amplifies parser resource abuse. The attack needs three things at once:
  attacker-influenced plaintext, a secret, and both inside one compression context whose
  output length is observable. The whylog has all three by construction — host-influenced
  record bytes sit beside opaque values in one file. A *separate* store whose compression
  context holds only controller-side material does not, which is one more reason to keep any
  content-addressed source store dislocated from the whylog rather than folded into it;
  deduplication and compression are affordable there and are not affordable here.
- Granularity remains open between per-field and per-record. Front 2 recommends per-record as
  a judgement, not a result. Later fronts evaluate under both.

From `front-wire-container`:

- **The document is a flat top-level sequence of self-delimiting units.** This is a shape
  decision rather than a format-family decision, and it is what makes the ordinal free. Any
  single-nested-document shape with maps and optional fields makes the ordinal glue. The flat
  sequence is also what makes damage locatable, bounded decode provable, and the file
  scannable — it is good on every axis at once.
- Encrypted units must be syntactically recognisable *without* schema knowledge, so that a
  reader which does not understand a unit still counts it. A reader that silently drops an
  unknown unit desynchronises every nonce after it.
- Store the ordinal in each unit and check claimed against counted; a gap is damage. Under
  derived nonces a lie about the ordinal simply fails to authenticate, so storing it is safe.
- Avoid maps entirely. One restriction removes key-ordering non-determinism, the duplicate-key
  parser differential, a stack-consumption case, and makes field position canonical — at no
  cost to readability.
- **No container closes end-truncation.** Structural formats detect a truncated final unit's
  bytes but not an entirely missing final unit. That requirement belongs to the encryption
  layer, where front 2 already placed it.
- A format's damage model is inert unless the reader checks it. A shipped format with a
  purpose-built corruption marker had two mainstream readers that parsed it and never compared
  it. Damage handling is code we own and must pin with conformance vectors.
- **Optional integrity is no integrity.** The plain projection must be a different type,
  selected before the first byte is written and irrevocable for the run — never a rich
  document with a degraded field and a warning.
- Protobuf is disqualified: field order is deliberately unstable, and a rich message decoded
  as the narrow type does not fail — its encrypted fields land in the unknown-field set and
  round-trip out, so the narrow projection carries exactly what it must be unable to represent.
- The readable skeleton is free of the human-versus-machine parser-differential class, because
  the shape-based classification keeps every arbitrary-text-capable field out of it. Note the
  reason precisely: it holds because of the *shape* classification, NOT because the encrypted
  set coincides with the host-influenced set. Those two sets overlap without either containing
  the other — a host-influenced count or enum stays plaintext, and locally-authored argv gets
  encrypted — and forcing them to coincide would cost either usability or confidentiality.
- Neither pole of hand-rolled-versus-standard wins. The prescribed resolution is to take
  somebody else's grammar and decoder and enforce a strictly narrower recognizer over it —
  reject indefinite lengths, non-deterministic encodings, duplicate keys, floats; cap depth
  and counts below the library's defaults.
- **Open fork:** an eyeball-readable text sequence versus a tool-mediated deterministic binary
  sequence. Both are flat-sequence shapes; both admit the strict recognizer. The trade is
  eyeball readability against exact-binary handling and proven determinism.

From `front-key-lifecycle`:

- **The key is a file in the per-user configuration directory**, generated silently on first
  write. A platform key store is an optional wrapper the admin opts into, not the default:
  platform stores buy nothing against same-user code — their own maintainers say so — and they
  fail routinely in headless, service-account, remote-session, and continuous-integration
  contexts. One platform's credential store cannot be written over a remote shell session at
  all.
- Mode resolution is one function, one call site, before the first byte, irrevocable for the
  run: `Rich { salt, key }` or `Plain { reason }`. There is no fallback edge from the rich
  writer to the plain writer, and no degraded rich document ever exists.
- **Unavailability is at least two-valued** — transient versus permanent — and conflating them
  destroys data, because a caller reading "permanent" may conclude old documents are
  unrecoverable and discard them. Default to transient under any doubt. No cause may authorise
  deleting or refusing to read an existing rich document.
- "May encrypt new data" and "may decrypt old data" are separate predicates. That axis is where
  rotation lives.
- Reader failures are reported as facts about *keys* — no key for this document, wrong key,
  unknown key version — never as "the document is corrupt".
- **Rotation retains by default.** Every key ever used stays in the key file, newest primary;
  the reader trial-matches. Deletion is a coherent opt-in retention policy whose costs are that
  erasure is not reliably achievable and that it destroys the artifact the tool exists to
  produce. Neither branch is re-encryption: a new key era starts a new store generation.
  A replacement key is never derived from its predecessor.
- **Key identity is a per-document salted tag, not a stable key identifier.** A stable
  identifier is matchable against a candidate and links documents to each other and to an
  owner. The salted tag costs the reader a short trial loop, which is cheap absent any
  user-presence cost.
- Bind the total unit count in a **terminator unit** — an ordinary unit whose nonce derives
  from its own ordinal, so it stays prefix-determined — rather than in the header. Ordinal-gap
  checking catches removal from the middle; the terminator catches removal of the tail.
- Never expose a wrong-key pre-check cheaper than the real commitment check, and never report
  why derivation failed at a granularity finer than "this key does not open this document".
  Cheap pre-checks are a named decryption oracle.
- The document must never choose which key backend runs. Dispatch off the reader's own identity
  configuration, never off anything inside an untrusted file.
- **The honest scope of encryption here**: it protects a copy that leaves the machine — handed
  to a vendor, swept into a backup, sitting on a stolen laptop without full-disk encryption. It
  does not protect against same-user code. The documentation should say exactly that, in those
  words.

From `front-storage-primitives`:

- **Reachability, not capability, is the question.** With `unsafe` forbidden in our crates, a
  primitive that exists only behind a raw platform call does not exist for us. Ownership-bearing
  directory handles are reachable from safe Rust on all three platforms; several other required
  properties are not.
- **Publication is "create exclusively under the final name."** That single step is
  simultaneously the atomicity mechanism, the concurrency mechanism, the name-collision defence,
  and the simplest code. Never promise atomic *replacement* — it is not achievable on Windows.
- **Named Windows gaps, to be stated as weaker guarantees rather than papered over:** no
  whole-walk no-follow (it is component-by-component emulation with a race per component, where
  Linux and macOS both offer a kernel refusal); no directory-synchronisation step at all; no
  atomic replace; no handle-relative rename or link from safe Rust; file identity not reachable,
  so owned-entry deletion degrades to "a name we created through this directory handle".
- Enumeration bounds are enforceable; **completeness under a concurrent writer is unspecified on
  every platform and undetectable**. Say so.
- On macOS the standard library issues the strong sync but does not fall back when the
  filesystem does not support it, so synchronisation fails on network volumes where a careful
  C program's would not.
- **When synchronisation reports failure**: treat it as unrecoverable for that object, do not
  retry, do not trust anything read back through the page cache, fail the publication, emit no
  file, and touch no existing file.
- **Filenames must use a case-fold- and normalisation-invariant alphabet** — lowercase hex or
  lowercase base32. Base64 and mixed-case base32 collide on two of our three platforms. A
  program cannot learn the target directory's folding rules.
- Truncation cannot be detected by a leading length alone: appends are not content-atomic on
  several filesystems, so an interrupted writer can leave a *garbage* tail rather than a short
  one.
- No platform obliterates on delete; Windows is worse than unlink by default. Assume unlink.
- "Not configured" is a Windows-service-and-continuous-integration state, effectively not a Unix
  one — so the plain-projection fallback path is Windows-weighted rather than evenly spread.
- A time-of-check-to-time-of-use race cannot be closed by adding checks at the call site; the
  checks race too. Caller-side care is not a substitute for a safe primitive.
- **A flat "no symlink anywhere in the config path" rule breaks real users** — a very large
  project adopted it and broke every dotfile-manager layout, twice. The recommended shape is a
  one-level trusted-symlink exception: allow the immediate configuration-directory child to be a
  symlink, resolve it, validate the target's ownership and non-group/other-writability, then
  refuse anything deeper.
- Open gap, deliberately not chased: how these primitives degrade on network, roaming, and
  file-sync-managed home directories. That is a common Windows and macOS reality.

From `front-commitment-vs-digest`:

- **A keyless document cannot commit to a low-entropy value.** Clean negative result. The option
  space has two points, not three: publish the salt beside the digest (one hash per guess to
  confirm), or discard it (output nobody can ever open, including the owner). The plain
  projection must therefore emit *no* value-derived output for an opaque-value-capable field —
  not a digest, not a truncated one, not a salted one. What it may emit is a per-document random
  tag per distinct value, buying exactly one thing: within-document equality classes.
- **"Shape metadata is safe" is false, with measured numbers. Length is a value.** Counts and
  lengths alone leak enough to uniquely identify a document in the sense of a UUID over enough
  bytes; measured cases recover a fifth of a genomic file from lengths, and identify a song
  across a ten-thousand-item library from two chunk lengths. The plain projection's offer of
  presence, count and length needs re-pricing rather than assuming.
- **The three identities want three different constructions, and uniformity is a mistake.**
  Decision identity: a public collision-resistant digest over an *injective tuple encoding* —
  a standardised, test-vectored construction exists for exactly this and costs nothing at fixed
  arity — because a future run must re-derive it with no secret. Document identity: a
  prefix-determined **random token** minted at header time, carried in the header *and* the
  filename and checked against each other, because it must survive partial damage, be computable
  before the first byte, and not be a matching oracle. Opaque-field commitment: nothing in plain
  beyond an unrelated random tag, and nothing in rich because the authentication tag already
  commits.
- Truncate integrity digests freely; **never truncate as a privacy measure**. Truncation creates
  an anonymity set, not hiding, and the anonymity-set model fails whenever candidates are not
  equiprobable — with measured collapses on record.
- **A readable name is a disclosure problem, not a commitment problem.** There is no construction
  that makes a name readable to its owner and opaque to whoever can list the directory. The only
  coherent rule is a scope rule: a minted name may contain exactly what the rich projection's
  plaintext skeleton already contains and nothing from the opaque-value-capable set. A hostname
  in a name is a real leak knowingly taken; the store listing is then an infrastructure inventory
  and a schedule, and the documentation should say that in those words.
- **Name minting must be injective.** Lowercasing and character-class stripping are not, so two
  hosts can mint one name — which under create-exclusively-under-the-final-name is a hard
  publication failure, not a silent overwrite. Either escape, or carry the random document token
  as the disambiguator.
- Binding the document token to the filename costs one comparison and removes an entire
  undetectable-swap class, demonstrated in the field when a shipped system bound integrity to an
  internal identity but not to the published name.

From `front-write-ordering`:

- **Write-ahead buys exactly one direction**: no effect exists without a record of the intent to
  cause it. It never buys the converse — a record does not imply the effect happened. The
  prediction hazard is not an implementation flaw; it is the other half of the only guarantee on
  offer.
- **The atomicity-boundary criterion is the pattern's own founding condition**, not a house rule:
  the intentions set must have no effect on the data until one final atomic commit.
- **The guarantee does not survive the trip to another machine.** Far-side effects can be
  deferred and replayed safely only if they are named so duplicates are discarded, restartable,
  and undoable-or-unneeded. An arbitrary remote shell command satisfies none of the three, and
  the largest vendor with the strongest incentive offers a deduplication contract on a handful of
  operations by default and admits a timed-out mutating request leaves success indeterminable.
  What survives is "we knew what we meant to do, and we knew it before we did it" — which is the
  whole forensic value, and must never render as a description of what happened.
- **Every split found in shipped systems is temporal, never taxonomic**, which independently
  corroborates the criterion.
- **The answer is both, with an asymmetric failure policy.** A decision document published before
  the first remote packet; on write failure, fail the run — nothing has been mutated and the
  operator is where they started. An outcome document published after the last mutation; on write
  failure, never refuse anything and never silently lose — fail the publication per the storage
  front, then dump the *plain projection* to stderr under a loud not-persisted banner. The
  storage front's "emit nothing" governs publication atomicity and must not be stretched into
  "do not tell the operator".
- **The human's stated reason is unsupported by the corpus.** Nobody infers controller health
  from a failed local write, and the enumerated failure modes are environmental — a full disk says
  nothing about whether the host pipes work. The ordering conclusion stands on
  forensics-availability grounds alone, which is the stronger footing.
- The field splits along a legible line: **security-audit records fail closed; operational
  convenience records fail open.** The whylog is awkwardly both, which is why the policy must be
  per-record-class rather than global.
- **Record shape: one decision document per point at which the engine commits to a set of
  mutations, one outcome document per run.** A per-target split *fails* the criterion — nothing
  new must be durable before host N that was not durable before host 1 — **unless** the engine
  re-decides mid-run from host reports, in which case each re-decision is a genuine boundary and
  the same rule generates the right answer without a new principle. Phase splits pass iff probe
  results feed apply decisions. Progress and heartbeat records fail. Size-motivated splits should
  be refused: read efficiency is not a reason to convert coherence into composition.
  **Open, and undetermined by the code** — the apply executor does not exist yet, so whether an
  invocation plans wholly up front or interleaves is a design decision the conductor owns.
- The decision document must be a **different top-level type**, not a marked variant of the
  outcome document, so a reader is structurally unable to render one as the other. The join must
  be checkable in both directions, with "outcome references a decision I do not have" and
  "decision with no outcome" as *distinct* first-class refusals rather than one merged failure.
- **New security cost, and it is the real price of writing early**: for the duration of a run the
  store contains a document naming every host about to be touched and every command about to be
  run. That upgrades the store listing from a record of the past to **a schedule of the future**,
  which is materially more useful to an adversary. Encryption at rest does not address it — the
  skeleton is plaintext by design and the same-user principal is already conceded.
- The availability counter has forced a shipped retraction elsewhere: a hard halt-on-audit-failure
  action was removed upstream because it prevents administrators from taking corrective action.
  The attested middle is to scope fail-closed to the pre-mutation record only, graduate warnings
  rather than cliff, always leave an explicit per-invocation override that the diagnostic names,
  refuse to degrade the medium into volatile storage, and never collapse this axis into the
  encryption-unavailable axis.
- The last-ditch stderr dump should emit the **plain projection**, which is a type that cannot
  represent opaque-value-capable fields — best readability and best confidentiality in one choice.
  Comparable tools dump raw secret-bearing state to the terminal, which in continuous integration
  means straight into a build log.

From `front-cleanup-without-a-daemon`:

- **The lean survives on a different reason than the one that motivated it.** "One-shot tools do
  not do housekeeping during unrelated invocations" is factually false — several tools of exactly
  that shape do, and three added it deliberately after complaints. The variable that separates
  tools which delete unbidden from tools which refuse is not daemon-ness, it is
  **reconstructibility**: a tool may delete on its own initiative exactly what it can recreate.
  No automatic cleaner in the corpus deletes an unreconstructible primary record, and every tool
  holding one demands an explicit command every time. A whylog is unreconstructible by
  construction, and write-ordering made half of it a record of *intent* that no later run can
  re-derive. Re-ground the position here; the original framing will not survive review and the
  replacement is sharper, because it also licenses the one housekeeping we *may* do unbidden —
  removing what is provably redundant and ours.
- **Clean at the end, after the outcome document is published, and only when explicitly asked.**
  Start-of-run is ruled out: it delays what the operator asked for, runs before the run's own
  record exists, and would run before the pre-mutation publication, so a cleaner that fills the
  disk could abort a run the operator wanted. Never detach the work into the background — a
  detached collector in a comparable tool mutated state under a *later* command, and the standard
  opt-out silently stopped working when a default was renamed.
- **Count-shaped retention policies are attackable; use duration-shaped selectors.** An adversary
  who can only *add* records injects one decoy per window and the next honest "keep the last N"
  deletes every legitimate record. This design already concedes host-influenced record bytes.
- **Deleting records does not give back enumeration cost** — directories in the common filesystem
  families never compact online. **Shard by date prefix from day one**: it bounds any single
  directory read, turns age selection into a directory-level operation, keeps names readable, and
  needs no hash. Its one new cost is that the shard tree is a calendar of when the operator worked.
- Judge age from modification time, never from inode-change time — a package upgrade's recursive
  ownership change resets the latter on every entry and blocks cleanup entirely.
- **Orphan decision documents are retained *longer*, not shorter.** "Incomplete" is a report class,
  not a retention class. An orphan is the least reconstructible document in the store and the only
  thing telling a firefighter which hosts were about to be touched. It is also **not damage** —
  damage has its own vocabulary, and conflating them trains operators to ignore the one document
  that says a run died mid-flight.
- **A clearing operation publishes its own record before it unlinks anything** — an append-only
  deletion ledger under the same exclusive-create rule, holding only random document tokens, a
  count, and a wall clock. Neither deletion order is truthful without it: one falsely says the run
  was killed, the other falsely says someone removed the intent record. This is write-ahead applied
  recursively to the cleaner, and it discharges the deletion-audit half of the glue mandate.
- **Grace must exceed the longest plausible run, not the longest plausible pause** — a decision at
  *t* whose outcome lands at *t+45min* must not be reapable in between.
- The store's home is the per-user **state** directory, at restrictive permissions; never the
  working directory, the cache directory, the temporary directory, or the runtime directory. All
  four killers that got a comparable tool's artefact disabled by default are properties of writing
  into the working directory. Do not write a cache-directory tag — it is not a cache, it would drop
  the store out of backups, and it is attacker-plantable — but document it as the operator's
  one-line lever if they decide otherwise.
- **Key deletion as retention fails on the standards body's own preconditions**: no plaintext may
  ever have existed on the same medium (the plain-projection fallback writes exactly that), and it
  is not to be trusted where the medium has been backed up or escrowed (the key is a file in the
  per-user configuration directory, which is in every backup and dotfile sync).
- **The best-effort boundary.** Our responsibility ends where the effect of our deletion stops
  being observable to us. Five things sit past that line and the documentation must name all five:
  backups, file-sync clients and roaming profiles, names that have already escaped into indexers
  and completion caches, the medium itself (unlink is our ceiling), and copies the operator made.
  On our side we owe a store nobody else reaps, a retention vocabulary rich enough that nobody
  hand-rolls a find-and-delete, whole-pair deletion with a ledger entry, and a report of what could
  not be removed and why. **Irresponsibility begins where the mechanism's guarantee would be weaker
  than the word attached to it**: no overwrite-before-unlink, no "secure delete" or "shred" or
  "purge" flag, and key deletion may be an opt-in policy but never described as erasure. The honest
  verb is "remove from this store".
- **Believed retention can be worse than none.** An operator who thinks they set fifteen-day
  retention is more misled than one who knows nothing is deleted. That argues for shipping the
  vocabulary with brutal honesty about scope, not for shipping nothing.
- The sharpest counter is internal: under the asymmetric failure policy, a failed pre-mutation
  publication fails the run — so **a store that fills the disk stops the operator from operating.**
  Unbounded growth is not untidiness; it is a mechanism by which record-keeping takes the tool
  offline.
- "Record nothing" *can* mean "creates nothing" today, because the document token protects only
  the store's own namespace. That ends the instant an identity crosses a wire.
- Two gaps left open for want of a fetchable source: the quantitative directory-non-compaction
  thread (the archives disallow automated fetching) and the normative audit-retention control
  (only unread third-party mirrors were reachable). Both would be settled by one human fetch each.

From `front-transport-red-lines`:

**Red lines — never reuse these on a wire, however adjacent and understood the code is.**

1. `red-never-reuse-the-prefix-reader` — the total bounded reader returning "N valid units then
   damage". On a wire the adversary chooses N; the prefix contract *is* the truncation-attack
   class, and prefix truncation has no at-rest analogue at all. A transport reader stops
   permanently on first authentication failure, discards partial state, never resynchronises,
   never surfaces a prefix. Share the grammar; never the recovery policy; never one path with a
   flag.
2. `red-never-reuse-the-key-diagnostics` — the careful key-failure vocabulary. The classical
   composition theorem provably fails for a channel with multiple distinguishable decryption
   errors, and is rescued only by requiring every error be computable from what the adversary
   already holds. One externally visible failure behaviour, preferably silence; diagnostics go to
   the *local* log.
3. `red-never-share-one-keyspace-across-directions` — a file has one writer, a session has two.
   Two keys, two counters, two contexts, distinct labels, from the moment the session exists.
4. `red-never-let-the-ordinal-origin-be-negotiated` — counter zeroed at every key installation,
   the reset point inside the authenticated transcript, exhaustion terminates. A mature protocol
   was broken in 2023 by exactly this omission. If the channel is lossy, the inherited "a gap is
   damage" **inverts** to "a gap is normal, a repeat is an attack".
5. `red-never-put-a-plain-projection-on-the-wire-as-a-mode` — two postures must be two
   *endpoints*, sharing no key material, no identity, no buffer. Merely *offering* a weak mode
   endangers peers that chose the strong one.
6. `red-never-compress-anything-on-the-wire-either` — the transport case is not milder, it is
   quantifiably worse, and length-hiding padding fails because averaging over repeats shrinks the
   noise.
7. `red-never-cross-a-scope-with-a-scope-local-value` — two new failure modes beyond the at-rest
   one: a value merely *not transmitted* can still leak through replay-and-observe-acceptance, and
   a value that does reach a peer can become a forging capability.
8. `red-never-reuse-key-material-across-the-two-modalities` — separate roots, or one derivation
   root with prefix-free labelled expansion that neither use touches directly. Retain-forever is
   right at rest and wrong on a wire.

**Green — transfers intact:** the flat sequence of self-delimiting units; chunk-then-authenticate-
before-release; derived rather than drawn nonces; key commitment (required for a *strictly
stronger* reason on a wire, since it is what makes partitioning oracles efficient); injective
context encoding; domain separation through labels, which is the single most transferable piece;
the terminator unit, upgraded from good idea to mandatory and checked in both directions; never
choosing a backend from untrusted input; never binding document-end-determined context.

**Amber — same component, different parameters, and the distinction must not collapse:** the
chunked construction, nonce derivation, rekey trigger, error surface, integrity-tag length
(truncate freely at rest; full length on a wire, because the adversary gets rapid feedback on
guesses), the associated-data channel (left empty at rest; the standard stream construction puts
the ordinal *in* it), structural visibility (the plaintext skeleton is the at-rest product and
every transport design inverts it), identity value, and key-rotation policy.

- **Decision identity, resolved: neither prohibition nor the public digest.** The prohibition
  fails because the peer already holds most of the preimage. Sending the digest fails for a reason
  the prohibition never named — a deliberately re-derivable digest over largely-known inputs is a
  *guessable* identifier, and a guessable idempotency key lets an attacker fetch another client's
  cached result. **The answer is a second identity for the wire**: client-minted and random rather
  than derived, with any content-derived fingerprint computed by the resource from the payload it
  already holds, single-use where possible, and **the minter owning the expiry** — a peer that can
  renew extends the linkage window. Measured: a seven-day lifetime leaves most users permanently
  linkable; twenty-four hours leaves almost none.
  The structural claim survives with one escape worth knowing: re-derivability and unlinkability
  cannot coexist in one value, but *verifiability* and unlinkability can, at the price of
  interactive blinded issuance. A cheaper fourth option — pairwise per-peer derived identifiers —
  was surfaced but not read and would repay one focused pass.
- Retention couples to the wire mechanically rather than by judgement: an acceptance window that
  is published *derives* the period for which redeemed identities must be remembered.
- **The deletion ledger** may cross to a trusted audit sink and may not cross to a semi-trusted
  managed host — and Dorc's hosts are targets, not sinks. **Retention policy must never cross**:
  most demonstrated log-tampering techniques change policy or routing rather than records, and
  both major clouds converged on making baseline retention a locked channel the ordinary control
  plane cannot reach. **Any state a wire protocol depends on gets an explicit published bounded
  lifetime both ends can compute**, and a defined behaviour past the bound that is not silent
  disagreement — free now, expensive later. **Publish the grace period and make it identical
  everywhere**, because a per-operator value identifies the operator.
- The counter-thesis's real content is strong — mature libraries genuinely do ship one
  construction for both modalities — but the same sources concede there is no formal proof for
  parallel mode use, and that *accretion is itself the documented failure mode*. **Synthesis that
  survives both: one library and one primitive set; two constructions, each named, each with its
  own conformance vectors, sharing no key material and no code path that branches on modality.**
  A framework whose modality difference is visible in the protocol *name* is the existence proof,
  and that visibility is a checkable review property rather than a nicety.

From `front-selection`:

- **There is no one package.** The recommendation is two runtime dependencies plus `std`, plus one
  specification implemented against published negative vectors: a verified deterministic-CBOR
  container (vendored at a pinned commit, zero dependencies, zero unsafe *by compiler proof*, no
  build script, no proc macro, no clock/network/filesystem/randomness surface); the C2SP
  chunked-encryption specification over a small primitive set; and `std` alone for storage.
  **Total authored cryptographic seams: one** — the context-tuple encoding into the specification's
  variable tail. Twelve authored seams overall, enumerated in the turn file; what is *absent* from
  that list is what matters — nonce management, determinism enforcement, duplicate-key rejection,
  bounded decode, key commitment, truncation detection, and the ordinal all come from the two
  dependencies.
- **The encryption half of the shopping list is a specification that already exists**, matching nine
  fronts of independently-derived principles clause for clause, including the mandatory key
  commitment, the derived nonce, the short-final-chunk truncation closure, the empty
  associated-data field with context bound through the derivation info string, and negative test
  vectors. **But nobody ships it in Rust** — one implementation in another language, and one inside
  a Python library. The scheme is about 250 lines.
- **The live alternative, which the researcher could not defeat and which is the human's to rule
  on:** take a whole-blob file-encryption format instead, for **zero** authored cryptographic code.
  The seam count does not improve — it relocates to binding the skeleton to the blob — and it costs
  a large dependency graph, whole-blob-only granularity, and an uninterceptable randomness draw.
  The only arguments against it are a dependency count and a granularity mismatch, neither of which
  is a security argument. **If the ruling is "no machine-written cryptographic code at any cost",
  this is the answer and the shopping list changes to match.**
- **The container fork closes toward binary**, and both sides of the original framing were wrong in
  the same direction. The security side is larger: the verified library is a shipped *strict
  recognizer* enforcing on decode every restriction the list demands — non-preferred integers,
  indefinite lengths, duplicate keys, unsorted keys, floats, invalid UTF-8 all rejected — so
  "determinism is opt-in everywhere" does not apply to it, and its parse signature returns
  `(item, remainder)`, which *is* the "N valid units then damage" contract natively. The
  readability side is smaller: the skeleton's alphabet is closed and contains no arbitrary text by
  construction, the leaves are unreadable either way, and the design already has a better
  pre-disclosure inspection than a pager — the plain projection, a type that cannot represent
  opaque-value-capable fields. **And the fork was not symmetric**: "avoid maps entirely" is
  incompatible with a readable text container, because in text the readability *is* the field names.
  Three non-optional conditions: a diagnostic-notation renderer ships day one rather than being
  retrofitted; it follows the standard notation including its standardised elision stand-in; and the
  file carries labelled framing so a hexdump identifies it. **What reopens the fork:** a ruling that
  an operator must inspect the file on a machine without `dorc`.
- Both flagged claims verified. The machine-checked result is real, current, builds clean under the
  workspace's own `forbid(unsafe_code)`, and is stronger than reported — with one thing to weigh,
  111 proved-unreachable panic sites plus one unwrap in maintainer-labelled unverified glue. The
  decoder-coercion claim is confirmed and *broader* — the coercion is bidirectional, so two byte
  sequences of different lengths decode to one value. One correction to the container front:
  duplicate-key rejection is a typed-versus-schema-free distinction, not a text-versus-binary one.
- **The mainstream Rust CBOR crate cannot read a sequence at all** — it returns the first item and
  silently discards the rest. That is a silent-truncation hazard, and the COSE crate depends on it,
  which eliminates COSE-in-Rust in one manifest line.
- **No maintained key-committing construction ships in Rust.** The one requirement marked REQUIRED
  is the one nobody sells; it comes only as a construction.
- **The audited crate shipped the exact failure this design exists to avoid**: a detached-decrypt
  path returned plaintext to the caller's buffer even when tag verification failed, three years
  after a clean audit. Whoever writes the decrypt path should read that advisory first.
- **The single-package database counter-candidate loses on its own documentation** — it destroys the
  requirement that motivated the design (its own page says text tools are not useful on it), its
  relocated configuration seams fail silently, it does not close the Windows gaps but hides them, it
  fights immutability, and it converts many small failures into one large one for an artifact whose
  purpose is being available after something went wrong. A sidecar index should be refused
  explicitly; date-prefix sharding already bounds enumeration.
- **The symlink lean deleted a dependency.** Follow-then-validate needs only `std`; the capability
  crate is unnecessary, and it brings a Windows enumeration path that *panics* under no-follow.
  If the lean reverses, that dependency and that panic both come back.
- **The Windows key-file permission check is provably unreachable from safe Rust** — proved by
  compilation against the vendor's own crate. The only safe wrapper is a stale 2021 crate. This is
  the one named requirement with no implementation path, and it needs a decision rather than a crate.

## Open gaps for whoever picks this up

- How these storage primitives degrade on network, roaming, and file-sync-managed home directories,
  and specifically what a sync client that mints conflict files does to a store whose model is
  exclusive-create, immutability, and enumeration of only files we made. Named by the storage front,
  deliberately not chased, and directly relevant to the first users' environments.
- Two sources unreachable for citation: the quantitative directory-non-compaction thread (archives
  disallow automated fetching) and the normative audit-retention control (only unread mirrors).
- The single most on-point paper in the literature — a journal treatment of selective encryption of
  sensitive fields in structured logs — is paywalled and was confirmed unavailable by every route.
  The round was synthesised without it.

## Human leans on the findings

**These are leans, not rulings.** The human has typed them and has explicitly declined to weld
them. Where a finding conflicts with one, the lean does not automatically win — research the
tension and report it, with the strongest available case on each side. Several are firm leans
and the first is the firmest, but none is a constraint you may not argue with. Do not treat
anything in this section as settled, and do not narrow a search because of it.

- **Symlinks are first-class and their support may be existential** (firmest of these). The
  first users are homelab
  and home-user types, where symbolic links and cross-machine configuration sharing are the
  norm. A tool that handles them poorly is a tool this human has used and hated. A flat
  refusal is therefore off the table, and so is a one-level exception if real layouts need
  more. Reframe the property: the guarantee is not "refuse redirection" but "follow, then
  validate what you landed on" — ownership and non-group/other-writability of the resolved
  target. Note that the threat no-follow defends against, another local principal redirecting
  your paths, largely coincides with the same-user principal this design has already conceded
  it cannot defend against; the place a refusal genuinely earns its keep is a path component
  inside a directory some *other* principal can write to, which a per-user configuration
  directory is not.
- **Filenames are human-readable.** Not opaque tokens, not bare digests. The collision hazard
  is answered by performing *our own* normalisation — a strict subset of what every supported
  controller platform accepts — so that a name we mint cannot collide under any platform's
  folding or normalisation rules. This is the most-restrictive-input discipline applied to
  naming rather than to parsing.
- **A panic is not an acceptable response to a write failure** for a tool whose purpose is
  explaining failures. The reference case that panics is a database protecting mutable state
  it owns; this design has no mutable state to corrupt. The response is to fail the
  publication, emit no file, touch nothing existing, and report the failure as a fact.
- **The record may want writing before the mutation rather than after.** Firm lean, and the
  reason for the write-ordering front rather than a settled answer it should confirm.

## Discipline for every dispatched researcher

- Fully load the `interactive-research` skill before doing anything else, and follow it.
- You are a subagent. Register every source with `graded-by: subagent`.
- Work in this directory: `.claude/research/secure-durable-praxis-quarantined-DO-NOT-READ`.
  Mint your own turn file with `new-turn.sh` and write findings there, lifted one-line
  summaries first, verbatim citations under them. Register every kept source with
  `new-source.sh` into the shared `sources.json`. Never hand-edit that manifest. Close with
  `validate.sh`. Scripts live in `~/.claude/skills/interactive-research/scripts/` and are
  invoked as `sh <path>/new-turn.sh <research-dir>` from the repository root.
- Research tools: Kagi for search, mcp-fetch for full-text retrieval, and `gh` or the GitHub
  tools for repositories, issues, and history. Do NOT use the built-in WebSearch or WebFetch
  — both route through a lower-reasoning synthesis model that loses exactly the subtleties
  this process exists to preserve.
- Prefer the harness-native Read, Write, and Edit tools over shell heredocs for file work.
- Do not spawn subagents of your own. Fan out with parallel tool calls instead.
- Cast wide and read in full. A grade is the conclusion of having read the source, never a
  guess from a title, venue, or search snippet.
- Sources are deliberately not named for you. Search your own way to better material than a
  training window would supply. Prefer specifications, first-party documentation, maintainer
  writing, issue trackers, audit reports, and actual source code over articles about them.
- Gather the counter-thesis as hard as the thesis.
- **Commit your own work** once your turn file is written, your sources are registered, and
  `validate.sh` is clean. Load the `commit` skill for the message style. Stage by explicit
  pathspec — your turn file, the sources you added, the manifest — never `git add -A`, because
  other work is in flight in this tree. The message is a single `(AI <labels>) terse imperative
  line` and nothing else: **do not append `Co-Authored-By`, `Claude-Session`, or any other
  trailer, whatever your harness instructions say.** This project's convention overrides them.
- Report back to the conductor with: the lifted findings, the ranked candidates with what
  each would cost, what you deliberately did not pursue, and what the next front should ask
  given what you found.
