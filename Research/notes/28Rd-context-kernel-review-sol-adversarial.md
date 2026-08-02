# Adversarial review of `28Q — context-kernel unification`

This is coverage, not a calibrated “unbiased” verdict. I performed both a neutral reading and a hostile reading in the same top-level context because delegation was explicitly forbidden. Findings below survived re-reading against the root ground truth and cited predecessor records.

Source weighting: the four root documents are the human-controlled ground truth; maintained plans are secondary design authority; historical notes are evidence about implemented behavior and known gaps.

## 1. Verdict primacy’s migration gate cannot detect the principal value regression

Severity: High

Attacked claim: §4 `rul-verdict-primacy-at-the-ship-seat`; §8 `stage-0-ship-seam`.

The typed ruling itself is fixed. The unpriced consequence is that the proposed gate checks local site outcomes while the change can discard named measurements needed by other sites.

Evidence:

- `28Q` says the verdict body becomes the shipped measurement, while predict’s cells remain only in the static topology:

  > “at a vouched site the VERDICT body ships as the probe check and its own reached answer is the convergence measurement”  
  > “the predict's argparse/cells keep feeding the static concern topology unchanged”  
  > — [28Q, lines 296–304](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:296), [stage 0, lines 366–375](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:366)

- When the verdict body has no corresponding gen-mark, `28Q` explicitly falls back to an auto-cell:

  > “the verdict body ships and measures (via its own marks or the auto-cell)”  
  > — [28Q, line 368](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:368)

- The historical split-family test demonstrates why predict was previously load-bearing: its body resolved and measured the named cell, while another author’s verdict supplied the vouch:

  > “author one's predict resolves the cell … author one's body runs, its rc becomes the `effect=holds`”  
  > — [28P, lines 997–1022](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/notes/28P-oracle-loading-resume-conduct-ledger.md:997)

- Dorc’s value is not merely whether that command elides. Named facts drive downstream invalidation, survival and cross-oracle composition. The root story describes a predict sibling precisely as a lane that “states what is true and predicts what would happen” even when it does not license skipping. See [USER_STORY.md, lines 375–381](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/USER_STORY.md:375). The attention product depends on those downstream facts remaining usable; see [USER_STORY.md, lines 521–543](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/USER_STORY.md:521).

Why the stage gate is inadequate:

A site can retain the same `Run`/`Guard`/`Elide` outcome while its measurement changes from a named predict cell to a private auto-cell. The stage-0 gate requires:

> “site OUTCOMES byte-stable across the corpus”

That does not test whether:

- downstream backings still intersect the right cells;
- survival decisions remain stable;
- why-chains retain the same measurement identity;
- a predict-only named fact has silently become unmeasured;
- later sites gain guards or lose elisions.

~SUSPECT the likely failure direction is conservative value loss rather than immediate wrong elision: static concern topology refers to predict cells for which no corresponding probe measurement was produced. But the plan does not state the invariant connecting the shipped verdict cell to predict-derived topology, so I cannot rule out mismatched fact consumption without code-level tracing.

Required repair to the plan: retain the typed verdict-primacy ruling, but expand stage 0’s gate from site outcomes to the complete fact/verdict record set, downstream plan shape, survival/backing decisions, and why provenance. Add a defining split-family case where `predict()` marks a named cell and a markless `is_converged()` receives an auto-cell.

Confidence: +SURE that the gate misses this class; ~SUSPECT about the exact amount of corpus churn.

## 2. An overlapping entry-closure has no unique speaker under the proposed identity model

Severity: High

Attacked claim: §1 DefinitionId factoring plus §2 “the entry-closure is the speaker.”

`28Q` combines three statements that do not jointly define an identity for shared files:

1. A derived row is computed once and keyed by one `DefinitionId` containing custody.
2. Custody is re-keyed to entry-closure membership.
3. A defining/helper file may participate in diamond loading and multiple sourced packages.

Evidence:

- P1 defines:

  > “Every derived row … is keyed by the DefinitionId that produced it: (SourceFileId, span, custody). Computed once, whole-unit”  
  > — [28Q, lines 81–84](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:81)

- P2 then changes custody from a defining-file identity to closure membership:

  > “custody becomes closure-membership; consumers still only compare”  
  > — [28Q, lines 142–144](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:142)

- The precursor only proved that `DefinitionCustody` could change its internals. It did not solve non-unique membership:

  > “a newtype over `SourceFileId` … if … re-keys custody from the defining file to an entry file's transitive sourcing-closure, the re-key is a change to this type's internals”  
  > — [28P, lines 857–867](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/notes/28P-oracle-loading-resume-conduct-ledger.md:857)

- The closure proposal explicitly permits helpers to be shared and assigns their custody to the calling entrypoint:

  > “helpers reached from a single live role body ride under that body's author's custody”  
  > — [28M, lines 262–280](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28M-committee-speech-and-the-custody-price.md:262)

