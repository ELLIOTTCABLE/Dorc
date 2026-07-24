# 29A - Round 29 catch-up ledger

The current status surface for round 29, replacing `298`'s stale implementation ledger
and `299`'s broken-relay disposition. Written 2026-07-24 by the security conductor, which
sits OUTSIDE the Fable firewall and may read every quarantined document directly.

Authority: the root docs, `spike/CLAUDE.md`, and human-typed rulings outrank this file.
`290`-`295` remain the review record; `297-security-refresh-build-plan.md` remains the
build plan and `297-security-refresh-phase-packets.md` the frozen per-phase specs.
Ahistorical and kept-current: if this document is wrong, rewrite it.

Status hygiene is unchanged from `298`: `Research/LIVING_STATUS.md`, `Research/README.md`,
and the other ordinary status documents stay untouched by round 29. Ordinary-engineering
*constraints* do leave quarantine (that is `295`'s export process, discharged in this pass
- see section 6); round *status* does not.

## 1 - Corrected round state

The gate is recovered. `29-reviewA` returned **ACK** over `49b66421..b6fde355`: zero
qualifying concerns, no new or revealed hidden invariant, two `~SUSPECT` residues recorded
as fenced. Only the relay failed. The four immediate ledger rows are ACCEPTED. Full
correction and the opaque-review waiver: `299`.

`rul-no-opaque-review-for-security-rounds` (human-typed 2026-07-24) - opaque accrual
review is neither necessary nor wanted for round-29 material. The reviewer's purpose is
reading NON-security work for security-critical findings; aiming it at a security round's
own output inverts that, and the attempt spent budget on a model unsuited to the material.
Scoped to the security lane and its builders. Deliberately recorded only in quarantine, so
that no out-quarantine conductor reads it as license covering their own work.

Phase state at `ai/main`@`fbbf88f1`, verified in-tree rather than from the handoffs. One row has
moved since: phase one landed on `ai/r29-drep-repair`, awaiting the fold.

| phase | state | evidence |
|---|---|---|
| 0 - seam map and freeze | partial | map and packets exist; the phase-zero item-4 compile-fail set was never built (see `fnd-compile-fail-set-collapses`) |
| 1 - owned report channel | landed, complete | the drain runs on a per-attempt `mkdir -m 700` scratch directory rooted at a controller literal; create-failure empties the guard and the lane degrades to `/dev/null`; cleanup is per-file `rm -f` plus an empty-only `rmdir`. Pinned by `emitting_auto_cell_owns_every_path_it_writes` (built per the section-4 spec, each of its properties proven falsifiable) |
| 2 - every establish vouched | landed, complete | private non-empty `AllEstablishesVouched{head,tail}`; ordered site/fact identity, cardinality, duplicate rejection; distinct `ReadSubstitutionProof`; per-establish `EstablishVouchReceipt` loci |
| 3 - bounded attributed ingress | landed | all seven `HostEvidenceLimits` bounds; byte-first `take(limit+1)`; closed `Admission`/`AdmissionRefusal`; refusal precedes plan, artifact, and whylog; independent inner whylog budget; controller-minted `Framing` |
| 4 - sink encoding and artifacts | not begun | `sanitize_report_raw` in `cli/main.rs` is still the only sanitizer |
| 5 - production fences | not begun | one item fell out incidentally: no `DORC_ALLOW_LEGACY_RESULTS` env read survives |

`rec-native-gate-on-shipped-revision` - the whole gate set ran natively on
`ai/main`@`fbbf88f1` on 2026-07-24: workspace build, workspace tests, `sh e2e/run.sh`
97/97 with real `dash -n` and exec-under-mocks, `cargo fmt --check`, cold
`clippy -D warnings` (fresh target dir, so not a stale incremental), `cargo deny`, and
`typos`. This supersedes `299`'s WSL-only evidence and its "native shell unavailable"
claim: `sh` and `dash` are both present in the Git Bash environment. It is also a
*stronger* result than the round ever had, because `299`'s evidence covered `b6fde355`
and this covers what is actually shipped.

