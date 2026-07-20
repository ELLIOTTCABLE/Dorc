# Round 29: changed-design security prior art

## 291:conclusion-security-delta-is-real

Round 10 found the enduring perimeter: the controller is the fleet-scale crown jewel; SSH and proxy hops are trust boundaries; probes are not magically side-effect-free; terminal output is an injection surface; binary identity drifts; and executable-oracle distribution is a supply-chain problem. Those conclusions still stand. Repeating them would waste this round.

The design since then has added a more consequential interior: host-produced report records, retained whylogs, rich oracle claims that can license underexecution elsewhere, executable plan artifacts, wrapper-authored context entry, and reactive planning with cancellation. The new security question is therefore not merely “can an attacker reach the controller?” It is: **which hostile or stale observations may become controller authority?**

The prior art supports a compact answer: bind every decision to immutable inputs; keep identity separate from trust and freshness; treat every managed-host byte as hostile data; make cancellation revoke authority without pretending it stopped execution; and export these conclusions as ordinary invariants and tests. These are release-boundary requirements, not all immediate implementation work.

## 291:finding-host-output-is-hostile

**+SURE:** Managed-host facts are an active controller attack surface, not passive diagnostics. Red Hat reproduced CVE-2021-3583, where Ansible facts used in multi-line templates enabled controller-side template injection; it was rated high severity and fixed in Ansible Engine 2.9.23 [A-redhat-ansible-fact-injection-2021]. MITRE’s canonical log-neutralization guidance likewise treats externally influenced log data as capable of forging records, corrupting downstream interpretation, or reaching executable log consumers [A-mitre-log-neutralization-2025].

For Dorc, this applies to `DREP_V1`, malformed/free-form report lines, captured stdout and stderr, tool errors, whylogs, and any later machine reader. “Noise tolerant” must mean tolerant storage of inert bytes, not permissive promotion into trusted structured facts.

The minimum invariant is concrete: bound input before allocation; parse known records against a closed grammar and types; derive host, invocation, and generation attribution from the controller’s transport context rather than payload claims; keep unknown records inert and separately namespaced; and encode again for each terminal, log, HTML, JSON, or shell sink. Canonicalize before validating where multiple encodings exist [A-mitre-log-neutralization-2025]. No host-originated string may enter templates, shell source, identifiers, paths, or claim algebra merely because it arrived through an oracle.

**~SUSPECT:** Cross-host aggregation should default to isolation. GitHub’s mature remote-execution guidance warns that jobs and actions sharing environment, files, or a Docker socket can compromise one another, and that persistent self-hosted runners expand compromise across repositories [A-github-actions-secure-use-2026]. By analogy, Dorc should not silently merge host-derived facts into global caches or licenses. Any cross-host fact should carry explicit provenance and a deliberately reviewed aggregation rule. This is an inference, not a direct prescription from GitHub.

## 291:finding-plan-identity-needs-binding

**+SURE:** Approval, replay, and apply require a single decision identity. Terraform distinguishes a speculative plan—which can differ when the world changes—from a saved plan that records the actual actions later supplied to apply. It also warns that saved plans contain configuration, planned values, options, and sensitive values in cleartext [A-hashicorp-terraform-plan-2026].

Dorc should bind the approved/executable plan to at least the book bytes, exact oracle set, analysis-relevant knobs, controller version, target identity, and rendered/executable artifact. Apply must reject mismatches or require a new decision. The human view and the `.sh` executed must be two representations of the same identified object, not separately regenerated cousins.

This does **not** solve world drift. Immutable artifact identity answers “is this the decision that was approved?” Guards and fresh observation answer “does the current world still license it?” Terraform’s distinction is useful precisely because it does not collapse these questions [A-hashicorp-terraform-plan-2026]. Plan and whylog artifacts should be considered sensitive by default, even when Dorc has not recognized a secret.

## 291:finding-oracle-identity-is-not-trust

Round 10’s basic supply-chain concern is still correct, but rich oracle claims raise the consequence: an oracle may now license other work not to run.

**+SURE:** Mature plugin systems separate byte identity from publisher trust. Terraform lock files select exact provider versions and record checksums, while documenting trust-on-first-use and platform/mirror caveats [A-hashicorp-terraform-lock-2026]. Its signing model separately distinguishes registry-authenticated publishers, partner signatures, self-signed providers, and manually installed unsigned providers; self-signing alone does not create a trusted authentication chain [A-hashicorp-plugin-signatures-2026]. GitHub similarly says a full commit SHA is the only immutable action reference, while still requiring source audit and trust judgment [A-github-actions-secure-use-2026].

