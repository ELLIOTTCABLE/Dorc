# 164 — spike checkpoint + resume-point (token-budget boundary)

> **Status (2026-06-05): checkpoint at commit `435abb7` (#14), branch
> `ai/spike-impl`, worktree `.claude/worktrees/spike-impl`.** Written because the
> session is near a token limit. This is the authoritative resume-point: what's
> built, what the adversarial passes found, the surfaced design problems (the
> spike's actual deliverable), and the ordered next steps.

## 1. Built + committed (cargo workspace under `spike/`, gnu toolchain via mise)
- `dorc-core` — vocabulary: newtype ids, `Span`, no-throw `Carrier<T>` (dn-7),
  deterministic `Interner`, referent-agnostic `OpaqueToken`/`KindId`/`ProviderId`
  (W4), `Phase`/`Verdict`/`Grade`/`Fact` — all `Ord`/`Hash` for lattice use.
- `dorc-syntax` — hand-rolled pure-Rust lexer + recursive-descent parser (NO
  tree-sitter/C; this box has no C compiler). Arena AST, lossless quoting
  (`Word::may_split`/`as_literal`), first-class redirections, `Unsupported`=⊤,
  total/no-throw (caught + fixed a backtick infinite-loop and unbounded recursion).
- `dorc-analysis::lattice` — generic `Lattice` trait + combinators
  Powerset/Flat/Product/MapL (MapL canonical: no-⊥ entries). SPA §4.3.
- `dorc-analysis::solve` — propagation worklist (SPA §5.10), generic over
  `Graph`+`Lattice`+`Direction` (forward AND backward — the human's day-1 req).
- `dorc-analysis::cfg` — Ast→Cfg (impl `Graph`); `CfgNodeKind`={Entry,Exit,
  Command,Redir,Merge,ScopeEnter,ScopeExit,Top}; coarse-but-sound `set -e`
  errexit fixpoint → failure-edges; subshell scope boundaries; terminating
  `exit`; `Unsupported`→`Top`. **Built by subagent; green+clippy-clean but NOT
  yet adversarially-checked / deeply reviewed by me** (do that next).

## 2. Design-note index (`Research/notes/`)
160 analyzer-chord synthesis (build-2 scope) · 161 dn-1 strawman (ph-1) · **162
dn-1 adversarial reconciliation → the fact-centric pivot (most important)** · 163
SPA-grounded engine design · 164 this checkpoint. (Human docs README/DESIGN/
KNOBS/TODO + the worktree `spike/CLAUDE.md` invariants are the ground truth.)

## 3. Framework adversarial findings (web-grounded critic) — DEFERRED FIX-SET
The monotone framework (`lattice`+`solve`) is textbook-correct *for the may/
⊥-start/monotone/fixed-finite-key shape it assumes*. The holes are all at the
**un-type-enforceable contract boundary** — and hole-1 is **empirically
confirmed** (the critic's scratch spun two `solve`-with-non-monotone-transfer
binaries to 435 & 783 CPU-sec before I killed them):
- **fix-1 (do first):** `solve` has no iteration cap + can't enforce
  monotonicity → a non-monotone transfer **loops forever**. Add a `Solution{
  states, converged }` return + a generous iteration cap → loud non-convergence,
  never a hang (inv-no-throw spirit). Add a test feeding a non-monotone transfer.
- **fix-2:** `MapL` finite-height holds **only for a bounded key set** (the
  trait's blanket "finite height" is over-stated). Dorc's keys are bounded (a
  script's finite literal tokens); document the precondition; the cap backstops it.
- **fix-3:** `solve` indexes `state[w]` from graph edges → a malformed `Graph`
  (endpoint ≥ node_count) **panics**. Document the well-formedness contract +
  `debug_assert`; the cfg builder is the trusted producer.
- **fix-4/5:** document that a `Lattice` impl's `Eq` must be semantic (holds for
  the combinators); and that ⊥-init for unreachable nodes is may/forward-correct
  but the backward/`kFAIL-perform` boundary (⊤-vs-⊥) is per-analysis.
The earlier (clean-context, web-grounded) audit text lives in this session's
transcript; the actionable set is the five fixes above.

**Both web-grounded critics (neutral + adversarial) converged, and the
adversarial half reproduced each break against the committed crate.** Precision
they add: (a) the worklist propagation AND `MapL` canonicalization are *confirmed
sound* (a 7-node multi-cycle reaches the true fixed point; no ⊥-leak) — do NOT
rewrite the engine, only add the cap + contracts; (b) a bad transfer has TWO
failure modes — an infinite hang (non-idempotent / non-monotone-toggle, the
empirically-spinning case) OR a *silently wrong result* (returns `x = x ⊔ f(x)`
rather than the equation's `x = f(x)`; no panic, so inv-no-throw is only
*vacuously* met) — the cap catches the hang, the silent-wrong-result needs the
monotonicity contract documented + law-tested; (c) the false "finite height"
claim applies to `Powerset<unbounded T>` too, not just `MapL` (any transfer that
mints a fresh element/key each visit climbs forever) — latent only because the
current fact vocab is `u32`-interned-from-source (finite per input); it bites the
first time a transfer synthesizes fresh facts mid-fixpoint; (d) `cfg.rs` already
self-checks graph consistency (`consistent()`), so the OOB panic is reachable
only from a *buggy* Graph producer — still, `solve` should `debug_assert` it.
Nit: `leq`'s default clones a full `join` to compare (O(n) perf footgun) — worth
a per-combinator override if it ever matters.

## 4. Surfaced DESIGN PROBLEMS for the planning corpus (the spike's deliverable)
- **DP-1 (the hinge, note 162):** dn-1's first strawman was *command-centric*
  (dry-run the mutator, `apt-get --simulate|grep`). The adversarial cross-check
  proved it solves the WRONG problem: the named-kind index (the dn-1 deliverable
  four rounds converged on) is **decorative** in that skip path. The hinge needs
  a **fact-centric** contract: oracle declares a *named kind* + a read-only
  *fact-probe* (`dpkg-query`, three-outcome 0/1/2) + an *effect-map*
  `(provider,verb)→establish|kill`. Open sub-problems: the effect-decl sh
  spelling (clobber-vs-no-op-shim), entity-extraction (sound-XOR-useful), the
  **kOOB ruling** (is `oracle_kind=package` config-in-disguise? — needs the
  human), and kFAIL-withhold enforcement (below).
- **DP-2 (note 163 + empirical):** the analysis engine's correctness rests on
  un-type-enforceable contracts (monotonicity, finite-height, semantic-Eq). A
  violation **hangs** (proven). Lesson: "TypeScript not Coq" (`kVERIFY`) means the
  unprovable invariants need DST/test backstops + loud-fail, not doc-comments alone.
- **DP-3 (note 162 F-1):** the verdict channel cannot emit three-valued
  {converged/diverged/unknown} via the canonical `cmd|grep -q` idiom (grep's
  no-match rc ≡ tool-failure rc; `pipefail` doesn't fix the `&&return1;return0`
  shape). Oracles need a probe *shape* that captures the tool's own rc.
- **DP-4 (note 162 O-2):** `kFAIL-withhold` (probe never mutates) is **NOT**
  enforceable by the contract frame — a frame-clean oracle can ship a mutating
  probe (`docker create` as "probe"; apt `-o` re-arming `--simulate`). Needs a
  SEPARATE mechanism (prove probe calls only declared-inert ops, or sandbox/
  observe — 077 seccomp). The `hostsim` can DST-detect a probe attempting a
  modeled mutation. Build it separately from the verdict channel.
- **DP-5 (note 162 O-3):** entity-extraction is sound-XOR-useful — ⊤-on-unknown-
  flag is safe but ⊤s the common idiomatic lines (gutting the value-prop);
  per-provider flag grammar fixes precision but re-arms the `-o` hazard and is a
  real authoring burden (contradicts "oracles are dumb one-liners").
- **DP-6 (positive, dd-2/note 163):** the analyzer kernel is pure + synchronous
  by construction → it **sidesteps the dn-8 async-vs-state-machine kernel fork
  entirely** (async only enters at the executor, which is mocked). DST-friendly
  with no DI ceremony. A genuine de-risk of a "decide-now retrofit-hostile" fork.
- **DP-7 (note 162 O-1 / now buildable):** the ambient∧invariant W5 gate (don't
  skip a mutation whose fact is killed upstream in-script — `purge X; …; install
  X`) is the ANALYZER's job (reaching-defs over the effect-map), not the oracle's;
  the dn-1 §2 wrongly stated the skip unconditionally. The CFG + effect-map now
  supply the gen/kill the gate needs.

## 5. Next steps (ordered)
1. **Apply the §3 framework fix-set** (Solution+cap, debug-assert edges, document
   preconditions, non-monotone + malformed-graph tests).
2. **Review + adversarially-check `cfg.rs`** (the `set -e` coarse modeling +
   scope-boundary semantics are the soundness-bearing parts). Open Q from the CFG
   subagent: pipeline stages are modeled flat (only last stage fallible), no
   per-stage subshell-env-isolation — decide if the effect-builder needs it.
3. **`oracle` crate** — fact-centric lift (note 162 v2): kind + fact-probe +
   accumulating effect-map; ⊤ on non-literal anchors / top-level mutators.
4. **`analysis::effect`** — may-mutate (forward-may, instantiate `solve`) + the
   W5 ambient∧invariant gate (reaching-defs over the effect-map) + MUST/MAY
   skip-licensing (note 163 §2). Effect-builder handoff is in the CFG subagent's
   report (effects on `Command` nodes via `node.ast`; redirs are separate `Redir`
   nodes; `Top` folds to ⊤; provenance via `AstId`).
5. **`probe` crate** — leaf-seam (dn-3, `LeafId→AstId`), read-only projection
   (kFAIL-withhold), `VerdictMemo{verdict,content_key,freshness}`, faithful+
   optimized modes (note 160 §5).
6. **`plan`/`cli`** — plan/diff as sh + why-provenance (the hard reporting).
7. **`hostsim`** (DST) — seeded state-machine host; synthesize verdicts; the
   kFAIL-withhold check (DP-4). note 163 §4.

## 6. Environment / resume notes
- Toolchain: `x86_64-pc-windows-gnu` pinned via `spike/mise.toml [env]` (no MSVC
  linker on this box). Build/test: `mise exec -- cargo …` from `spike/`. Plain
  `cargo` (no `+toolchain`) is correct.
- SPA read: §4 (lattices) + §5 (monotone frameworks) + §2.5 (CFG) in main context
  (synthesized into 163); §7 (path-sensitivity, for guard-narrowing) + §8/§9
  (interprocedural/IFDS, Tier-B) deferred — read when those phases begin (and
  fetch a clean render of §9's exploded-supergraph figure, which the PDF→txt
  mangled). SPA text: `Research/sources/B-moller-schwartzbach-…txt`.
- Subagent rule (spike/CLAUDE.md): every subagent reads README+DESIGN first;
  implementation subagents get my design notes; **adversarial critics get NO
  notes + must web-research independently** (so they don't echo my blind spots).
- Commits are small/granular on `ai/spike-impl`; never pushed.
