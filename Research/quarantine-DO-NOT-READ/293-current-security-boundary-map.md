# 293 — Current security-boundary and mechanism map

Status: conductor-owned working review, 2026-07-19. This is an independent code-and-contract
inspection derived without reading `292`. It is not a completeness claim. Findings remain
quarantined and unexported pending adjudication.

## Scope and present shape

The implemented spike is still a width-one compiler, not the eventual fleet controller. It reads
book and oracle source locally, emits probe shell, ingests a returned text record stream, derives a
plan, and emits apply shell. It does not execute apply, manage SSH, schedule multiple hosts, or
implement the reactive planner. Consequently, several round-29 threats are present only as design
obligations; inventing findings against absent machinery would obscure the live seams.

The most important present boundary is:

`book + oracle bytes -> static analysis -> emitted probe sh -> hostile host execution -> returned
records -> controller parsing/folding -> emitted apply sh`

The controller is the crown jewel. Oracle and book bytes are trusted local inputs today; returned
records are host-controlled bytes. Whylogs are sensitive local postmortem artifacts, explicitly
decision-inert. Context-entry probe code may reuse pre-existing connection authority when both the
admin policy and oracle-authored contract license it.

## Boundary inventory

### 293:map-local-source-to-shell-artifacts

- `spike/crates/cli/src/main.rs` loads books and ordered oracle files as UTF-8 strings. The same
  sources feed parsing, oracle lifting, probe construction, plan construction, and final rendering.
- Oracle source is executable shell in the shipped probe/apply artifacts, not an untrusted plugin
  sandbox. The security boundary is therefore provenance and review of oracle bytes, plus strict
  narrowing of what those bytes are allowed to license—not process isolation.
- Sealed core claim tiers and private plan-license constructors are useful capability boundaries:
  ordinary analysis facts cannot be silently upgraded into an oracle vouch. This concentrates the
  security review onto a small minting surface.

### 293:map-host-records-to-controller-decisions

- Probe output is framed by `spike/crates/plan/src/records.rs`. Header keys bind nonce, attempt,
  host, book digest, and expected site count. A sentinel terminates the attempt; stale, alien,
  torn, glued, and late records are diagnosed. Production refuses headerless streams unless the
  test-only environment escape enables legacy tolerance.
- The parser consumes `&str`; the CLI first reads the entire results file or stdin into one
  unbounded `String`. Deframing then allocates owned strings for accepted records and several
  collections can grow with attacker-chosen record count and value length.
- Malformed, missing, or refused facts generally become `Unknown`, which prevents evidence-based
  replacement/elision and leaves the authored operation in the apply artifact. This is a safe
  correctness fallback, but it is not an availability defense and it is not equivalent to aborting
  mutation after a transport-integrity failure.
- The current nonce and host are fixed spike defaults; the attempt is always one. The mechanism is
  tested with injected alternatives, but the production edge that mints fresh unpredictable
  per-attempt identity does not exist yet.
- Book and decision digests are FNV-1a-64 drift detectors. Comments explicitly scope out
  adversarial collision resistance and promise a cryptographic digest in the real edge.

### 293:map-context-entry-to-reused-authority

- `spike/crates/oracle/src/entry.rs` permits context entry only through structurally detected
  `<provider>__enter` forms. The decision combines modeled dimensions, a caller-supplied
  `Capability`, the admin's `EscalationDial`, and path-scoped oracle tolerance vouches.
- `VouchedOnly` requires author consent; `NoEscalation` prohibits entry; `AnyProbe` deliberately
  lets the admin override absent author tolerance. Missing capability, forms, or dimension
  knowledge degrades to no entry.
- The engine does not verify that an authored entry command is non-interactive or semantically
  performs the claimed shift. Authoring is treated as the vouch. This makes oracle review and
  attribution load-bearing.
- The CLI currently defaults the injected capability to `Root`. It is a declared spike posture,
  not a measured connection property. A fleet implementation must not inherit that default or
  allow a host to self-assert controller-held authority.

### 293:map-whylogs-to-sensitive-local-state

- `spike/crates/plan/src/whylog.rs` stores invocation arguments, book/oracle paths and content
  digests, host/attempt/nonce, the raw host-controlled result stream, predicted apply dispositions,
  and a decision digest. It is therefore likely to contain secrets and hostile display text.
- Replay re-reads recorded source paths, verifies content digests, re-deframes the recorded result
  stream, and re-runs the same kernel. A decision-digest mismatch refuses narration. Whylogs are
  not decision inputs to later plan/apply runs.
