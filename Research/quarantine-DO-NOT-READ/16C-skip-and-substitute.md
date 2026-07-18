# 16C — skip-and-substitute: the output-dependency contract, threaded to the oracle

> **Status (2026-06-05): spike, design round — the leading design-surfacing of the
> spike (the human's words).** Surfaced by find-cli-1 (note 16B/169): eliding a
> command whose **output is consumed** (`cid=$(docker run …)`, a pipeline stage, a
> status branched-on) can't be done by line-omission — the rest of the script needs
> the value/status. An adversarial pair pressure-tested the analyzer-side
> "synthesize the value" design; the human then reframed it as an **oracle-declared
> probe→apply bridge**. This note records both + the synthesis. The bridge itself is
> **OUTSIDE-THE-SPIKE** (human's call); the *floor* is in-spike. Append-only (round
> 16: …16B → 16C). Confidence-marked.

## 0. The axis (justified)
"Skip" must preserve a leaf's interface to the rest of the script: its stdout, its
exit status, its side-effects. Side-effects are what we elide (converged). So skip
must preserve whatever stdout/status downstream consumes. A top-level `;`-sequenced
command consumes *nothing* (stdout discarded; status asserted-zero under `set -e`,
which a converged command satisfies) ⇒ skip = omit the line. A command whose output
IS consumed (subst-capture `x=$(cmd)`, pipeline stage `cmd | …`, status guard
`if cmd`) ⇒ omission breaks downstream ⇒ skip must **synthesize** the output. So the
real axis is **output-consumption**, not top-level-vs-subst. Omission is the
degenerate skip; "synthesize" is the general case (dn-3: "the probe projection is a
leaf-id-preserving rewrite" — omission is the rewrite that deletes a node nothing
reads).

## 1. Adversarial verdict — ANALYZER-side value-synthesis is unsound (refuted)
A clean-context neutral+adversarial pair re-read the corpus + code. **Both converged:
every soundness lock in the kernel is FACT-shaped** (a boolean predicate over
`{kind, entity}`); a skip is mintable only from `EstablishAmbient ∧ Must ∧
Converged`. Skip-and-substitute bolts a **value** plane onto a kernel that has no
value representation at all (`$cid`/`$?` are opaque `Param` tokens; W2 ⊤-approximate
aliasing; 163 §5 excludes precise value-flow). So the locks don't transfer. Findings,
worst-first, with survivor-verification (my judgment in-union):

- **brk-1 (SURVIVES → transferred): the synthesized value is run-derived / wrong.**
  `docker run` mints a *new* id; the converged host holds a *prior* container's id
  (and short-vs-long-id differs). The fact "container:web holds" does not entail
  "the value a fresh run would emit." `kVOLATILES` (welded) says non-determinism
  breaks a sound skip. → The analyzer cannot know the value; **the oracle author can**
  (§2). Transferred, not eliminated.
- **brk-6 (PARTLY DEFUSED): temporal-meaning hazard.** `old=$(readlink current); ln
  -sfn v2 current; rm -rf "$old"` — synthesizing `old` from the *converged end-state*
  deletes the live release. BUT: `readlink` establishes no fact ⇒ Opaque ⇒ MustRun
  (not a skip candidate). The fact-gate (only establishing mutators are skippable;
  pre-reads run) defuses the example; the general "captured value's meaning is
  temporal" hazard is rare for *mutating* captures (whose value is the post-state the
  converged probe gives) and is author-handled.
- **brk-2 (SURVIVES → confined): read-only-ness of the value-probe is not
  frame-enforceable** (162 O-2, verbatim, re-armed for values: a `oracle_value(){ …
  || docker run …; }` mutates). → Confined by the §2 gather/compute split (only the
  *gather* touches the host; same kFAIL-withhold locus as the fact-probe — no wider
  surface).
