# 27Xm — context-entry crosscheck: raw archived report (quarantine)

> ARCHIVED RAW foreign-lineage output — do NOT read into an active conductor
> context. Working digest lives at notes/27Xn-context-entry-correctness-digest.md.
> Retained here per the purity protocol (raw foreign output is digested, not
> propagated). Provenance: OpenAI Codex / gpt-5.6-sol, read-only lane.

----

# _tmp-27C-crosscheck-sol-report — outside-lineage (Codex/Sol) review of 27C

Crosscheck deliverable for `plans/27C-context-entry-probing-design.md` (context-entry
probing / the block-context crosscheck manager arc). Two blocks: a manager provenance
digest, then the raw foreign report verbatim. Every finding is tagged `[SOL-RAW]` (the
foreign model's words, UNADJUDICATED) or `[MANAGER-NOTE]` (dispatch/provenance only —
never an adjudication). The conductor adjudicates under maximum skepticism; nothing here
is credited as truth by virtue of being written down.

---

## [MANAGER-NOTE] Provenance and process

- **Model / lineage:** OpenAI Codex, `gpt-5.6-sol` (the harness's default reasoning
  effort for the read-only lane). Foreign lineage — decorrelated from Claude, but
  precision-ranked below Fable/Opus and prone to over-flagging severity (skill
  calibration: "treat its 'critical's as 'worth checking'"). Weight accordingly.
- **Lane / mode:** `codex-reviewer` sonnet-shim, read-only review (no worktree writes,
  no repo mutation), dispatched under the `foreign-models` skill. Base `ai/spike3-r27`
  @ `7794838`. Shim exit 0, `turn.completed` clean, first-attempt success, no
  dispatch/setup errors reported.
- **Packet:** fully self-contained (dispatch bundle at
  `…/scratchpad/27C-sol-dispatch-bundle.md`). Inlined verbatim: 27C in full; 273 §§0–4
  (the `predict`/`lend_map` wrapper surface it extends); the correctness frame
  (IMPLEMENTATION "To execute, or not to execute?" + DESIGN "Priorities"); a neutral
  glossary of project jargon. Codex needed no repo reads and confirmed none were used.
- **Framing given (exclusions-not-inclusions):** kill-mandate, disowned third person,
  full latitude to attack ANY aspect; the correctness invariants stated as the bar to
  judge against. NO suspected weak-points named, NO finding checklist supplied. Declared
  out-of-scope: STRAWMAN name bikeshedding; relitigating §10-ruled decisions as taste
  (attacking their CONSEQUENCES was explicitly in-scope); filing §4's deliberate
  under-design as incompleteness. The design's own §7 "residual holes" was included as
  part of the artifact but framed as claims-to-test, not a map.
- **Finding count:** 9 (3 blocker · 4 serious · 2 moderate) + 5 "attacked and it holds"
  observations.

## [MANAGER-NOTE] Neutral scope-interaction pointers (NOT adjudication — for the conductor)

Flagging where findings brush against the design's own fences, so the conductor can
route them; this is bookkeeping, not a verdict on whether Sol is right:

- SOL-8 (`finding-conditional-tail-misses-partial-failures`) attacks §5 conditional-tail
  mechanics, which §10 marks STRAWMAN and §5 defers to "the placement-spectrum round."
  Sol itself notes this. In-scope as a consequence-attack; the conductor decides whether
  it lands against present design or against admitted future work.
- SOL-9 (`finding-fallback-flag-does-not-contain-underexecution`) targets §4, the
  deliberately-fenced fallback lane. Sol concedes the `--risk-faultless-skips` flag may
  satisfy the "explicit user dictum" exception to invariant #2, and narrows its claim to
  the "self-heals / contained" WORDING rather than an under-execution soundness breach.
- SOL-2 and SOL-7 (`entered-world-executes-untrusted-code`,
  `entry-failure-does-not-prove-clean-degradation`) lean on the design's OWN §3
  admission that entry has self-effects ("auth-log line", "timestamp refresh"). Sol
  presents that admission as self-refuting rather than as a priced trade (§7
  `hole-bad-oracle-blast` claims it is attributed/bounded). Whether "modeled +
  elide-alongside" discharges invariant #1, or merely relabels a mutation, is the crux
  the conductor must weigh — Sol asserts the latter; I record the disagreement without
  resolving it.
- SOL-5 (`context-key-omits-security-state`) is a completeness attack on the four-named
  dimensions (adds supplementary groups, capabilities, MAC/SELinux domain, cwd, umask,
  seccomp, mount-ns). Whether "user" is intended as shorthand for the full credential
  context is not settled in the packet; Sol flags the gap. Cross-check against 273's
  dimension model and 272 topology before crediting.
