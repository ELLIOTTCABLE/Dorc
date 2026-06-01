# Test-data corpus acquisition plan (Space 1)

> ⟢ 2026-06 — operational updates (current guide: charter §9 + `Research/plans/spike-session-prompt.md`): (1) **contrast-not-compound** — sample representative *public* corpora, **not** the user's own scripts (which already encode the design's biases); (2) corpus data lives **out-of-tree** (a cache + an in-tree manifest with pinned sources + checksums), never in the synced tree (avoids the `Vendor/` churn/lock pain); (3) gathering a large representative Ansible collection is a real research subtask — academic curated sets first, then Galaxy/GitHub for breadth; (4) **provenance** — treat as a research study; thread raw numbers + method through every takeaway.

Goal: bootstrap substantial evaluation/testing/comparison for **both** Dorc language-usages — **(1A) ops corpuses** (whole-org gitops/IaC trees; hunt awkward jank + "fuzzy edges") and **(1B) provider corpuses** (Galaxy collections/roles + equivalents; real imperative orchestration of tasks). Both ≠ the CoLiS Debian-maintainer-script corpus. Eventual use: derive validation criteria (deterministic + LLM-assisted), rewrite real tasks into Dorc, mine corner-cases. **This session = plan + a non-rate-limit-hammering tooling spike; bulk acquisition deferred for the user to review/run.**

## Key strategic insight: bootstrap from curated academic corpora before scraping GitHub
The IaC-research community has already scraped, deduped, and **labeled** large corpora — reusing them is faster, cleaner, and *comes with validation criteria* (defect/smell labels). Scrape fresh GitHub only for what they don't cover (the "fuzzy edges").

## Source inventory (graded)

