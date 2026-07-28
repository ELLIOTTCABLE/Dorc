# SIBLINGS — Dorc positioned against its neighbors, in a friendly voice

> Document tier: AI-written (r26 ops-glue-residue round, 2026-07-28), human-reviewed
> in place — the human deletes what they disagree with. Purpose: keep permanently
> in-scope **what we are not trying to own, what we cannot do well, and which users
> we intentionally push toward better choices.** This is not a competitive
> scorecard; most rows exist so that a future work-session doesn't accidentally
> spend months building a cell someone else has locked down, and so the README's
> honest "if you're doing X, just go use Y" advice has a maintained source.
>
> Row discipline: ARCHITECTURE-tier only — fundamental decisions and their
> lock-in/lock-out. Never NYI/someday rows that either side could close with mild
> implementation work (library breadth, polish, maturity are real but excluded;
> they belong in status docs, not here). The near-term-important rows are the
> **N-for-Dorc / Y-for-them** ones. To-be-added-to: new columns and rows accrete
> as new neighbors are examined.
>
> Full evidence: the round's graded ledger (`.claude/research/ops-glue-residue/`,
> 183 sources) and the builder fragments beside the strawman books
> (`Research/notes/r26-glue-strawmen/SIBLINGS-fragment-*.md`), which carry the
> long-form versions of many rows below.

## The three postures

1. **The Big Boys** (Kubernetes · Terraform · nix/NixOS · Home Manager) — **not a
   choice.** No sane person should persistently choose Dorc over these for
   anything they model; we say "go use them" out loud and constantly. Their one
   guaranteed cost is structural (an agent, a state store, a total description,
   an owned language) — and Dorc exists for the *residue* those costs define:
   the machines below them, the glue before them, the raw sh trapped inside them.
2. **The siblings** (pyinfra · cdist · Ansible) — **a legitimate choice.** Same
   floor (ssh + POSIX sh on the target — the agentless floor is *shared* here and
   is never a differentiator among siblings), overlapping niche, different
   fundamental bets. A user may reasonably prefer any of us; the rows say what
   each bet buys and costs.
3. **The channels** (cloud-init · the OS-installer hooks · Ignition) — **neither.**
   Delivery substrates every tool in this table (ourselves included) must arrive
   through. "Dorc vs cloud-init" is a category error; the rows describe what each
   channel does and doesn't do for whoever lands in it. *(Posture minted by a
   round builder and conductor-adopted; human may re-cut.)*

---

## Big Boy: Kubernetes

Choosing between Kubernetes and Dorc is not a decision anybody should be making.
The residue around it is unusually well-defined because Kubernetes draws its own
boundaries explicitly: it does not manufacture nodes (the kubelet is a
precondition of the API), and it governs workload *health*, never the *content*
of the shell inside init containers, entrypoints, and hooks.

| Capability / decision | k8s | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| Continuous reconciliation with nobody present | **Y** | **N** | Controllers run forever. Dorc converges exactly when you run it (`kAGENTLESS` welded). Neither side can move without becoming the other product. |
| An agent on every managed node | **Y** | **N** | The *cost* of the row above, not a defect: declarative reconciliation at scale requires a resident process. |
| Desired state as a durable, watchable API | **Y** | **N** | Dorc holds nothing between runs and re-measures reality. There is no `kubectl get` for Dorc and never will be. |
| Scheduling, replicas, rollouts, rollback, RBAC, cluster networking | **Y** | **N** | Different layer entirely; a book has no notion of a workload. Rollback would require the state store Dorc refuses. |
| Immutable deployment unit | **Y** (the image) | **N** | Opposite bets; the immutable one is usually right, which is why Dorc's bet is scoped to where you cannot take it. |
| Extension model | **Y** (CRDs + controllers, a programmable API) | **N** (publishing a shell file) | Enormously cheaper to author, enormously less powerful, not convertible. |
| **Governing the content of a script** | **N**, by design | **Y** | The sharpest row. K8s itself: init code *"should be idempotent"*, while `readinessProbe` on an init container is *"prohibited… enforced during validation."* The hole is deliberate, permanent, and exactly Dorc-shaped. A seam we fill — never a gap they failed to close. |
| Bringing a node into existence | **N**, structurally | **Y** | Everything before `kubeadm join` returns is somebody's shell script. |
| Operating on a machine in no cluster | **N** | **Y** | The bastion, the edge boxes, the GPU pet, the laptop. |
| Zero infrastructure of its own | **N** | **Y** | Why Dorc can sit in the residue at all: nothing to contest, lock, or strand beside your control plane. |
| Trivial off-ramp | **N** | **Y** | **K8s' lock-in is what buys continuous reconciliation** — a fair trade and usually the right one. Dorc offers the off-ramp *only because* it refuses that product. |
| Waiting / convergence predicates | **Y**, and better | delegates | `kubectl wait --for=jsonpath --timeout=0s` is a shipped, general, read-only convergence predicate over every resource type — the clearest case of a Big Boy having already built a thing we must never build. Delegation-oracle target. |

