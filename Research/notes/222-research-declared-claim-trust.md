# 222 — research: author-declared behavioral claims in real ecosystems (trust, lies, calibration)
<!-- /* slug corrected 2026-06-11: drafted under provisional title "21J" before being
filed under the 22x research convention; 21J was never a filed note. Old slug preserved
here for grep. (note 217 §7) */ -->

> Deep-research note serving 20V (door-2 "declared converged-run", door-4 guard-insertion,
> §8 dq-errexit ledger). Question: how do real automation/build ecosystems handle
> AUTHOR-DECLARED claims about command behavior — dry-run fidelity, idempotence,
> check-mode support, cache-safety — what happens when declarations are WRONG, and what
> mechanisms kept trust calibrated. Source-IDs graded [A|B|C|D-slug-year]; full list §8.
> Confidence markers per claim. ~45 sources surveyed, ~14 full/deep-read.

## §0 Conclusions up front

- **c-1 (+SURE).** Every ecosystem that shipped author-declared dry-run/idempotence
  claims accumulated a lying-declaration bug class; none escaped it. Ecosystems differ
  not in *whether* declarations lie but in *where the lie surfaces*: silent wrong plan
  (Ansible check-mode) vs loud, party-attributed error (Terraform apply-time
  cross-check, Nix hash mismatch) vs symptom-far-from-cause (Bazel cache poisoning).
- **c-2 (+SURE).** The canonical failure is *compositional*, not per-claim — Chef's
  "assumptions problem": per-resource forecasts conditioned on *unexecuted prior
  mutations* compound into wrong plans even when each declaration is individually
  defensible. Chef's own vendor told users to stop using why-run and floated removing
  it [A-chef-whyrun-2018]. Dorc's probe-grounded elision dodges the worst of this
  (plans condition on *measured present*, not simulated future) — EXCEPT at
  stale-probe boundaries, and wherever a declared counterfactual *output* feeds later
  control flow: door-2's consumed-stdout cell re-imports the assumptions problem.
- **c-3 (+SURE).** What survived as trustworthy: predictions *computed from measured
  present state* (Puppet declarative resources reading state under noop; Ansible
  `--diff` on rendered templates; Terraform plan-after-refresh) and claims verified
  *ex-ante in a harness* (Test Kitchen/InSpec; Hummer-style repeated execution). What
  died: forecast-through-unexecuted-mutation. Door-4 (re-measure live at apply) sits in
  the surviving family; door-2 (static declared) sits in the dying one unless fenced.
- **c-4 (+SURE).** "The dry-run lane runs author-shipped read-side code" is universal,
  accepted practice — Puppet noop *must* execute `onlyif`/`unless` commands
  [A-puppet-exec-2026]; Ansible check-mode modules connect and query; Chef why-run
  evaluates guards. dq-errexit-3's probe-side precedent is total; and the same guard
  code already runs in real (apply) runs in all three ecosystems, so guard-insertion at
  apply is *less* exotic than probing. The trust line ecosystems drew: the kind's
  author owns the kind's read-side; disclosure is owed; failure attributes to the
  declaring party.
- **c-5 (+SURE).** Convergent evolution onto weld-5: when unsure, over-predict change
  or refuse to predict — never fabricate "no change". DeHaan, 2014: modules that don't
  fully understand check mode "report 'changed=True' automatically … rather than risk
  making a change" [A-ans-forum-2014]; modules with no support do nothing and predict
  nothing; a module that unconditionally returned `changed=false` was treated by
  maintainers as a bug, arguably mis-declared support [A-cg-aerospike-2024].
- **c-6 (+SURE).** Attribution quality is the single biggest trust differentiator.
  Terraform's apply-time cross-check of the provider's *own* declared plan emits
  "This is a bug in the provider, which should be reported in the provider's own issue
  tracker" [A-tf-blame-code-2026] — blame routes off the core tool and bugs get filed
  against the declaring party. Bazel's unattributed cache poisoning (red default
  branch, retry-button culture, remediation = flush the cache) is the anti-example
  [A-bazelci-1174-2021]. Nix FOD hash-mismatch errors are precise and self-attributing —
  yet FODs are *also* the cautionary tale for suppressed re-verification: a matching
  declared hash means the commands are never re-run, so drift is invisible
  [A-eigenvalue-fod-nd], and even `--check` exempts FODs [A-nix-3369-2020].
- **c-7 (~SUSPECT).** Contracts tighten by *phased enforcement*, not flag-day: Terraform
  grandfathered legacy-SDK providers — violations logged as `[WARN] … tolerating it
  because it is using the legacy plugin SDK … may be the cause of any confusing errors
  from downstream operations` [A-tf-legacywarn-2026] — before the framework made them
  hard errors; Puppet noop and Ansible check stayed opt-in forever. The dq-errexit
  default-OFF CLI-flag ruling matches precedent exactly.
- **c-8 (~SUSPECT).** Trust-psych literature: appropriate reliance needs *calibration*
  (trust matching capability) [B-leesee-2004]; false-alarm-prone automation damages
  both compliance and reliance, miss-prone mostly reliance [C-dtic-favsmiss-2006].
  Dorc's chronic risk is over-warning (false alarms); its catastrophic risk is wrong
  elision (a miss). Per-door attributed disclosure is the calibration mechanism; one
  popular lying oracle ⇒ correlated identical failures across books/hosts ⇒ the
  reputational cliff (why-run's reputation died ecosystem-wide this way).
- **c-9 (mechanism proposal; -GUESS on novelty).** door-2 and door-4 are two independent
  sources of the *same* claim. Where both exist, occasionally running door-4's live
  guard under a door-2-elided site (sampled, apply-lane) is a free Trustix-style
  cross-check — converts never-verify into verify-statistically and catches declaration
  rot before disaster.
- **c-10 (+SURE).** Pin declarations to what they're sensitive to (Flyway/Liquibase
  checksum lesson; FOD lesson): a converged-run declaration must be keyed at least to
  oracle text/version — un-keyed declarations rot silently. Liquibase's `validCheckSum`
  (author re-vouches after an edit) is the idiom for explicit re-vouching, with
  documented divergence hazards [C-so-validchecksum-2021].

