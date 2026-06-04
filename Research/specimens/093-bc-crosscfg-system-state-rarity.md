# 093 — specimen: bc-crossCFG — system-state save/restore is rare (but rarity ≠ safety)

> AI-generated, excerpt-class (`specimens/090` style). The `bc-crossCFG` finding (task #2): do *bounded
> CFG-sections that depend-upon, mutate, then restore shared **system** state* (do_x;…;undo_x over host
> state — the W5 wrong-skip, `plans/099` §2) actually occur in the wild? **Verdict: rare.** A GitHub
> tool-pair sweep found the obvious markers dominated by *non-transient* shapes, with one clean
> counterexample (pinned + verbatim below). Hold loosely — code-search can't see private ops — but the
> signal is consistent across four tool families.
>
> **⚠ Corrected 2026-06-03:** an earlier draft of this note called the restoring `trap` a *sound, free
> transient-detector*. That is wrong (the human caught it). The `trap` is a **contract, not a detector**:
> transient-ness is undecidable (Rice/W1), the trap-free do/undo is identical and more common, and an
> opaque mutator can perturb ambient state with no syntactic trace (W3 frame axiom). Rarity (this note's
> real finding) bounds **cost**, never safety or detectability. Corrected in the carries + ledger below.

## The negative result (the rare-hope holds)
Each canonical "system-state save/restore" tool-pair is dominated by a shape that is **not** a transient bracket:
- `iptables-save` / `iptables-restore` → **persistence**, not revert: `iptables-save > /etc/iptables.up.rules` then reload-at-boot via `if-pre-up.d` / `netfilter-persistent` (doubi, FunctionClub/SSR-Bash-Python, ezpptp, many VPN installers). The pair snapshots *desired* rules to disk; it does not bracket a mutate-then-undo.
- `setenforce 0` → **permanent disable**: `setenforce 0` + `sed -i 's/SELINUX=enforcing/disabled/'` (crazy886/SSR, self20/mt-server, rootsongjc, billchen8888, summmer121, …). SELinux is turned off and left off.
- `modprobe` / `rmmod` → **permanent blacklist-disable** (CIS hardening: `echo "install X /bin/true" >> modprobe.d; rmmod X`) or **reload-after-rebuild**, not a depend-restore window.
- `systemctl stop` / `start` → **service-control wrappers** (`start()`/`stop()` subcommands of a control script), not a transient stop-work-start.
- `chattr -i/+i` → not assessed (the operator was tokenized away by code-search).

The load-bearing distinction: **a save/restore tool-pair is usually *persistence*, not a do/undo bracket** — do not mistake `iptables-save…restore` for transient state. This materially supports the welded conservative default: since true system-state transient brackets are rare, "transient ⇒ maintain-cfg / never-hoist" (W5) costs almost nothing in practice.

## The counterexample, and its canonical shape
When the bracket *is* real, it clusters where `sp-tr-1` did — **image/rootfs build scripts** relaxing a host protection to do `chroot`/`mkosi` work. The clean case — [leifliddy/asahi-fedora-builder build.sh#L299-L304](https://github.com/leifliddy/asahi-fedora-builder/blob/43becccd00ebdf36673313a255e158af5f97f58f/build.sh#L299-L304):
```bash
if [[ $(command -v getenforce) ]] && [[ "$(getenforce)" = "Enforcing" ]]; then
    setenforce 0
    trap 'setenforce 1; exit;' EXIT SIGHUP SIGINT SIGTERM SIGQUIT SIGABRT
fi

mkosi_create_rootfs
```
The shape, exactly: **read-state** (`getenforce` = Enforcing?) → **conditionally relax** (`setenforce 0`) → **restore via `trap`** (`setenforce 1` on EXIT/signals) → **do work that needs it relaxed** (`mkosi_create_rootfs`). Three carries:
- The undo here is a live, state-mutating `trap` — but **the `trap` is a contract, not a detector** (this corrects the original draft). The identical, *more common* spelling drops it entirely: `[ "$(getenforce)" = Enforcing ] && setenforce 0; mkosi_create_rootfs; setenforce 1` — the same do/undo with **no `trap` at all**. A trap-keyed rule would treat the two oppositely, so it keys on crash-safety *hygiene*, not transient-ness. Worse, any opaque mutator (`echo 1 > /proc/sys/...`, a vendor binary, `eval`, a backgrounded flip) can transiently perturb an *ambient* fact with no `trap` and no tool-keyword — invisible under the frame axiom (W3). "Is this fact stable from T₀ to the guard?" is a behavioural property ⇒ Rice/W1 ⇒ **undecidable**; no syntax decides it. Soundness is the conservative default (`⊤`/don't-hoist unless *positively* ambient∧invariant via reaching-defs over *oracle-known* mutators, `plans/099` §5) **plus** the author **contract** (`trap`, or an explicit declaration) flagging frame-violations Dorc cannot infer — best-effort, never detection. The `trap`, when present, only *adds* a don't-hoist signal; its absence licenses nothing.
- It is **conditional** + **state-introspecting** (`getenforce`) — the same shape as `nvm.sh`'s `$-` introspection in `092`, but over *system* state.
- Partial sibling: `MeowDump/Integrity-Box action.sh` captures `orig_selinux` then `[ "$orig_selinux" = "Enforcing" ] && setenforce 0` (save → conditional-relax; restore elsewhere) — same family.

## Ledger / task-#2 outcome
- **Finding:** system-state transient brackets are **rare**; the obvious tool-pairs are persistence / permanent-disable / service-wrappers. Strong across four tool families (hold loosely; private ops unseen).
- **Tool-keywords (oracle library):** `setenforce`/`getenforce` (SELinux enforce-state) — the one real transient-toggle found; `iptables-save/restore` and `modprobe/rmmod` flagged as *usually-persistence/permanent* (an oracle should model them as ambient-state writers, not transient brackets).
- **Bake-into-core consequence:** rarity bounds the **cost** of the conservative default (we seldom lose a skip by withholding), **not** its safety and **not** detectability. There is no sound transient-*detector* (Rice/W1; the trap-free do/undo above is identical and invisible; opaque mutators leave no trace under the W3 frame axiom). Safety is the positive-license default — `hoist iff ambient∧invariant` (`plans/099` §5) — plus the author contract; rarity just makes that conservative default *cheap*. **Effect ≠ rarity:** this note measured how *often* the bracket occurs, not how bad it is when it does (silently unsound, undecidably-detectable) — do not read rarity as handleability.

> Source (commit-pinned): leifliddy/asahi-fedora-builder @43becccd, `build.sh` L299-L304, verbatim. Negative-result repos cited in aggregate (search observations, not pinned excerpts).
