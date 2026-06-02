# Round 3 — userbase & problem-space research map

Frame: assume a good analysis engine (round 2) + good language path (next) + a good community. Validate against the real userbase/problem-space **without losing the eventual goal: a better imperative / non-declarative orchestrator.** Grade every source (human practitioner > vendor doc > AI listicle slop — the web is *full* of slop here). Recency-weighted (sentiment/tools shift) but keep durable classics. Save real human sources to `Research/`.

## Space 1 — test-data corpus (DEEP; deliverable = a PLAN + optional light tooling spike, NOT bulk download this session)
Two subtly-different targets, both ≠ the CoLiS Debian maintainer-script corpus:
- **1A — "ops corpuses"**: whole-org gitops/IaC trees — Ansible playbooks + entire role-trees, Terraform monorepos, k8s manifests, NixOS configs. Seek **awkward jank Dorc could solve**, especially the **"fuzzy edges"**: where Ansible/declarative tools were so bad a team **shoehorned Terraform/K8s/Nix into doing imperative-orchestration (Dorc-y) jobs** they're ill-suited for — `local-exec`/`null_resource`/`remote-exec` provisioners, Terraform-as-script-runner, k8s `Job`/`initContainer`/`postStart` as imperative steps, Helm hooks, Ansible `command`/`shell`/`raw` sprawl, `creates:`-guarded shell, etc.
- **1B — "provider corpuses"**: Ansible Galaxy collections/roles (+ non-Ansible equivalents) — where people invested real work *orchestrating* roles/tasks imperatively. This is the "what role/oracle authors actually wrestle with" corpus (round-1 found `community.general` is mostly thin idempotent CLI-wrappers — confirm/extend).
- **Use** → derive validation criteria (deterministic and/or LLM-assisted); rewrite real tasks into Dorc; hunt corner-cases that get awkward in our design.
- Plan must cover: candidate sources, **sampling strategy** (by stars/activity/domain), **licensing/ToS** (we'll redistribute/derive from these — permissive matters), how to **derive validation criteria** from a corpus item, and the **tooling-spike shape**.
- Tooling spike: a careful `gh`/`curl` harness — **do NOT hit rate-limits this session**; validate gently (≤a handful of calls) or leave un-run for the user to review/run later.

## Space 2 — orchestration feature go/no-go (MINOR pass)
Enumerate every fundamental function of tools in this space; for each, a founded **integrate-vs-delineate** decision:
- **INTEGRATE** iff the feature benefits from conflation with {analysis/extraction engine + sh-superset language/parser + core evaluation-planner} enough to keep in mind while planning.
- **DELINEATE** iff cleanly separable AND (limited benefit to our target uses OR well-covered by do-one-thing-well external tools).
Function inventory to assess: inventory/host-mgmt; transport/connection (ssh/ControlMaster/winrm/local); become/privilege-escalation; secrets/vault; facts/gathering & caching; vars/templating (Jinja); handlers/notify; check/diff mode; tags/`--limit`; serial/rolling/throttle/`run_once`/delegation; async/poll/long-running; retries/until/delay; callbacks/output/structured-logging/realtime-streaming; error-handling/blocks/rescue/`any_errors_fatal`; collections/distribution/registry/versioning; modules-vs-raw; idempotency; (Terraform) state-backend, plan/apply, provider/resource graph, drift-detection, import, workspaces, locking. Goal: keep the planner honest about what's core vs ceded, per the planning-log's "not an orchestrator" stance — but eyes-open about the eventual orchestrator goal.

## Space 3 — user-study: Ansible & Terraform (SUBSTANTIAL)
What failed; which implementation-choices cost them long-run; what people **love and won't give up**. Sources: HN/Lobsters threads, practitioner blogs/retrospectives, conference talks/postmortems, surveys (SO Developer Survey, CNCF, HashiCorp State-of-Cloud-Strategy), vendor-neutral "lessons learned." Grade hard. Recency-weighted, durable classics kept. Target output: a ranked "failures / costly-choices / sacred-cows" synthesis that feeds Space-1 (what jank to hunt) and Space-2 (what to integrate vs cede).

## Deliverables (gated on coverage)
- `plans/063-corpus-acquisition-plan.md` (+ optional `tools/corpus-*` spike, un-run or gently-validated).
- `plans/064-orchestration-go-no-go.md` (the matrix + per-feature rationale).
- `notes/06x-*` user-study synthesis (failures / costly choices / sacred cows).
