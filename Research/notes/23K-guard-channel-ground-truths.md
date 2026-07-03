# 23K — the guard-channel ground-truths + the sibling-functions direction (human, 2026-07-02)

Banked from the human's written response to `23J` conv-rc-soundness (drafted in a worktree
scratch file, reproduced near-verbatim below, lightly formatted). This is CHARTER-INPUT for
the upcoming oracle↔Dorc interface round — ground-truths and wishes, not yet rulings, except
where marked. The naming discipline in §2 was directed ("propose, then proceed using") and is
IN EFFECT for all conductor writing from this note forward.

## §1 The human's response (near-verbatim)

> To the meta-point, it's clear that there needs to be type/doc discipline around this.
> Repeated conflation of 'convergence-state' and 'RCs read or created at various points' has
> passed under the radar like four separate times now.
>
> Start off by proposing, then proceed using, a new, and strict, naming-discipline — akin to
> the skip-decomposes-to-elide/guard regimen; there also needs to be corresponding encoding
> work to turn that discipline into typechecking and lints, but that's probably deferred for
> this precise design-discussion moment.
>
> Then, to the concrete oracle-contract, compiler/analyzer-behaviour, and applytime-semantic:
> worked backwards.
>
> The only *non*-optional part: for guards, we *have* to produce *a boolean* value, somewhere,
> at the end of all this. That boolean needs to encode *convergence* (in the constrained,
> literal sense stated previously): the run-wishness of the immediate-following command, a
> byte-for-byte line from the runbook. "falseness" → unconvergence → run it; "truthness" →
> convergence → runtime-skip.
>
> All else is, to some degree or another, mutable: we can control the oracle contract, we can
> control what we compile out of the probe (depending on what we unweld), we can control all
> the other channels and lanes of communication.
>
> *To get* the semantic value of that, to *know* whether to run or not at that moment, we need
> work from the oracle author: thus, a second uncontrovertible truth, we *must* ship
> *something* from the oracle's work to the host. Behaviour, not static information — the
> 'is this converged' may be a complex *predicate* on system-state, that we (Dorc) can't know
> ahead-of-time.
>
> The mutable wishes:
> - we don't *want* to manipulate code — expensive, hard to do correctly, subtle, bug-prone,
>   surprising.
> - we don't *want* to multiplex/demux from information channels, especially multiple times
>   (i.e. collapse information into rc.)
> - we don't *want* to ship complex non-oracle behaviour to the hosts (prior art in the ocean;
>   we deliberately strayed back from a powerful on-host 'executor'.)
>
> And the moving parts needed for implementation reasons:
> - probe-time information beyond "is this converged" must travel somewhere (collect + display);
> - we want to *replace* the candidate-command inline with a "predicted rc" in the
>   observed-skip case; and
> - we want to *guard* the candidate-command with an "is this converged" boolean.
>
> The don't-multiplex wish is the argument held strongest *against* the 'safe' approach of
> "fully detangle all usage of rc" (concretely: lifting into a richer communication substrate,
> like a stdout string-compare). While probably the *best* choice for correctness, it drives
> too hard against the other goals — it's, frankly, ugly. Exploring the alternative space:
>
> Maybe this is a good indicator that it's time to **re-enrich the oracle language to "more
> than just one function."** Not the accidental probe-vs-oracle split just removed, but
> something space has been reserved for this entire time: *siblings* to `foo.predict()` that
> perform other tasks than checking. The author has the full richness of turing-complete sh to
> D.R.Y./dedupe them. Perhaps a *rich* oracle consists of both `foo.predict()` (only invoked
> in rc-insertion-position) and `foo.converged()` (invoked to decide elision-potential, and
> further inlined as a local guard). Stuff to work out re: performance and information-sharing
> (a single function branching on an environment-variable was considered and disliked — less
> "spelled-as-sh" — whereas `foo_predict` and `foo_is_converged` are reasonable, valuable
> shell-script functions on their own, outside Dorc, that we strongly-contract with specific
> demands on shape and outputs).

