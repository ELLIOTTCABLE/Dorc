# 280A — Review working notes for the post-crosscheck design package

Status: private review ledger, not a specification.  Claims below are provisional
until the final cross-reference and implementation-steering passes are complete.

## 1. Authority and scope read

Read before judging the package: `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`,
`USER_STORY.md`, `KNOBS.md`, `ANALYZER-NEEDS.md`, `TODO-ADDTL.md`, `AGENTS.md`,
and `Research/README.md`.

Read in the package: 279F, 277, 275, 271, 276, 278, 27A, 27B, 27C, the root
spike steering file, and all seven crate steering files.  Supersession banners in
27A and 27B were treated as controlling rather than as invitations to review their
withdrawn designs.

## 2. Provisional high-value checks

- `280A:suspect-structural-transport-repeats-completeness-gap`: 27C's unflagged
  carried-by fallback establishes invariance of a disclosed backing cell, but does
  not establish invariance of every input consumed by the foreign-context
  measuring body.  This appears to recreate the exact positive-disclosure hole
  accepted in 279F and carefully explained in 27A, while the adjacent authored
  kind-invariance row correctly requires an explicit risk flag.
- `280A:suspect-authority-rulings-conflict-on-entry`: 27C says 271 outranks it,
  while re-scoping the no-escalation premise that 271's later stopping-point ruling
  expressly incorporates.  The same section also says the four root/non-root ×
  held/acquirable cells are all implemented while deferring the non-root acquisition
  mechanism.
- `280A:suspect-steering-restores-diverged-role`: the steering compression names
  `__is_diverged` as an authored role even though the package's settled surface
  ditches it in favor of complement syntax.
- `280A:suspect-steering-overstates-forged-verdict-defense`: hostsim steering asks
  tests to prove a forged Converged verdict cannot suppress apply, while the
  maintained analyzer needs explicitly leave malicious-host verdict forgery open;
  a Must license cannot make a dishonest measurement true.
- `280A:suspect-context-entry-self-effects-cross-probe-law`: entry wrappers are
  permitted persistent self-effects such as authentication logging/timestamp
  refresh while the root probe contract is presented as nonmutating.  This may be
  an intentional oracle-vouched exception, so the earlier self-effect and threat
  rulings must be checked before crediting it as a conflict.
- `280A:suspect-pipefail-handshake-is-not-context-qualified`: the pipefail
  handshake predates context entry and is host/session-shaped.  A shell selected
  inside a chroot/container can differ from the host shell, so the claim that
  pipefail was verified may need `(host, context)` qualification.  Expected failure
  may still degrade safely; consequence needs tracing.

## 3. Positive findings so far

- Context entry is a materially better primary answer than out-of-context
  measurement: it measures the wrapped site's actual world rather than merely
  renaming transport confidence.
- 279F's amendments to 277 make minting, backing, claiming, and policy ownership
  substantially more attributable, and explicitly retain the operator risk flag
  where read-completeness cannot be expressed in shell.
- 276/278 state the pipefail crack and the preserved-source rule plainly rather
  than silently rewriting authored shell.

## 4. Remaining trace work

Read the directly cited wrapper/context and threat-model rulings (especially 273,
272, 24S, and 262), then inspect the relevant spike types/tests.  For every
provisional issue, construct the smallest shell counterexample and test it in the
reverse propagation, probe/apply, admin/author, and reliable/unreliable-oracle
cells.  Withdraw findings whose consequence is only already-priced unsafe mode or
throwaway implementation detail.

## 5. Cross-reference pass 1 (verified / withdrawn)

### Verified

