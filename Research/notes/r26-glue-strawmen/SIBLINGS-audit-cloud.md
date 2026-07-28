# SIBLINGS audit — cloud lane (Terraform, cloud-init; + Ignition/Talos prose)

> Round r26, writing-phase builder B1, follow-up pass. Light tier.
> Lane: the **Terraform** and **cloud-init** columns of root `SIBLINGS.md`, plus
> my prose sections ("Terraform"; the cloud-init and Ignition/Talos bullets of
> "The channels"). I did not see or touch `SIBLINGS.md` — the conductor
> integrates; everything below is {row, column, footnote, URL} for splicing.
>
> Every URL below was fetched once this pass and reported its status. **✓** = I
> fetched it myself; **⟳** = a clamped Sonnet verifier fetched it. Nothing is
> asserted from memory; where a claim could not be established from a
> first-party page, it says so and is not dressed as a quote.

---

## 0. Two corrections I owe, before the audit

Both are cases where my own earlier prose was over-broad. Fixing them is the
main reason this pass was worth running.

### 0a. Azure custom-data is deliberately NOT in IMDS

My fragment and the boothook book said the payload is "readable back through
IMDS" **on EC2 and Azure**. The EC2 half is right; the Azure half is backwards.

> "Is custom data made available in IMDS? **No.** Custom data isn't surfaced
> through the Azure Instance Metadata Service (IMDS). If you need to retrieve a
> payload from inside the VM after provisioning … use the user data feature
> instead."
> — ✓ <https://learn.microsoft.com/en-us/azure/virtual-machines/custom-data>

Azure has *two* payload fields; cloud-init consumes custom-data, and custom-data
is the one that is **not** in IMDS. Azure's advice against secrets covers both
("We advise *not* to store sensitive data in custom data", same page). The safe
posture is unchanged — treat the channel as world-readable, take per-cloud
hardening as a bonus — but the reason must be stated per-cloud.

EC2's half is solid and lives on one page that also carries the size cap:

> "User data is an instance attribute."
> "User data is limited to 16 KB, in raw form, before it is base64-encoded."
> — ✓ <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html>

### 0b. "cloud-init still reports `done`" is stale — it reports `degraded done` now

This is the more important one, because it understated an incumbent. I wrote
that a failing boothook is swallowed "and cloud-init still reports `done`". The
swallow is still true and verified in source at tag 26.1. The `done` is not:
cloud-init's return-codes page frames that behaviour in the **past** tense, as
the pain point the recoverable-error work was built to fix, and the status
vocabulary now carries a first-class degraded pair.

> "Cloud-init would report a status of 'done' in the following cases: a user's
> configuration was invalid; if the operating system or cloud environment
> experienced some error that prevented cloud-init from configuring the
> instance; if cloud-init internally experienced an error — all of these
> **previously** reported a status of 'done'."
> — ⟳ <https://docs.cloud-init.io/en/latest/explanation/return_codes.html>

> statuses: "not started" / "running" / "done" / "error - done" /
> "error - running" / "degraded done" / "degraded running" / "disabled"
> — ⟳ <https://docs.cloud-init.io/en/latest/howto/status.html>
> exit codes: 0 clean, 1 crash, **2 finished with recoverable errors**
> — ⟳ <https://docs.cloud-init.io/en/latest/reference/cli.html>

The honest claim, which is narrower and survives: cloud-init now *reports*
partial application accurately; it still does not *repair* it, and it disclaims
re-running its own payload. Reporting is not repairing — that gap, not any
dishonesty about it, is the cell Dorc occupies. Corrected in the boothook book,
its note, and the fragment.

Also filed: the "log errors, but proceed" quote is **not** on
`explanation/failure_states.html` (which covers critical-vs-recoverable and the
0/1/2 exit codes). It is on `explanation/return_codes.html`. Anyone citing
`failure_states` for that sentence is citing the wrong page.

---

## 1. Citations

One-line footnotes, ready to paste. Obvious cells get nothing, per the brief.

### Terraform column

