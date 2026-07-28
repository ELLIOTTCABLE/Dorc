# SIBLINGS -- where Dorc sits among its neighbors

> AI-written (r26 glue round), human-reviewed in place. This is a friendly map,
> not a scorecard: a Y under Dorc is a choice you may or may not want, and a Y
> under anyone else is a strength worth having -- usually bought with a cost
> that is also in the table. Its standing job is to keep three things visible
> while we work: what we are not trying to own, what we cannot do well, and
> which users we should push toward a better tool for them.
>
> Rows are architecture -- fundamental decisions and their lock-in/lock-out --
> never gaps either side could close with ordinary implementation work. Rows
> are sorted by what professional ops work actually demands, which is why the
> table opens with things Dorc does not do; Dorc's strengths come after, and
> the adoption/bootstrap rows sit at the end. Library size and maturity are
> deliberately absent (Dorc's answer today is "young and nearly empty"; that
> sentence belongs to the README's own voice, not a table).
>
> Legend: Y yes, as a fundamental capability. N no, as an architectural
> decision. ~ partly, see that tool's notes. N/A[^na] outside that tool's
> layer. Long-form evidence: `Research/notes/r26-glue-strawmen/
> SIBLINGS-fragment-*.md` and the round ledger under
> `.claude/research/ops-glue-residue/`.

| Capability | Dorc | Ansible | pyinfra | cdist | Terraform | Kubernetes | nix/NixOS | cloud-init |
| --- | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| You declare the end state; the tool owns the steps | N[^d5] | ~ | ~ | ~ | Y | Y | Y | ~ |
| Converges continuously with nobody present | N | N | N | N | N | Y | N | N |
| Remembers what it built; delete by un-declaring | N | N | N | N | Y | Y | Y | ~ |
| Fleet: inventory, groups, per-host data | N | Y | Y | ~ | Y | Y | ~ | N/A |
| Secrets management story | NYI[^d2] | Y | N | N | ~ | Y | N | N |
| Privilege escalation built in | NYI[^d2] | Y | Y | ~ | N/A | N/A | ~ | N/A |
| Whole-system rollback | N | N | N | N | ~ | ~ | Y | N |
| Creates infrastructure (VMs, DNS, networks) | ~ | ~ | ~ | N | Y | ~ | N | N |
| Templating / config-file generation | N[^d1] | Y | Y | ~ | Y | ~ | Y | ~ |
| Check-then-converge inside its own units | N/A | Y | Y | Y | Y | Y | Y | N |
| Preview before mutating (plan / dry-run / diff) | Y | ~ | ~ | ~ | Y | Y | ~ | N |
| Re-measures the live system every run | Y | Y | Y | Y | ~ | Y | N | N |
| Convergence machinery for raw shell content | Y | N | N | N | N | N | N | N |
| The reviewed text is byte-for-byte what executes | Y | N | N | N | N | ~[^k1] | Y | N |
| Skips explained with queryable provenance | Y | N | N[^p1] | N | N/A | N/A | N/A | N |
| Partial work chosen by measuring the machine | Y | N | N | ~ | Y | Y | ~ | N |
| Plan artifact runs without the tool installed | Y | N | N[^p2] | N | N | N | N | N/A |
| Off-ramp: stop using it, keep working artifacts | Y | N | N | N | N | N | N | ~ |
| No resident software on the managed machine | Y | ~[^a1] | Y | Y | Y | N | N | N |
| Works before ssh exists (the first-boot seat) | ~[^d3] | N | N | ~ | N | N | N | Y |
| Existing scripts run unchanged (the adoption floor) | Y[^d4] | ~ | ~ | N | N/A | N/A | N | Y |
| Full value without learning an authoring layer | N[^d6] | N | N | N | N | N | N | ~ |
| Authored in the target's own language (plain sh) | Y | N | N | ~[^cd1] | N | N | N | ~ |

[^na]: N/A: outside that tool's layer or model; the cell neither wins nor concedes.
[^d5]: The defining no. Books are ordered, imperative sh, on purpose -- and most of the table's opening block follows from that choice.
[^d2]: Not yet; these are musts before I call Dorc ship-able, but there's no completed, coherent, locked-in plan.
[^d1]: By design, not omission: heredocs are the templating, and branching on probed facts is the inventory.
[^d3]: As a compiled, guards-only payload shipped down the channel's own formats -- safety travels; elision waits for day N.
[^d4]: Unchanged also means undescribed: an unannotated script gains safety floor and a plan surface, nothing more. Value accrues only as guards and oracles describe it.
[^d6]: The ceiling costs real learning here too: oracle authorship in a typed sh dialect, plus its invariants. Only Dorc's floor is free, not its full value.
[^a1]: Needs Python on the target; its own docs offer `raw` as the no-Python emergency floor.
[^cd1]: Manifests look like sh but execute through a Python emulation layer -- the aesthetic without the off-ramp.
[^k1]: True for its own objects; the shell embedded in init containers and hooks is reviewed as an opaque string.
[^p1]: Skips are announced in prose at `-v`; there is no queryable record of why.
[^p2]: Structural, not backlog: deploy code is live Python, and its own author's plan-file design has been blocked on serializing it since 2021.

---

## The three postures

The Big Boys -- Terraform, Kubernetes, nix -- are not a choice. Nobody should
persistently pick Dorc over them for anything they model, and we will keep
saying so. Their strengths each carry one structural cost (a state file, an
agent, a total description), and Dorc lives in the residue those costs define.
The siblings -- Ansible, pyinfra, cdist -- share our floor (ssh + sh) and our
niche; choosing among us is a legitimate matter of taste and situation. The
channels -- cloud-init and the OS installers -- are neither: they are how
everyone's payload arrives, ours included.

## Ansible

- The module model is genuinely good: a well-written module already does,
  internally, what a Dorc guard does. Dorc adds nothing to it -- the overlap is
  only the escape hatches (`shell:`/`script:` tasks), which check-mode can't see
  and `changed_when` annotations describe on the honor system.
- If a mature module exists for everything you touch and your team lives in its
  YAML happily: use Ansible. Dorc's compatible offer is a `dorc-run` shebang
  inside the `script:` files you already have -- composition, not competition.
- Its connection-plugin catalog is the best empirical map of what channels can
  and can't do (where a channel lacks file transfer, Ansible synthesizes it --
  the SSM plugin requires an S3 bucket just to move files). We design to the
  floor that map reveals.