- **brk-9 (SURVIVES → prerequisite): no substrate for the live-splice.**
  `parse_subst_body` re-lexes `$()` from offset 0, so subst-body spans are
  substring-relative (the find-cli-1 garbage). Splicing a bridge into a `$()` needs
  the subst's source offset recorded (note 169 names the fix). Bounded prerequisite.
- **brk-5 (SURVIVES → deferred): captured values are the primary cross-host
  courier** (`token=$(swarm join-token); ssh b "join $token"`). A gather on host B ≠
  host A's value. → host-local bridges only; cross-host captured values fold to
  MustRun (the cross-host-kind layer is already deferred, 16A §2).
- **brk-3 / brk-8 (SURVIVE → conservative rule): `Owes` is a half-model of
  value-flow** (alias `y=$cid`, `$?` later, `eval`, `tee`, `>&3`); the easy answer
  ("found no consumer") is the *unsafe* default (→ Omit). → `Owes` must be a lattice
  that **rounds toward more-owed** (`Output ⊒ Status ⊒ Nothing`); unknown ⇒ `Output`
  ⇒ needs a bridge ⇒ none ⇒ MustRun (inv-top-reject direction).
- **brk-4 (SURVIVES → carve-out): the guard-carrier collision.** `if docker inspect
  web; then …` — the command *is* the read-only probe; "substituting" it is nonsense
  and risks branch-inversion. → A read-only leaf in condition position is never a
  substitute candidate (160 §5 `pj-guard-purity-precondition`; the kernel already
  elides the *establish*, not the guard).
- **brk-7 (PARTIAL): `set -e`/`pipefail`** — synthesizing rc from a probe *pipeline*
  with different pipefail posture than the original. Mostly handled by "converged ⇒
  rc 0" (below); flagged.

**Adversarial bottom line:** *never let the analyzer synthesize; a consumed
producer is MustRun* (the floor, applied to all `Owes` variants). The neutral
independently judged the analyzer-safe value-set "nearly vacuous" for the ops
domain. **The floor is correct and is the in-spike posture.**