Go-use-Y lines: containerized workloads on >1 machine → Kubernetes, not close ·
packaging for clusters → Helm · keeping declared state matching git → a GitOps
reconciler (the outer reconcile loop is ceded) · cattle nodes → your cloud's node
pools. Making a *pet* node reproducible, or fixing one at 3am → the residue, ours.

**Helm** (escape-hatch posture): templating **Y**/**N**-by-design (heredocs are
our templating; a TRUE-DON'T-BUILD row on both sides) · release state + rollback
**Y**/**N** (the state-store trade again) · its imperative escape hatch is
argv-on-an-image, so the dorc-inside seat there is *thin, not dead* — one level
down, inside the hook Job's image (same seat as the init-container book).

## Big Boy: Terraform

| Capability / decision | Terraform | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| Declarative dependency graph, free reordering/parallelism | **Y** | **N**, by construction | Book order is sacred — the price of the plan being *your own script in your own order* and of the delete-Dorc-and-run-the-file off-ramp. If your problem decomposes into declared resources, go get the graph. |
| Cross-run state (mandatory there, forbidden here) | **Y**, and pays | **N**, and pays | The deepest row. Terraform gets cross-run identity — it can *destroy what is no longer declared*; Dorc structurally cannot express "remove what I stopped asking for." Dorc gets no lock contention, no stranded state, no state-file secrets, no record/reality drift. Symmetric, and why the two aren't substitutes in either direction. |
| Post-apply interior of the machine | **N** — conceded in writing | **Y** — the niche | *"Terraform cannot predictably model provisioner behaviors"*; their own post-apply page routes users to cloud-init, image baking, and configuration management. The residue is a documented product boundary. Dorc starts after `apply` returns. |
| Coverage edge behavior | modeled-or-unmanageable | unmodeled-still-runs | Provider breadth vs a floor that is useful with zero oracles. What "gradual enhancement" means concretely. |
| Sees the API vs sees the host | **Y** outside | **Y** inside | `terraform plan` answers "does the cloud's record match my declaration," never "is nginx running." A pivot book doing Terraform's half is right for one VPS + a DNS record and wrong by resource seven — this table says so before a user finds out. |
| Identity of a machine you just created | **N** | **N** | Honest N/N: ssh host-key validation "disabled by default" (their words); Dorc's pivot book narrows the punt, nobody has closed it. |

## Big Boy: nix / NixOS (and Home Manager)

| Capability | nix | Dorc | Why it is architecture |
|---|---|---|---|
| Whole-system state with a single comparable identity | **Y** — an input-addressed store path | **N** | We have per-cell measurements and no global identity because we never require a total description (`kDEPS-accept-partial`, welded). |
| Atomic activation + rollback to generations | **Y** | **N** | We mutate in place; there is no artifact we built the machine from. |
| Cross-machine identical outcome | **Y**, given a lock | **N** | Best-effort against drifted, partly-unknown machines *is the thesis*. |
| Manages a machine you did not build and don't own | **N** — all-or-nothing | **Y** | Ours is the whole reason to exist; not fixable on nix's side without stopping being nix. |
| Incremental adoption / imperative fix-this-now | **N** by axiom | **Y** | DESIGN.md's opening argument, with nix named. |
| Off-ramp cost | **High** — leaving means rewriting | **~Zero** — strip to sh | Theirs is inherent to owning a language; ours is welded (`kLANG`). |
| Secrets in the managed description | **N** — the store is world-readable; their manual says read from the filesystem at runtime | **N** | A shared N worth keeping: secrets are the largest member of nix's residue, and residue is our seat. Neither side should claim it. |
| Convergence verb worth delegating to | **Y** — the best of any incumbent (profile store-path comparison) | — | The row where they win and we *consume* the win. NB their dry-run is the opposite: `dry-activate` self-documents as incomplete — a tool can have a sound convergence check and an unsound dry-run. |
| Ships its own push-over-ssh deployment | **Y** (`nixos-rebuild --target-host`) | **Y** | Correcting an easy assumption: nix is a genuine sibling *deployer* too. |

