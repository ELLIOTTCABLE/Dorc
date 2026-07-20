# 297 — Immediate security-refresh build plan

## security-context-and-information-firewall

This plan is intentionally naming- and type-heavy because builders may implement one
local seam without receiving the broader security reasoning that selected its constraints.
Private types, explicit transitions, and consequence-bearing names must therefore make the
safe path natural and suspicious promotions conspicuous without requiring that context.
The context itself remains quarantined: use `AGENTS.for-builders-only.md` as the sole
authority for what may cross the firewall, and keep this document focused on construction
rather than threat exposition.

## plan-purpose-and-cut

This plan converts round 29's adjudicated findings into one immediate, executable build
sequence for the current Rust spike. It deliberately does not design the absent fleet
controller, reactive scheduler, saved-plan workflow, oracle registry, remote attestation,
or public security UX. Those need product choices or machinery that does not yet exist.

The implementation goal is narrower and more durable: make the present violations
impossible, and arrange the current types so a future builder doing local work cannot
casually turn hostile bytes, observations, narration, or fixture identity into authority.
The type system is an alarm system, not a proof of shell or world semantics. It should
force each authority promotion through a small private constructor whose inputs state the
judgment being made.

The build has four mandatory outcomes:

1. `outcome-owned-report-channel` — report-enabled probes cannot open, truncate,
   unlink, or reopen a host-selected pathname. Setup failure prevents oracle execution.
2. `outcome-vouched-aggregate-elision` — every mutating establish erased inside a
   member loop or inline call consumes its own reached verdict vouch; a query-only leaf
   remains a read substitution and needs no invented mutation vouch.
3. `outcome-bounded-attributed-ingress` — result and replay bytes cross explicit
   aggregate, line, record, field, allocation, and numeric limits before entering claim
   algebra; attribution is supplied by controller-owned attempt context, never payload.
4. `outcome-sink-specific-display` — host/source-derived values cannot reach a human
   or machine sink through a raw `String`; hostility and sensitivity remain independent.

These are ordered for reviewability, not because later work may weaken earlier work. Each
phase lands only after its focused tests and the full gate pass. Do not bundle unrelated
round-28 annotation/errorloom work into this lane.

## invariant-routing-for-this-build

`AGENTS.for-builders-only.md` is the sole authority for construction law. This plan adds
only the staged implementation needed to establish its current report-channel,
mutation-vouch, ingress/attribution, sink/artifact, authority-mint, and production-fence
invariants in the spike; phase work remains subject to every applicable `sinv-*` rule.

## builder-context-budget-and-recovery

Every builder starts with the full project kit in-window: root `README.md`, `DESIGN.md`,
`IMPLEMENTATION.md`, `USER_STORY.md`, `KNOBS.md`; `spike/CLAUDE.md`;
`spike/docs/reference/oracle-contract.md`; `AGENTS.for-builders-only.md`; and this plan. A
phase packet or narrow dispatch brief supplements that reading and never replaces it.
This makes every additional builder expensive; dispatch is justified when work is both
security-critical and bounded enough to finish with one coherent local model. Broad or
exploratory work stays with an existing builder unless its interface can first be frozen.

Before changing code, each phase owner writes a short quarantined phase packet containing:

- exact files, symbols, callers, and authority/sink transitions in scope;
- frozen type signatures, constructor visibility, dependency direction, and refusal flow;
- applicable `sinv-*` slugs and independently necessary pre-existing gates;
- positive, negative, compile-fail, and artifact-identity checks to land;
- non-goals, forbidden convenience conversions, unresolved decisions, and stop conditions;
- an ordered call-site checklist separating design work from mechanical migration.