- Writes in `spike/crates/cli/src/main.rs` are opt-in and capped at 1 MB with five-file retention,
  but use `create_dir_all` and direct `fs::write` to predictable names. There is no atomic
  create-and-rename, restrictive permission establishment, symlink/reparse-point defense, or
  authenticated integrity. Write failures are swallowed by design.
- Replay reads the whole selected whylog before parsing; a pre-existing oversized file bypasses the
  writer's cap. The parser is total for malformed UTF-8 strings once loaded, but the file read and
  allocation have no hostile-local-file bound.

### 293:map-reactive-and-cross-host-machinery

- Cross-host fact partitioning, generation/finality, cancellation, saved-plan approval, and remote
  apply execution are not implemented in the inspected spike. The existing record frame has host
  and attempt keys, but that does not discharge future partitioning or cancellation laws.
- These remain hold-before-implementation obligations, not present-code vulnerabilities: saved
  approval must bind exact apply bytes and relevant execution context; host facts must be typed by
  host/attempt/generation; cancellation must invalidate stale authority before effects; transport
  failure after uncertainty must not silently degrade into continued mutation.

## Candidate findings

### 293:finding-bound-host-controlled-ingestion

Preconditions: a managed host, results file, or local pipe can supply arbitrarily large UTF-8 input.

Impact: controller memory exhaustion or severe CPU/allocation amplification before a safe plan is
produced. This is a controller-availability boundary crossed by hostile host bytes.

Confidence: +SURE for the missing aggregate byte/record bounds; ~SUSPECT for practical severity
until representative controller limits and SSH plumbing exist.

Cheapest safe response: introduce edge-level byte, line, record-count, and free-field limits before
or during streaming decode; make overflow a typed refused-attempt result. Keep the pure deframer
fuzzable by injecting bounded chunks or a prevalidated bounded buffer. Test boundary, one-over,
long-line, many-line, invalid-UTF-8, and post-sentinel flood cases.

Value cost: bounded diagnostic/report detail and some streaming complexity. Rebuttal: the spike is
not network-facing and authored fixtures are small. That justifies deferral to the real transport
edge, not omission from its contract.

Exclusion check: applies in probe-return and whylog-replay directions; both admin and oracle author
can encounter it; reliable oracles do not help when the host or transport is hostile. Apply output
itself is not the ingress, but an unavailable controller prevents safe apply generation.

### 293:finding-report-scratch-can-clobber-host-files

Preconditions: a probe includes an oracle verdict body with report emission enabled; the managed
host can pre-create filesystem objects in `${TMPDIR:-/tmp}` or control `TMPDIR`; and the SSH
principal can write the symlink target. The current fixed nonce makes the path
`dorc-drep.dorc.<site>` predictable before execution.

Impact: `Probe::record_scaffold_draining` in `spike/crates/plan/src/render.rs` executes
`: >"$_drep"` before and after the oracle invocation. Ordinary shell redirection follows a
prepositioned symlink, so hostile-host state can redirect a supposedly observational probe into
truncating an arbitrary file writable by the connection principal. On a root connection, that can
cross directly into root-owned host state. A collision with another live probe can also corrupt
reports and decisions.

Confidence: +SURE about the predictable direct truncation and missing exclusive creation;
~SUSPECT about exact cross-platform details, but the emitted artifact targets POSIX sh and the
attack requires only conventional symlink semantics. No exploit reproduction is necessary to
establish the unsafe primitive.

Cheapest safe response: stop using a guessable shared path. At the remote execution edge, create a
private per-attempt directory/file with an atomic exclusive primitive and restrictive mode, reject
unsafe `TMPDIR` siting, hold the opened resource rather than reopening a pathname, and remove it
without following replacements. If portable sh cannot provide the invariant, carry the report lane
over an already-owned file descriptor or controller-created channel instead. Fresh cryptographic
attempt identity is necessary but not sufficient: random names alone still leave pathname races.
Tests should use an inert temporary directory and prepositioned symlink/collision fixtures, proving
the target is unchanged and the probe refuses safely.

Value cost: portable shell-only implementation becomes harder and may require a small trusted
remote helper or extra file descriptor. Strongest product-preserving rebuttal: report emission is
optional and the managed host is already being automated. That fails: probe safety is a stated
product boundary, and a compromised host must not be able to turn read-oriented collection into a
controller-authorized clobber primitive.