**Home Manager**: declarative dotfiles ownership cleanly ceded (we would be a
worse chezmoi). Its activation escape hatch mandates idempotence ("running twice
or more times produces the same result") and assists none of it — the seat,
first-party-sourced. Activation blocks are concatenated into one generated bash
script (no per-block file) under a deliberately empty PATH — which is *why* the
only correct Dorc spelling there is a store-path-named spliced invocation, their
hygiene rule and our spelling being the same fact.

## Sibling: pyinfra — the closest living architecture *(conductor column, turns B/D evidence)*

Radman's 2021 taxonomy drew one "Two-Stage Model" box around gather-facts →
generate → execute with no remote dependencies — and it is pyinfra's box and
Dorc's. Choosing between us is real, and mostly one decision wearing different
clothes: **what language do you author in, and what does the artifact cost you?**

| Capability / decision | pyinfra | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| Authoring language | **Python** — real loops, libraries, types, an ecosystem | **sh** — the target's own language | Their bet buys expressive authoring; its cost is structural: deploy code is *"simplified Python"* with parse-time gotchas their maintainer names as a seven-year complexity source. Our bet buys the next three rows; its cost is that you are writing POSIX sh. |
| A transferable plan artifact | **N** — structurally; their author designed `plan \| apply -` and pickling blocked it, five years unshipped | **Y** | The payload language decides this, not effort. |
| The plan is your own reviewed bytes | **N** — since v3, commands are generated at runtime; `--dry` cannot print them | **Y** — byte-honest, the whole render | Same root. |
| Off-ramp | **N** — deploys are Python against the pyinfra runtime | **Y** — strip to sh | Same root, third face. |
| Staleness of facts vs execution | admitted architectural — facts run at start; deploy-code branches see pre-deploy state; remedy is opt-in `_if=` | same physics, opposite posture — stale-and-*shown* plan + consent moment; guards as *default* degradation | Their v3 arc is the proof the gap is physics; the differentiator is honesty machinery, not avoidance. |
| Skip explanation | prose at `-v` (`host.noop("apt is already up to date")`) | an interrogable chain with typed provenance and a why-verb | They print the shine; nobody upstream owns being *wrong* about it — no named claimant, no receipt. |
| Cross-host lock-step ordering | **Y** — defended by the maintainer to the end; what forced their hybrid architecture | **N** — per-host books, embarrassingly parallel | Both coherent. Lock-step must never be re-imported into Dorc as an engine promise. |
| Idempotence vouch vocabulary | exists (`is_idempotent=`, notices on 57 ops) — wired only to docs generation | the licensing core | "The vocabulary exists upstream; the licensing doesn't." |
| Channel rc | trusted (`exit_code in success_codes`) | never trusted; in-band sentinel | Different floors; theirs assumes rc-faithful connectors, ours survives channels that have no rc at all. |
| Operations/facts library | **Y** — 182/143, maintained, *in Python* | **N** today; oracles strip to reusable sh | Breadth itself is maturity (excluded); the *language* of the library is architecture — their operations are unusable outside pyinfra, a stripped oracle runs on any POSIX box. |

Go-use-Y: your team writes Python and wants a mature operation library today,
with no need for a reviewable/transferable artifact → **pyinfra**, comfortably.

## Sibling: cdist — the same aesthetic, the opposite architecture *(conductor column)*

| Capability / decision | cdist | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| "Configuration is sh" | manifests *look like* sh but do not run — `__package` resolves through PATH-planted symlinks to a Python emulator | books *are* sh, byte-honest | The defining difference: zero off-ramp vs the off-ramp as a weld. |
| Remote interrogation model | explorers: per-object remote scripts, O(objects) connections — their own stated regret ("as cdist makes many connections…"; ControlMaster bolted on; `MaxSessions` ceiling) | one artifact per phase, one connection | The evidenced answer to the closest prior art's named pain. |
| Types library | 153 types whose gencode *emits* sh | oracles authored *in* sh | Closer kin than pyinfra's ops; the emission layer is still theirs alone. |
| Day-zero | PreOS: manufacture the ssh precondition (debootstrap + key + PXE), then normal cdist | compile convergence into the payload itself | Two coherent answers; theirs re-affirms that the payload cell was empty of convergence machinery. |

Honest status note (not a row): upstream is unreachable, the Debian package is
orphaned, and the public mirror is three minor versions stale — recommending
cdist today is hard on non-architectural grounds. It remains the record's best
proof that sh-native config management has an audience.

## Sibling: Ansible *(conductor column, corpus + turns B/D)*

| Capability / decision | Ansible | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| Module model | **Y** — check-then-converge fused *inside* each module; a good module already does internally what a Dorc guard does | orthogonal — Dorc adds nothing to a good module | The value claim lives only in their escape hatches (`shell:`/`script:` tasks, check-mode-blind, `changed_when` hand-annotation) — per USER_STORY's transition rungs. |
| Target floor | Python required; `raw` documented as the no-Python emergency | the emergency floor *is* the normal mode | The framing gift, verbatim from their docs. |
| Inventory, group vars, templating | **Y** — owned, mature, layered | **N** — TRUE-DON'T-BUILD; branch-on-probed-facts is the inventory, heredocs the templating | Different in kind, not lesser versions. |
| Connection-plugin capability catalog | **Y** — {exec, put_file, fetch_file}; where a channel lacks one, synthesize out-of-band (the aws_ssm S3 bucket) or relocate to the controller | capability-probed per feature; file-transfer synthesized from the pipe when absent | Their catalog is the best empirical map of the channel lattice; the S3 exhibit is why Dorc designs to a minimal floor. |
| Two-host attribution | `delegate_facts` opt-in, grown the moment one play addressed two hosts | scope-typing seam, deliberately not yet built | Incumbent precedent for the exact trigger Dorc's law names. |
| Check-mode honesty for shell content | **N** — shell tasks skipped or judged by hand annotation | **Y** — measured | The `changed_when: false` standing lie vs a probe. |

Go-use-Y: a large heterogeneous estate where a mature module exists for
everything you touch, and a team fluent in its YAML → **Ansible**. Dorc's rung-1
offer (a `dorc-run` shebang inside `script:` tasks) composes rather than competes.

## Channels

**cloud-init** — runs where nothing else can reach (first boot, no inbound
network, no controller in existence): the only honest Dorc posture is to be a
*good payload* down its own formats. Its rationing is sem-files against a cached
instance-id, never the world — upstream disclaims re-running its payload
("may be destructive… never on a production system") while a failing boothook is
caught, logged, swallowed, and reported `done`: a half-applied instance that
claims success is the designed-for normal case, and that empty cell is the
boot-book's entire reason to exist. Its `#cloud-config` YAML modules are better
than sh for everything they cover — use them; the Dorc-shaped half is the
ordered, conditional `runcmd`/`x-shellscript` work people guard by hand. Its
"log errors, but proceed" paradigm and eight-valued status (`degraded done` ≠
`done`) independently restate Dorc's own fail-fast-on-human-timescales doctrine
— evidence the doctrine is forced by the domain, and that the tools compose.
User-data is IMDS-world-readable: a compiled artifact carries code and
probe-shaped reads, **never credential material**.

**The OS-installer hooks** (autoinstall · preseed · kickstart) — one-shot by
shape: the machine leaves the channel forever, nothing is mandated or assisted
in the hook, and the same-file-re-runs-on-day-N property is Dorc's contribution,
not the channel's. The hooks run in a chroot with no init manager and borrowed
DNS — the artifact survives both regimes by branching on observable host facts,
never on a delivery flag. This is also the one channel found where the *channel*
trusting our exit code is correct (non-zero aborts the install) — never-trust-
channel-rc does not mean never-produce-a-meaningful-rc.

**Ignition (and Talos)** — the principled negation, stated without hedging:
"machine modification requires that users discard the old node and re-provision;"
"Ignition produces the machine specified or no machine at all." Where you can
genuinely re-provision on every change, **their axiom is better than ours — take
it.** Dorc's counter is only that the population where the axiom holds is
smaller than its advocates believe. Three coherent positions on one axis:
Ignition refuses a half-machine; cloud-init produces one and reports `done`;
Dorc produces one, names which parts are missing, and finishes the job next run
— *nameable and resumable* is our cell, "fails correctly" is theirs. Talos (no
shell at all) is genuinely conceded, permanently, by `kLANG` — this row exists
so nobody spends a round looking for an angle.

---

## Standing wording fences (survive all future edits)

- Never claim the agentless floor against the siblings (shared floor); against
  the Big Boys the correct wording is that declarative reconciliation at scale
  *requires* an agent — their necessary cost, never their mistake.
- Never frame a Big Boy's lock-in as a flaw; pair every Dorc strength with the
  capability it is the price of.
- The maturity/community "use Y" line is product-tier framing reserved to the
  human's voice; this document carries per-feature advice only.
- The tri-state "nothing to do" exit convention: three incompatible wild
  encodings exist (systemd `ExecCondition` · GCP OS-policy · rinstall); no
  incumbent ships a portable one. The oracle rc contract fills a real,
  independently-groped-for hole — state it as such, without triumph.
