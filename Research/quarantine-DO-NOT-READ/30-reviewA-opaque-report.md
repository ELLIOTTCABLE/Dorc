# Opaque accrual review 30-reviewA

- Review identity: `30-reviewA`
- Pass: `initial` (no prior CONSTRAIN)
- Exact range: `3b999e47^..633fd954`
- Reviewed HEAD: `633fd9543305d8d14a82ae912df15c7ae95a86bd`
- Subject: pre-build testing-architecture design in `Research/notes/30X-testing-architecture-seams-sessions-and-seeds.md` and `Research/notes/30Xa-test-infra-decruft-conductor-ledger.md`, with the rest of the range considered for net interaction.

## Evidence inspected

- `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`, read completely before the git range.
- Exact-range commit list, diffstat, file-status list, and the product-code changes in the range.
- The complete current 30X design and 30Xa conductor ledger, including the serial lane plan and the 2026-09-02 amendments.
- `spike/CLAUDE.md` governing safety, host-ingress, production-fence, durable, receipt, and test-runner rules; `spike/crates/cli/CLAUDE.md` and `spike/crates/aid/CLAUDE.md` pointers for the affected edges.
- `Research/plans/282-transcript-case-prose-pipeline.md` replay-driver and execution-harness contract, especially its controlled environment, allow-listed PATH, worktree-local/no-network rail, and explicit whole-command modeling requirement.
- `Research/plans/128-cross-network-tdd-ci-conclusion.md` for the controller-to-host transport seam and the separation between synthetic transport testing and real-network integration.
- Current `spike/crates/cli/tests/e2e.rs`, including the narrow `run_replay_block`, `Harness::rail_under`, profile-root redirection, closed-loop local transport, and artifact-execution rail.
- Current `spike/crates/cli/src/main.rs`, `transport_edge.rs`, `durable.rs`, `apply.rs`, `lib.rs`, and `Cargo.toml`; `receipt-local`'s sealed `LocalIo` and public deterministic `ModelIo` boundary.
- Prior reports `28-reviewA-opaque-report.md` and `29-reviewA-opaque-report.md` for continuity on fixture re-entry and width-one production fences.
- Supplied gate evidence was taken as given. No build lane exists yet, so the subject's claims were assessed as architecture rather than implemented behavior.

## Accrual-threshold assessment

### Qualifying concern: the proposed loom/harness execution plane crosses the existing test-to-host boundary without a containment contract

+SURE The range changes an ordinary loom replay from a narrow, modeled `dorc` invocation into an unrestricted persistent POSIX-shell session. `30X:loom-is-a-shell-session` says that no block content is restricted, that the process tier's unsupported set is empty, and explicitly treats a future need to run `chroot` as a reason to broaden the driver. `30X:loom-process-driver-is-a-real-shell` makes `export`, `cd`, pipes, external tools, and redirects native in the ordinary process proof. Lane B is committed to making that driver the route for the whole corpus.

+SURE This is a delta from the architecture the range says it composes with. `plans/282` section 7 confines generic execution to an `env -i`-style environment, a PATH containing the built tool and inert mocks, a temporary cwd, no real mutators, no network, and worktree-local effects. The current runner is narrower still: extra replay blocks accept only a whitespace-split `dorc ...` command with two closed redirect forms, while rendered artifacts execute through `env_clear`, mocks-only PATH, and throwaway roots. The range preserves that artifact rail but does not preserve or replace its containment rules for the new general session rail.

+SURE A throwaway cwd and a shimmed `dorc` name are not a containment boundary for a real shell. Native `cd`, absolute or parent-relative redirections, command paths, sourced files, inherited descriptors/environment, and networking remain ambient unless separately denied. The design requires expressive freedom while omitting the policy and mechanism that bound those authorities. Consequently, converting every directory case and every prose-bearing loom onto the one runner turns committed authoring artifacts into an ordinary-suite execution language with ambient host authority.

+SURE The seam matrix compounds the same boundary failure. `model-seams-are-one-bundle` gives each seam an independent `Seeded` / `Pinned` / `Os` selector and gives transport `Scripted` / `Hostsim` / `RealSsh`; `model-unpeg-per-seam` permits an individual seam to become OS-backed; the harness parses selections from the shell session's environment. The plan protects the shipped `dorc` from fixture environment reads, but does not forbid the inverse composition: fixture identity/key/clock/root choices combined with real transport or production persistence. `publish = false`, a sibling binary name, and refusing only the all-default invocation do not make those combinations structurally unreachable.