Exclusion check: reverse propagation makes the report bytes hostile on return; probe performs the
clobber while apply is not yet licensed; both admin and oracle author can trigger the path without
seeing the generated scratch implementation; reliable oracle code does not remove hostile host
filesystem state. This is hold-now for the report lane, not a future fleet-only concern.

### 293:finding-mint-cryptographic-attempt-identity

Preconditions: the real controller adopts the spike's fixed nonce/host/attempt or FNV identity, or
uses similarly forgeable identifiers across an adversarial return channel.

Impact: replay/cross-attempt acceptance or deliberate digest collision could attach observations to
the wrong analyzed input. The current fixed values also make concurrent attempts indistinguishable.

Confidence: +SURE this is unsafe in the eventual hostile-host model; +SURE the source labels it a
spike scope-cut rather than claiming production readiness.

Cheapest safe response: make the transport edge mint a cryptographically unpredictable per-attempt
nonce; use a collision-resistant digest over exact book/oracle source-set identity; type host,
attempt, and generation rather than passing raw strings; keep deterministic kernels by DI. Tests
must attempt wrong host, attempt, generation, source order, duplicate source, and stale nonce.

Value cost: a crypto dependency at the I/O edge and nondeterminism that must be injected in DST.
Rebuttal: mismatch detection rather than hostile collision resistance is enough for the width-one
spike. Accepted only while the limitation is fenced from a production edge.

### 293:finding-harden-sensitive-whylog-files

Preconditions: whylogs are enabled in a directory visible to another local principal, or the
directory/file namespace can be raced or pre-populated.

Impact: argv, paths, raw host output, and operational metadata may be disclosed; predictable direct
writes can follow attacker-controlled filesystem objects or expose torn files; forged durables can
mislead postmortem narration even though they cannot license apply decisions.

Confidence: +SURE about absent explicit permissions and atomicity; ~SUSPECT about cross-platform
exploitability because directory ownership and deployment siting are not yet designed.

Cheapest safe response: define a privacy contract, create files exclusively with restrictive
permissions, write-then-atomically-rename within a trusted directory, reject symlink/reparse-point
surprises where supported, bound reads independently of writer behavior, sanitize every rendering
sink, and visibly report persistence failure without changing plan bytes.

Value cost: platform-specific filesystem code and fewer silently best-effort writes. Rebuttal:
whylogs are opt-in and decision-inert. That reduces integrity severity, not confidentiality risk.

Exclusion check: the admin chooses siting but oracle authors may unknowingly cause sensitive output;
reliable oracles can still expose secrets; replay is the reverse ingress; apply receipts will make
the durable richer and raise the risk.

### 293:finding-sanitize-all-host-derived-display

Preconditions: a hostile host returns a syntactically accepted derived coordinate, resolver
canonical value, or dynamically reached entity containing terminal-control or deceptive Unicode
characters; a later why/diagnostic surface renders that interned value to a terminal.

Impact: terminal escape execution, forged-looking diagnostics, hidden/reordered text, or misleading
postmortem output at the controller. The report lane has a local control-character scrub and
200-byte cap, but the other host-derived free-content lanes do not pass through that sanitizer.

Confidence: +SURE the unsanitized strings enter the shared interner and can reach `render_coord`;
~SUSPECT about the complete set of reachable display sinks until the next narrow sink inventory.

Cheapest safe response: preserve raw bytes only in typed evidence storage, and require every
human-facing sink to use one centralized bounded renderer that escapes C0/C1 controls, ESC, bidi
controls, newlines, and other format-confusing characters without changing identity comparisons.
Test each host-derived lane against terminal escapes, bidi overrides, very long values, and
diagnostic-prefix forgery.

Value cost: escaped output is less pretty and raw forensic bytes require a deliberately separate
surface. Rebuttal: only report text is intended for display. Current why attribution renders entity
coordinates too, so the boundary is broader in practice.

Exclusion check: applies to reverse host-to-controller propagation in probe and whylog replay; apply
receipts will add more sinks; both users consume diagnostics; reliable oracles cannot sanitize a
compromised command's output. This revalidates a round-10 hazard against newly added record lanes.

### 293:finding-freeze-context-authority-at-edge

Preconditions: future connection code derives `Capability` from mutable/remote claims, retains the
spike's `Root` default, or lets authority change between plan and probe execution.

Impact: oracle probe code can execute through a context-entry form with more authority than the
admin intended. Because `AnyProbe` is a deliberate override and entry semantics are authored, a
wrong capability value widens the blast radius substantially.

Confidence: ~SUSPECT as a future-integration hazard, not a present exploit: the current CLI accepts
an explicit simulation flag and does not connect to hosts.

