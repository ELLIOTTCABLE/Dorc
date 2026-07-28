# SIBLINGS audit: the siblings' columns (Ansible · pyinfra · cdist)

> docID for cross-reference: **SIBaud**. Advocate-and-auditor pass over the three columns
> the conductor synthesised from turns 02/04/05 without a builder to defend them. Written
> from the round ledger under `.claude/research/ops-glue-residue/`; the web was used only
> to verify that a URL resolves, to reach a first-party page the ledger lacks, and to
> re-test the one claim that had gone stale. **Every URL below was checked and returns 200
> on 2026-07-28.**
>
> Standing limits observed: no edit was made to root `SIBLINGS.md`. Footnote *texts* below
> are proposals for the conductor and the human to accept, rewrite or reject — they are
> not text to paste unreviewed. No subagents were dispatched.

---

## 0. Ledger corrections — read first, one of these makes current prose wrong

### SIBaud:cor-cdist-upstream-is-live — CDIST UPSTREAM IS REACHABLE

The cdist prose says *"upstream is unreachable, the Debian package is orphaned, and the
public mirror lags the last release."* Clause one is **false as of this audit**, and the
turn-04 note it descends from (`Blocked / not obtained`: "`code.ungleich.ch` — down
(human-confirmed in turn B)") should be treated as superseded.

Measured, 2026-07-28:

| Fact | Value | Where |
| --- | --- | --- |
| Host | HTTP 200; a live **Gitea** instance (not the GitLab the README still describes; the repo's own `original_url` is `code-legacy.ungleich.ch`) | <https://code.ungleich.ch/ungleich-public/cdist> |
| Repo state | public, **not archived**, `updated_at` 2026-06-05, 59 open issues, 15 open PRs | <https://code.ungleich.ch/api/v1/repos/ungleich-public/cdist> |
| Newest tag | **7.0.0**, 2022-07-31 — matches PyPI exactly | <https://code.ungleich.ch/api/v1/repos/ungleich-public/cdist/tags> |
| Newest commit on `master` | **2026-06-05**, by Nico Schottelius (project founder), after a ~22-month gap; ~40 commits span 2022-05 → 2026-06 | <https://code.ungleich.ch/ungleich-public/cdist/commits/branch/master> |
| Debian | orphaned, bug **#947641**, package version 7.0.0-6, with Debian-side commits in 2026-04 | <https://bugs.debian.org/947641> · <https://tracker.debian.org/pkg/cdist> |
| GitHub mirror | tip 6.0.4, 2019-11-19 — genuinely stale | <https://github.com/ungleich/cdist> |

Two of the three clauses survive; the conclusion ("hard to recommend today for
non-architectural reasons") survives easily on *four years without a release*. Only the
"lights are off" framing has to go. Proposed replacement sentence, for the human to take
or leave: **"Status, said plainly and kindly: no release since 7.0.0 in 2022, upstream
commits at a trickle (last one June 2026), the Debian package orphaned, and the public
GitHub mirror four releases behind. An architecture to learn from rather than a moving
competitor."**

Why this matters beyond accuracy: "unreachable" reads as *the maintainer walked away*,
which is both untrue and the least kind reading available — and the section's own header
promises to say it *kindly*.

### SIBaud:cor-cdist-types-verified-current — the 7.0.0 tree is now readable

Turn 04 marked "whether 7.0.0 changed the type/explorer/gencode contract" as genuinely
unverifiable and guessed *no*. It is verifiable now, and the guess holds. Counted from the
7.0.0 tree: **173 shipped types** (turn 04's 153 was the 6.0.4 number; the "-1:GUESS for
7.0.0" line can be retired). The contract is unchanged. Every cdist source claim in the
round that was hedged as "6.0.4 only" can be re-grounded at the current release if anyone
wants to spend the calls. <https://code.ungleich.ch/ungleich-public/cdist/src/tag/7.0.0/cdist/conf/type>

### SIBaud:cor-pyinfra-rebuild-count-unsupported — "three times over" is not in evidence

The pyinfra prose says the maintainer *"rebuilt the ordering engine three times over."*
The ledger does not support a count of three, and neither do the changelogs. What is
documented is **two** engine rewrites: v2.0 *"Parallel operation generation & facts
rewrite — this is a huge improvement to how pyinfra generates commands to run on target
hosts"* (<https://raw.githubusercontent.com/pyinfra-dev/pyinfra/v2.9.2/CHANGELOG.md>), and
v3.0 *"pyinfra now executes operations at runtime, rather than pre-generating commands"*
(<https://raw.githubusercontent.com/pyinfra-dev/pyinfra/v3.10.0/CHANGELOG.md>). The 1.0
release was largely legacy-API removal. Recommend "twice" or dropping the number.

Two smaller nits in the same sentence, both cheap to fix:

- *"worked seven years"* — the "7(!) years" quote is the creator writing in **May 2022**.
  Read in 2026 the sentence implies a seven-year span ending now. Tense it to the quote.
- *"and finally declared architectural"* — v3.0 shipped a real fix for the pre-generation
  half and says so ("this fixes an entire class of bugs and confusion"). What is standing
  and documented today is the *fact ordering* consequence, with `_if` as the opt-in
  execute-time answer. "Declared architectural" is defensible about the surviving half but
  reads as defeat about the whole. The Dorc contrast is actually sharper stated precisely:
  their guard is opt-in per call site, ours is the default.

---

## 1. Citations

Eyebrow cells and loaded prose claims only; obvious cells get nothing. Column in **bold**.

### Ansible

**SIBaud:cit-ansible-ordered-play-imperative** — row *"You declare the end state; the tool
owns the steps"*, **Ansible ~**
> Modules are desired-state (`state: present`); the play around them is not — "a playbook
> runs in order from top to bottom," and so do the tasks inside each play.

<https://docs.ansible.com/projects/ansible/latest/playbook_guide/playbooks_intro.html>

**SIBaud:cit-ansible-check-mode-partial-shell** — row *"Preview before mutating"*,
**Ansible ~**
> `shell`, `command` and `script` all declare check-mode support "partial": the command
> "cannot be subject to the check mode semantics", with `creates`/`removes` offered as the
> workaround.

<https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/shell_module.html>

**SIBaud:cit-ansible-preview-thins-on-conditionals** — same row, the deeper half
> Check mode "will not generate output for tasks that use conditionals based on registered
> variables" — the preview thins exactly where a play branches on what an earlier task
> returned.

<https://docs.ansible.com/projects/ansible/latest/playbook_guide/playbooks_checkmode.html>

**SIBaud:cit-ansible-script-transfers-verbatim** — row *"Existing scripts run unchanged"*,
**Ansible ~**
> `script:` transfers your local file and runs it verbatim, requiring no Python on the
> target; what it adds is `creates`/`removes` and a result record, nothing that reads the
> script.

<https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/script_module.html>

**SIBaud:cit-ansible-changed-when-unverified** — prose, "`changed_when` annotations
describe on the honor system"
> `changed_when` is an author-supplied expression over the task's own rc and output;
> nothing checks it against the machine, and `changed_when: False` is a documented way to
> say "never report a change."

<https://docs.ansible.com/projects/ansible/latest/playbook_guide/playbooks_error_handling.html>

**SIBaud:cit-ansible-ssm-bucket-even-for-shell** — prose, "the SSM plugin requires an S3
bucket just to move files"
> Required "even for modules which do not explicitly send files (such as the `shell` or
> `command` modules), because Ansible sends over the `.py` files of the module itself, via
> S3" — and passwords so moved "will be included in plaintext in those files in S3
> indefinitely."

<https://docs.ansible.com/projects/ansible/latest/collections/amazon/aws/aws_ssm_connection.html>

> Accuracy note for the prose: "just to move files" *undersells* it. The bucket is
> mandatory when you are moving no files at all. Recommend "even when you are moving no
> files."

**SIBaud:cit-ansible-script-forces-pseudo-tty** — prose, the `-tt` stderr-merge wart —
this one is first-party and verbatim
> "The `ansible.builtin.ssh` connection plugin will force pseudo-tty allocation via `-tt`
> when scripts are executed. Pseudo-ttys do not have a stderr channel and all stderr is
> sent to stdout."

<https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/script_module.html>
(Notes) — mechanism at `ssh.py` L1515-1519, `if not in_data and sudoable and use_tty:
args = ('-tt', ...)`:
<https://raw.githubusercontent.com/ansible/ansible/v2.21.2/lib/ansible/plugins/connection/ssh.py>

**SIBaud:cit-ansible-raw-not-byte-clean** — existing footnote `[^a1]`, add a link
> "This action is one of the few that requires no Python on the remote"; its own limit is
> that non-UTF-8 output "must be encoded to avoid issues."

<https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/raw_module.html>

### pyinfra

**SIBaud:cit-pyinfra-dry-run-is-a-matrix** — row *"Preview before mutating"*, **pyinfra ~**
> `--dry` renders an operation×host change matrix, not commands: the literal commands
> "aren't generated until execute time." `--diff` covers file contents only; shell appears
> at `-vv`, after consent.

<https://docs.pyinfra.com/en/3.x/using-operations.html>

**SIBaud:cit-pyinfra-limit-selects-hosts-only** — row *"Partial work chosen by measuring
the machine"*, **pyinfra N**
> `--limit` and `--exclude` select host names, globs and groups. Nothing selects by what
> has drifted.

<https://docs.pyinfra.com/en/3.x/cli.html>

**SIBaud:cit-pyinfra-script-op-uploads-verbatim** — row *"Existing scripts run
unchanged"*, **pyinfra ~**
> `server.script` uploads your local script and runs it verbatim; the deploy that calls it
> is Python, and does not survive pyinfra's removal.

<https://docs.pyinfra.com/en/3.x/operations/server.html>

**SIBaud:cit-pyinfra-facts-read-during-prepare** — prose, the staleness claim, current and
first-party
> "Facts are read during prepare, before any operation runs, so a mutable fact branched on
> in Python sees pre-deploy state — not the state at the point of the branch in source
> order."

<https://docs.pyinfra.com/en/3.x/using-operations.html>

**SIBaud:cit-pyinfra-lockstep-is-why-prepare** — prose, the "lock-step … the very thing
that forces its two-phase machinery" claim. **I set out to break this one and could not:
upstream states the causal link outright.**
> "all hosts complete operation N (or fail it) before any host starts operation N+1 …
> Getting that ordering right is precisely what the prepare phase is for."

<https://docs.pyinfra.com/en/3.x/deploy-process.html>

**SIBaud:cit-pyinfra-ordering-not-commands** — row *"Plan artifact runs without the tool
installed"*, footnote `[^p2]`, add a second link
> Upstream on what prepare produces: "What it has built is an ordering, not a stored list
> of commands."

<https://docs.pyinfra.com/en/3.x/deploy-process.html> · design and blocker at
<https://github.com/pyinfra-dev/pyinfra/issues/688>

**SIBaud:cit-pyinfra-noop-prose-at-verbose** — row *"Skips explained with queryable
provenance"*, footnote `[^p1]`
> "`-v`: print out facts collected as well as noop information (package X already
> installed)" — author-written prose per operation, printed only at raised verbosity.

<https://docs.pyinfra.com/en/3.x/cli.html>

**SIBaud:cit-pyinfra-creates-containers-only** — row *"Creates infrastructure (VMs, DNS,
networks)"*, **pyinfra ~**
> Containers and their local resources only (`docker.*`, `lxd.container`, `vzctl`, `zfs`).
> There is no cloud provider module; `terraform` and `vagrant` appear as *inventory
> sources*, never as provisioners.

<https://raw.githubusercontent.com/pyinfra-dev/pyinfra/v3.10.0/pyinfra-metadata.toml> ·
<https://docs.pyinfra.com/en/3.x/operations/lxd.html>

### cdist

**SIBaud:cit-cdist-inventory-still-beta** — row *"Fleet: inventory, groups, per-host
data"*, **cdist ~**
> "Inventory functionality is still in beta so it can be used only if beta command line
> flag is specified" — and "Tags have no values, as tags are just tags." No per-host or
> per-group data exists to attach.

<https://www.cdi.st/manual/latest/cdist-inventory.html>

**SIBaud:cit-cdist-explorers-per-object** — row *"Partial work chosen by measuring the
machine"*, **cdist ~** (see the chafe entry; this cites the cell as written)
> Stage 3 runs each type's own explorers on the target, per object; a converged object's
> `gencode` then prints nothing, and the object does no work.

<https://www.cdi.st/manual/latest/cdist-stages.html>

**SIBaud:cit-cdist-sh-through-an-emulator** — row *"Authored in the target's own
language"*, footnote `[^cd1]`
> Type invocations look like commands (`__package tree --state installed`) because cdist
> prepends a private `bin/` of emulator symlinks to `PATH`; each writes an object into a
> database. Run the manifest without cdist and you get `__package: command not found`.

<https://www.cdi.st/manual/latest/cdist-type.html> ·
<https://raw.githubusercontent.com/ungleich/cdist/c1633d9301c053b604cceb39b57621636e80b9cd/cdist/config.py>

**SIBaud:cit-cdist-dry-run-withholds-code** — row *"Preview before mutating"*, **cdist ~**
> `-n` is documented as "Do not execute code": explorers and generators still run, only
> objects that generated code are logged, and the generated sh lands in the out-dir
> without ever being shown.

<https://www.cdi.st/manual/latest/man1/cdist.html> ·
<https://raw.githubusercontent.com/ungleich/cdist/c1633d9301c053b604cceb39b57621636e80b9cd/cdist/config.py>

**SIBaud:cit-cdist-has-no-script-type** — row *"Existing scripts run unchanged"*, **cdist
N** — verified at the current release, not inferred
> Of the 173 types shipped at 7.0.0 not one is a `__script`, `__shell`, `__exec` or
> `__command`. Running a file you already have means authoring a type around it.

<https://code.ungleich.ch/ungleich-public/cdist/src/tag/7.0.0/cdist/conf/type>

**SIBaud:cit-cdist-root-unless-rewrapped** — row *"Privilege escalation built in"*,
**cdist ~**
> The defaults are `ssh -o User=root` and `scp -o User=root`; escalation means replacing
> the whole transport with a script that "must behave as if it were ssh" — the manual
> ships a `sudo` example, which is the whole story.

<https://www.cdi.st/manual/latest/cdist-remote-exec-copy.html>

**SIBaud:cit-cdist-explorers-run-on-target** — row *"Re-measures the live system every
run"*, **cdist Y**
> "Explorers are small shell scripts, which will be executed on the target host" — every
> global explorer, every run, before anything is decided.

<https://www.cdi.st/manual/latest/cdist-explorer.html>

**SIBaud:cit-cdist-connection-per-explorer** — prose, "pays one connection per explorer
per object — its own manual's stated regret"
> "As cdist makes many connections to each host successive connections can be sped up by
> [ControlMaster]" — inheriting sshd's `MaxSessions 10` as a concurrency ceiling; the
> source shows one remote exec per explorer, repeated per object in stage 3.

<https://www.cdi.st/manual/latest/cdist-best-practice.html> ·
<https://www.cdi.st/manual/latest/cdist-stages.html>

> Register note: "its own manual's stated regret" is a shade stronger than the manual's own
> register — the manual states a cost and a mitigation, it does not repent. "its own
> manual's stated cost" would be exact and loses nothing.

**SIBaud:cit-cdist-preos-manufactures-ssh** — prose, PreOS
> "Since PreOS is configured with ssh authorized key it can be accessed through ssh, i.e.
> it can be further installed and configured with cdist" — the precondition is
> manufactured, not bypassed.

<https://www.cdi.st/manual/latest/cdist-preos.html>

**SIBaud:cit-cdist-project-status-precise** — prose, the status sentence. Use
`SIBaud:cor-cdist-upstream-is-live` above; the citable pieces are the orphan bug
(<https://bugs.debian.org/947641>), the tracker page
(<https://tracker.debian.org/pkg/cdist>), PyPI's 7.0.0 / 2022-07-31 record
(<https://pypi.org/pypi/cdist/json>), and upstream's own commit log
(<https://code.ungleich.ch/ungleich-public/cdist/commits/branch/master>).

---

## 2. Row-chafe — as the siblings' advocate

Severities: **CHAFE-WRONG** (cell is factually wrong today; evidence required) ·
**CHAFE-WORDING** (cell would flip under a slightly different row wording; fairness) ·
**CHAFE-DISCIPLINE** (row is not architecture-tier for this tool; either side could close
it with ordinary work). Cutting both ways is the point: three of these say a sibling cell
is too *generous*.

### SIBaud:chf-resident-software-conflates-interpreter — CHAFE-WORDING, high

- **Row:** "No resident software on the managed machine" — **Ansible ~[^a1]**, pyinfra Y,
  cdist Y, Dorc Y.
- **Chafe:** "Resident" means *stays there*. Ansible leaves nothing: it transfers module
  files, runs them, deletes them. What it *needs* is a pre-existing Python interpreter —
  a prerequisite, not a residency. Under the row as worded Ansible is a clean **Y**; the
  `~` is smuggling in a different axis (interpreter floor) under a word that does not
  carry it. This is also the one place the table quietly does what its own editing-notes
  footer forbids: "never claim the agentless floor against the siblings (it is shared)."
- **Proposed:** split, or retitle. Either *"Leaves nothing resident on the managed
  machine"* (Ansible **Y**, all four Y, row becomes a Big-Boys-only row and probably
  earns its keep there) or *"Runs with nothing on the target but ssh and POSIX sh"*
  (Ansible **~** honestly, pyinfra/cdist/Dorc **Y**). The second is the one that carries
  information, and it is the one the footnote already describes.
- **URL:** <https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/raw_module.html>

### SIBaud:chf-partial-by-state-double-counts — CHAFE-WORDING, high; cuts both ways

- **Row:** "Partial work chosen by measuring the machine" — Ansible N, pyinfra N,
  **cdist ~**, Dorc Y.
- **Chafe:** as literally worded, all three siblings do this: every Ansible module, every
  pyinfra operation and every cdist object measures and then does only what is needed.
  That is precisely the row two above it, "Check-then-converge inside its own units",
  where all three are already Y. So cdist's `~` (per-object explorers) is **too
  generous** and Ansible's and pyinfra's N are **too harsh** — the same fact is scored
  three different ways. Meanwhile the distinction the round actually established (turn
  04: "Both incumbents partition the *fleet*; neither partitions the *book*") is invisible
  in the wording.
- **Proposed:** *"Chooses which parts of your program to run by measuring the machine
  (not just which hosts)"* — under which Ansible **N**, pyinfra **N**, cdist **N** (its
  `-t`/`-a`/`-A` selection is fleet-shaped and beta), Dorc **Y**. The row gets sharper and
  the double-count disappears.
- **URL:** <https://docs.pyinfra.com/en/3.x/cli.html> ·
  <https://www.cdi.st/manual/latest/cdist-inventory.html>

### SIBaud:chf-raw-shell-convergence-too-harsh — CHAFE-WORDING, high

- **Row:** "Convergence machinery for raw shell content" — **Ansible N**, pyinfra N, cdist
  N, Dorc Y.
- **Chafe:** this is the table's headline Dorc row, which makes an over-harsh sibling cell
  the most damaging kind of unfairness. Ansible ships `creates:` and `removes:` on
  `command`, `shell` and `script`, first-party, documented *as the check-mode workaround
  for arbitrary commands*. That is convergence machinery for raw shell content. It is
  hand-declared rather than derived, which is the real difference — and the row does not
  say so.
- **Proposed:** *"Convergence machinery **derived from** raw shell content"* or
  *"…without the author declaring the guard."* Ansible stays **N** and deserves to; the
  claim survives intact and stops being contestable by a reader holding the `shell` module
  page.
- **URL:** <https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/shell_module.html>

### SIBaud:chf-creates-infrastructure-flattens-ansible — CHAFE-WORDING, medium; cuts both ways

- **Row:** "Creates infrastructure (VMs, DNS, networks)" — **Ansible ~**, **pyinfra ~**,
  cdist N, Terraform Y.
- **Chafe:** Ansible and pyinfra are not remotely the same thing here and the shared `~`
  says they are. Ansible ships mature first-party provisioning across AWS, Azure, GCP,
  OpenStack, VMware and Proxmox — `amazon.aws.ec2_instance` creates VMs,
  `amazon.aws.route53` creates DNS. pyinfra creates *containers* (`docker.*`,
  `lxd.container`, `vzctl`) and nothing else; its `terraform` and `vagrant` entries are
  inventory sources, not provisioners. Ansible reads as **Y** on this row. The reason it
  feels like a `~` is that it forgets what it made — but that is the very next row up,
  "Remembers what it built; delete by un-declaring", where Ansible is already N. The `~`
  is charging Ansible twice for statelessness.
- **Proposed:** Ansible **Y**, pyinfra **~** with the containers-only footnote
  (`SIBaud:cit-pyinfra-creates-containers-only`). If the conductor would rather keep both
  at `~`, the row needs a footnote saying the two `~`s mean different things.
- **URL:** <https://docs.ansible.com/projects/ansible/latest/collections/amazon/aws/ec2_instance_module.html>
  · <https://raw.githubusercontent.com/pyinfra-dev/pyinfra/v3.10.0/pyinfra-metadata.toml>

### SIBaud:chf-first-boot-seat-ignores-pull — CHAFE-WORDING, medium

- **Row:** "Works before ssh exists (the first-boot seat)" — **Ansible N**, pyinfra N,
  **cdist ~**, Dorc ~[^d3], cloud-init Y.
- **Chafe:** cdist earns `~` for PreOS, which *manufactures* the ssh precondition. Ansible
  has a first-party mode that does not need ssh at all: `ansible-pull` running locally on
  the box, wired up by cloud-init's own `cc_ansible` module, which installs Ansible and
  then runs it. That is at least as good an answer to "before ssh exists" as
  building a PXE image is. Its real cost — a Python-plus-Ansible payload on a machine that
  had nothing — is exactly the cost the *other* rows already charge for.
- **Proposed:** Ansible **~**, footnoted "`ansible-pull` under cloud-init's `cc_ansible`,
  at the price of installing the whole runtime first." Or, if the row means the *payload*
  seat specifically rather than the pull-inversion, say so in the row.
- **Counter-consideration, recorded so it is not re-litigated:** jpmens' point (in the
  ledger) that pull "invalidates the managed-nodes-need-nothing property" is real, but it
  argues about *cost*, not about whether the capability exists. Cost belongs in the
  footnote.
- **URL:** <https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html> ·
  <https://raw.githubusercontent.com/canonical/cloud-init/main/cloudinit/config/cc_ansible.py>

### SIBaud:chf-existing-scripts-measures-two-things — CHAFE-WORDING, medium; cuts both ways

- **Row:** "Existing scripts run unchanged (the adoption floor)" — **Ansible ~**,
  **pyinfra ~**, cdist N, **Dorc Y[^d4]**.
- **Chafe:** the row asks one question and the cells answer another. If "run unchanged"
  means *your bytes execute without modification*, Ansible's `script:` and pyinfra's
  `server.script` are both **Y** — they upload the file and run it, Ansible's without even
  needing Python on the target. If it means *and the tool then does something with them*,
  Dorc's own footnote `[^d4]` concedes "an unannotated script gains safety floor and a plan
  surface, nothing more", which is a `~`-shaped admission sitting under a Y. As written the
  row is scored on the second reading for the siblings and the first reading for Dorc.
- **Proposed:** keep Y/~ as they are but rename the row to what actually separates them —
  *"Your script is the unit of work, not an argument to theirs"* (Ansible **N**, pyinfra
  **N**, cdist **N**, Dorc **Y**) — or accept Ansible **Y** / pyinfra **Y** on the honest
  literal reading and let `[^d4]` carry Dorc's real differentiator. The first is stronger
  and truer.
- **URL:** <https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/script_module.html>
  · <https://docs.pyinfra.com/en/3.x/operations/server.html>

### SIBaud:chf-continuous-convergence-ignores-pull — CHAFE-WORDING, low

- **Row:** "Converges continuously with nobody present" — **Ansible N**, pyinfra N, cdist
  N, Kubernetes Y.
- **Chafe:** `ansible-pull` in cron, with `--sleep` for jitter and `--only-if-changed`, is
  a shipped first-party mode for exactly this. It is not a resident reconciler, which is
  what the row plainly wants; but no sibling has *any* mode for it and Ansible does.
- **Proposed:** either reword to *"A resident process converges continuously"* (Ansible
  cleanly N, row unaffected) or footnote Ansible's `~`. Low severity because the relative
  position does not move — but the reworded row is free and removes the objection.
- **URL:** <https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html>

### SIBaud:chf-queryable-provenance-needs-parity — CHAFE-DISCIPLINE, low, and under-evidenced

- **Row:** "Skips explained with queryable provenance" — **Ansible N**, pyinfra **N[^p1]**,
  cdist N.
- **Chafe:** pyinfra's N carries a footnote explaining what it *does* have. Ansible's bare
  N is the odd one out: a `when:`-skipped task records which conditional was false in
  structured task output (`UnifiedTaskResult.record_conditional_false`), which is at least
  as machine-readable as pyinfra's prose noop.
- **Honesty about the evidence:** I could not source this first-party. It is **not** in
  `common_return_values`, and `skip_reason` in current `devel` appears only on the `meta:`
  path. The one hard artefact I have is the executor call site. ~SUSPECT, not +SURE.
- **Recommendation:** leave Ansible at **N** — the row's word is *queryable*, and Ansible
  has no `why` verb any more than pyinfra does. Add a parity footnote only if it can be
  sourced properly; do not spend the calls for a footnote.
- **URL:** <https://raw.githubusercontent.com/ansible/ansible/v2.21.2/lib/ansible/executor/task_executor.py>
  (L401)

### SIBaud:chf-cdist-plan-artifact-exists-unsupported — CHAFE-WORDING, low

- **Row:** "Plan artifact runs without the tool installed" — **cdist N**.
- **Chafe:** N is right, but for a more interesting reason than "no". cdist *does* write
  the generated target sh to `code-remote` in the out-dir on every run; the material exists
  as plain sh on the controller's disk. What is missing is any supported export, and the
  fact that it is generated code rather than anybody's source. That is a better sentence
  than a bare N, and it is the nearest miss anyone in the table has to Dorc's cell.
- **Proposed:** N with footnote: "`code-remote` really does land on disk as plain sh —
  generated, undisplayed, and with no supported way to take it anywhere."
- **URL:** <https://raw.githubusercontent.com/ungleich/cdist/c1633d9301c053b604cceb39b57621636e80b9cd/cdist/config.py>

### SIBaud:chf-cdist-offramp-half-true — CHAFE-WORDING, low

- **Row:** "Off-ramp: stop using it, keep working artifacts" — **cdist N**.
- **Chafe:** turn 04's own honest half: cdist *types* (explorers and `gencode-remote`) are
  portable sh a human could lift by hand; only the manifest is welded to the emulator. The
  bare N slightly overstates against the one sibling that came closest to our bet.
- **Proposed:** N with footnote, or leave it — the manifest is the entry point and without
  it nothing runs, so N is defensible. Flagged for completeness because the cdist prose
  section already calls it "the closest kin to our oracle idea", and a reader who believes
  that will ask.

### SIBaud:chf-cdist-creates-infrastructure-vs-install — CHAFE-WORDING, low

- **Row:** "Creates infrastructure (VMs, DNS, networks)" — **cdist N**.
- **Chafe:** cdist's `install` mode partitions disks, makes filesystems, writes bootloaders
  and reboots — roughly fifteen `__install_*` types — and PreOS PXE-boots a machine that
  did not exist. A reader who has just read this table's own admiring PreOS paragraph will
  notice the tension with a flat N on "creates infrastructure".
- **Proposed:** the parenthetical is doing the work; make it do it explicitly — *"Creates
  cloud/provider resources (VMs, DNS, networks)"*, under which cdist is cleanly N and
  nobody trips.
- **URL:** <https://www.cdi.st/manual/latest/cdist-preos.html>

### Cells I tested as an advocate and could not move

Recorded so the conductor does not re-run them, and so the exclusion-check is visible:

- **pyinfra "Works before ssh exists" N** — `@docker`/`@chroot` connectors give an
  image-time seat, but there is no boot channel and no first-boot story. N stands.
- **cdist "Existing scripts run unchanged" N** — checked the *current* release, not the
  mirror: 173 types at 7.0.0, no `__script`/`__shell`/`__exec`/`__command`. The
  `--onchange` hooks on `__config_file`/`__staged_file` are hooks, not a unit of work. N
  stands, now on hard evidence.
- **pyinfra "lock-step … forces its two-phase machinery"** — I expected this to be an
  inverted causal claim and it is not; the 3.x docs state it outright. Claim stands
  unaltered (`SIBaud:cit-pyinfra-lockstep-is-why-prepare`).
- **cdist "Templating ~"** — no engine, but `__file`/`__block`/`__line`/`__key_value`/
  `__config_file` plus the documented heredoc practice make `~` exactly right.
- **cdist "Secrets N", "Whole-system rollback N", "Check-then-converge Y"**, **pyinfra
  "Privilege escalation Y", "Fleet Y", "Templating Y"**, **Ansible "Secrets Y", "Fleet Y"**
  — all obvious and correct; no footnote owed.

---

## 3. Out-of-lane flags (free; no research owed, low confidence by construction)

- **Dorc, "Preview before mutating" Y** — the only Y in the table belonging to a tool that
  does not exist yet. Every other Y in every column is shipping software. Not a
  correctness claim, a *reader-trust* one: the banner says "human-reviewed in place" but
  not that one column is prospective. One line in the banner would cost nothing and buy a
  lot. (The `NYI` marks the human has since added to two Dorc cells suggest this is
  already being addressed; if so, ignore.)
- **Terraform, "Re-measures the live system every run" ~** — `terraform plan` refreshes
  state by reading every resource from the provider API every run. The `~` presumably
  means "reads the provider's model, not the machine's interior", which is exactly the
  reword the conductor floated in the mid-audit note; under the current wording it looks
  wrong, and under the new wording it is right. Supports making that reword.
- **nix/NixOS, "Preview before mutating" ~** — consistent with the prose ("their
  `dry-activate` self-documents as incomplete"), but `nix build` + store-path comparison is
  a complete and sound preview of *what will be installed*; only *activation* is
  incompletely previewed. The `~` may be scoring the weaker of two different previews.
- **cloud-init, "Templating / config-file generation ~"** — `write_files` plus jinja
  templating over instance-data is a real config-generation surface; `~` looks right but
  the neighbouring `N`s on that column make it read as grudging.
- **Kubernetes, "Creates infrastructure ~"** — the `~` is presumably Cluster API /
  cloud-controller. If so it is doing a lot of unexplained work next to a prose section
  that says flatly "it does not manufacture nodes." One of the two should soften.

---

## 4. Answers to the conductor's mid-audit note

### SIBaud:ans-byte-identity-row-split — the byte-for-byte row splitting in two

All three siblings are **N under both candidate rows**. Per tool, one line each:

- **Ansible — N / N.** Row A: what you approve is YAML; what executes is a Python module
  file transferred to the target and run by the remote interpreter — the SSM plugin's
  requirements page states this outright even for `shell`, since "Ansible sends over the
  `.py` files of the module itself"
  (<https://docs.ansible.com/projects/ansible/latest/collections/amazon/aws/aws_ssm_connection.html>).
  Row B: doubly N — the language you wrote is YAML, and even a `shell:` string is
  Jinja-templated before dispatch, so the literal text you read is not necessarily the
  text that runs.
- **pyinfra — N / N.** Row A is structurally impossible for it: commands "aren't generated
  until execute time", so at review time there is no text to be identical *to* — the plan
  is a change matrix over operation names
  (<https://docs.pyinfra.com/en/3.x/using-operations.html>). Row B: the language is Python
  and the executed material is generated shell. The two rows collapse to the same N here,
  which is worth knowing: **pyinfra is the tool for which the split changes nothing.**
- **cdist — N / N, and this is where the split earns its keep.** Row A: the executed
  artefact is `code-remote`, generated by a controller-side `gencode` script and never
  displayed. Row B: cdist is the *nearest miss in the table* — the user writes sh and the
  emitted code is also sh — but it is still generated sh, resolved through PATH-planted
  emulator symlinks into a Python emulator
  (<https://www.cdi.st/manual/latest/cdist-type.html>). Under one combined row cdist and
  pyinfra look identical; under Row B cdist is a near-miss N and pyinfra a structural one.
  Recommend the cdist Row B cell carry the link and one clause, since that is the only
  place in the table where the split is visibly load-bearing.

Only cdist's Row B needs a link in my view; the Ansible and pyinfra cells are obvious once
the rows are worded.

### SIBaud:ans-remeasure-inside-machine — the "re-measures INSIDE the managed machine" reword

**No disagreement. All three are genuine Y under either wording**, and the reword makes the
column read *better*, not worse, because each measurement is unambiguously target-side:

- **Ansible Y** — `gather_facts` runs the `setup` module on the target, and each module
  re-checks its own state there on every run.
- **pyinfra Y** — 143 fact classes, each a shell command executed on the host and parsed
  controller-side, collected fresh every run
  (<https://docs.pyinfra.com/en/3.x/using-operations.html>).
- **cdist Y** — explorers are "small shell scripts, which will be executed on the target
  host", all of them in stage 1 and per-object again in stage 3
  (<https://www.cdi.st/manual/latest/cdist-explorer.html>).

One caveat that does not change any cell: pyinfra caches facts per run and cdist re-explores
per object, so "every run" is the right granularity for all three, and "every decision" is
not — for anybody, including Dorc.
