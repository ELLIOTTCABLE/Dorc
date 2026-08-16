> Sol adversarial minispec review; reviewed tip d31378e8. Codex exit: 0.

Static review only: I did not execute `lake`, Lean, or Kani.

The neutral and hostile passes converged on several real failures. The strongest is not hypothetical: the committed corpus is already internally stale.

## Findings

1. +SURE — the promoted Lean corpus is presently broken/stale, and the ordinary gate cannot detect it.

`Minispec.lean` imports nonexistent `Minispec.LeqIsReflexive` and omits `JoinIsAssociative` ([Minispec.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec.lean:11)). The directory instead contains `JoinIsAssociative.lean`, and the catalogue promotes it as elaborated/interrogated ([catalogue_lock.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/catalogue_lock.rs:15), [REPORT.md](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/REPORT.md:11)).

The cheap gate deliberately returns `NotAtThisTier` for those badges ([evidence.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/evidence.rs:67)), and `NotAtThisTier` agrees with every promoted expectation ([badge.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/badge.rs:133)). Lean is explicitly opt-in, outside the ordinary gate ([mise.toml](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/mise.toml:511), [mise.toml](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/mise.toml:569)).

Thus the committed report can retain `promoted: earned` after the corpus stops building. An explicit engine-tier run should expose this particular rot, but the advertised everyday ratchet has demonstrably not done so.

2. +SURE — the catalogue seat is misdirected: it does not bind the laws to `Flat<T>::join`.

All three laws invoke the translated `Flat<T>` implementation specifically ([JoinIsCommutative.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/JoinIsCommutative.lean:29), [JoinIsIdempotent.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/JoinIsIdempotent.lean:27), [JoinIsAssociative.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/JoinIsAssociative.lean:32)). That generated definition corresponds to Rust’s `impl Lattice for Flat<T>` body ([Funs.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Generated/Funs.lean:706), [lattice.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/crates/analysis/src/lattice.rs:143)).

The catalogue instead names the generic trait seat `dorc_analysis::lattice::Lattice::join` ([catalogue_lock.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/catalogue_lock.rs:16)). The resolver discards intermediate path segments and merely finds any `fn join` in `lattice.rs`; it explicitly treats a trait signature and implementation as interchangeable ([seat.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/seat.rs:28), [seat.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/seat.rs:72)).

Consequences:

- Removing or renaming `Flat::join` need not invalidate the seat while the trait’s `fn join` remains.
- A broken new `Lattice` implementation remains inside what the report appears to call the verified `Lattice::join` boundary, although none of these laws constrain it.
- Production `solve` calls `join` for arbitrary `L: Lattice`, not only `Flat` ([solve.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/crates/analysis/src/solve.rs:114), [solve.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/crates/analysis/src/solve.rs:157)).

The laws are not decorative with respect to `Flat::join`; the catalogue’s claimed linkage and boundary are.

3. +SURE — `interrogated` can be earned by an irrelevant, vacuous battery.

The unit reader checks only:

- whether the text contains `theorem <Slug>_nonvacuous`;
- whether any line begins `example` or `#guard`.

It does not inspect theorem types, hypotheses, constructors, or relevance to the law ([unit.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/unit.rs:152)). `interrogated` then requires only those syntactic flags plus a globally successful Lean build ([evidence.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/evidence.rs:148)).

This would qualify:

```lean
theorem JoinIsCommutative_nonvacuous : True := by trivial
example : True := by trivial
```

No mechanism verifies the claimed `Bottom · Elem · Top` coverage or that the “nonvacuity” theorem exercises a law precondition. The current batteries themselves are concrete and nontrivial; the badge definition does not enforce that property.

4. +SURE — `proved` can be claimed without proving the catalogued proposition.

`proved` is classified as not requiring an external engine ([badge.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/badge.rs:60)). It is earned when a claimed file:

- exists;
- contains the substring `theorem <Slug>_holds`;
- contains none of three lexical hole spellings.

It never checks the theorem’s type ([evidence.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/evidence.rs:131)).

Therefore this can earn `proved`:

```lean
theorem JoinIsCommutative_holds : True := by trivial
```

Even a non-elaborating file can obtain the cheap syntactic verdict. A separate Lean failure may make `elaborated` absent, but badges are independent cross-states, so that does not retract `proved`.

5. +SURE — the advertised promote machinery does not exist.

The lock says it was generated by `dorc-verify promote`, with expectations computed at promotion ([catalogue_lock.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/catalogue_lock.rs:1)). The CLI exposes only `check`, `report`, `materialize`, `lean-build`, and `kani` ([dorc-verify.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/bin/dorc-verify.rs:16)). The verify task block likewise contains no promotion task ([mise.toml](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/mise.toml:522)).

