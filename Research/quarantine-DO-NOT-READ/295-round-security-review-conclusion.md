# 295 — Round 29 security-review conclusion

## Answer

+SURE Dorc should continue, but two present spike mechanisms violate its own security/correctness
boundaries and should be held before further reliance on those paths:

1. `293:finding-report-scratch-can-clobber-host-files` — report-draining probes truncate a
   predictable remote temporary pathname through ordinary shell redirection, permitting a hostile
   host to redirect a nominally observational probe into clobbering any file writable by the SSH
   principal.
2. `293:finding-aggregate-elisions-bypass-vouch-tier` — member-loop and inline-call aggregate mints
   can erase mutating work from converged observations without consuming the oracle-authored vouch
   required by the ordinary establish path.

+SURE Neither finding is product-fatal. Both have narrow, testable fixes that reinforce Dorc's
existing architecture: use an owned, exclusive report channel rather than a shared pathname; and
make aggregate mutation-elision require a private non-empty proof containing a reached vouch for
every establish member/body site.

+SURE The larger release boundary is also coherent: treat every managed-host byte as hostile
controller input; bind every approval to one immutable decision object; isolate host/attempt/
generation facts; revoke superseded-generation authority independently of remote quiescence; treat
oracles as executable authority; and describe whylogs/plans as sensitive rather than promising
generic redaction [A-redhat-ansible-fact-injection-2021]
[A-hashicorp-terraform-plan-2026] [A-google-operation-cancellation-2026]
[A-github-actions-secure-use-2026].

## Ranked mechanism findings

### 29:rank-one-report-scratch-clobber

+SURE Present and directly effectful. `Probe::record_scaffold_draining` constructs
`${TMPDIR:-/tmp}/dorc-drep.<nonce>.<site>` and truncates it before and after invocation. The spike's
fixed nonce makes prepositioning trivial; randomness alone would not close pathname races. This is
the highest-priority hold because it converts probe scaffolding—not faulty oracle semantics—into a
write primitive on the managed host. Full preconditions, rebuttal, and exclusion check are in
`293:finding-report-scratch-can-clobber-host-files`.

Required export after human adjudication:

- invariant: controller-owned probe plumbing must not open, truncate, or remove a host pathname it
  did not exclusively create and retain ownership of;
- type/design obligation: report capture is an owned per-attempt resource/channel, not a `String`
  pathname reconstructed for later operations;
- tests: inert symlink, collision, hostile `TMPDIR`, replacement-race model, root/non-root, and
  cleanup-failure cases; every target remains byte-identical and unsafe setup refuses before oracle
  execution.

### 29:rank-two-aggregate-vouch-bypass

+SURE Present and capable of under-execution. The normal `EstablishAmbient` replacement consumes
`ByVouch<VerdictVouch>`; `prove_members_replaceable` and `prove_inline_replaceable` do not accept one
and explicitly record no vouch locus. Convergence, self-reach, and observable-consumption gates
establish freshness/control-flow facts; they do not establish the authored semantic judgment that a
mutator may be erased. Full analysis is in `293:finding-aggregate-elisions-bypass-vouch-tier`.

Required export after human adjudication:

- invariant: every erased mutating establish, including establishes hidden in aggregate plan nodes,
  consumes a reached oracle verdict vouch; observation alone can reproduce reads but cannot erase a
  mutation;
- type obligation: a private non-empty aggregate license contains one vouch proof per establish and
  cannot be constructed from mismatched cardinalities;
- tests: all-converged/unvouched refuses; one missing/declined/dynamic member refuses the whole
  aggregate; all-vouched licenses; query-only sites do not acquire a fake mutation-vouch burden.

### 29:rank-three-host-ingestion-and-display

+SURE Result stdin/files and whylog replay are read wholly into unbounded `String`s before parsing;
accepted record count and most free fields are unbounded. +SURE host-derived derived coordinates,
canonical values, and reached entities enter the interner without the report lane's control-byte
scrub and can reach human-facing coordinate rendering. External data that reaches logs or automated
consumers can forge interpretation or reach executable downstream tooling; closed validation and
sink-specific encoding are standard mitigations [A-mitre-log-neutralization-2025].

Hold before a real remote transport: aggregate byte, line, record-count, and field bounds; typed
overflow refusal; centralized bounded terminal rendering for every host-derived lane; raw forensic
bytes separated from display. This need not block the current width-one fixture harness if the real
edge cannot accidentally reuse its unbounded reader.

### 29:rank-four-attempt-and-decision-identity