- One wart worth knowing when embedding: ssh `script:` tasks force a TTY, which
  merges stderr into stdout before any payload sees it.

## pyinfra

- The closest living architecture to Dorc's -- gather facts, decide, execute
  over plain ssh -- with the other authoring bet: real Python, with libraries
  and types, in exchange for the artifact never leaving the runtime. If your
  team writes Python and wants a maintained operation library today, pyinfra is
  an excellent tool and the right call.
- Its history is generous with lessons: facts are gathered before execution, so
  code that branches on them sees pre-deploy state -- a gap its maintainer
  worked seven years, rebuilt the ordering engine three times over, and finally
  declared architectural. Dorc has the same physics and the opposite posture:
  the staleness is shown (the plan you consent to), and the default fallback is
  a runtime guard rather than an opt-in flag.
- It also chose fleet-wide lock-step ordering (every host finishes step 1
  before any host starts step 2) -- powerful for control-node/worker dances,
  and the very thing that forces its two-phase machinery. Dorc promises no
  cross-host ordering at all; a dependency between hosts is a line you write.

## cdist

- The proof that sh-native configuration management has an audience, and the
  closest kin to our oracle idea (its "types" emit sh). Its remote-interrogation
  model pays one connection per explorer per object -- its own manual's stated
  regret, and the pain one-artifact-per-phase exists to avoid.
- Its PreOS feature answers first-boot by manufacturing the ssh precondition (a
  bootable image with your key baked in); we answer it by compiling convergence
  into the payload. Both coherent.
- Status, said plainly and kindly: upstream is unreachable, the Debian package
  is orphaned, and the public mirror lags the last release. Hard to recommend
  today for non-architectural reasons.

## Terraform

- What you should use for anything that is a resource: machines, networks, DNS,
  load balancers. Dorc's pivot books can drive a cloud CLI for a box or two,
  and are the wrong tool by the seventh dependent resource.
- The state file is the deep divide, and it is a fair trade in both directions:
  state buys Terraform the ability to delete what you stop declaring -- Dorc,
  keeping nothing, structurally cannot; deletion in a book is a line someone
  wrote. Statelessness buys Dorc no locks, no stranded state, no secrets at
  rest in a record, no record/reality drift.