The current “generated, promote-gated” catalogue is therefore an ordinary editable Rust constant. Nothing mechanically establishes that its seats, proof paths, harness names, bindings, or expected badges were computed by an authorized promotion act.

Engine-tier mismatch checking is real ([dorc-verify.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/bin/dorc-verify.rs:185)), but that is not equivalent to the claimed promote ceremony.

6. +SURE — the trusted-base census has material blind spots.

The generated-tree walker reads only `.lean` files directly inside `Generated/`, not nested directories ([pipeline.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/pipeline.rs:120)). Its axiom count recognizes only lines beginning exactly `axiom ` after whitespace ([pipeline.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/pipeline.rs:89)). Legal multiline declarations and other trust-introducing forms are outside that census.

More importantly, the implementation acknowledges that the imported Aeneas dependency contains holes capable of invalidating downstream proofs ([pipeline.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/pipeline.rs:173)). These are counted only from build-warning text and printed transiently ([dorc-verify.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/bin/dorc-verify.rs:94)); they neither prevent `elaborated`/`interrogated` from being earned nor appear in the committed report’s “trusted base” ([REPORT.md](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/REPORT.md:65)).

So “Generated holes: 0; external axioms: 13” is not a complete visible trusted base.

7. +SURE — the generic law assumptions are substantially stronger than Rust’s live trait bounds.

The Rust implementation accepts every `T: Clone + Eq` ([lattice.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/crates/analysis/src/lattice.rs:137), [lattice.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/crates/analysis/src/lattice.rs:143)). Lean additionally assumes that cloning is exactly identity and equality exactly decides propositional equality ([TrustedBase.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/Vocabulary/TrustedBase.lean:29), [TrustedBase.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/Vocabulary/TrustedBase.lean:35)).

Rust’s type system does not enforce those semantic laws for arbitrary user implementations of `Clone`, `PartialEq`, or marker trait `Eq`. The predicates are visible in each formal statement, so this is not a hidden Lean premise. The misleading part is TrustedBase’s claim that the Rust compiler and derive machinery “keep enforcing” these properties over the generic shipping implementation ([TrustedBase.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/Vocabulary/TrustedBase.lean:6)). Unless every actual `Flat<T>` instantiation is separately restricted to vetted derived dictionaries, the theorem covers a proper subset of the live Rust implementation’s accepted types.

8. ~SUSPECT — the external-axiom prose overstates auditability.

The report claims every axiom has a fence entry carrying its class and reason ([REPORT.md](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/REPORT.md:68)). The fence supplies one category-level rationale followed by five patterns, which generate thirteen reported axioms ([Cargo.toml](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/aeneas/Cargo.toml:26)). The individual axiom names are visible in generated files, but there is no per-axiom mapping to a specific fence pattern and reason.

## Attacks that did not hold

- +SURE — the generated `Flat::join` is not a hand-maintained copy. The translation crate directly `#[path]`-includes the live Rust `sorted.rs` and `lattice.rs` ([lib.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/aeneas/src/lib.rs:21)). If translation is actually rerun, it operates on live source.

- +SURE — the present three propositions are not logically vacuous implications. They quantify explicit dictionaries and require substantive `LawfulClone` and `LawfulEq` premises before asserting join equality ([JoinIsCommutative.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/JoinIsCommutative.lean:29), [TrustedBase.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/Vocabulary/TrustedBase.lean:29)). The concrete `u32` premises are inhabited and proven directly ([TrustedBase.lean](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/minispec/Minispec/Vocabulary/TrustedBase.lean:43)).

- +SURE — ordinary `sorry`, `sorryAx`, and `admit` holes in units, governed vocabulary, and current direct Generated files are conservatively detected ([unit.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/unit.rs:80), [check.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/check.rs:57)).

- +SURE — a binding cannot currently earn `demonstrated` merely by existing. Every nonempty binding is still refused because assertion-subset verification is explicitly unbuilt ([evidence.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/evidence.rs:168)). Proposal/catalogue presence is also checked bidirectionally ([binding.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/binding.rs:127)).

- +SURE — Kani pins are not currently being overclaimed. All catalogue harnesses are `None` and all `pinned` badges are `todo` ([catalogue_lock.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/catalogue_lock.rs:18)). When enabled, pinning requires both tool-reported name resolution and a green result ([evidence.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/src/evidence.rs:108)).

- +SURE — `solve.rs` itself is not falsely claimed as translated or formally proved. The translation unit explicitly excludes it ([lib.rs](C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a2ec4235117b360ae/spike/verify/aeneas/src/lib.rs:11)). The overclaim is instead the catalogue/report’s generic `Lattice::join` boundary wording.

The most consequential repair order appears to be: fix the already-stale corpus root; make the seat identify `Flat<T>::join` rather than a textual trait method; make `interrogated` and `proved` semantic engine-backed checks; then either implement the claimed promotion transaction or remove every assertion that such a transaction currently protects the lock.