## 2 - Findings from the catch-up scout

`fnd-ack-does-not-bind-shipped-bytes` - `ai/main` is nine commits past the reviewed
`b6fde355`, and one of them (`5d74bfcc`) folded round 29 into the phase-three-close line,
touching exactly the security-relevant files: +209/-41 across `cli/main.rs`,
`plan/whylog.rs`, `plan/lib.rs`. The ACK therefore certifies a revision that is not the
one running. Both substantive post-ACK changes were traced by hand in this pass and are
recorded below; the conclusion is that nothing in the delta re-opens a host-ingress hole,
but the delta was never gated and the two findings under it are real.

`fnd-whylog-inspect-reopens-raw-parse` - `whylog::inspect(Option<&str>, ...)`, added
post-ACK, is public and calls `whylog::parse` on raw unbounded `&str`. That is precisely
the surface `298` listed as an unrepaired phase-three violation, re-instated as a new
consumer. Present reachability is `dorc-loom/src/consumer.rs` only, over repo-local
materialized case fixtures - controller-local tooling, not managed-host bytes - so it is
not presently a hole. It is the sibling-route class the phase-zero map exists to prevent,
and it will become one the moment a host-sourced durable reaches it. Disposition this
pass: doc-comment fence plus this record; the type change is deferred because
`dorc-loom` is mid-restructure under `plans/288` and editing under that lane costs more
than it buys. Re-entry: fence properly when the aid extraction settles, or immediately if
any host-sourced path acquires a call.

`fnd-duplicated-fnv-digest` - `book_digest` (FNV-1a-64) is a deliberate spike
drift-detector, explicitly not adversarial identity, and `sinv-production-fences` wants
exactly one named production substitution point. `plan/invocation.rs` is that point and
says so; `cli/main.rs` is a documented thin delegate to it. The post-ACK whylog change
re-inlined a second, byte-identical implementation locally, which is the opposite of one
substitution point. FIXED in this pass (the duplicate deleted, the canonical one imported
with a note); no behavior change, since the two were the same function.

`fnd-export-step-never-ran` - `295`'s export process is mandatory and had not happened:
`spike/CLAUDE.md` and all seven crate steering files contained zero round-29 entries.
The repaired invariants existed only inside quarantine, which the ordinary conductor
structurally cannot read - so nothing in ordinary law stopped a future builder from
re-opening either repaired finding. Discharged in this pass (section 6).

`fnd-steering-asserts-a-dead-lane` - three documents asserted that the `27W` report
lane's runtime drain was built and working, which phase one made false:
`spike/CLAUDE.md`'s build-status block, and `AID-NEEDS.md`'s `aid-refusal-breadcrumbs`
and `aid-authored-decline-classes` rows. `spike/docs/reference/oracle-contract.md` 6a
was already correct and served as the model. Discharged in this pass.

`fnd-gate-two-refuses-the-drained-render` - `e2e/scan_redirects.awk` (gate-2, the pre-exec
redirection sandbox) refuses any redirect whose target word contains `$`, and it runs on BOTH
rendered artifacts before the exec gates. The drained probe carries three such lines: the
scaffold's `: >"$DREP_V1"` and `<"$DREP_V1"`, and - independently of anything the engine emits -
the ORACLE's own authored `>>"${DREP_V1:-/dev/null}"`, which is the contract's fixed spelling.
So the refusal predates this repair and is not caused by it: any e2e case shipping an
emitting verdict body in a RESOLVABLE probe would have hard-failed gate-2 before the disable
too, which is presumably why no such case was ever authored. The consequence is that the
tier-3 lane has no behavioural e2e coverage and cannot get any until gate-2 learns to accept
the engine-supplied sink value. That is a harness change - out of `29B`'s scope, and it needs
care, because the scanner's conservatism is itself a safety property. Verified by hand on WSL2
Linux instead (section 4b), which is evidence, not a gate. Whoever opens the harness next
should treat "allow exactly `"$DREP_V1"`, `"${DREP_V1:-/dev/null}"`, and `"$_dsc"`-rooted
targets" as the shape to consider - the same closed allowlist the unit pin uses - never a
blanket relaxation of the dynamic-target rule.

