> Re-run of the sol-adversarial lane (original 308e lacked a completion marker); clean context, same kit section, reviewed tip 083efd8a. Codex exit: 0.

## 1. Pure-predicate carry consults a shadowed verdict and can wrongly elide a wrapped mutation

Severity: Critical

Confidence: +SURE

Location:

- [spike/crates/cli/src/survival.rs:695](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:695): `resolve_inner_check(... node, live)` selects the positionally live inner body.
- [spike/crates/cli/src/survival.rs:762](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:762): `try_carry(&chain, inner_provider, verdict_sets, &invariance)`.
- [spike/crates/cli/src/survival.rs:865](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:865): `verdict_sets.iter().find_map(|set| set.get(inner_provider))?`.
- [spike/crates/cli/src/survival.rs:868](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:868): `read_set_closed(verdict)` is therefore run over that first global match, not the definition live at the site.
- [spike/crates/cli/src/survival.rs:801](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:801): a successful answer creates `WrappedProbe::Carry`.
- [spike/crates/cli/src/survival.rs:802](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:802): the fact is consequently re-sited to `Context::HostDefault`.

Attacked plan/law:

> "A query at site S = `live_definition(frame(S), name)` → read THAT definition's rows."  
> — [Research/plans/28Q-context-kernel-unification.md:102](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/Research/plans/28Q-context-kernel-unification.md:102)

> "every SITE-KEYED consuming act … answers only from the definition live AT the site"  
> — [spike/crates/analysis/CLAUDE.md:74](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/analysis/CLAUDE.md:74)

> "A substrate-axis fact travels unflagged iff … the engine proves the verdict body READ-SET-CLOSED"  
> — [spike/CLAUDE.md:257](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/CLAUDE.md:257)

How it breaks:

+SURE The site's shipped body and its carry license can come from different authored definitions. `resolve_inner_check` correctly selects the frame-live verdict, but `try_carry` independently chooses the first matching verdict in load order. The latter body supplies the read-set-closure proof that licenses `Carry`; the former body is what the probe actually executes.

+SURE This is not merely conservative disagreement. A read-set-closed earlier definition can license ambient measurement of a later, frame-live definition containing an unmarked context-sensitive read. A positive ambient result can then elide a mutation whose verdict would differ in the wrapped filesystem context. This violates the highest-priority "never under-execute" rule at [IMPLEMENTATION.md:173](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/IMPLEMENTATION.md:173).

Failing-world construction:

`chroot.oracle.sh`, loaded ambiently:

```sh
# dorc-lang/v0.2
chroot__predict() {
   shift
   "$@"
}
chroot__lend_map() {
   printf '%s\n' "$1" : lends fs-view
   : lends user
   : lends netns
   shift
   "$@"
}
```

`kp-good.oracle.sh`, loaded before the book:

```sh
# dorc-lang/v0.2
kp__is_converged() {
   kp check "$1" "$2" : sm.dorc.KernelParam:"$1"
}
sm_dorc_KernelParam__state_stored_only_in() {
   printf 'kernel-sysctls\n' : stored-in kernel
   : undivided-by-transit-across fs-view
}
```

`kp-bad.oracle.sh`, sourced only inside the subshell:

```sh
# dorc-lang/v0.2
kp__is_converged() {
   [ -e /context-ready ] &&
      kp check "$1" "$2" : sm.dorc.KernelParam:"$1"
}
```

Book:

```sh
#!/bin/sh
(
   . ./kp-bad.oracle.sh
   chroot /mnt kp ip_forward 1
)
```

World and load structure:

- `kp-good.oracle.sh` is earlier in `source_refs`/`verdict_sets`.
- The subshell source positionally replaces `kp__is_converged` with `kp-bad` only inside the parentheses; this is the explicitly sanctioned regional-shadow case.
- `/context-ready` exists in the ambient filesystem but not under `/mnt`.
- Ambient `kp check ip_forward 1` returns 0, while the state under `/mnt` still requires the mutation.

