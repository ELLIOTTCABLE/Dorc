# 101 — Security threat-modeling: problem-space map + fronts (round 10)

Synthesis/charter for the security-prior-art spike (raw findings + citations: `../notes/100`).
**Variance question** (leverage tradeoffs the human weighs), so this is a map + dig-list, not a
conclusion. Every claim grounds in a graded `[slug]` (`../sources.json`). The human's round-10
gap-answers are folded in where they change a front. Nothing here is feature-design — it's a
threat-map and a research order.

**Methodology (dogfooded):** the field's own answer to "how should we threat-model" is **Shostack's
Four Questions + STRIDE-per-element, kept lightweight, run by everyone** — *not* expert attack-trees
(Shostack deprecates them [B-shostack-threat-modeling-guide-2022]; the Manifesto's *Hero-Threat-Modeler*
anti-pattern rejects expert-gating [B-threat-modeling-manifesto-2020]). That maps onto Dorc's
no-cliff/two-user constraint. Keep Schneier's attack-tree *reuse* [A-schneier-attack-trees-1999] as the
*oracle-composition* lens only; Klein premortem [C-klein-premortem-author-page-2007] as the kickoff.

## The landscape — trust boundaries (the diagram to draw, STRIDE per element)
```
 [engineer]→authors→ [oracle library/registry] —obtain→ [operator workstation = CONTROL NODE]
                                                              │ holds SSH keys to whole fleet
                                                       push (ssh)│  emits plan-as-shell to operator TTY
                                                              ▼
                                              [managed host] — probe (read-only?) then apply (root)
```
| element | prior-art lesson | slug |
|---|---|---|
| oracle library | Dorc *is* a package manager (transitive exec); Galaxy got integrity/immutability wrong | [B-nesbitt-quacks-package-manager-2026] |
| control node | push avoids Salt's listening-master RCE but relocates the crown-jewel to the operator | [A-fsecure-saltstack-bypass-2020] · [B-embracethered-ssh-agent-hijack-2022] · [A-cfengine-security-trust-model-2001] |
| probe phase | "read-only" ≠ "side-effect-free"; guards create inter-resource deps a dry-run can't evaluate | [A-chef-whyrun-harmful-2018] · [B-terraform-external-plan-exec-2017] |
| probe sandbox | seccomp is a classifier, not containment (44/300 syscalls; ptrace bypass) | [B-docker-seccomp-docs-2026] · [B-jessfraz-seccomp-profiles-2016] |
| plan output | Dorc emits + *displays* shell → terminal-escape & quoting injection in its own UX | [A-wheeler-filenames-in-shell-2025] |

## Leverage fronts (ranked by lock-in × under-coverage)

**front-1 · Don't own a code-fetching/registry surface (human's lean: "by-contract, no code-fetching
features — allergic to unnecessary responsibility").** Dorc oracles compose transitively, so Dorc would
inherit all six package-manager problems [B-nesbitt-quacks-package-manager-2026]; Galaxy proves they're
retrofit-hostile (opt-in/off integrity, overwritable versions, become-escalating exec). **The
highest-leverage move is to not build the registry/installer at all** — the most secure registry is no
registry. Oracles/books are obtained however you obtain shell today (`git clone`, vendoring, copy-paste),
**disowned-and-lazy** by design (do-one-thing-well; ergonomic). **Human caveat that weakens the easy
story: real users do *not* read the shell they run** — so "supply-chain collapses to 'you ran a script you
read'" is too optimistic. The realistic backstop is **defensive linting** (shellcheck-grade + Dorc-specific
security lints; back-of-mind, "about the best we can do"; ties to front-5). **Registry / identity /
integrity as a *Dorc* responsibility is out-of-scope for this pass** (human: sensitive to too many things,
not soon, dangers incl. security-responsibility understood — parked, not adjudicated). NB *distinct* from
the separately-planned **version-lattice**: first-classing "version" as a comparable lattice over foreign
registries' version strings (a *meta-registry* for canonical comparison — e.g. an `npm` oracle declares
`semver` as the correct parser for its domain — **not** a place-to-get-code). Some analyses may *depend* on
that comparability; it is not the supply-chain surface and is not decided here.