| # | Row | Cell | Proposed footnote | URL |
|---|---|---|---|---|
| T1 | Re-measures the live system every run | ~ | plan reads "the current state of any already-existing remote objects" — the cloud's record of the object, never inside a machine | ⟳ <https://developer.hashicorp.com/terraform/cli/commands/plan> |
| T2 | Whole-system rollback | ~ | the CLI has no rollback verb at all; HCP Terraform's UI rolls back the *state version*, which reverts tracking data, not infrastructure | ⟳ <https://developer.hashicorp.com/terraform/cloud-docs/workspaces/state> |
| T3 | Secrets management story | ~ | `sensitive`, `ephemeral` and write-only arguments exist, but a secret placed plainly in configuration is stored in state in plaintext | ✓ <https://developer.hashicorp.com/terraform/language/state/sensitive-data> |
| T4 | Converges continuously with nobody present | N | HCP Terraform's health assessments *detect* drift on a schedule, on paid tiers; they never apply | ✓ <https://developer.hashicorp.com/terraform/cloud-docs/workspaces/health> |
| T5 | No resident software on the managed machine | Y | true of managed resources; provisioners still need a pre-existing ssh or WinRM server, and file uploads need `scp` installed on the target | ✓ <https://developer.hashicorp.com/terraform/language/resources/provisioners/syntax> |
| T6 | Check-then-converge inside its own units | Y | true of resources; provisioners run at create time and have no convergence at all — which is precisely where Dorc's territory begins | ✓ same page as T5 |
| T7 | The reviewed text is byte-for-byte what executes | N | the plan renders a diff of resource attributes; a provisioner's command string is opaque to it — "Terraform cannot predictably model provisioner behaviors" | ✓ same page as T5 |
| T8 | Off-ramp: stop using it, keep working artifacts | N | "State is a necessary requirement for Terraform to function"; the config-to-real-world mapping is the artifact, and it runs under nothing else | ⟳ <https://developer.hashicorp.com/terraform/language/state/purpose> |
| T9 | Remembers what it built; delete by un-declaring | Y | state's stated primary purpose is exactly this mapping — "Terraform uses this mapping to know that the resource … represents a real world object with the instance ID `i-abcd1234`" | ⟳ same page as T8 |
| T10 | Templating / config-file generation | Y | `templatefile()` "reads the file at the given path and renders its content as a template using a supplied set of template variables" | ⟳ <https://developer.hashicorp.com/terraform/language/functions/templatefile> |
| T11 | Fleet: inventory, groups, per-host data | Y | see CHAFE-6 — HashiCorp frames none of `for_each` / variables / workspaces as inventory | ⟳ <https://developer.hashicorp.com/terraform/cli/workspaces> |
| T12 | Existing scripts run unchanged | N/A | see CHAFE-3 — `remote-exec`'s `script`/`scripts` copy and execute an existing local file, create-time only | ⟳ <https://developer.hashicorp.com/terraform/language/block/resource> |

Verbatim, ✓ from
<https://developer.hashicorp.com/terraform/language/resources/provisioners/syntax>
(one page carrying T5, T6, T7 and three prose claims):

> "You should exhaust all alternatives before using provisioners in your
> configurations. This is because Terraform cannot predictably model provisioner
> behaviors represented in the configuration."

> "To use provisioners that upload files over SSH, the `scp` service program must
> be installed on the remote system."

> "Because the SSH connection type is most often used with newly-created remote
> resources, SSH host key validation is disabled by default."

Verbatim, ✓ from
<https://developer.hashicorp.com/terraform/language/post-apply-operations> —
the single best citation in the table for Dorc's niche being a documented
product boundary:

> "Terraform is primarily designed for immutable infrastructure operations, so we
> strongly recommend using purpose-built solutions to perform post-apply
> operations."

…followed by the sanctioned alternatives: pass data via the provider's cloud-init
implementation, use the `cloudinit-config` data source, provision machine images,
or **use configuration management tools**.

Verbatim, ✓ from <https://developer.hashicorp.com/terraform/language/state>:

> "Terraform must store state about your workspace's managed infrastructure and
> configuration."
> "Terraform expects a one-to-one mapping between configured resource instances
> and remote objects."
> "…can result in data loss or exposure of secrets stored in the state file."

Verbatim, ⟳ from
<https://developer.hashicorp.com/terraform/language/state/purpose> — stronger
than the page above and the better citation for T8/T9:

> "State is a necessary requirement for Terraform to function."

Verbatim, ✓ from
<https://developer.hashicorp.com/terraform/language/state/sensitive-data>:

> "If you are developing with Terraform locally, Terraform stores your state in a
> plaintext file, which includes any secret values you defined in your
> configuration."

