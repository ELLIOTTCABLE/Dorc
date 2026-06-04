# Terraform user-study + cross-tool synthesis (completes Space 3)

Companion to notes/061 (Ansible, anchored on the Carreira 2025 study). Terraform has no single equivalent peer-reviewed anchor; sentiment is distributed across HN threads + practitioner blogs (graded individually). Quotes/URLs captured here = the durable record.

## Terraform — what's loved / kept (sacred cows)
- **`plan`/`apply` preview** — the validated UX (round-2 borrowed it). "preview changes before making them."
- **State as tracked truth** — "maintains state so scripts run repeatedly without unnecessary changes," "keeps locks so users can't both modify simultaneously," and the **mental model / written record** of all infra. NB: people love the *concept* (knowing current-vs-desired) even while hating the *implementation* (see below).
- **Declarative diffing / idempotence-by-state.** The "Confessions of a Terraform Hater → love" arc (medium) is a recurring genre: grudging acceptance once the mental model clicks.
Grade: HN threads (27436605 "underappreciated"; 37004617 "best practices at scale") A-ish for raw sentiment; Medium retrospectives B (human, opinion).

## Terraform — what failed / cost them long-run (the expensive implementation choices)
- **State is the greatest strength AND the biggest liability.** Monolithic state → single point of contention (engineers queue to plan/apply); **`plan` becomes 15–30 min at thousands of resources** ("stops being a quick feedback loop, starts being a liability"); locking (S3+DynamoDB) is a band-aid that "doesn't solve the underlying architectural issue"; **secrets leak into state**; performance degrades as state grows. (InfoWorld: "infrastructure-as-code becomes infrastructure-as-complexity.")
- **Slow feedback loops** vs software dev ("you can set up automated test suites and get instant feedback" — Terraform can't).
- **"Great until it doesn't work, after which the mess requires a lot of surgery."** Drift, provider surprises, risky refactors (move/rename = state surgery).
- **HCL's expressiveness ceiling** drove escapes: provisioners/`null_resource` (the fuzzy edges), and upward to CDK/Pulumi ("IaC in a real language").
- **BSL relicensing (2023) → OpenTofu fork** ("The Beginning of the End for Terraform?", HN 40182037) — community-trust cost of a governance choice.
- Ownership friction: "Who Should Write the Terraform?" (HN 32396892) — dev-vs-ops boundary.

## Cross-tool synthesis (ranked by what Dorc should learn)
### Costly long-run implementation choices to AVOID (highest-value lessons)
1. **YAML/Jinja (or any config-language) as the programming substrate.** Ansible's #1+#4 pains; the Carreira authors explicitly conclude "to introduce higher-level language concepts, we must go beyond configuration languages." Dorc's sh-superset is the direct answer. *(Also indicts k8s YAML.)*
2. **A central/monolithic state store as the architectural center** (Terraform). Contention, slow plans, secrets, locking band-aids. Dorc **probes real host state** → no central state to contend (go/no-go: AVOID state backend).
3. **Statelessness with no record of what was established** (Ansible). Can't undo/delete/detect-drift ("deletes are hard… nothing knows what to undo"). Dorc's **derived** establish/kill-fact model gives knowing-what-to-undo *without* a Terraform-style central store — the synthesis of both tools' best property minus both implementations' cost.
4. **An execution model that fights performance** (Ansible: per-task SSH + Python; sequential). Dorc: compositional summaries + parallelism-from-dependencies + sparse probing (round 2).
5. **Declarative-without-an-imperative-escape** → forces the provisioner/`null_resource`/`local-exec`/k8s-Job jank (the fuzzy edges). Dorc is imperative-first with derived declarative *properties*.

### Sacred cows to PRESERVE (won't give up)
- From Ansible: **agentless-ssh**, **idempotence**, **low-barrier/readable** ("small set of concepts"), **heterogeneity** (one tool, many OS/targets).
- From Terraform: **plan/apply preview**, **knowing current-vs-desired** (drift/diff), the **reviewable written record**.
- **The Dorc bet, stated precisely**: deliver the loved *properties* — plan/apply, idempotence, agentless, current-vs-desired knowledge, reviewable diff — via *different implementations* — a **derived** check-projection (not declared state), **probing real state** (not a state file), an **sh-superset** (not YAML/Jinja), **compositional analysis** (not per-task SSH). Loved-properties ∩ ¬hated-implementations.

### Catastrophic-failure pattern (the stakes)
IaC bugs cause large-blast-radius outages — GitHub DNS, GitLab, Reddit, Rust DNS, AWS S3 **$150M typo** (all cited in the Carreira study). Privileged/root execution + small errors = disaster. → Validates **elision-soundness** (a silently-wrong skip isn't a slow build, it's an outage) + the observability/calibration emphasis. Probe-soundness — never mutating during a "read-only" plan — is the co-equal other half (see `kFAIL`).

## Net (Space 3 conclusion)
The two tools' pains are *complementary* and Dorc's design sits exactly in the gap: Ansible users want **real control flow + state + debugging + speed** (which Dorc's language+engine give); Terraform users want **plan/apply + current-vs-desired without the central-state tax** (which Dorc's probe-derived check gives). Nobody wants to give up **agentless + idempotence + plan/apply + readability** — and Dorc keeps all four. The single most-repeated practitioner escape hatch, across both tools, is **"drop to a shell script"** (to debug, to do the imperative thing, to escape YAML/HCL) — which is *literally Dorc's substrate*. That's the strongest market signal in the whole round.
