# Corpus spike — session progress: instrument built + first real tally

> **Status (2026-06-01): INTERIM, not the final go/no-go.** The measurement instrument is
> built, typed, and validated on real shell + Ansible; a representative public sample is
> acquired out-of-tree; a first raw tally is done. The §3 primary answer (`Q-BAND`/
> `Q-ANTICORR` band sizes) is **deliberately not yet computed** — the first tally exposed a
> selection-bias problem and the apply-cost/check-depth classification rules are a judgment
> call I've parked for the user (§7). Confidence markers throughout.

## TL;DR
- +SURE the instrument works and scales: 10,350 files scanned in 3.7s, streaming, 0 crashes.
- +SURE parse-feasibility is strong on this sample: **0.0% hard parse-failure, 1.5% ERROR-node** (shell); 0.17% YAML parse-failure (Ansible).
- +SURE the ⊤-bound seed is low: shell **eval 0.1%, dynamic command-name 1.4%** (`Q-UNANALYZABLE`).
- ~SUSPECT a genuine early signal: explicit Ansible idempotency guards (`creates:`/`removes:`) are **rare (~0.1%)**; idempotency is overwhelmingly *module-native* → tends to support "the oracle library is load-bearing" (the `kDEPS` split leaning oracle-heavy), but this is pre-bias-correction so hold it loosely.
- +SURE the raw tally was **contaminated by collection test-code** (~95% of raw Ansible tasks sat in test paths; `assert` ≈ 24%). Scanner **v2** de-biases + stratifies (§4b): real workload = 362 mutating Ansible tasks, ~40% pre-guarded, but that varies **7–55% by source stratum** (Simpson's paradox, live).
- −GUESS the headline go/no-go (`Q-BAND` VALUE-band size) is **still open** — it needs the apply-cost×check-depth thresholds (§7, the user's call) plus a larger sample for tight error bars.

## 1. The instrument (kept in-tree: `tools/corpus/`)
A TypeScript scanner; `acquire` (network, gated) kept separate from `scan` (offline, pure).
- **Toolchain** (`mise.toml` + `tools/corpus/package.json`, lockfiled): node 22 LTS via mise;
  deps `web-tree-sitter@0.25.10` + `yaml@^2.9.0`. Pure-JS/wasm only (no native bindings) so the
  committed lockfile survives the Win/WSL mise split.
- **Grammar**: vendored `tools/corpus/grammars/tree-sitter-bash.wasm` (see its README) — committed,
  not installed, so the scanner runs on any platform with just node.
- **`src/scan.ts`** (v1): walks roots; shell→tree-sitter-bash (command extraction, parse-health,
  eval/dynamic flags); Ansible→`yaml` (module + idempotency-key extraction). Defensive
  (skip-and-log, never crash on one bad file), streaming (holds one AST at a time; the tally is
  bounded by *distinct* commands, not file count — the `Q-WORKINGSET` answer in miniature).
- **Run**: `cd tools/corpus && npm install && npm run scan -- <corpus-root>` (uses the mise node;
  see §2 for the Win-side `mise exec` caveat).

## 2. Toolchain/parser saga + resolution (recorded so it isn't re-derived — it cost hours)
- This box is **Windows MINGW64** (not WSL). OCaml/`shstats` was rejected: GPL-3 (would contaminate
  the kept instrument), shell-only (no Ansible YAML), strict-POSIX (not error-tolerant), and
  opam/dune absent on MINGW64. TypeScript + tree-sitter chosen instead (user-confirmed).
- **`mise exec` is blocked here**: the user's *global* `~/.config/mise/config.toml` pins a Linux/WSL
  toolset (`opam`/`lua`/`postgres`/`racket`) with no Windows build, so any `mise install`/`mise exec`
  tries (and fails) to provision them. Workaround in use: invoke the mise-installed node binary
  directly (`~/AppData/Local/mise/installs/node/22.22.3/node`). The `node@22` pin itself installs fine.
- **Native `tree-sitter` bindings**: unavailable — no Windows/node-22 prebuild, no MSVC to compile.
- **The wasm grammar trap** (the real time-sink): `tree-sitter-wasms@0.1.13`'s bash grammar is
  ABI-incompatible with *every* published web-tree-sitter — it *loads* under 0.25.x but `parse()`
  throws `resolved is not a function` (an unresolved wasm dylink stub) the instant real shell
  exercises the affected rule; 0.24/0.26 fail even to load. Misleading because trivial inputs
  (`echo hi`) don't hit the stub, and an early test bug (`argv[1]` = the script's own path) made it
  look like it worked. **Fix**: use the grammar bundled inside the `tree-sitter-bash@0.25.1` npm
  package, vendored into the repo and loaded from bytes. Verified: `nvm.sh` (156 KB) → 36,298 nodes,
  1,821 commands, `err=false`.

## 3. Acquisition (the sample) — `~/.cache/dorc-research/corpus/` (out-of-tree)
Rate-safe per the user's hard constraint (no IP ban): `gh api rate_limit` preflight, serial
`git clone --depth 1`, 2s spacing, SHA-pinned to `resolved.lock`. **Contrast-not-compound**: public
world-representative repos, not the user's own. (Pinned source catalogue: `tools/corpus/manifest.toml`.)
- **Ansible (19 repos)**: 15 geerlingguy canonical roles (real provisioning) + 3 top collections
  (`community.general`, `ansible.posix`, `community.docker`) for breadth.
- **Homelab/gitops (8 repos)**: GitHub `topic:home-ops`, top by stars (the realistic ops demographic).
- **Shell (1)**: `nvm` (a large real shell codebase — used mostly to stress the parser).
- Total 73 MB, 26 repos. **Not** committed (out-of-tree); `resolved.lock` makes it re-acquirable.
- Big academic corpora deferred (not yet needed for a go/no-go; see §7): bash-in-the-wild (13.9 GB,
  Zenodo 5732299), GLITCH (1.7 GB Docker image, 196k labeled IaC scripts), PIPr (197 GB — excluded;
  its `repos_list.txt` manifest is the useful part).

## 4. First raw tally — exact numbers + method (provenance)
Command: `scan ~/.cache/dorc-research/corpus`. Files seen 10,350 (1 skipped >1 MB). Rules: a "shell
file" = extension `.sh/.bash/.ksh/.dash`; an "Ansible task" = a list item under a tasks/handlers key
or a top-level task list whose keys include a reserved task key or a dotted module name; "command" =
a tree-sitter-bash `command` node's `name` field (path-stripped); "dynamic" = a name containing
`$`/backtick/`(`.

**Shell** — 194 files, 4,691 commands, 386 distinct:
- parse threw (hard fail): **0 (0.0%)**; parsed-with-ERROR-node: **3 (1.5%)**.
- eval: 4 (**0.1%** of commands); dynamic command-name: 66 (**1.4%**).
- top commands (biased — see §5): echo 437, nvm_echo 356, return 313, command 242, nvm_err 208,
  set 180, kubectl 136, exit 128, read 97, printf 74, ansible-test 70, … git 36, docker 33, sed 31.

**Ansible** — 6,418 YAML files, 11 parse-failures (**0.17%**), 1,125 task-files, **10,429 tasks**, 384 distinct modules:
- top modules (biased): assert 1758, ansible.builtin.assert 724, community.docker.docker_container 597,
  …docker_swarm_service 395, debug 241, set_fact 238, command 185, copy 179, debug 172, file 158,
  include_tasks 140, …docker_container_copy_into 126, shell 120, command 119, file 109, …
- check/idempotency keys: **when 994, check_mode 560, changed_when 111, failed_when 65, creates 8, removes 2.**

## 4b. De-biased + stratified tally (scanner **v2** — supersedes §4's raw numbers as the headline)
Same sample, scanner v2: excludes test/scaffolding paths (`tests?/`, `molecule/`, `.github/`, …),
normalizes module FQCNs (`ansible.builtin.apt`→`apt`), splits mutating-vs-control, stratifies by
source-type. Scaffolding exclusion alone cut Ansible "tasks" **10,429 → 558** — ~95% of raw tasks were
collection *test-code* (the `assert` spike). De-biased:

**Shell (workload, 101 files):** parse-threw **0.0%**, ERROR-node **1.0%**; commands 3,725; eval 0.1%,
dynamic 1.3%. ("mutating" 59% is ⚠ overcounted — nvm's own function *calls* aren't in the non-mutating
set; shell mutating-classification stays crude until guard/CFG work, and the shell sample is nvm/CI-biased.)

**Ansible (workload, 141 task-files, 558 tasks):**
- mutating **362 (64.9%)**, control/diagnostic **181 (32.4%)** — ~a third of "tasks" never touch host
  state (`set_fact`/`include_*`/`debug`/`stat`) ⇒ must be subtracted from any skip-rate denominator.
- mutating-with-pre-guard (`when`/`creates`/`removes`) **145 = 40.1% of mutating** — a crude VALUE
  *shallow* proxy, NOT the apply-cost×check-depth banding. Mostly `when` (263); `creates`/`removes` ~nil.
- top real modules (normalized): set_fact 62, file 57, include_tasks 54, service 36, template 36,
  package 30, command 28, apt 27, include_vars 26, stat 15, lineinfile 12, get_url 10, systemd 7, yum 7.
- `check_mode` collapsed 560→**14**: hand-written dry-run support is almost entirely a *test* artifact;
  real roles lean on module-native check-mode ⇒ Dorc's derived probe does work authors skip (opportunity).

**Per-stratum (the Simpson guard — aggregating misleads):** guarded/mutating = **role 45% (125/275),
homelab 7% (4/58), other 55% (16/29)**. Roles guard heavily; thin gitops/homelab Ansible barely does.
Collections contributed **0** workload tasks (all in tests/) — "breadth via collections" adds plugins+tests,
not workload; more *roles/playbooks* are the lever for Ansible-workload N.

**Method honesty:** N is small (362 mutating tasks ⇒ ~±5% on a proportion — directional, not tight) and
shell-workload is nvm/CI-biased. Both are *acquisition-scale* issues, not instrument issues: the
de-biasing + stratification machinery now exists, so scaling the sample is cheap (§7 scale decision).

## 5. What this already says (qualified) and the big caveat
- +SURE **`Q-PARSE`**: on a messy real sample, a strict-ish tree-sitter-bash pass hard-fails on 0% and
  flags ERROR nodes on 1.5%. Parse-feasibility is not the risk (corroborates CoLiS 99.9% / the priors).
- +SURE **`Q-UNANALYZABLE` (seed)**: eval 0.1% + dynamic-name 1.4% ⇒ the dynamic-construct ⊤-bound is
  small here. (Caveat: this counts eval/dynamic *command names* only; `source "$x"`, indirect
  expansion, and non-deterministic *reads* are not yet detected — the rate is a floor, not the full ⊤-bound.)
- ~SUSPECT **idempotency is module-native, not guard-native**: `creates:`/`removes:` together ≈ 0.1% of
  tasks. If real ops Ansible barely uses explicit cheap guards, then "derive a cheap check" has little
  to lift from the source — the check lives inside the module ⇒ Dorc needs an *oracle* per module-class.
  That nudges the `kDEPS` investment split toward oracle-library-heavy. **Hold loosely** (pre-bias).
- **THE CAVEAT (selection bias, load-bearing):** the raw tally is dominated by **collection test-code**.
  `assert` (1758+724 ≈ 2,482) is ~24% of all tasks and is test-scaffolding, not ops workload; the
  `community.docker.*` spike is integration-test churn; `ansible-test`/`ansible-playbook` dominate the
  shell side. **No band/anti-correlation number is trustworthy until test files are excluded, modules
  are split mutating-vs-control, and tallies are stratified by source-type.** This is exactly the
  "corpus shapes the instrument" lesson — discovered from data, not assumed.

## 6. Methodological fixes the data demanded — items 1–3 now done in scanner v2 (§4b); item 4 pending
1. **Exclude test/scaffolding** paths (`tests/`, `tests/integration/`, `molecule/`, `spec/`, `.github/`);
   count them separately, don't fold into workload.
2. **Mutating vs control split** (Ansible): drop `assert`/`debug`/`set_fact`/`include_*`/`import_*`/`meta`/
   `fail`/`block` from the `Q-BAND` denominator (they don't mutate host state). Shell: drop pure builtins
   (`echo`/`printf`/`:`/`true`/`[`/`test`/`return`/`read`/`cd`/`export`).
3. **Stratify** by source-type (role / collection / homelab / shell) and report per-stratum — the
   Simpson's-paradox guard.
4. **Shell sample is weak** (nvm + CI noise): for a real shell `Q-OPDIST`/banding, acquire actual
   provisioning shell (homelab bootstrap scripts; the bash-in-the-wild corpus; Debian maintainer scripts).

## 7. Next steps + the judgment-questions parked for the user
The remaining work is the analytically-delicate core; the classification *rules* are the user's call
(they determine the go/no-go), so they're flagged rather than guessed:
- **Q-BAND apply-cost rule** (`kPROBING`): which command/module classes count as *cheap-idempotent*
  (probe is pure overhead — `mkdir -p`, `file`, a no-op `package` on a current pkg) vs *expensive/
  dangerous/slow* (`apt`/`package` fresh install, service restart, `docker_*` up, build, migration,
  remote call)? This needs a per-class cost table — a defensible first draft is cheap to write, but the
  thresholds are taste + ops-experience.
- **Q-BAND check-depth rule** (`kPRECISION`): what counts as a *shallow* check (a cheap read-only guard
  fully captures need — pkg@version, file-exists, port-open) vs *deep* (daemon-mediated/hidden —
  `nginx -t`, `docker compose up`, the `/mnt/blah` catch)? In Ansible terms: does the module expose a
  native idempotency check (shallow) or is correctness daemon-/content-dependent (deep)?
- Then: **VALUE = expensive∧shallow** (size = primary go/no-go), JUST-RUN = cheap, HARD = expensive∧deep;
  and **Q-ANTICORR** = among expensive ops, the shallow:deep ratio. Each with N + the exact rule + error bars.
- **Scale decision** (already analysed in-session): a stratified subset suffices statistically for the
  go/no-go (proportions; ±1–2% at a few thousand ops); the full-corpus/cloud run is for the downstream
  oracle-library long-tail, not the spike. Build-to-scale (done — streaming), defer the run.

## 8. Tasks state (this session)
Done: toolchain (T1), scanner plumbing (T3). In progress: acquisition (T2 — modest sample done, can
scale). Pending: calibrate banding heuristics (T4 — needs §7 rules), run §3 set (T5), deliverable +
`kDEPS` go/no-go (T6).
