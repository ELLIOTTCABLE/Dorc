# 30Pb - Load-plane and emission-planner review findings

> Tier: **REVIEW FINDINGS, NOT RULED DESIGN.** This document is input to a later
> human-led design sitting. It does not settle project direction, authorize implementation,
> amend `30P`, or override root documents, `spike/CLAUDE.md`, `KNOBS.md`, or human-typed
> rulings. A design-sitting conductor must present the choices to the human and record the
> resulting rulings elsewhere.
>
> Scope: design-tier findings from a critical review of
> `plans/30P-emission-planner-and-inclusion.md` through `a3e021bb`. Additional analysis is
> recorded in `Research/quarantine-DO-NOT-READ/30Pa-load-emission-security-review.md`, which
> may be read only by a reviewer explicitly authorized to access quarantined content.

## 30Pb:review-summary

The plan's broad direction remains coherent: one emission planner, explicit form refusal,
set-valued load analysis, point-local unknown-source handling, and an uneven floor across
multipart and single-stream forms. The review found nine places where the design needs a
sharper invariant before those mechanisms harden into shared types.

Each item below is a question/recommendation for adjudication, never a ruling.

## 30Pb:fnd-possible-singleton-is-not-exact-selection

`30P:200-218` correctly models a partly dynamic operand as a set of possible sources. A
singleton set does not by itself prove that the original shell expression selects that
member at runtime:

```sh
. "$LIB/shared.oracle.sh"
```

One matching suffix in the current snapshot says what Dorc happens to hold, not what every
value of `LIB` means. Re-saying the import makes the emitted program internally consistent
but can change the authored program.

**Engineering goal:** represent possible load, exact authored load, and engine-selected
load as distinct states. Only an exact-load proof may silently rewrite an import or mint
speaker/custody attribution. A possible singleton may still support diagnostics, conservative
acquisition, or an explicit user choice.

**Design question:** what witness, if any, promotes a possible singleton to exact selection?

## 30Pb:fnd-controller-source-and-target-source-are-distinct

Plain `.` is used for at least two different jobs:

```sh
. ./book-helper.sh       # potentially a controller-side companion to ship
. /etc/os-release       # target-runtime state
```

Path syntax and local file existence do not distinguish those intentions. Automatically
using the controller's copy of a target-runtime path changes the book's meaning.

**Engineering goal:** carry source provenance explicitly. Invocation-admitted controller
sources and their admitted dependency tree may enter the snapshot and artifact; target-runtime
sources remain runtime operations and analysis walls; unresolved sources remain unknown.
Path resolution selects within a source class and never chooses the class.

**Design question:** what ordinary authored or invocation act admits a controller-side plain-sh
source tree without introducing a second configuration language?

## 30Pb:fnd-emission-legality-covers-all-shell-state

Function dominance is necessary but insufficient for hoisting:

```sh
command -v helper >/dev/null 2>&1 || install_helper
. ./oracle.sh                         # defines helper
```

Hoisting `helper` changes the earlier `command -v`. File-level assignments can change earlier
branches. Alias state can change how a moved body parses. Helper/PATH resolution can change how
the same body executes.

**Engineering goal:** every movable emission unit carries the shell-state facts it reads and
writes. A move is legal only when no crossed observation or mutation can distinguish it, the
definition still dominates every use, and its scope/lifetime is unchanged. Otherwise choose
in-place, sink, or refusal.

**Design questions:** are file-level assignments ever hoistable? If a body depends on one, must
the whole closure remain at the source position?

## 30Pb:fnd-emitted-names-need-freshness-and-hygiene

`name_h<digest>` is deterministic, but determinism is not freshness. A generated name can
duplicate another generated name or a book name. Header-only renaming is safe only where Dorc
owns every reference to the renamed definition; helpers whose calls remain in authored bodies
need hygienic reference rewriting or must not be renamed.

**Engineering goal:** mint an emitted name only after proving it is injective over the complete
emitted namespace. Detect collisions and deterministically lengthen or refuse. If the namespace
is open, prefer in-place placement or refusal. Keep header-only renaming limited to definitions
whose references Dorc fully controls.

**Design question:** which dependency classes satisfy the header-only condition, and which
require alpha-renaming or conservative placement?

## 30Pb:fnd-unknown-source-recovery-is-domain-specific

An unresolvable source can change more than function bindings: variables, cwd, options,
positional parameters, aliases, helper/command resolution, traps, and control-flow termination.
A later role definition restores that role's binding; it does not restore every dependency of
its body.

**Engineering goal:** model unknown source as domain-specific shell-state havoc. Later acts
recover only the domains they provably overwrite. A vouch or guard is available only when the
body's complete resolution environment is known to agree between measurement and execution;
otherwise the authored command remains unguarded and runnable.

**Design question:** which shell-state domains participate at v0, and what explicit recovery
acts close each one?

## 30Pb:fnd-reviewed-artifact-is-one-exact-set

The emission planner decides placement, names, import edits, and sidecar bytes. Those decisions
collectively define what the user reviews and what apply must execute. Recording them in
`Plan::decided` is necessary but does not alone identify a multipart artifact set.

**Engineering goal:** derive human and executable forms from one immutable artifact-set value.
Its identity covers every path and byte, source provenance, analysis-relevant policy, target
execution context, generation, and cwd. Apply consumes that exact set and rejects a mismatch;
it never rereads or regenerates a cousin. World freshness remains a separate question.

**Design question:** does `30P` own this canonical artifact-set identity, or reserve a mandatory
identity projection for a separate approval/apply design?

## 30Pb:fnd-path-spelling-resolution-and-content-identity-differ

A source path has several useful identities:

- the spelling the author wrote;
- the path shell semantics resolve at a program point; and
- the exact snapshot bytes/object the controller opened.

Lexically removing `..` is not equivalent to shell/filesystem resolution when an earlier path
component is a symlink. Case-folding and hardlinks provide other examples.

**Engineering goal:** give those identities separate types and consumers. Loading follows shell
resolution; custody and bundling bind the immutable snapshot content/object; diagnostics retain
the authored spelling. No one string is called canonical for all three purposes.

## 30Pb:fnd-glob-order-needs-whole-program-meet

When glob order is unknown, checking whether one name has one defining file is insufficient.
Other members can unset/rebind that name, alter values it reads, change cwd/options, or terminate.

**Engineering goal:** reason over the complete sequence of load-program effects. If runtime order
remains unknown, meet universally over every possible order. If Dorc chooses an order, disclose and
justify that semantic transformation separately. This is a future constraint on the deferred glob
work, not an immediate implementation request.

## 30Pb:fnd-dot-source-remains-an-execution-frame

Dot sourcing shares the caller's shell state, but it is not parser-level text paste. `return`,
errexit context, alias parsing, traps, positional parameters, and state changes make the source
boundary observable.

**Engineering goal:** retain a first-class source-execution frame in the IR. CFG splicing may be
an implementation technique, but every source-boundary observable remains represented. The
single-stream paste path stays a separately measured lowering under its unwelded exclusion set.

## 30Pb:design-sitting-checklist

Before implementation relies on these findings, the human design sitting should explicitly
accept, reject, or replace each proposed goal:

1. possible singleton versus exact source selection;
2. controller-source admission versus target-runtime sourcing;
3. the shell-state effect model for placement;
4. generated-name freshness and the alpha-renaming boundary;
5. domain-specific recovery after unknown source;
6. artifact-set identity ownership;
7. path identity layers;
8. whole-program glob-order semantics; and
9. the source-execution-frame representation.

Silence is not acceptance. Builders should receive only the rulings produced by that sitting,
not this review document as if it were a specification.