+SURE Fixed nonce/host/attempt values and FNV-1a-64 are explicitly scoped spike drift detectors,
not adversarial identity. They are acceptable only behind a production fence. A real edge must mint
cryptographically unpredictable attempt identity and use collision-resistant content identity over
the exact ordered source set and execution context. Saved approval must bind exact executable bytes,
not a later regeneration; world freshness remains a separate guard/observation question
[A-hashicorp-terraform-plan-2026].

Hold before concurrency, hostile remote transport, saved approval, or publication—not necessarily
before continued deterministic spike work.

### 29:rank-five-sensitive-whylog-hardening

+SURE Whylogs contain argv, paths, raw host output, predicted decisions, and identity metadata. They
are currently opt-in and decision-inert, which limits integrity impact, but direct predictable writes
with ambient permissions are not an adequate confidentiality/durability contract. The writer cap
does not bound replay of a pre-existing oversized file. Automatic secret redaction is not generally
reliable for transformed or structured shell data [A-github-actions-secure-use-2026].

Hold before whylogs become default, receipts become audit evidence, or third parties use them:
exclusive restrictive creation, atomic replacement, bounded reads, trusted-directory rules,
per-sink rendering, visible persistence failure, and an explicit “may contain secrets” contract.

### 29:rank-six-future-reactive-authority

+SURE Cross-host planning, saved apply execution, and reactive cancellation are not implemented, so
claiming present vulnerabilities would be fiction. Their pre-implementation laws are nevertheless
clear: host/generation attribution comes from controller transport context; supersession immediately
revokes decision authority; late records are diagnostic-only; execution quiescence is observed
separately because cancellation is best effort [A-google-operation-cancellation-2026]. Capability
must be controller-owned immutable attempt context with unknown/degraded as the absence default.

## Product conclusions

+SURE The product must state four hard truths from `294`:

- a compromised host can lie about its own state; Dorc's security job is to keep those lies from
  compromising the controller or peers, not to promise unattained remote attestation;
- oracles are executable, security-bearing authority, not passive metadata; hashes and full Git SHAs
  establish identity, not semantic trust [A-hashicorp-terraform-lock-2026]
  [A-hashicorp-plugin-signatures-2026] [A-github-actions-secure-use-2026];
- “probe” cannot honestly mean “zero side effects”; the defensible promise is no intentional managed-
  resource mutation plus explicit, bounded, attributed controller/oracle residue;
- best effort changes meaning at the mutation boundary: analysis uncertainty normally preserves the
  authored operation, while lost attempt/execution integrity withholds further mutation.

+SURE A fifth public truth is that generic secret scrubbing is false for arbitrary shell output.
Minimization, sensitivity classification, bounded retention, access control, and explicit redaction
limits are stronger and testable [A-github-actions-secure-use-2026].

~SUSPECT The most dangerous long-term incentive is measuring success by elision rate. It rewards
broader vouches and footprints even when the evidence quality worsens. Product/engineering metrics
should pair optimization counts with guard fall-through, walls, declined/unverifiable claims, and
safety regressions.

## Comparison against the prospective 292 synthesis

`292` was written in an empty window before the mechanism/product audit and intentionally withheld
from this conductor. +SURE Its agreement with `291`/`293`/`294` is not independent corroboration—it
partly synthesized the same round-10 and `291` evidence—but the comparison is useful for coverage.
Most of its concrete controls already appear above: hostile-byte handling, exact decision identity,
identity-versus-trust, cancellation-versus-quiescence, redaction limits, context-entry axes,
cross-host isolation, and truthful Fable export. The following angles materially extend `295`.

### 295:add-authority-map-per-knowledge-source

+SURE `292` supplies the clearest general review abstraction: every optimization is authority to
prevent user-written code from running. Data-flow reachability answers what *can* influence a
decision; an authority map answers what *may* influence it. These are not equivalent.

For every observation, vouch, footprint, resolver result, cache entry, generation marker, and admin
knob, the design should record:

- supplier and interpreter;
- permitted decision species (display, guard, local replacement, downstream survival, cross-host
  reuse);
- host/program/generation scope;
- expiry and revocation rule; and
- who bears the consequence of a false claim.

This sharpens the export/review process: changing a source's authority map is a security-design event
even if its Rust type and data flow are unchanged. The aggregate-vouch bug is a concrete example:
observations reached the aggregate mint, but their permitted authority should not have included
mutation-erasure without authored interpretation.

### 295:add-bounded-observables-contract

+SURE The structured review identified probe residue and observable-consumption gates, but did not
state the full product limit sharply enough. Reproducing rc/stdout dependencies does not prove that
replacement with `true` preserves traps, signal timing, inherited file descriptors, shell state,
locks, audit trails, credential lifetimes, concurrent observers, or unmodeled world effects.