- The three "probe-mutation" blockers (SOL-1/2/7) and the TOCTOU blocker (SOL-3) are the
  highest-value items IF they survive scrutiny; they are also exactly where an
  over-flagging foreign model would cluster false positives. Spend skepticism here first
  (skill: "wrong findings cluster in the unverified bucket").

---

## [SOL-RAW] Raw Codex/Sol report — verbatim, UNADJUDICATED

> Everything below this line is the foreign model's own output, reproduced without
> edit. Finding slugs are Sol's. Do not treat any claim, severity, or confidence rating
> as established; each is an input to adjudication, not a conclusion.

# Adversarial review of 27C

The design has three blocker-grade failures. The most severe is that its flagship entry mechanism cannot uphold the categorical "probe never mutates" promise even for ordinary, honestly authored wrappers. More fundamentally, context entry causes non-oracle code from the entered world to execute with connection authority, contradicting the claimed "only oracle bytes execute" security boundary.

## Blockers

### [SOL-RAW] finding-entry-self-effects-mutate-probe

- Severity: blocker
- Location: §3 entry forms; §7 `hole-bad-oracle-blast`
- The break: The design explicitly permits entry self-effects such as "an auth-log line" and "a timestamp refresh," calls them modeled, and proposes to "elide-alongside" them.

  Concrete execution:

  ```sh
  sudo pipx install poddle
  ```

  Host state and authority:

  - Dorc has a root connection.
  - The default dial permits entry.
  - `pipx__is_converged` carries `tolerates:user`.
  - Dorc invokes `sudo -n ...` during planning.
  - sudo/PAM/audit facilities append an authentication or session record, or refresh sudo timestamp state.

  The remote machine is now different because Dorc offered and ran a plan-stage check. Modeling the mutation does not make it non-mutating; "elide-alongside" does not undo it.

  Composition makes this worse:

  ```sh
  sudo chroot /missing oracle-check
  ```

  The outer sudo entry may log or refresh state before the inner chroot fails. The ladder then produces can't-say and claims safe degradation, but the probe mutation has already happened.

- Violated invariant: #1, probe must never mutate. Also §3's "every direction safe" claim.
- Confidence: high. Refutation would require a construction proving that every supported entry implementation, including its authentication, PAM, audit, and timestamp paths, is observationally non-mutating on every supported host. The document instead concedes the opposite.

### [SOL-RAW] finding-entered-world-executes-untrusted-code

- Severity: blocker
- Location: §3 "only oracle bytes execute"; chroot and shifted-environment entry
- The break: Source-byte provenance is not execution provenance. Entering a filesystem or identity context changes the interpreter, dynamic loader, libraries, command lookup, NSS modules, configuration, and executable files used to run those source bytes.

  Concrete chroot case:

  ```sh
  chroot /mnt/target apt-get install -y openssh-server
  ```

  Host state and authority:

  - Dorc connects as root.
  - `/mnt/target` is an image assembled from less-trusted or partially provisioned content.
  - Its `/bin/sh`, dynamic loader, `grep`, or another utility used by the oracle has been replaced with a program that writes `/probe-ran` and then returns a plausible answer.
  - Dorc enters the chroot and runs the serialized oracle body.

  Although the shell text came from an oracle, target-controlled executable code runs as root during the probe. It can mutate the target or escape through exposed devices, mounts, sockets, or kernel interfaces.

  A simpler user-shift case is:

  ```sh
  sudo -u app pipx install X
  ```

  If shifted command lookup resolves `pipx` or a helper from an app-controlled path, a supposedly read-only `pipx list` body executes app-controlled code. A tolerance vouch about the oracle's shell body does not establish the integrity or behavior of the programs resolved after the shift.

  This also creates apply under-execution: a target-controlled `grep`, package query, or shell can fabricate "converged," causing Dorc to elide the real mutation.

- Violated invariants: #1, probe non-mutation; #2, never under-execute; #4, attributed/local failures. The mutation or false verdict cannot honestly be attributed solely to the oracle line, wrapper author, or admin dial—the entered world supplied executing code.
- Confidence: high. Refutation requires an entry scheme that executes a trusted interpreter and all dependencies from outside the shifted filesystem while still measuring the target context, plus protected command resolution and data/code separation. No such mechanism is specified.

### [SOL-RAW] finding-probe-result-races-apply-elision

- Severity: blocker
- Location: §0 default lane; §3 measurement; §5 guards
- The break: A successful plan-time entry measurement licenses elision, but §5 supplies runtime guards only to sites that could not elide. Nothing revalidates a successfully elided wrapped site at apply.

  Concrete execution:

  ```sh
  sudo pipx install poddle
  ```

  Sequence:

  1. In-context probe observes root's `poddle` installation and returns converged.
  2. Dorc removes the original line from the apply plan.
  3. Before apply, another administrator or service removes the package, replaces the root home, or changes the mounted filesystem.
  4. Apply runs without the install line.
  5. The desired final state is absent.

  The same break is sharper for mutable chroot mounts:

  1. Dorc probes `/mnt/target` filesystem A.
  2. The mount is replaced or remounted to filesystem B.
  3. Apply elides `chroot /mnt/target apt-get install X` based on A's fact.

  The fact was correctly measured but was stale when consumed. Context qualification does not cure temporal invalidation.

