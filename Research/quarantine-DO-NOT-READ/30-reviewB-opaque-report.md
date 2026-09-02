# Opaque accrual review 30-reviewB

- Review identity: `30-reviewB`
- Pass: `initial` (no prior CONSTRAIN)
- Exact range: `3b999e47^..914c49a8`
- Reviewed HEAD: `914c49a8f8869678b3f6dc5e261eb2df6cbade38`
- Subject: the re-stabilized pre-build testing architecture in `Research/notes/30X-testing-architecture-seams-sessions-and-seeds.md`, with the full supplied range considered for net interaction.

## Evidence inspected

- +SURE `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` was read completely before inspecting the range.
- +SURE The exact-range commit list, diffstat, changed-path list, current clean branch/tip, and product-code diff were inspected.
- +SURE The complete current 30X design, the lane-A builder brief, the relevant conductor/scout/dispatch ledgers, and the exact `8ab53489` and `914c49a8` amendments were inspected.
- +SURE `30-reviewA-opaque-report.md`, including its appended human disposition, was inspected as quarantined predecessor evidence rather than accepted as a correctness claim.
- +SURE The relevant current contracts in `spike/CLAUDE.md`, `spike/crates/cli/CLAUDE.md`, `plans/282`, and `plans/128` were inspected, together with the current e2e sandbox/root routing, replay runner, shipped environment-selector seats, transport edge, and receipt `ModelIo` boundary.
- +SURE Supplied gate evidence was taken as given. No 30X build lane exists, so the subject was assessed as an architectural commitment rather than implemented behavior.

## Delta-scoped assessment

### The predecessor concern has been adjudicated and the remaining product boundary is explicit

- +SURE `30-reviewA` identified two coupled changes: the ordinary corpus becoming unrestricted persistent shell sessions, and fixture seam selections being able to meet production transport or roots. The human explicitly adjudicated the first as accepted developer-tool authority and rejected restoring a closed grammar or adding a general host-containment regime.
- +SURE That adjudication is now an explicit threat-model choice in the candidate rather than a silent step into new territory. `loom-syntax-grants-no-production-authority` states that the ordinary shell session is not contained from the developer machine, keeps unrestricted POSIX grammar, and routes intentionally ambient real-host/privileged exercises through the separate livetest composition.
- +SURE The accepted tightening removes the second half of the predecessor concern from the ordinary harness architecture. `Seams` is the union consumed by the engine; `HarnessSeams` is a constructor-side subtype with no `RealSsh` or OS-root arm; session configuration parses only the subtype; conversion into `Seams` is total; the shipped binary constructs the production row through `Seams::os()` and reads no harness selectors; runner drives begin with scrubbed inherited credentials and runner-owned roots.
- ~SUSPECT This split is sufficient at accrual-review granularity because the authority-bearing variants and their composition roots remain few and explicit before any corpus migration. The design does not teach loom authors a fixture-to-production selector spelling, persist a mixed format, or expose a production-capable harness contract that later callers must preserve.
- +SURE Native receipt I/O under a runner-owned pinned root does not undo the split. It exercises the production store state machine inside fixture-owned storage; `RealSsh` and platform production roots remain outside the ordinary harness type. The in-process receipt lane similarly composes the real edge over deterministic `ModelIo` without adding host transport or OS-root authority.

### Accrual and repairability

- ~SUSPECT The remaining implementation risk is concentrated at the two composition roots, the `HarnessSeams` parser/conversion, and runner root/environment setup. Those are identifiable choke points with objective negative tests before lane B converts the corpus; they do not require a persisted-format migration, recovery of discarded provenance, an unbounded authority-caller audit, or a compatibility break.
- +SURE The design requires lane A to stop at a checkpoint over exactly those fences before the session migration begins. A later ordinary security review can reject local construction mistakes at those points without reopening the architecture settled here.
- +SURE The prior unrestricted-shell concern cannot independently support another NACK after the human has explicitly selected that developer-tool threat boundary. Reissuing it would substitute the reviewer’s policy preference for the required human adjudication.

### Other range surfaces

- +SURE The why-surface tail refuses receipt-less remote apply before I/O, preserves the durable edge’s closed refusal word and store locus, and reports shipment outcome through an existing choke point. It does not introduce or entrench an accrual-scale boundary flaw.
- +SURE The pivot-book design and secure-durable review artifacts do not independently introduce a qualifying delta in this range. Their unresolved security-sensitive future work remains explicitly gated or deferred rather than silently constructed here.
- ~SUSPECT No other changed representation, public contract, authority mint, persistence route, or caller population in the range meets both qualification halves.

## Qualifying concerns

None.

## Repairability judgment

- ~SUSPECT Any defect in realizing the accepted two-type seam split remains locally repairable before or at the lane-A checkpoint. The candidate has not yet accrued corpus dependencies on mixed fixture/production selections, and the chosen shell-session authority is an explicit human-owned product boundary rather than missing design information.

## Confidence

- ~SUSPECT High on the delta and predecessor disposition: the amended design directly removes runtime selection of real transport and production roots from the ordinary harness while explicitly retaining the developer-machine shell authority the human accepted.
- ~SUSPECT Moderate on eventual implementation fidelity because no build exists; that uncertainty belongs to the required lane-A construction review and negative tests, not to an architectural NACK now.

## Hidden invariant inventory maintenance

- +SURE `sinv-production-fences` was updated in place rather than duplicated. It now records the human-adjudicated boundary: ordinary shell sessions are powerful developer tooling and make no host-containment claim, while fixture identity and harness selection remain structurally separated from real transport, OS production roots, shipped binaries, and public artifacts. It retains the runner credential/root obligations, separate artifact-execution rail, constructor-routing tests, and prior lexical-fence law. Review identity `30-reviewB/opaque-accrual` was appended.

## Final outcome

ACK