+SURE This inherently changes the quarantined threat model rather than merely underspecifying hardening. The current model separates deterministic fixture authority from real remote transport, production roots/default persistence, ambient credentials, and public artifacts, while the proposed cross-product and unrestricted shell runner make those boundaries selectable or ambient. The change is silent at the user-facing design level: 30X frames the breadth as testing expressiveness and places only the shipped-main one-way fence at its checkpoint.

~SUSPECT Deferral would make repair materially harder. The one-runner conversion, session-shaped corpus, sh-spelled seam declarations, driver derivation, process/in-process equality gate, and promise that process-tier support is unrestricted will teach hundreds of cases and all future prose-bearing tests that ambient shell semantics and independently mixed seams are part of the harness contract. Restoring a contained execution boundary later would require either invalidating that contract and auditing/migrating an expanding corpus, or constructing a cross-platform sandbox around semantics the cases already assume. The range deliberately deletes the old per-shape routing and marker grammar, so the former choke points would no longer remain available for a local repair.

### Why this cannot cross as one CONSTRAIN packet

+SURE A truthful complete repair requires a product-level choice among materially different security boundaries: keep a closed command/session grammar; permit arbitrary shell syntax inside an OS-enforced sandbox; split trusted integration fixtures from ordinary looms; or accept ambient developer/CI authority. It also requires deciding which `Os`/`RealSsh` seam combinations are legitimate, how the sibling harness is excluded from distribution, and whether real-host exercises remain only in the opt-in livetest lane. These choices alter the design's central expressiveness promise, cross-platform behavior, and test-tier contract. A mechanical local directive cannot settle them without importing the hidden rationale or asking the conductor to adjudicate risk.

## Other range surfaces

+SURE The why-surface tail strengthens the remote-apply dispatch boundary by refusing `--no-receipt` before I/O and by retaining closed durable-refusal words. It does not create an accrual-scale flaw.

~SUSPECT Rendering the selected store path as an engine-owned parameter and the current debug transport environment remain ordinary localized review subjects; neither changes this verdict and neither independently meets both qualification halves.

+SURE The `dorc-loom` dependency on the real receipt edge over `ModelIo`, seeded entropy, and a ticking model clock is repairable at identifiable composition points if kept wholly model-backed. It becomes part of the qualifying concern only when the harness matrix permits those fixture values to meet real transport or production roots.

+SURE The pivot-book language design, quarantine closure artifacts, steering edits, and reviewer-prompt changes in the range do not independently introduce a qualifying delta under this pass's exclusive question.

## Qualifying concerns

One. The proposed general shell-session runner and independently selectable harness seam matrix introduce a new execution/authority boundary without preserving the existing containment and production-fence contract. The concern is cross-cutting, is expected to become the single route for the complete corpus, and changes the applicable threat model.

## Repairability judgment

~SUSPECT Repair is cheap only before lane A/B establish the public types, selector grammar, harness artifact, one-runner routing, and migrated corpus. After those accrue, repair would require a corpus-wide capability audit, a changed session-support promise, new cross-platform isolation infrastructure or test-tier bifurcation, and potentially removal of relied-upon seam combinations. The needed information has not yet been destroyed because no build lane started; this review is the last low-cost design checkpoint.

## Confidence

~SUSPECT High on delta attribution and threat-model invalidation: the old runner and plan state an explicit closed/contained execution rail, while 30X explicitly removes command-content restrictions and exposes the full per-seam cross-product. Moderate on which replacement boundary the human should choose; that policy uncertainty is why the outcome is NACK rather than CONSTRAIN.

## Hidden invariant inventory maintenance

+SURE `sinv-production-fences` was merged, not duplicated. Its governed surface now names harness binaries, test seam selectors, and session executors; it requires a scrubbed/network-denied/runner-rooted execution boundary, prohibits fixture-to-real-transport or production-persistence combinations, names non-fences, and adds positive negative-test obligations. Review identity `30-reviewA/opaque-accrual` was appended.

## Final outcome

NACK