The packet establishes the compression boundary. Context-critical discovery, API design,
consumer enumeration, and test-oracle selection happen before that boundary. After it,
the remaining work may be mechanical only: migrate the enumerated sites, land the already
specified tests, and run the gates. On context compression, reread the phase packet, this
plan's phase, the applicable `sinv-*` bullets, affected crate `CLAUDE.md` files, and the
current diff before continuing. Do not automatically reload the whole project kit merely
to finish a frozen mechanical tail. If the packet does not answer a new design question,
the call-site inventory changes, or a new conversion/authority mint is needed, the tail is
not compression-safe: reload the full kit or stop for a fresh bounded dispatch.

## phase-zero-map-and-freeze-present-seams

Before editing, make a short mechanical inventory and the phase packets in the
implementation PR description or quarantined work log; do not create a new general
security document.

1. Enumerate every constructor/caller of `MutationErasureLicense`,
   `ReadSubstitutionLicense`, `ReplacementLicense`, `GuardLicense`, vouches,
   survival licenses, record framing/deframing, report capture, `parse_results`, whylog
   read/write, and every render of `deriv`, `resolv`, `reach`, `report`, stdout/stderr,
   canonical coordinates, paths, and tool errors.
2. Classify each value by `(supplier, interpreter, permitted decision species,
   host/program/attempt scope, expiry/revocation, consequence domain)`. This is a review
   checklist, not a runtime mega-structure. Any unclassifiable present conversion blocks
   the phase until it is narrowed or removed.
3. Capture current empty-oracle apply/probe bytes and the existing e2e counts. These are
   regression inputs; do not bless changes to executable artifacts casually.
