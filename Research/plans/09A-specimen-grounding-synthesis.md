# 09A — specimen-grounding synthesis (round-9 companion): bless/abdicate × bake-into-core

> **Status (2026-06-03): round-9 companion synthesis.** The on-ramp for the real-world **specimen** hunt
> (`specimens/090`–`093` + `tools/inline-specimen.sh`), run in parallel with the state-tracking conclusion
> `plans/099`. It is the "real-world grounding" `099` §9 lists as an open thread — it **complements `099`,
> does not edit it**. Specimens are **purposive design-quarries, not measurements**: real scripts chosen to
> show how facts are *spelled* in the wild so the designer can pick what to **bless** vs **abdicate** — not a
> representative corpus statistic (that's the parked `plans/086` go/no-go, a different question). AI-generated;
> confidence-marked; the bless-decisions themselves are the human's.

## 0. The result in one paragraph
Every real-world specimen sorts onto one side of a single line, and that line *is* `099`'s W4 made operational:
**abdicate** the higher-kinded *content* (what a `package`/`service`/`mount` *is*, and the m×n zoo of providers ×
aliases — `091`) vs **bake-into-core** the *control/state-flow scaffolding* the analyzer owns regardless of any
oracle (`090`/`092`/`093`). The bake-into-core column has a clean canonical form per concept (the Tier-A ledger,
§3c); the abdicate column has *no* canonical form, by design (Tier-B, oracle-declared per named kind). The round's
sharpest correction: **rarity ≠ effect** — system-state transient brackets are *rare* (`093`), bounding how often
the conservative default *costs* a skip, but transient-ness is **undecidable** (Rice/W1) and opaque mutators are
**invisible** (W3 frame axiom), so it is never *detectable*; the `trap` is a **contract, not a detector**.

## 1. The organizing frame — abdicate (W4) vs bake-into-core
+SURE. From `090`/`091`'s bless/abdicate lens, confirmed across all four specimens:
- **Abdicate** = the open-ended, higher-kinded *content* — kinds and their providers/equivalences. Dorc is
  referent-agnostic (`099` W4); it never learns what `nginx` *is*. Oracles declare it against a **named kind**,
  and cross-oracle identity binds to that name (`099` C3 coherence), never an accidental shared token.
- **Bake-into-core** = the control/state-flow the analyzer must model itself to stay correct — *not* abdicatable,
  because it is the book's own flow, not external knowledge.
The boundary is not a vibe: §3c shows it is exactly the Tier-A / Tier-B split of the blessed-forms ledger.

## 2. Abdicate, confirmed in the wild (`090`/`091`)
+SURE. `091` (stack `get-stack.sh`) is the motivator — a hand-built Puppet-RAL (OS→distro→provider dispatch,
`has_X` capability predicates, a `try_lsb || try_release || try_issue` fact cascade). The cost of *not* abdicating
is **m × n**: every alias/equivalence table (`ubuntu|linuxmint|elementary|neon|pop`, `arm64≡aarch64`) times *every
kinded concept that exists* — baking it in is shipping "a maintainer-arbitrated list of Every Thing as a built-in
type." What we *cannot* abdicate is the **meta-contract debt**: the plain-sh idiom by which mutually-unaware
oracles declare `provide` / equivalence / wrinkles (open; `099` C1–C3 bound it). Human's live lever: first-class
`$PATH` as availability-truth may cleave "is X *runnable*?" (a bakeable core fact) from "is X *managed*?" (abdicated).

## 3. Bake-into-core, grounded (`090`/`092`/`093`)
### 3a. Shell-execution-environment state (`092`) — a category that needed naming
+SURE. A third kind of state, distinct from system-state (`099` W5) and program-variables (the variable-closure
the human warns against): the **shell execution environment itself** — options (`$-` / `set -e` / `pipefail` /
`noglob`), cwd, fds, traps. Real scripts read, mutate, **scope**, and restore it, and it alters reachability
(errexit), path-resolution (cwd), and deferred execution (traps). Not abdicatable (it is the book's flow). Two
scoping mechanisms are load-bearing: subshell `( )` / `$( )` *contains* a `cd`/`set`/assignment (the inverse of
the transient problem — looks global, is local); and `set +e … set -e` brackets a region of changed reachability
(`nvm.sh` even introspects `$-` to do it conditionally).

### 3b. System-state transient brackets (`093`) — rare
~SUSPECT-rare (small, nvm-biased corpus; code-search can't see private ops). The obvious save/restore tool-pairs
are dominated by **non-transient** shapes: `iptables-save/restore` = *persistence* (snapshot→reload-at-boot), not
revert; `setenforce 0` = *permanent disable*; `modprobe/rmmod` = *blacklist* / reload; `systemctl stop/start` =
service-control wrappers. Genuine do/undo-over-system-state is rare and clusters in **image/rootfs builds**
relaxing a host protection for `chroot`/`mkosi` (lone clean counterexample: leifliddy asahi-fedora-builder).

### 3c. The canonical blessed-forms ledger (the `bc-mustguard` output)
Answers "one spelling per concept" — and the answer is a **two-tier split that *is* the abdicate line**:
- **Tier A — core-blessed** (analyzer recognises for everyone, no oracle): guarded-establish
  (`if ! PROBE; then ESTABLISH; fi` ≡ `PROBE || ESTABLISH`, note 094) · `[ -f X ]` (file-existence dep) ·
  `[ A -nt B ]` (freshness dep) · `command -v X` (tool presence — POSIX-blessed; `which` rejected *regardless of
  frequency*) · `set -euo pipefail` (order/fail-fast) · `trap '…' EXIT` (transient-cleanup *spelling*) ·
  `cmd || true` (best-effort) · `( cd X && … )` (scoped run).
- **Tier B — oracle-declared** (per-kind, abdicated, plural-by-design): the actual probe/establish *commands*
  (`dpkg -s`, `apt install`, `systemctl is-active`, …). No canonical here, by design.
Tier A blesses the *structure & predicates*; Tier B is the *commands inside*, oracle-named. (+SURE on the split
and the predicate forms; ~SUSPECT only on blessing one-vs-both guarded-establish surface forms — lean both,
CFG-identical. Final bless-decisions are the human's; `kBURDEN` = declare-anchor + infer-propagation, `099` §5.)

## 4. The load-bearing correction — rarity ≠ effect; contract ≠ detector
+SURE (proved by counter-example + Rice/W1; the human caught the error — recorded so it cannot silently
propagate). An earlier draft of `093` called the restoring `trap` a *sound, free transient-detector*. **Wrong and
dangerous.** Counter-example: the trap-free `[ "$(getenforce)" = Enforcing ] && setenforce 0; work; setenforce 1`
is semantically identical to the trap-wrapped form and *more common* — a trap-keyed rule keys on crash-safety
hygiene, not transient-ness. And any opaque mutator (`echo 1 > /proc/sys/…`, a vendor binary, `eval`, a backgrounded
flip) perturbs ambient state with no trace. "Is fact F stable T₀→guard?" is a behavioural property ⇒ **Rice/W1 ⇒
undecidable**; opaque effects are **invisible ⇒ W3 frame axiom**. Therefore:
- The `trap` is a **contract** (author *declares* a frame-violation Dorc can't infer), never a **detector**; its
  absence licenses nothing.
- Soundness is the **conservative default** — hoist *only* on a positively-established ambient∧invariant
  MUST-anchor (`099` §5 `kFLATTEN`; reaching-defs over *oracle-known* mutators) — plus author contracts.
  Best-effort under W3, never sound-by-detection (`kVERIFY`-calibrate, not prove).
- **Rarity bounds COST, not safety or detectability.** `093`'s rarity means the conservative default *rarely
  loses a skip* — nothing more. Never read rarity as handleability.

## 5. Method notes (for the next specimen pass)
- **Tooling reality:** GitHub code-search (octocode) tokenizes away operators (`set +e`, `|| true`, `nginx -t`,
  `-i/+i`) → useless for operator-heavy idioms; use **local ripgrep** over the round-6 corpus or **`gh`-raw +
  Read** for those. octocode is fine for bare-word markers (`pipefail`, `trap`, `setenforce`, `modprobe`).
  Also: octocode's file-fetch **elides comments + blank lines** (~20% on `sp-tr-1`) — never deep-read a specimen
  through it; use `gh`-raw.
- **Verbatim discipline:** whole-file specimens via `tools/inline-specimen.sh` (commit-pinned, byte-checked,
  edit-around-the-fence, never retype); multi-source snippet *classes* (`092`/`093`) hand-quoted with
  commit-pinned, line-anchored permalinks.

## 6. Scope — explicit non-goals + open threads
- **Out of scope (do not reopen):** `bc-alias` (local non-state analysis assumed soundly-analyzable; ShellCheck
  is prior-art); `bc-topboundary` (eval / `source "$dyn"` / dynamic-name / heredoc-codegen — excluded *hard*;
  fully our control; touches no unwelded knob).
- **Open threads:** (a) the Ansible-role→oracle **rewrite** exercise — candidates surfaced (`sp-an-*`: uncletopia
  `rc`-interpretation, geerlingguy `changed_when: false` probes) but none rewritten; (b) deeper **`tgt-lazy`**
  (low-derivable-info / rare-tool) coverage; (c) `Research/Vendor/` round-2 (held by the human); (d) the
  meta-contract notation (§2; also `099` §9 — a language/UX pass, not this round).

## 7. Artifact index
- `specimens/090-literate-specimen-kernel-task-runner.md` — whole-file; transient/`trap`, transport-via-mount,
  atomic-publish/TOCTOU, the bless/abdicate ledger. (`sp-tr-1`; the grounding `099` §9 asked for.)
- `specimens/091-specimen-stack-get-stack.md` — whole-file; RAL-by-hand, m×n abdication motivator, meta-contract debt.
- `specimens/092-bc-cfg-shell-env-state.md` — excerpt-class; shell-execution-environment state (§3a).
- `specimens/093-bc-crosscfg-system-state-rarity.md` — excerpt-class; system-state transient rarity + the §4 correction.
- `tools/inline-specimen.sh` — verbatim commit-pinned inliner.
- Round tasks #1–3 (`bc-cfg` / `bc-crossCFG` / `bc-mustguard-canonical`) — completed.