- `280A:finding-structural-row-repeats-completeness-gap` is stronger after the
  provenance trace. `279f` 67--77 says the fact-side form has the same defect as
  the value-side form: a store claim does not close the verdict body's reads.
  `27A` 199--208 states the invariant directly (store-invariance is not
  answer-invariance), and its corrected 428--435 disposition says unsayable
  measuring-body completeness is what forces the flag. `27C` 193--196 exempts the
  structural carried-by row without adding any premise about the measuring body,
  then applies the accepted completeness analysis only to the authored row at
  197--211. This is a default, unflagged wrong-elision path, not merely stale prose.
  The internal one-screen summary also contradicts itself: 26--30 permits an
  unflagged row and then says absent the flag nothing travels, ever.
- `280A:finding-steering-restores-hard-deleted-role` is verified. The human-typed
  removal is explicit at `24C` 734--763, including deletion of the suffix,
  `VerdictSense`, glue, and test. `spike/CLAUDE.md` 104--108 and 353--360 restore
  the role. The stale compression has already propagated through oracle parsing,
  reservation, verdict evaluation, plan glue, and positive tests; the focused
  `cargo test -p dorc-oracle diverged_sense` passes both obsolete-role tests.
- `280A:finding-nonroot-authority-partition-is-incoherent` is verified as a spec /
  compression split. `27C` 56--64 simultaneously says non-root connections perform
  no shifts and says the only implementable predicate is zero-new-credential
  capability; its own entry form is `sudo -n` (158--160). The superseded trail had
  explicitly classified NOPASSWD `sudo -n` as authority already held (`27B`
  133--139), but the new four-cell list has no non-root + already-held cell. The
  steering compression keeps the zero-new-credential rule and omits the categorical
  non-root refusal, so a builder reading only it implements different semantics.
- `280A:finding-nested-wrapper-rider-dropped` is verified. `279f` 54 and 127--133
  credited and dispatched a pointwise lend/rho composition rule with top
  propagation. `27C` 172 says only that entry forms recurse; its batching and
  context-keying then rely on `(host, context)` without defining nested lend/rho
  composition. The steering wrapper law (`spike/CLAUDE.md` 386--392) retains only
  single-wrapper/dual-peel coherence. This is a direct failure to carry an accepted
  review rider into either the new spec or its compression.

### Withdrawn or narrowed

- `280A:suspect-authority-rulings-conflict-on-entry` is withdrawn as an authority-
  ordering attack. `271:rul-stopping-point-unpinned` cites `24S:imp-1`, and the
  cited imp-1 now contains an in-place re-scope to reuse-never-acquire. Citation
  indirection is ugly but newest-wins resolves it. The non-root partition problem
  above remains independently.
- `280A:suspect-context-entry-self-effects-cross-probe-law` is narrowed below a
  finding. Root doctrine distinguishes non-mutation from side-effect-free reads,
  and explicitly accepts that read-only probing may have incidental effects
  (`USER_STORY` 956--964; threat model 55--75). The auth-log/timestamp cost should
  be named, but 27C does name and attribute it; treating that alone as a hidden
  contract breach would overstate the root promise.
- `280A:suspect-steering-overstates-forged-verdict-defense` is withdrawn. The
  hostsim bullet's actual assertion is narrower than its parenthetical: a forged
  verdict may not bypass the independent Must license. The maintained open need
  correctly says a hostile host can still lie inside a Must-licensed measurement.
- The pipefail emit-never / off-ramp suspicion is withdrawn. The package now draws
  parser acceptance separately from base-dialect conformance, and the executable
  floor is the conformance gate. This is ceremony and lint dependence, but it is a
  recorded, human-accepted price rather than a quiet guarantee failure.

## 6. Maturity/cost question still being ranked

`27C` leans on two components it labels unfinished: the non-root acquisition cell is
said to be one of four implemented cells (78--83, 304--307) while its mechanism/UX is
deferred (324--326); conditional tails carry the residue-containment/value argument
(31--33, 221--236, 268--270) while mechanics remain STRAWMAN and a later-round design
(231--234, 345--349). This is likely a medium finding about the weight placed on a
direction sketch, not a correctness kill: every unimplemented case can degrade to
guard/run safely.
