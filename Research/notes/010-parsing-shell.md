# Parsing POSIX shell — findings (Morbig paper + Dozer)

## Why POSIX shell defeats the textbook Lex→Yacc pipeline (Morbig §2)
SURE, and this is the single most important Phase-1 feasibility fact: **a clean lexer→parser pipeline is impossible for shell.** Concretely:
- **Token recognition is not regular** — nested quotes/subshells (`$( … $( … ) )`) need pushdown power, not a finite-state lexer.
- **Lexing depends on the *parse* context** — a `WORD` is promoted to a reserved word (`for`/`do`/…) or an `ASSIGNMENT_WORD` only if the parser state expects it there (`ls for i` → `for` is just an argument).
- **Lexing depends on the *evaluation* context** — `alias`/`eval` define macros expanded before lexing, so *lexing unrestricted shell is undecidable*. Morbig handles aliases only at top-level via a preprocessing pass; `eval` is simply out of the analyzable fragment.
- **Newlines have 4 distinct interpretations**; here-documents are *non-local* (the body for `<<EOF1` can appear several tokens later); `#` is not a delimiter (`foo#bar` is one word).
- **The official grammar is ambiguous** — 5 shift/reduce conflicts (all around `case`-item newlines, semantically harmless) + 9 "special rules" that are really parsing-dependent lexical hacks.

GUESS the design consequence for Dorc: our "strict superset, our needs first, not bashism-compatibility" stance (planning-log principle 8) is *vindicated by this paper* — the undecidable/irregular parts (alias/eval-driven lexing) are exactly what a from-scratch superset can define away. We don't inherit POSIX's parsing pain; we choose a statically-parseable core and push the rest to `unsafe`/⊤.

## Morbig's architecture [A-morbig-sle-2018] (the canonical answer to "how do you parse shell statically")
SURE this is the reference architecture, and it's small:
- **Prelexer** (parsing-*independent*, OCamllex): "token recognition" → pretokens (operator / word / significant-layout). Uses *mutually-recursive parametric lexer entry points* — recursion+params give the lexer pushdown power for nested quoting, while each entry point maps to a named POSIX spec section (review-friendly).
- **Lexer** (parsing-*dependent*): promotes words→keywords/assignments, switches to here-doc mode, disambiguates newlines — *by asking the parser*.
- **Parser**: **Menhir's incremental, purely-functional LR(1) interface** (`offer`/`resume` over an immutable `checkpoint`). This one choice buys everything: **speculative parsing** (try a token, backtrack for free because state is immutable), **reentrancy** (sub-parsers for `$(...)`), **longest-valid-prefix** mode (to delimit subshells), and **parser-state introspection** to drive the context-dependent lexing.
- Output is a **concrete syntax tree (parse tree), not an AST** — deliberately, so many analyses (incl. statistical) run over one stable tree. Visitors (iter/map/reduce + 2-arg variants) auto-generated from the type defs by Pottier's `visitors` ppx; 108 CST cases.
- Size: ~2141 LoC OCaml total (~1000 for the shell-specific peculiarities) vs Dash's 1569 lines of inscrutable C / Bash's ~5000 extra C lines. Speed: 31,521 Debian scripts in 41 s (1.3 ms avg, 100 ms max).
- License: **GPL-3** ⚠ — reusing Morbig source contaminates Dorc with GPL-3. Cribbing the *architecture/techniques* is free; copying code is not. Decide deliberately in Phase 2.

## The trust argument — directly supports "verification is bounded, not a goal"
SURE and important for the Coq question: the CoLiS group — about as formal-methods-heavy as this field gets — explicitly says of the **parser**: *"as the specification is informal, it is impossible to prove our code formally correct. We actually do not even claim the absence of bugs."* Their trust basis is instead: (i) code written for review (40% comments quoting POSIX), (ii) the **official BNF cut-and-pasted verbatim** (the mapping to spec *is* the argument), (iii) a spec-structured test-suite, (iv) **differential testing** against dash/bash in POSIX mode. → Even the people who *could* prove it chose engineering-grade trust for the parser. This is strong evidence for our "calibrated, best-effort" line over Coq-grade soundness, at least for the front-end.

## ShellCheck contrast (Morbig §7.1) — the production vs research-grade tradeoff
SUSPECT this frames a core Phase-2 design choice. ShellCheck: Haskell + Parsec combinators, hand-crafted, **no embedded Yacc grammar** (so its relation to POSIX is unclear) and **analyses run on-the-fly during parsing with no intermediate CST** → new analyses require editing the parser (poor modularity). But hand-crafting gives fine context control → **excellent error messages**. Morbig: modular, spec-faithful CST, but research-grade UX. Dorc wants *both*: Morbig's separable-analyses-over-a-stable-IR + ShellCheck's error quality (Morbig notes they plan to adopt Pottier's Menhir error-diagnosis work [Pottier CC2016] for exactly this) + our own syntax extensions. This is the "own the parser" justification (planning-log) made concrete.

## Dozer [B-dozer-icse-seip-2022] (2-page short paper; code at github.com/config-migration/dozer)
SURE on the one borrowable insight: shell commands and Ansible modules **change system state only via syscalls**, so the kernel/syscall boundary is the ground-truth altitude for "what did this command actually change." That informs *what a `.check`/effect oracle must ultimately observe*.
- Approach is **dynamic** (strace, run in Docker, compare straces to a knowledge base of Ansible-module traces, synthesize+validate a migration). We rejected dynamic syscall-tracing as the primary mechanism; this is premise-validation, not an approach to copy.
- Their Discussion explicitly names our hard problem as future work: **composing configuration tasks** — "correct propagation of information as outputs and inputs, selecting tasks that don't conflict, and preserving control flow and error handling." → Independent confirmation that *composition* (not per-command effect-detection) is the novel/hard core — matches the user's reframe ("the derivable part is how oracles compose").
- GUESS: cloning the Dozer code is low-value (Python, dynamic, tangential); keep as paper + repo-link reference.

## Open implications for the two plans
- Phase 2 parser decision is now sharper: **Menhir(incremental)+OCamllex is a proven, ~2kLoC recipe for exactly this problem** — a strong point in OCaml's favor *for the parser specifically*. If Rust, the equivalent (incremental/functional LR with state introspection + speculative parsing) is not obviously available off-the-shelf — needs investigation (lalrpop/chumsky don't expose Menhir-style incremental checkpoints AFAIK; GUESS, verify in Phase 2). This is a real, possibly-decisive language input.
- Phase 1 architecture: front-end (parse→CST/IR) is a *solved* problem with a referenceable codebase; the novel work is everything downstream (effect lattice, check-projection, composition). Don't re-research parsing; crib the recipe.