`fnd-attribution-scope-carried-not-consumed` - `WidthOneAttemptScope` is minted, attached
to admitted evidence via `ScopedHostEvidence`, and then never checked against anything;
`retain()` is a no-op that borrows its own fields to keep the compiler quiet, and the CLI
does `let _scope = scoped_results.scope();`. This is honest scaffolding at width one - no
second scope can exist, so there is nothing to compare - and `29-reviewA` accepted it on
exactly that reasoning. It becomes real work at the first transport, concurrency, retry,
cache, or cross-host boundary, which is where `sinv-controller-attribution` re-enters. Not
a defect today; recorded so it is not mistaken for a built enforcement.

`fnd-compile-fail-set-collapses` - `297` phase-zero item 4 lists four compile-fail proofs.
Inspected against the tree, two name types that do not exist yet (sink types are phase
four; a production-capable attempt context is phase five) and one already exists in
`core/src/evidence.rs` and `core/src/room.rs`. The honestly buildable subset today is the
mutation-vouch proof, plus one genuinely useful addition the plan did not name: that
`AllEstablishesVouched` cannot be constructed from outside `plan`. Built in this pass at
that scope, and stated as that scope rather than reported as four-of-four.

## 3 - Standing residues the ACK fenced

Recorded so they are not rediscovered as findings. `LegacyPolicy::Tolerate` remains
reachable in principle though production reads pass `Refuse` and no ambient environment
grants it; `records::deframe` and `whylog::parse` remain public over raw input; the
width-one identity types remain fixture-grade. All three are phase-five work, all three
are local and rediscoverable, and none of them satisfied both halves of the accrual
threshold in the reviewed delta.

## 4 - The report lane: what broke, and the specified repair

Phase one disabled runtime report capture rather than repairing it. That was correct
under `297` given a pathname protocol, but it left the `27W` lane's third tier unfed and
- until this document - unspecified, reading as an abandoned feature rather than a
deferred one. It is deferred. The design below is the successor to phase one and should
be read as its completion, not as a new proposal.

The out-of-band requirement that produced the file mechanism is not negotiable and was
not the error (human, 2026-07-24): stdout, stderr, and rc all carry existing semantics
owned by whatever the oracle delegates to, and cannot be redefined for all oracles
forever. A per-invocation file, orthogonal to the standard streams, with no muxing,
demuxing, parsing, or collision, is the correct shape. The error was narrower.

### 4a - The primitive, separated from the mechanism

Three primitives were handed to the managed host, none of them inherent to
out-of-band-via-file:

1. `: > "$f"` truncated a path Dorc did not own - through a pre-positioned symlink, that
   is arbitrary-file-destruction during the phase that promises no mutation;
2. the read-back accepted attacker-substituted content as author breadcrumbs;
3. cleanup by name could unlink an attacker-chosen file, and opened a second race.

The oracle's own `>>` append is not on that list: appending to a path Dorc supplied is no
new primitive, since the author already executes arbitrary shell there. The repair target
is therefore narrow - Dorc must stop performing create, truncate, read, or unlink on a
name it does not own - and does not require abandoning files.

### 4b - `rul-probe-scratch-is-exclusively-created`

POSIX `mkdir` is an exclusive-create primitive available in bare sh, and it does not
resolve a symlink at the final component: if anything exists at the path - file,
directory, symlink, dangling symlink - it fails `EEXIST`.

```sh
d=/tmp/dorc-drep.<controller-supplied-token>
mkdir -m 700 "$d" || { DREP_V1=/dev/null; }   # degrade; never retry, never unlink
```

