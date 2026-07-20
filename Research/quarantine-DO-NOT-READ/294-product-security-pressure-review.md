# 294 — Product security-pressure review

Status: conductor working review, 2026-07-19. This answers the product question separately from
`293`'s mechanism audit: where Dorc's promises, defaults, and incentives can repeatedly pull users
toward unsafe behavior even after individual bugs are fixed.

## 294:hard-truth-compromised-host-can-lie

+SURE Dorc cannot guarantee convergence or correct elision *for* a host whose privileged state and
probe execution are controlled by an adversary. The host is the measurement source. Framing,
freshness, and closed parsing can stop its bytes from compromising the controller or contaminating
other hosts; they cannot make the host report its own world truthfully.

The product boundary should say this directly. Dorc can promise controller and peer-host isolation,
bounded ingestion, attribution, and refusal of stale/malformed evidence. It cannot promise remote
attestation it does not possess. The product-preserving rebuttal is that managed hosts are normally
trusted targets. Correct—but the hostile-host threat model is still valuable because one compromised
node must not become fleet-scale controller authority.

### 294:release-state-host-trust-boundary

Before third-party use, documentation must distinguish “host facts are untrusted controller input”
from “Dorc can discover lies about the host's own state.” Cross-host caches or aggregation remain
prohibited until they carry typed host/generation provenance and a reviewed aggregation rule.

## 294:hard-truth-oracles-are-executable-authority

+SURE An oracle is not a passive schema or harmless provider description. It contributes shell bytes
executed during probe/apply and authored judgments that can license mutation-elision, survival, and
context entry. Exact hashes establish which bytes participated, not whether their publisher or
semantics are trustworthy [A-hashicorp-terraform-lock-2026]
[A-hashicorp-plugin-signatures-2026].

Calling installation “adding metadata” or presenting community oracles as ordinary configuration
would train admins to skip the needed trust judgment. The useful product remains possible without a
registry: retain an exact ordered oracle manifest in decision identity, show provenance and local
modification plainly, route security-bearing changes to owners, and make unsigned/unreviewed status
visible without pretending signatures prove semantics.

The engineer/admin split is load-bearing. Engineers need narrow typed authoring surfaces and tests
that state what their vouches license. Admins need a short authority summary and controllable policy,
not the oracle dialect's internal proof story.

## 294:hard-truth-probes-are-not-side-effect-free

+SURE “read-only probe” cannot truthfully mean “causes no state change.” Tool invocation can update
access times, caches, audit logs, authentication timestamps, process state, or daemon state; context
entry explicitly accepts some residue. The current report scratch implementation additionally shows
how controller scaffolding itself can become mutation.

The defensible promise is narrower: probe code must not intentionally mutate the managed resource;
every controller-owned non-pure action is explicitly designed, bounded, attributed, and tested;
oracle-authored tolerated residue is visible; and unknown probe failure revokes mutation authority.
This preserves useful probing while avoiding a categorical claim the architecture cannot keep.

The other-reliability cell matters: reliable oracle authors may still overlook tool side effects;
unreliable authors may deliberately disguise them. Structural sh analysis can narrow risk but cannot
prove external-command purity.

## 294:hard-truth-best-effort-splits-at-mutation-boundary

+SURE Dorc's best-effort posture is beneficial before network effects: accumulate independent
analysis errors, wall uncertain optimizations, and produce as much useful diagnosis as possible. It
becomes unsafe if carried unchanged across the apply boundary. After transport corruption, lost
generation identity, uncertain context entry, or an unrecoverable remote failure, “unknown means run”
can turn uncertainty into continued mutation.

The product rule should be two-part:

- analysis uncertainty about whether an authored operation is already converged usually preserves
  the operation (`run` or a live guard);
- execution-integrity uncertainty about whether Dorc still owns a valid attempt must withhold further
  mutation for that host/generation.

This is not a contradiction. The first protects against under-execution; the second protects against
acting under lost authority. User-facing language that collapses both into “fail safe” will obscure
the distinction.

## 294:hard-truth-approval-is-not-freshness

+SURE A human-readable plan, executable shell, whylog replay, and later apply must identify one exact
decision object. Approval must bind exact executable bytes plus book bytes, ordered oracle set,
analysis-relevant knobs, controller version, host, capability context, and generation. Regenerating a
similar script later is not applying the approved plan [A-hashicorp-terraform-plan-2026].

That identity does not prove the world stayed still. Fresh guards and observations answer freshness;
the digest answers authorship/approval. Marketing saved plans as “safe to run later” without this
distinction would create dangerous confidence. Saved artifacts and whylogs must be treated as
sensitive by default because they can contain paths, argv, values, and host output.

## 294:hard-truth-generic-secret-scrubbing-is-false

+SURE Arbitrary shell output has no generally recognizable secret type. Transformations, structured
values, errors, and partial encodings defeat generic redaction [A-github-actions-secure-use-2026].
Dorc should not promise “secrets are scrubbed.”

The useful promise is minimization and containment: do not capture opaque output without a concrete
need; cap it before allocation; classify it as sensitive; keep raw evidence separate from display;
encode every sink; establish restrictive retention and permissions; and document that whylogs may
contain secrets. This costs some diagnostic richness but makes the guarantee testable.

## 294:pressure-defaults-and-overrides

The following defaults would predictably erode the boundaries above:

1. `294:pressure-permissive-capability-default` — defaulting an unknown real connection to `Root`
   makes absence of evidence an authority grant. The real edge must construct degraded/unknown by
   default.
2. `294:pressure-unbounded-any-probe-override` — `AnyProbe` is a legitimate admin override, but its
   name and disclosure must state that it bypasses absent oracle tolerance and may execute entry code
   with existing connection authority. It should never be silently inherited by automation.
3. `294:pressure-optimization-success-metric` — rewarding elision rate alone pressures oracle authors
   to broaden vouches and footprints. Quality metrics must also count walls, guard fall-through,
   unverifiable claims, and safety regressions; “more elision” is not monotonic success.
4. `294:pressure-quiet-best-effort-durability` — silently swallowing whylog persistence failures is
   acceptable only while whylogs are optional aids. Once receipts/audit claims depend on them,
   durability failure must become visible and the product wording must change.
5. `294:pressure-friendly-oracle-installation` — one-command oracle acquisition without immutable
   identity, provenance display, and review cues would turn the supply-chain boundary into routine
   consent fatigue.

## 294:product-preserving-controls

The smallest control set that preserves Dorc's value proposition is:

- keep optimization licenses private, typed, and denied by absence; aggregate shapes must require
  aggregate vouch evidence rather than bypassing the tier;
- derive host/attempt/generation/capability at controller-owned edges and carry them immutably;
- bind approval to exact artifacts while rechecking world freshness separately;
- isolate every host's facts and revoke superseded generation authority immediately, without claiming
  cancellation stopped execution [A-google-operation-cancellation-2026];
- treat oracle code as executable authority and whylogs/plans as sensitive artifacts;
- use bounded hostile-data parsers plus centralized per-sink rendering;
- test the exact merged revision and route security-bearing exception changes to named owners, using
  the Rust governance patterns recorded in `round29-research/turn02-2026-07-19-notes.md`.

These are mostly boundary constraints, not a request to build a registry, sandbox arbitrary shell,
or solve remote attestation before MVP.