- The interior of the machine is a boundary they drew on purpose: provisioners
  are documented as a last resort ("Terraform cannot predictably model
  provisioner behaviors"), and their post-apply guidance names configuration
  management as the answer. Dorc starts where `apply` returns.

## Kubernetes (and Helm)

- Not a choice, and the residue around it is unusually crisp because Kubernetes
  states its own boundaries: it does not manufacture nodes (everything before
  `kubeadm join` returns is somebody's shell script), and it governs workload
  health, never the content of the shell inside init containers and hooks -- it
  mandates that content be idempotent and, for init containers, rejects
  readiness probes at validation. That deliberate hole is exactly Dorc-shaped;
  we fill a seam they drew, we do not close a gap they missed.
- Its lock-in is the price of its best feature: continuous reconciliation
  requires the resident agent and the API estate. Dorc can offer a trivial
  off-ramp only because it refuses to offer that product.
- `kubectl wait --for=jsonpath --timeout=0s` is a shipped, general, read-only
  convergence predicate over every resource type -- a thing we should only ever
  delegate to, never rebuild.
- Helm: templating and release-rollback for cluster apps, cleanly theirs. Its
  hooks hand you argv-on-an-image rather than raw sh, so the Dorc seat there is
  one level down, inside the hook's image.

## nix / NixOS (and Home Manager)

- The strongest version of the describe-the-world bet: one input-addressed
  identity for a whole system, atomic activation, generations, rollback. If you
  can pay its buy-in for a machine, pay it. "Go use nix" remains standing
  advice.
- It is also a genuine sibling deployer (`nixos-rebuild --target-host` pushes
  over ssh) -- and its convergence question has the cleanest delegation answer
  of any tool here: compare the running system's store path to the built one.
  Our oracle consumes their soundness. (Their `dry-activate` self-documents as
  incomplete; a tool can have a sound convergence check and an unsound dry-run
  -- different questions.)
- What it structurally declines: machines it doesn't wholly own, incremental
  adoption, imperative fix-this-now, and secrets in the description (the store
  is world-readable; their manual says read secrets from the filesystem at
  runtime). That residue -- the bootstrap glue before nix exists on a box, and
  the imperative edges around it -- is our seat beside it.
- Home Manager: dotfiles ownership cleanly ceded. Its activation escape hatch
  mandates idempotence and assists none of it; blocks are concatenated into one
  generated script under a deliberately empty PATH -- which is why the only
  correct Dorc splice there is an invocation named by store path.

## The channels: cloud-init, the installers, Ignition

- cloud-init runs where no push tool can exist yet: first boot, no inbound
  network, no controller. Dorc's only posture there is to be a good payload
  down its own formats. Its per-instance rationing remembers that a payload
  ran, never whether it worked -- and upstream explicitly disclaims re-running
  -- so the "same file, day zero and day N" convergence cell is empty and open.
  Its `#cloud-config` modules are better than sh for everything they cover: use
  them, and keep Dorc for the ordered, conditional remainder. Its "log errors,
  but proceed" doctrine and eight-valued status independently mirror our own
  fail-on-human-timescales rule -- good evidence the two compose. One law it
  teaches: user-data is world-readable from inside the box, so payloads carry
  code, never secrets.
- The OS installers (autoinstall, preseed, kickstart) are one-shot by shape --
  the machine leaves the channel forever, and nothing in the hook is assisted.
  A compiled book dropped there gains a day-N future the channel itself can't
  offer. Notably it is also the one channel that should consume our exit code
  (non-zero aborts the install) -- producing a meaningful rc and refusing to
  trust a channel's rc are different things.
- Ignition states the principled opposite of our bet: modification means
  re-provision; the config produces "the machine specified or no machine at
  all." Where you can truly re-provision on every change, take their axiom --
  it is better than ours. Three coherent positions on one axis: Ignition
  refuses a half-machine, cloud-init produces one and reports done, Dorc
  produces one, names what is missing, and finishes next run. Talos (no shell
  at all) is fully, permanently conceded.

---

*Editing notes for future revisions:* never claim the agentless floor against
the siblings (it is shared); against the Big Boys the wording is that
declarative reconciliation at scale requires an agent -- their necessary cost,
never their mistake. Pair every Dorc strength with the capability it is the
price of. Per-feature "use Y" advice lives here; the maturity version of that
sentence belongs to the human's voice.
