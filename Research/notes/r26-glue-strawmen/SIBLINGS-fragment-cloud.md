# SIBLINGS fragment — cloud lifecycle domain

> Round r26, writing-phase builder B1. Rows for the root `SIBLINGS.md` merge.
> Columns claimed here: **cloud-init** (channel/ecosystem posture) and
> **Terraform** (Big Boy posture), plus a short extra column for
> **Ignition/Talos** that the conductor may cut — flagged as such below.
>
> Row discipline honoured: ARCHITECTURE-tier only. Every row below is a
> fundamental decision or a lock-in/lock-out, not something either side could
> fix with mild implementation work. Sources are inline; all fetched 2026-07-28.
>
> Posture, per the charter's two-posture cut: Terraform is **not a choice**. No
> sane person should persistently choose Dorc over Terraform for anything
> Terraform models. Dorc exists for its residue, and every row below should read
> that way — including the ones where Dorc wins.

---

## Column: Terraform

### `row-declarative-graph-vs-ordered-sh`
**Terraform: Y. Dorc: N, by construction.**

Terraform builds a dependency graph from declared resources and is free to
reorder and parallelise within it. Dorc's book order is sacred: no intra-host
reordering, no intra-host parallelism, ever. That is not a missing optimiser —
it is the price of the plan being *your own script, in your own order*, and of
the off-ramp being "delete Dorc and run the file".

Consequence for users: if your problem decomposes into declared resources with
declared dependencies, you want the graph, and you should go get it.

### `row-state-is-mandatory-there-and-forbidden-here`
**Terraform: Y (and pays for it). Dorc: N (and pays for it).**

> "Terraform must store state about your workspace's managed infrastructure and
> configuration… for more information on why Terraform requires state and why
> Terraform cannot function without state, please see [state purpose]."
> — <https://developer.hashicorp.com/terraform/language/state>

Dorc keeps nothing between runs and re-measures from host reality. The trade is
symmetric and neither side is being clever:

- **Terraform gets** cross-run identity. It knows which real object each
  declaration made, so it can *destroy what is no longer declared*. Dorc
  structurally cannot express "remove what I stopped asking for" — with no
  memory there is no diff to take. Deletion in a Dorc book is an imperative line
  somebody wrote on purpose, and that is the whole story.
- **Dorc gets** no lock contention, no stranded state, no state-file secrets
  (Terraform's own docs warn: "risks… exposure of secrets stored in the state
  file"), and no drift between what the record says and what the box is. Two
  Dorc runs against one host cannot corrupt each other because there is nothing
  shared to corrupt.
- **Terraform pays** for its one-to-one mapping in operator work whenever
  reality and record disagree: "Terraform expects a one-to-one mapping between
  configured resource instances and remote objects… If you add or remove
  bindings in the state by other means… you'll then need to ensure for yourself
  that this one-to-one rule is followed."

This is the deepest row in the table and it is the reason the two tools are not
substitutes in either direction.

### `row-post-apply-is-conceded-in-writing`
**Terraform: N, deliberately and in their own words. Dorc: Y — this is the niche.**

> "Terraform is primarily designed for immutable infrastructure operations, so we
> strongly recommend using purpose-built solutions to perform post-apply
> operations."
> "You should exhaust all alternatives before using provisioners in your
> configurations. This is because **Terraform cannot predictably model
> provisioner behaviors** represented in the configuration."
> — <https://developer.hashicorp.com/terraform/language/resources/provisioners/syntax>

And the sanctioned alternatives named on their own post-apply page are: pass
data via cloud-init, bake the image, or **use configuration management tools**.
— <https://developer.hashicorp.com/terraform/language/post-apply-operations>

Terraform does not merely leave the interior of a machine unmanaged; it
identifies the gap, names the mechanisms that fill it, and routes users out. The
residue Dorc exists for is a *documented product boundary*, not an oversight.
The honest README line writes itself: if what you need is a machine, a network,
a DNS record or a load balancer, go use Terraform. Dorc starts after `apply`
returns.

### `row-modeled-or-unmanageable-vs-unmodeled-still-runs`
**Terraform: Y on breadth. Dorc: N on breadth, Y on floor.**

Terraform's coverage is provider-shaped: a resource type either has a provider
that models it or it is outside the tool. That buys enormous, tested, versioned
breadth. Dorc has no equivalent and never will — its oracle library is a
long-tail community bet with a bootstrap slice of a few dozen.

The architectural difference is what happens at the edge of coverage. In
Terraform, unmodeled means *unmanageable*. In Dorc, unmodeled means the command
**runs, exactly as written** — degraded to the behaviour of the shell script it
already was. That is what "gradual enhancement" means concretely, and it is why
Dorc's floor is useful with zero oracles and Terraform's floor is a
`null_resource`.

### `row-sees-the-api-versus-sees-the-host`
**Terraform: Y outside. Dorc: Y inside. Not the same question.**

`terraform plan` refreshes state against the provider API. It answers "does the
cloud's record of this machine match my declaration". It cannot answer "is nginx
running on it", and nothing in its architecture points that way. Dorc's entire
measurement surface is the opposite: it has no view of a cloud API except by
running a CLI as an ordinary command, and its facts are all read from inside a
host.

Practical consequence, and the reason `pivot-vps-standup.sh` looks the way it
does: a pivot book is Dorc doing a *bad* job of Terraform's half for the two or
three resources a small deployment actually needs, in exchange for those
resources living in the same file as the interior convergence. That trade is
right for one VPS and a DNS record. It is wrong the moment there are twelve
resources with dependencies, and the SIBLINGS table should say so rather than
letting a reader discover it at resource seven.

### `row-immutable-versus-pets`
**Terraform: Y. Dorc: N — and Dorc's users mostly should not be here either.**

"Primarily designed for immutable infrastructure operations" is Terraform's own
framing of itself. Immutable infrastructure is better. Where a machine can be
rebuilt instead of repaired, rebuild it, and Dorc has nothing to add. Dorc's
territory is the machines that genuinely cannot be rebuilt on a Tuesday, and
that territory should shrink over time, not grow.

### `row-identity-of-a-machine-you-just-created`
**Terraform: N. Dorc: N. Nobody: N.** *(a "neither" row — kept because the
SIBLINGS purpose includes what we cannot do well)*

> "Because the SSH connection type is most often used with newly-created remote
> resources, SSH host key validation is disabled by default."
> — Terraform provisioners doc, above.

Every reference implementation in this space does the same thing, for the same
structural reason: whoever verifies the host key did not create the machine.
Dorc's pivot book narrows the punt (`StrictHostKeyChecking=accept-new` plus an
explicit, attributed `ssh-keygen -R` only on the path where the controller
itself rebuilt the box) but does not close it. Nobody has closed it. It belongs
in the table as an honest N/N so no reader mistakes the narrowing for a fix.