- Violated invariant: #2, never under-execute; also the stated TOCTOU goal.
- Confidence: high as written. Refutation requires a guaranteed atomic plan/apply transaction, immutable/version-pinned contexts, or apply-time revalidation of every elision. None appears in 27C.

## Serious findings

### [SOL-RAW] finding-authority-acquisition-contradicts-core-rule

- Severity: serious
- Location: §1 four operational cells; `rule-reuse-never-acquire`
- The break: The design simultaneously says that probe "never acquires authority" and requires a non-root cell with an explicit pre-probe acquisition mechanism such as `sudo -v`.

  Concrete execution:

  - Dorc connects as an unprivileged user without a usable sudo timestamp.
  - It invokes `sudo -v`.
  - The administrator supplies a credential.
  - sudo creates or refreshes an authorization timestamp.
  - Probe entry then exercises authority that the original connection did not have before Dorc's acquisition step.

  This is acquisition, credential handling, and usually state mutation. Calling it "pre-probe," "one-shot," or "credential never stored" does not preserve the promise. Dorc caused a new authority grant to exist for subsequent commands, potentially including commands outside Dorc during the credential-cache lifetime.

  The deferred UX is also security-relevant: the design cannot claim all four cells implemented while leaving the consent boundary, lifetime, revocation, and concurrency effects of the acquired authority unspecified.

- Violated invariants: #1 and #4; the design's central security sentence and reuse-never-acquire rule.
- Confidence: high. It could be resolved by removing this cell from 27C's promise or defining it as a separately acknowledged authority-acquisition phase with its mutations and ambient lifetime explicitly accounted for. That would still abandon "never acquire."

### [SOL-RAW] finding-context-key-omits-security-state

- Severity: serious
- Location: §3 batching by `(host, context)`; §0/Glossary dimension model
- The break: User, filesystem view, netns, and environment are not a complete execution-security context. Relevant state also includes supplementary groups, capabilities, MAC label/domain, cwd, umask, resource limits, seccomp state, mount namespace details, and potentially sudo policy-selected execution attributes.

  Concrete example:

  ```sh
  sudo -u postgres -g dbread check-db
  sudo -u postgres -g restricted check-db
  ```

  Both sites can map to the same user identity, filesystem view, netns, and ρ while having different supplementary/effective group authority. A database socket or state file may be readable in one context and inaccessible in the other. If Dorc batches them under one `(host, context)` segment or transports facts using the four-dimension key, one site's answer can be consumed for the other.

  Another example is two entries with the same uid and filesystem root but different SELinux execution domains; peer sockets and files can yield different answers.

  Missing dimensions are not harmless merely because entry performs the real transition. They become unsound when facts, batching, disturbance routing, or context reuse identifies two distinct effective worlds.

- Violated invariants: #2 and #4; context-locality and fact attribution.
- Confidence: medium-high. The finding is refuted if "user" is formally defined as the entire credential/security context and batching keys include every wrapper-selected execution attribute, rather than the fixed-string uid-like mapping shown. The current examples and four-dimension enumeration do not establish that.

### [SOL-RAW] finding-batched-oracles-contaminate-later-checks

- Severity: serious
- Location: §3 "one entered segment per `(host, context)`"
- The break: A tolerance vouch constrains machine mutation, not process-local mutation. Shell oracle bodies can alter cwd, umask, variables, traps, options, file descriptors, limits, or function definitions. If several checks share an entered shell segment, one oracle can silently change the environment in which later checks execute.

  Concrete execution:

  ```sh
  first__is_converged() {
      : tolerates:fs-view
      cd /tmp || return 2
      test -e marker
  }

  second__is_converged() {
      : tolerates:fs-view
      test -e relative/package-installed
  }
  ```

  If both run in the same entered segment, the second check reads `/tmp/relative/package-installed` rather than its intended directory and may falsely return converged. Dorc can then elide a required mutation at an unrelated site.

  Similar contamination follows from `set +e`, `trap`, `exec 2>/dev/null`, `umask`, or exported variables. This defeats the claim that failure and consent remain local.

- Violated invariants: #2 and #4.
- Confidence: medium. The break disappears if every oracle invocation runs in a fresh, fully normalized subprocess with independently constructed argv, environment, cwd, descriptors, traps, limits, and shell options. Section §3 specifies batching but no such isolation boundary.

### [SOL-RAW] finding-entry-failure-does-not-prove-clean-degradation

