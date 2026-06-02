# Phase 2 plan — language, parser strategy, orchestration scope

> ⟢ 2026-06 — two calls here are reconsidered: the Melange (OCaml-core + TS-harness) option is **not** "over-engineering" — it's a live adoption/contributor-friendliness contender — and the impl-language decision is deferred until after the corpus spike (downstream of the `kFACTS` substrate measurement). Language choice is a USER DECISION, not a knob. Grep "Melange" / "language" for current direction.

The "bog-standard but workload-shaping" decisions. These are mostly *the user's calls*; this plan lays out the tradeoffs with a leaning and marks each `[USER DECISION]`. Confidence markers throughout. A key structural insight up front collapses several of these:

> **Decouple the system into four parts with a serialization seam between them:** (1) **analyzer core** (parse → CFG → effect analysis), (2) a **serializable IR / verdict contract** (JSON-ish), (3) an **executor/orchestrator** (ships probes, streams output, applies mutations), (4) the **shipped probe** = *portable POSIX shell* with **zero Dorc runtime on the target** (the cdist/rset "only needs /bin/sh" property the planning log admired). Because the seam is a serialization boundary, the analyzer and executor need not even be the same language, and the impl-language choice does **not** leak onto target hosts.

## A. Implementation language — `[USER DECISION]`, leaning below
Inputs gathered this round:
- **Parser fit no longer forces OCaml** (see §B): the ideal-for-*POSIX* tool (Menhir incremental-LR, Morbig's recipe) is OCaml-only — but for a *superset we design ourselves* the recommended parser strategy is hand-rolled recursive-descent + lexer-modes (Oils-style), which is **language-agnostic**. So the parser stops being a forcing function. This is the single biggest update to the language question.
- **Author strength**: OCaml + TS strong; Rust/C-family rusty (user's stated profile). "OCaml ages well" (mild longevity plus, *not* a selection criterion per the user's clarification).
- **Prior-art lineage** is OCaml (Morbig/morsmall/CoLiS/Goblint/Smoosh) — but all reference, only Goblint/Smoosh are permissive; we crib *concepts*, so lineage-language is a weak pull.
- **Distribution/executor**: single static binary + easy cross-compile favors **Rust** (or Go); OCaml native binaries are fine but cross-compile is fiddlier.
- **Ecosystem/growth**: Rust larger/modern + a skill the user is "rusty" in (growth opportunity); OCaml = comfort + speed-to-first-prototype.

Three viable shapes:
1. **OCaml end-to-end.** Fastest prototype for *this user*; can use Menhir if we ever want LR; native binaries OK. Best if speed-to-working-analyzer dominates.
2. **Rust end-to-end.** Best distribution/executor story + ecosystem + deliberate skill-building; costs parser-gen convenience (mitigated by hand-rolling, §B) and some author-velocity. Best if single-binary distribution + longevity-of-ecosystem dominate.
3. **OCaml analyzer + TS/JS harness (optionally via Melange).** Typed analyzer core + rich TS orchestration/CLI/UX (realtime streaming, etc.). GUESS: **the Melange code-sharing is over-engineering for v1** — the §-intro serialization seam already lets an OCaml analyzer feed a TS executor over JSON *without* Melange. Reach for Melange only if you later want to *share types/logic* across the seam.

Leaning (GUESS, low-confidence — genuinely your call): **analyzer core in OCaml** (author velocity + Menhir-optionality + the strongest prior-art to read alongside), **executor as a separate component** in whatever best ships a streaming-ssh single binary (could be OCaml too; Rust/Go if distribution friction bites). If you'd rather invest in Rust for the single-binary/longevity story and treat the hand-rolled parser as the vehicle for learning it, that's equally defensible — the architecture (§-intro) makes the choice low-regret because the seam isolates it. **This is the decision I'd most like to make *with* you.**

## B. Parser: hand-roll vs library — recommend **hand-rolled recursive-descent + lexer-modes**
Three referenceable architectures (detail in `notes/040`): Morbig (Menhir incremental-LR, GPL-3, OCaml), Oils/OSH (recursive-descent + re2c lexer-modes, Apache-2.0), mvdan/sh (hand-rolled, BSD, Go).
- **Why hand-roll (Oils-style), SUSPECT:** (i) we're parsing a *superset we design*, not POSIX-as-given — so we don't need Morbig's incremental-LR machinery whose whole purpose is faithfully matching POSIX's nasty ambiguous grammar + 9 special rules; we can define a clean grammar and avoid that pain entirely. (ii) Matches the user's stated stance ("own the parser, extend with explicit non-backwards-compatible syntax, parsers aren't that hard"). (iii) Best error messages (a stated nice-to-have) — hand-rolled keeps fine context control (the one thing ShellCheck does better than Morbig). (iv) **Language-agnostic** → doesn't constrain §A. (v) Avoids GPL-3 reuse and any parser-gen dependency.
- **Keep from the references regardless:** the **lossless invariant** (tokens reconstruct source — needed to re-emit/annotate scripts), **lexer-modes** (the technique that makes shell statically lexable), a **schema-defined IR with generated visitors** (ASDL-style or `visitors`-ppx — gives the modular "analysis = visitor over stable IR" that ShellCheck lacks), and the **enumerated dynamic-construct boundary** as the `unsafe`/⊤ set.
- **Use tree-sitter-bash (MIT) only as the Step −1 spike tool** (cheap corpus parse now), not the production parser — it's a *bash* grammar, gives a generic tree not our IR, and is awkward to extend with our syntax.
- Menhir stays a fallback (GUESS): if the designed grammar turns out cleanly LR(1), a vanilla Menhir grammar (no incremental tricks needed) is a legitimate low-effort option in OCaml. Decide after the grammar stabilizes.

## C. Orchestration scope — **separate components; thin built-in streaming executor**
- The planning log defers inventory/transport but makes **realtime output a hard requirement** — which the obvious throwaway engine (Ansible) *structurally cannot meet* (per-action SSH + buffering).
- Recommendation (SUSPECT): build a **minimal push-over-ssh executor** — `ControlMaster` multiplexing, parallel fan-out, **live stdout/stderr streaming** — i.e. the planning-log's rset-like primitive (rset is ~hundreds of LoC). Architect analyzer ⟂ executor as **separate components over the IR/verdict seam**, so a real orchestrator (or pyinfra-as-library) can be swapped in later. So Dorc *is* a minimal orchestrator, but minimally and do-one-thing-well — the value is the analysis; the executor is thin and swappable. (Aligns with the cdist/rset/UNIX-philosophy admiration.)
- **Repo shape (GUESS):** one repo, cleanly separable components (analyzer / IR / executor / oracle-stdlib), seam = serialized IR. Not artificially split, not a tangled monolith.

## D. The throwaway-v1 (transpile-to-Ansible) — **lean AGAINST it now** (a reversal worth flagging)
The planning log tentatively picked transpile-to-Ansible-v1 to "borrow the module library for ergonomics." This round undercuts that:
- The **bootstrap oracle set is small, enumerable, and cheap** (the bash-in-the-wild ranking: ~top-30 builtins + ~top-30 coreutils cover the bulk; oracles are "dumb-as-rocks 98%"). We don't need Ansible's modules to start — we need ~50 one-line check-oracles we can write ourselves.
- Ansible's execution model **conflicts with the hard realtime-output requirement** and **can't frontload** (the planning log's own conclusion).
→ SUSPECT the better v1 is: **thin ssh-streaming executor + the ~50 hand-written bootstrap oracles + per-host skip**, skipping the Ansible detour entirely. (Flag: this contradicts the planning-log's tentative v1 pick; worth a deliberate decision.)

## E. Distribution / packaging
- **Control-node tool**: single static binary strongly preferred (Rust/Go trivially; OCaml native fine). 
- **Target hosts**: nothing installed — the shipped probe is **portable POSIX shell** (zero deps beyond `/bin/sh`), per §-intro. This is a hard design constraint, not a nicety: it's the "works on the unkillable sshd-and-a-prompt box" promise. It also means the impl-language never touches the target.
- **Oracle distribution**: oracles are shell + metadata; ship with the binary as a stdlib, plus a path for user/community oracles (registry deferred per planning log).

## F. Open calls to settle with the user (ranked)
1. **`[USER DECISION]` Implementation language** (§A) — the headline call; low-regret due to the seam, but sets velocity vs distribution-vs-growth tradeoff. *Recommend deciding interactively.*
2. **`[USER DECISION]` Drop the Ansible-transpile v1?** (§D) — contradicts the planning log; I lean yes-drop.
3. Parser strategy (§B) — I'm fairly confident on hand-rolled recursive-descent; confirm.
4. Executor: build-thin vs defer-to-existing (§C) — I lean build-thin for the realtime requirement.
5. Whether the analyzer/executor are one binary or two over the seam — minor; lean two components, one repo.