---

## Column: cloud-init

cloud-init is not a competitor; it is the channel Dorc's day-zero artifact
travels down, and the ecosystem whose conventions a boot-book must honour. The
rows are still architecture-tier, but the posture is "good neighbour" rather
than "chooser's dilemma".

### `row-runs-where-nothing-else-can-reach`
**cloud-init: Y, structurally. Dorc: N, structurally.**

cloud-init executes at first boot with no inbound network, no credentials
distributed, and no controller in existence. That position cannot be occupied by
a push tool from outside — it is upstream of the existence of a channel to push
over. Dorc's only honest posture here is to be a *good payload*: ship a plain-sh
artifact down cloud-init's own `#cloud-boothook` / `x-shellscript` formats and
let cloud-init execute it.

Anyone reading the table for "should I stop using cloud-init" should read this
row and stop reading.

### `row-instance-id-keyed-versus-state-keyed-rationing`
**cloud-init: N. Dorc: Y. This is the whole reason the boot-book cell is open.**

cloud-init's frequency vocabulary is `PER_ONCE` / `PER_INSTANCE` / `PER_ALWAYS`,
and the gate is a **sem file named after the thing**, compared against a cached
instance-id — never against the world. `cloud-init-per instance <name> <cmd>`
writes `/var/lib/cloud/instance/sem/bootper.<name>.instance` and thereafter
believes it. So does chezmoi's `run_onchange_` content hash; so does Ansible's
`creates:`. Six ecosystems in the round's evidence mandate script idempotence by
documentation and assist none of it.