Verbatim, ✓ from
<https://developer.hashicorp.com/terraform/cloud-docs/workspaces/health>:

> "Drift detection determines whether your real-world infrastructure matches your
> Terraform configuration."

**Rollback, the clean negative (T2).** The first-party CLI command index
(⟳ <https://developer.hashicorp.com/terraform/cli/commands>) lists every command
and contains no `rollback` or `undo` verb — a citable negative result. Two
honesty riders the verifier surfaced and I am passing through rather than
smoothing over:

- HCP Terraform (paid) *does* document a rollback, UI-only, of the **state
  version** — which reverts what Terraform believes it is tracking, not what the
  infrastructure is. "Terraform has no rollback" is true of the CLI and false in
  that narrow bookkeeping sense; the footnote above is worded to survive both.
- There is **no** first-party sentence saying "restoring `terraform.tfstate.backup`
  does not revert infrastructure". That is inference from state's framing as a
  mapping rather than a change-log, and it must not be quoted as HashiCorp's
  words. Relatedly, the page titled *Recover state from backup*
  (`/terraform/cli/state/recover`) is a force-unlock / `state pull` / `state push`
  workflow that never mentions `terraform.tfstate.backup` at all — a documented
  gap worth one wry sentence in prose if the conductor wants it.

**Wording rider on T1.** No Terraform page says "provider API". The operative
term throughout is "remote objects"; the closest first-party phrasing is
state/purpose's "Terraform can query your providers and sync the latest
attributes from all your resources." Cite that, not a paraphrase.

**Page-identity rider.** The verifier reports that `provisioners/syntax` and
`provisioners/remote-exec` render effectively identical content, and that the
`script`/`scripts` argument prose actually lives on
`/terraform/language/block/resource`. Cite the resource-block page for those two
arguments; do not cite `remote-exec` for text that is not on it.

### cloud-init column

| # | Row | Cell | Proposed footnote | URL |
|---|---|---|---|---|
| C1 | You declare the end state; the tool owns the steps | ~ | `#cloud-config` is declarative; `bootcmd`, `runcmd` and the script formats are not | ✓ <https://docs.cloud-init.io/en/latest/explanation/format.html> |
| C2 | Remembers what it built; delete by un-declaring | ~ | see CHAFE-4 — it remembers *that a unit ran* (sem files keyed on a name and a cached instance-id), never what it produced | ✓ <https://github.com/canonical/cloud-init/blob/26.1/tools/cloud-init-per> |
| C3 | Secrets management story | N | the payload channel is an unauthenticated read from inside the box on EC2; Azure advises against secrets in either of its two payload fields | ✓ §0a |
| C4 | Templating / config-file generation | ~ | real but narrow: `cc_write_files`, plus jinja over instance-data (`## template: jinja` as the literal first line, stacked *above* the format header) — no inventory to loop over | ⟳ <https://docs.cloud-init.io/en/latest/explanation/format/jinja.html> |
| C5 | Check-then-converge inside its own units | N | see CHAFE-1 — I believe this cell is wrong today | ⟳ cloud-init source at tag 26.1 |
| C6 | Preview before mutating | N | `cloud-init schema --system` validates a config; nothing previews what it would change | ⟳ <https://docs.cloud-init.io/en/latest/reference/cli.html> |
| C7 | Re-measures the live system every run | N | boothooks and `scripts/per-boot/` re-EXECUTE on every boot; nothing re-MEASURES | ✓ <https://docs.cloud-init.io/en/latest/explanation/format/boothook.html> |
| C8 | Convergence machinery for raw shell content | N | upstream disclaims re-running its own payload, and a failing boothook is caught, logged and swallowed rather than failing the boot | ⟳ rerun-howto + ✓ `boot_hook.py` |
| C9 | The reviewed text is byte-for-byte what executes | N | see CHAFE-2 — no review surface exists, but for the script formats the executed file *is* the authored bytes minus the stripped header line | ✓ `boot_hook.py` |
| C10 | Skips explained with queryable provenance | N | see CHAFE-5 — a frequency-gated skip is logged and carries a distinct event description ("previously ran") in the stream `cloud-init analyze` parses | ⟳ source at 26.1 |
| C11 | Partial work chosen by measuring the machine | N | chosen by frequency and a cached instance-id, never by state | ⟳ <https://docs.cloud-init.io/en/latest/explanation/first_boot.html> |
| C12 | Off-ramp: stop using it, keep working artifacts | ~ | a `#!` user-data script is plain sh you keep and run anywhere; a `#cloud-config` is inert without cloud-init | ✓ format.html |
| C13 | Works before ssh exists (the first-boot seat) | Y | boothooks run in the Network stage, before every config module — including `cc_ssh`, which is what generates the host keys sshd will present | ✓ boothook + ⟳ <https://docs.cloud-init.io/en/latest/explanation/boot.html> |
| C14 | Existing scripts run unchanged | Y | "a single script to be executed once per instance" — prefix with `#!` and hand the file over | ✓ <https://docs.cloud-init.io/en/latest/explanation/format/user-data-script.html> |
| C15 | Full value without learning an authoring layer | ~ | the script format needs nothing new; the module half needs `#cloud-config` YAML | ✓ format.html |
| C16 | Authored in the target's own language (plain sh) | ~ | same split as C15: script formats are sh, `#cloud-config` is YAML | ✓ format.html |
| C17 | Converges continuously with nobody present | N | boothooks and `scripts/per-boot/` *do* re-run unattended every boot — re-running is not converging, and that gap is this round's thesis | ✓ boothook |

Verbatim, ✓ from
<https://docs.cloud-init.io/en/latest/explanation/format/boothook.html>:

> "It is run very early in boot, during the network stage, before any cloud-init
> modules are run."
> "It runs every boot."

Verbatim, ⟳ from
<https://docs.cloud-init.io/en/latest/howto/rerun_cloud_init.html>:

> "Making cloud-init run again may be destructive and must never be done on a
> production system. Artefacts such as ssh keys or passwords may be overwritten."

Verbatim, ⟳ from
<https://docs.cloud-init.io/en/latest/explanation/first_boot.html> — and note it
names ssh host keys, which is the pivot book's identity-churn story arriving from
a second direction:

> "By default, `cloud-init` attempts to determine which case it is running in by
> checking the instance ID in the cache against the instance ID it determines at
> runtime."
> "…`cloud-init` is responsible for rotating SSH host keys on first boot, and this
> will not happen on these instances."

**Tagged permalinks, now pinned at release `26.1`** (⟳ verified; replaces the
`main` reads my first pass relied on). Five table cells and one prose claim rest
on these two files, so the pin matters more here than anywhere else in my lane:

`https://github.com/canonical/cloud-init/blob/26.1/cloudinit/handlers/boot_hook.py`
> `prefixes = ["#cloud-boothook"]`
> `handlers.Handler.__init__(self, PER_ALWAYS)`
> `util.write_file(filepath, contents.lstrip(), 0o700)`
> `subp.subp([filepath], update_env=env, capture=False)`
> `except subp.ProcessExecutionError: util.logexc(LOG, "Boothooks script %s execution error", filepath)`

`https://github.com/canonical/cloud-init/blob/26.1/tools/cloud-init-per`
> `DATA_PRE="/var/lib/cloud/sem/bootper"`
> `INST_PRE="/var/lib/cloud/instance/sem/bootper"`
> `[ "$(id -u)" = "0" ] || fail "must be root"`
> `name=$(echo $2 | sed 's/-/_/g')`

### Prose-section citations

| Claim in my prose | Status | URL |
|---|---|---|
| "Terraform cannot predictably model provisioner behaviors" | ✓ verbatim | provisioners/syntax |
| post-apply guidance naming cloud-init / images / config management | ✓ verbatim | post-apply-operations |
| "State is a necessary requirement for Terraform to function" | ⟳ verbatim | state/purpose |
| one-to-one mapping; secrets in state | ✓ verbatim | state |
| SSH host key validation disabled by default | ✓ verbatim | provisioners/syntax |
| cloud-init's re-run disclaimer | ⟳ verbatim | howto/rerun_cloud_init.html |
| "log errors, but proceed" | ⟳ verbatim | **explanation/return_codes.html**, NOT failure_states.html |
| eight-valued status incl. the degraded pair | ⟳ verbatim | howto/status.html |
| IMDS readability + 16 KB cap (EC2) | ✓ verbatim | AWS user-data |
| 64 KB cap; custom-data NOT in IMDS (Azure) | ✓ verbatim | Azure custom-data |
| Ignition: provisioning-not-config-management; "discard the old node and re-provision"; no arbitrary logic | ✓ verbatim | <https://coreos.github.io/ignition/rationale/> |
| Ignition: "produces the machine specified or no machine at all" | ✓ verbatim | same page |
| Talos: "We have no shell. We have no SSH." | ✓ verbatim | <https://www.talos.dev/v1.11/learn-more/philosophy/> |

Four URL-hygiene notes for whoever integrates:

- **Talos**: `/latest/learn-more/philosophy/` 404s; `/v1.11/…` resolves. A pinned
  Talos URL rots on their next release, an unpinned one is wrong today. Pinned is
  the lesser evil — it stays true about the version it names.
- **EC2**: `instancedata-add-user-data.html` (a plausible-looking guess) 404s.
  `user-data.html` resolves and carries both the cap and the instance-attribute
  language — one citation, two claims.
- **cloud-init user-data script**: the page is `format/user-data-script.html`
  (hyphens), not `user_data_script.html` (underscores); the underscore form 404s
  to the format index.
- **cloud-init failure semantics**: `failure_states.html` and `return_codes.html`
  are different pages with different content; the famous quote is on the latter.

Bonus precision, worth a footnote wherever `INSTANCE_ID` appears: it is
documented and current for **boothooks** and deprecated for **user-data
scripts** ("Use of `INSTANCE_ID` variable within user-data scripts is deprecated.
Use jinja templates with `v1.instance_id` instead" — ✓ user-data-script page).
The boothook book uses it in the format where it is correct; a reader who knows
only the deprecation will think otherwise.

---

## 2. Row-chafe

### CHAFE-1 · CHAFE-WRONG · cloud-init, "Check-then-converge inside its own units" = N

**Why it chafes.** The row asks whether the tool's own units re-check live state
before mutating. cloud-init's *rationing* is name/instance-id keyed and that is
genuinely N-shaped — but individual modules are not blind, and the source says so
at tag 26.1:

- `cc_growpart` is `frequency = PER_ALWAYS`, so it runs every boot and **cannot**
  lean on the gate. It runs `growpart --dry-run` first and, when there is no size
  delta, records `RESIZE.NOCHANGE` with `"no change necessary (%s, %s)"` and does
  not invoke the real `growpart`. That is check-then-converge, unambiguously.
- `add_user()` in `cloudinit/distros/__init__.py`, under `cc_users_groups`:
  `if util.is_user(name): LOG.info("User %s already exists, skipping."); return False`.
- `cc_ssh` checks `if os.path.exists(keyfile): continue` per key type — with the
  honest caveat that `ssh_deletekeys` defaults `True` and wipes the keys first, so
  in stock config the net first-boot effect is still "regenerate". Weakest of the
  three; cite the first two.

A cloud-init maintainer reading a flat `N` will not recognise their tool.

**Proposed.** `~`, footnote: *"several config modules read before they act
(`cc_growpart` dry-runs before resizing; `add_user` skips existing users); the
run-once frequency gate, not a state check, is what rations the payload as a
whole."*

**Why this costs Dorc nothing.** The differentiator Dorc actually claims is one
row down — "Convergence machinery for raw shell content" — where cloud-init is an
unambiguous `N` and the evidence is strongest. Conceding the module-interior row
makes the shell-content row *more* credible.

### CHAFE-2 · CHAFE-WORDING · cloud-init, "The reviewed text is byte-for-byte what executes" = N

**Why it chafes.** The row bundles two independent claims: (a) a review surface
exists, and (b) what executes is the bytes you wrote. cloud-init fails (a)
completely and **passes** (b) for its script formats — `boot_hook.py` strips the
one header line, `lstrip()`s, writes the remainder `0o700`, and `subp`s the file
directly. That is byte-fidelity of exactly the kind the row asks about.

**Proposed.** Mark `~` with the footnote *"no review surface exists at all; but
for the script formats the executed file is the authored bytes minus the stripped
header line"*. The structurally honest alternative is to split the row — "there is
a preview surface" (already one row up) versus "what executes is the authored
text". `N/A` is also defensible on the grounds that "the reviewed text" is vacuous
where nothing is reviewed.

**Countervailing, stated so the conductor can weigh it.** This is Dorc's headline
row and softening a neighbour's cell on it reads as a concession. It is not: the
Dorc claim is about the *plan*, and a plan is what cloud-init has none of. As
worded, the row credits Dorc for byte-fidelity — which the script formats also
have — and hides the genuinely rare property, which is a *reviewable* plan whose
bytes are the executed bytes.

### CHAFE-3 · CHAFE-WORDING · Terraform, "Existing scripts run unchanged (the adoption floor)" = N/A

**Why it chafes.** `N/A` means "outside that tool's layer". It is not outside
Terraform's layer: `provisioner "remote-exec"` takes `script` and `scripts`
arguments documented as specifying "the relative or absolute path to a script on
the local machine to **copy and execute** on the remote resource"
(⟳ `/terraform/language/block/resource`). HashiCorp documents the feature and
simultaneously routes you away from it. "Inside the layer and discouraged" is a
different and more interesting statement than "outside the layer".

**Proposed.** `~`, footnote: *"`remote-exec`'s `script`/`scripts` copy and execute
an existing local file, at create time only, from a feature HashiCorp tells you to
exhaust all alternatives before using."* Terraform `~`, cloud-init stays `Y`, Dorc
stays `Y`.

**Rider.** The docs say "copy and execute", never the word "unchanged", and there
is no templating step in that path — so "as-is" is a fair characterisation but
should not be quoted. Create-time-only is `~SUSPECT`-tier synthesis by the
verifier from the create/destroy framing, not a single verbatim sentence.

### CHAFE-4 · CHAFE-WORDING, downward · cloud-init, "Remembers what it built; delete by un-declaring" = ~

**Why it chafes.** `~` over-credits. cloud-init remembers exactly one thing —
*that a named unit already fired* — via a sem file and a cached instance-id. It
never records what the work produced, and there is no path from removing a
directive to undoing its effect; `cloud-init clean` erases cloud-init's own
bookkeeping, not the machine's state. Both halves of the row fail.

**Proposed.** `N`, footnote: *"remembers that a unit ran — sem files keyed on a
name and a cached instance-id — never what it produced; removing a directive
undoes nothing."*

Flagged deliberately: an audit that only argues its own column upward is not an
audit.

### CHAFE-5 · CHAFE-WORDING, downward · cloud-init, "Skips explained with queryable provenance" = N

**Why it chafes.** A frequency-gated skip *is* recorded, in two places, at 26.1:

- `cloudinit/helpers.py`, `Runners.run()`:
  `if sem.has_run(name, freq): LOG.debug("%s already ran (freq=%s)", name, freq)`
- `cloudinit/config/modules.py`, `_run_modules()`: a module that did not run gets
  `myrep.message = "%s previously ran" % run_name` instead of "ran successfully",
  and that message becomes the description of a reporting `finish` event — the
  same event stream `cloud-init analyze dump/show/blame` parses.

So the skip and its cause are machine-readable, not merely inferable. What
cloud-init does not have is a *chain* (no "…because the cached instance-id
matched, because…") or a why-verb.

**Proposed.** `~`, footnote: *"a frequency-gated skip is logged and carries a
distinct event description ('previously ran') in the stream `cloud-init analyze`
parses; there is no provenance chain and no why-verb."*

**Rider, honestly.** The verifier is `+SURE` on both source quotes and
`~SUSPECT` that the literal string survives into a rendered `analyze show`
transcript — the code path was traced, no worked example was found. If the
conductor wants `~` to be safe rather than merely defensible, the footnote should
say "in the event stream" rather than naming the rendered output.

### CHAFE-6 · CHAFE-WORDING · Terraform, "Fleet: inventory, groups, per-host data" = Y

**Why it chafes.** This one I got wrong in my own fragment and the verifier caught
it. HashiCorp frames *none* of `for_each`, variables, or workspaces as inventory.
`for_each` is documented as managing "several similar objects … without writing a
separate block for each"; variables are a module input interface; and the
workspaces page actively discourages the closest analogue — "Workspaces alone are
not a suitable tool for system decomposition", and CLI workspaces sharing one
backend "are not a suitable isolation mechanism". There is no group-variable or
dynamic-inventory concept, and no unifying fleet framing to point at.

**Proposed.** `~`, footnote: *"resource multiplicity via `for_each` and
parameterisation via variables; HashiCorp unifies none of it as inventory, and
steers away from workspaces as an environment-separation mechanism."*
⟳ <https://developer.hashicorp.com/terraform/cli/workspaces>

### CHAFE-7 · CHAFE-WORDING · Terraform, "Re-measures the live system every run" = ~

**Why it chafes.** Terraform re-measures its entire managed surface on every plan
and apply, by default, with no cached truth — it "reads the current state of any
already-existing remote objects to make sure that the Terraform state is
up-to-date". The `~` is presumably charged for measuring remote objects rather
than the interior of a machine; but a machine's interior is not Terraform's
managed surface, so the row demotes it for declining to do something outside its
layer — the exact thing the `N/A` convention exists to prevent.

**Proposed, two options.**
- *(a) Fairness-first:* **"Re-measures its own managed surface every run (no
  cached truth)"**. Terraform `Y`, cloud-init `N`, Dorc `Y`; the
  remote-object-versus-host-interior distinction moves to prose, where it is a
  *layer* fact rather than a capability deficit.
- *(b) Keep the intent, say it out loud:* **"Re-measures state inside the managed
  machine every run"**. Terraform becomes an honest `N` rather than `~`,
  Kubernetes stays `Y` (probes look inside containers), Dorc `Y`.

I prefer (a) for fairness and (b) for usefulness to an ops reader. Either beats
the current wording. If another lane pulls this row the other way, it is an honest
"ehhhh" and should stay.

### CHAFE-8 · CHAFE-DISCIPLINE · the header's discipline sentence versus the new `NYI` mark

Not a request to drop rows — the human introduced `NYI` deliberately and I am not
relitigating it. The observation is narrow: the stated discipline is
*architecture-tier only, never NYI/someday items either side could fix with mild
work*, and `NYI`'s own footnote says "musts before I call Dorc ship-able" — i.e.
exactly the excluded class. Two rows now sit inside a fence the header declares
closed, and that will be a reader's first question.

**Proposed.** Keep both rows; give the discipline sentence a clause — *"…except
where a cell is marked NYI, which is a deliberate, Dorc-side-only admission that
the row is architecture-tier for everyone else and unbuilt here."* One sentence.

### Low-severity, one line each

- **Terraform, "Privilege escalation built in" = N/A.** Terraform does run remote
  commands via provisioners and offers no escalation helper for them, so `N` is
  arguably more accurate than "outside its layer". Low stakes.
- **cloud-init, "Preview before mutating" = N.** Correct, but `cloud-init schema
  --system` is close enough that a reader may think we missed it — worth the C6
  footnote for that reason alone.
- **Terraform, `terraform refresh` is deprecated** ("This command is deprecated.
  Instead, add the `-refresh-only` flag…"). Not a cell change; just do not cite
  the refresh command as live machinery anywhere in prose.

---

## 3. Out-of-lane flags

Free observations, no research owed — take or ignore.

- **OUT-OF-LANE — Ansible, "Existing scripts run unchanged" = ~.**
  `ansible.builtin.script` transfers and runs an existing local script unmodified;
  the `~` may be charging Ansible for the playbook wrapper rather than for its
  script handling.
- **OUT-OF-LANE — nix/NixOS, "Re-measures the live system every run" = N.**
  `nixos-rebuild switch` compares the current-system store path against the newly
  built closure — a live-system measurement of an unusually exact kind, arguably
  the soundest convergence check in the table. `N` looks harsh; interacts with
  CHAFE-7's rewording.
- **OUT-OF-LANE — Kubernetes, "Privilege escalation built in" = N/A.**
  `securityContext` / `privileged` is first-class privilege control; whether it
  answers this row depends on whether the row means "escalate a step" or "control
  privilege at all". One look from that column's owner.

---

## 4. Scoreboard

- **29 citations** supplied: 12 Terraform cells, 17 cloud-init cells, plus 13
  prose-claim citations and 2 tagged source permalinks.
- **URLs fetched and status-checked this pass: 22.** Three were wrong in my first
  pass and are corrected above (Talos `/latest/`, EC2 `instancedata-add-user-data`,
  cloud-init `user_data_script` underscores). One was cited against the wrong page
  entirely (`failure_states` for the "log errors, but proceed" quote).
- **8 chafes**: 1 WRONG, 6 WORDING (two of them downward, against cells in my own
  lane), 1 DISCIPLINE. Plus 3 low-severity one-liners and 3 out-of-lane flags.
- **2 substantive self-corrections** (§0), both cases of my own prose being
  over-broad in Dorc's favour.