### Tier 1 — curated, labeled academic corpora (grade A; research-licensed; start here)
- **Opdebeeck — "Static Analysis for QA of Ansible IaC" (VUB PhD 2024)** + MSR'22/'23 papers. **15,000+ Ansible scripts**, and a **Program Dependence Graph (control+data flow) model of Ansible** — both a corpus *and* a direct round-2 analysis precedent (closest existing "CFG/PDG of ops code"). PDF: `soft.vub.ac.be/Publications/2024/vub-soft-phd-25102024-rubenopdebeeck.pdf`; data via `ropdeb.ee/publications`. **Pull first** (overlaps engine round too).
- **GLITCH (Saavedra & Ferreira)** — polyglot IaC IR over **Ansible/Chef/Docker/Puppet/Terraform**; **replication package with 3 manually-annotated oracle datasets** (9 security + 9 design/impl smells). arXiv 2205.14371 / 2308.09458; `joaoff.com` (same group as the Carreira user-study). The annotated oracles = ready validation labels; the polyglot IR is a design reference for cross-tool corpus normalization.
- **Rahman "Gang of Eight" defect taxonomy** (ICSE'20) + 2025 replication (arXiv 2505.01568). Labeled defects from **1,448 OpenStack defect-commits + 80,425 commits / 291 repos**, **including an explicit `idempotency` defect category**. `chrisparnin.me/pdf/GangOfEight.pdf`, NSF PAR 10139112. → idempotency/defect labels are directly our validation criteria.
- **Carreira et al. 2025 user-study replication package** (Zenodo 10.5281/zenodo.15031088) — 59k labeled Q&A posts (challenge topics) — corpus of *pain*, not scripts, but useful for corner-case targeting.
- **Begoug et al.** IaC-on-StackOverflow dataset [7]; **InfraFix** (arXiv 2503.17220) technology-agnostic IaC repair (a "rewrite/validate" precedent).

### Tier 2 — "ops corpuses" (1A): fresh, human, fuzzy-edge-rich (grade A for realism; license-filter for redistribution)
- **Homelab-GitOps community** — the user's actual demographic. GitHub topics **`k8s-at-home`**, **`homelab`** (`?l=hcl`), **`home-ops`**; exemplar repos: `ThonyPrice/home-ops`, `ahinko/home-ops`, `blackjid/home-ops`, `angelnu/k8s-gitops`, `IAreKyleW00t/k2net` (Terraform+Ansible+k3s), `clearlybaffled/homelab`. Whole-org-ish trees mixing Ansible+Terraform+k8s+Flux/ArgoCD — exactly the fuzzy-edge target. The `onedr0p/cluster-template` lineage spawns hundreds of near-clones (good for variety + dedup practice).
- **Public org gitops monorepos** (companies that open-source infra) + `awesome-ansible` / `awesome-tf` curated lists.
- **The "fuzzy edge" signatures to grep** (imperative work shoehorned into declarative tools — the Dorc-niche detector): `provisioner "local-exec"`, `provisioner "remote-exec"`, `null_resource`, `terraform_data` + `triggers =`; k8s `kind: Job`, `initContainers:`, `lifecycle.postStart`, Helm `hooks`; `cloud-init`/`runcmd`/`user_data` inline scripts; Ansible `ansible.builtin.{command,shell,raw,script}` + `creates:`/`changed_when:`/`failed_when:` guards. Density of these = "this team is fighting their declarative tool" = prime Dorc rewrite candidates.

### Tier 3 — "provider corpuses" (1B): orchestration/role authorship (grade A; check license)
- **Ansible Galaxy** — collections + roles via the Galaxy API (`galaxy.ansible.com/api`), rank by download count / community size. `community.general` (already cloned) confirmed round-1 to be mostly thin idempotent CLI-wrappers — extend to the top-N collections.
- Non-Ansible equivalents: **Chef Supermarket** cookbooks, **Puppet Forge** modules, **Terraform Registry** modules (for the declarative contrast).

## Sampling, dedup, licensing
- **Sample** by (stars/activity tier) × (domain: k8s/bare-metal/cloud/network) × (fuzzy-edge-signature density). Cap per stratum; log what's dropped (no silent truncation).
- **Dedup** by content-hash (CoLiS & Bash-in-the-Wild both did SHA-based dedup; the `cluster-template` clones make this essential).
- **License filter for any redistributable derived corpus**: keep permissive (MIT/Apache/BSD/CC); record SPDX per item; the academic Tier-1 sets are already research-licensed. GitHub API use respects ToS; honor rate limits.

## Deriving validation criteria (the eventual payoff)
- **Deterministic**: (a) defect/smell labels from Gang-of-Eight (esp. `idempotency`) + GLITCH oracles as ground-truth a Dorc rewrite must not regress; (b) **parse→Dorc-IR→pretty-print round-trip** (the lossless invariant); (c) **differential**: run original task in a container fixture vs run the Dorc rewrite, assert identical end-state (the round-2 calibration harness, reused).
- **LLM-assisted** (the user's "LLM-y" option): prompt-translate an Ansible task/role into Dorc; judge behavioral equivalence + flag constructs that don't map cleanly (the corner-case miner). Use Galaxy's most-downloaded roles as the translation gauntlet.
- **Corner-case mining**: the constructs that resist clean translation ARE the design feedback — feed back into language (next round).

## Tooling spike
`tools/corpus-survey.sh` (written this session, **gently validated, not run in bulk**): uses `gh search`/`gh api --jq` (gh's built-in jq; system jq absent) to enumerate candidate repos by topic + fuzzy-edge code-signature, with conservative `--limit`s and an explicit `gh api rate_limit` preflight. The user runs it later to materialize a candidate manifest; nothing is cloned by the script itself.

## What the spike must MEASURE (the go/no-go — added round 5; this doc was acquisition-only)
Acquisition above answers *where to get data*. This section answers *what to compute from it* — the actual feasibility gate. **Raw skip-rate is a trap**: "90% skippable!" is worthless if those are cheap-to-just-run ops where the probe costs more than the work it saves. Classify each mutating operation on **two axes** (the depth-split; full rationale in `performance-architecture.md` §4b/§5 and the "Blow 3" anti-correlation):
- **apply-cost**: *cheap-idempotent* (just-run it; probe is pure overhead — `mkdir -p`, a no-op `apt` on a current pkg) vs *expensive/dangerous/slow* (`apt install` fresh, service restart, build, migration, large pull, remote call).
- **check-depth**: *shallow* (a cheap read-only guard **completely** captures need — pkg@version, file-exists, port-open) vs *deep* (hidden/hard-to-specify deps — `nginx -t`, `docker compose up`, daemon-mediated; the `/mnt/blah` catch).

**The three bands and their verdicts:**
1. **VALUE = expensive-apply ∧ shallow-check.** Dorc's whole win (skip expensive via cheap *sound* check). **Its size is the primary go/no-go.** Fat → build the engine; thin → thesis in trouble.
2. **JUST-RUN = cheap-apply (any check).** Probe is overhead; just run. This is what inflates a naive skip-rate — **subtract it out**, it is not Dorc's value-add.
3. **HARD = expensive-apply ∧ deep-check.** Unservable without M4 (lifted guards + oracle-library + the deferred tracing tool converting deep→shallow). Fat here → **the oracle-library is load-bearing, not optional.**

**The single most decision-relevant number: the anti-correlation ratio** — among *expensive* ops, the shallow:deep check ratio. If expensive ops are mostly deep, the value band is thin *by nature* and M4 is the only lever (Blow 3 confirmed). If many expensive ops are shallow-checkable, the thesis is healthy.

**Supporting (necessary, not sufficient):** the **un-analyzable rate** (leaves forced to must-probe/can't-skip by external/non-det reads, no oracle, dynamic constructs — note 71 / phase-1 Step −1; caps everything) and the **cheap-mutation fraction** (how much headline skip-rate is illusory JUST-RUN).

**Method honesty:** on a *static* corpus, both axes are *heuristic estimates* (command-name → cost-class; "guard exists ∧ is a known-shallow predicate" → shallow) — enough to *size* bands, not to measure precisely. State the heuristic + error bars; no silent precision-claims. A later container-fixture pass (the calibration harness) can ground-truth a sample by *running* it. **Shortcut available:** `shstats` (cloned, `Vendor/colis-anr/shstats`) already does corpus-wide shell AST statistics — extend it for the two-axis tally rather than writing from scratch.

**Net go/no-go:** (1) is the VALUE band fat enough that "skip expensive via cheap sound checks" is real? (2) is the un-analyzable rate low enough that a useful fraction is analyzable at all? (3) anti-correlation ratio → is the oracle-library a nicety or the actual project? Fat value band → build the engine. Thin value but fat hard band → the oracle-library/tracing strategy *is* the project, engine secondary. High un-analyzable rate → rework the thesis before any code.
