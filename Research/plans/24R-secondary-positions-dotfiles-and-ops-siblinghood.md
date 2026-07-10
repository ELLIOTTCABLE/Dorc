# 24R — Dorc's secondary positions: dotfiles/local-machine + happy-siblinghood with ops tooling

AI-authored (Fable, research/planning session), 2026-07-10. Synthesis of a three-round
prior-art/design-fit investigation into Dorc's *secondary* positions — everything besides
the titular Hard Problem of being the core orchestrator responsible for everything. Half 1:
the dotfiles / dev-environment / local-machine repurposing. Half 2: cooperation with the
professional ops stack (the DESIGN "not the only toy" postures: happy parent / happy child /
happy sibling). Evidence base: `.claude/research/other-purposes-and-cheap-wins/` (three
turn-note files; findings `repurp-finding1`–`91`; 30 graded sources in its `sources.json`,
subagent-gathered with conductor spot-verification where marked). Plan-tier per the
Research/README convention: a durable synthesis, lightly kept-current — annotate
corrections in place. Direction-setting research, not build direction; nothing here jumps
the r24 queue.

Reading rule, per the human's framing ruling (2026-07-10, this session): **impossibilities
frontloaded.** The design-job in every position below is to shift/shove/comb the
impossibilities into manageable UI/UX — never to claim they are fixed. §0 is the ledger;
if a later section seems to contradict §0, §0 wins and the section is mis-written.

---

## §0 — The impossibility ledger (read first, cite often)

### §0a — the why-run ledger (the sharpest one; human-challenged and re-derived this session)

Chef's first-party "why-run considered harmful" post [A-chef-whyrun-harmful-2018] is the
canonical statement that dry-run-over-shell fails in battle. It is ALREADY in-corpus:
`plans/102` E3 (r10) cites it by this same slug and rules the consequences. Decomposed, with
each claim's disposition for Dorc:

- **whyrun1 — dry-run is not side-effect-free.** Guards/probes execute real code; a
  *logically* read-only interrogation can *operationally* wedge a fragile host (Chef's
  nightly-why-run-broke-production systemd story). Disposition: **conceded permanently**
  (`plans/102` E3-T/E3-D: transfer-to-author + mitigate + accept, NEVER eliminate; "never
  market the stronger claim"). Dorc's deltas are real but bounded: probes are *selected*
  (only vouched checks ship — kFAIL-withhold) and *attributed* (a named author's contract),
  and — the one mechanism-difference — probe bodies are in the analyzable dialect, so
  static effect-checking of probe bodies (owed) and the `plans/077` runtime observe
  backstop (reserved) can machine-assist the contract. None of that makes reads harmless:
  our own corpus history contains a shipped probe-mutation by a *paranoid* author
  (`notes/151` X4, the `apt-get -o` case), and Dorc's parallel probe fan-out plausibly
  *aggravates* the wedge-a-fragile-host class relative to Chef's serial why-run.
  Corollary: read-only ≠ non-blocking ≠ side-effect-free, in every doc and pitch, forever.
- **whyrun2 — forecasts across mutations are wrong in principle** (the frame/staleness
  problem; Chef generalized it to Puppet and Ansible in the same post). Disposition:
  **dissolved within a run, by refusal** — Dorc does not forecast across a mutation it
  will perform; below a running wall, facts demote to in-sequence guards (staleness-free
  *by construction*: they re-check after the upstream mutation really happened) or plain
  runs. The r23 ternary-verdict redesign (`plans/233`→`239`) is this refusal, arrived at
  independently when the same problem hit from inside. Residual, accepted-with-receipt:
  the elide tier forecasts across **time** (probe at T0, apply at T1; external mutation in
  the window under-executes; open-world drift machinery is wontfix by standing ruling) —
  window is user-controlled, next plan re-measures, and the r25 fires-often × bites-rarely
  question (`plans/252` B4) is the empirical check. The survival tier crosses mutation
  boundaries only under double-opt-in with the receipt printed (USER_STORY, the bought
  unsoundness).
- **whyrun3 — incomplete simulation misleads worse than nothing.** Disposition: **failure
  direction inverted by construction.** Why-run's incompleteness produced false forecasts
  presented as truth; Dorc's incompleteness (unmodeled commands, cmdsubs, sourced files)
  produces walls ⇒ conservatism: lines stay in the plan, guarded or running; can't-say
  never rounds to converged. The residual false-confidence channels are exactly the priced
  ones: a wrong vouch (bites its own author's line), a wrong footprint (the eight-condition
  survival corner), and whyrun2's time window.
- **whyrun4 — "test in a sandbox instead."** Complementary, not contradictory: the
  kVERIFY-calibrate posture and the oracle-author container/CI story are the same answer.

**The defensible comparative claim** (and the ceiling on every "dry-run" sentence in this
note): incumbents' check-modes are *structurally unable* to be both safe and informative on
shell content — Ansible `--check` silently skips shell tasks; Puppet `--noop` and Salt
`test=True` execute guards for real at agent privilege; Chef disowned why-run
[A-ansible-checkmode-2026] [A-chef-whyrun-harmful-2018] (`repurp-finding74` as corrected).
Dorc's architecture is *organized around* that impossibility — refusal, guards, vouches,
attribution — rather than denying it. Shortest honest form: **why-run made a promise nobody
can keep; Dorc sells the bookkeeping of exactly who is promising what, and runs the rest.**

One clean carve-out: the **static-only preview** (analysis with NO probes shipped — the
lint/CI/pre-commit mode) is the single mode that is genuinely side-effect-free, because it
never touches a host. Its price is emptiness: structure only (walls, hazards, idioms),
no world-facts. Everything with probes inherits whyrun1.

Meta-warning for future rounds (logged in the turn-3 corrections): a
pains-and-empty-slots gathering method structurally produces confirmation-shaped output.
The evidence below survived quote-verification; the *leaps* must be checked against this
ledger and the corpus impossibility record (`102`, `233`, `238`) before any becomes a
selling-point. That check failed once this session (the retracted "a dry-run you can
trust" phrasing) and was caught by the human, not the process.

### §0b — scope impossibilities (welded or structural; priced, not relitigated)

- **Native-Windows config management**: out (kLANG weld). Softener from evidence: nobody
  solves it well — chezmoi is best-in-class and still second-class there (symlink
  privilege/DevMode, junction workarounds) [A-gitforwindows-symlinks-2024]
  (`repurp-finding9`/`21`); the WSL-vs-native two-worlds problem is unsolved by every tool.
- **Kubernetes app delivery (GitOps core) + immutable cattle-core**: out. The residue is
  *conceded* territory, not contested — Fowler's own canon concedes config management
  persists on a reduced surface [A-fowler-immutable-server-2013]; 2026 practitioners name
  the surviving layers (image build; k8s node bootstrap; edge/on-prem where "mutable
  infrastructure is the only economically viable model"; the emergency-mutation gap)
  [B-opsio-converge-residue-2026] (`repurp-finding83`/`85`).
- **home-manager's activation slot**: unreachable by the shebang vector (blocks are spliced
  into one generated bash script; no per-block file) (`repurp-finding58`/`60`).
- **The bash/zsh artifact gap**: the famous dotfiles installers are bash/zsh, not POSIX sh
  (`repurp-finding16`); the long-haul portability practitioners write POSIX deliberately
  (`repurp-finding38`). Bears on kWHICHSH's *scope*; open, human-owned; not settled here.
- **sudo/`su -c`/source opacity**: the wrapper-context seam is reserved
  (deferred-surface: 23J lane-privilege, 17N §7) but unbuilt; personal machines sudo
  per-line and ops `become` is pervasive, so both halves of this report raise its weight
  (`repurp-finding5`/`18`; ops confirmation turn-3). The single heaviest capability gap for
  every secondary position.
- **Per-iteration loop verdicts**: deferred by the atomic-command axiom (`23D` §3 —
  "figure out loops eventually"); the dotfiles symlink-farm loop is the *starting* shape
  of that cohort's books, so the deferral is benign for ops and load-bearing here
  (`repurp-finding26`).
- **Secrets**: quarantined from this session's tier by standing order; slots and market
  pressure NOTED only (chezmoi's #1 graduation wedge; `repurp-finding20`/`32`; the
  plan-display-vs-secret-hygiene tension `repurp-finding12` awaits its own round).

---

## §1 — Half 1: the dotfiles / local-machine position

### §1a — demand side (evidence-backed)

Graduation triggers away from plain scripts: {second OS family, work/personal secrets
split, public-repo desire, per-host content divergence} [B-chezmoi-why-use-2026]
(`repurp-finding22`). The community wish-list maps near-1:1 onto existing Dorc properties
— "declarative/per-machine power WITHOUT an evaluation/rebuild step in the edit loop" (the
#1 home-manager pain [B-jade-use-nix-less-2025]); standalone/single-user/rootless; one tool
spanning files+packages+settings+commands without a second bootstrap dependency; config
files that stay real files [C-hn-dotfiles-megathread-2026] (`repurp-finding37`/`41`).
Re-run rationing by state-memory (chezmoi `run_once_`/`run_onchange_` content hashes;
devcontainer create-once lifecycle; cloud-init semaphores) is a cross-ecosystem confusion
class that reality-probing dissolves (`repurp-finding30`/`55`/`62`/`87`). Genuine drift
*reporting* is near-absent across the space — apply-and-hope everywhere
(`repurp-finding27`).

### §1b — fit audit

No weld is hostile to this cohort; two actively favor it: marker-gating (an unmarked book
is plain sh — existing POSIX scripts are day-zero inputs, `repurp-finding7`, tempered by
the bash gap §0b) and statelessness (no ownership fights — the exact home-manager failure
mode users flee, `repurp-finding30`/`40`; no second state DB beside the host tool's,
`repurp-finding53`). The exposures are priority-inversions, not bakes: the read-value/
cmdsub lane (`case "$(hostname|uname)"` is THE per-machine-variance idiom and walls whole
books at HEAD — existential here where it was one bad line in ops; `repurp-finding3`/`4`);
loops (§0b); the stdlib slice composition (this cohort writes zero oracles — value =
f(stdlib): ln/cp/mkdir/git/curl/`defaults`/brew/`command -v`; `repurp-finding6`/`15`/`24`);
and local-exec as a blessed product mode (`repurp-finding2` — the spike is de-facto
local in e2e; the design never names localhost as a target class).

### §1c — cooperation surfaces (the round-2 postures; USER_STORY "Other usage-patterns" holds the two worked stories)

- **Happy child** (Dorc runs the scripts a host tool owns): the shebang vector is the
  minimal-glue slot — chezmoi direct-execs scripts (shebang honored; `interpreters.sh`
  remap = whole-corpus variant) [A-chezmoi-interpreters-2026]; yadm bootstrap/hooks, mise
  file-tasks, git hooks likewise (`repurp-finding52`/`58`). The niche is *durable* because
  it is ceded doctrinally: four tools mandate script idempotence verbatim and none assist
  it (chezmoi "scripts break chezmoi's declarative approach… should be idempotent"
  [A-chezmoi-scripts-doc-2026]; yadm [A-yadm-bootstrap-2026]; dotbot [A-dotbot-readme-2026];
  home-manager's writeBoundary/DRY_RUN protocol is a hand-written probe/apply split
  demanded of authors [A-homemanager-activation-2026]) (`repurp-finding54`/`59`/`60`).
  The requirements contract (glue-pain minimization): local-exec (R1); byte-faithful
  stdout/stderr/rc/cwd with ALL Dorc signalling out-of-band (R2 — hosts key on the shallow
  rc contract and punish chatty stdout, `repurp-finding63`/`67`); headless consent with the
  attention product relocated to the after-the-fact why-artifact (R3 — consonant with the
  attention-chronology doctrine, `23D` §4); analysis latency in the interactive hot path
  (R4); env-var facts (R5, rides the read-value lane); rendered-input tolerance (R6);
  no second state store (R7). The `dorc-sh` naming seam: strip-and-exec semantics stay
  forever; an analyzed runner is a DIFFERENT token (`repurp-finding44`/`50`; flagged as a
  respell-brief rider in-session).
- **Happy parent** (a book orchestrates the incumbents): the stack-glue layer (bootstrap
  ordering across repo+Brewfile+defaults) is nobody's product; delegation oracles ride the
  incumbents' own check verbs (`chezmoi verify` rc 0/1; `brew bundle check || brew bundle
  install` is Homebrew's first-party-blessed idiom and already the guard shape Half-B
  lifts [A-brew-bundle-docs-2026]) (`repurp-finding49`/`61`). Judgment stays the author's:
  verify licenses `apply` but declines `update` (remote-truth); Brewfile-is-Ruby means
  `bundle check` can execute `system` guards — a delegation vouch inherits the verb's
  honesty (`repurp-finding61`, whyrun1 applies).
- **Happy sibling**: statelessness = nothing to go stale when the host tool mutates the
  world between runs (`repurp-finding30`/`53`).
- **Zero-runtime**: the preview-analyzer slot is EMPTY — no tool statically previews a
  setup script's effects; the state of the art is "read it first"
  [C-so-no-shell-dryrun-2014] (`repurp-finding64`); the pre-commit/CI distribution contract
  is trivially satisfiable (filename argv + nonzero exit) [A-precommit-docs-2026]
  (`repurp-finding65`). This is the one genuinely-dry mode (§0a carve-out) and the
  lowest-commitment on-ramp: value before anyone lets Dorc execute anything.

### §1d — dotfiles cheap-adds (ranked; sequencing respects the live queue)

1. Read-value oracle family + cmdsub folding (hostname/uname/`command -v`/version-reads) —
   P5 rider already owed; this cohort upgrades it to existential.
2. Dotfiles stdlib slice (ln/cp/mkdir/git/`defaults`/brew/`command -v`); brew and
   `defaults` are the big attention/wallclock wins (`repurp-finding15`/`24`).
3. Local-exec blessing (now triple-load-bearing: here, ops pull-security §2c, edge/residue).
4. Lints: book-reads-stdin (interactive prompts hang embedded; they sit in the CORE loop of
   the most-copied installer [A-holman-dotfiles-bootstrap-2026], `repurp-finding17`);
   sudo-keepalive-daemon pattern (`repurp-finding18`). kWARN-rich sanctioned.
5. Previewer/pre-commit packaging (static-only mode).
6. Positioning, zero build: plan-as-drift-report; the fresh-machine story (book runs bare
   under sh before Dorc exists — beats even chezmoi's single-binary wedge,
   `repurp-finding34`).

---

## §2 — Half 2: happy-siblinghood with ops tooling

### §2a — demand side: the manual-annotation universe

Every CM tool's shell escape-hatch defaults to always-changed and demands hand-written
convergence annotations — Ansible `creates:`/`removes:`/`changed_when` plus the lint trio
whose rationale is verbatim "shell and command modules… always report changed"
[A-ansiblelint-no-changed-when-2026]; Puppet `unless`/`onlyif`/`creates` (with the
documented creates-staleness trap); Chef `not_if`/`only_if` (guard-interpreter traps);
Salt `onlyif`/`unless`/`creates` (`repurp-finding73`). Those annotations ARE hand-written
oracles, one site at a time; the migration story maps them near-1:1 onto lifted guards and
verdict functions. Terraform extends the doctrinal-cession pattern to IaC ("Terraform
cannot predictably model provisioner behaviors… exhaust all alternatives"
[A-terraform-provisioners-2026], `repurp-finding88`). Salt got closest to an oracle
contract (`stateful` + `test_name`: an author-supplied check-mode command,
`repurp-finding76`) — prior-art convergence for the role-sibling family, as is systemd's
`ExecCondition` native three-way rc (0 proceed / 1–254 skip-cleanly / 255 fail
[A-systemd-service-man-2026], `repurp-finding89`) and pyinfra's two-phase model with its
immutable-facts contract and execute-time `_if` callables — an incumbent hand-authoring
the guard discipline our analyzer derives [A-pyinfra-deploy-process-2026]
(`repurp-finding69`).

> **ack-ansible-architecture (annotated 2026-07-10, same session — human challenge #2,
> upheld):** the architectural relationship to Ansible must be stated before the gaps
> below, or they read as overblown. The guard tier — check-then-converge, fused
> in-sequence at act-time — is architecturally IDENTICAL to what a good Ansible module
> does internally (and Puppet/Chef resources before it); it is the shared *sound* design,
> with no staleness gap, and Dorc does not improve on a good module at its own game. The
> oracle library is the module-library problem re-run (DESIGN component-4 already concedes
> this). What is genuinely not spelling: the compile step and its products — the phase
> split with a plan ARTIFACT and consent; the elide tier (Ansible's apply never gets
> shorter: every task ships and self-checks on every run — the founding 45-minute pain);
> whole-book static claims (walls, dead branches, survival) a runtime-interpreted task
> list structurally cannot make. And the honesty coupling: the SIMILARITY is where the
> soundness lives; every difference is bought from the disclosed trust budget (§0a).
> Where oracles are thin or walls dominate, Dorc degrades to Ansible-in-sh — the
> spelling-advantage-only regime, and the realistic early-adoption regime. Formulation of
> record: **Dorc is Ansible's architecture wherever that architecture is sound, plus a
> compiler wherever it isn't — and the compiler's products are bought, with receipts.**
> Check-mode scope note, source-verified (ansible script.py action plugin): shell/script
> tasks under `--check` are skipped outright OR judged by the `creates:`/`removes:`
> annotation alone — never by observing the task; module-covered content in check mode is
> decent, and §2b's gap-checkmode claims are SHELL-SCOPED only.

### §2b — the three market gaps (stated under §0a discipline)

- **gap-checkmode**: the tri-modal incumbent dry-run breakage (§0a's comparative claim),
  SHELL-SCOPED per ack-ansible-architecture above. What Dorc sells here is the
  *bookkeeping* — refusal+guards+attribution — never "a dry-run you can trust"
  unqualified.
- **gap-inhost-drift**: terraform-plan/driftctl drift detection compares cloud-API state vs
  tf-state; nothing sees inside the host (`repurp-finding86`). A scheduled `dorc plan`
  with a `-detailed-exitcode`-shaped rc [A-terraform-plan-cli-2026] fills an empty slot —
  **and inherits whyrun1 in exactly Chef's nightly-cron incident shape**: viable only with
  timeouts/rlimits, probe cost-classing, and fragile-host opt-outs carried loudly.
- **gap-assert-fix**: the verification harnesses are read-only BY DESIGN (goss refuses to
  auto-capture `command` asserts "for safety" [A-goss-cli-2026]; InSpec detects, Chef Infra
  remediates, coupling is external glue) (`repurp-finding68`). Our guard
  (`( check ) || fix`) is natively the coupling; the safety objection that keeps them
  read-only is *answered by* vouch+attribution, not eliminated by it.

### §2c — postures with mechanisms

- **Happy child**: Ansible `script` pushes a file and executes via the remote shell without
  Python (shebang honored; caveat: `-tt` merges stderr into stdout — embedded-contract
  hazard); Terraform `remote-exec` uploads-and-runs (taint-on-failure context);
  cloud-init `runcmd` invoking a shebang'd script (rc≠0 fails provisioning — rc fidelity
  matters); systemd `ExecStart` at a script; CI committed-scripts, and GitHub Actions'
  `shell: dorc-run {0}` routes every run-block through us with zero platform cooperation
  (`repurp-finding77`/`80`/`87`). Optional per-host-tool changed-report adapters (Salt's
  `stateful` stdout protocol is a blessed channel; the stdout-purity tension is resolved by
  the host tool *defining* the convention — `repurp-finding78`).
- **Happy parent**: delegation oracles — `terraform plan -detailed-exitcode` is a native
  ternary (0 converged / 2 diverged / 1 can't-say) mapping directly onto verdict senses;
  `ansible-playbook --check` is the anti-specimen (its own community doesn't gate on it —
  decline or scope narrowly, `repurp-finding79`). Inventory: consume the blessed
  executable-inventory protocol (`--list`/`--host`/`_meta.hostvars`
  [A-ansible-inventory-devguide-2026]) — mechanism for `064`'s consume-don't-reinvent
  ruling; the `ansible-inventory` CLI dump carries a stability asterisk
  (`repurp-finding70`).
- **Happy sibling**: statelessness dodges the Terraform partial-apply/stuck-lock pain class
  wholesale (no state file, no lock, reality re-probed; `repurp-finding82`); structured-
  output seams exist in all four CM tools if we ever consume THEIR runs
  (`repurp-finding77`).
- **Zero-runtime**: plan-as-PR-artifact works for Terraform because the plan is
  deterministic and applied verbatim; ours is nearly free (the rendered plan IS an
  executable script; deterministic render is a DST property) with the whyrun2 time-window
  disclosed (`repurp-finding79`).

### §2d — ops cheap-adds (ranked)

1. `dorc plan --exit-code` (0/1/2; STRAWMAN name) — CI gating + gap-inhost-drift, with the
   §2b caveats and the wrapper-swallows-exit-code failure mode documented
   (`repurp-finding86`).
2. Ops delegation oracles in P5: terraform-plan (the ternary), goss-validate, git,
   systemctl; an explicitly-declining ansible-check entry as the teaching counter-example.
3. Executable-inventory consumption.
4. Reviewed-plan-artifact flow blessing (plan-is-a-script).
5. Changed-report adapters (Salt first; post-respell; small).
6. Docs-tier CI recipes (`shell: dorc-run {0}`; committed-script embedding; the
   static-binary story vs "exit 127: ansible-playbook not found" runner pains,
   `repurp-finding82`).

---

## §3 — Consolidated human-flags (decisions this document deliberately does NOT make)

- flag-dorc-sh-seam: pin in the respell brief that `dorc-sh` stays strip-and-exec forever;
  the analyzed runner is a different token (one sentence, time-sensitive; raised in-chat).
- flag-kwhichsh-scope: the bash/zsh artifact evidence (§0b) bears on kWHICHSH's *scope*;
  human-owned, deliberately not argued here.
- flag-sudo-weight: both halves independently promote wrapper-opacity to the #1 capability
  gap; the reserved seam (23J/17N §7) is where the eventual design lands.
- flag-secrets: quarantined; the market pressure is recorded (§0b) for whenever that round
  runs.
- flag-embedded-transparency: R2/R3 deserve a named home as design rules (cheap now,
  retrofit-painful; the TODO-tier "embedded-transparency hygiene" item is the same ask).
- flag-marketing-fence: any future pitch touching dry-run/plan-safety must pass §0a; the
  102 fence ("never market the stronger claim") is standing and was nearly breached once,
  in the session that produced this document.

## §4 — Evidence pointers

Durable base: `.claude/research/other-purposes-and-cheap-wins/` — `turn01`/`turn02`
(dotfiles + cooperation rounds), `turn03` (ops round + the WRCH corrections block),
`sources.json` + `sources/` (30 graded archived sources; bracketed slugs in this document
resolve there). The worked user-stories live in USER_STORY "Other usage-patterns"
(dotfiles pair landed 2026-07-10, pending human audit; the ops heading remains TBD and is
fillable from §2). Findings are cited sparsely above by design; the turn notes carry the
full 91-finding record with per-finding certainty markers.