---

## §1 (r-1) Ansible check_mode: the closest prior art for door-2

**The declaration mechanism.** A module author passes `supports_check_mode=True` when
instantiating `AnsibleModule`; with `--check`, "Modules that support check mode report
the changes they would have made. Modules that do not support check mode report nothing
and do nothing" [B-ans-checkmode-docs-2026]. So the *default* for an undeclared module
is do-nothing-predict-nothing — degraded coverage, never fabricated convergence. The
admin has per-task override in both directions: `check_mode: true` (always simulate)
and `check_mode: false` (really execute even under `--check`); pre-2.2 the latter was
spelled `always_run: yes`. Note the precedence stack this builds:
**admin task-keyword > module declaration > engine default** — structurally identical
to 20V §5's admin-explicit > oracle-default > engine-conservative.

**Declaration taxonomy evolved into three levels + prose.** Modern modules carry an
`attributes:` documentation block with `support: full | partial | none` and a `details:`
field for the partial case. The `command` module verbatim
[A-ans-command-attrs-2026]:

```yaml
attributes:
    check_mode:
        details: while the command itself is arbitrary and cannot be subject to the
                 check mode semantics it adds O(creates)/O(removes) options as a workaround
        support: partial
    diff_mode:
        support: none
```

`apt` declares `check_mode: support: full` [B-ans-apt-attrs-2026]. The `partial` +
prose pattern is the mature form: a *scoped* declaration ("only when you give me
`creates:`/`removes:` guard expressions can I predict") rather than a boolean. That is
exactly an effort-rung: the admin adds a guard expression (spelled in the task), and
the arbitrary command becomes predictable — Ansible's r-1/r-2 analogue.

**Bug archaeology — the lie classes observed:**

- *lie-1, fabricated no-change (the weld-5 sin):* `aerospike_migrations` "code for
  check_mode simply returns `changed=false` without validating any state … without even
  connecting to the aerospike database" [A-cg-aerospike-2024]. Filed *by a
  community.general maintainer* (russoz) — i.e. the norm is: declaring support without
  implementing the predictive read-side is a bug, "arguably it does not support
  check_mode" at all.
- *lie-2, over-prediction (tolerated, chronic):* `pip` "always reports changed in check
  mode when installing with version range" even when installed [C-ans-pip-2025]; `apt`
  "made to return changed-status whenever APT cache update occurred" [C-so-apt-changed-2017];
  command/shell report changed-always [C-sf-cmd-changed-2019]. Annoying, trust-eroding,
  not dangerous — and per DeHaan it was a deliberate posture from the start:
  "Some ansible modules don't fully understand check mode and will report 'changed=True'
  automatically without running in check mode rather than risk making a change"
  [A-ans-forum-2014].
- *lie-3, check-mode crash:* `github_key` "fails when run in check mode … without the
  check flag, the module works fine" [C-cg-githubkey-2024] — the predictive path is a
  *separately-buggy second implementation*. Important general lesson: a declared
  check path is a second code path, and it rots independently of the real one.
- *lie-4, prediction-gap interactions:* `--diff` with `--check` silently lacking for
  sub-features (apt autoremove [C-ans-aptdiff-2024]); 2014-era false-positive diffs
  ("It makes me believe that what Ansible feeds to the diff is wrong, because of that
  --check option") [A-ans-forum-2014].
- *lie-5, mis-documentation of the declaration itself:* the docs render "Can run in
  check_mode and return changed status prediction without modifying target" even for
  modules with `support: none` [A-ansdoc-1006-2024] — even the *metadata display layer*
  can lie about the declaration.

**Trust norms that emerged (+SURE, from the corpus above):** the simulation is openly
disclaimed — "Check mode is just a simulation. It will not generate output for tasks
that use conditionals based on registered variables (results of prior tasks)"
[B-ans-checkmode-docs-2026] — i.e. Ansible *documents its own assumptions problem* and
hands the residue to the admin (`ansible_check_mode` magic variable to hand-patch
around it). Community practice treats check mode as a smoke test, not a contract;
wrong predictions are filed as module bugs (blame lands on the declaring module, good)
but are discovered by user confusion, not by any runtime cross-check (bad — no
mechanism ever compares predicted-changed against actually-changed on the next real
run). The `check_mode: false` escape hatch exists because admins *demanded* mutation
inside the simulation lane to keep later forecasts coherent (gather a fact for real) —
i.e. the fidelity-pressure on a non-mutating lane reliably produces a
mutation-escape-hatch. Dorc's probe lane explicitly refuses this (plan/apply promise);
expect the same pressure and pre-decide the answer.

## §2 (r-2) Chef why-run and Puppet noop: why dry-run-by-declaration disappointed

**Chef — the canonical critique [A-chef-whyrun-2018], full-read.** Dry-run was
requested at Chef's birth (CHEF-13, Jan 2009) "because both CFEngine and Puppet had
this feature"; "right from the beginning, everyone knew that such a feature has 'crazy
warts', based on prior experience with these tools"; shipped anyway in 10.14 (2012).
The product post (Julian Dunn, 2018) then recommends *against* its own feature:

- *The assumptions problem:* "no-op modes by definition can only observe resources in
  isolation and try to *forecast* what will happen based on that limited view."
  Canonical example: `package 'httpd'` then `execute … only_if 'rpm -q httpd'` —
  "Why-run mode will infer that only a single resource is going to change, because it
  has no way to evaluate the guard in the subsequent execute block to know that its
  value will change based on the *real* execution of the first resource."
- *Not side-effect-free:* "despite the name, no-op modes are not side-effect-free
  against systems generally" — war story: a customer's *nightly why-run cron job* was
  "randomly breaking production servers" because a buggy systemd "would occasionally
  lock up when interrogated about the state of running services even though 'no changes
  were being made'." Read-side interrogation at fleet scale guaranteed nightly outages.
  (Direct Dorc lesson: the probe lane's "non-mutating" is a *claim about intent*, not
  about consequences; massively-parallel probing multiplies low-probability read-side
  damage.)
- *Separation of duties:* "the infamous proverb 'trust but verify' only holds when the
  verification of the change is done by a different program – or even different teams –
  than the program or groups making the change" — hence InSpec as a separate verifier,
  and the explicit warning against importing the manager's own attributes into the
  verifier.
- Endgame: "we will consider the removal of why-run mode in a future major version."

**The buried treasure: Chef had a full author-declared-assumptions API**
[A-chef-whyrun-api-2012]. `Chef::Mixin::WhyRun::ResourceRequirements` "provides a
framework for making assertions about the host system's state" and "a mechanism for
making *assumptions about what the system's state might have been* when running in why
run mode." Authors write `a.assertion { … }`, `a.failure_message(…)`, `a.why_run("…")`
(the declared counterfactual: e.g. "if the init script is not available, and we're in
why run mode, assume that some previous action would've created it"), and
`a.block_action!`. In a real run the failed assertion raises; in why-run it logs a
warning, *assumes the declared counterfactual state*, and continues — "a more useful
approximation of what would happen in a real chef run." This is door-2's exact shape:
a per-kind, author-declared, counterfactual claim with a vouched stand-in state. The
arc to note: Chef *built the careful version* — scoped, per-resource, with declared
assumptions and warnings — and the compositional problem still sank the mode's
reputation within ~6 years, hard enough that marketing recommended against it.
(~SUSPECT on "sank" causality: the post emphasizes the testing-workflow alternative as
the bigger driver; both forces point the same way.)
Practitioner dissent existed even post-mortem: why-run as the fastest
edit-loop "in a production-like environment … time-to-deploy on production is lower
than any other way I've tried" [C-hn-whyrun-2018] — the demand that door-2 serves
(fast, mostly-right forecasting) does not go away just because the mechanism is impure.

**Puppet noop — which subsets survived.** Puppet's noop is better-reputed than why-run
(~SUSPECT, by community usage rather than any formal survey) and the reasons are
instructive:

- *It probes rather than forecasts where it can:* "When running in noop mode, Puppet
  will check whether each resource is in sync, like it does when running normally …
  will take no action, and will instead report the changes it would have made"
  [B-pe-noop-2025]. The prediction for declarative resources is grounded in a real
  read of present state — the surviving subset per c-3.
- *The read-side runs, by documented contract:* "The `onlyif` and `unless` commands of
  an `exec` are used in the process of determining whether the `exec` is already in
  sync, therefore they must be run during a noop Puppet run" [A-puppet-exec-2026]; and
  "even when running in noop mode, Puppet will still execute commands on the target
  node to determine the current state of each resource" [C-pup-users-2013]. Nobody
  treats this as scandal; it's the accepted cost of grounded prediction. (dq-errexit-3
  precedent, again.)
- *`exec` is the documented hole:* "Any command in an `exec` resource **must** be able
  to run multiple times without causing harm — that is, it must be *idempotent*" —
  pure convention, author-vouched, unverified. And the noop interactions are
  hair-raising even in the docs: "If the exec has `noop => true`, would otherwise have
  run, and receives an event from a non-noop resource, it runs once" [A-puppet-exec-2026]
  — a per-resource noop declaration is *overridden by refresh events* from non-noop
  resources. Mixing noop and enforce scopes has formally-documented surprise-execution
  semantics.
- *Cross-node blast radius:* with server-side noop, an exported concat fragment
  "is exported with noop = true" and collector nodes rebuild configs *without* the
  noop'd fragments — noop on node A corrupts node B's load-balancer config
  [A-ex42-noop-2017]. Their fix was scope mechanics: client-side noop (whole-node),
  plus a `no_noop` class parameter where "all the resources of the class are enforced
  and are applied whatever are the noop settings" (critical infra must never be
  simulated — DNS, the agent itself). Same shape as Ansible's `check_mode: false`:
  the simulation lane grew an enforcement escape hatch under fidelity/ops pressure.
- *Mode-interaction bugs at the engine level:* cached catalogs silently ignored when
  noop is set [C-pup-8258-2018]; a noop-configured node executing real changes due to
  config-read races [surveyed via search, ungraded] — even the *engine's own* handling
  of the declared mode has lied.

**The empirical base rate for author-vouched idempotence (+SURE, quoted verbatim):**
Hummer et al., Middleware 2013 — "Our extensive evaluation covers testing of roughly
300 publicly available, real-life Chef scripts. After executing 3671 test cases, our
framework correctly identified 92 of those scripts as non-idempotent in our test
environment" [A-hummer-2013] — ~31% of public, real-life cookbooks violated the
property the entire tool model rests on, in a community whose docs hammer idempotence.
Their verification approach — repeated execution from varied start states in
lightweight VMs — is the harness pattern (c-3, T-3) and is directly reusable for
verifying door-2 declarations (run mutator twice in a container; diff the declared
converged-run observables against the *measured second run*).

## §3 (r-3) Build-cache trust: Bazel, Nix, and declared hermeticity

**Bazel.** The declaration surface is the rule/action contract: declared inputs,
declared outputs, implicit "this action is a pure function of its declared inputs."
Undeclared-input lies are converted into loud local failures *only where sandboxing is
on*; cache poisoning concentrates exactly where it's off: "This non-hermeticity can
easily lead to cache poisoning when sandboxing is disabled (primarily under Windows or
by using `--spawn_strategy=local`)" — stale virtual-header symlinks produce a
*correct-at-the-time* dependency error which is then cached; "even after running
`bazel clean`, … Bazel will detect that inputs did not change (which is technically
true) and return the same, cached, error message" [A-bazel-28530-2025]. The poisoned
state outlives every local remediation ritual the user knows.

Bazel's own CI got poisoned in 2021: digest mismatch on
`platformclasspath.jar`, "developers repeatedly pressed the Retry button on PRs",
default branch red, and the issue thread contains *no root-cause attribution* —
remediation was "can we simply clear the cache?" [A-bazelci-1174-2021]. Attribution of
cache poisoning is the unsolved UX: the symptom (wrong/failed downstream build) is
arbitrarily far from the cause (one non-hermetic action, possibly weeks earlier,
possibly another machine).

The verification mechanism that emerged is execution-log diffing: build twice with
`--noremote_accept_cached`, `--execution_log_json_file=…`, diff the sorted logs to find
actions whose output hashes differ [A-merino-edet-2025]. Adoption reality: expert
tooling, expensive (logs can exceed 100GB on big builds [C-so-execlog-2023]), used
reactively when determinism is already suspected; Merino's own assessment is layered
pragmatism — "sandboxing isn't going to protect us from them" (perf/kernel limits), so
CI-side verification + hermetic toolchains + vigilance, not a solved problem. Google's
internal answer (~SUSPECT, folklore-grade) is to make the cache write-path trusted
(only CI writes), which is blast-radius scoping, not verification.

**Nix.** Two distinct declared-claim regimes:

- *Input-addressed + signed cache:* trust is in the signing key, not verification —
  "if the key used by cache.nixos.org is ever compromised, all builds that were ever
  added to the cache can be considered tainted"; "one needs to put either full trust or
  no trust at all in the build machines of a binary cache — there is no middle ground"
  [A-trustix-2020]. Reproducible-builds gives the *possibility* of checking — "they
  allow randomized or systematic checking to uncover tampered builds"
  [C-lobsters-nixcache-2025] — but stock Nix doesn't do it; Trustix built the
  verify-statistically layer: a Merkle append-only log per builder mapping input-hash →
  output-hash, compared across independent builders, with local, configurable
  acceptance policy ("what your Trustix considers trustworthy is up to you"), degrading
  gracefully for non-reproducible derivations (tracks them rather than pretending).
  Adoption reality (~SUSPECT): experimental/niche; the *pattern* matters more than the
  deployment.
- *Fixed-output derivations — the system's declared counterfactual:* the author
  declares "whatever command I specify, it will result in the given hash"
  [A-eigenvalue-fod-nd], buying network access (an impurity license) in exchange.
  Verification at first build is exact and self-attributing (hash mismatch errors name
  the derivation and print expected/got — community workflow handles upstream re-tags
  routinely [C-nixdisc-fodmismatch-2024]). The scar tissue is *suppressed
  re-verification*: "The commands of the derivations are ignored if we already have a
  matching derivation result!" — change the URL to `https://there.was.never.anything/here`,
  keep the hash, "the CI job would rubber stamp this lovely PR while a human would
  obviously object" [A-eigenvalue-fod-nd]. Even the explicit re-verification command
  has a carve-out: `nix-build --check … succeeds even though the output has changed`
  for FODs [A-nix-3369-2020] (stale, unresolved). Lesson for door-2: a declaration that
  *suppresses execution* must be keyed to every input the claim is sensitive to, or
  drift is structurally invisible; and your "re-verify" tooling must not inherit the
  suppression.
- *Content-addressing (RFC 062)* — the principled fix (validity decided by content, not
  promise) — remains experimental years later [B-nix-rfc62-2020]; the
  engineering-economics of retrofitting verification into a trust-based design are
  brutal (~SUSPECT as generalization).

## §4 (r-4) Idempotence/consistency claims: Terraform, Kubernetes, migrations

**Terraform — the strictest contract, the best attribution.** The provider protocol is
a declared-prediction contract: PlanResourceChange's result is *binding* — "If a
computed attribute has any _known_ value in the planned new state, the provider will be
required to ensure that it is unchanged in the new state returned by
`ApplyResourceChange`, or return an error explaining why it changed"; final-plan vs
initial-plan likewise constrained ("Any attribute that had a known value in the Initial
Planned State must have an identical value in the Final Planned State")
[A-tf-lifecycle-2026]. Terraform Core *cross-checks every apply* against the provider's
own plan and, on violation, emits the canonical blame-routed error:

> "Provider produced inconsistent result after apply … When applying changes to %s,
> provider %q produced an unexpected new value: %s. This is a bug in the provider,
> which should be reported in the provider's own issue tracker." [A-tf-blame-code-2026]

Enforcement was phased (c-7): "For historical reasons, the original Terraform SDK is
exempt from error messages produced when certain assumptions are violated"
[A-tf-lifecycle-2026]; legacy violations log `[WARN] Provider %q produced an invalid
plan … but we are tolerating it because it is using the legacy plugin SDK. The
following problems may be the cause of any confusing errors from downstream operations:`
[A-tf-legacywarn-2026] — tolerated, but *breadcrumbed for later attribution*. Support
reality: the error fires plenty (eventual-consistency races, API normalization);
official guidance assigns blame to provider implementation gaps and routes users to
"report this behavior to the provider maintainers … so they can implement the necessary
retry logic" [B-tf-support-2026]; meanwhile the user-facing cost is real (resource
dropped from state; recover via `terraform import`). Net: the cross-check did not make
providers honest, it made provider lies *cheap to diagnose and correctly addressed* —
thousands of provider-repo issues open with the error text as the title, filed against
the right party [C-tfaws-15512-2020 et al.].

**Kubernetes.** Controller idempotence is pure convention, not enforced: "the
controller's reconciliation loop needs to be idempotent", reconcile-the-whole-object
not events; violations "may lead to unforeseen consequences, such as resources becoming
stuck and requiring manual intervention" [B-kubebuilder-2026]. The architecture is the
mitigation: level-triggered re-reconciliation means a wrong single pass is *re-measured
and re-corrected next pass* — convergence-by-retry absorbs lies that would be fatal in
one-shot systems. (Dorc parallel: DESIGN.md's "we can always reserve the right to fail
back to multiple executions" is the same absorber, and door-4's
diverged-since-probe ⇒ real-mutator-runs is its per-site form.)

**SQL migrations — checksum-pinned declarations.** Flyway/Liquibase trust the author's
migration *as-of-its-recorded-text*: checksums of applied migrations are stored and
re-validated every run; edit an applied migration and `validate` fails loudly
("Migration checksum mismatch for migration version … Either revert the changes … or
run repair") [C-so-flyway-2014, B-flyway-validate-2026]. Two refinements matter here:
`repair`/`validCheckSum` are *explicit re-vouch* idioms — the author takes ownership of
a text change post-hoc, with documented hazards ("you will have 'old' changeset already
executed and 'new' changes … skipped. On an empty database your 'new' changeset will be
executed, so you will potentially have model inconsistencies" [C-so-validchecksum-2021])
— and the verifier itself versions badly: Liquibase 4.22's checksum-algorithm change
(whitespace normalization) broke validation workflows fleet-wide
[B-liquibase-checksum-2024]. Pinning works; pinning-format churn is its own incident
class.

## §5 (r-5) Disclosure/attribution UX when a declared claim fails

Ranking the surveyed ecosystems by blame-surface quality (+SURE on the ordering's
endpoints, ~SUSPECT on the middle):

1. **Terraform** — *names the liar in the error*, states the contract violated, routes
   the report ("provider's own issue tracker"), and keeps a tolerated-legacy breadcrumb
   in logs pre-empting "confusing errors from downstream operations". The error is
   raised *at the moment of divergence*, adjacent to the cause.
2. **Nix FOD mismatch** — exact, content-addressed, names the derivation, prints
   expected/got; failure is local to the lying declaration. (But: only fires when the
   build actually re-runs — the suppression hole above.)
3. **Flyway/Liquibase** — names the migration version and both checksums; fails the run
   *before* touching the database; remediation verbs (`repair`, `validCheckSum`) are
   first-class.
4. **Puppet noop / Chef why-run** — honest *mode-level* disclaimers, per-resource
   warnings in why-run ("Assuming …" lines render the assumption visibly — a genuinely
   good idiom), but no post-hoc reconciliation of forecast-vs-actual; wrong forecasts
   are discovered by humans noticing.
5. **Ansible check mode** — no runtime cross-check exists or is possible (the
   prediction and the real run never co-occur); lies surface as user confusion
   ("check shows changes that won't actually happen"), get triaged by maintainers from
   symptom descriptions, and the support-level metadata itself has rendered wrong
   [A-ansdoc-1006-2024]. Blame *does* eventually land on the right module via the issue
   tracker — the module boundary is a good blame boundary — but the path runs through
   blind user debugging.
6. **Bazel remote cache** — the floor: digest-mismatch tells you *that* poison exists,
   nothing about which action/author/machine minted it; remediation is flush-everything;
   trust-recovery is fleetwide and slow [A-bazelci-1174-2021, A-bazel-28530-2025].

**Industrial-psych grounding.** Lee & See: reliance is governed by trust; appropriate
reliance requires *calibration* (trust ∝ true capability) and *resolution*; "trust
guides reliance when complexity and unanticipated situations make a complete
understanding of the automation impractical" [B-leesee-2004] — precisely the orchestrator
condition. Error-type asymmetry: "False alarm prone automation also clearly affected
both operator compliance and reliance, while miss-prone automation only appeared to
affect [reliance]" [C-dtic-favsmiss-2006]; FA-rate effects on compliance/reliance are
mediated by trust [C-chancey-2017]; repeated false alarms ⇒ cry-wolf/alarm-fatigue,
quantifiable via false-discovery rate [C-fdr-crywolf-2025]. No ecosystem-specific
quantified trust-recovery study surfaced for IaC dry-runs (--WONDER whether one exists;
nothing found in two targeted searches) — the closest is the revealed-preference
evidence: Chef's vendor abandoning why-run, and DeHaan's over-predict-changed norm
(deliberately trading chronic false alarms for zero misses — defensible for state
safety, corrosive for prediction trust, and a root cause of "check mode is just a
smoke test" culture).

## §6 (r-6) Taxonomy of declaration-trust mechanisms

- **T-1 verify-eagerly** (cross-check the claim inline, every use): Terraform
  plan/apply consistency checks; Flyway/Liquibase checksum validation per run; Bazel
  content-digest verification on cache download (detects, doesn't attribute);
  Nix FOD hash check *at first build*. Costs ~zero when claims are honest; converts
  lies into attributed, adjacent errors. Only viable when the engine can observe the
  ground truth at use-time.
- **T-2 verify-lazily / on-demand** (expert-invoked audit): `nix-build --check`
  (with the FOD hole); Bazel execution-log double-build diffing; Hummer-style harness
  runs invoked by CI. High cost, reactive, finds root causes; adoption limited to
  specialists and post-incident archaeology.
- **T-3 verify-statistically / by-diversity** (sample or compare independent
  derivations of the same claim): Trustix cross-builder consensus;
  reproducible-builds spot-checking; canary-node noop/enforce splits
  [A-ex42-noop-2017]; (proposed for Dorc: sampled door-4-under-door-2, c-9). Buys
  calibration without per-use cost; needs a second independent source of truth.
- **T-4 never-verify-but-attribute** (trust the declaration; record provenance so
  failure finds it): Terraform's legacy-SDK tolerated-WARN breadcrumb; Chef why-run's
  rendered "Assuming …" lines; 20V §5's disclosure floor ("line 14 elided per
  package-oracle's converged-claim"). Cheapest; the *only* option for genuinely
  counterfactual claims at the moment of use; quality determined entirely by how
  reliably the post-hoc path from symptom to declaration is paved.
- **T-5 scope-the-blast-radius** (constrain what a wrong declaration can corrupt):
  Bazel sandboxing (lie ⇒ local failure, not poisoned cache) and trusted-writer caches;
  Ansible's tri-level support + do-nothing-default + per-task overrides; Puppet
  client-side-noop-only and `no_noop` critical classes; Liquibase validCheckSum being
  per-changeset rather than global. Orthogonal to verification; determines whether a
  lie is an incident or an outage.

Every surveyed ecosystem ended up running a *portfolio* (+SURE): e.g. Terraform = T-1
(checks) + T-4 (legacy WARN) + T-5 (per-resource state isolation); Nix = T-1 (first
build) + T-4 (signatures) + aspirational T-3 (Trustix) — and the holes line up with
whatever the portfolio lacks (Nix lacks re-verify ⇒ FOD staleness; Ansible lacks any
T-1/T-2 ⇒ blind debugging; Bazel-without-sandbox lacks T-5 ⇒ poisoning).

---

## §7 Design consequences for Dorc

**Locating the doors (and dq-errexit-3) in the taxonomy.**

- *door-4 (constructed guard)* is **T-1 verify-eagerly**: the declaration
  (converged-run-equivalence) licenses a transform whose runtime behavior *re-measures*
  the world (live rc provenance) and runs the real mutator on divergence. Prior art
  says this is the only family that stayed trusted at runtime; 20V's instinct to make
  door-4 the keystone, with door-2 the fallback, is exactly what the scar tissue
  predicts. Door-4's four-world trace is structurally Terraform's
  plan-check-but-better: where Terraform can only *error* on divergence, door-4
  *self-corrects* (kFAIL-perform by construction).
- *door-2 (static declared converged-run)* is **T-4 never-verify-but-attribute** at
  use-time. Its prior-art siblings are Chef's `a.why_run` assumptions (careful,
  per-kind, rendered — still reputationally drowned by composition) and Nix FODs
  (exact-once verification, then suppression ⇒ silent rot). Both siblings say: T-4
  alone is not enough for a claim that *suppresses execution*. The 20V plan already
  contains the two fences prior art demands — the disclosure floor (§5: verdict-lane +
  artifact-comment attribution) and analytic pre-validation via door-1-on-wrappers
  (§4: "validates its semantics analytically before declarations are trusted") — the
  latter is the Hummer/TestKitchen **T-3-ex-ante** move. Name them as such and keep
  both load-bearing, not optional polish.
- *dq-errexit-3 (guard-insertion = oracle code running at sites the book didn't
  spell)*: prior art is unanimous that the read-side of a kind is the kind-author's
  code and it runs in *both* lanes — Puppet `onlyif`/`unless` execute during noop *and*
  during real runs; Ansible modules' check path and real path are the same authored
  artifact; Chef guards likewise. No surveyed ecosystem treated this as a crossed line;
  what they required instead: (i) the inserted code is the same trust-object already
  vouched for probing (Dorc: the kind's structurally-vouched probe body — satisfied by
  construction), (ii) it is disclosed where the admin reads (Dorc: idiomatic-sh
  artifact diff — satisfied, and *stronger* than prior art since the admin could have
  written it), (iii) its own failure attributes to the declaring oracle, not the book
  (must be built — see m-2). Verdict this research supports (~SUSPECT, human's call):
  same-trust-extended, not a line crossed; the default-OFF flag matches the universal
  phased-enforcement pattern (c-7).
- *dq-errexit-2 (oracle owns the bare-middle default)*: Ansible and Puppet both
  landed on exactly 20V §5's stack — module/kind owns the default
  (`supports_check_mode`, resource-type semantics), admin overrides per-site
  (`check_mode:`/`noop =>`), engine never flips globally. Twenty years of combined
  operation produced *module-level* blame routing as the chief benefit and
  wrong-declarations-as-ordinary-bugs as the chief cost — a cost that stayed bounded
  *because* the blame boundary matched the declaration boundary. Supports LEANING-yes.

**What the scar tissue predicts for us (p-slugs):**

- *p-1, the consumed-stdout cell is the assumptions problem reborn.* Chef died on
  forecast-state feeding later guards. Door-2's declared body emits stdout; the moment
  a book *branches on* that declared output (`if apt-get install -y nginx | grep -q
  'newest version'`), a wrong declaration steers real control flow — under-execution or
  mis-execution beyond the canary trade. Prior art says treat
  declared-rc (small, checkable domain) and declared-stdout-consumed-by-control-flow
  (open domain, version/locale-sensitive) as *different risk classes*. Concretely:
  apt's "is already the newest version." text varies with version and locale —
  a declaration of exact bytes is the most rot-prone claim-shape in this entire survey
  (~SUSPECT; LANG-sensitivity +SURE, message-drift across apt versions -GUESS but
  cheap to check). Consider restricting door-2's sanctioned channels to rc (+
  stdout-only-when-unconsumed) for the spike.
- *p-2, declarations rot on the verifier's schedule, not the author's.* FODs rot when
  upstream moves; Liquibase checksums "rot" when the checksum algorithm changes; check
  paths rot as second implementations (lie-3). Door-2 declarations are
  "version-sensitive, stale-able" by 20V's own words — pin them (m-3) and give the
  pin a place to be re-vouched.
- *p-3, correlated failure is the reputational cliff.* One popular oracle's wrong
  converged-run claim fails *identically across every book and host using it* — the
  exact correlated-failure profile that turns "a bug" into "the tool lies" (c-8; Chef's
  fleet-wide nightly outages from one systemd interrogation bug). The per-door
  dashboard attribution (20V §7) is not reporting polish; it is the trust-calibration
  mechanism, and it must work *fleet-wide* (aggregate "this oracle's declaration elided
  N sites across M hosts") so a single rot event reads as one cause, not M incidents.
- *p-4, the simulation lane will be asked to mutate.* Ansible grew `check_mode: false`;
  Puppet grew `no_noop`; both because admins needed real facts mid-simulation for the
  rest of the forecast to cohere. Dorc's plan/apply promise forbids it (README:
  even probe-causes-eventually-desirable-mutation is verboten). Keep refusing, but
  expect the pressure and have the answer ready: Dorc's equivalent escape is *run the
  plan sooner on a narrower scope*, not *let the probe write*.
- *p-5, read-side at fleet scale is its own hazard class.* Chef's why-run cron
  systemd-lockup outage: "no changes were being made" yet production broke nightly.
  Massively-parallel probing of oracle Query bodies is Dorc's core move; a per-kind
  "interrogation is heavier than it looks" failure (locking package dbs, hammering a
  daemon's status socket) is predictable. Worth a future knob: probe concurrency/jitter
  per kind (-GUESS on mechanism; the hazard itself +SURE by precedent).

**Mechanisms worth stealing (m-slugs):**

- *m-1, tri-level declared support with prose details* (Ansible attributes block):
  oracle converged-run declarations should carry `full | partial-with-conditions |
  none` rather than boolean, the partial conditions being sh-spelled guards
  (`creates:`-analogue: "only when invoked with -y and a single package arg") — which
  is exactly kind-shape narrowing, already Dorc-idiomatic.
- *m-2, the blame-template error* (Terraform): when a door-2/door-4 site's residual
  reality contradicts the declaration (door-4's guard says diverged but mutator then
  no-ops; post-hoc failure under an elided site; sampled cross-check mismatch), the
  error/verdict text should name the oracle, the declaration, the site, and the
  reporting route: "line 14 elided per package-oracle's converged-claim (declared
  oracle X vY); this is a bug in the oracle, report to X" — plus the tolerated-WARN
  breadcrumb form for default-OFF mode.
- *m-3, checksum-pin the declaration* (Flyway/FOD lesson): bind each accepted
  converged-run declaration to (oracle source hash, kind, and — where statable — tool
  version probe). On mismatch: declaration lapses to r-2 behavior (door-1 only) with a
  verdict-lane notice, and a `validCheckSum`-shaped re-vouch idiom exists. Avoid
  FOD's mistake: the *site key* must include everything the claim is sensitive to, and
  any future `--check`-like re-verifier must not inherit elision.
- *m-4, sandbox-converts-lies-into-local-failures* (Bazel): the planned DX tooling
  (containerized oracle TDD, eBPF dependency surfacing) is the T-5 layer — its absence
  is where lying declarations become fleet incidents. Prioritize the harness that runs
  a kind's mutator twice and diffs declared-vs-observed second-run observables
  (Hummer's exact protocol, narrowed to door-2's claim) — this makes declarations
  *testable by their authors* before anyone trusts them.
- *m-5, sampled live cross-check* (Trustix shape, c-9): where door-2 statically elides,
  occasionally emit door-4's guard instead (engine-chosen sample; apply-lane, so no
  probe-purity issue). Disagreement (guard says diverged) is recorded as
  declaration-suspect — converting silent rot into early small signals. Cost is
  bounded and order-sacred (door-4 is in-line anyway). (-GUESS: may interact with
  plan-presentation determinism — the plan must disclose "sampled verification site"
  honestly; defer if it muddies plan/apply.)
- *m-6, render the assumption where the human reads* (Chef why-run's one good UX idea):
  the plan artifact already gets comments; ensure every door-2 site's comment carries
  the *counterfactual text itself* ("assumes converged re-run ⇒ rc=0, 'already newest'")
  not just "elided per oracle" — admins can veto a wrong assumption only if they can
  read it.

**One deliberate divergence to record:** prior art's surviving dry-runs all degrade by
*not predicting* (Ansible do-nothing, Puppet exec-hole) — their conservative direction
is "silent coverage gap" because their default lane is simulation. Dorc's r-0 is "runs,
full stop" — the conservative direction is *execution*, because our default lane is
apply. This inversion is why the DeHaan norm (over-predict change) maps for us onto
"under-elide" — same weld-5 spirit, opposite lane — and why Dorc's coverage gaps are
*louder but safer* than Ansible's. The dashboard should sell that honestly: an
unelided site is a missing declaration, never a wrong one.

---

## §8 Graded source list

Grading: A = primary/load-bearing, full or deep targeted read · B = official docs /
primary partially read · C = community/secondary, snippet-to-moderate read · D =
surveyed only/unread. (Issue threads graded as primary evidence of ecosystem
experience.)

**r-1 Ansible**
- [B-ans-checkmode-docs-2026] docs.ansible.com — Validating tasks: check mode and diff mode. Targeted read.
- [A-ans-command-attrs-2026] ansible/ansible `lib/ansible/modules/command.py` attributes block. Verbatim.
- [B-ans-apt-attrs-2026] ansible/ansible `lib/ansible/modules/apt.py` attributes block. Verbatim.
- [A-ansdoc-1006-2024] ansible/ansible-documentation#1006 — check_mode attr description vs support status. Targeted read.
- [A-cg-aerospike-2024] ansible-collections/community.general#9448 — declared support, fabricated changed=false. Targeted read.
- [A-ans-forum-2014] forum.ansible.com t/16153 — "check shows changes that won't actually happen"; DeHaan norms. Targeted read.
- [C-ans-pip-2025] ansible/ansible#85592 — pip always-changed in check. Snippet.
- [C-cg-githubkey-2024] community.general#9185 — fails only in check mode. Snippet.
- [C-sf-cmd-changed-2019] serverfault 976424 — command/shell changed-in-check. Snippet.
- [C-so-apt-changed-2017] stackoverflow 41473289 — apt changed on cache update. Snippet.
- [C-ans-aptdiff-2024] ansible/ansible#83626 — autoremove diff in check. Snippet.
- [C-ans-aptyum-2018] forum 27495 — apt/yum check-mode surprises. Snippet.
- [D-shuklin-2019] Shuklin, "Understanding Ansible's check_mode" (Medium; 403 — snippets only).

**r-2 Chef / Puppet**
- [A-chef-whyrun-2018] Dunn, chef.io — "Why 'Why-Run' Mode Is Considered Harmful." FULL READ. The canonical critique; CHEF-13 archaeology, assumptions problem, systemd war story, separation-of-duties.
- [A-chef-whyrun-api-2012] rubydoc Chef::Mixin::WhyRun::ResourceRequirements — the declared-assumptions API. Targeted read.
- [C-hn-whyrun-2018] HN 16855918 (via Algolia API) — lone practitioner defense of why-run iteration speed. Full read (1 comment).
- [A-puppet-exec-2026] puppetlabs/puppet `references/types/exec.md` — idempotence obligation; onlyif/unless run under noop; noop×refresh wart. Verbatim, 2 slices.
- [B-pe-noop-2025] Puppet Enterprise docs — noop simulation semantics. Snippet.
- [A-ex42-noop-2017] example42 — "noop, no-noop and the path to safe Puppet deployments"; exported-resource noop poisoning; no_noop classes. Targeted read.
- [C-pup-users-2013] puppet-users Google Group — noop still executes state-checks. Snippet.
- [C-pup-8258-2018] PUP-8258 — cached catalogs ignored under noop. Snippet.

**r-3 build caches**
- [A-nix-3369-2020] NixOS/nix#3369 — --check doesn't validate FOD hashes; stale. Targeted read.
- [A-eigenvalue-fod-nd] blog.eigenvalue.net — "Re-running fixed output derivations at the right time." Deep read. (Undated post; -GUESS ~2023-24.)
- [B-bmcgee-fod-2023] bmcgee.ie — FOD explainer. Snippet.
- [C-nixdisc-fodmismatch-2024] NixOS Discourse 52353 — upstream force-push hash mismatch workflow. Snippet.
- [A-bazel-28530-2025] bazelbuild/bazel#28530 — virtual-header non-hermeticity ⇒ cache poisoning incl. post-clean persistence. Targeted read. (Year from issue range; -GUESS.)
- [A-bazelci-1174-2021] bazelbuild/continuous-integration#1174 — "Remote cache poisoned?"; digest mismatch, retry culture, flush remediation, no attribution. Targeted read.
- [A-merino-edet-2025] Merino, Blog System/5 — "Bazel and action (non-)determinism"; execution-log diff protocol; layered-pragmatism verdict. Targeted read.
- [B-bazel-cachedocs-2026] bazel.build — debugging remote cache hits. Snippet.
- [C-so-execlog-2023] stackoverflow 77566430 — 100GB execution logs. Snippet.
- [A-trustix-2020] Tweag — Trustix announcement; single-key critique; cross-builder consensus; configurable local policy. Targeted read.
- [C-lobsters-nixcache-2025] lobste.rs 8nyk1p — "Stop Trusting Nix Caches" discussion. Targeted read.
- [B-nix-rfc62-2020] NixOS/rfcs 0062 + NixOS/nix#4087 + haskell.nix CA tutorial — CA-derivations status. Snippets.
- [C-groups-stamping-nd] remote-execution-apis group — stamping + remote cache = trivial poisoning. Snippet.

**r-4 idempotence/consistency contracts**
- [A-tf-lifecycle-2026] hashicorp/terraform `docs/resource-instance-change-lifecycle.md` — the provider prediction contract; legacy-SDK exemption. Deep read (230/375 lines).
- [A-tf-blame-code-2026] hashicorp/terraform `internal/terraform/node_resource_abstract_instance.go` — "inconsistent result after apply … bug in the provider" text. Verbatim.
- [A-tf-legacywarn-2026] same repo — "[WARN] … tolerating it because it is using the legacy plugin SDK." Verbatim.
- [B-tf-support-2026] HashiCorp KB 1500006254562 — present-but-now-absent; remediation + blame framing. Targeted read.
- [C-tfaws-15512-2020] terraform-provider-aws#15512 — "changed the planned action from CreateThenDelete to DeleteThenCreate. This is a bug in the provider." Snippet.
- [B-kubebuilder-2026] book.kubebuilder.io good-practices — idempotent, level-triggered reconcile; convention-only. Targeted read.
- [B-flyway-validate-2026] Redgate Flyway docs — validate command. Snippet.
- [C-so-flyway-2014] stackoverflow 23776706 — checksum mismatch + repair semantics. Snippet.
- [B-liquibase-checksum-2024] liquibase.com checksum guide + docs — incl. 4.22 algorithm change fallout. Snippets.
- [C-so-validchecksum-2021] stackoverflow 66303898 — validCheckSum divergence hazard. Snippet.
- [C-liquibase-5761-2024] liquibase/liquibase#5761 — checksum-upgrade compatibility. Snippet.
- [A-hummer-2013] Hummer/Rosenberg/Oliveira/Eilam, Middleware'13 — "Testing Idempotence for Infrastructure as Code." Abstract full-read (HAL) + headline numbers verbatim via Springer/scispace PDF mirrors: ~300 real Chef scripts, 3671 test cases, 92 non-idempotent.

**r-5 trust-psych**
- [B-leesee-2004] Lee & See, Human Factors 46(1) — "Trust in Automation: Designing for Appropriate Reliance." Abstract read (PubMed); calibration/resolution framing.
- [C-dtic-favsmiss-2006] DTIC ADA496817 — false alarms vs misses asymmetry on compliance/reliance. Snippet.
- [C-chancey-2017] Chancey et al. — trust mediates FA-rate → compliance/reliance. Snippet.
- [C-fdr-crywolf-2025] ScienceDirect S0951832025010907 — FDR as cry-wolf quantifier. Snippet.