Dorc therefore needs an explicit bounded-observables contract: which shell- and world-observables it
claims to preserve, which are oracle-vouched risks, and which force guard/run. “Observationally
equivalent” must never be left unqualified. Types can enforce the chosen boundary; they cannot prove
that the boundary is complete. This becomes a publication gate and a review input for every new
observable channel, rather than an implementation task to solve arbitrary shell equivalence.

### 295:add-mint-time-authority-witness

~SUSPECT Whylogs and later explanations should preserve an authority derivation at license mint time,
not reconstruct it afterward from paths, logs, or current oracle source. The minimum witness links
the controller-attributed observation, exact oracle judgment and source identity, admin policy,
host/generation scope, resulting license, and revocation identity. This is richer than display-only
provenance but must remain on the exempt/receipt plane so it cannot feed its own decision.

This does not require a general graph database. Private license constructors can emit a typed compact
witness beside the license, and erasability tests can prove that changing receipt narration alone
does not change executable bytes.

### 295:add-oracle-upgrade-authority-diff

+SURE Exact oracle identity and publisher trust are necessary but insufficient when an upgrade
changes what existing scripts are authorized not to do. Oracle review should include an *authority
diff*: new or widened vouches, footprints, carry rules, context-entry forms, resolver equivalences,
and decline behavior. A version bump with identical public function names may still expand
under-execution authority substantially.

Before community distribution, update UX should distinguish byte/version changes from authority-map
changes and require proportionate review. Composition deserves the same treatment: two individually
acceptable claims may widen one another through survival or wrapper entry.

### 295:add-hostility-and-sensitivity-axes

+SURE Hostility and confidentiality are orthogonal. A value may be hostile but public, sensitive but
trusted, both, or neither. A universal `sanitize()` or `SecretString` cannot represent the needed
rules. The architecture should distinguish raw/bounded/recognized/controller-attributed evidence
from sink encodings such as terminal text, and independently carry sensitivity/retention policy.

This refines `29:rank-three-host-ingestion-and-display` and
`29:rank-five-sensitive-whylog-hardening`: output encoding does not declassify a secret, while
redaction does not make attacker-controlled structure safe.

### 295:add-context-siting-and-confused-deputy

~SUSPECT The context-entry audit covered capability, admin dial, author tolerance, and structural
entry forms, but correct *siting* remains a separate claim. `sudo -n` or an injected `Capability`
does not establish target identity, environment integrity, namespace selection, credential scope, or
that the wrapper is not acting as a confused deputy. The future entry license should therefore bind
the resolved target/context identity and environment policy separately from mechanical capability.
Unknown siting must wall; it must not be inferred from successful command execution.

### 295:add-boundary-erosion-review-trigger

+SURE Typed boundaries are an alarm system, not self-defending policy. Compatibility modes, generic
claim conversions, warning downgrades, permissive defaults, inference fallbacks, and “temporary”
escape constructors can redefine a witness while leaving downstream consumers apparently type-safe.

Add a review trigger covering changes to private authority mints, required witnesses, scope
conversions, legacy-tolerance entry points, and the meaning of evidence types. Review ownership must
include callers capable of violating the invariant, matching the Rust governance lesson below. This
is more precise than routing only visibly security-named files.

### 295:comparison-ledger-adjustments

The comparison adds four re-entry gates without changing the two immediate holds:

1. define the bounded-observables public contract before publication;
2. design mint-time authority witnesses before whylogs/receipts become audit evidence;
3. add oracle authority-diff review before community upgrades/distribution;
4. bind context siting/environment identity before real privileged context entry.

~SUSPECT No other `292` recommendation changes the ranking. Its strongest enduring conductor rule is
worth adopting verbatim in substance: never review what Dorc knows without also reviewing what that
knowledge is permitted to prevent.

## Security controls that survive many Rust authors

+SURE Rust's type system is useful here only when the architecture concentrates authority. Mature
large-project practice layers denied-by-default unsafe/exception rules, point-of-use contracts,
review ownership, exact-merge testing, merge queues, and regression-bearing reverts. Ferrocene's
safety guidance makes the review unit include safe callers able to violate an unsafe invariant;
Linux prefers self-expiring lint expectations over blanket allowances; Cargo Vet keeps typed named
review evidence in-tree and makes missing evidence a CI refusal
[B-ferrocene-development-process-2026] [B-ferrocene-handling-unsafety-2026]
[A-linux-rust-coding-guidelines-2026] [A-cargo-vet-workflow-2026]
[A-cargo-vet-audit-criteria-2026].

For Dorc this means private proof constructors, tiny named exception types, owners for those modules
and all callers capable of violating their invariants, exact merged-revision tests, and exported
test/type obligations. It does not mean expecting uneven contributors to reconstruct quarantined
threat reasoning or claiming Rust proves external-command semantics.