## 2. The human's reframing — thread synthesis to the ORACLE (the rescue)
The refutation kills *analyzer-invented* values. It does NOT kill *author-declared*
ones — that is the Dorc thesis (DESIGN: "we export correctness-claims onto the
oracle-writer… transfer-to-contract, never eliminate", 162 O-2). The human's
decomposition (recorded verbatim, for outside-the-spike), a (gather@probe,
compute@apply) pair × {stdout, rc}:

1. *probe-phase:* generate what apply will need to allow elision **where stdout is
   depended on**.
2. *probe-phase:* generate what apply will need **where rc is depended on** (≈ the
   fact-probe/verdict — the only one analyzed deeply so far).
3. *apply-phase:* cheaply compute the **output** this would generate, given `x` from
   the probe.
4. *apply-phase:* cheaply compute the **rc** this would generate, given `x` from the
   probe.

**My synthesis — why this is the rescue, and a soundness IMPROVEMENT over the
value-probe:**
- It converts every brk-1/brk-5/brk-6 "the analyzer can't know the value" into
  "the *author* declares it, or declines → MustRun." Same posture as the fact-probe;
  the floor is the safety net. This is the only sound home for the feature.
- **The gather/compute split is not mere reorg — it localizes the hazards.** Today's
  "value-probe" conflated reading-the-host with producing-the-value. Splitting them:
  the **gather** (#1/#2) is read-only and is the *only* host-touching part ⇒ the
  kFAIL-withhold check (hostsim now, seccomp later) applies to it alone, same locus
  as the fact-probe (kills brk-2's "wider surface"). The **compute** (#3/#4) is a
  **pure function of `x`** — constrain it to pure sh builtins (whitelist-checkable,
  the "declared-inert ops" idea of 162 O-2) ⇒ it cannot mutate and is deterministic
  (volatility is snapshotted into `x` at gather-time; re-gather-before-apply, 165 L4,
  closes drift). So `kVOLATILES` is satisfied by construction: synth = pure(gathered).
- #2/#4 (rc) is *mostly the existing machinery*: gather = fact-probe; compute =
  "Converged ⇒ rc 0" (a converged command would have succeeded). #4's "given `x`"
  hook is for tools with richer rc semantics; default trivial. #1/#3 (stdout) is the
  genuinely new capability.
- debconf precedent (162): config (no-fs-mutation, gather) / postinst (act) splits
  exactly this way — the split is idiomatic, not invented.

## 3. The brutally-typed shape (outside-the-spike target)
- `Owes` lattice in `analysis` (`Nothing ⊏ Status ⊏ Output`), transfer **joins
  upward**; unknown var-flow ⇒ `Output` (inv-top-reject direction). Computed by a
  small, conservative use-analysis; precision bounded by how much copy-prop is built
  (measure the fire-rate — 162 O-3 stance — don't assume).
- `Discharge` witness in `plan` (mirrors `SkipLicense`, 165 L2): `Omit` (Nothing) |
  `SynthStatus` (Status) | `SynthOutput(Bridge)` (Output, CapturedScalar only). The
  type forbids `Omit` for an `Output` leaf AND forbids minting any non-`Omit`
  discharge without an oracle `Bridge`.
- Oracle contract gains a **`Bridge { gather: <read-only sh>, compute: <pure sh> }`**
  per kind, lifted like `FactProbe`. Grade::Must. No bridge ⇒ no discharge ⇒ MustRun.
- Restrictions baked in: CapturedScalar only (no pipeline *streams* — a stream is
  transient/un-probeable, 160 §7); read-only guard-carriers excluded (brk-4);
  multi-entity/partial-convergence folds to MustRun (162 O-4); host-local only
  (brk-5); requires the subst-offset fix (brk-9) before any splice.

## 4. Spike posture (what to do IN the spike vs OUTSIDE)
- **IN-SPIKE (the floor, sound today):** an output-consumed leaf is **MustRun**.
  This is what find-cli-1's exclusion *approximates* for the `$()` case — but note
  the **framing correction**: subst-internal commands **are leaves** (the human was
  right; `id=$(docker run)` is a real, liftable, eventually-skippable unit). 16B's
  "not a leaf" was too strong; the exclusion is a *temporary flat-plan
  floor-approximation* (they stay analyzed; they're MustRun-equivalent), pending the
  bridge + offset-fix + nesting-aware plan. **Latent gap the floor should also
  close:** a pipeline-stage *establishing mutator* (`apt-get install x | tee log`)
  is currently classifiable EstablishAmbient ⇒ a skip would omit it and break the
  pipe — the `Owes`-floor (consumed ⇒ MustRun) fixes this; worth a regression test.
- **OUTSIDE-THE-SPIKE (the human's enrichment, this note's §2/§3):** the
  oracle-declared gather/compute Bridge. A substantial dn-1 contract extension +
  the `Owes` analysis + the subst-offset substrate. Sound, viable, high-value, but a
  design-corpus item, not a spike-implementation task right now.

## 5. The standing tension (for the human)
Even threaded to the oracle, this is the most "magic" feature yet (DESIGN priority 4)
vs simplicity/validation (code priorities 2/3): the preconditions (CapturedScalar ∧
Must ∧ read-only-gather ∧ pure-compute ∧ host-local ∧ ambient ∧ offset-fixed) are
many, and the canonical demo (`cid=$(docker run)`) only works in the converged
branch with an `inspect` gather. The floor concedes only *performance* (kPRECISION),
never correctness. So the go/no-go for building the bridge (outside the spike) is:
is the recovered skip-rate worth a dn-1 contract this large? — a taste/ops-experience
call, parked for the human (like the kDEPS band).

**NOTES INDEX:** …169 cli capstone · 16A apply+multi-host direction · 16B leaf-seam
scope · 16C (this — skip-and-substitute / output-dependency contract).