- Severity: serious
- Location: §3 degrade ladder
- The break: The ladder treats any entry refusal, rc 127, or in-context decline as a clean can't-say. That is safe only if the failed attempt had no prior effects and did not leave the remote state unknown.

  Concrete execution:

  ```sh
  ip netns exec blue oracle-check
  ```

  An implementation of named-network-namespace entry may create a temporary mount namespace and arrange namespace-specific `/etc` bindings before executing the check. Likewise, sudo/PAM can open sessions or update audit state before the guest fails, and nested entry can complete outer transitions before an inner rc 127.

  A single scalar exit code does not distinguish:

  - failure before entry;
  - failure after entry self-effects;
  - failure after partial oracle execution;
  - a read error;
  - a mutation followed by failure.

  Converting all of these to can't-say → guard/run can hide a probe mutation and can proceed to apply after state has become unknown.

- Violated invariants: #1 and #4, especially the requirement to stop after unsafe unknown cross-network state.
- Confidence: high for the sudo/nested-entry case because §3 admits entry self-effects. Medium for specific `ip netns exec` internals; confirming that part requires platform-specific tracing.

## Moderate findings

### [SOL-RAW] finding-conditional-tail-misses-partial-failures

- Severity: moderate
- Location: §5 conditional tails
- The break: The flag is described as being set "iff its fallback body actually executed," but the mechanics are not pinned down for nonzero exit, signal termination, partial execution, or loss of connection.

  Concrete execution:

  ```sh
  apt-get install X
  subsequent-command-derived-from-old-dpkg-fact
  ```

  `apt-get` can modify package state and then fail. If the conditional-tail flag is set only after successful completion, downstream lines retain probe-time elisions even though the wall fired and disturbed relevant state. If the connection drops after mutation but before the flag assignment reaches Dorc, the controller cannot know which branch occurred.

  Correctness requires the disturbed branch to become active before invoking the fallback, irrespective of its eventual return code, and an indeterminate completion must poison affected downstream facts or stop apply.

- Violated invariants: #2 and #4.
- Confidence: medium. The design calls this mechanism STRAWMAN, so an implementation could fix it. As specified, however, "sets a flag iff … executed" does not define the crash-consistent state transition needed for the claimed soundness.

### [SOL-RAW] finding-fallback-flag-does-not-contain-underexecution

- Severity: moderate
- Location: §4 invariance fallback; §7 claim that the next plan "self-heals"
- The break: `--risk-faultless-skips` may authorize a cross-dimension answer whose completeness has no attributable claimant. If that answer is wrong, the mutating line is elided. "The next plan re-measures and self-heals" does not contain the failure:

  - the next plan may never run;
  - the skipped command may have been needed before a dependent apply command;
  - the dependent command may irreversibly consume the absent state;
  - the same incomplete measurement can repeat the same false answer.

  Concrete form:

  ```sh
  chroot /target install-provider X
  chroot /target deploy-consumer-requiring-X
  ```

  An outside measurement crosses the boundary under a kind invariance declaration and the risk flag, but the foreign checker also depends on configuration outside the declared store. It falsely reports X converged. The install is elided, and deployment fails or writes partial state.

  The admin flag is explicit risk acceptance, so this is not necessarily a violation of the "except by explicit user dictum" exception. It is nevertheless incorrect to describe the hole as self-healing or contained.

- Violated goal: honest risk pricing and failure containment; potentially invariant #4.
- Confidence: high regarding the false containment claim. Whether the explicit flag is accepted as sufficient dictum for invariant #2 is a product-contract question.

## Attacked mechanisms that hold

- `observation-missing-entry-fails-closed`: Missing entry forms, unmodeled wrappers, missing dimension keys, and failed in-context checks all nominally go to can't-say/guard/run rather than elision. That direction is sound provided no attempted entry has already had effects.
- `observation-dial-applies-both-lanes`: Applying the escalation dial to both probe checks and apply guards correctly recognizes that a damaging oracle is dangerous in either lane.
- `observation-unresolved-maps-do-not-identify`: Treating unresolved mapped values as unknown rather than equal is the safe direction for preventing fact transport.
- `observation-partial-chain-consent-fails-closed`: Requiring entry, dial permission, and tolerance coverage for every crossed dimension is structurally conservative. The remaining failure is that the vouch cannot secure code loaded from the entered world.
- `observation-book-bytes-survive-guarding`: Preserving original apply bytes avoids an obvious rewrite-equivalence bug. It does not cure false short-circuit verdicts or TOCTOU.

No repository or web access was needed for the central findings. The platform-specific extent of `ip netns exec`, sudo/PAM, and audit side effects varies, so I have not relied on any single optional behavior: the design itself expressly admits sudo logging/timestamp effects, and that admission alone establishes the categorical probe-mutation failure.