On success Dorc created the directory, exclusively, at that instant; nothing could have
been pre-positioned, because pre-positioning would have made it fail. `-m 700` applies the
mode at creation with umask not applied, so no group- or other-writable window exists.

The consequence is a severity collapse, not a mitigation: a pre-positioned path no longer
gets Dorc to destroy a file, it gets Dorc to decline capture. That is exactly `297`'s
stated posture - setup failure disables the lane rather than simulating safety - except
the lane now usually works. Name predictability stops being an integrity concern and
becomes availability only, which is why the spike may keep a deterministic token (goldens
stay stable) and a real transport can mint an unpredictable one purely for
DoS-resistance.

`+SURE` as of the `29B` build (was `~SUSPECT`, and load-bearing): `mkdir -m 700` refuses
rather than follows, verified by fixture on Linux 6.18 (WSL2 Ubuntu, ext4-backed `/tmp`)
across five pre-positioned legs — regular file, directory, symlink to a real file, dangling
symlink, FIFO. All five returned rc 1; a canary file the symlink pointed at was byte-intact
after every leg, and the symlink was still a symlink. The nothing-there leg created. `-m 700`
was separately confirmed to apply at creation under `umask 000` (mode read back `700`), so no
group- or other-readable window exists. The behavioural legs were run against the ENGINE'S
OWN rendered probe, not a hand-written approximation: happy path drained its report record and
left no residue; the pre-positioned-symlink path still emitted its effect record, emitted no
report record, and touched neither the victim nor the symlink. Not encoded as a permanent test
(it tests the OS, and is flaky across the platforms this repo builds on) — per `29B` section 5.
Still unverified on non-Linux target families; msys cannot host the check (it copies rather
than links).

### 4c - `rul-report-file-per-invocation`

Per-site files live inside the owned directory - `$d/1`, `$d/2` - and sequential integers
are fine because the namespace is private. Dorc may now safely create, truncate, read, and
unlink *inside its own mode-700 directory*, because a non-root party cannot place a
symlink there.

The considered alternative, worth recording because it is the purer answer to the
no-muxing requirement: since the probe is strictly serial per host (`no-reorder-ever`),
one session file plus a byte offset recorded between bodies partitions the stream with
zero framing and zero parsing, at one `wc -c` fork per site. Rejected on isolation - with
a shared file, an author who writes `>` instead of `>>`, or a delegated tool that
truncates, destroys other authors' breadcrumbs from earlier sites. Per-file confines every
author's mistake to their own site. The fork cost is irrelevant against a network
round-trip.

### 4d - `rul-scratch-root-never-read-from-host`

`${TMPDIR:-/tmp}` was a second, independent hostile-input read. An attacker who sets
`TMPDIR` to a directory they own gets Dorc's directory created inside a parent they
control, and on a non-sticky parent they can then unlink-and-substitute *after* the
exclusive create, defeating the whole property.

The rule that removes the question: the scratch root is a controller-supplied literal,
never a host-environment expansion. A compiled-in default plus an admin flag when they
know better (a root-owned directory removes even the sticky-bit dependence). If
`$XDG_RUNTIME_DIR`-grade siting is ever wanted, it is a probed fact the controller decides
on, not a variable the probe expands.

The residual assumption to state in the contract rather than assume: on a world-writable
non-sticky parent, another unprivileged user can unlink and replace the directory after
creation. `/tmp` is sticky essentially everywhere; an admin-named root-owned root removes
the assumption entirely.

### 4e - `rul-report-sink-value-is-engine-supplied`

The authored spelling never changes - `>>"${DREP_V1:-/dev/null}"` - because `DREP_V1` is
a sink *value the engine supplies*, which the oracle contract already promises. The engine
may supply any of three realizations, chosen by what the execution edge can actually
establish:

1. `/dev/fd/N`, where the edge can genuinely inherit a descriptor. Strongest form: no
   namespace, nothing to race. Available today for local `dorc-run`, hostsim, and any
   in-process edge.