**front-2 · The probe contract is *forced*, not chosen (human corrected round-10's false menu).** Chef's
first-party post-mortem [A-chef-whyrun-harmful-2018] shows the guard-dependency hazard (a probe can't
evaluate `only_if 'rpm -q httpd'` correctly without the prior install actually running — the TODO
elision-soundness hazard generalised) *and* that "no-op modes are not side-effect-free" (read-only
interrogation locked up systemd). There is **no 2a/2b/2c design choice**: *read-only* is welded-forced
(`kFAIL-withhold` — withhold any probe we can't prove non-mutative),
<!-- /* superseded 2026-06-06: carve-out — "withhold any probe we can't prove non-mutative" excepts the oracle's OWN declared command (`mycmd` inside `mycmd.check()`): opaque, never provable-inert, yet self-vouched and therefore shippable, or no oracle could exist (DESIGN "Inference limitations"). The withhold binds the *non-self-vouched* leaves; the self-command is "inert relative to the author's grounding" — this doc's own L63-64, `plans/102` E3. */ -->
and *bounded-cost* is inherently
best-effort (you cannot statically bound an arbitrary leaf's cost; and even a genuinely read-only call can
block / wedge the host). The only honest contract is "**read-only or withheld; cost & effect are
best-effort *flags*, not promises**." What's left is not a decision but:
- a **hazard to state, never promise against**: read-only ≠ non-blocking ≠ side-effect-free (the
  systemd-lockup class) [A-chef-whyrun-harmful-2018] · [B-terraform-external-plan-exec-2017];
- an **existing dial**, not a new knob: how hard the analyzer tries to *establish* a leaf inert before
  shipping it as a probe = `kPRECISION` / `kPROBING` — best-effort, bounded by the author's grounding
  (non-mutation is the author's contract; soundness stops at the oracle boundary — see `plans/102` E3);
- one **partial mechanism**: a timeout / rlimit bounds probe *wallclock* (safe to kill a truly read-only
  probe) but does nothing for the lockup class — mitigation, not guarantee.
Touches `kFAIL` (welded for *mutation*; the observation-effect class sits *outside* the weld and is
unpromisable), `kPROBING`, `plans/077`.

**front-3 · Push is *ergonomic*, NOT a security win — and real bastion-hopping reintroduces the risk
(human's steer).** Salt's CVE is the listening-master disaster [A-fsecure-saltstack-bypass-2020]; push
removes that daemon, BUT the human's point stands: real deployers do **m→n→o bastion-hopping** (the
Mitogen connection-delegation pattern, already in `notes/073`/source-manifest) for genuine reasons, and
multi-hop SSH reintroduces multi-hop trust — SSH-agent-hijack on any intermediate host pivots to the
fleet [B-embracethered-ssh-agent-hijack-2022]. So: **make no security claim for push/agentless**; treat
the operator node + the hop chain as the threat surface. CFEngine's pull/voluntary-cooperation is the
honest counter-model [A-cfengine-security-trust-model-2001] and donates a trust checklist worth adopting
regardless of push/pull: integrity-over-secrecy ("the input file does not have to be private as long as
it is authentic"), host-identity verification (DNS/host-key spoofing = pushing config+secrets to an
imposter), encryption≠trust. Dig: the secure multi-hop patterns (ProxyJump vs `ForwardAgent`; short-TTL
certs) against the Mitogen delegation model.

**front-4 · Separate "observe" from "contain" in the design vocabulary (resolves the TODO seccomp
doubt).** seccomp blocks ~44/300 syscalls, is "compat not containment," and ptrace can bypass it
[B-docker-seccomp-docs-2026]; profiles are error-prone to hand-author [B-jessfraz-seccomp-profiles-2016].
`plans/077`'s use is **defensible *because* observe-only** (log `socket(AF_INET)`); the hazard is
category-creep into "sandbox for untrusted oracle code." Given front-1's lean (oracles are reviewed, not
auto-fetched), Dorc *needn't* sandbox untrusted code at all — which dissolves most of the seccomp-as-
containment worry. State plainly: seccomp = cost-class *classifier*; real isolation (container/VM/gVisor)
is a different order of problem `plans/078` already declines to claim.

**front-5 · Dorc is a shell code-generator AND displayer — injection in its own output (cheap,
concrete).** Wheeler [A-wheeler-filenames-in-shell-2025]: terminal-escape injection via the defining
plan-as-shell CLI output (a hostile oracle or adversarial remote filename injects into the *operator's*
terminal); quoting/codegen correctness when emitting sanitized probes/plans; adversarial remote filenames
during probing. Strip control chars before display; template quoting. Cheap now, expensive to retrofit.

**front-6 · The methodology artifact (the human's primary ask).** A per-oracle lightweight threat-model
template: STRIDE-per-element on the trust-boundary diagram, fillable by a non-expert author (no-cliff),
attack-tree *reuse* as the composition model, premortem as kickoff. Low lock-in, high recurring value.

## KNOBS — adjudicated at this gate
- **`kAGENTLESS` (was `kBLAST`) — ADDED to `../../KNOBS.md` (welded section).** `kAGENTLESS-push ↔
  kAGENTLESS-host-autonomy`: the blast-radius/security dimension behind the welded push choice. *Not a live
  knob* (push is welded per DESIGN); named in the welded section to keep visible what push gives up
  (operator-node crown-jewel; multi-hop bastion trust). Renamed to the industry term per human (front-3).
- **`kTRUST` — NOT added; deferred / out-of-scope.** The oracle-distribution integrity axis. Human's lean:
  cede code-distribution to `git` (ergonomic, do-one-thing-well); registry-as-Dorc-responsibility is too
  sensitive and not soon (dangers understood, parked). Subject is new but shares `kOOB`'s
  plain-shell-vs-machinery shape; not worth a slug while parked. (Separate, not this: the planned
  version-lattice meta-registry — front-1.)

## Gating questions — resolved at the round-10 gate (2026-06-02)
- gap-1 (oracle execution trust): **no code-fetching features; cede to `git`.** Caveat: users don't read
  scripts → **defensive linting** is the realistic backstop, not review. Registry/version out-of-scope. → front-1.
- gap-2 (probe contract): **not a decision — read-only welded-forced, cost best-effort.** Honest contract =
  "read-only or withheld; cost/effect best-effort flags." Hazard named; proof-effort = `kPROBING`/`kPRECISION`. → front-2.
- gap-3 (push framing): **ergonomic, not a security claim; bastion-hopping reintroduces risk.** Knob =
  `kAGENTLESS` (welded). → front-3.

## Next — (b) threat-model pass, then (c) implementation synthesis (human's framing; not a new gather-round)
"Round 11" is retired: the leftover gather folds *into* the model — pull a source only where a STRIDE cell
or premortem branch lacks backing, not as a standalone round.
- **(b) threat-model** (work-the-fronts, in-skill `narrow-and-regrade`): Shostack-4Q + STRIDE-per-element +
  a premortem over the trust-boundary diagram → the front-6 per-oracle template + a concrete `[slug]`-cited
  threat list. Targeted gather as needed (via `gh`, not mcp-fetch): front-2 Ansible `--check` + CoLiS/smoosh
  static-non-inert detection; front-3 OpenSSH `ProxyJump`/host-key + Mitogen delegation; front-1
  defensive-lint prior-art (shellcheck capabilities/limits).
- **(c) implementation synthesis** (`final-synthesis`): fold modeled threats + mitigations into
  implementation-shaped, per-component findings (feeds TODO "preparation-for-agentic-implementation").