Cheapest safe response: remove permissive defaults at the real edge; mint capability only from
controller-owned connection setup; bind it into immutable attempt context and saved identity; make
unknown/degraded the construction default; preserve the two-axis admin/author disclosure.

Value cost: more probes wall when capability cannot be established. Rebuttal: users want useful
root-connected probes. The response preserves them when the controller actually owns that
authority; it only rejects assumption as evidence.

### 293:finding-aggregate-elisions-bypass-vouch-tier

Preconditions: an in-loop `EstablishMembers` site or an inlined function call contains mutating
establishes; all corresponding hostile-host observations report `Converged`; the existing
self-reach/top/consumption gates pass; but no reached oracle verdict vouches that convergence means
the mutation may be erased.

Impact: `ReplaceLicense::prove_members_replaceable` and
`ReplaceLicense::prove_inline_replaceable` can replace the mutating body/call with `true` using
observations alone. Unlike the ordinary `EstablishAmbient` mint, neither function accepts or
consumes `ByVouch<VerdictVouch>`; their derivations explicitly record no vouch locus. This violates
the elide-weld boundary stated elsewhere in the same module: a measurement may reproduce a read but
must not license mutation-elision. A wrong, unreliable, or malicious oracle result can therefore
cause under-execution specifically through aggregate shapes.

Confidence: +SURE about the type-level bypass and emitted `Replace`; ~SUSPECT only about whether an
older human ruling intentionally exempted these two shapes. No such discharge appears in the
current source contract, and the comments call them mutators while simultaneously saying no vouch
is consumed.

Cheapest safe response: require an aggregate proof object containing a reached
`ByVouch<VerdictVouch>` for every establish member/body site, plus the existing all-converged and
consumption evidence. Make the aggregate constructor private and non-empty. Query-only body sites
remain fact-tier substitutions and need no mutation vouch. Absence, decline, dynamic argv, or a
cardinality mismatch must run the whole aggregate. Add negative tests for all-converged-but-
unvouched members/calls and one-missing-vouch among many, plus positive all-vouched tests.

Value cost: some loop/call elisions disappear until the vouch builder learns their per-member/body
argv structure. Rebuttal: all members converged and self-reach makes elision a fixed point. That
establishes freshness and control-flow consistency, not the semantic claim that "converged" permits
erasing the mutator and all its observables; the project already assigned that judgment to oracle
authorship.

Exclusion check: reverse propagation is precisely the hostile observation feeding the bypass;
probe evidence causes apply omission; both oracle author and admin are affected; unreliable oracles
make the risk concrete, while reliable oracles still require explicit authorship for auditability.
This is a present hold-now correctness/security invariant, not reactive-planner debt.

## Rust delegation lesson

The Rust-specific prior-art lane supports a process design, not a claim that Rust proves Dorc safe.
Large projects concentrate unsafe or exceptional authority behind denied-by-default build rules and
narrow APIs; require point-of-use safety contracts; route review to owners; test the exact merged
revision; use merge queues; and prefer self-expiring exceptions such as lint `expect` over permanent
blanket allowances. Ferrocene additionally treats the review unit as all code capable of violating
the invariant, including safe clients of unsafe internals. Cargo Vet demonstrates typed, named,
in-tree review evidence with CI refusal when evidence is absent. See the source-graded records in
`round29-research/turn02-2026-07-19-notes.md`.

For Dorc, the likely export form is: sealed security-bearing constructors; tiny named exception
types; ownership over their defining modules and callers; CI source scans only as supplements to
type boundaries; exact-merge testing; and test obligations attached to each exported invariant.
Uneven contributor capability is handled by making exceptional authority conspicuous and routed,
not by expecting every contributor to reconstruct quarantined threat reasoning.

## Next narrow reviews

1. `293:next-audit-rendered-shell-quoting` — trace every attacker- or path-derived value baked into
   single-quoted emitted shell, beginning with record nonce/host/book fields and entry heads.
2. `293:next-audit-output-sanitization` — enumerate stderr/stdout/whylog rendering sinks and prove
   hostile host/oracle text cannot inject terminal control sequences or forge diagnostics.
3. `293:next-audit-license-mint-surface` — enumerate every private replace/guard/survival mint and
   prove only correctly typed oracle evidence reaches it.
4. `293:next-specify-future-identity-plane` — define exact bytes and execution context bound by
   saved approval, host/attempt/generation identity, and cancellation finality before those features
   are implemented.