## §2 The rc/verdict naming discipline (proposed per §1's directive; IN EFFECT for docs/notes/briefs)

Parallel to skip→{elide, guard}: the old blurry words decompose, and no term crosses lanes.

- **tool-rc** — the raw exit status of a tool, read only INSIDE oracle bodies. Opaque to Dorc,
  always. (`dpkg-query`'s rc IS the measurement; that stays idiomatic sh.)
- **probe-report** — everything the probe lane carries back: facts, observables, refusals,
  can't-tells. Travels the OOB lane, never in-band.
- **plan-verdict** — engine-side, per-site, TERNARY {converged, diverged, can't-tell}; derived
  at plan time from the probe-report via the reached marks. Exists only where the engine is.
- **guard-verdict** — host-side, apply-time, the ONE authored BOOLEAN of §1: truth ⇒ converged
  ⇒ runtime-skip; false ⇒ run the byte-for-byte line. Its transport channel is the interface
  round's central open question. There is no can't-tell at the host: everything non-truth runs.
- **predicted-rc** — a probe-sourced replacement VALUE emitted in an elided command's
  rc-position (`inv-probe-sourced-values` machinery). It is DATA (output-reproduction), never
  a decision. NB: the human's `foo.predict()` sibling would be the FIRST sanctioned instance
  of the invariant's reserved carve-out ("an oracle-declared fact the human has explicitly
  sanctioned — none currently exist").
- **apply-rc** — the exit status MINTED FROM a guard-verdict to drive the artifact's sh
  connectives (`||`-fails-toward-run, the subshell machinery): sh's conditionals eat exit
  codes, so this mint necessarily exists — the substrate demands it. SETTLED (human,
  2026-07-02): it happens **once, in a controlled way** — exactly one design-sanctioned
  mechanism, never ad-hoc rc-punning. OPEN (the round's question): who owns the mint —
  oracle-contract discipline (`foo_is_converged`'s own rc IS the mint), an interposed shipped
  helper, or true cross-compilation of oracle bodies (in visible tension with
  never-engine-synthesized-sh). Distinct from predicted-rc's replacement-mint: two different
  rc-emissions, two licenses, never conflated.
- Bare "rc" is a banned word in design text (like bare "skip"): always qualify (tool-rc /
  predicted-rc / apply-rc). Verdicts are never rc's. There are exactly TWO blessed crossings
  between the rc-world and the verdict-world, one per direction: tool-rc → verdict (authored,
  in exactly ONE place per oracle — today the predict body's path; post-round perhaps
  `foo_is_converged`'s body) and verdict → apply-rc (minted once, per the bullet above).
  Everywhere else their non-correlation is an invariant, to be ENCODED as types/lints during
  the build (deferred from this design moment, owed).

## §3 Fences for the round (conductor, so the siblings don't rot into old mistakes)

- **sibling ≠ st-2.** The retired probe/check split filed MEASUREMENT per (kind, selector) —
  engine-keyed, engine-resolved. The sibling family splits by authored ROLE (decide / predict
  / check), each a whole authored function invoked with the site's argv, still
  predict-is-the-oracle in spirit: the author writes behaviour, the engine never synthesizes or
  files it. Any drift back toward per-kind filing re-opens the jc-fblessed grave.
- The guard-verdict channel question (how `foo_is_converged`'s boolean reaches the glue
  without re-multiplexing through an unqualified exit status — the broken-path/127 conflation
  from `23J` applies to ANY single-function-rc scheme) is explicitly NOT settled here; §1's
  wishes constrain it, the round decides it.
- The round's charter (working name: **the oracle↔Dorc interface round**): guard-verdict
  transport · the vouch spelling (dq-kOOB cluster) · the sibling-function family shape ·
  the rc/verdict discipline's type-encoding · disposition of the two deferred rc-soundness
  pins from `23J` (they respell against whatever channel wins). Seeds: this note + `23J`
  conv-rc-soundness + the two in-flight intuitions (no-rc-at-the-interface; minted
  ProbeRc/GuardRc types, one blessed interpretation site).
