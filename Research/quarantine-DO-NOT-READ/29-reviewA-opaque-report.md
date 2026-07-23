# Opaque accrual review 29-reviewA

- Review identity: `29-reviewA`
- Pass: `initial`
- Exact range: `49b6642110126091bfe8ba4f7d67ea1952baad5f..b6fde35572feaa0d45f76fa1c30091d5e7d24b28`
- Reviewed HEAD: `b6fde35572feaa0d45f76fa1c30091d5e7d24b28`
- Assigned scope: round-29 immediate bounded-authority ingress closeout, including report-channel repair, aggregate-vouch repair, bounded records/whylog-v2 admission, private width-one CLI attribution, closed refusal, fixture/hostsim migration, and quarantine handoff.

## Evidence inspected

- The exact range's commit list, file list, aggregate diff statistics, and focused diffs.
- `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`.
- Round-29 charter, conclusion, build plan, phase packets, resumption handoff, and closeout handoff (`290`, `295`, `297`, historical `298`, and `299`).
- Current report-channel rendering and oracle-contract surfaces.
- Aggregate vouch types, mint paths, disposition callers, receipt projection, erasability projection, and focused negative/positive tests.
- Live records admission, limits, closed grammar, retained-byte and collection accounting, refusal flow, CLI conversion, private width-one scope wrapper, and downstream authority consumers.
- Whylog-v2 bounded outer parsing, independently bounded inner record admission, writer gating on admitted records, replay-claim comparison, replay conversion, and CLI durable handling.
- Fixture, e2e framing, hostsim byte-fault, and diagnostic-consumer migrations.
- Supplied exact-revision verification evidence: WSL workspace build/tests green; e2e 97/97; formatting; clippy with warnings denied; cargo-deny; typos; and loom fixpoint green. Native shell tests were unavailable because `dash`/`sh` was absent. Tests were not rerun during this read-only review because the task authorized workspace writes only for this report and any required hidden-invariant update.

## Accrual-threshold assessment

+SURE The report-channel delta removes the host-pathname capture protocol rather than preserving a pathname compatibility route. Runtime capture is disabled at the renderer boundary, authored report writes fall through to `/dev/null`, and the remaining report recognition/rendering is not a replacement authority path.

+SURE Aggregate mutation replacement now crosses private, non-empty `AllEstablishesVouched` construction. The constructor checks exact ordered site/fact identities, duplicate state, cardinality, and per-entry reached vouch presence before either aggregate mint can consume the proof. Query-only substitution remains a distinct proof and does not manufacture mutation authority. No range-added alternate aggregate mint was found.

+SURE Live and replay ingress now have bounded byte-first admission and a closed refusal outcome before the CLI constructs planning inputs. Refusal returns before plan construction, artifact rendering, or whylog writing. The whylog writer accepts only previously admitted record bytes, and v2 replay re-applies an independent inner-record budget instead of trusting the outer durable budget.

~SUSPECT The private CLI width-one attribution wrapper is intentionally narrower than the eventual controller-wide scope law: downstream spike kernels still operate in the pre-existing single-attempt type universe after the wrapper is borrowed. This does not meet the accrual threshold in this exact delta because no range-added transport, concurrency, cache, cross-host reuse, retry, saved approval, or public production identity boundary can create a second scope; the private width-one types and hard-coded width-one durable claims make the missing re-entry visible. Repair before any such widening remains possible at the current CLI/admission choke point and does not yet require migration of public persisted authority or user-authored artifacts.

~SUSPECT Several legacy/raw and spike-only surfaces remain alongside the new admission route, but the exact production CLI path in this range no longer calls them. Their removal or phase-five fencing is local and rediscoverable; it does not satisfy both accrual qualification halves.

## Qualifying concerns

None.

## Repairability judgment

+SURE No range-introduced concern was found that is both fundamental/cross-cutting and likely to become compatibility-breaking, provenance-destroying, or impractical to repair if further bounded spike work accrues. Residual re-entry obligations remain at identifiable private or module-level choke points and are already represented in the hidden inventory.

## Hidden invariant inventory

+SURE No new security-critical invariant was established or revealed beyond the existing `sinv-owned-probe-channel`, `sinv-mutation-elision-vouch`, `sinv-host-evidence-ingress`, `sinv-controller-attribution`, `sinv-integrity-failure-mutation`, `sinv-sensitive-artifacts`, and `sinv-production-fences` entries. The inventory was not modified.

## Confidence

~SUSPECT Confidence is high for the exclusive accrual question and moderate for ordinary localized correctness, which this review intentionally does not certify. The unavailable native shell lane does not change the architecture-scale outcome because the WSL shell/e2e lane was supplied green and the report-channel repair deletes rather than adds emitted shell plumbing.

## Final outcome

ACK
