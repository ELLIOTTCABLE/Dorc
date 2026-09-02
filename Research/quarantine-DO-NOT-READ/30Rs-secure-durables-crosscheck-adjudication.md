# 30Rs — secure-durables crosscheck adjudication

> Tier: quarantined conductor adjudication. Review target:
> `7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
> Current-tree disposition was checked after the later 30V why-surface work, because a repair lane
> will start from current `ai/main`, not from the reviewed historical tip.
>
> Inputs: `30Rm` Fable-N, `30Rn` Fable-A, `30Ro` Sol-N, `30Rp` Sol-A. The two Kimi lanes never
> ran because their harness failed; no Kimi output influenced this document. This is widened
> coverage, not calibration to truth. Agreement is a signal; every finding below remains a claim
> about the artifact and its governing design, never a model vote.
>
> **Unattended builder order if this work resumes:** (1) before the testing-architecture rebuild,
> one CLI/apply builder repairs receipt opt-out, durable-result reporting, and livetest vocabulary;
> (2) after that rebuild settles, one authority builder closes publication/identity mints, entropy
> failure, and fixture-clock production reach while preserving deterministic construction; (3)
> after that, one durable-edge builder repairs verification-only recovery, key-buffer custody,
> partial narration, and handle-relative local I/O. This is scheduling, not ratification of the
> in-flight testing design. Human-input items remain outside all three.

## verdict-in-one-screen

The receipt **format kernel is worth keeping**. All four reviewers found substantial, unusually
careful work in exact-byte signing, strict grammar, plain/rich separation, reverse-overlay
accounting, apply-image fidelity, recorded/live separation, crypto-role separation, deterministic
fault injection, and immutable-store basics. No reviewer found a semantic-signature gap, a
rich-to-plain downgrade, a partial-overlay release, a recorded-to-live authority conversion, or a
custom-crypto mistake.

The integrated product at `4414af7a` was nevertheless not closeable. The dominant signal was not
inside cryptography; it was at composition seams where locally strong types were fed facts they had
not established, or where the completed subsystem did not serve the user journey it replaced.
Later 30V work repaired the largest user-visible regressions — receipt-rooted why, explicit-file
selection, line addressing, rooted closure, current-source comparison, help/steering recasts — so
those must not be re-fixed. The current tree still carries several unambiguous implementation
breaches and several larger design debts.

**Closure verdict: NACK pending two different classes of work.**

1. **Fix now without product-direction debate:** authority/API holes, failure propagation,
   partial-recovery loss, fixture leakage, secret-buffer discipline, and local-handle continuity.
   These contradict already-ruled effects; the repair question is how, not whether.
2. **Return to the human before building:** genuine transport standup, mandatory durability versus
   the shell floor, plan/apply correlation, outcome observability, receipt-store lifecycle,
   no-observation identity, durable-content growth, and the intentionally temporary total why
   surface. These alter product behavior, durable contents, or high-lock architecture.

## method-and-weighting

Provenance labels used below:

- **F-N** — `30Rm`, Fable, disowned stance, security excluded.
- **F-A** — `30Rn`, Fable, skeptical stance, security excluded.
- **S-N** — `30Ro`, Sol, broad stance with quarantined design access.
- **S-A** — `30Rp`, Sol, hostile stance with quarantined design access.

Mechanical claims are accepted as reported, per conductor direction. The conductor independently
re-grounded in `30R`, `30Ra`, `30Rb`, `30Rd`, the receipt/crypto/local crate laws, the CLI/plan
boundary laws, and verified-core discipline. No ocean-scale code rereview or new test writing was
performed. A small current-tree census checked only whether 30V had already repaired review-tip
findings.

Weighting:

- Cross-lineage agreement receives the strongest priority.
- Same-lineage Fable agreement is strong capability evidence but not decorrelation.
- Sol-only or Fable-only findings survive when mechanically demonstrated and directly contradict a
  ruled invariant; they are labeled solitary rather than laundered into consensus.
- Severity words in the source reports are not inherited. Sol tends to overstate severity; the
  categories below are conductor judgments.

## fix-now-work

These items are suitable for bounded repair work. None requires the human to choose a new product
posture. Changes touching governed receipt/publication surfaces still follow the existing
quarantined-review process before fold; that is validation of the repair, not a request to reopen
whether the defect should be fixed.

A testing-architecture rebuild is in flight but not settled here. This adjudication does not ratify
or depend on its prospective shape. Scheduling below merely avoids making security repairs against
fixture/composition seams that are actively being replaced, and preserves one requirement whatever
that architecture becomes: closing production authority must not remove deterministic, granular
construction at test time.

### P0 — authority and identity boundaries

#### fix-required-publication-mint

**Sources:** S-N `30Ro:required-publication-proof-is-caller-mintable`; related S-A concern about
self-certified authority.

`RequiredPlacementLanding::of` remains a public primitive that a caller-supplied placement closure
can return into `publish_through`; matching caller-known bytes is not proof that required local
publication happened. This contradicts the explicit no-fixture-bypass and private production-proof
claims. The current production caller uses the real store, but the advertised type invariant is
false and a future caller can compile the bypass.

**Repair:** make production permit minting consume an unforgeable result of the required local
publication edge. Preserve deterministic testing of the semantic transition without letting fixture
code manufacture production authority from digest and policy strings. The exact harness/test-support
shape is deliberately not ruled here. Add the external/caller negative proof the design already
requires.

#### fix-entropy-failure-must-refuse

**Source:** S-N `30Ro:entropy-failure-mints-fixed-zero-identities` (solitary, mechanically traced).

Entropy failure still returns an all-zero receipt identity and sets a separate `intact` latch that
production callers never consume. This lets degraded production persistence and apply identities
enter the fixed-ID fixture class.

**Repair:** make the identity mint fallible and propagate refusal before receipt publication,
session/intent construction, or permit minting. Remove the remember-to-check latch and narrow raw
identity constructors to parsing or structural test-only use.

#### fix-draft-and-identity-mints

**Source:** S-N `30Ro:receipt-draft-mint-is-an-open-field-bag` (solitary, direct API finding).

Public skeleton fields, broad draft construction, raw identity constructors, and write-edge access
let a future internal caller sign controller-authenticated but semantically invented history without
passing an authoritative species projector. This does not itself mint mutation authority; it breaks
the historical-account boundary the receipt exists to provide.

**Repair:** close semantic draft construction behind species projectors/producer facades; separate
parser reconstruction from live controller mints; retain deliberate deterministic construction for
tests behind a boundary that cannot enter production. The exact test-support mechanism is not ruled
here.

### P0 — apply CLI honesty and durable failure

#### fix-apply-receipt-opt-out

**Sources:** F-A `30Rn:fnd-no-receipt-is-ignored-by-apply`; S-N
`30Ro:no-receipt-is-ignored-for-host-apply`; S-A `30Rp:fnd-receipt-disable-is-ignored`.

Three reviewers independently found the same concrete behavior: remote apply accepts
`--no-receipt`, then writes rich intent/outcome receipts anyway. V1 rules no bypass, so the correct
current behavior is a pre-I/O typed refusal, not mutation without an intent and not silent
persistence.

**Repair:** reject `--no-receipt` for required-publication apply before key/store/transport work,
with one registry code explaining the incompatibility. Do not design a bypass in this lane.

#### fix-apply-durable-reporting

**Source:** F-N `fnd-apply-route-swallows-durable-failure-and-never-names-its-receipts` (solitary,
production consumer traced).

The apply composition produces intent, outcome, and `durable_failure`, but the production consumer
uses only the shipped result. Post-dispatch durable failure therefore satisfies the important
“continue execution” half while violating the equally ruled “report it” half. Pre-dispatch failures
also collapse useful closed reasons into `intent-not-published`.

**Repair:** retain and route intent/outcome identities and durable failure through the normal output
model; preserve typed pre-dispatch root/key/store reason and store locus. This lane supplies data and
codes only where prose is absent; it does not author user-facing words.

#### fix-retired-livetest-flag

**Sources:** F-N `fnd-livetest-passes-a-retired-flag`; current-tree census confirms both call sites
remain.

`spike/e2e/livetest.sh` still invokes `--no-whylog`; the parser now accepts
`--no-receipt`. This breaks the human-gated live acceptance tool.

**Repair:** update the two invocations to the ruled current behavior. Where apply must retain
required receipts, omit the flag rather than replacing it with a flag that is refused there.

### P1 — partial recovery and value exits

#### fix-verification-only-skeleton-recovery

**Sources:** S-N `30Ro:missing-age-key-hides-authenticated-skeleton`; S-A
`30Rp:fnd-verification-only-recovery-disappears`.

The keyset correctly represents verification-ready without an Age opener, but the next CLI seam
turns every such rich receipt into `RegionUnopenable` before signature/skeleton recovery. Losing
detail custody unnecessarily loses authenticated structural history and its exact cause.

**Repair:** retain a signature-checked rich-skeleton/report state with unavailable detail and the
specific encryption-role reason. It must never release opaque slots or impersonate a complete rich
receipt.

#### fix-private-key-buffer-discipline

**Source:** S-N `30Ro:private-key-write-callback-extracts-owned-bytes` (solitary, API and production
caller traced).

The generic private-key callback can return arbitrary owned data, and production copies both key
documents into ordinary non-zeroizing vectors. That is broader than the claimed scoped persistence
exit.

**Repair:** narrow the callback to a persistence/status operation and hold unavoidable owned copies
in zeroizing containers. Add a negative API test for returning/capturing owned key material, not
only escaping a borrowed slice.

#### fix-partial-publication-narration

**Sources:** F-N `fnd-interrupted-publication-bricks-last-selection`; F-A
`fnd-store-lifecycle-edges`; S-A lifecycle analysis.

Direct-final publication intentionally leaves bounded prefix files after interruption and correctly
refuses fallback to older history. At the reviewed tip, production discarded partial/read reasons,
then reported `no-receipt` or an unreadable root without naming the damaged newest file. 30V fixed
rooted rendering but did not establish identity-conditioned cleanup.

**Repair:** preserve the partial/root reason through acquisition and report the selected damaged
entry truthfully. Do not add name-based cleanup or fallback to older history. If the current
reconstruction already carries the state, close only the remaining acquisition loss.

### P1 — local filesystem capability continuity

#### fix-created-directory-handle-continuity

**Source:** S-N `30Ro:fresh-keyset-children-are-created-by-path` (solitary, clean-profile path
traced).

Freshly created directories are recorded by pathname/kind but not immediately retained as handles;
child creation can therefore fall back to absolute path traversal before validation/open. This
contradicts the ruled created-component ownership chain.

**Repair:** successful directory creation returns or immediately acquires an inspected,
non-following owned directory handle before any child operation.

#### fix-handle-relative-enumeration

**Source:** S-A `30Rp:fnd-directory-handle-is-abandoned` (solitary, distinct operation traced).

Enumeration uses `read_dir(path)` after the root handle was validated. The first-use empty-store
check and normal store walk can therefore observe a replacement directory rather than the object
whose handle authorizes the operation.

**Repair:** enumerate relative to the retained directory capability and bind entry tokens to that
parent. If a platform cannot safely express the authority-bearing empty-store check, refuse rather
than silently fall back to a pathname.

### P1 — fixture/production separation

#### fix-fixture-clock-publication-fence

**Source:** F-N `fnd-fixture-clock-reaches-published-order` (solitary, current-tree census confirms
reachability).

`DORC_FIXTURE_CLOCK_MS` remains honored by the production clock path and now determines published
receipt ordering. Fixed fixture state has crossed into default persistence, exactly the production
fence trigger.

**Repair:** structurally exclude fixture-clock selection from production publication while
preserving an explicit deterministic clock input for granular tests. A malformed fixture value must
not silently make a production run clockless. The testing-architecture work owns the eventual
composition spelling; this finding owns only the production exclusion and continued injectability.

### P2 — straightforward hygiene

#### fix-current-steering-residue

**Sources:** both Fable reviews; current-tree census.

30V repaired the large stale set and rewrote the help/register surface. A small alias residue remains
in `spike/CLAUDE.md` and `cli/CLAUDE.md`. The broad historical findings are **already fixed** and
must not be replayed wholesale.

**Repair:** inspect only the current remaining aliases/citations and make them truthful; retain
necessary née references where they intentionally explain a rename.

## human-input-work

These items should not enter the mechanical repair lane. Each changes product posture, requires a
new capability, changes durable contents/identity, or activates an explicitly deferred design.

### H0 — genuine standup before apply authority

#### ask-real-session-standup

**Sources:** S-N `30Ro:apply-ready-target-is-only-an-argv-spelling`; S-A
`30Rp:fnd-standup-is-self-certified`.

Both Sol reviewers independently found that `ReadyApplyTarget` is minted from the host argv plus
“not established” context axes before the transport driver exists. This directly violates `30Rb`'s
sentence forbidding a fake ready target. The affine publication chain can only preserve the truth of
its premise; today that premise is caller-authored.

**Human decision required:** either fund a genuine transport standup that returns immutable resolved
target/session/context and is retained through dispatch, or make the current one-shot SSH route
refuse required apply. This is not repaired by renaming the thin context or adding another check.

### H0 — mandatory durability versus the shell floor

#### ask-audit-gate-versus-availability

**Sources:** F-A `fnd-apply-is-gated-on-a-per-user-profile`; S-A
`30Rp:fnd-mandatory-durable-breaks-shell-floor`.

The implementation accurately reflects an ACKed design: apply refuses when controller-local key or
store durability is unavailable. That creates a real conflict with the root product's “no worse than
plain sh” and incident-response floor. Read-only/full home directories, unavailable XDG roots,
incomplete keysets, or filesystem sync failure can block otherwise viable remediation.

**Human decision required:** explicitly retain the audit-dependent apply product and narrow the shell
floor promise, or design a separately consented raw-apply posture that makes no receipt/publication
claim. Do not sneak a bypass into the current authority chain.

### H0 — plan/apply causal correlation

#### ask-explicit-plan-origin-handoff

**Source:** S-A `30Rp:fnd-normal-plan-apply-never-correlates` (solitary, normal path traced).

The normal `plan > plan.sh; apply --plan plan.sh` path records `PendingOrigins::Unavailable`; outcome
reaches intent but not the originating plan receipt. The graph's flagship causal chain therefore
is absent on the primary workflow.

**Human decision required:** design an explicit, non-inferential handoff binding final plan bytes,
presentation identity, receipt identity, and admin edits while preserving the ruled M:N relation.
Filename guessing or automatic “same bytes” inference is not enough.

### H1 — no-observation run identity

#### ask-last-run-misattribution

**Source:** F-A `fnd-no-observation-plan-writes-nothing-silently` (solitary but red-test demonstrated).

A completed no-observation plan writes no receipt and says nothing; the next bare `dorc why` selects
an older receipt, even though the just-completed plan told the user to ask why about this book. This
is the project's worst aid class: confidently answering about another run.

**Human decision required:** choose whether every completed run gets a thin receipt, or whether a
run without a receipt explicitly suppresses/qualifies the “last run” advice and creates an
unambiguous no-receipt moment. The safe immediate floor is to stop pointing at an older answer; the
stable identity semantics need a ruling.

### H1 — outcome observability

#### ask-whole-script-site-outcomes

**Source:** S-N `30Ro:apply-outcome-never-records-sites`; echoed by the wider Fable concern that apply
never tells the operator what its receipts mean.

Production always supplies an empty site population although the durable model supports site
outcomes. External whole-script execution may genuinely lack per-site observability, so simply
filling the vector is not a mechanical fix.

**Human decision required:** choose instrumentation capable of per-site outcome reporting, or mint a
typed explicit state distinguishing “unobservable whole script” from “zero sites” and “no site ran.”

### H1 — store lifecycle and retention

#### ask-bounded-store-lifecycle

**Sources:** F-A `fnd-store-lifecycle-edges`; S-A `30Rp:fnd-immutable-store-has-hard-expiry`; F-N
interrupted-publication analysis.

The immutable default-on store has no retention and bounded enumeration. Publication continues while
store-based reading eventually refuses above the entry bound; unknown/conflict/partial files
accelerate the cliff. The design explicitly deferred retention, but a default durable that disables
its own recovery surface is not a closeable product state.

**Human decision required:** pick a bounded lifecycle: reviewed retention/archival, sharded/indexed
immutable discovery, or direct ID/file paths independent of whole-store enumeration. Never silently
delete history. The scout's later “fixed” classification confused byte bounds with the still-live
entry-count problem; the reviewers' mechanical account controls here.

### H1 — default why selection and sensitive detail

#### ask-temporary-total-surface-exit

**Sources:** S-A `30Rp:fnd-default-why-dumps-sensitive-detail`; both Fables' broader why regression.
Current status: 30V **fixed** the old listing, explicit file, address, and rooted closure, but
intentionally installed a total temporary surface. Current receipt `why` still receives no depth and
`--all` is byte-identical; every datum, including encoded opaque values, reaches the total render.

**Human decision required:** move from the temporary total register to goal-derived selection under
`AID-NEEDS:law-selection-is-goal-derived`; make `--all` the labeled exhaustive tier. Selection must
precede sink encoding. This is user-aid design/prose work, not a security hotfix that simply hides
fields.

### H1 — durable contents and decision identity

#### ask-recorded-consent-and-absent-scalars

**Sources:** F-N `fnd-argv-uncollected-drops-the-consent-link` and
`fnd-skeleton-scalars-fabricated-not-absent`.

The durable cannot reconstruct the admin's `--risk-faultless-skips` consent link, while several
skeleton zeros currently mean “not collected” rather than measured zero. Both are real historical
truth problems. Correcting them changes receipt contents/grammar and therefore clears
`rul-durable-contents-reviewed-before-design` first.

**Human decision required:** decide the minimal stable controller-authored policy fields that belong
in the receipt and replace fabricated zeros with typed absence where the producer lacks data. Do not
smuggle this into an ordinary projection cleanup.

#### ask-planning-identity-completeness

**Source:** S-N `30Ro:planning-input-identity-omits-live-controls` (solitary, input/identity tables
compared).

Escalation and connection capability can affect planning but do not enter `PlanningInputId`; the
unreleased controller semantics token is also constant across behavior-changing builds.

**Human/opaque input required:** settle the complete closed analysis-control identity and controller
semantics versioning. This is a decision-identity governed surface, not a local hash-field addition.

### H2 — broader cost and usability consequences

#### consider-receipt-complexity-budget

**Sources:** both Fables quantified code/dependency growth; F-A/F-N and S-A noted that the old why
surface shrank at the review tip. 30V recovered the surface, so the strongest “dead report module”
claim is historical. The remaining fact is still material: the family adds a large custom grammar,
crypto/provider lifecycle, many dependencies, and operational key/store state to a tool whose value
rests on invisibility and easy off-ramp.

**Consideration:** do not rip out the format kernel on this evidence; it survived every deep attack.
Do require future receipt work to delete superseded pathways and justify dependencies/variants
against reachable product behavior. Provider expansion, import/rotation, and retention should not
proceed as automatic completion of the existing architecture.

#### consider-readable-structure-limits

**Source:** F-A `fnd-receipts-are-legible-only-where-written`.

The directly readable skeleton is structurally inspectable but most human-useful material is in the
Age overlay; V1 has no import/key-transfer UX. This is a ruled trade, not an implementation breach.
Record it when specifying vendor handoff, old-machine recovery, backup, and issue-attachment claims;
do not market direct readability as full stand-alone legibility.

## already-fixed-after-review-tip

The following findings were correct at `4414af7a` and useful — they exposed bad delivery sequencing —
but later 30V work repaired them on current `ai/main`. The fix lane must not recreate their patches.

- **Receipt why was only an inventory** — F-N/F-A, with S-A support. Current
  `report_recorded_store` reconstructs and renders the total surface.
- **`--receipt <file>` did nothing** — all four reviewers converged. Current
  `root_from_file` opens the exact named file.
- **Receipt address was ignored** — both Fables. Current `named_address` feeds source comparison.
- **Whole-store graph was appended to rooted output** — both Sols. Current output uses
  `closure_from(root)`; store walking remains discovery.
- **Most stale help/steering text and `[unwritten:]` receipt option rows** — both Fables. 30V's
  conductor prose/steering pass repaired the broad set; only narrow residue belongs in fix-now.
- **`RecordedWhyFacts` had no production consumer** — both Fables/S-A. It is now the reconstruction
  input and was widened exhaustively over persisted plan families.

These repairs do **not** dispose `ask-temporary-total-surface-exit`: current output is now a
correctly rooted total reconstruction, but it is intentionally uncurated and depth-insensitive.

## rejected-or-non-action-findings

- **Delete the crypto/format core** — rejected. Every reviewer found the central exact-byte,
  projection, overlay, and recorded/live properties strong; no counterexample survived.
- **Automatic fallback to an older receipt** — rejected. Newest damaged/partial state must remain
  visible; fallback would hide the event. Repair narration/acquisition instead.
- **Name-based cleanup of partial files** — rejected. The governed local-store design deliberately
  chooses leaked incomplete evidence over deleting a replacement object.
- **Windows parity as if equivalent to Unix** — not found. The implementation states the weaker
  Windows baseline and keeps platform properties typed separately.
- **Recorded receipts authorizing live action** — not found by any reviewer.
- **Crypto role aliasing or in-file algorithm negotiation** — not found.
- **Parser allocation-first or permissive grammar** — no supported finding; ordinary tests and
  mutation/vector corpora are strong.
- **Merge-time bless/refactor shape** — F-N process finding is valid history hygiene, but no current
  product repair follows. Preserve it as a lesson for future folds.
- **Bulk lexical fences** — no cleanup directive. Current steering says existing fences stand;
  future ones require human-ack value, not reflexive growth.
- **Real-profile keys/receipts and orphan old whylogs** — likely harness-era residue, not a product
  code finding. Human owns inspection/removal; no agent cleanup.
- **Quarantined rationale cited from ordinary code** — real maintainability friction but not a
  security defect. Restate non-sensitive engineering reasons locally when those sites are next
  touched; do not bulk-copy quarantined rationale.

## unattended-fix-schedule

The unattended work should not be one builder or one concurrent fan-out. Three cohesive dispatch
arcs keep each builder's context useful and avoid rebuilding temporary fixture infrastructure.

### dispatch-before-testing-architecture — CLI contract and reporting

**One builder; small-to-medium; dispatch now.** This arc is intentionally limited to behavior the
ongoing suite work should inherit rather than mechanisms it is replacing:

1. `fix-apply-receipt-opt-out` — reject the incompatible flag before I/O.
2. `fix-apply-durable-reporting` — preserve intent/outcome identity and typed durable failures in the
   production output model.
3. `fix-retired-livetest-flag` — repair the live acceptance invocation to the ruled current CLI.

These three share the remote-apply/CLI boundary and can be carried together. The builder may add
focused Rust assertions and update the existing live script, but should avoid restructuring test
composition or minting a new fixture path. Landing this first gives the test-architecture work the
correct product behavior to preserve.

### dispatch-after-testing-architecture-a — authority construction and deterministic inputs

**One strong builder; medium-large; serial after the testing-architecture work settles.** Re-scout
current names first because some fixture-clock plumbing may already have moved.

1. `fix-required-publication-mint`.
2. `fix-entropy-failure-must-refuse`.
3. `fix-draft-and-identity-mints`.
4. `fix-fixture-clock-publication-fence`.

These are one question: which constructors and injected values may reach production authority while
remaining deterministically constructible in tests. Doing them before the active suite work would
force the repair to target seams known to be moving. Doing them together lets one builder close the
production API without independently breaking receipt IDs, publication DST, and clock-controlled
fixtures.

### dispatch-after-testing-architecture-b — durable recovery and local object custody

**One strong builder; large but cohesive; serial after arc A to avoid CLI/receipt conflicts.**

1. `fix-verification-only-skeleton-recovery`.
2. `fix-private-key-buffer-discipline`.
3. `fix-partial-publication-narration`.
4. `fix-created-directory-handle-continuity`.
5. `fix-handle-relative-enumeration`.

All five live on the receipt-crypto → receipt-local → CLI recovery path and should be proved against
one shared deterministic I/O/state model plus focused native checks. Splitting them would make
several builders repeatedly load the same keyset/store state machine and risks different notions of
partial recovery or object ownership.

`fix-current-steering-residue` is not a builder arc. Recheck it once the test-architecture and both
post arcs have settled, then make one conductor edit over the current truth; editing it earlier would
create immediate steering churn.

Across all three arcs:

- Reviewer red evidence may be adopted only after confirming it isolates current behavior; it is not
  automatically a regression suite.
- No arc may implement transport standup, a durability bypass, plan-origin inference, per-site
  executor instrumentation, retention, durable content additions, identity expansion, or why
  curation without the corresponding human decision above.
- If closing publication authority appears to require a new product posture or to make deterministic
  construction materially worse, STOP rather than replacing the problem with a production-only path
  or another lexical roster.
- Each builder ends with the ordinary both-platform completion gate; governed receipt changes receive
  their required review before fold. A finding is never fixed by weakening its test question.

## final-priority-order

1. **Before the suite rebuild:** repair the remote-apply CLI contract and reporting so the rebuilt
   tests target truthful behavior.
2. **After the suite rebuild:** close authority/identity/input boundaries without sacrificing
   deterministic construction.
3. **Then:** repair partial recovery and local object continuity against the settled test seams.
4. **With human input later:** genuine standup, durability availability, plan/apply origin, site
   outcomes, store lifecycle, no-observation identity, durable contents, and why curation.
5. **Only after those:** expand providers, rotation, import, policy profiles, or stronger platform
   claims.

The work should not be called closed merely because the strongest local kernels held. Their purpose
is to support an honest, available, causal explanation product. The current tree is materially
better than the reviewed tip because 30V repaired the delivery-sequencing failures, but closure
still depends on these unattended arcs plus the explicitly human-owned rulings above.