## Hold-now and defer ledger

### 29:ledger-hold-now-properties

1. Disable or repair the report scratch lane before treating report-enabled probes as safe.
2. Close aggregate mutation-elision's vouch bypass before relying on member-loop/inline-call elision.
3. Do not create new host-derived display sinks outside a centralized sanitizer.
4. Preserve private claim/license tiers and the test-only status of legacy headerless records.
5. Keep the current FNV/fixed identity explicitly fenced as spike-only.

### 29:ledger-defer-with-reentry-conditions

1. Input streaming/bounds: re-enter before real remote transport or untrusted results files.
2. Cryptographic identity and host/generation types: re-enter before concurrency, retries, saved
   approval, or multi-host caches.
3. Whylog filesystem/privacy hardening: re-enter before default-on persistence, audit claims, or
   third-party use.
4. Saved-plan binding: re-enter before any separated approve/apply workflow.
5. Reactive cancellation/finality: re-enter before implementing reactive generations.
6. Oracle distribution/provenance UX: re-enter before community acquisition or publication; a
   registry itself remains unnecessary.
7. Public security contract: release blocker before publication or third-party use.

## Accepted risks for the present spike

- +SURE A root-compromised managed host can lie about its own state. No remote attestation is claimed.
- +SURE Oracle source is trusted executable input; arbitrary-oracle sandboxing is not an MVP goal.
- +SURE Fixed identifiers and FNV digests may remain only in deterministic width-one harness code
  with an explicit production substitution point.
- +SURE Whylogs may remain opt-in and best-effort while decision-inert, provided their sensitivity is
  not misrepresented and they are not promoted to audit evidence.
- ~SUSPECT `AnyProbe` can remain a deliberate admin override if authority reuse is conspicuous and
  never silently inherited by automation.

## Competing options rejected

- “Sandbox arbitrary shell oracles now”: incompatible with the product's ordinary-sh execution model
  and unnecessary if trust/provenance and license boundaries are honest.
- “Build an oracle registry now”: identity/provenance manifests and local review are sufficient for
  pre-MVP; a registry would not solve semantic trust.
- “Treat every malformed probe result as run”: safe against under-execution but unsafe as a universal
  execution-integrity response; corruption after lost authority must withhold mutation.
- “Randomize the report filename”: reduces guessing but does not close symlink/replacement races or
  ownership loss.
- “Scrub secrets from all output”: an untestable promise for arbitrary transformed shell data.
- “Make cancellation synchronous”: distributed execution cannot guarantee it; revoke authority and
  observe quiescence separately.
- “Run a broad adversarial crosscheck now”: the two severe present findings are direct source-level
  invariant contradictions, while remaining uncertainty is explicitly future-scoped. A crosscheck
  would add breadth but is unlikely to change the immediate holds; reserve it for a later contested
  release decision.

## Export process

+SURE Quarantine is viable only if adjudicated conclusions leave it as truthful ordinary engineering
constraints. NIST SSDF supports maintained security requirements, provenance, tracked risks/design
decisions, and risk-scaled continuous improvement rather than a ritual checklist
[A-nist-secure-development-framework-2026].

For each accepted finding, export exactly:

1. a locally understandable implementation-neutral invariant;
2. the narrow type/API constraint that makes violation difficult;
3. negative and positive deterministic tests, including the other phase/user/reliability cells;
4. a private mapping back to the quarantined finding;
5. a re-entry trigger if the property is deferred.

Do not export attacker narratives, inaccessible rationale as ritual, or false cover stories. If an
ordinary implementation changes a security-bearing invariant, return it to this review lane. Before
publication, replace the temporary omission with an honest public contract for host trust, oracle
authority, artifact sensitivity, probe residue, and redaction limits.

## Quarantine list

- Full shell-quoting audit of every source-derived literal embedded in emitted single-quoted
  `printf` formats; current nonce/host/book values are fixed but their future types need validation.
- Complete sink inventory for host-derived canonical/reach/derivation values; the existence of the
  gap is established, but every display route has not been enumerated.
- Atomic cross-platform whylog design, including Windows reparse-point behavior.
- Concrete saved-plan identity schema and controller-version compatibility policy.
- Cross-host aggregation semantics beyond default isolation.
- Reactive generation/cancellation state machine once implementation begins.
- Product UX for oracle provenance and conspicuous `AnyProbe` consent.

+SURE The round's most important result is not a general warning: it is two immediate narrow holds
plus a small release-boundary contract. Repair the report channel and restore the vouch tier across
aggregate elisions; preserve the remaining identity, hostile-input, and delegation obligations as
typed re-entry gates rather than prematurely building a security subsystem.
