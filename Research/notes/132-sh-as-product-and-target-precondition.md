# 102 — sh-is-the-product weld + the target-sh-precondition (round 13, 2026-06-03)

> Durable notes for the platform-compatibility round (plan: `plans/130-platform-compatibility-research-plan.md`).
> interactive-research turn 2 — the human's design-steer at the `plans/130` gate + the synthesis it unlocked.
> Not a gather turn (no new sources; reframe rests on existing slugs + two ~SUSPECT leads for a later front).
> Corrects a load-bearing turn-1 claim (→ `notes/131`). Knobs: new weld `kLANG`, + `kWINLOCAL`/`kTPLATFORMS`, drop `kTRANSPORT`.

## Findings (most-attended first)
- **CORRECTION (load-bearing flip): a non-sh *target-backend* is NOT a deferrable add-on — it is a new product.**
  Because Dorc analyzes the *authored* language: the probe-compiler emits sanitized sh and its derivable-truth
  ceiling is sh-shaped. A 2nd *input* language ⇒ new analyzer/parser/oracle-contract-idiom; only the name + thin
  pluggable orchestrator survive. Human is "pretty much dead-sure" this is a day-one decision. → new foundational
  knob **`kLANG` (sh-is-the-product ↔ pluggable-language)**, NOT currently in KNOBS.md (verified). Proposed WELD
  to `kLANG-sh-is-the-product`.
- **My "prior art is unanimous" point was the wrong lesson.** Prior art is unanimously the *executor* pattern
  (put something on the target that speaks the target's language). Dorc's welds (agentless + what-you-type-is-
  what-runs + `kLANG`-sh) REJECT that pattern. So Windows-target isn't a "separate backend you defer"; it is
  out-of-scope OR precondition-gated. [A-ansible-windows-guide-2026][A-pyinfra-compatibility-2024][B-salt-supported-os-2026]
- **The escape from the human's "I hate all these options" dilemma — the sh-precondition pattern (Ansible<->Python
  analogy).** Their dilemma: to ship the first install-wsl to a sh-less Windows target you must either
  *learn-to-speak-pwsh* (pwsh as input) or *transpile sh->pwsh* — both violate `kLANG`. Third door: make
  "a reachable POSIX-sh evaluator" a *target precondition*, exactly as Ansible makes Python a target prereq
  (you don't ansible-install the first Python; raw runs pre-Python over the bare shell, and the very first
  Python is image-baked / manual). For Windows that precondition = git-bash / WSL / cygwin / busybox-w32, set
  once (+ sshd DefaultShell). No pwsh-input, no transpile, agentless intact, what-you-type-is-what-runs intact.
- **Why the precondition doesn't breach `kLANG`, and why it makes `kTPLATFORMS-wide` tractable:** the *authored
  language* stays sh (constant analysis ceiling across all targets); per-target variance lives in the *commands*
  = the *oracle library* (Get-Service vs systemctl), the same community long-tail story as everything else. So
  target-OS breadth is gated on a *capability* ("can evaluate POSIX sh"), not on Dorc learning N input languages.
  Genuine boundary: a target that *cannot* run any POSIX sh is unsupported — honest, consistent.
- **Knob bookkeeping:**
  - `kLANG` — new, foundational, NOT in KNOBS; propose welded -> sh-is-the-product. (Human asked "is it in KNOBS?" -> no.)
  - `kWINLOCAL` (Windows-as-orchestrator) — human: low priority, very short entry, place right before Welded,
    mild-lean `kWINLOCAL-nix-only-controller`.
  - `kTPLATFORMS` (target-platform breadth: mainstream <-> wide/long-tail) — human: slightly more important, lean
    wide; entangled with `kLANG` (the pwsh nightmare is the cautionary example).
  - `kTRANSPORT` — DROP. Human: subsumed by the "own-orchestration <-> farm-out-to-pyinfra-alike" tension. But
    that tension is **DESIGN.md component-3 prose, NOT a named knob in KNOBS.md** (verified) — flagged the
    discrepancy (human believed it already existed). Agreed sub-point: "own the orchestrator" *may* include
    Rust-native SSH (russh); the native-Windows no-ControlMaster gap [A-win32openssh-controlmaster-1328-2019]
    only reinforces it.
- ~SUSPECT leads to verify in a later front (NOT yet sourced): (1) busybox-w32 (rmyorston) — single static ash
  on native Windows = lightest sh-precondition; quirks unknown. (2) Ansible raw-module bootstraps-Python pattern
  as the precise prior-art analogy for "sh as target precondition."

## Citations
(no new sources this turn; corrections reference existing slugs above. the `notes/131` finding amended in place with a
forward-pointing correction line per notes-log discipline.)