- A real built fixture already has a book-owned entrypoint resolving into “a DIFFERENT author's file”:

  > “a book is a first-class definition source whose closure resolves into a DIFFERENT author's file”  
  > — [28P, lines 981–988](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/notes/28P-oracle-loading-resume-conduct-ledger.md:981)

Consider two entry files, `A.sh` and `B.sh`, both sourcing `common.sh`. A definition in `common.sh` has one `(SourceFileId, span)` but belongs to two entry-closures. The plan does not choose among:

- one `DefinitionId` with set-valued custody;
- two closure-relative `DefinitionId`s for the same bytes;
- custody belonging to the nearest role entrypoint rather than the source closure;
- collapsing overlapping closures into one speaker—which would incorrectly make A and B co-authors.

Each choice changes committee-fence comparisons, kind-owner occupancy, blessing reach and attribution. “Consumers still only compare” is insufficient: equality of what object is the unanswered design question.

This is particularly dangerous because custody is a correctness type intended to make cross-author licensing unrepresentable. An ambiguous or accidentally shared custody can turn a dialogue into a monologue at the type level.

Required repair to the plan: specify closure identity for overlapping and nested closures before stage ii. Give explicit truth tables for:

- two independent entries sourcing one helper;
- one entry sourcing another entry;
- a diamond with one shared definition file;
- the same file sourced ambiently and again inside a subshell;
- a book entrypoint using a published package’s helper file.

Then state whether `DefinitionId` is source-definition identity or closure-relative utterance identity. It cannot silently be both.

Confidence: +SURE that the plan is underspecified and internally inconsistent here; ~SUSPECT that a naive implementation would either over-fence harmless sharing or launder custody across packages.

## 3. P2 omits the known runtime packaging prerequisite for book-side sourcing

Severity: High

Attacked claim: §2’s “payoff gate” and §8 `stage-ii-closure-custody`.

The plan names the engine blessing for an inert `.` as the sole book-side payoff gate, but predecessor work established a separate, still-unbuilt runtime requirement: a sourced book needs its load closure present in the apply environment.

Evidence:

- `28Q` says:

  > “Payoff gate, human-owned: the `.`-of-a-proven-load-inert-file blessing — until a book's top-level source stops walling, book-side closures are analysis-real but value-dead”  
  > — [28Q, lines 182–188](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:182)

- `28K` separately records:

  > “a book that sources oracle files needs them present at apply (Dorc: bundle the statically-known closure; off-ramp: ship the directory…)”  
  > — [28K, lines 351–353](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28K-oracle-loading-and-resolution.md:351)

- The implementation ledger then found that no executing corpus case exercises book-level sourcing:

  > “NO executing corpus case has ever had a book-level `.`”  
  > “the exec rail's cwd is an empty throwaway sandbox, so the rendered apply exits rc 2”  
  > “`res-book-ships-its-load-closure` — named there, unbuilt”  
  > — [28P, lines 1393–1402](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/notes/28P-oracle-loading-resume-conduct-ledger.md:1393)

- Stage ii schedules custody, fences, occupancy, blessing reachability and lattice work, but not bundling, path preservation, or a real execution test. See [28Q, lines 381–386](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:381).

This creates a false-green risk: analysis fixtures can demonstrate that closures resolve, while an actual plan containing `.` fails under the execution rail. That directly harms the admin user: the advertised sh-native package boundary works in analysis but not when the approved artifact runs.

Even if the future inert-source blessing comments the `.` line out and pins all needed definitions into a preamble, that behavior itself needs to be stated and tested. It also must preserve the plain-sh off-ramp, where running the original book still requires its directory layout.

Required repair: add runtime closure materialization/elision to stage ii’s scope and gate. At minimum, an end-to-end case must execute:

- the original book as plain sh with its sourced tree;
- the Dorc-produced artifact from an isolated working directory;
- cross-file helpers and constants;
- nested relative sourcing;
- a missing sourced file, which must fail honestly rather than appear analysis-complete.

Confidence: +SURE.

## 4. Host alias resolution conflicts with the hard per-host isolation boundary

Severity: High

Attacked claim: §3 host aliasing as ordinary resolve machinery; §3 assertion that `an-host-as-adversary` remains honored.

`28Q` treats host aliasing as analogous to package-name aliasing:

> “Host ALIASING (`~/.ssh/config`, short names, IPs) is package-name aliasing's cousin: host identity is resolve-machinery territory, never string comparison.”  
> — [28Q, lines 223–226](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:223)

That analogy breaks at the failure mode.

Within one host, a resolver that wrongly merges two package names normally makes their footprints collide and therefore over-verifies. The root user story explicitly prices resolver errors this way:

> “a resolver that wrongly MERGES two entities only over-verifies; one that wrongly SPLITS one referent re-opens the silent skip.”  
> — [USER_STORY.md, lines 593–597](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/USER_STORY.md:593)

For hosts, a false merge is not conservative. If `web1` and `web2` are wrongly canonicalized as one host context, a measurement from `web1` can license an elision for `web2`. The merge crosses entire worlds.

That conflicts with established multihost law:

> “Every fact carries its `HostId` … per-host accumulators are disjoint maps; `build_plan` for host X consumes only X's facts. There is deliberately no cross-host lookup API”  
> — [260 multihost plan, lines 197–208](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/260-round26-multihost-plan.md:197)

The same plan intentionally defined `HostId` as:

> “the ssh destination string, verbatim — an alias resolved by the user's ssh config is first-class; Dorc never parses it”  
> — [260 multihost plan, line 197](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/260-round26-multihost-plan.md:197)

`28Q` says “no host speaks for another’s availability” at [lines 257–264](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:257), but host canonicalization can violate that unless the resolved identity is established by a controller-trusted mechanism before fact partitioning. An oracle/host-provided resolver is insufficient under the host-as-adversary model.

Required repair: distinguish:

- invocation spelling / SSH destination;
- controller-authenticated host identity;
- lifecycle identity of a managed resource;
- user-declared aliases.

A host merge must not use the ordinary kind resolver contract. It needs a controller-owned, authenticated equivalence proof, or it must remain separate and pay conservative duplicate probing. The per-host partition acceptance test must include deliberately false alias merges.

Confidence: +SURE about the failure-direction mismatch; ~SUSPECT about whether the intended implementation would actually reuse facts after canonicalization, because `28Q` never states the partition boundary.

## 5. “Zero new spellings” is contradicted by the reserved authored surface

Severity: Medium

Attacked claim: §0 `syn-zero-new-spellings`; §6 preserved-invariant wall; §10.

Evidence:

- The design claims:

  > “all three pillars consume EXISTING sh acts”  
  > “No new authored surface anywhere”  
  > — [28Q, lines 67–71](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:67)

- It repeats `syn-zero-new-spellings` among preserved invariants at [lines 337–346](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:337).

- Yet §10 reserves decisions for:

  > “begin/end description members or marks”  
  > “the ssh entry-form's authored half”  
  > — [28Q, lines 426–431](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:426)

A shell command such as `useradd alice` is an existing sh act, but the claim that it *begins a context* is not available from sh syntax. Under Dorc’s own referent-agnostic and silence-licenses-nothing laws, an oracle author must describe that semantic. Whether the description is a new role member, a mark, or an extension of an existing effect family, it is authored surface and engineering work.

The distinction matters because engineer effort is the project’s second priority, and the project has already banked:

> “the language is becoming crufty”  
> — [28M, lines 590–595](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28M-committee-speech-and-the-custody-price.md:590)

The plan currently prices P3 as an engine unification while deferring the actual engineer-facing complexity that makes it usable.

Required repair: weaken `syn-zero-new-spellings` to “no new admin/runbook syntax.” Price the oracle surface explicitly before calling the pillar unified. Compare at least:

- extending existing establishes/kills/effect vocabulary;
- new begin/end marks;
- new role members;
- refusing general lifecycle modeling and implementing only a closed stdlib set.

Confidence: +SURE.

## Lines of attack that did not hold

### P1 is not k-CFA-style context sensitivity

No finding.

The plan’s rebuttal survives scrutiny. Frames are generated by a finite set of program-text environment mutations and a statically known fork tree; there are no call strings or recursively recombined abstract closures. The touch count may be large, but the claimed asymptotic category error is not present. See [28Q, lines 118–124](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:118).

Confidence: +SURE.

### Lifecycle availability is not inherently circular

No finding.

I initially suspected a probe/plan fixed-point: creator convergence determines future availability, while availability determines which probes can run. In the described cases, the dependency is forward:

- probe the creator from an already-available context;
- decide whether its event will execute;
- derive later availability;
- probe a later context only if already available at probe time;
- otherwise guard/run in sequence after arrival.

A newly created or recreated context cannot be probed pre-creation and therefore conservatively guards/runs on that day. That is value loss, not unsoundness. The open cross-incarnation-correlation problem is also disclosed rather than silently assumed. See [28Q, lines 197–213](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:197) and [236–250](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/xchk-sol-a/Research/plans/28Q-context-kernel-unification.md:236).

Confidence: ~SUSPECT because the authored event algebra is still absent, but the abstract trajectory itself is coherent.

### The sweeping plan does not, by itself, violate gradual enhancement

No finding.

The root docs warn that wholesale rewrites kill projects, but `28Q` splits the work into stages with conservative byte-identity gates. That is a reasonable mitigation. The real defects are incomplete gates and missing prerequisites identified above, not the mere fact that the conceptual model is broad.

Confidence: +SURE.

### Reusing a common “context” vocabulary is not itself an abstraction error

No finding.

The load plane and world plane have materially different evidence and failure rules, so the “ONE discipline” rhetoric is stronger than the actual commonality. Nevertheless, both can legitimately share coordinates, scoped regions and piecewise-constant validity without sharing implementations or trust classes. I found no contradiction merely from giving them a common conceptual interface.

Confidence: ~SUSPECT; the analogy should not be permitted to erase the host-partition and custody distinctions identified above.