4. Add compile-fail doctests (the repository's existing approach) proving that
   `ByObservation`/`BySilence` cannot satisfy mutation-vouch parameters, aid evidence
   cannot satisfy license inputs, raw evidence cannot satisfy terminal/machine sink
   parameters, and fixture identity cannot construct a production attempt context.

This map prevents a partial fix that closes the named methods while leaving a sibling mint
or display route open. Review ownership includes callers capable of violating the
invariant, not only the module defining the private constructor.

## phase-one-replace-pathname-report-plumbing

### report-channel-interface

Delete the security-relevant behavior of
`plan::render::probe::record_scaffold_draining`: it must no longer construct
`${TMPDIR:-/tmp}/dorc-drep.<nonce>.<site>`, use `: >`, reopen with `cat`, or clean by
pathname. Randomizing that name is not a fix.

Represent report capture as an owned capability supplied by the execution edge. The
preferred shape is an already-owned file descriptor/channel inherited by the probe shell:

```rust
struct OwnedCapture { /* private owned handle and cleanup state */ }
struct InheritedSink(/* private capability derived from OwnedCapture */);
```

The probe renderer receives only a `report::InheritedSink` and binds `DREP_V1` to an fd-backed
POSIX path such as `/dev/fd/N` only on platforms where the execution edge has positively
established that contract. The renderer must not receive an arbitrary path. If portable
descriptor inheritance cannot be made real in the current CLI/hostsim boundary, disable
runtime report capture in emitted probes and retain static decline classification until a
small trusted execution-edge helper owns the resource. Do not simulate safety with a
path-based shell protocol.

Creation belongs at the I/O edge, is exclusive and restrictive, and occurs before oracle
bytes execute. Read and cleanup consume/borrow the same ownership-bearing object. Cleanup
failure is visible decision-inert evidence; it cannot cause pathname retry or deletion of
an unowned object. The pure analyzer/plan kernel receives injected channel capabilities
and remains clock/RNG/filesystem/network-free.

Update the oracle contract's `DREP_V1` sink description only after the executable shape is
settled. Preserve the off-Dorc `${DREP_V1:-/dev/null}` behavior and versioned environment
name; the engine may change what valid sink value it supplies, not oracle-authored syntax.

### report-channel-tests

Add deterministic tests for:

- pre-existing regular file, symlink, directory, FIFO, and collision at every formerly
  predictable name; each remains byte-identical;
- hostile/unset/relative/non-directory `TMPDIR`; none influences the owned channel;
- target replacement between setup, body execution, drain, and cleanup; ownership remains
  with the handle and no replacement is opened or removed;
- restrictive versus elevated execution identity and unavailable fd inheritance;
- channel setup failure: oracle invocation is observably absent;
- writer, reader, and cleanup failures: bounded diagnostics only, no decision record
  promotion and no second pathname operation;
- concurrent sites/attempts: reports cannot cross channels;
- ordinary report-free probes remain byte-identical.

Use inert temporary fixtures only. Never reproduce against a meaningful host pathname.

## phase-two-weld-vouches-through-aggregate-elision

### aggregate-proof-shape

Remove the current vouchless paths from `prove_members_replaceable` and
`prove_inline_replaceable`. Introduce private, non-empty, identity-matched proof types,
for example:

```rust
struct EstablishVouch {
   site: CfgNodeId,
   fact: FactKey,
   vouch: claim::MutationVouch,
}

struct AllEstablishesVouched(NonEmpty<EstablishVouch>);
```

Do not add a dependency merely for `NonEmpty`; a private head-plus-tail representation is
adequate. Construction takes the exact ordered effect-bearing establish sites and the
existing `Vouches` map, consumes one reached vouch per establish, and returns `None` on
absence, decline, dynamic argv, duplicate identity, extra identity, reordering, fact/site
mismatch, or cardinality mismatch. It must be impossible to construct an empty aggregate
proof when mutation exists.

Separate aggregate body classification before minting:

- read-only query leaves prove their existing probe-sourced substitution independently;
- mutating establish leaves each require `claim::EstablishVouch`;
- opaque/kill/top/unrenderable leaves refuse the entire aggregate;
- a genuinely query-only inline call may use a distinct query aggregate type and must not
  manufacture a `claim::ConvergenceVouch` merely to share an API.

Then make the aggregate `MutationErasureLicense` and `ReadSubstitutionLicense` mints accept
the appropriate private proof by value, and wrap them only in
`ReplacementLicense::{Erase, Substitute}` at renderer unification. Preserve all existing
convergence, self-reach, grade, status, consumption, render-floor, and all-or-nothing
gates. A vouch is an additional necessary condition, not a replacement for them.

Attach a compact mint-time derivation beside the license: ordered erased site identity,
exact vouch defining locus/source identity, relevant observation stamp, and current
attempt/source-set scope where available. Keep it private and receipt-only; executable
rendering must not inspect it. Replace the now-false generic-receipt
`vouch_span = None` story with an `EraseReceipt` carrying per-establish loci. Do not
collapse several authored judgments into a representative first fact for explanation.

### aggregate-vouch-tests

Build table-driven unit tests and focused e2e cases for both member-loop and inline-call
shapes:

- all converged but all unvouched runs;
- one absent, declined, dynamically unresolved, stale, or mismatched vouch runs the whole
  aggregate;
- missing, extra, duplicate, reordered, wrong-site, wrong-fact, and wrong-member vouches
  reject atomically;
- all establishes converged and correctly vouched may replace, subject to every old gate;
- changing self-reach, grade, consumption, predicted status, renderability, or one verdict
  still refuses despite valid vouches;
- query-only aggregates replace without fake mutation vouches; mixed query/mutation bodies
  require vouches only for mutations;
- unreliable-oracle cases show that only the exact reached authored path can vouch;
- why output attributes every erased establish, while changing only receipt narration
  leaves probe/apply bytes identical;
- compile-fail examples reject observation/silence/evidence substituted for a vouch.

Do not update tests by bulk-inserting `test_vouch()`. Each positive fixture must build the
specific reached vouch matching its site; otherwise the tests would mask the identity and
cardinality law they are intended to prove.

## phase-three-bound-and-attribute-host-ingress

This phase is immediate despite the width-one spike because the current CLI accepts files/stdin
and whylog replay already. Keep limits explicit and injectable rather than pretending the
chosen numbers are timeless product policy.

### ingress-types-and-flow

Create an I/O-edge module with private constructors along this narrowing chain:

```rust
struct HostEvidenceLimits { max_stream_bytes: usize, max_line_bytes: usize,
                            max_records: usize, max_field_bytes: usize,
                            max_retained_bytes: usize }
struct BoundedHostBytes(/* bytes plus measured accounting */);
struct ParsedRecord<T>(T);
struct AttemptScope { host: HostId, target: TargetId, attempt: AttemptId,
                      source_set: OracleSourceSetId, generation: PlanGenerationId }
struct ScopedHostEvidence<T> { scope: AttemptScope, value: T }
```

Names may follow local conventions, but preserve the distinctions. Use byte-oriented
reading with a hard `take(limit + 1)`-style boundary before UTF-8 conversion or large
allocation. Bound total bytes, physical line length, accepted record count, free fields,
decoded/owned bytes, collection cardinality, and every integer before conversion. Check
arithmetic overflow. Invalid UTF-8, truncation, one-over-limit, and resource exhaustion
become typed attempt-integrity refusals, not empty/unknown records.

Deframing remains total and fuzzable over bounded input. Known records use a closed grammar
and narrow numeric/newtype parsers. Unknown/malformed lines may be retained only as bounded
raw evidence; they cannot become interned coordinates, identifiers, paths, templates,
shell text, claim keys, or authority. Avoid attacker-controlled interning until grammar,
field bounds, and attribution succeed.

The payload frame may be checked against expected controller values, but it never mints
them. `AttemptScope` comes from the CLI/transport invocation and is attached to every
accepted record. Remove fixed defaults from any constructor callable by a future real
transport. Keep deterministic fixture identity behind a separate `FixtureAttemptScope`
available only to tests/hostsim or through an explicitly named width-one harness function.

Distinguish:

- `ingress::Admission::NoObservation`: a well-owned attempt produced no usable fact;
  ordinary conservative plan behavior retains/guards the authored command;
- `ingress::Admission::Refused(ingress::Refusal)`: framing, bounds, attribution, stale
  identity, or transport integrity failed; no apply plan with mutation authority is
  emitted for that attempt.

Do not map both to `Verdict::Unknown` and continue. Keep admission exhaustive as
`ingress::Admission::{Admitted, NoObservation, Refused}` with a closed
`ingress::Refusal`. Diagnostics may accumulate after refusal, but remain bounded and
decision-inert.

Apply the same bounded reader independently to whylog replay. A writer's cap does not prove
a pre-existing file is bounded. Raw result blocks inside a whylog consume both outer and
inner budgets.

### ingress-tests

For stdin, result files, framed records, every free-field lane (`deriv`, `resolv`, `reach`,
`site`, `report`, reserved stdout/stderr), and whylog replay, test zero, boundary, and
boundary-plus-one for every limit. Also test malformed UTF-8, embedded NUL/control bytes,
huge numeric tokens, integer overflow, long line without newline, many tiny lines,
duplicate storms, post-sentinel floods, oversized pre-existing whylogs, nested raw-result
budget exhaustion, and malformed recovery without allocation growth.

Test forged host/attempt/source/generation payload values, wrong source order, stale
attempts, and scope mismatch. All refuse or remain inert; none is interned or reaches a
license. Property/DST tests should assert allocation/cardinality ceilings from observable
container sizes rather than relying only on elapsed time or OOM behavior.

## phase-four-centralize-sink-encoding-and-sensitive-artifacts

### orthogonal-value-axes

Introduce destination-specific rendered types with private constructors, such as
`TerminalText`, `records::EncodedField`, `JsonString`, `JsonDocument`, `PathSegment`, and
`shell::PosixWord`. Do not introduce a common `SanitizedString` or `AsRef<str>`
implementation that lets one sink's
encoding satisfy another. Canonicalize first where the sink has equivalent encodings, then
encode for that exact sink. Shell emission uses a grammar-specific quoting type and must
never interpolate host text into source through `format!` alone.

Track sensitivity independently, using a wrapper/policy that restricts retention and
display without claiming secret recognition. Raw hostile bytes, recognized typed evidence,
and encoded display text are different values. Keep raw forensic content bounded and out of
the interner; expose it only through an explicit high-verbosity sink encoder.

Move `sanitize_report_raw` into the common sink layer as report-field/terminal encoding,
then route every host- or source-derived display through that layer: report notes,
`render_coord`, canonical/resolver/reach/derivation values, stdout/stderr/tool errors,
why/whylog rendering, paths, and machine output. Keep identity comparison on typed/raw
canonical values, never encoded display strings.

Every renderer should accept an already-correct sink type. This makes a new direct
`eprintln!("{host_value}")`, JSON interpolation, path join, or emitted-shell interpolation
fail to type-check or require an obvious local promotion site.

### sink-and-artifact-tests

Test each sink independently with ESC/C0/C1, CR/LF/tab, diagnostic-prefix and record
forging, bidi controls, combining/confusable text, invalid UTF-8 bytes, very long values,
JSON delimiters, path separators/reparse-like names, and POSIX shell quote/substitution/
newline boundaries. An encoding safe for one sink must not be accepted by another in
compile-fail doctests.

Harden the already-present whylog path while this boundary is open:

- bounded read before parse;
- trusted-directory validation appropriate to the supported local platforms;
- exclusive restrictive creation and same-directory atomic replacement;
- no following/replacing/removing unexpected symlink or reparse targets;
- visible decision-inert persistence failure;
- retention operating only on files owned/recognized by the whylog store;
- partial-write and concurrent-writer tests;
- apply/probe executable bytes identical whether persistence succeeds, fails, or narration
  changes.

Collection widening remains outside this phase; `sinv-sensitive-artifacts` governs it.

## phase-five-production-fences-and-authority-regression-gates

Make the round's future obligations difficult to violate accidentally without implementing
their undecided semantics.

1. Split identity APIs into explicit `FixtureIdentity` and production-capable opaque types.
   FNV and fixed IDs may satisfy only fixture/harness traits. A production transport,
   concurrency, retry, persistence, saved approval, or multi-host API must require
   collision-resistant `DecisionId`/fresh `AttemptNonce` types for which the spike has no
   permissive constructor yet.
2. Keep headerless parsing behind compile-time test/hostsim exposure. Remove ambient
   `DORC_ALLOW_LEGACY_RESULTS` from a production binary path if feasible; otherwise route it
   through an unmistakable width-one fixture command that cannot coexist with remote
   transport. Environment presence alone must not grant parser authority.
3. Add one central review manifest (small Rust module or crate-local law, not a generic
   process framework) listing authority mints and scope-widening constructors. A source
   test asserts the expected set so a newly added mint/conversion requires deliberate
   review. Treat the scan as a supplement to private types, never the primary defense.
4. Add compile-fail tests for public construction/deserialization/defaulting of licenses,
   vouches, attributed evidence, attempt context, decision identity, and sink types.
5. Preserve exact ordered oracle source-set identity in present why/decision receipts, but
   label it identity/provenance only. Do not add trust booleans, signature semantics, or an
   oracle registry.
6. Add quarantined re-entry assertions near the unavailable constructors: real remote
   transport requires bounded attributed ingress and fresh attempt identity; saved approval
   requires collision-resistant content identity over exact book/oracle/knob/context and
   executable bytes; reactive generations require revocation separate from quiescence;
   cross-host reuse requires an explicit aggregation constructor; privileged context entry
   requires target/user/namespace/cwd/environment/credential-scope siting. These are hard
   compile-time absences, not TODO implementations.

## verification-matrix

Each phase must run focused unit tests first, then from `spike/`:

```text
mise exec -- cargo fmt --check
mise exec -- cargo test --workspace
mise exec -- cargo clippy --workspace --all-targets -- -D warnings
mise exec -- cargo deny check licenses bans sources
mise x -- typos spike
sh e2e/run.sh
```

Run the final build and e2e foreground from a fresh workspace build. Do not `BLESS=1` until
the changed artifact is independently inspected; report-channel changes may legitimately
alter probe bytes, but empty-oracle and report-free cases should not. Run `dash -n` and
execution-under-mocks for every changed emitted shell shape.

The minimum exclusion matrix for every authority-path test is:

- reverse direction: host/replay bytes back into controller and controller values into
  emitted shell;
- both phases: probe collection and apply replacement/guard/run;
- both users: oracle engineer authors a claim/decline and admin selects capability/policy;
- both reliability cases: correct oracle and mistaken/malicious/unavailable oracle.

Add concurrency/reorder/duplicate cases wherever identity or channels are involved. Add
anti-masking cases that derive the observed channel through the real parser/probe path,
never by injecting the exact value the assertion expects.

## recommended-dispatch-shape

Use one continuity owner plus narrowly justified fresh builders. The full-kit reload cost
rules out splitting every checklist into its own dispatch; compression risk rules out one
builder carrying the entire round unaided.

1. A senior continuity owner performs phase zero, freezes the shared map and phase packets,
   and owns integration. This owner should continue through phase three after integrating
   phases one and two: the ingress work is broad and tightly coupled to the map, so reusing
   context is preferable to paying for another full bootstrap. Its packet must freeze the
   admission algebra and call-site inventory before code changes make the later tail rote.
2. Dispatch phase one to a fresh report-channel builder. It is security-critical, narrow,
   and has a crisp owned-capability boundary and finite negative-test matrix; the full-kit
   reload is worth buying an uncompressed local implementation.
3. Dispatch phase two concurrently to a fresh aggregate-vouch builder after phase zero.
   It is likewise critical and bounded. The two builders work in disjoint files and may
   not change shared public interfaces frozen by the continuity owner.
4. The continuity owner integrates and verifies phases one and two before beginning the
   phase-three ingress packet and implementation. Do not parallelize bounded reading,
   attribution, admission refusal, and replay migration: their conversion graph is the
   security boundary, not separable plumbing.
5. Dispatch phase four's sink inventory, encoder API, and renderer migration to one fresh
   sink builder after ingress lands. This work is broad rather than cheaply fan-out-able;
   keep it together behind one frozen sink matrix and tolerate a recoverable compression.
6. After the sink matrix freezes, the continuity owner may split whylog store hardening to
   one additional fresh builder only if its files, constructor API, and tests are disjoint
   from the remaining renderer migration. This is a dynamic exception: filesystem
   ownership is critical and can become bounded, so an extra full-kit load is justified;
   if the boundary is not crisp, the sink builder keeps it serially.
7. Phase five belongs to the continuity owner after all implementation merges. It is an
   integrated authority-surface pass and must not be fragmented into per-type chores.
8. A fresh opaque reviewer checks the exact merged revision against every governed surface,
   followed by a separate ordinary correctness review and the full gates. Fresh review is
   critical and bounded, so its duplicate context cost is deliberate.

Every fresh builder receives the full kit plus exactly one frozen phase packet. Builders
must route authority widening through `sinv-authority-map`; a packet cannot authorize them
to restate, reinterpret, or relax it. Do not create a new builder merely because a task is
easy to describe: absent both criticality and a frozen bounded finish, continuity is worth
more than parallelism.

## explicit-deferrals-and-reentry

Do not build these in this lane:

- reactive cancellation/generation state machine — re-enter before reactive planning;
- saved-plan schema/version compatibility — re-enter before separated approval/apply;
- cross-host/fleet cache aggregation — re-enter before any fact crosses host scope;
- oracle publisher trust, acquisition, registry, or authority-diff UX — re-enter before
  community distribution;
- remote attestation — explicitly outside the current product promise;
- a complete public bounded-observables/security contract — publication gate, requiring
  human product judgment;
- new privileged context-entry semantics — re-enter before real execution and bind exact
  target/user/namespace/cwd/environment/credential scope;
- generic secret detection/redaction — rejected as an untestable promise for arbitrary
  shell output.

The immediate code should make each re-entry visible by lacking a constructor rather than
by carrying a comment beside a permissive `String` or `bool`.

## completion-criteria

The security refresh is complete only when all of the following are true:

- no emitted report-enabled probe performs pathname truncate/open/read/remove plumbing;
- every erased mutating establish on every current plan shape consumes an identity-matched
  reached verdict vouch;
- no unbounded result/whylog read precedes a limit, and integrity refusal cannot produce
  mutation authority;
- controller attribution cannot be supplied or widened by payload;
- every current hostile/source-derived sink routes through a destination-specific encoder;
- hostility, sensitivity, recognition, attribution, and retention are not collapsed into
  one trust/sanitize flag;
- fixture identities and legacy parsers cannot type-check at future production re-entry
  boundaries;
- receipt/whylog/narration changes cannot change executable bytes;
- all positive, negative, compile-fail, DST/property, e2e, syntax, lint, dependency, and
  typo gates pass on the exact reviewed revision;
- the opaque accrual review reports no weakened governed invariant.

Do not declare completion on the basis of added checks alone. The central criterion is that
ordinary local code no longer has a convenient type-correct route from hostile bytes or
mere observation to broader authority.

## security-name-minting-reasoning

When this build needs a name not already specified, reason in this order:

1. `name-the-security-distinction` — identify the one distinction the type must keep a
   local builder from forgetting: authority species, epistemic source, scope, ownership,
   lifecycle, grammar position, or a deliberately weakened boundary. Keep that word even
   when it makes the name longer.
2. `make-the-module-carry-context` — let a narrow module path supply routine nouns
   (`report`, `ingress`, `records`, `resolver`, `whylog`), but assume imports and compiler
   errors may erase that context. Retain the distinguishing noun when the bare name would
   become vague or falsely reassuring.
3. `name-only-enforced-properties` — adjectives such as `Owned`, `Reached`, `Bounded`,
   `Exclusive`, `Canonical`, or `Verified` are valid only when private construction makes
   the property unavoidable. Never use an impressive name to compensate for a permissive
   field, constructor, conversion, or fallback.
4. `expose-the-consequence` — authority-bearing names should state the planner action they
   license. Prefer an alarming but accurate name such as mutation erasure over a generic
   optimization or replacement name when the consequence is unequal.
5. `preserve-epistemic-species` — use `Observation` for measured/reported input, `Vouch`
   for authored judgment, `Proof` for a private proposition required by a mint, `License`
   for authority to perform one action, and `Receipt` for post-mint narration that cannot
   return to the decision plane. Do not use generic `Result`, `Witness`, or `Evidence`
   where one of these distinctions governs behavior.
6. `keep-universal-words-visible` — retain `All`/`Every` when non-emptiness, exact
   cardinality, ordering, or identity matching is the property. A collection-shaped name
   can make a partial set look valid.
7. `separate-orthogonal-axes` — compose types instead of minting compound claims:
   hostility, recognition, attribution, sensitivity, retention, encoding, freshness, and
   semantic authority are independent. No `Safe`, `Trusted`, or `Sanitized` umbrella.
8. `name-the-exact-sink` — encoded values name their destination grammar and position;
   terminal text, record fields, JSON strings, paths, and POSIX-shell words are not
   interchangeable. Prefer no type or constructor over a falsely universal one.
9. `reserve-strong-conclusions` — keep names such as `DecisionId`, `Approved`,
   `Canonical`, and `Verified` unavailable until every promised binding/check exists.
   Earlier states name only what they actually bind or establish.
10. `test-the-name-in-errors` — read the proposed bare import, constructor call, enum
    match, and compiler failure as a hurried local builder would. Choose the shortest name
    that still makes the unsafe substitution, widening, or cross-boundary reuse look wrong.

If two names remain plausible, prefer structural separation over vocabulary debate. Mint
distinct private types first, then shorten names only where module context and impossible
conversions preserve the distinction without narration.