2. a path inside the exclusively-created directory - the remote case. Plain SSH hands the
   remote side exactly three descriptors, which is *why* the file mechanism existed and
   why it was not a mistake.
3. `/dev/null` - setup failed, or capture is off.

This is the part that changes the schedule: `299`'s "disable until an owned channel
exists" was right about pathname protocols and wrong as a long-run shape, because
realization 2 is an owned channel too. The lane is not blocked on an execution edge that
does not exist.

### 4f - `rul-report-bytes-encoded-at-re-emit`

The owned-file mechanism keeps author bytes out of any shared stream on the host side, but
they still travel back, and on the session lane they sit beside `@@dorc@@`-framed records.
The answer is not to hope no author ever types the terminal token: Dorc's scaffolding,
which reads the file, encodes at re-emit - length-prefixed or escaped - so author bytes are
structurally incapable of forging a record. That is not new work invented here; it is
`sinv-sink-encoding`, phase four, which must happen regardless.

Composition, and the reason this is cheap: exclusive-create directory (new, small) +
per-site file inside it (new, small) + bounded read-back (BUILT - `HostEvidenceLimits`,
closed grammar, typed refusal) + encoded re-emit (phase four, shared) + noise-tolerant
ingestion (BUILT). Three of five pieces exist.

### 4g - Residual cost, priced

- Availability, not integrity: pre-positioned path, unwritable parent, read-only rootfs,
  exhausted inodes all degrade capture to `/dev/null` while the plan proceeds and the
  static decline tiers continue working. The degradation mints decision-inert evidence.
- Bounded residue: a crashed run leaves a mode-700 directory. This is the residue class
  `295` already told the product to admit out loud - no intentional managed-resource
  mutation plus explicit, bounded, attributed residue - and a private temporary directory
  with one name, one owner, and one purpose is the textbook instance.
- Hostile-oracle disk fill: unbounded in principle. `ulimit -f` would also constrain the
  delegated tool, so it is not the answer. Accepted: oracle source is trusted executable
  input per `295`, and a hostile oracle has strictly worse options available.
- Root on the managed host wins. Already an accepted risk; no attestation is claimed.

### 4h - Exclusion-check cells

Run the four directions `AGENTS.md` demands. One bites.

- Other phase: identical bytes run as apply-time guards. Guards need no capture, but if
  the scaffold ships there, the same discipline applies and setup failure must never fail
  an apply. Degrade, always.
- **Entered contexts** - `rul-entry-machinery-owns-the-context-report-lane` (human-typed
  2026-07-24): the context-entry machinery, not the report renderer, owns this. In-context
  tests and setup must validate, maintain, and answer for the permissions story around
  context-specific report lanes. The mechanics that force it: a `sudo`-entered body writes
  into the directory as root, so the file is root-owned and cleanup by the ordinary
  principal fails on a sticky directory, leaving residue; and the scratch root must be
  writable by the connection principal, so an admin-named root-only root would starve the
  unentered bodies. Cleanup failure stays decision-inert regardless.
  `ask-context-reentry-for-cleanup` (human, same date, explicitly unresolved): we may be
  *forced* to perform context RE-entry purely to clean up what an entered body left. Needs
  consideration when `plans/27C`'s machinery is next opened; not answered here.
- Other user: the admin sees none of this unless it fails; the engineer's authored line is
  untouched.
- Unreliable oracle: a body that truncates its own file, emits megabytes, emits the
  terminal token, emits invalid UTF-8, or emits nothing - all confined, all already bounded
  at ingestion.

### 4i - Test matrix

Deterministic and hostsim-injectable, and it maps nearly one-for-one onto `297`'s
phase-one list, which is the tell that this is the repair that list was written for.
`mkdir` refusing a pre-existing regular file, directory, symlink, dangling symlink, and
FIFO (each: refuse-and-degrade, target byte-identical); non-writable parent; non-sticky
parent with post-create substitution; replacement between create, body, drain, and
cleanup; a root-owned file left by an entered body; a body truncating its own file; a body
exceeding each limit; a body emitting the terminal token; cleanup failure; concurrent
sessions colliding on the token; and the ordinary report-free probe rendering
byte-identically.

