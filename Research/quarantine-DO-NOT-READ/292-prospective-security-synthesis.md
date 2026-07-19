# 292 — Prospective security synthesis

## 292:status-bank-before-review

This is a prospective final report written before the mechanism and product reviews of
round 29. It preserves conclusions reached during initial research and human–conductor
rubber-ducking so they can later be differenced against the independent review package.
It must not be used to prime those reviews. Agreement with this document is not
corroboration if the reviewer saw it first.

The report is deliberately self-contained. It synthesizes the changed-design prior art
in `291`, the earlier round-10 threat model, and the subsequent conceptual discussion.
Its claims remain provisional wherever the current implementation has not been audited.

## 292:executive-frame-elision-is-authority

The most useful security model for Dorc is not “untrusted input reaches shell.” That is
real, but generic. Dorc's distinctive power is that it can contradict the apparent
control flow of a shell program: replace an operation, guard it, elide it, or allow a
claim about one operation to preserve downstream work.

**+SURE:** Every optimization is therefore a grant of authority to prevent
user-written code from running. The governing question is:

> Who or what may cause which written effect not to happen, using what evidence, over
> what scope, for how long—and who suffers if that decision is wrong?

This question applies to every mechanism, not only visibly security-flavoured ones. A
probe result, oracle claim, cache entry, generation marker, administrator knob, context
entry wrapper, or controller inference gains security significance when it can license
replacement, elision, or downstream survival.

**~SUSPECT:** Dorc is best understood as a system for issuing narrowly scoped licenses
to contradict the apparent control flow of shell programs, with command optimization as
the principal benefit of those licenses. This framing is intentionally stricter than
the product description; it exposes the consequence of a false conclusion instead of
describing only the saved work.

## 292:model-knowledge-needs-authority-map

Dorc's gradual-enhancement problem and its security problem have the same shape. Both
resist local reasoning, cut across mechanisms, and place valuable goals in tension.
Gradual enhancement asks what additional value Dorc can derive from better knowledge.
Security asks what additional authority Dorc grants to the supplier and interpreter of
that knowledge.

**+SURE:** Knowledge and authority are separate axes. “The host returned `rc=0`” does
not contain its own permission to influence a displayed hint, add a runtime guard,
license a local replacement, preserve downstream operations, or affect another host.
Those are successively stronger uses of the same observation.

Every knowledge source therefore needs an explicit authority map:

- who supplied and interpreted it;
- which decisions it may influence;
- how far that influence reaches;
- when the authority expires or is revoked; and
- what the consequence is if the underlying claim is wrong.

The code-path intuition is useful but incomplete. Data flow answers what *can*
influence a decision. Authority answers what *is permitted* to influence it. Authority
is a policy overlay on the same dependency graph, not another name for reachability.

The existing observation/vouch distinction is an early form of this separation. A
remote observation supplies evidence; an oracle author interprets its meaning; an
administrator accepts operational consequences. None should silently stand in for the
others. A `__disturbs` claim has greater reach than a local convergence claim because it
can affect the survival of later operations. Cross-host aggregation is stronger again.

## 292:current-security-delta

Round 10 established Dorc's perimeter risks: the controller is the fleet-scale crown
jewel; SSH and proxy hops are trust boundaries; read-only probes are neither necessarily
harmless nor nonblocking; remote output can attack controller-side consumers; oracle
distribution is a software-supply-chain problem; and a version or content hash is not
publisher trust.

The design subsequently grew a more consequential interior. Host-produced report
records and retained whylogs bring hostile bytes into controller data structures. Rich
oracle claims can license underexecution elsewhere. Saved and reactive plans introduce
identity, freshness, cancellation, and replay questions. Wrapper-authored context entry
introduces privilege, siting, consent, and confused-deputy questions.

**+SURE:** The changed-design question is no longer only who can reach the controller.
It is which hostile, mistaken, or stale observations may become controller authority.

The initial prior-art pass supports six concrete concerns:

1. Managed-host bytes require controller-derived attribution, bounded ingestion, closed
   parsing for recognized records, inert handling for unknown records, and encoding for
   each eventual sink [A-redhat-ansible-fact-injection-2021]
   [A-mitre-log-neutralization-2025].
2. The approved decision needs an immutable identity spanning its relevant inputs and
   both human-readable and executable representations. Artifact identity remains
   separate from knowledge that the world is still fresh
   [A-hashicorp-terraform-plan-2026].
3. Oracle content identity and provenance must be preserved without claiming that a
   digest, immutable reference, or self-signature proves publisher trust
   [A-hashicorp-terraform-lock-2026]
   [A-hashicorp-plugin-signatures-2026]
   [A-github-actions-secure-use-2026].