The failure this produces is documented at both ends. Upstream disclaims
re-running its own payload — "Making cloud-init run again may be destructive and
must never be done on a production system"
(<https://docs.cloud-init.io/en/latest/howto/rerun_cloud_init.html>) — while the
boothook handler catches, logs and swallows a failing payload rather than
failing the boot
(<https://github.com/canonical/cloud-init/blob/26.1/cloudinit/handlers/boot_hook.py>).

Give cloud-init its due here: it no longer hides that. The recoverable-error
work added `degraded done` / `degraded running` as first-class statuses and an
exit code of 2, so a half-applied instance now *says* it is half-applied
(<https://docs.cloud-init.io/en/latest/howto/status.html>). What has not changed
is that reporting is not repairing: the machine is in a known-partial state, the
tool disclaims re-running, and nothing will apply the rest. That gap — not
dishonesty about it — is the cell Dorc occupies.

Dorc's rationing is a measurement of the world. That difference is the entire
value of putting a Dorc artifact in this cell, and it is architectural on both
sides: cloud-init's sem model cannot become state-keyed without becoming a
configuration-management system, which it has explicitly declined to be.

### `row-yaml-modules-versus-sh`
**cloud-init: Y. Dorc: N.**

cloud-init's primary surface is `#cloud-config` YAML consumed by ~50 maintained,
distro-portable Python modules — users, ssh keys, disks, partitions, package
sources, ntp, and so on — tested across 27 datasources. Dorc has none of that
and offers instead one sh dialect. For anything a cloud-config module already
does, use the module: it is better, it is tested on your distro, and it runs
earlier than you can.

The Dorc-shaped half is what cloud-config *cannot* express: the ordered,
conditional, branch-on-what-you-find work that people currently write as a
`runcmd:` list or an `x-shellscript` part and then guard by hand.

### `row-log-errors-but-proceed`
**cloud-init: Y. Dorc: Y. Same instinct, arrived at independently.**

> "Since cloud-init provides access to cloud instances, the paradigm for
> handling errors was 'log errors, but proceed'. Exiting on failure conditions
> doesn't make sense when that may prevent one from accessing the system to
> debug it."
> — <https://docs.cloud-init.io/en/latest/explanation/return_codes.html>

That is a near-verbatim restatement of Dorc's own rule that fail-fast means fail
on human timescales, not stop in the frame that failed. Worth a row precisely
*because* it is agreement: it is evidence that the doctrine is forced by the
domain rather than chosen by taste, and it is the strongest available argument
that the two tools compose rather than collide.

cloud-init's eight-valued status — with `degraded done` distinct from `done` —
is the same instinct one level up, and is an incumbent shipping
wrong-but-not-broken as a first-class state. Dorc should not model host
readiness as a boolean anywhere, and cloud-init is the citation for why.

### `row-payload-is-world-readable`
**cloud-init: N, irreducibly. Dorc: N too, and must say so.**

On EC2, "User data is an instance attribute" and retrieving it "using instance
metadata" is a documented, unauthenticated read from inside the box
(<https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html>). Azure is
the careful counter-example and the exception that proves the rule: custom-data
is deliberately *not* surfaced through IMDS while Azure's separate user-data
feature is, and Azure advises against secrets in either — "We advise *not* to
store sensitive data in custom data"
(<https://learn.microsoft.com/en-us/azure/virtual-machines/custom-data>). The
safe posture is to treat the channel as world-readable everywhere and take
per-cloud hardening as a bonus. This is a property of the
delivery channel, not of any payload format, so a Dorc compiled artifact
inherits it exactly. The architectural consequence for Dorc is a standing law
rather than a feature: **the offline guard-artifact carries code and
probe-shaped reads, never credential material**, and secrets travel later, over
a channel that can authenticate.

---

## Extra column (conductor may cut): Ignition / Talos

Included because it is the sharpest architectural boundary in this domain and
the round decided to engage it rather than route around it.

### `row-provisioning-is-not-configuration-management`
**Ignition: Y (by axiom). Dorc: N (by axiom). Directly opposed, both coherent.**

> "Ignition is designed to be used as a provisioning tool, not as a
> configuration management tool. Ignition encourages immutable infrastructure,
> in which **machine modification requires that users discard the old node and
> re-provision the machine**."
> "Ignition configs do not allow users to provide arbitrary logic (including
> scripts for Ignition to run)."
> — <https://coreos.github.io/ignition/rationale/>

This is the principled negation of the boot-book thesis, and it deserves to be
stated in the table without hedging: if you can genuinely re-provision on every
change, Ignition's axiom is *better than ours* and you should take it. Dorc's
counter is not that the axiom is wrong; it is that the population of machines
where the axiom holds is smaller than its advocates believe, and Dorc is for the
remainder.

Note the escape hatch that exists anyway: Ignition maintainers' own sanctioned
pattern for "I need to run a script" is a written file plus a one-shot
`ConditionFirstBoot=true` systemd unit that "can run whatever commands you want"
(ignition issue #909). The axiom holds at the config-format layer and is
conceded at the unit layer.

### `row-fail-closed-versus-best-effort`
**Ignition: Y. cloud-init: N. Dorc: N, on purpose.**

> "Ignition produces the machine specified or no machine at all… If for any
> reason Ignition cannot deliver the exact machine that the config asked for,
> Ignition prevents the machine from booting successfully."
> — <https://coreos.github.io/ignition/rationale/>

Three tools, three positions, all coherent, on one axis. Ignition refuses to
produce a half-machine. cloud-init produces one and reports `done`. Dorc
produces one, tells you which parts are missing, and finishes the job on the
next run.

The row matters because it locates Dorc's honesty claim precisely: it is not
"we fail correctly" (Ignition does that better) and not "we always come up"
(cloud-init does that better). It is that a partial result is *nameable and
resumable* — the state Ignition refuses to enter and cloud-init refuses to
report is the one Dorc is built to sit in and grind down.

### `row-no-shell-at-all`
**Talos: Y (and Dorc is simply out). Dorc: N/A.**

> "We have no shell. We have no SSH. We have none of the GNU utilities, not even
> a rollup tool such as busybox."
> — <https://www.talos.dev/v1.11/learn-more/philosophy/>

Genuinely conceded, with no residue to claim. `kLANG` is welded to sh-as-the-
product, so a target with no shell is a target with no Dorc, permanently and by
design. This row exists so nobody spends a round looking for an angle.