### 4j - Sizing, measured

Costed against the actual deletion (`adef70d3`) and the current tree, not estimated. It
is smaller than the phase-one disposition suggests, and the four things that usually make
this kind of repair expensive all turn out to be cheap here:

- **The deleted surface is one function.** `render::probe::record_scaffold_draining`, 35
  lines of `format!`-assembled sh, plus its call-site gate in `plan/src/lib.rs`. The
  ported `297-security-refresh-phase-packets.md` phase-zero map plus `adef70d3`'s diff are
  together a complete record of what to rebuild.
- **A session prologue has a natural slot.** `ProbePlan::render_sh` already opens with
  `render::probe::header()` + `records::header_line(...)`; the one-time `mkdir -m 700` and
  its degradation branch go there, and the cleanup at the sentinel. No new architecture.
- **Golden churn is ZERO cases** (measured at the build, correcting an estimate of two).
  The draining scaffold is emitted only for an `emits_report` check, so every other probe
  stays byte-identical (`empty-world-byte-identical`). The two cases that reference the sink -
  `e2e/cases/decline27-tier3-dynamic` and `e2e/cases/report27-decline-static-classed` - both
  render `sites=0`: their site is an unresolvable DECLINE, so no check ships and no scaffold
  is emitted. 97/97 passed with an empty `git status`; no bless was needed or run.
  Consequence, and the real gap: **no e2e case exercises a drained probe at all**, so
  section 5's "the happy path is covered behaviorally for free" does not hold. See
  `fnd-gate-two-refuses-the-drained-render` below for why one cannot simply be added.
- **The entered-context cell is ALREADY carved in code.** `cli/main.rs` records that
  entry-composition is out of the tier-3 drain's scope, so `emits_report` is ignored on
  the entry path. The human's ruling that the entry machinery owns the in-context lane
  therefore does not block this: the repair lands for unentered sites and leaves the
  entered carve exactly where it already sits, to be answered when `plans/27C`'s machinery
  is next opened.

What is genuinely NOT cheap, and why this still wants its own bounded lane rather than a
tail-end of a catch-up pass:

- **The pin test inverts.** `emitting_auto_cell_never_constructs_a_report_path` currently
  forbids the whole vocabulary outright (`TMPDIR`, `dorc-drep`, `DREP_V1=`, `$_drep`,
  `: >`, the read loop). The repair brings most of it back, so the test must be rewritten
  from "no report plumbing may exist" to "only the owned shape may exist" - and a
  too-permissive rewrite silently re-opens the write primitive with a green suite. That
  inversion is the one artifact in this repair that must not be got wrong, and `297`'s own
  dispatch law says a security-critical bounded piece gets a frozen packet.
- **The scratch root must be threaded, not constant.** `rul-scratch-root-never-read-from-host`
  means the renderer receives the root; it cannot read the environment and should not hard-code
  a literal it cannot vary. `records::Framing` is the right carrier - it is already the
  controller-minted per-run object holding nonce, host, and attempt - but that is a
  signature change through `render_sh` and its callers.
- **The e2e exec gate really runs the probe.** A real `mkdir` executes under the harness,
  so the sandboxed root has to be supplied there and the created directory has to land
  somewhere the fixture-safety law tolerates. This is the piece most likely to interact
  with `plans/288` phase 5, which is concurrently moving the e2e tree and retiring
  `sh e2e/run.sh`.

Estimate: the render and prologue are an afternoon; the test inversion and the harness
siting are the work. Recommended as the immediate next security lane, with a frozen packet
covering the inverted pin's exact assertions before any code moves.

## 5 - Cross-lane collisions with `plans/288`