4. Cancellation or supersession must revoke a generation's decision authority
   immediately, while remote execution may continue until separately observed quiescent
   [A-google-operation-cancellation-2026].
5. Arbitrary shell output cannot receive a credible generic secret-redaction promise.
   Dorc should minimize collection, classify and bound retained material, encode for
   sinks, and treat whylogs as sensitive artifacts
   [A-github-actions-secure-use-2026].
6. Context entry requires a targeted audit. `sudo -n` can establish non-interactivity;
   by itself it does not establish correct target identity, privilege siting,
   environment integrity, or administrator consent.

## 292:typed-boundaries-as-alarm-system

Strong typing can make much of the authority plumbing explicit. Its best role is to
make security-significant promotions impossible except through small, named,
reviewable constructors.

Host bytes should not arrive as an ambient `String`. A narrowing chain can distinguish
bounded bytes, a recognized record, controller-attributed host-scoped evidence, and a
decision input. The payload must not be able to assert its own host, invocation, or
generation identity.

Evidence should remain distinct from permission. Conceptually, Dorc can separate:

```rust
struct Observation<T>(T);
struct OracleVouch<K>(K);
struct AdminConsent<C>(C);
struct License<Action, Scope> { /* mint witness */ }
```

A local replacement license could require an observation and an oracle interpretation.
A downstream-survival license should additionally require a disturbance footprint,
administrator consent where appropriate, and evidence that the surviving operation is
outside that footprint. Cross-host influence should require a separate reviewed
aggregation policy, never an implicit conversion from host scope to fleet scope.

Plan state can similarly distinguish draft, reviewed, and executable decisions. Review
should bind book bytes, the exact oracle set, analysis-relevant knobs, controller build,
target identity, and the rendered and executable representations of the same decision.
World freshness remains a different value, established through guards or renewed
observation rather than smuggled into artifact identity.

Generation state should distinguish `Live`, `Superseded`, `Canceled`, and `Quiesced`.
Admission of evidence requires live generation authority. Revocation and a request to
stop remote work are separate operations; late records may remain useful as bounded
diagnostics but cannot return to analysis.

Diagnostics need two orthogonal axes. Untrusted data is not necessarily secret, and
secret data is not necessarily hostile. Types such as `TerminalText`, `JsonText`, and
`WhylogField` can enforce sink-specific encoding; a separate `Sensitive<T>` wrapper can
restrict retention and display. There should be no universal `sanitize()` that implies
all sinks and secrets have been handled.

Context entry should avoid a single permissive boolean. Mechanical capability,
administrator consent, oracle-author tolerance, and correct siting are different claims
and should be required separately when minting a context-entry license.

Finally, why-explanations should preserve the authority derivation when the license is
minted. A witness graph can record that an observation was interpreted by a particular
oracle claim, scoped to a target, consented to under a particular policy, and used to
license a particular decision. Reconstructing that chain later from logs is both harder
and less trustworthy.

**+SURE:** These types are not a proposed frozen API. Their value is architectural:
weak claims become more influential only through named promotions. The promotions form
an alarm system for review.

## 292:limits-types-preserve-not-decide

The typed model is suspiciously tidy because it concentrates on authority plumbing.
Authority plumbing is unusually typeable. The security-critical remainder is not.

### 292:limit-external-meaning-is-open

A type can preserve the claim `Fresh<Generation42,
HostScoped<Web01, PackageInstalled>>`. Somewhere, however, code must decide that a
particular command and result mean “package installed.” No signature proves that the
probe checked the intended installation, configuration, service health, reboot
survival, namespace, or credentials.

**+SURE:** Types can concentrate semantic judgments at choke points; they cannot validate
the outside-world model from which those judgments arise. Oracles exist specifically to
provide knowledge the generic analyzer cannot derive. Richer oracle knowledge therefore
increases the consequence of an oracle author's mistaken model.

### 292:limit-shell-equivalence-is-global

Proving that a command's return code and stdout are unused does not prove replacement by
`true` preserves every relevant observable. Execution may affect traps, signal timing,
inherited file descriptors, shell state, audit trails, locks, concurrent processes,
credential lifetimes, or world state that Dorc does not model.

**+SURE:** `NoConsumedStdout` is locally typeable; `ObservationallyEquivalentToTrue`
quantifies over an open set of observers. Dorc can define and enforce a deliberately
bounded observable model, but choosing that boundary is product policy. It determines
what Dorc promises to preserve and what risks users accept.

### 292:limit-world-freshness-races

A value can remain well typed while another process changes the remote world. Known
cancellation, generation mismatch, and supersession are typeable; unobserved drift is
not. Dorc must choose operational responses such as immediate guards, repeated probes,
leases, atomic primitives where available, narrow validity windows, or explicit race
acceptance.