Wrong result:

1. +SURE `resolve_inner_check` selects and ships `kp-bad`, because that is live at the site.
2. +SURE Entry is unavailable for `chroot`, so the carry fallback runs.
3. +SURE `try_carry` selects `kp-good` via the first-match scan and proves that different body read-set-closed.
4. +SURE `WrappedProbe::Carry` measures the selected `kp-bad` body ambiently; `/context-ready` exists there, so the probe can report convergence.
5. +SURE The wrapped `chroot /mnt kp ip_forward 1` line can be elided although the live body would not report convergence in `/mnt`.

The fix boundary is not merely changing iteration order: `try_carry` needs the same resolved `DefinitionId`/file already selected for `inner_sh`, or a typed object coupling the selected body to its closure proof.

## 2. The wrapper/entry bundle still uses first-loaded whole-unit winners and bypasses both positional resolution and contested withdrawal

Severity: High

Confidence: +SURE

Location:

- [spike/crates/cli/src/survival.rs:632](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:632): every site receives one unit-wide `WrapperIndexBundle`.
- [spike/crates/cli/src/survival.rs:942](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:942): the bundle re-lifts raw `oracle_refs`.
- [spike/crates/cli/src/survival.rs:943](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:943): `lift_predicts(interner, src)` ignores the driver's already-withdrawn `checks`.
- [spike/crates/cli/src/survival.rs:963](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:963): `enter_defs.entry(p).or_insert(...)` freezes the first entry body.
- [spike/crates/cli/src/survival.rs:965](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:965): `wrappers.entry(word).or_insert(...)` freezes the first wrapper model.
- [spike/crates/cli/src/survival.rs:979](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:979): tolerance authority is also first-match.
- [spike/crates/cli/src/survival.rs:682](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:682): every book site is peeled through that frozen bundle.
- [spike/crates/cli/src/survival.rs:718](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/src/survival.rs:718): the resulting first-loaded entry definitions are placed in the executable composed probe.

Attacked plan/law:

> "Every derived row — a check, a cell declaration, an argparse arm-model … — is keyed by the DefinitionId that produced it."  
> — [Research/plans/28Q-context-kernel-unification.md:87](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/Research/plans/28Q-context-kernel-unification.md:87)

> "A query at site S = `live_definition(frame(S), name)` → read THAT definition's rows."  
> — [Research/plans/28Q-context-kernel-unification.md:102](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/Research/plans/28Q-context-kernel-unification.md:102)

> "Seats read the driver's WITHDRAWN per-file sets, never raw source … A seat that re-lifts is a seat that will disagree."  
> — [spike/crates/oracle/CLAUDE.md:62](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/oracle/CLAUDE.md:62)

> "a contested family is removed from EVERY lifted set at THIS edge … Its sites fall to `Opaque` → `MustRun` → no vouch candidate, no probe ship, no license"  
> — [spike/crates/cli/CLAUDE.md:113](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/cli/CLAUDE.md:113)

How it breaks:

+SURE `build_wrapper_index` is an unconverted derived-row seat. Wrapper argparse/peel models, lend maps, entry forms, and tolerance vouches are collapsed to first-loaded values before any site or frame is considered. The selected data carries no `DefinitionId`.

+SURE It also re-lifts raw source after the driver has removed contested families from `checks` and `verdict_sets`. Thus an ordinary same-frame collision that the binding law says must become wholly undescribed can still be recognized as a wrapper, peeled, supplied an entry form, and granted entry authority.

+SURE The regional case is independently wrong even without a contest: a subshell-local redefinition is allowed to replace the outer body within that frame, but every site continues to use the outer, first-loaded wrapper model.

Failing-world construction:

`sudo-base.oracle.sh`, loaded first:

```sh
# dorc-lang/v0.2
sudo__predict() {
   while [ "${1#-}" != "$1" ]; do
      case "$1" in
      -u) shift 2 ;;
      *) shift ;;
      esac
   done
   env -i HOME=/root "$@"
}
sudo__lend_map() {
   target=root
   while [ "${1#-}" != "$1" ]; do
      case "$1" in
      -u) target="$2"; shift 2 ;;
      *) shift ;;
      esac
   done
   printf '%s\n' "$target" : lends user
   : lends fs-view
   : lends netns
   "$@"
}
sudo__enter() {
   sudo -n "$@"
}
```

`hork.oracle.sh`:

```sh
# dorc-lang/v0.2
hork__is_converged() {
   : safe-across user
   case "$1" in
   install) hork query "$2" ;;
   *) return 2 ;;
   esac
}
```

`sudo-local.oracle.sh`, sourced only in the subshell:

```sh
# dorc-lang/v0.2
sudo__predict() {
   return 2
}
```

Book:

```sh
#!/bin/sh
(
   . ./sudo-local.oracle.sh
   sudo hork install wombat
)
```

Wrong result:

1. +SURE The frame-live `sudo__predict` at the call site is `sudo-local`, which declines and supplies no peeling/entry judgment. Under the ruled positional regime, `sudo` must remain opaque and the line must run.
2. +SURE `build_wrapper_index` instead retains `sudo-base` through `or_insert`.
3. +SURE The site is peeled to `hork install wombat`; the first-loaded `sudo__enter` and wrapper context are attached.
4. +SURE If the capability and dial admit entry and the root-context probe reports `hork` converged, the original `sudo hork install wombat` can be elided.
5. +SURE The license therefore rests on a definition that is not live at the site. The local engineer's explicit decline has been silently bypassed, and the admin sees a removed line rather than the required run.

+SURE A same-frame variant is simpler: load two sources defining `sudo__predict`, with the second replacing the first and no intervening `unset -f`. The driver's contested-family set withdraws the normal predict/verdict vectors, but the raw re-lift at line 943 reconstructs the first wrapper anyway. That directly falsifies the law's "no seat sits outside the edge" assertion.

A correct repair needs per-definition wrapper-related rows and site resolution over the already-withdrawn vectors. Merely changing `or_insert` to last-wins would still fail subshell, re-source, and `unset -f` worlds.

## did not hold:

- +SURE `DefinitionId` itself does distinguish two definitions in one file by `(SourceFileId, span)`; I found no identity collapse in [spike/crates/core/src/definition.rs:35](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/core/src/definition.rs:35).
- +SURE `answering_file` withholds ambiguous rows and plural `NoOpinion` candidates; the permissive fallback did not yield a concrete wrong license in the production solved-environment path at [spike/crates/core/src/definition.rs:150](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/core/src/definition.rs:150).
- +SURE The ordinary predict effect/cell lane couples argparse resolution and cell lookup to the same selected file; the old chimera attack did not survive [spike/crates/analysis/src/effect.rs:463](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-ada9eb2cc59612aa7/spike/crates/analysis/src/effect.rs:463).
- +SURE The ordinary verdict auto-cell, probe-ship, ambient vouch, and wrapped-vouch seats all call `definition_before` plus `answering_file`; I found no forced wrong-author construction within those individual seats.
- +SURE The `disturbs()` resolution and shipping changes are positional and the main driver now withdraws contested sets before those seats; the earlier first-resolving-footprint attack did not hold after this range.
- +SURE The retained `dialect_minting_source` fold is whole-unit by design and is filtered by `binds_somewhere`; I found no construction where a never-live definition enlarged the dialect.
- +SURE Same-file repeated role definitions become `DefinitionProvenance::Ambiguous` and withhold. That is conservative rather than a wrong license; it does not deliver regional precision, but the current law explicitly treats same-file redefinition as a refusal class.
- +SURE The new differential frame tests exercise direct role bodies and helper collision, but not wrapper-model selection or carry-proof selection; this explains why both findings can remain green without making the existing byte-identity gate itself fraudulent.