`collide-host-evidence-is-not-narrative` - 288 section 2b renames `core/src/evidence.rs`
to `aid/src/narrative.rs` and `CollapseEvidence` to `CollapseNarrative`. Round 29 minted a
completely separate "evidence" family in `plan` and `cli`: `HostEvidenceLimits`,
`read_host_evidence`, `ScopedHostEvidence`, `AdmittedUnscopedHostRecords`, and the
quarantined `sinv-host-evidence-ingress`. Those name *hostile managed-host bytes* - the
opposite plane from narration. A grep-driven rather than module-driven rename would erase
exactly the distinction the ingress work exists to keep, and would make raw hostile input
read as narration. The phase-1 brief needs one sentence excluding the `plan`/`cli`
host-evidence family. The exported steering entry in section 6 states the distinction in
ordinary terms so the hazard survives without the quarantine.

`collide-cli-error-migration-touches-ingress` - 288 section 6 migrates CLI argument errors
and names the whylog exclusivity pair and `humane_read_error`; both sit inside the round-29
ingress path in `cli/main.rs`. Overlapping edits, not dangerous.

`collide-executor-role-moves` - 288 phase 5 retires `sh e2e/run.sh` and transfers the
one-sanctioned-fixture-executor role. Every security-lane brief's safety block cites it,
and the report-lane repair's test matrix assumes it.

## 6 - The export, and its private mapping

`295`'s export process requires each accepted finding to leave quarantine as a truthful
ordinary-engineering constraint carrying no threat exposition and no back-pointer, with
the mapping held privately here. Landed in `spike/CLAUDE.md` this pass:

| exported ordinary invariant | quarantined source |
|---|---|
| `rul-probe-writes-only-what-it-owns` | `sinv-owned-probe-channel`; `293:finding-report-scratch-can-clobber-host-files`; `29:rank-one-report-scratch-clobber` |
| `rul-every-erased-establish-is-vouched` | `sinv-mutation-elision-vouch`; `293:finding-aggregate-elisions-bypass-vouch-tier`; `29:rank-two-aggregate-vouch-bypass` |
| `rul-host-bytes-bounded-before-admission` | `sinv-host-evidence-ingress`; `29:rank-three-host-ingestion-and-display` |
| `rul-attribution-is-controller-minted` | `sinv-controller-attribution` |
| `rul-integrity-failure-withholds-mutation` | `sinv-integrity-failure-mutation` |
| `rul-fixture-identity-never-production` | `sinv-production-fences`; `29:rank-four-attempt-and-decision-identity` |

Re-entry triggers travel with the exported text where they are ordinary (transport,
concurrency, saved approval, cross-host reuse); the quarantined rationale does not.

## 7 - What this pass changed

Documents: this ledger; `299` corrected and the opaque-review waiver recorded; `298` and
the phase packets ported onto the `ai/main` lineage with superseded headers.

Steering: the six exported invariants (one new `spike/CLAUDE.md` section for the
controller-host intake, two additions to the license-and-trust block); the three false
report-lane claims corrected to disabled-with-repair-specified; crate-local seatings in
`plan/CLAUDE.md` (the aggregate mints carry the same vouch demand) and `cli/CLAUDE.md`
(admission precedes the fold; attribution is minted at this edge). Two drive-by coherence
fixes found while there: `spike/CLAUDE.md` contradicted itself on the `24J` raw-ship debt
(its build-status said cleared, its invariant bullet still carried the HEAD-DEBT warning)
and `cli/CLAUDE.md` repeated the stale warning - the repair is real and machine-pinned by
probe-render tests, so both now say so.