**+SURE:** Those choices trade network work, availability, predictability, and safety.
They require global product judgment rather than builder compliance with a local coding
rule.

### 292:limit-policy-is-not-derived

Types can distinguish local replacement, downstream survival, and cross-host reuse.
They cannot decide whether an oracle footprint is complete, whether administrator
consent was meaningful, whether an oracle upgrade may expand prior authority, or which
wrong-world executions the product accepts.

**+SURE:** Encoding a chosen policy in types is valuable after the policy is chosen. The
type system does not choose, complete, or justify that policy.

### 292:limit-maintainers-erode-boundaries

Safe typed paths will reject some useful shell programs. Product pressure will then
favour an escape hatch, compatibility mode, warning downgrade, inference fallback, or
generic trust conversion. A type system cannot defend itself from maintainers who
change its constructors or meanings.

**~SUSPECT:** The important review sites are therefore not all consumers of authority
types. They are the few places that mint authority or redefine what a witness means.
Changing one of those sites is a security-design event even when the feature appears to
be ordinary planner ergonomics.

## 292:process-fable-compatible-not-invisible

The Fable security gate creates a temporary process constraint, not a required property
of Dorc. Security-bearing work should remain in quarantine while the gate persists, but
adjudicated requirements can cross into ordinary implementation as truthful local
invariants, types, tests, and interface contracts. The rationale may remain quarantined;
the exported explanation must not be false or ritualistic.

Hidden builder instructions can preserve already-decided mechanical properties:

- remote payloads cannot choose controller attribution;
- revoked generations cannot contribute decision evidence;
- host scope cannot widen implicitly;
- untrusted text requires sink-specific encoding.

They cannot safely decide or evolve Dorc's semantics: which observations license
elision instead of guarding, which observables the product promises to preserve, how
oracle trust transfers through reuse and upgrades, when stale evidence remains
actionable, or which unsafe cases receive escape hatches.

**+SURE:** A capable but security-blind conductor can invalidate a hidden security
argument without touching recognizably security-related code—for example by changing
the meaning of disturbance, introducing aggregation, or adding a convenient conversion
between authority scopes. Local builders cannot reconstruct the global policy from type
names alone.

The workable boundary is therefore visible but narrow. Fable-class agents may implement
behind adjudicated boundaries. Changes to authority mints, their required witnesses, or
the semantic promises those witnesses represent must return to a security-capable
reviewer or be explicitly deferred. Some properties can be held and tracked today;
others should remain named deferrals until a later recurring review has better conductor
options.

This compartment is acceptable while Dorc remains private. Before publication or use by
other people, omission of the real trust, artifact-sensitivity, redaction, and accepted-
risk boundaries becomes a release blocker. Process convenience cannot become a public
security claim.

## 292:prospective-product-hard-truths

The conversation supports several prospective hard truths for later review rather than
final acceptance now.

**~SUSPECT:** Dorc cannot promise general semantic equivalence for arbitrary shell. It
needs an explicit bounded-observables contract and must fail toward execution when the
required equivalence is outside that contract.

**~SUSPECT:** Correctness-focused oracles are executable dependencies with unusual
authority: they can cause other code not to run. Their provenance, update, composition,
and trust story must eventually be stronger and more explicit than ordinary shell
sourcing, even if no registry is built before MVP.

**~SUSPECT:** Whylogs and saved plans are sensitive operational artifacts, not harmless
explanations. They may reveal host topology, commands, state, configuration, and
unrecognized secrets even when every displayed field is correctly encoded.

**~SUSPECT:** Cross-host optimization should be exceptional and explicitly licensed.
Evidence valid for one host should not silently become fleet authority merely because a
controller can aggregate it.

**~SUSPECT:** Best-effort analysis does not justify best-effort authority. Analysis may
degrade under uncertainty; a promotion that licenses underexecution needs its complete
required witness or must fall back to guarding or running.

## 292:prospective-review-check

For each mechanism that can guard, replace, elide, or preserve downstream work, the
completed review should be able to answer:

1. What observation or claim begins the derivation, and which principal supplied it?
2. Which principal interprets it, and where is that interpretation encoded?
3. Which exact action does it license, at what host and program scope?
4. Which global semantic assumptions cannot be expressed in the local type?
5. What revokes the license, including time, generation, input identity, and world drift?
6. What is the safe fallback when any witness is missing or unreliable?
7. Can the admin understand and consent to the consequence independently of the oracle
   author?
8. Does the answer remain correct under reverse propagation, probe/apply, the other
   user, and unreliable-oracle cases?

The single conductor habit worth preserving is:

> Never review what Dorc knows without simultaneously reviewing what that knowledge is
> allowed to prevent.

That habit does not solve Dorc's security problem. It makes the product's distinctive
security decisions difficult to mistake for ordinary optimization plumbing.

