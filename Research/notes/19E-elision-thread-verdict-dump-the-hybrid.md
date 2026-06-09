# 19E — The elision/rc thread's verdict: the Verdict+bolted-rc hybrid is the wrong shape (dumped)

> Orchestrator round-finding; human-directed ("dump it and move on — no useless make-work"). Terse;
> caps the `19C`/`19D` elision thread. Trust R/D/I/K + `19A §5`/`19B` over this.

- **The fold *direction* is RIGHT** (`19A §5`): the apply phase abstract-interprets the CFG over each
  command's *observables*. build-1 (`19C`) built it; it is sound. Not dumped.
- **The rc-*sourcing* is the wrong shape (dumped).** The spike sources observables as a keystone
  `Verdict{Converged/Diverged/Unknown}` (about a state-cell) with build-1's `Option<rc>` **bolted on as a
  separate field** (`Observed{verdict, rc}`). Two separately-sourced things ⇒ the state
  "verdict=Converged, rc=absent" exists ⇒ the CLI/hostsim defaulted that absent rc to `0` ⇒ **strain-B**,
  the live priority-1 under-execute (a non-conforming establish like `useradd` on a `|| fallback`).
- **The cure = the contract we already agreed (`19B`): the check is an observable-PREDICTOR.**
  `mycmd.check()` predicts the command's whole observable-tuple `{effect, rc, stdout, stderr}` for given
  inputs, atomically — or fails loudly OOB ⇒ unknown ⇒ run. There is no separable "converged-without-rc,"
  so strain-B *cannot arise*. **"Convergence" is then the engine's *derived* effect-state** (the check's
  `effect`-observable being no-mutation, refined by the ambient gate for in-script staleness) — **not
  something the check reports.** `Verdict{Converged}` is merely a *name* for that derived state; the bug
  was sourcing the rc *beside* it instead of *from the same prediction*.
- **The rc-fix (`19D`, committed `930dd59..2b620fc`) is a sound safety-floor on the dumped hybrid.** Both
  crosscheck passes (neutral + adversarial, un-seeded) **converged**: it closes the under-execute (an
  undeclared rc ⇒ ⊤ ⇒ run); errexit-vouching is an **over-execute** (priority-2), *not* an under-execute
  (proven by executing `set -e; useradd[exit 9]; mkdir` under `dash`). It stands as the floor. Per the
  human, we are **not** investing further in patching/validating the hybrid.
- **Hand-forward:** realizing the cure is build-2 — the observable-predictor `.check()` producing the
  atomic tuple (or OOB-failing), which dissolves the verdict/rc seam at the root. Until then, a
  branch-consumed converged establish elides only with a *declared* rc (the floor). The render-layer
  retirement of the `if`/`elif` `Status` floor still waits on the leaf-exact render (`C-5`).