Code: the buildable compile-fail proofs at their honest scope (four in `core::claim`
pinning the tier algebra, each verified to fail for its intended reason rather than a
typo; two in `plan` pinning the mint's vouch demand by naming its whole signature, and the
aggregate proof's unconstructibility); a doc-comment fence on `whylog::inspect`; the
re-inlined digest collapsed back onto its single substitution point.

Housekeeping: fifteen stranded round-29 worktrees removed plus one orphaned directory
cleared; eleven provably-merged branches deleted with `-d`.

Two pieces of uncommitted work were found and FROZEN rather than lost, both now recoverable
from their branches:

- `ai/r29-ingress`@`8d2cc3b7` - the phase-three in-flight patch `298` describes (the
  public-`AttemptScope` and pre-admission-clone repairs). Superseded: both gaps are closed
  differently and better on `ai/main`. Kept as evidence, not for porting.
- `ai/r29-resume`@`1849a3a1` - a substantially expanded, never-committed rewrite of `298`
  itself: named work-unit slugs (`secphase0-map-and-freeze-present-seams` through
  `secphase5-production-fences-and-authority-regression-gates`), an explicit five-clause
  acceptance bar, and split immediate/later resumption sequences. Better-structured than
  the committed `298` and the origin of the `secphase*` vocabulary, but written before the
  immediate unit landed, so most of its process is now discharged. NOT ported: this ledger
  supersedes both, and a second overlapping handoff would make the quarantine less
  coherent, not more. Read it if the `secphase*` naming is wanted.

Branch disposition, decided by `git cherry` against `ai/main` rather than by merge status:

- KEPT, unique commits not upstream, and cited by hash in `298`'s ledger:
  `ai/r29-impl`, `ai/r29-report-channel`, `ai/r29-aggregate-vouch`, `ai/r29-continuity`,
  `ai/r29-ingress`, `ai/r29-resume`. Deleting these would orphan those citations.
- SUPERSEDED, every commit patch-equivalent upstream (rebased in, nothing unique):
  `ai/r29-resume-report`, `ai/r29-resume-vouch`, `ai/r29-resume-vouch-fix`,
  `ai/r29-resume-ingress-diagnostic`. Deletion needs `-D` (they are unmerged in the
  ancestry sense), which the repository's git hook reserves for the human. Left in place;
  the command is safe whenever they want it.

## 8 - Outstanding

Ordered by leverage, not urgency.

1. Behavioural coverage for the report lane. The repair itself LANDED (`29B`, branch
   `ai/r29-drep-repair`); what remains is `fnd-gate-two-refuses-the-drained-render` - the e2e
   harness cannot run a drained probe, so the lane's only executable evidence is a hand-run
   fixture. The context cells still depend on entry-machinery work.
2. Phase four - sink encoding and sensitivity separation, plus whylog filesystem
   hardening. The report repair's encoder is a subset.
3. Phase five - fixture/production identity split (including
   `fnd-third-fnv-digest-copy`), legacy-parser fencing, the authority-mint manifest and
   its source test, and the compile-fail proofs that phase four and five make buildable.
4. `fnd-whylog-inspect-reopens-raw-parse` - fence properly once `plans/288` settles.
5. `295`'s four unowned re-entry gates: the bounded-observables public contract (a
   publication blocker), mint-time authority witnesses, oracle upgrade authority-diff, and
   context siting bound separately from mechanical capability.
6. `ask-context-reentry-for-cleanup`, section 4h.

## 9 - Confidence

`+SURE`: the phase state table (read from the tree, not the handoffs); the ACK contents
and the four accepted rows; `rec-native-gate-on-shipped-revision` (run by hand);
`fnd-ack-does-not-bind-shipped-bytes`, `fnd-export-step-never-ran`,
`fnd-steering-asserts-a-dead-lane`, `fnd-third-fnv-digest-copy`.
`~SUSPECT`: `mkdir`'s `EEXIST`-on-symlink guarantee (4b - load-bearing, fixture-provable,
must be proven before the repair ships); the judgment that `fnd-whylog-inspect-reopens-raw-parse`
is presently unreachable from host bytes (traced by hand, one call site, could acquire
another); the claim that the `ai/r29-ingress` dirty patch is fully superseded.
`-GUESS`: the sizing in 4j.