The pre-MVP requirement is not “build a registry.” It is to preserve an exact oracle-set manifest/digest in decisions and diagnostics, expose unpinned or unauthenticated provenance honestly, and never describe a hash or Git SHA as proof of trust. Publication may defer a distribution mechanism; it cannot omit an honest executable-oracle trust boundary.

## 291:finding-redaction-cannot-be-generic

**+SURE:** Dorc cannot promise generic secret scrubbing of arbitrary shell output. GitHub documents that automatic redaction is not guaranteed: transformed secrets must be separately registered, exact matching can fail, structured values interfere with redaction, and exception or error output can leak secrets [A-github-actions-secure-use-2026].

The defensible design is data minimization: collect structured safe fields where possible; keep opaque bodies bounded and separately classified; avoid retaining them unless they serve a concrete purpose; sanitize for each output sink; and treat whylogs as sensitive artifacts even when no recognized secret appears. “We scrub secrets” is too strong unless the product defines a narrow, testable secret representation. A safer promise is that Dorc minimizes, isolates, marks, and access-controls diagnostic material, with explicit limits on redaction.

## 291:finding-cancellation-revokes-authority

**+SURE:** Cancellation acknowledgement is not execution finality. Google’s long-running-operation contract says cancellation is best effort and not guaranteed; clients must subsequently inspect the operation to learn its outcome [A-google-operation-cancellation-2026].

Dorc therefore needs two separate states:

- decision finality: cancellation or supersession immediately revokes every license and fact produced by that generation; and
- execution quiescence: the remote command may still be running or emitting output until independently observed finished.

Every inbound record should be tagged by the controller with host, invocation, and generation. Late records from canceled or superseded generations may be drained for bounded diagnostics, but cannot re-enter analysis or resurrect licenses. This is more important than promising that cancellation kills a remote process, which distributed systems cannot generally guarantee.

## 291:finding-security-process-must-export

**~SUSPECT:** A quarantined threat narrative can coexist with ordinary implementation work only if its decisions escape quarantine as truthful engineering constraints. NIST’s SSDF explicitly frames secure practices as part of each SDLC, calls for maintained security requirements, provenance, and tracking of risks and design decisions, and emphasizes outcome-, risk-, resource-, feasibility-, and automation-based tailoring rather than a fixed checklist [A-nist-secure-development-framework-2026].

That supports a small repeated process: quarantine may contain attacker narratives and sensitive rationale; exported artifacts should be plain invariants, interface contracts, regression tests, fixture expectations, and release criteria. It does **not** support disguising security work through false explanations. Fable need not receive the full threat model, but it must receive accurate requirements such as “records from superseded generations never affect analysis.” Temporary pre-public documentation may omit quarantine detail; a public release must state the real trust and redaction boundaries.

## 291:uncertainty-context-entry-needs-audit

No retained source in this pass was sufficiently close to Dorc’s wrapper-authored `cmd__enter` mechanism to justify a firm design prescription. `sudo -n` establishes non-interactivity, not correct privilege siting, target identity, environment integrity, or freedom from confused-deputy behavior. This remains a targeted mechanism-audit item, not evidence that the current design is unsafe.

Likewise, the report-stream recommendations above establish trust boundaries, not a wire-format redesign. The implementation review should test actual length bounds, parser recovery, attribution, sink encoding, and allocation behavior before choosing changes.

## 291:quarantine-release-boundary-actions

The smallest useful carry-forward package is:

1. **291:action-bind-approved-decision:** define one decision identity spanning source, oracle set, knobs, target, and executable/rendered representations; distinguish it explicitly from world freshness.
2. **291:action-isolate-host-records:** specify controller-derived attribution, generation tags, size bounds, closed parsing for known records, inert handling for unknown records, and per-sink encoding.
3. **291:action-revoke-stale-evidence:** make canceled and superseded generations incapable of licensing later analysis, even while their remote execution may continue.
4. **291:action-state-oracle-boundary:** preserve exact oracle provenance and state plainly that immutability is not publisher trust.
5. **291:action-narrow-secret-promise:** replace any generic scrubbing claim with testable collection, classification, retention, and display guarantees.
6. **291:action-audit-context-entry:** inspect the concrete wrapper and privilege-entry implementation in the code-review phase.

This round should not build all six mechanisms. Before MVP, it should decide which are immediate correctness invariants, which are release gates, and which are explicitly deferred with an owner and trigger. The hard stop is publication: the product must not ship with undocumented trust, artifact-sensitivity, or redaction boundaries.

